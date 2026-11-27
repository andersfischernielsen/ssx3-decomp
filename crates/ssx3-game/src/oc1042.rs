//! Staging module oc1042 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1042`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002d_66a0,
    avalanche_bake_node_history,
    "Avalanche_BakeNodeHistory",
)];

/// `void Avalanche_BakeNodeHistory(sAvNode* node, sAv* av)` (0x002d66a0,
/// `tAvalancheNode_calculate` in the peak3 fork): bakes one avalanche node's
/// path. It allocates the node's frame array (10 bytes per frame, `length`
/// frames), then simulates a rolling snow ball frame by frame (`D_004A3AC8`
/// sub-steps each): drag/noise on the velocity, a sphere sweep against the
/// world (`Nash_FindClosestObjectContact`) that records hit objects in the
/// av table, a second sweep for surface contact with bounce/slide response and
/// avalanche events, surface colour sampling interpolated along the path, and
/// the packed per frame colour, orientation and velocity bytes. Finally the
/// object table and the event entries are padded and sorted (`qsort`).
/// The body is the generated one (labelled blocks) with its calls through
/// `call_ptr`/`Callee`: every register, cycle and call is kept.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn avalanche_bake_node_history(c: &mut Cpu) {
    if c.pc != 0x002d66a0 {
        return ee::s09::f_002d66a0__generated(c);
    }
    guest(c, |c| {
        'b_2d7af0: {
            c.cycles += 99;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3e0));
            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
            c.g[T0][0] = 0xa;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
            c.g[S0][0] = 0x500000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
            c.g[S1][0] = 0x7f0000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
            c.g[S5][0] = c.g[A0][0];
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
            c.g[S6][0] = c.g[A1][0];
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
            c.g[A1][0] = 0x480000;
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7538));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
            c.g[S1][0] = c.g[S1][0] | 0xffff;
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x2f0), c.g[RA][0]);
            c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
            c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
            c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 25);
            c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x100));
            c.g[A2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xf8));
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.lq(3, (c.g[A2][0] as u32).wrapping_add(0x40));
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
            c.mult(0, 0, 4, 8);
            c.g[A2][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
            c.sq(3, (c.g[S5][0] as u32).wrapping_add(0xe0));
            c.swc1(25, (c.g[S5][0] as u32).wrapping_add(0xec));
            c.g[A0][0] = c.lo[0];
            c.g[RA][0] = sx(0x002d675c);
            c.sq(3, (c.g[SP][0] as u32));
            call_ptr(c, 0x00317e30, 0x002d675c)?;
            c.g[V1][0] = 0x1;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x4;
            c.sb((c.g[S5][0] as u32).wrapping_add(0xf3), c.g[V1][0]);
            c.g[RA][0] = sx(0x002d6778);
            c.sw((c.g[S5][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
            call_ptr(c, 0x00416210, 0x002d6778)?;
            c.g[V0][0] = 0x500000;
            c.lwc1(23, (c.g[GP][0] as u32).wrapping_add(0x990));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
            c.sw((c.g[SP][0] as u32), 0u64);
            c.lq(4, (c.g[V0][0] as u32));
            c.neg_s(22, 23);
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
            c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3cf8));
            c.lq(2, (c.g[S5][0] as u32).wrapping_add(0xe0));
            c.lq(3, (c.g[S0][0] as u32));
            c.mul_s(22, 22, 21);
            c.lwc1(24, (c.g[SP][0] as u32));
            c.mul_s(21, 23, 21);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), 0u64);
            c.g[S0][0] = 0x3f800000;
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sub_s(21, 21, 22);
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[RA][0] = sx(0x002d67dc);
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x003177f0, 0x002d67dc)?;
            c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
            c.mtc1(2, 0);
            c.sub_s(20, 0, 25);
            c.mul_s(20, 21, 20);
            c.g[RA][0] = sx(0x002d67f8);
            c.add_s(20, 22, 20);
            call_ptr(c, 0x003177f0, 0x002d67f8)?;
            c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
            c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
            c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mtc1(2, 1);
            c.c_lt_s(24, 4);
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
            c.sub_s(0, 1, 25);
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x5c));
            c.mul_s(21, 21, 0);
            c.add_s(22, 22, 21);
            let t = !c.fcc();
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x54));
            if t {
                break 'b_2d7af0;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 28);
                'l_2d6838: loop {
                    'b_2d68a0: {
                        'b_2d689c: {
                            'b_2d6874: {
                                c.cycles += 13;
                                c.lwc1(12, (c.g[S5][0] as u32).wrapping_add(0x108));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c8));
                                c.cvt_s_w(0, 0);
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3cf4));
                                c.sub_s(2, 4, 12);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
                                c.mov_s(27, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x6c));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x64), 0u64);
                                c.c_le_s(0, 2);
                                let t = !c.fcc();
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x68), 0u64);
                                if t {
                                    break 'b_2d6874;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lwc1(21, (c.g[S5][0] as u32).wrapping_add(0x110));
                                    if t {
                                        break 'b_2d689c;
                                    } else {
                                        break 'b_2d6874;
                                    }
                                }
                            }
                            c.cycles += 9;
                            c.sub_s(0, 4, 27);
                            c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x110));
                            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.mul_s(1, 1, 0);
                            c.div_s(21, 1, 12);
                            c.c_lt_s(2, 21);
                            let t = !c.fcc();
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_2d68a0;
                            } else {
                                c.cycles += 1;
                                c.sqrt_s(21, 21);
                                break 'b_2d689c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                        break 'b_2d68a0;
                    }
                    'b_2d7690: {
                        c.cycles += 17;
                        c.g[AT][0] = 0x438c0000;
                        c.mtc1(1, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
                        c.g[S0][0] = 0u64;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                        c.min_s(21, 21, 0);
                        c.g[AT][0] = 0x42700000;
                        c.mtc1(1, 1);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
                        c.max_s(21, 21, 1);
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 26);
                        let t = (c.g[V1][0] as i64) <= 0;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2dc), c.g[A0][0]);
                        if t {
                            break 'b_2d7690;
                        } else {
                            c.cycles += 17;
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                            c.g[S7][0] = 0x7f0000;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 23);
                            c.mov_s(22, 26);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d0), c.g[A1][0]);
                            c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d8), c.g[A2][0]);
                            c.g[S4][0] = 0x7f800000;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                            c.g[S3][0] = 0x7fc00000;
                            c.g[S2][0] = 0xffffffffff800000;
                            c.g[S1][0] = 0xffffffffffc00000;
                            c.g[S7][0] = c.g[S7][0] | 0xffff;
                            'l_2d6928: loop {
                                'b_2d6e2c: {
                                    'b_2d6e28: {
                                        c.cycles += 34;
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.g[A0][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[S0][0]);
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[RA][0] = sx(0x002d6968);
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        Callee {
                                            addr: 0x002d64d8,
                                            f: ee::s09::Avalanche_ClampVector,
                                        }
                                        .call(c, 0x002d6968)?;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
                                        c.mov_s(12, 21);
                                        c.sq(2, (c.g[SP][0] as u32));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x002d6988);
                                        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        call_ptr(c, 0x0032fb48, 0x002d6988)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2cc));
                                        c.g[RA][0] = sx(0x002d699c);
                                        c.g[A3][0] = 0x1;
                                        call_ptr(c, 0x003378c0, 0x002d699c)?;
                                        c.cvt_w_s(1, 0);
                                        c.mfc1(2, 1);
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                                        if t {
                                            break 'b_2d6e28;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
                                            let t = c.g[A0][0] == 0u64;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                            if t {
                                                break 'b_2d6e2c;
                                            } else {
                                                'b_2d6a64: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S6][0] as u32).wrapping_add(0xc));
                                                    c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x9dc)
                                                        );
                                                    if t {
                                                        break 'b_2d6a64;
                                                    } else {
                                                        'b_2d6a60: {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x002d69d4);
                                                            call_ptr(c, 0x00350698, 0x002d69d4)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V1][0] = 0u64;
                                                            if t {
                                                                break 'b_2d6a60;
                                                            } else {
                                                                'b_2d6a20: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A2][0] = c.g[V0][0];
                                                                    if t {
                                                                        break 'b_2d6a20;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        c.g[A0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xf0));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[A0][0];

                                                                        if t {
                                                                            break 'b_2d6a20;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[A3][0] = c.g[A0][0];
                                                                            c.g[A1][0] = c.g[A2][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S6][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x14,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            'l_2d6a08: loop {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = ((c.g
                                                                                    [V1][0]
                                                                                    as i64)
                                                                                    < (c.g[A1][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    == 0u64;
                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                if t {
                                                                                    break 'b_2d6a20;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        (c.g[A0][0]
                                                                                            as u32),
                                                                                    );
                                                                                    if c.g[V0][0]
                                                                                        != c.g[A3]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x002d6a08);
                                                                                        }
                                                                                        continue 'l_2d6a08;
                                                                                    } else {
                                                                                        break 'b_2d6a20;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[V1][0] != c.g[A2][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x9dc));
                                                                if t {
                                                                    break 'b_2d6a64;
                                                                } else {
                                                                    c.cycles += 14;
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x3cf0),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2d4));
                                                                    c.mul_s(0, 27, 0);
                                                                    c.g[V1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V1][0] as u32),
                                                                        ));
                                                                    c.swc1(0, (c.g[V1][0] as u32));
                                                                    c.g[V0][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xf0));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 3);
                                                                    c.g[V0][0] = sx((c.g[S6][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V0][0] as u32),
                                                                        ));
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x14),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[V1][0] = c.lhu(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.sh(
                                                                        (c.g[S6][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                        c.g[V1][0],
                                                                    );
                                                                    break 'b_2d6a60;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x9dc));
                                                        break 'b_2d6a64;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                                if t {
                                                    break 'b_2d6e2c;
                                                } else {
                                                    'b_2d6b88: {
                                                        c.cycles += 65;
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.qmfc2(2, 4);
                                                        c.mtc1(2, 0);
                                                        c.g[A1][0] = 0u64;
                                                        c.mfc1(2, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.mfc1(3, 22);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x9a0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(3, 22);
                                                        c.mtc1(3, 0);
                                                        c.mul_s(0, 0, 1);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x200),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x130));
                                                        if c.g[V0][0] == c.g[S4][0] {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d6b88;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d6b88;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d6b88;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S1][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d6b88;
                                                                    } else {
                                                                        break 'b_2d6b88;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6c20: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] != 0u64 {
                                                            c.g[A1][0] = 0x1;
                                                            break 'b_2d6c20;
                                                        } else {
                                                            'b_2d6bb8: {
                                                                'b_2d6bb4: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d6bb4;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d6bb8;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6bb8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d6bb8;
                                                                                } else {
                                                                                    break 'b_2d6bb4;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d6bb8;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A1][0] = 0x1;
                                                                break 'b_2d6c20;
                                                            } else {
                                                                'b_2d6be8: {
                                                                    'b_2d6be4: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x138));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d6be4;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6be8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6be8;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d6be8;
                                                                                    } else {
                                                                                        break 'b_2d6be4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6be8;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A1][0] = 0x1;
                                                                    break 'b_2d6c20;
                                                                } else {
                                                                    'b_2d6c18: {
                                                                        'b_2d6c14: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x13c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d6c14;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6c18;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d6c18;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d6c18;
                                                                                        } else {
                                                                                            break 'b_2d6c14;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6c18;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d6c20;
                                                                    } else {
                                                                        break 'b_2d6c20;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6c34: {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;
                                                        c.g[V1][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d6c34;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0xee0));
                                                            c.lq(2, (c.g[V1][0] as u32));
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            break 'b_2d6c34;
                                                        }
                                                    }
                                                    'b_2d6d30: {
                                                        'b_2d6d2c: {
                                                            c.cycles += 56;
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x99c),
                                                            );
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.mfc1(3, 0);
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1e0),
                                                            );
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.mfc1(3, 21);
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.qmtc2(3, 3);
                                                            c.vu0().mul(
                                                                0b1111,
                                                                5,
                                                                4,
                                                                Src::Bc(3, X),
                                                            );
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xa0),
                                                            );
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.g[RA][0] = sx(0x002d6cac);
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            Callee {
                                                                addr: 0x002d64d8,
                                                                f: ee::s09::Avalanche_ClampVector,
                                                            }
                                                            .call(c, 0x002d6cac)?;
                                                            c.lqc2(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.lqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.vu0().opmula(4, 5);
                                                            c.vu0().opmsub(6, 5, 4);
                                                            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                                            c.sqc2(
                                                                6,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.g[A0][0] = 0u64;
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.g[V1][0] = 0u64;
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.lqc2(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x200),
                                                            );
                                                            c.vu0().add(
                                                                0b1000,
                                                                6,
                                                                0,
                                                                Src::Bc(0, W),
                                                            );
                                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                            c.vu0().madd(
                                                                0b1000,
                                                                4,
                                                                6,
                                                                Src::Bc(4, W),
                                                            );
                                                            c.vu0().rsqrt(0, 3, 4, 0);
                                                            c.vu0().waitq();
                                                            c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                            c.sqc2(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x210),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1f0),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            let t = c.g[V0][0] == c.g[S4][0];
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_2d6d2c;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S3][0] {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6d30;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S2][0] {
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6d30;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[S1][0];

                                                                        if t {
                                                                            break 'b_2d6d30;
                                                                        } else {
                                                                            break 'b_2d6d2c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = 0x1;
                                                        break 'b_2d6d30;
                                                    }
                                                    'b_2d6dc8: {
                                                        c.cycles += 2;
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d6dc8;
                                                        } else {
                                                            'b_2d6d60: {
                                                                'b_2d6d5c: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d6d5c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d6d60;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6d60;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d6d60;
                                                                                } else {
                                                                                    break 'b_2d6d5c;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d6d60;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d6dc8;
                                                            } else {
                                                                'b_2d6d90: {
                                                                    'b_2d6d8c: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d6d8c;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d6d90;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6d90;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d6d90;
                                                                                    } else {
                                                                                        break 'b_2d6d8c;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d6d90;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d6dc8;
                                                                } else {
                                                                    'b_2d6dc0: {
                                                                        'b_2d6dbc: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d6dbc;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d6dc0;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d6dc0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d6dc0;
                                                                                        } else {
                                                                                            break 'b_2d6dbc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d6dc0;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d6dc8;
                                                                    } else {
                                                                        break 'b_2d6dc8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d6e20: {
                                                        'b_2d6de4: {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[A0][0] = 0x500000;
                                                            if t {
                                                                break 'b_2d6de4;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.mtc1(0, 24);
                                                                c.g[V1][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0xec0));
                                                                c.lq(2, (c.g[V1][0] as u32));
                                                                let t = true;
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                if t {
                                                                    break 'b_2d6e20;
                                                                } else {
                                                                    break 'b_2d6de4;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 15;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x978),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(2, 22);
                                                        c.mtc1(2, 2);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x3cec),
                                                        );
                                                        c.mul_s(1, 1, 2);
                                                        c.mul_s(0, 21, 0);
                                                        c.div_s(24, 1, 0);
                                                        break 'b_2d6e20;
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x3ce8),
                                                    );
                                                    c.min_s(24, 24, 0);
                                                    break 'b_2d6e28;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d0));
                                    break 'b_2d6e2c;
                                }
                                'b_2d7654: {
                                    'b_2d7650: {
                                        c.cycles += 15;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.mov_s(12, 21);
                                        c.g[RA][0] = sx(0x002d6e3c);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, 0x0032fb48, 0x002d6e3c)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d8));
                                        c.g[RA][0] = sx(0x002d6e50);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00336850, 0x002d6e50)?;
                                        c.mtc1(0, 1);
                                        c.mov_s(26, 0);
                                        c.c_le_s(1, 26);
                                        let t = !c.fcc();
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                                        if t {
                                            break 'b_2d7650;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
                                            if t {
                                                break 'b_2d7654;
                                            } else {
                                                'b_2d6ebc: {
                                                    c.cycles += 15;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_add(0x960),
                                                    );
                                                    c.abs_s(0, 0);
                                                    c.c_lt_s(1, 0);
                                                    let t = !c.fcc();
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2c8)
                                                        );
                                                    if t {
                                                        break 'b_2d6ebc;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c.g[S6][0];
                                                        c.g[RA][0] = sx(0x002d6ebc);
                                                        c.g[A2][0] = c.g[S5][0];
                                                        Callee {
                                                            addr: 0x002d9738,
                                                            f: ee::s09::Avalanche_AddNodeEvent,
                                                        }
                                                        .call(c, 0x002d6ebc)?;
                                                        break 'b_2d6ebc;
                                                    }
                                                }
                                                'b_2d7470: {
                                                    'b_2d7270: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.lhu(
                                                            (c.g[S5][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            break 'b_2d7270;
                                                        } else {
                                                            c.cycles += 11;
                                                            c.g[RA][0] = sx(0x002d6ed0);
                                                            call_ptr(c, 0x003177f0, 0x002d6ed0)?;
                                                            c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
                                                            c.g[A1][0] = 0x3f800000;
                                                            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x97c),
                                                            );
                                                            c.mtc1(2, 2);
                                                            c.sub_s(0, 2, 23);
                                                            c.c_lt_s(0, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_2d7270;
                                                            } else {
                                                                'b_2d70d0: {
                                                                    c.cycles += 113;
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.lqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(5),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.qmfc2(2, 4);
                                                                    c.mtc1(2, 0);
                                                                    c.g[A1][0] = 0u64;
                                                                    c.mfc1(2, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.qmtc2(2, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.g[A0][0] = 0u64;
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.mfc1(3, 22);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().rsqrt(0, 3, 4, 0);
                                                                    c.vu0().waitq();
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Q,
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x9a0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().sqrt(4, 0);
                                                                    c.vu0().waitq();
                                                                    c.cfc2(3, 22);
                                                                    c.mtc1(3, 0);
                                                                    c.mul_s(0, 0, 1);
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x998),
                                                                    );
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x120),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[GP][0] as u32)
                                                                            .wrapping_add(0x99c),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.mfc1(3, 0);
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.qmtc2(3, 3);
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        4,
                                                                        Src::Bc(3, X),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Vec(4),
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x200),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.lqc2(
                                                                        4,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0xb0),
                                                                    );
                                                                    c.lqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x60),
                                                                    );
                                                                    c.vu0().opmula(4, 5);
                                                                    c.vu0().opmsub(6, 5, 4);
                                                                    c.vu0().sub(
                                                                        0b0001,
                                                                        6,
                                                                        6,
                                                                        Src::Vec(6),
                                                                    );
                                                                    c.sqc2(
                                                                        6,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.lqc2(
                                                                        3,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1e0),
                                                                    );
                                                                    c.vu0().add(
                                                                        0b1000,
                                                                        6,
                                                                        0,
                                                                        Src::Bc(0, W),
                                                                    );
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        4,
                                                                        3,
                                                                        Src::Vec(3),
                                                                    );
                                                                    c.vu0().adda(
                                                                        0b1000,
                                                                        4,
                                                                        Src::Bc(4, Y),
                                                                    );
                                                                    c.vu0().madda(
                                                                        0b1000,
                                                                        6,
                                                                        Src::Bc(4, Z),
                                                                    );
                                                                    c.vu0().madd(
                                                                        0b1000,
                                                                        4,
                                                                        6,
                                                                        Src::Bc(4, W),
                                                                    );
                                                                    c.vu0().rsqrt(0, 3, 4, 0);
                                                                    c.vu0().waitq();
                                                                    c.vu0().mul(
                                                                        0b1111,
                                                                        5,
                                                                        3,
                                                                        Src::Q,
                                                                    );
                                                                    c.sqc2(
                                                                        5,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.lq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x1f0),
                                                                    );
                                                                    c.sq(
                                                                        2,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130));
                                                                    if c.g[V0][0] == c.g[S4][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d70d0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[A0][0] = 0x1;
                                                                            break 'b_2d70d0;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[A0][0] = 0x1;
                                                                                break 'b_2d70d0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S1][0]
                                                                                {
                                                                                    c.g[A0][0] =
                                                                                        0x1;
                                                                                    break 'b_2d70d0;
                                                                                } else {
                                                                                    break 'b_2d70d0;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_2d7168: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d7168;
                                                                    } else {
                                                                        'b_2d7100: {
                                                                            'b_2d70fc: {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x134));
                                                                                let t = c.g[V0][0]
                                                                                    == c.g[S4][0];
                                                                                c.g[V1][0] = 0u64;
                                                                                if t {
                                                                                    break 'b_2d70fc;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S3]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d7100;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == c.g
                                                                                                [S2]
                                                                                                [0]
                                                                                        {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_2d7100;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t = c.g[V0][0] != c.g[S1][0];

                                                                                            if t {
                                                                                                break 'b_2d7100;
                                                                                            } else {
                                                                                                break 'b_2d70fc;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 1;
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d7100;
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[V1][0] != 0u64 {
                                                                            c.g[A1][0] = 0x1;
                                                                            break 'b_2d7168;
                                                                        } else {
                                                                            'b_2d7130: {
                                                                                'b_2d712c: {
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == c.g[S4]
                                                                                            [0];
                                                                                    c.g[V1][0] =
                                                                                        0u64;
                                                                                    if t {
                                                                                        break 'b_2d712c;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V0]
                                                                                            [0]
                                                                                            == c.g
                                                                                                [S3]
                                                                                                [0]
                                                                                        {
                                                                                            c.g[V1][0] = 0x1;
                                                                                            break 'b_2d7130;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7130;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7130;
} else {
break 'b_2d712c;
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 1;
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d7130;
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[V1][0] != 0u64 {
                                                                                c.g[A1][0] = 0x1;
                                                                                break 'b_2d7168;
                                                                            } else {
                                                                                'b_2d7160: {
                                                                                    'b_2d715c: {
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x13c));
                                                                                        let t = c.g[V0][0] == c.g[S4][0];
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        if t {
                                                                                            break 'b_2d715c;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
if c.g[V0][0] == c.g[S2][0] {
c.g[V1][0] = 0x1;
break 'b_2d7160;
} else {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S1][0];

if t {
break 'b_2d7160;
} else {
break 'b_2d715c;
}
}
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7160;
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V1][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[A1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7168;
                                                                                } else {
                                                                                    break 'b_2d7168;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_2d717c: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A1][0] == 0u64;
                                                                    c.g[V1][0] = 0x500000;
                                                                    if t {
                                                                        break 'b_2d717c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0xee0,
                                                                                ));
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[V1][0] as u32),
                                                                        );
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x130,
                                                                                ),
                                                                        );
                                                                        break 'b_2d717c;
                                                                    }
                                                                }
                                                                c.cycles += 61;
                                                                c.g[RA][0] = sx(0x002d7184);
                                                                c.lwc1(
                                                                    20,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x980),
                                                                );
                                                                call_ptr(
                                                                    c, 0x003177f0, 0x002d7184,
                                                                )?;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[S7][0];
                                                                c.g[A0][0] = 0x3f800000;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[A0][0];
                                                                c.neg_s(0, 20);
                                                                c.mtc1(2, 2);
                                                                c.sub_s(20, 20, 0);
                                                                c.sub_s(1, 2, 23);
                                                                c.mul_s(20, 20, 1);
                                                                c.add_s(0, 0, 20);
                                                                c.mfc1(3, 0);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x130),
                                                                );
                                                                c.qmtc2(3, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1111,
                                                                    5,
                                                                    3,
                                                                    Src::Vec(4),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1000,
                                                                    6,
                                                                    0,
                                                                    Src::Bc(0, W),
                                                                );
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    4,
                                                                    3,
                                                                    Src::Vec(3),
                                                                );
                                                                c.vu0().adda(
                                                                    0b1000,
                                                                    4,
                                                                    Src::Bc(4, Y),
                                                                );
                                                                c.vu0().madda(
                                                                    0b1000,
                                                                    6,
                                                                    Src::Bc(4, Z),
                                                                );
                                                                c.vu0().madd(
                                                                    0b1000,
                                                                    4,
                                                                    6,
                                                                    Src::Bc(4, W),
                                                                );
                                                                c.vu0().rsqrt(0, 3, 4, 0);
                                                                c.vu0().waitq();
                                                                c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lwc1(
                                                                    20,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x984),
                                                                );
                                                                c.g[RA][0] = sx(0x002d720c);
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                call_ptr(
                                                                    c, 0x003177f0, 0x002d720c,
                                                                )?;
                                                                c.g[V1][0] = 0x3f800000;
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[S7][0];
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] | c.g[V1][0];
                                                                c.mtc1(2, 1);
                                                                c.sub_s(0, 1, 23);
                                                                c.mul_s(20, 20, 0);
                                                                c.mfc1(3, 20);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1f0),
                                                                );
                                                                c.qmtc2(3, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.lq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x200),
                                                                );
                                                                c.sq(
                                                                    2,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.lqc2(
                                                                    3,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x1e0),
                                                                );
                                                                c.vu0().add(
                                                                    0b1111,
                                                                    5,
                                                                    3,
                                                                    Src::Vec(4),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x988),
                                                                );
                                                                c.mfc1(2, 0);
                                                                c.lqc2(
                                                                    4,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.qmtc2(2, 3);
                                                                c.vu0().mul(
                                                                    0b1111,
                                                                    5,
                                                                    4,
                                                                    Src::Bc(3, X),
                                                                );
                                                                c.sqc2(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                let t = true;

                                                                if t {
                                                                    break 'b_2d7470;
                                                                } else {
                                                                    break 'b_2d7270;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d7390: {
                                                        c.cycles += 66;
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.qmfc2(2, 4);
                                                        c.mtc1(2, 0);
                                                        c.g[A1][0] = 0u64;
                                                        c.mfc1(2, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.mfc1(3, 22);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32).wrapping_add(0x9a0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().sqrt(4, 0);
                                                        c.vu0().waitq();
                                                        c.cfc2(3, 22);
                                                        c.mtc1(3, 0);
                                                        c.mul_s(0, 0, 1);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x130));
                                                        if c.g[V0][0] == c.g[S4][0] {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d7390;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d7390;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d7390;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S1][0] {
                                                                        c.g[A0][0] = 0x1;
                                                                        break 'b_2d7390;
                                                                    } else {
                                                                        break 'b_2d7390;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d7428: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] != 0u64 {
                                                            c.g[A1][0] = 0x1;
                                                            break 'b_2d7428;
                                                        } else {
                                                            'b_2d73c0: {
                                                                'b_2d73bc: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x134));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d73bc;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d73c0;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d73c0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d73c0;
                                                                                } else {
                                                                                    break 'b_2d73bc;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d73c0;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A1][0] = 0x1;
                                                                break 'b_2d7428;
                                                            } else {
                                                                'b_2d73f0: {
                                                                    'b_2d73ec: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x138));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d73ec;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d73f0;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d73f0;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d73f0;
                                                                                    } else {
                                                                                        break 'b_2d73ec;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d73f0;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A1][0] = 0x1;
                                                                    break 'b_2d7428;
                                                                } else {
                                                                    'b_2d7420: {
                                                                        'b_2d741c: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x13c,
                                                                                ));
                                                                            let t = c.g[V0][0]
                                                                                == c.g[S4][0];
                                                                            c.g[V1][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2d741c;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S3][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d7420;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == c.g[S2]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_2d7420;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[V0][0] != c.g[S1][0];

                                                                                        if t {
                                                                                            break 'b_2d7420;
                                                                                        } else {
                                                                                            break 'b_2d741c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d7420;
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[V1][0] != 0u64 {
                                                                        c.g[A1][0] = 0x1;
                                                                        break 'b_2d7428;
                                                                    } else {
                                                                        break 'b_2d7428;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2d743c: {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;
                                                        c.g[V1][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d743c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0xee0));
                                                            c.lq(2, (c.g[V1][0] as u32));
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x130),
                                                            );
                                                            break 'b_2d743c;
                                                        }
                                                    }
                                                    c.cycles += 13;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x130),
                                                    );
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x99c),
                                                    );
                                                    c.mfc1(3, 0);
                                                    c.lqc2(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1f0),
                                                    );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                    c.sq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                    );
                                                    break 'b_2d7470;
                                                }
                                                'b_2d7558: {
                                                    'b_2d7554: {
                                                        c.cycles += 51;
                                                        c.mfc1(2, 21);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.qmtc2(2, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = c.g[S5][0];
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A1][0] = c.g[FP][0];
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xa0),
                                                        );
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32).wrapping_add(0x994),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.sub_s(0, 23, 0);
                                                        c.mfc1(3, 0);
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.qmtc2(3, 3);
                                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.g[RA][0] = sx(0x002d74d4);
                                                        Callee {
                                                            addr: 0x002d64d8,
                                                            f: ee::s09::Avalanche_ClampVector,
                                                        }
                                                        .call(c, 0x002d74d4)?;
                                                        c.lqc2(
                                                            4,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.vu0().opmula(4, 5);
                                                        c.vu0().opmsub(6, 5, 4);
                                                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                                        c.sqc2(
                                                            6,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[A0][0] = 0u64;
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.g[V1][0] = 0u64;
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lqc2(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                        c.vu0().rsqrt(0, 3, 4, 0);
                                                        c.vu0().waitq();
                                                        c.vu0().mul(0b1111, 5, 3, Src::Q);
                                                        c.sqc2(
                                                            5,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x1e0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = c.g[V0][0] == c.g[S4][0];
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x1c),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_2d7554;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[V0][0] == c.g[S3][0] {
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d7558;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[V0][0] == c.g[S2][0] {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d7558;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[V0][0] != c.g[S1][0];

                                                                    if t {
                                                                        break 'b_2d7558;
                                                                    } else {
                                                                        break 'b_2d7554;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x1;
                                                    break 'b_2d7558;
                                                }
                                                'b_2d75f0: {
                                                    c.cycles += 2;
                                                    if c.g[V1][0] != 0u64 {
                                                        c.g[A0][0] = 0x1;
                                                        break 'b_2d75f0;
                                                    } else {
                                                        'b_2d7588: {
                                                            'b_2d7584: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                let t = c.g[V0][0] == c.g[S4][0];
                                                                c.g[V1][0] = 0u64;
                                                                if t {
                                                                    break 'b_2d7584;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == c.g[S3][0] {
                                                                        c.g[V1][0] = 0x1;
                                                                        break 'b_2d7588;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S2][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d7588;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[S1][0];

                                                                            if t {
                                                                                break 'b_2d7588;
                                                                            } else {
                                                                                break 'b_2d7584;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 1;
                                                            c.g[V1][0] = 0x1;
                                                            break 'b_2d7588;
                                                        }
                                                        c.cycles += 2;
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = 0x1;
                                                            break 'b_2d75f0;
                                                        } else {
                                                            'b_2d75b8: {
                                                                'b_2d75b4: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t =
                                                                        c.g[V0][0] == c.g[S4][0];
                                                                    c.g[V1][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2d75b4;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == c.g[S3][0]
                                                                        {
                                                                            c.g[V1][0] = 0x1;
                                                                            break 'b_2d75b8;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S2][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d75b8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V0][0]
                                                                                    != c.g[S1][0];

                                                                                if t {
                                                                                    break 'b_2d75b8;
                                                                                } else {
                                                                                    break 'b_2d75b4;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_2d75b8;
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != 0u64 {
                                                                c.g[A0][0] = 0x1;
                                                                break 'b_2d75f0;
                                                            } else {
                                                                'b_2d75e8: {
                                                                    'b_2d75e4: {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        let t = c.g[V0][0]
                                                                            == c.g[S4][0];
                                                                        c.g[V1][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2d75e4;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            if c.g[V0][0]
                                                                                == c.g[S3][0]
                                                                            {
                                                                                c.g[V1][0] = 0x1;
                                                                                break 'b_2d75e8;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    == c.g[S2][0]
                                                                                {
                                                                                    c.g[V1][0] =
                                                                                        0x1;
                                                                                    break 'b_2d75e8;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_2d75e8;
                                                                                    } else {
                                                                                        break 'b_2d75e4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_2d75e8;
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[V1][0] != 0u64 {
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_2d75f0;
                                                                } else {
                                                                    break 'b_2d75f0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_2d7648: {
                                                    'b_2d760c: {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.g[A0][0] = 0x500000;
                                                        if t {
                                                            break 'b_2d760c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 24);
                                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0xec0));
                                                            c.lq(2, (c.g[V1][0] as u32));
                                                            let t = true;
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            if t {
                                                                break 'b_2d7648;
                                                            } else {
                                                                break 'b_2d760c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 15;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[GP][0] as u32).wrapping_add(0x978),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.vu0().sqrt(4, 0);
                                                    c.vu0().waitq();
                                                    c.cfc2(2, 22);
                                                    c.mtc1(2, 2);
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x3ce4),
                                                    );
                                                    c.mul_s(1, 1, 2);
                                                    c.mul_s(0, 21, 0);
                                                    c.div_s(24, 1, 0);
                                                    break 'b_2d7648;
                                                }
                                                c.cycles += 2;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ce0));
                                                c.min_s(24, 24, 0);
                                                break 'b_2d7650;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x994));
                                    break 'b_2d7654;
                                }
                                c.cycles += 15;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e0));
                                c.g[A1][0] = c.g[FP][0];
                                c.sub_s(0, 23, 0);
                                c.mfc1(2, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.qmtc2(2, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x002d7680);
                                Callee {
                                    addr: 0x002d64d8,
                                    f: ee::s09::Avalanche_ClampVector,
                                }
                                .call(c, 0x002d7680)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9d8));
                                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002d6928);
                                    }
                                    continue 'l_2d6928;
                                } else {
                                    break 'b_2d7690;
                                }
                            }
                        }
                    }
                    'b_2d7958: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_le_s(0, 26);
                        let t = !c.fcc();
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
                        if t {
                            break 'b_2d7958;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                            if t {
                                break 'b_2d7958;
                            } else {
                                'b_2d76f0: {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x156));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                    let t = (c.g[A1][0] as i64) < 0;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_2d76f0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x155));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = c.g[V1][0] ^ 0x6;
                                        if c.g[V1][0] != 0u64 {
                                            c.g[V0][0] = 0u64;
                                            break 'b_2d76f0;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x18;
                                            c.mult(0, 3, 5, 2);
                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x3f0));
                                            c.g[V1][0] = c.g[V1][0] ^ 0x3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                                            break 'b_2d76f0;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_2d7958;
                                } else {
                                    'b_2d77e8: {
                                        c.cycles += 50;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf4));
                                        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.g[RA][0] = sx(0x002d770c);
                                        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        Callee {
                                            addr: 0x002edb20,
                                            f: ee::s09::WorldPainter_SampleSurfaceColours,
                                        }
                                        .call(c, 0x002d770c)?;
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                        c.sub_s(1, 25, 1);
                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x88));
                                        c.sub_s(2, 25, 2);
                                        c.sub_s(4, 25, 4);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x9b4));
                                        c.sub_s(3, 25, 3);
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x150));
                                        c.mul_s(9, 1, 0);
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x154));
                                        c.mul_s(7, 2, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        c.mul_s(8, 4, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x124));
                                        c.mul_s(0, 3, 0);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x128));
                                        c.sub_s(6, 25, 9);
                                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                        c.sub_s(5, 25, 7);
                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x158));
                                        c.sub_s(2, 25, 0);
                                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x15c));
                                        c.sub_s(1, 25, 8);
                                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x134));
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x13c));
                                        c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x144));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x148));
                                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x14c));
                                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x137));
                                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
                                        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                        c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                        if t {
                                            break 'b_2d77e8;
                                        } else {
                                            c.cycles += 10;
                                            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x87));
                                            c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                            c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x88));
                                            c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x47));
                                            c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x48));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), 0u64);
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                            break 'b_2d77e8;
                                        }
                                    }
                                    'b_2d7910: {
                                        c.cycles += 13;
                                        c.g[S0][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_sub((c.g[A1][0] as u32)));
                                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x47));
                                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
                                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x137));
                                        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S2][0] = c.g[V0][0];
                                        if t {
                                            break 'b_2d7910;
                                        } else {
                                            c.cycles += 7;
                                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c4));
                                            c.cvt_s_w(0, 0);
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                            c.g[V0][0] = 0xa;
                                            c.mult(0, 17, 6, 2);
                                            c.sub_s(20, 27, 0);
                                            'l_2d7838: loop {
                                                c.cycles += 54;
                                                c.mtc1(16, 4);
                                                c.cvt_s_w(4, 4);
                                                c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x44));
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                c.div_s(4, 4, 20);
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
                                                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x48));
                                                c.sub_s(0, 0, 7);
                                                c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x4c));
                                                c.sub_s(1, 1, 8);
                                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x88));
                                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                                c.sub_s(2, 2, 6);
                                                c.g[A0][0] =
                                                    c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                                                c.sub_s(3, 3, 9);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x160));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x164));
                                                c.mul_s(0, 0, 4);
                                                c.mul_s(1, 1, 4);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x168));
                                                c.mul_s(5, 3, 4);
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x16c));
                                                c.mul_s(2, 2, 4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x150));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x154));
                                                c.add_s(0, 0, 7);
                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x15c));
                                                c.add_s(1, 1, 8);
                                                c.add_s(6, 2, 6);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x158));
                                                c.add_s(5, 5, 9);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x144));
                                                c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_add((c.g[S1][0] as u32)));
                                                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x148));
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0xa));
                                                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x14c));
                                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x147));
                                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x14f));
                                                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x148));
                                                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x137));
                                                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x13f));
                                                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x134));
                                                c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x138));
                                                c.g[RA][0] = sx(0x002d7904);
                                                c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x13c));
                                                Callee {
                                                    addr: 0x002d55c0,
                                                    f: ee::s09::Avalanche_PackClamped255x3,
                                                }
                                                .call(c, 0x002d7904)?;
                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                    < (c.g[S2][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d7838);
                                                    }
                                                    continue 'l_2d7838;
                                                } else {
                                                    break 'b_2d7910;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 18;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                                    c.g[A0][0] = 0xa;
                                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                                    c.mult(0, 4, 3, 4);
                                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x84));
                                    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x88));
                                    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8c));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
                                    c.g[RA][0] = sx(0x002d7938);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    Callee {
                                        addr: 0x002d55c0,
                                        f: ee::s09::Avalanche_PackClamped255x3,
                                    }
                                    .call(c, 0x002d7938)?;
                                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x87));
                                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                    c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x88));
                                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x47));
                                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x4f));
                                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x48));
                                    break 'b_2d7958;
                                }
                            }
                        }
                    }
                    'b_2d7a5c: {
                        c.cycles += 44;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c8));
                        c.g[V0][0] = 0xa;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.mult(0, 16, 3, 2);
                        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3cdc));
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                        c.mul_s(15, 0, 15);
                        c.lwc1(12, (c.g[SP][0] as u32));
                        c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[RA][0] = sx(0x002d7988);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                        Callee {
                            addr: 0x002d5658,
                            f: ee::s09::Avalanche_PackSnorm3Alpha,
                        }
                        .call(c, 0x002d7988)?;
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.sub_s(0, 0, 3);
                        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.sub_s(2, 2, 4);
                        c.mul_s(0, 0, 28);
                        c.g[AT][0] = 0x42f00000;
                        c.mtc1(1, 5);
                        c.sub_s(1, 1, 3);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x12c), 0u64);
                        c.mul_s(2, 2, 28);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.mul_s(1, 1, 28);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x128));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(2, 22);
                        c.mtc1(2, 0);
                        c.c_lt_s(5, 0);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                            break 'b_2d7a5c;
                        } else {
                            c.cycles += 21;
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.vu0().rsqrt(0, 3, 4, 0);
                            c.vu0().waitq();
                            c.vu0().mul(0b1111, 5, 3, Src::Q);
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                            c.mfc1(3, 5);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                            break 'b_2d7a5c;
                        }
                    }
                    c.cycles += 37;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2dc));
                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x124));
                    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x128));
                    c.g[RA][0] = sx(0x002d7a78);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c8), c.g[A1][0]);
                    Callee {
                        addr: 0x002d5598,
                        f: ee::s09::Avalanche_PackBytes3,
                    }
                    .call(c, 0x002d7a78)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xfc));
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c8));
                    c.cvt_s_w(3, 3);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.mtc1(3, 1);
                    c.cvt_s_w(1, 1);
                    c.mtc1(4, 2);
                    c.cvt_s_w(2, 2);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), 0u64);
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(1, 1, 1);
                    c.add_s(2, 2, 2);
                    c.add_s(0, 0, 0);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x148));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x144));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x100));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.c_lt_s(3, 4);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d6838);
                        }
                        continue 'l_2d6838;
                    } else {
                        break 'b_2d7af0;
                    }
                }
            }
        }
        'b_2d7b3c: {
            c.cycles += 6;
            c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2d4), c.g[A2][0]);
            c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_2d7b3c;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cd8));
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                'l_2d7b20: loop {
                    c.cycles += 7;
                    c.swc1(0, (c.g[A1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 32i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d7b20);
                        }
                        continue 'l_2d7b20;
                    } else {
                        break 'b_2d7b3c;
                    }
                }
            }
        }
        'b_2d7b7c: {
            c.cycles += 4;
            c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            if t {
                break 'b_2d7b7c;
            } else {
                c.cycles += 5;
                c.g[A2][0] = 0xffff;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x112));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                'l_2d7b60: loop {
                    c.cycles += 7;
                    c.sh((c.g[A1][0] as u32), c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sh((c.g[V1][0] as u32), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d7b60);
                        }
                        continue 'l_2d7b60;
                    } else {
                        break 'b_2d7b7c;
                    }
                }
            }
        }
        c.cycles += 33;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = 0x2d0000;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2d4));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5718));
        c.g[RA][0] = sx(0x002d7b94);
        c.g[A2][0] = 0x8;
        call_ptr(c, 0x00418ef8, 0x002d7b94)?;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0xe));
        c.g[A3][0] = 0x2d0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5750));
        c.g[RA][0] = sx(0x002d7bac);
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x00418ef8, 0x002d7bac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x380));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x370));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x360));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x350));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x340));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x330));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x320));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x310));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x300));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x3c8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x3c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x3b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x3a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x398));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x390));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the
    /// machine's RAM: other tests' machines run the real bodies), then the
    /// word of random bits that steers the faked collision answers.
    const OC1042_CTRL: u32 = DATA + 0x20_0000;
    const OC1042_MAGIC: u32 = 0x1042_f00d;
    const OC1042_STEER: u32 = OC1042_CTRL + 4;
    /// Fixture objects, clear of the fake log (`DATA+0x100000`) and of the
    /// pointers the fakes return (`DATA..DATA+0x40000`).
    const OC1042_NODE: u32 = DATA + 0x5_0000;
    const OC1042_SRC: u32 = DATA + 0x5_1000;
    const OC1042_TRI: u32 = DATA + 0x5_2000;
    const OC1042_TBL: u32 = DATA + 0x5_3000;
    const OC1042_BASE: u32 = DATA + 0x5_3100;
    const OC1042_AV: u32 = DATA + 0x5_4000;
    const OC1042_OBJ: u32 = DATA + 0x5_6000;
    const OC1042_WORLD: u32 = DATA + 0x5_7000;
    const OC1042_WORLD2: u32 = DATA + 0x5_7100;
    const OC1042_CTX: u32 = DATA + 0x5_7200;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and returns a pointer; `oc1042_after` then
    /// steers the collision answers.
    macro_rules! oc1042_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1042_CTRL) != OC1042_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1042_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1042_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// Bit `k` of the steering word, rotated by the call index.
    fn oc1042_bit(c: &mut Cpu, k: u32) -> bool {
        let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
        c.bus.read32(OC1042_STEER) >> ((n * 7 + k) % 32) & 1 != 0
    }

    /// The faked answers: the first sweep test (`Nash_FindClosestObjectContact`)
    /// finds a contact or not, the second (`0x336850`) returns a signed
    /// distance, `0x350698` accepts the object or not.
    fn oc1042_after(c: &mut Cpu, target: u32) {
        match target {
            0x0033_78c0 => {
                if !oc1042_bit(c, 0) {
                    c.g[V0][0] = 0;
                }
            }
            0x0033_6850 => {
                c.f[0] = if oc1042_bit(c, 1) {
                    1.5f32.to_bits()
                } else {
                    (-1.0f32).to_bits()
                };
            }
            0x0035_0698 => c.g[V0][0] = oc1042_bit(c, 2) as u64,
            _ => {}
        }
    }

    oc1042_fakes! {
        oc1042_fake_00317e30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc1042_fake_003177f0, 0x0031_77f0, ee::s10::OVERRIDE_003177F0, ee::s10::f_003177f0__generated;
        oc1042_fake_0032fb48, 0x0032_fb48, ee::s11::OVERRIDE_0032FB48, ee::s11::f_0032fb48__generated;
        oc1042_fake_003378c0, 0x0033_78c0, ee::s11::OVERRIDE_003378C0, ee::s11::f_003378c0__generated;
        oc1042_fake_00350698, 0x0035_0698, ee::s11::OVERRIDE_00350698, ee::s11::f_00350698__generated;
        oc1042_fake_00336850, 0x0033_6850, ee::s11::OVERRIDE_00336850, ee::s11::f_00336850__generated;
        oc1042_fake_00418ef8, 0x0041_8ef8, ee::s15::OVERRIDE_00418EF8, ee::s15::f_00418ef8__generated;
        oc1042_fake_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
        oc1042_fake_002d64d8, 0x002d_64d8, ee::s09::OVERRIDE_002D64D8, ee::s09::f_002d64d8__generated;
        oc1042_fake_002d9738, 0x002d_9738, ee::s09::OVERRIDE_002D9738, ee::s09::f_002d9738__generated;
        oc1042_fake_002edb20, 0x002e_db20, ee::s09::OVERRIDE_002EDB20, ee::s09::f_002edb20__generated;
        oc1042_fake_002d55c0, 0x002d_55c0, ee::s09::OVERRIDE_002D55C0, ee::s09::f_002d55c0__generated;
        oc1042_fake_002d5658, 0x002d_5658, ee::s09::OVERRIDE_002D5658, ee::s09::f_002d5658__generated;
        oc1042_fake_002d5598, 0x002d_5598, ee::s09::OVERRIDE_002D5598, ee::s09::f_002d5598__generated;
    }

    /// Hooked fakes, the game image (constants, `D_004FF120` vectors), the
    /// node and the avalanche table, a random hit record on the stack.
    /// `calls`: the call index at which a fake may unwind.
    fn oc1042_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc1042_hook();
        let mut r = fakes::machine(c, seed, 0x1042, calls);
        elf_image(c);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(c, OC1042_CTRL, OC1042_MAGIC);
        w(c, OC1042_STEER, r.next_u32());
        // World -> +0x84 -> +0x20: the collision context.
        w(c, GAME_GP.wrapping_sub(0x848), OC1042_WORLD);
        w(c, OC1042_WORLD + 0x84, OC1042_WORLD2);
        w(c, OC1042_WORLD2 + 0x20, OC1042_CTX);
        // Sub-steps per frame and the "bounce on contact" flag.
        w(c, GAME_GP + 0x9d8, 1 + seed % 3);
        w(c, GAME_GP + 0x9dc, (seed / 3) % 2);
        // The node: source (a vec4 at +0x40), length, fade, radius, type.
        for i in 0..4 {
            w(c, OC1042_SRC + 0x40 + 4 * i, r.f32(500.0));
        }
        w(c, OC1042_NODE + 0xf0, (seed / 2) % 2);
        w(c, OC1042_NODE + 0xf8, OC1042_SRC);
        let length = [1.0f32, 3.0, 4.0, 2.5, 5.0][(seed % 5) as usize];
        let fade = [1.0f32, 2.0, 3.0][((seed / 5) % 3) as usize];
        let radius = [100.0f32, 30.0, 400.0, 200.0][((seed / 7) % 4) as usize];
        w(c, OC1042_NODE + 0x100, length.to_bits());
        w(c, OC1042_NODE + 0x108, fade.to_bits());
        w(c, OC1042_NODE + 0x110, radius.to_bits());
        // The avalanche table: object count (< 32 or full), event count, rows.
        let count = [0u32, 1, 3, 31, 32][((seed / 4) % 5) as usize];
        c.bus.write16(OC1042_AV + 0xc, count as u16);
        c.bus.write16(
            OC1042_AV + 0xe,
            [0u16, 5, 63, 64][((seed / 9) % 4) as usize],
        );
        let obj = if seed % 6 == 0 {
            0
        } else {
            OC1042_OBJ + 0x40 * (seed % 3)
        };
        for k in 0..count.min(32) {
            // Row k holds the hit object when `seed / 3` is even and k == 0.
            let o = if k == 0 && (seed / 3) % 2 == 0 {
                obj
            } else {
                OC1042_OBJ + 0x400 + 4 * k
            };
            w(c, OC1042_AV + 0x10 + 8 * k, r.next_u32());
            w(c, OC1042_AV + 0x14 + 8 * k, o);
        }
        // The surface test of the hit triangle: a table entry of kind 6 and
        // a record of kind 3 (some seeds fail one of them).
        w(c, GAME_GP + 0x16c8, OC1042_TBL);
        w(c, OC1042_TBL, OC1042_BASE);
        let tri_ok = seed % 4 != 1;
        c.bus.write8(OC1042_TRI + 0x155, 1);
        c.bus
            .write16(OC1042_TRI + 0x156, if seed % 7 == 3 { 0xffff } else { 2 });
        w(c, OC1042_BASE + 8 + 0x24, if seed % 5 == 2 { 4 } else { 6 });
        w(
            c,
            OC1042_BASE + 0x30 + 0x3f0,
            if seed % 3 == 2 { 1 } else { 3 },
        );
        // The stack frame below the call: a random hit record (palette of
        // zero, -1, small floats and random bits), then the pieces the
        // faked callees would have filled.
        let sp0 = (c.g[SP][0] as u32).wrapping_sub(0x3e0);
        for i in 0..0xf8u32 {
            let v = match r.next_u32() % 12 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                3 => 1.0f32.to_bits(),
                4 => (-1.0f32).to_bits(),
                5 => 0.5f32.to_bits(),
                6 => r.f32(2.0),
                7 => r.f32(300.0),
                _ => r.next_u32(),
            };
            w(c, sp0 + 4 * i, v);
        }
        w(c, sp0 + 0xf0, obj);
        w(c, sp0 + 0xf4, if tri_ok { OC1042_TRI } else { 0 });
        c.g[A0][0] = sx(OC1042_NODE);
        c.g[A1][0] = sx(OC1042_AV);
        r
    }

    /// `Avalanche_BakeNodeHistory`: one to five frames of one to three
    /// sub-steps; contact or not, object already in the table or not (and the
    /// table full), bounce flag, both node types, surface record valid or not,
    /// fakes that unwind after any call.
    #[test]
    fn avalanche_bake_node_history_matches() {
        covered(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            300,
            &|c, seed| {
                oc1042_setup(c, seed, 60);
            },
        );
    }

    /// The call arms the seeds reach (return addresses of logged calls).
    #[test]
    fn avalanche_bake_node_history_reaches_every_call() {
        fakes::check_reaching(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            300,
            &[
                0x002d_675c,
                0x002d_6778,
                0x002d_67dc,
                0x002d_67f8,
                0x002d_6968,
                0x002d_6988,
                0x002d_699c,
                0x002d_69d4,
                0x002d_6cac,
                0x002d_6e3c,
                0x002d_6e50,
                0x002d_6ebc,
                0x002d_6ed0,
                0x002d_7184,
                0x002d_720c,
                0x002d_74d4,
                0x002d_7680,
                0x002d_770c,
                0x002d_7904,
                0x002d_7938,
                0x002d_7988,
                0x002d_7a78,
                0x002d_7b94,
                0x002d_7bac,
            ],
            &|c, seed| {
                oc1042_setup(c, seed, 0);
            },
        );
    }

    /// The seeds make many different paths (distinct cycle totals).
    #[test]
    fn avalanche_bake_node_history_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc1042_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s09::f_002d66a0__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 40, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn avalanche_bake_node_history_events() {
        check_events(
            ee::s09::f_002d66a0__generated,
            avalanche_bake_node_history,
            120,
            &|c, seed| {
                let mut r = oc1042_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x002d_66a0)
            },
        );
    }
}
