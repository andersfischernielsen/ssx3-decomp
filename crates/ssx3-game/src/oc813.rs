//! Staging module oc813 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc813`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_4040, ea_file_cd_init, "EAFile_CdInit"),
    (0x0042_7a90, sce_fs_init, "sceFsInit"),
];

/// `EAFileCdInit` (0x003e4040): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ea_file_cd_init(c: &mut Cpu) {
    if c.pc != 0x003e4040 {
        return ee::s14::f_003e4040__generated(c);
    }
    guest(c, |c| {
        'b_3e4480: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1210));
            c.g[V0][0] = 0x450000;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1180));
            c.g[A2][0] = 0x40;
            c.g[FP][0] = c.g[A0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1160), c.g[A1][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1200));
            c.g[A0][0] = 0x520000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x11c0));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x11f0));
            c.g[A1][0] = 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63c0));
            c.g[S4][0] = c.g[V0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x11e0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x11d0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x11b0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x11a0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1190));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x1170), c.g[RA][0]);
            c.g[RA][0] = sx(0x003e4098);
            c.g[S1][0] = 0u64;
            Callee {
                addr: 0x003e6448,
                f: ee::s14::EA_MemSet,
            }
            .call(c, 0x003e4098)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
            c.mult(0, 0, 6, 30);
            c.g[A2][0] = c.lo[0];
            c.g[RA][0] = sx(0x003e40b4);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1160));
            Callee {
                addr: 0x003e6448,
                f: ee::s14::EA_MemSet,
            }
            .call(c, 0x003e40b4)?;
            c.g[RA][0] = sx(0x003e40bc);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x00401718, 0x003e40bc)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_3e4480;
            } else {
                'b_3e40fc: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x003e40cc);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00402450, 0x003e40cc)?;
                    c.g[RA][0] = sx(0x003e40d4);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x00401bf0, 0x003e40d4)?;
                    c.g[RA][0] = sx(0x003e40dc);
                    call_ptr(c, 0x00402268, 0x003e40dc)?;
                    c.g[V1][0] = 0x12;
                    if c.g[V0][0] == c.g[V1][0] {
                        c.g[S1][0] = 0x1;
                        break 'b_3e40fc;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x003e40f0);
                        call_ptr(c, 0x00402268, 0x003e40f0)?;
                        c.g[V1][0] = 0x14;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.g[S1][0] = 0x1;
                            break 'b_3e40fc;
                        } else {
                            break 'b_3e40fc;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_3e4480;
                } else {
                    'b_3e4354: {
                        'b_3e4350: {
                            'b_3e4260: {
                                c.cycles += 33;
                                c.g[V0][0] = 0x520000;
                                c.g[A0][0] = 0u64;
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                                c.g[RA][0] = sx(0x003e4118);
                                c.g[S1][0] = 0x450000;
                                call_ptr(c, 0x00401bf0, 0x003e4118)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
                                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                c.g[RA][0] = sx(0x003e4128);
                                c.g[S2][0] = 0x450000;
                                call_ptr(c, 0x00401fd8, 0x003e4128)?;
                                c.g[S7][0] = c.g[S2][0];
                                c.g[V1][0] = 0x10;
                                c.g[V0][0] = 0x1;
                                c.g[A0][0] = 0u64;
                                c.sw((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                                c.sb((c.g[S3][0] as u32).wrapping_add(0x1d), c.g[V0][0]);
                                c.sb((c.g[S3][0] as u32).wrapping_add(0x1c), 0u64);
                                c.g[RA][0] = sx(0x003e414c);
                                c.sb((c.g[S3][0] as u32).wrapping_add(0x1e), 0u64);
                                call_ptr(c, 0x00424020, 0x003e414c)?;
                                c.g[S0][0] = 0x108;
                                c.g[A0][0] = 0x490000;
                                c.mult(0, 16, 30, 16);
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5de0));
                                c.g[A2][0] = 0u64;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003e416c);
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, t, 0x003e416c)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xc68), c.g[V0][0]);
                                c.g[RA][0] = sx(0x003e417c);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x003e6220,
                                    f: ee::s14::EA_MemZero,
                                }
                                .call(c, 0x003e417c)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V0][0] == 0u64;
                                c.g[S0][0] = c.g[V0][0];
                                if t {
                                    break 'b_3e4260;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[V0][0] = c.lb(c.g[V1][0] as u32);
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                                    if t {
                                        break 'b_3e4350;
                                    } else {
                                        c.cycles += 4;
                                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1050));
                                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1150));
                                        c.g[S6][0] = 0x450000;
                                        'l_3e41a8: loop {
                                            'b_3e41f4: {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[S5][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x108));
                                                c.g[S4][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x003e41c0);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x7));
                                                call_ptr(c, 0x004166f4, 0x003e41c0)?;
                                                c.g[V1][0] =
                                                    c.lb((c.g[SP][0] as u32).wrapping_add(0x1050));
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_3e41f4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = 0x5c;
                                                    c.g[V1][0] = 0x2f;
                                                    'l_3e41d8: loop {
                                                        'b_3e41e4: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                                            if c.g[V0][0] == c.g[A1][0] {
                                                                c.sb(c.g[A0][0] as u32, c.g[V1][0]);
                                                                break 'b_3e41e4;
                                                            } else {
                                                                break 'b_3e41e4;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003e41d8);
                                                            }
                                                            continue 'l_3e41d8;
                                                        } else {
                                                            break 'b_3e41f4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 25;
                                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                            c.g[RA][0] = sx(0x003e4200);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x004166f4, 0x003e4200)?;
                                            c.g[RA][0] = sx(0x003e4208);
                                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                            call_ptr(c, 0x00416810, 0x003e4208)?;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x003e421c);
                                            c.sb((c.g[V0][0] as u32).wrapping_sub(0x2), 0u64);
                                            call_ptr(c, 0x004162d0, 0x003e421c)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x003e4228);
                                            c.g[A1][0] = c.g[S2][0];
                                            call_ptr(c, 0x004166f4, 0x003e4228)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x100),
                                                c.g[V1][0],
                                            );
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[S0][0] = c.g[S4][0];
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x104),
                                                c.g[V1][0],
                                            );
                                            c.sw(
                                                (c.g[S6][0] as u32).wrapping_add(0xc6c),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[S1][0] = c.g[S5][0];
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003e41a8);
                                                }
                                                continue 'l_3e41a8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] = 0x450000;
                                                if t {
                                                    break 'b_3e4354;
                                                } else {
                                                    break 'b_3e4260;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            'b_3e4334: {
                                c.cycles += 13;
                                c.g[RA][0] = sx(0x003e4268);
                                Callee {
                                    addr: 0x003e3618,
                                    f: ee::s14::EAFile_CdReset,
                                }
                                .call(c, 0x003e4268)?;
                                c.g[V1][0] = 0x520000;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1380));
                                c.g[A0][0] = 0x2f;
                                c.sb((c.g[V0][0] as u32).wrapping_add(0x1), 0u64);
                                c.g[S0][0] = c.g[V1][0];
                                c.sb((c.g[V1][0] as u32).wrapping_sub(0x1380), c.g[A0][0]);
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                                let t = c.g[V1][0] == 0u64;

                                if t {
                                    break 'b_3e4334;
                                } else {
                                    c.cycles += 10;
                                    c.lwl(2, (c.g[A2][0] as u32).wrapping_add(0xa1));
                                    c.lwr(2, (c.g[A2][0] as u32).wrapping_add(0x9e));
                                    c.g[RA][0] = sx(0x003e42a4);
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                    Callee {
                                        addr: 0x003e3618,
                                        f: ee::s14::EAFile_CdReset,
                                    }
                                    .call(c, 0x003e42a4)?;
                                    c.g[V1][0] = 0x520000;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5b80));
                                    c.g[V0][0] = (c.g[V1][0] < c.g[S1][0]) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = 0x450000;
                                    if t {
                                        break 'b_3e4354;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S2][0] = c.g[V1][0];
                                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                                        'l_3e42c8: loop {
                                            'b_3e42f4: {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x003e42d0);
                                                c.g[A2][0] =
                                                    c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                                                call_ptr(c, 0x00416b18, 0x003e42d0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0xc58));
                                                if t {
                                                    break 'b_3e42f4;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                                                    c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.g[V1][0] = (c.g[S2][0] < c.g[S1][0]) as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A0][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x21));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x003e42c8);
                                                        }
                                                        continue 'l_3e42c8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = 0x450000;
                                                        if t {
                                                            break 'b_3e4354;
                                                        } else {
                                                            break 'b_3e42f4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 16;
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_sub(0x1380));
                                            c.g[RA][0] = sx(0x003e4300);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x004162d0, 0x003e4300)?;
                                            c.g[A1][0] = 0x490000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x003e4310);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x5df0));
                                            call_ptr(c, 0x004162d0, 0x003e4310)?;
                                            c.lwl(5, (c.g[S1][0] as u32).wrapping_add(0xd));
                                            c.lwr(5, (c.g[S1][0] as u32).wrapping_add(0xa));
                                            c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                                            c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
                                            c.g[RA][0] = sx(0x003e432c);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
                                            Callee {
                                                addr: 0x003e3758,
                                                f: ee::s14::EAFile_CdScanDir,
                                            }
                                            .call(c, 0x003e432c)?;
                                            let t = true;
                                            c.g[A0][0] = 0x450000;
                                            if t {
                                                break 'b_3e4354;
                                            } else {
                                                break 'b_3e4334;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.lwl(5, (c.g[A2][0] as u32).wrapping_add(0xa9));
                            c.lwr(5, (c.g[A2][0] as u32).wrapping_add(0xa6));
                            c.lwl(4, (c.g[A2][0] as u32).wrapping_add(0xa1));
                            c.lwr(4, (c.g[A2][0] as u32).wrapping_add(0x9e));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
                            c.g[RA][0] = sx(0x003e4350);
                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
                            Callee {
                                addr: 0x003e3758,
                                f: ee::s14::EAFile_CdScanDir,
                            }
                            .call(c, 0x003e4350)?;
                            break 'b_3e4350;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = 0x450000;
                        break 'b_3e4354;
                    }
                    'b_3e4460: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc6c));
                        c.g[V0][0] = (c.g[FP][0] < c.g[V1][0]) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = 0x450000;
                            break 'b_3e4460;
                        } else {
                            c.cycles += 63;
                            c.g[A0][0] = 0x520000;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1160));
                            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x63c0));
                            c.g[V0][0] = 0x450000;
                            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc70));
                            c.g[A2][0] = c.g[V1][0];
                            c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                            c.g[RA][0] = sx(0x003e438c);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            Callee {
                                addr: 0x003e39a8,
                                f: ee::s14::EAFile_CdBuildHashTable,
                            }
                            .call(c, 0x003e438c)?;
                            c.g[A0][0] = 0x450000;
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc10));
                            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                            let t = c.g[A1][0] as u32;
                            c.g[RA][0] = sx(0x003e43a4);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            call_ptr(c, t, 0x003e43a4)?;
                            c.g[V0][0] = 0x450000;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc6c), 0u64);
                            c.g[V0][0] = 0x20;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                            c.g[RA][0] = sx(0x003e43c4);
                            c.sw((c.g[S7][0] as u32).wrapping_add(0xc68), 0u64);
                            call_ptr(c, 0x00423da0, 0x003e43c4)?;
                            c.g[V1][0] = 0x3e0000;
                            c.g[A1][0] = 0x520000;
                            c.g[A2][0] = 0x4a0000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3be0));
                            c.g[A3][0] = 0x4000;
                            c.g[T0][0] = 0xc;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30f0));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5380));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[A0][0] = c.g[SP][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A3][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T0][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            c.g[RA][0] = sx(0x003e4408);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                            call_ptr(c, 0x00423ba0, 0x003e4408)?;
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x003e4418);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            call_ptr(c, 0x00423bc0, 0x003e4418)?;
                            c.g[A1][0] = 0x520000;
                            c.g[A2][0] = 0x800;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5b80));
                            c.g[RA][0] = sx(0x003e442c);
                            c.g[A0][0] = 0xb;
                            call_ptr(c, 0x00400a78, 0x003e442c)?;
                            c.g[A0][0] = 0x3e0000;
                            c.g[RA][0] = sx(0x003e4438);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ad8));
                            call_ptr(c, 0x004008a0, 0x003e4438)?;
                            c.g[V1][0] = 0x520000;
                            c.g[A0][0] = 0x3e0000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4000));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x003e4458);
                            c.g[A2][0] = 0u64;
                            Callee {
                                addr: 0x003e57f8,
                                f: ee::s14::EATimer_AddCallback,
                            }
                            .call(c, 0x003e4458)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_3e4480;
                            } else {
                                break 'b_3e4460;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e4470);
                    c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc68));
                    call_ptr(c, t, 0x003e4470)?;
                    c.g[V1][0] = 0x450000;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0xc68), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc6c), 0u64);
                    c.g[V0][0] = 0u64;
                    break 'b_3e4480;
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1200));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x11f0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x11e0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x11d0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x11c0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x11b0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x11a0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1190));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1180));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1170));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1210));
        c.pc = t;
        return Ok(());
    });
}

/// `SceFsInit` (0x00427a90): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sce_fs_init(c: &mut Cpu) {
    if c.pc != 0x00427a90 {
        return ee::s15::f_00427a90__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00427a90;
        loop {
            match pc {
                0x00427a90 => {
                    c.cycles += 30;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[A0][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
                    c.g[S0][0] = 0x530000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1180));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
                    c.g[RA][0] = sx(0x00427abc);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
                    Callee {
                        addr: 0x00426408,
                        f: ee::s15::sceSifInitRpc,
                    }
                    .call(c, 0x00427abc)?;
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x1180), 0u64);
                    c.g[RA][0] = sx(0x00427ac8);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    Callee {
                        addr: 0x0042c078,
                        f: ee::s15::DIntr,
                    }
                    .call(c, 0x00427ac8)?;
                    c.g[A1][0] = 0x420000;
                    c.g[A2][0] = 0x530000;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7590));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x11c0));
                    c.g[RA][0] = sx(0x00427ae8);
                    c.g[A0][0] = c.g[A0][0] | 0x11;
                    Callee {
                        addr: 0x00426020,
                        f: ee::s15::sceSifAddCmdHandler,
                    }
                    .call(c, 0x00427ae8)?;
                    c.g[A1][0] = 0x420000;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a60));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00427b00);
                    c.g[A0][0] = c.g[A0][0] | 0x13;
                    Callee {
                        addr: 0x00426020,
                        f: ee::s15::sceSifAddCmdHandler,
                    }
                    .call(c, 0x00427b00)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S3][0] = 0x530000;
                    if t {
                        pc = 0x00427b3c;
                        continue;
                    }
                    pc = 0x00427b08;
                    continue;
                }
                0x00427b08 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427b10);
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    Callee {
                        addr: 0x0042c0c0,
                        f: ee::s15::EIntr,
                    }
                    .call(c, 0x00427b10)?;
                    let t = true;

                    if t {
                        pc = 0x00427b40;
                        continue;
                    }
                    pc = 0x00427b18;
                    continue;
                }
                0x00427b18 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffffffffffffffff;
                    pc = 0x00427b20;
                    continue;
                }
                0x00427b20 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];

                    if t {
                        if back_edge_unwinds(c, 0x00427b20) {
                            return Ok(());
                        }
                        pc = 0x00427b20;
                        continue;
                    }
                    pc = 0x00427b3c;
                    continue;
                }
                0x00427b3c => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    pc = 0x00427b40;
                    continue;
                }
                0x00427b40 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffffffff80000000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[A1][0] | 0x1;
                    c.g[RA][0] = sx(0x00427b54);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00426b48,
                        f: ee::s15::sceSifBindRpc,
                    }
                    .call(c, 0x00427b54)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x00427c74;
                        continue;
                    }
                    pc = 0x00427b5c;
                    continue;
                }
                0x00427b5c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x100000;
                    if t {
                        if back_edge_unwinds(c, 0x00427b18) {
                            return Ok(());
                        }
                        pc = 0x00427b18;
                        continue;
                    }
                    pc = 0x00427b68;
                    continue;
                }
                0x00427b68 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00427b70);
                    c.g[S4][0] = 0x450000;
                    Callee {
                        addr: 0x00427438,
                        f: ee::s15::sceFs_FdSemaInit,
                    }
                    .call(c, 0x00427b70)?;
                    c.g[RA][0] = sx(0x00427b78);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    Callee {
                        addr: 0x00423de0,
                        f: ee::s15::WaitSema,
                    }
                    .call(c, 0x00427b78)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1400));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x200));
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0x530000;
                    if t {
                        pc = 0x00427bbc;
                        continue;
                    }
                    pc = 0x00427b90;
                    continue;
                }
                0x00427b90 => {
                    c.cycles += 2;
                    c.g[S1][0] = 0x530000;
                    c.g[S0][0] = 0x530000;
                    pc = 0x00427b98;
                    continue;
                }
                0x00427b98 => {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x00427b98) {
                            return Ok(());
                        }
                        pc = 0x00427b98;
                        continue;
                    }
                    pc = 0x00427bb4;
                    continue;
                }
                0x00427bb4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    if t {
                        pc = 0x00427bc8;
                        continue;
                    }
                    pc = 0x00427bbc;
                    continue;
                }
                0x00427bbc => {
                    c.cycles += 3;
                    c.g[S1][0] = 0x530000;
                    c.g[S0][0] = 0x530000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    pc = 0x00427bc8;
                    continue;
                }
                0x00427bc8 => {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x00427bd0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1cc0));
                    Callee {
                        addr: 0x00423dc0,
                        f: ee::s15::SignalSema,
                    }
                    .call(c, 0x00427bd0)?;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1c80));
                    c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x2940));
                    c.sw((c.g[S2][0] as u32).wrapping_sub(0x2940), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x440));
                    c.g[A1][0] = 0xff;
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A2][0] = 0u64;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[T0][0] = 0x8;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x8;
                    c.g[RA][0] = sx(0x00427c08);
                    c.g[T3][0] = 0u64;
                    Callee {
                        addr: 0x00426d18,
                        f: ee::s15::sceSifCallRpc,
                    }
                    .call(c, 0x00427c08)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = 0x20000000;
                    if t {
                        pc = 0x00427c1c;
                        continue;
                    }
                    pc = 0x00427c10;
                    continue;
                }
                0x00427c10 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    if t {
                        pc = 0x00427c74;
                        continue;
                    }
                    pc = 0x00427c1c;
                    continue;
                }
                0x00427c1c => {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.g[S0][0] | c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[A1][0] = 0x530000;
                    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x11d8));
                    c.lwl(4, (c.g[V1][0] as u32).wrapping_add(0x3));
                    c.lwr(4, c.g[V1][0] as u32);
                    c.swl(4, (c.g[T0][0] as u32).wrapping_add(0x3));
                    c.swr(4, c.g[T0][0] as u32);
                    c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0x3));
                    c.lwr(3, c.g[V0][0] as u32);
                    c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x13));
                    c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = 0x450000;
                    c.g[A1][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x520c), c.g[A0][0]);
                    c.g[V0][0] = 0u64;
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x5210), c.g[V1][0]);
                    pc = 0x00427c74;
                    continue;
                }
                0x00427c74 => {
                    c.cycles += 8;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00427a90, 0x00427c94);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, fresh, put_words, run_generated, DATA, SP as STACK_TOP};

    /// Gate word of this module's fakes (in the machine's RAM) and its magic.
    pub(crate) const OC813_GATE: u32 = DATA + 0x20_0000;
    pub(crate) const OC813_MAGIC: u32 = 0x4f43_3831;
    /// Fixture words the fakes read (steering the paths of the callers).
    pub(crate) const OC813_W: u32 = DATA + 0x20_0100;
    pub(crate) const OC813_INIT_RES: u32 = OC813_W;
    pub(crate) const OC813_DISK_VALS: u32 = OC813_W + 0x4;
    pub(crate) const OC813_DISK_CNT: u32 = OC813_W + 0xc;
    pub(crate) const OC813_STRLEN_VAL: u32 = OC813_W + 0x10;
    pub(crate) const OC813_CDRESET_VAL: u32 = OC813_W + 0x14;
    pub(crate) const OC813_STRNCMP_MISS: u32 = OC813_W + 0x18;
    pub(crate) const OC813_STRNCMP_CNT: u32 = OC813_W + 0x1c;
    pub(crate) const OC813_DINTR_VAL: u32 = OC813_W + 0x20;
    pub(crate) const OC813_BIND_RES: u32 = OC813_W + 0x24;
    pub(crate) const OC813_CALL_RES: u32 = OC813_W + 0x28;
    pub(crate) const OC813_BIND_LEFT: u32 = OC813_W + 0x2c;
    pub(crate) const OC813_STR_EMPTY: u32 = DATA + 0x20_0800;
    pub(crate) const OC813_STR_NAME: u32 = DATA + 0x20_0810;
    pub(crate) const OC813_LIST: u32 = DATA + 0x20_0900;
    pub(crate) const OC813_CD_BUF: u32 = DATA + 0x20_0400;
    /// Region of the "disc directory records" (below the scan limit 0x51a480).
    pub(crate) const OC813_RECORDS: u32 = 0x0051_a000;

    fn oc813_nop(_c: &mut Cpu, _a0: u32) {}

    /// A gated fake: with the gate set it logs the call like `fakes::fake`
    /// and then applies `post` (a forced result); with it off, the body that
    /// was in the slot runs.
    macro_rules! oc813_fake {
        ($name:ident, $generated:path, $target:expr, $post:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC813_GATE) != OC813_MAGIC {
                    return crate::difftest::fakes::fall_back(c, prev_cell($name), $generated);
                }
                let a0 = c.g[A0][0] as u32;
                let ra = c.g[RA][0] as u32;
                fake(c, $target, ra);
                if c.pc == ra {
                    let post: fn(&mut Cpu, u32) = $post;
                    post(c, a0);
                }
            }
        };
    }

    fn oc813_post_cd_init(c: &mut Cpu, _a0: u32) {
        if c.bus.read32(OC813_INIT_RES) == 0 {
            c.g[V0][0] = 0;
        }
    }

    fn oc813_post_disk_type(c: &mut Cpu, _a0: u32) {
        let n = c.bus.read32(OC813_DISK_CNT);
        c.bus.write32(OC813_DISK_CNT, n + 1);
        c.g[V0][0] = c.bus.read32(OC813_DISK_VALS + 4 * n.min(1)) as u64;
    }

    fn oc813_post_strlen(c: &mut Cpu, _a0: u32) {
        c.g[V0][0] = c.bus.read32(OC813_STRLEN_VAL) as u64;
    }

    fn oc813_post_strncmp(c: &mut Cpu, _a0: u32) {
        let n = c.bus.read32(OC813_STRNCMP_CNT);
        c.bus.write32(OC813_STRNCMP_CNT, n + 1);
        c.g[V0][0] = (n < c.bus.read32(OC813_STRNCMP_MISS)) as u64;
    }

    fn oc813_post_cd_reset(c: &mut Cpu, _a0: u32) {
        c.g[V0][0] = c.bus.read32(OC813_CDRESET_VAL) as u64;
    }

    fn oc813_post_dintr(c: &mut Cpu, _a0: u32) {
        c.g[V0][0] = c.bus.read32(OC813_DINTR_VAL) as u64;
    }

    /// `sceSifBindRpc`: the client's server word (`+0x24`) stays 0 for
    /// `OC813_BIND_LEFT` attempts, then becomes non-zero.
    fn oc813_post_bind(c: &mut Cpu, a0: u32) {
        c.g[V0][0] = c.bus.read32(OC813_BIND_RES) as i32 as i64 as u64;
        let left = c.bus.read32(OC813_BIND_LEFT);
        if left > 0 {
            c.bus.write32(OC813_BIND_LEFT, left - 1);
            c.bus.write32(a0.wrapping_add(0x24), 0);
        } else {
            c.bus.write32(a0.wrapping_add(0x24), 1);
        }
    }

    fn oc813_post_call_rpc(c: &mut Cpu, _a0: u32) {
        c.g[V0][0] = c.bus.read32(OC813_CALL_RES) as i32 as i64 as u64;
    }

    oc813_fake!(
        oc813_f_cd_init,
        ee::s15::f_00401718__generated,
        0x0040_1718,
        oc813_post_cd_init
    );
    oc813_fake!(
        oc813_f_cd_mmode,
        ee::s15::f_00402450__generated,
        0x0040_2450,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_disk_ready,
        ee::s15::f_00401bf0__generated,
        0x0040_1bf0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_disk_type,
        ee::s15::f_00402268__generated,
        0x0040_2268,
        oc813_post_disk_type
    );
    oc813_fake!(
        oc813_f_get_toc,
        ee::s15::f_00401fd8__generated,
        0x0040_1fd8,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_flush,
        ee::s15::f_00424020__generated,
        0x0042_4020,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_strcpy,
        ee::s15::f_004166f4__generated,
        0x0041_66f4,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_strlen,
        ee::s15::f_00416810__generated,
        0x0041_6810,
        oc813_post_strlen
    );
    oc813_fake!(
        oc813_f_strcat,
        ee::s15::f_004162d0__generated,
        0x0041_62d0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_strncmp,
        ee::s15::f_00416b18__generated,
        0x0041_6b18,
        oc813_post_strncmp
    );
    oc813_fake!(
        oc813_f_mem_zero,
        ee::s14::f_003e6220__generated,
        0x003e_6220,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_mem_set,
        ee::s14::f_003e6448__generated,
        0x003e_6448,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_cd_reset,
        ee::s14::f_003e3618__generated,
        0x003e_3618,
        oc813_post_cd_reset
    );
    oc813_fake!(
        oc813_f_scan_dir,
        ee::s14::f_003e3758__generated,
        0x003e_3758,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_hash,
        ee::s14::f_003e39a8__generated,
        0x003e_39a8,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_start_thread,
        ee::s15::f_00423bc0__generated,
        0x0042_3bc0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_cd_ee_cb,
        ee::s15::f_00400a78__generated,
        0x0040_0a78,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_cd_callback,
        ee::s15::f_004008a0__generated,
        0x0040_08a0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_add_callback,
        ee::s14::f_003e57f8__generated,
        0x003e_57f8,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_dintr,
        ee::s15::f_0042c078__generated,
        0x0042_c078,
        oc813_post_dintr
    );
    oc813_fake!(
        oc813_f_eintr,
        ee::s15::f_0042c0c0__generated,
        0x0042_c0c0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_add_cmd,
        ee::s15::f_00426020__generated,
        0x0042_6020,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_bind,
        ee::s15::f_00426b48__generated,
        0x0042_6b48,
        oc813_post_bind
    );
    oc813_fake!(
        oc813_f_fd_sema,
        ee::s15::f_00427438__generated,
        0x0042_7438,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_wait_sema,
        ee::s15::f_00423de0__generated,
        0x0042_3de0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_signal_sema,
        ee::s15::f_00423dc0__generated,
        0x0042_3dc0,
        oc813_nop
    );
    oc813_fake!(
        oc813_f_call_rpc,
        ee::s15::f_00426d18__generated,
        0x0042_6d18,
        oc813_post_call_rpc
    );

    macro_rules! oc813_hook {
        ($slot:path, $fake:ident) => {
            hook(std::ptr::addr_of_mut!($slot), $fake, prev_cell($fake));
        };
    }

    /// Put every fake of this module in its override slot (once per process;
    /// they do nothing while the gate is off).
    pub(crate) fn oc813_hook_all() {
        oc813_hook!(ee::s15::OVERRIDE_00401718, oc813_f_cd_init);
        oc813_hook!(ee::s15::OVERRIDE_00402450, oc813_f_cd_mmode);
        oc813_hook!(ee::s15::OVERRIDE_00401BF0, oc813_f_disk_ready);
        oc813_hook!(ee::s15::OVERRIDE_00402268, oc813_f_disk_type);
        oc813_hook!(ee::s15::OVERRIDE_00401FD8, oc813_f_get_toc);
        oc813_hook!(ee::s15::OVERRIDE_00424020, oc813_f_flush);
        oc813_hook!(ee::s15::OVERRIDE_004166F4, oc813_f_strcpy);
        oc813_hook!(ee::s15::OVERRIDE_00416810, oc813_f_strlen);
        oc813_hook!(ee::s15::OVERRIDE_004162D0, oc813_f_strcat);
        oc813_hook!(ee::s15::OVERRIDE_00416B18, oc813_f_strncmp);
        oc813_hook!(ee::s14::OVERRIDE_003E6220, oc813_f_mem_zero);
        oc813_hook!(ee::s14::OVERRIDE_003E6448, oc813_f_mem_set);
        oc813_hook!(ee::s14::OVERRIDE_003E3618, oc813_f_cd_reset);
        oc813_hook!(ee::s14::OVERRIDE_003E3758, oc813_f_scan_dir);
        oc813_hook!(ee::s14::OVERRIDE_003E39A8, oc813_f_hash);
        oc813_hook!(ee::s15::OVERRIDE_00423BC0, oc813_f_start_thread);
        oc813_hook!(ee::s15::OVERRIDE_00400A78, oc813_f_cd_ee_cb);
        oc813_hook!(ee::s15::OVERRIDE_004008A0, oc813_f_cd_callback);
        oc813_hook!(ee::s14::OVERRIDE_003E57F8, oc813_f_add_callback);
        oc813_hook!(ee::s15::OVERRIDE_0042C078, oc813_f_dintr);
        oc813_hook!(ee::s15::OVERRIDE_0042C0C0, oc813_f_eintr);
        oc813_hook!(ee::s15::OVERRIDE_00426020, oc813_f_add_cmd);
        oc813_hook!(ee::s15::OVERRIDE_00426B48, oc813_f_bind);
        oc813_hook!(ee::s15::OVERRIDE_00427438, oc813_f_fd_sema);
        oc813_hook!(ee::s15::OVERRIDE_00423DE0, oc813_f_wait_sema);
        oc813_hook!(ee::s15::OVERRIDE_00423DC0, oc813_f_signal_sema);
        oc813_hook!(ee::s15::OVERRIDE_00426D18, oc813_f_call_rpc);
    }

    const OC813_SEEDS: u32 = 240;

    /// `EAFile_CdInit(max_files, table)`: boots the CD driver (`sceCdInit`,
    /// disc type 0x12 or 0x14), reads the disc directory list (or the ISO
    /// root records), builds the file table and starts the CD callbacks.
    /// The fixture steers each stage: `sceCdInit` result, the two
    /// `sceCdGetDiskType` answers, the directory list (none, empty, 1-2
    /// entries, with a `\` path), the root record scan (hit after 0, 2 or
    /// never) and the entry count against `max_files`.
    pub(crate) fn oc813_cd_init_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 813_001, 34);
        oc813_hook_all();
        c.bus.write32(OC813_GATE, OC813_MAGIC);
        c.pc = 0x003e_4040;
        c.g[A0][0] = (seed % 4) as u64;
        c.bus.write32(OC813_INIT_RES, (seed % 13 != 0) as u32);
        let disk: [u32; 2] = match seed % 7 {
            1 | 4 => [0x13, 0x14],
            2 => [0x13, 0x15],
            5 => [0x14, 0x14],
            _ => [0x12, 0],
        };
        put_words(c, OC813_DISK_VALS, &disk);
        c.bus.write32(OC813_STRLEN_VAL, 4 + seed % 3);
        let cdreset = if (seed / 2) % 2 == 0 {
            OC813_RECORDS
        } else {
            OC813_CD_BUF
        };
        c.bus.write32(OC813_CDRESET_VAL, cdreset);
        c.bus
            .write32(OC813_STRNCMP_MISS, [0, 2, 40][((seed / 7) % 3) as usize]);
        // Disc root records: length 0x40, name length 3.
        for i in 0..24 {
            let rec = OC813_RECORDS + 0x40 * i;
            c.bus.write32(rec, 0x40);
            c.bus.write32(rec + 0x20, 3);
        }
        c.bus.write32(OC813_CD_BUF, 0x40);
        // Globals of the file system (0x450c10 table, 0x450c58 config).
        c.bus.write32(0x0045_0c14, FAKE + 0x10);
        c.bus.write32(0x0045_0c18, FAKE + 0x20);
        c.bus.write32(0x0045_0c5c, 4 * (seed % 3));
        c.bus.write32(0x0045_0c6c, (seed / 11) % 3);
        c.bus.write32(
            0x0045_0c58,
            if (seed / 5) % 2 == 0 {
                OC813_STR_EMPTY
            } else {
                OC813_STR_NAME
            },
        );
        c.bus.write32(OC813_STR_NAME, 0x0043_4241);
        let kind = (seed / 3) % 4;
        if kind == 0 {
            c.bus.write32(0x0045_0c30, 0);
        } else {
            c.bus.write32(0x0045_0c30, OC813_LIST);
            let n = kind - 1;
            for i in 0..n {
                put_words(
                    c,
                    OC813_LIST + 16 * i,
                    &[
                        OC813_STR_NAME,
                        seed.wrapping_mul(7) + i,
                        0x1000 + i,
                        0x20 + i,
                    ],
                );
            }
            c.bus.write32(OC813_LIST + 16 * n, OC813_STR_EMPTY);
        }
        // The path string strcpy would have left (fake: not written), with
        // backslashes for the conversion loop.
        if seed % 2 == 0 {
            put_words(c, STACK_TOP - 0x1c0, &[0x0062_5c61, 0]);
        }
    }

    #[test]
    fn ea_file_cd_init_matches() {
        check(
            ee::s14::f_003e4040__generated,
            ea_file_cd_init,
            OC813_SEEDS,
            &|c, seed| oc813_cd_init_setup(c, seed),
        );
    }

    /// The fixture reaches the early outs, the directory-list and root-scan
    /// branches and the full init: many distinct cycle totals.
    #[test]
    fn ea_file_cd_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..OC813_SEEDS {
            let mut c = fresh();
            oc813_cd_init_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003e4040__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 12,
            "only {} distinct paths: {seen:?}",
            seen.len()
        );
    }

    /// `sceFsInit`: brings up SIF RPC, binds the fs RPC server (retrying
    /// while the server word is 0), clears the fd table, creates the fd
    /// semaphore and asks the IOP for its version (`sceSifCallRpc`).
    pub(crate) fn oc813_fs_init_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 813_002, 12);
        oc813_hook_all();
        c.bus.write32(OC813_GATE, OC813_MAGIC);
        c.pc = 0x0042_7a90;
        // sceSifInitRpc already done (its flag), so the real callee is short.
        c.bus.write32(0x0045_5180, 1);
        c.bus.write32(OC813_DINTR_VAL, seed & 1);
        let case = (seed >> 1) % 5;
        c.bus
            .write32(OC813_BIND_RES, if case == 2 { u32::MAX } else { 0 });
        c.bus.write32(OC813_BIND_LEFT, (case == 4) as u32);
        c.bus
            .write32(OC813_CALL_RES, if case == 3 { u32::MAX } else { 0 });
        // The IOP's reply (uncached alias of 0x52e340): version word at +4.
        put_words(
            c,
            0x0052_e340,
            &[
                seed.wrapping_mul(0x9e37_79b9),
                if case == 0 { 2 } else { 3 },
            ],
        );
        c.bus.write32(0x0045_5218, 5 + seed % 3);
    }

    #[test]
    fn sce_fs_init_matches() {
        check(
            ee::s15::f_00427a90__generated,
            sce_fs_init,
            40,
            &|c, seed| oc813_fs_init_setup(c, seed),
        );
    }

    #[test]
    fn sce_fs_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..40 {
            let mut c = fresh();
            oc813_fs_init_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00427a90__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 6,
            "only {} distinct paths: {seen:?}",
            seen.len()
        );
    }
}
