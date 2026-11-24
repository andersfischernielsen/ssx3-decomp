//! Staging module oc482 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc482`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x00337a48, nash_find_nearest_rail, "Nash_FindNearestRail")];

/// `Nash_FindNearestRail` (0x00337a48): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn nash_find_nearest_rail(c: &mut Cpu) {
    if c.pc != 0x00337a48 {
        return ee::s11::f_00337a48__generated(c);
    }
    guest(c, |c| {
        'b_337c90: {
            'b_337c8c: {
                c.cycles += 40;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x54));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S5][0] = c.g[SP][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                c.lq(2, c.g[A1][0] as u32);
                c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(2, c.g[SP][0] as u32);
                c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.g[SP][0] as u32);
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[SP][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x54), 0u64);
                let t = c.g[S1][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x50), 0u64);
                if t {
                    break 'b_337c8c;
                } else {
                    'b_337c68: {
                        c.cycles += 7;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x58));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00337af8);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x00328360, 0x00337af8)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_337c68;
                        } else {
                            'b_337bc4: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_337bc4;
                                } else {
                                    c.cycles += 4;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_337b20: loop {
                                        'b_337bb8: {
                                            'b_337b50: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_337b50;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337b48);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337b48)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337bb8;
                                                    } else {
                                                        break 'b_337b50;
                                                    }
                                                }
                                            }
                                            'b_337b7c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_337b7c;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337b74);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337b74)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337bb8;
                                                    } else {
                                                        break 'b_337b7c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337bb8;
                                            } else {
                                                'b_337bb4: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_337bb4;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00337bb4);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x00337bb4)?;
                                                        break 'b_337bb4;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337bb8;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00337b20);
                                            }
                                            continue 'l_337b20;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_337bc4;
                                        }
                                    }
                                }
                            }
                            'b_337bd8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337bd8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337bd4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337bd4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337bd8;
                                }
                            }
                            'b_337bec: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337bec;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337be8);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337be8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337bec;
                                }
                            }
                            'b_337c00: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337c00;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337bfc);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337bfc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337c00;
                                }
                            }
                            'b_337c14: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337c14;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337c10);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337c10)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337c14;
                                }
                            }
                            'b_337c28: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337c28;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337c24);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337c24)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337c28;
                                }
                            }
                            'b_337c3c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337c3c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337c38);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337c38)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337c3c;
                                }
                            }
                            'b_337c50: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337c50;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337c4c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337c4c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337c50;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                break 'b_337c90;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00337c60);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00337c60)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                if t {
                                    break 'b_337c90;
                                } else {
                                    break 'b_337c68;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                        break 'b_337c90;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[A3][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00337c8c);
                        c.g[T1][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x00337c8c)?;
                        break 'b_337c8c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
            break 'b_337c90;
        }
        'b_337e70: {
            'b_337e6c: {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x50));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V1][0]);
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x64));
                c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                c.g[FP][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x78));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8c));
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                if t {
                    break 'b_337e6c;
                } else {
                    'b_337e48: {
                        c.cycles += 8;
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x59));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00337cd4);
                        c.g[A1][0] = c.g[S5][0];
                        call_ptr(c, 0x00328360, 0x00337cd4)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_337e48;
                        } else {
                            'b_337da4: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_337da4;
                                } else {
                                    c.cycles += 5;
                                    c.g[S7][0] = 0x2;
                                    c.g[S6][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_337d00: loop {
                                        'b_337d98: {
                                            'b_337d30: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S7][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_337d30;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337d28);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337d28)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337d98;
                                                    } else {
                                                        break 'b_337d30;
                                                    }
                                                }
                                            }
                                            'b_337d5c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_337d5c;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337d54);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337d54)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337d98;
                                                    } else {
                                                        break 'b_337d5c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337d98;
                                            } else {
                                                'b_337d94: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_337d94;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00337d94);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x00337d94)?;
                                                        break 'b_337d94;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337d98;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00337d00);
                                            }
                                            continue 'l_337d00;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_337da4;
                                        }
                                    }
                                }
                            }
                            'b_337db8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337db8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337db4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337db4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337db8;
                                }
                            }
                            'b_337dcc: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337dcc;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337dc8);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337dc8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337dcc;
                                }
                            }
                            'b_337de0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337de0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337ddc);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337ddc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337de0;
                                }
                            }
                            'b_337df4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337df4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337df0);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337df0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337df4;
                                }
                            }
                            'b_337e08: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337e08;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337e04);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337e04)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337e08;
                                }
                            }
                            'b_337e1c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337e1c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337e18);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337e18)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337e1c;
                                }
                            }
                            'b_337e30: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337e30;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337e2c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337e2c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337e30;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                break 'b_337e70;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00337e40);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00337e40)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                                if t {
                                    break 'b_337e70;
                                } else {
                                    break 'b_337e48;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                        break 'b_337e70;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[T1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x00337e6c);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x00337e6c)?;
                        break 'b_337e6c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
            break 'b_337e70;
        }
        'b_338020: {
            'b_33801c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5a));
                if t {
                    break 'b_33801c;
                } else {
                    'b_337ff8: {
                        c.cycles += 7;
                        c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x00337e88);
                        c.g[A1][0] = c.g[S5][0];
                        call_ptr(c, 0x00328360, 0x00337e88)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_337ff8;
                        } else {
                            'b_337f54: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_337f54;
                                } else {
                                    c.cycles += 4;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_337eb0: loop {
                                        'b_337f48: {
                                            'b_337ee0: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_337ee0;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337ed8);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337ed8)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337f48;
                                                    } else {
                                                        break 'b_337ee0;
                                                    }
                                                }
                                            }
                                            'b_337f0c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_337f0c;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00337f04);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00337f04)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_337f48;
                                                    } else {
                                                        break 'b_337f0c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337f48;
                                            } else {
                                                'b_337f44: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_337f44;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00337f44);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x00337f44)?;
                                                        break 'b_337f44;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_337f48;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00337eb0);
                                            }
                                            continue 'l_337eb0;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_337f54;
                                        }
                                    }
                                }
                            }
                            'b_337f68: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337f68;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337f64);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337f64)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_337f68;
                                }
                            }
                            'b_337f7c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337f7c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337f78);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337f78)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_337f7c;
                                }
                            }
                            'b_337f90: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337f90;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337f8c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337f8c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_337f90;
                                }
                            }
                            'b_337fa4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337fa4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337fa0);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337fa0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_337fa4;
                                }
                            }
                            'b_337fb8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337fb8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337fb4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337fb4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_337fb8;
                                }
                            }
                            'b_337fcc: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337fcc;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337fc8);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337fc8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_337fcc;
                                }
                            }
                            'b_337fe0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337fe0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00337fdc);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00337fdc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_337fe0;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                                break 'b_338020;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00337ff0);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00337ff0)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                                if t {
                                    break 'b_338020;
                                } else {
                                    break 'b_337ff8;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                        break 'b_338020;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[T1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x0033801c);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x0033801c)?;
                        break 'b_33801c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
            break 'b_338020;
        }
        'b_3381d0: {
            'b_3381cc: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5b));
                if t {
                    break 'b_3381cc;
                } else {
                    'b_3381a8: {
                        c.cycles += 7;
                        c.g[S1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[FP][0];
                        c.g[RA][0] = sx(0x00338038);
                        c.g[A1][0] = c.g[S5][0];
                        call_ptr(c, 0x00328360, 0x00338038)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_3381a8;
                        } else {
                            'b_338104: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_338104;
                                } else {
                                    c.cycles += 4;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_338060: loop {
                                        'b_3380f8: {
                                            'b_338090: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_338090;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00338088);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00338088)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3380f8;
                                                    } else {
                                                        break 'b_338090;
                                                    }
                                                }
                                            }
                                            'b_3380bc: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_3380bc;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x003380b4);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x003380b4)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3380f8;
                                                    } else {
                                                        break 'b_3380bc;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3380f8;
                                            } else {
                                                'b_3380f4: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_3380f4;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x003380f4);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x003380f4)?;
                                                        break 'b_3380f4;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3380f8;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00338060);
                                            }
                                            continue 'l_338060;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_338104;
                                        }
                                    }
                                }
                            }
                            'b_338118: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338118;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338114);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338114)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338118;
                                }
                            }
                            'b_33812c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33812c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338128);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338128)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33812c;
                                }
                            }
                            'b_338140: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338140;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033813c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033813c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338140;
                                }
                            }
                            'b_338154: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_338154;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338150);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338150)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_338154;
                                }
                            }
                            'b_338168: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338168;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338164);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338164)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338168;
                                }
                            }
                            'b_33817c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_33817c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338178);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338178)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_33817c;
                                }
                            }
                            'b_338190: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338190;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033818c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033818c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338190;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x60));
                                break 'b_3381d0;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003381a0);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x003381a0)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x60));
                                if t {
                                    break 'b_3381d0;
                                } else {
                                    break 'b_3381a8;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x60));
                        break 'b_3381d0;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[T1][0] = c.g[FP][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x003381cc);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x003381cc)?;
                        break 'b_3381cc;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x60));
            break 'b_3381d0;
        }
        'b_338388: {
            'b_338384: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                if t {
                    break 'b_338384;
                } else {
                    'b_338360: {
                        c.cycles += 8;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.g[A1][0] = c.g[S5][0];
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x003381ec);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00328360, 0x003381ec)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_338360;
                        } else {
                            'b_3382bc: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_3382bc;
                                } else {
                                    c.cycles += 5;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_338218: loop {
                                        'b_3382b0: {
                                            'b_338248: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_338248;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00338240);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00338240)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3382b0;
                                                    } else {
                                                        break 'b_338248;
                                                    }
                                                }
                                            }
                                            'b_338274: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_338274;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x0033826c);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x0033826c)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3382b0;
                                                    } else {
                                                        break 'b_338274;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3382b0;
                                            } else {
                                                'b_3382ac: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_3382ac;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x003382ac);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x003382ac)?;
                                                        break 'b_3382ac;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3382b0;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00338218);
                                            }
                                            continue 'l_338218;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_3382bc;
                                        }
                                    }
                                }
                            }
                            'b_3382d0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3382d0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003382cc);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003382cc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3382d0;
                                }
                            }
                            'b_3382e4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3382e4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003382e0);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003382e0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3382e4;
                                }
                            }
                            'b_3382f8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3382f8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003382f4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003382f4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3382f8;
                                }
                            }
                            'b_33830c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_33830c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338308);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338308)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_33830c;
                                }
                            }
                            'b_338320: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338320;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033831c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033831c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338320;
                                }
                            }
                            'b_338334: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_338334;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338330);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338330)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_338334;
                                }
                            }
                            'b_338348: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338348;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338344);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338344)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338348;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x74));
                                break 'b_338388;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00338358);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00338358)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x74));
                                if t {
                                    break 'b_338388;
                                } else {
                                    break 'b_338360;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x74));
                        break 'b_338388;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x00338384);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x00338384)?;
                        break 'b_338384;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x74));
            break 'b_338388;
        }
        'b_338540: {
            'b_33853c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                if t {
                    break 'b_33853c;
                } else {
                    'b_338518: {
                        c.cycles += 8;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5d));
                        c.g[A1][0] = c.g[S5][0];
                        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x003383a4);
                        c.g[A0][0] = c.g[V1][0];
                        call_ptr(c, 0x00328360, 0x003383a4)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_338518;
                        } else {
                            'b_338474: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_338474;
                                } else {
                                    c.cycles += 5;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_3383d0: loop {
                                        'b_338468: {
                                            'b_338400: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_338400;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x003383f8);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x003383f8)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_338468;
                                                    } else {
                                                        break 'b_338400;
                                                    }
                                                }
                                            }
                                            'b_33842c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_33842c;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00338424);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00338424)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_338468;
                                                    } else {
                                                        break 'b_33842c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_338468;
                                            } else {
                                                'b_338464: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_338464;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00338464);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x00338464)?;
                                                        break 'b_338464;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_338468;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003383d0);
                                            }
                                            continue 'l_3383d0;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_338474;
                                        }
                                    }
                                }
                            }
                            'b_338488: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338488;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338484);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338484)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338488;
                                }
                            }
                            'b_33849c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33849c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338498);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338498)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33849c;
                                }
                            }
                            'b_3384b0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3384b0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003384ac);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003384ac)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3384b0;
                                }
                            }
                            'b_3384c4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3384c4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003384c0);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003384c0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3384c4;
                                }
                            }
                            'b_3384d8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3384d8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003384d4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003384d4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3384d8;
                                }
                            }
                            'b_3384ec: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3384ec;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003384e8);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003384e8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3384ec;
                                }
                            }
                            'b_338500: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338500;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003384fc);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003384fc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338500;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
                                break 'b_338540;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00338510);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00338510)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
                                if t {
                                    break 'b_338540;
                                } else {
                                    break 'b_338518;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
                        break 'b_338540;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x0033853c);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x0033853c)?;
                        break 'b_33853c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x88));
            break 'b_338540;
        }
        'b_3386f8: {
            'b_3386f4: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                if t {
                    break 'b_3386f4;
                } else {
                    'b_3386d0: {
                        c.cycles += 8;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5e));
                        c.g[A1][0] = c.g[S5][0];
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x0033855c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00328360, 0x0033855c)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_3386d0;
                        } else {
                            'b_33862c: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_33862c;
                                } else {
                                    c.cycles += 5;
                                    c.g[S6][0] = 0x2;
                                    c.g[S4][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_338588: loop {
                                        'b_338620: {
                                            'b_3385b8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S6][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_3385b8;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x003385b0);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x003385b0)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_338620;
                                                    } else {
                                                        break 'b_3385b8;
                                                    }
                                                }
                                            }
                                            'b_3385e4: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_3385e4;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x003385dc);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x003385dc)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_338620;
                                                    } else {
                                                        break 'b_3385e4;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_338620;
                                            } else {
                                                'b_33861c: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_33861c;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0033861c);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x0033861c)?;
                                                        break 'b_33861c;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_338620;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00338588);
                                            }
                                            continue 'l_338588;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_33862c;
                                        }
                                    }
                                }
                            }
                            'b_338640: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338640;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033863c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033863c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_338640;
                                }
                            }
                            'b_338654: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_338654;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338650);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338650)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_338654;
                                }
                            }
                            'b_338668: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338668;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338664);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338664)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338668;
                                }
                            }
                            'b_33867c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_33867c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338678);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338678)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_33867c;
                                }
                            }
                            'b_338690: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338690;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033868c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033868c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338690;
                                }
                            }
                            'b_3386a4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3386a4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003386a0);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003386a0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3386a4;
                                }
                            }
                            'b_3386b8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3386b8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003386b4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003386b4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3386b8;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x9c));
                                break 'b_3386f8;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003386c8);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x003386c8)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x9c));
                                if t {
                                    break 'b_3386f8;
                                } else {
                                    break 'b_3386d0;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x9c));
                        break 'b_3386f8;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[A1][0];
                        c.g[RA][0] = sx(0x003386f4);
                        c.g[A3][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x003386f4)?;
                        break 'b_3386f4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x9c));
            break 'b_3386f8;
        }
        'b_3388b0: {
            'b_3388ac: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x6c));
                if t {
                    break 'b_3388ac;
                } else {
                    'b_338888: {
                        c.cycles += 8;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x5f));
                        c.g[A1][0] = c.g[S5][0];
                        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00338714);
                        c.g[A0][0] = c.g[V1][0];
                        call_ptr(c, 0x00328360, 0x00338714)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_338888;
                        } else {
                            'b_3387e4: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                if c.g[S0][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_3387e4;
                                } else {
                                    c.cycles += 5;
                                    c.g[S4][0] = 0x2;
                                    c.g[S3][0] = 0x3;
                                    c.g[S2][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    'l_338740: loop {
                                        'b_3387d8: {
                                            'b_338770: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                if t {
                                                    break 'b_338770;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00338768);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x0035c698,
                                                        f: ee::s11::RailModifier_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00338768)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3387d8;
                                                    } else {
                                                        break 'b_338770;
                                                    }
                                                }
                                            }
                                            'b_33879c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S3][0];
                                                c.g[A3][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_33879c;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    c.g[A2][0] = c.g[A1][0];
                                                    c.g[T0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[T1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                    c.g[T2][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x00338794);
                                                    c.g[T3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    Callee {
                                                        addr: 0x00348290,
                                                        f: ee::s11::FlexRail_FindClosestPoint,
                                                    }
                                                    .call(c, 0x00338794)?;
                                                    let t = true;
                                                    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                    if t {
                                                        break 'b_3387d8;
                                                    } else {
                                                        break 'b_33879c;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != c.g[S2][0] {
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3387d8;
                                            } else {
                                                'b_3387d4: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_3387d4;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x003387d4);
                                                        c.g[T1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x34));
                                                        Callee {
                                                            addr: 0x00335128,
                                                            f: ee::s11::Rail_FindNearestPoint,
                                                        }
                                                        .call(c, 0x003387d4)?;
                                                        break 'b_3387d4;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                                                break 'b_3387d8;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00338740);
                                            }
                                            continue 'l_338740;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_3387e4;
                                        }
                                    }
                                }
                            }
                            'b_3387f8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3387f8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003387f4);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x003387f4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3387f8;
                                }
                            }
                            'b_33880c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33880c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338808);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338808)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_33880c;
                                }
                            }
                            'b_338820: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338820;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033881c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033881c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_338820;
                                }
                            }
                            'b_338834: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_338834;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338830);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338830)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_338834;
                                }
                            }
                            'b_338848: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338848;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338844);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338844)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_338848;
                                }
                            }
                            'b_33885c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_33885c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00338858);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x00338858)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_33885c;
                                }
                            }
                            'b_338870: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338870;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033886c);
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    Callee {
                                        addr: 0x003411b8,
                                        f: ee::s11::Cell_FindRailSubtree,
                                    }
                                    .call(c, 0x0033886c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_338870;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_3388b0;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00338880);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                Callee {
                                    addr: 0x003411b8,
                                    f: ee::s11::Cell_FindRailSubtree,
                                }
                                .call(c, 0x00338880)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_3388b0;
                                } else {
                                    break 'b_338888;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_3388b0;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x6c));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A3][0] = c.g[S5][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003388ac);
                        c.g[A2][0] = c.g[A1][0];
                        Callee {
                            addr: 0x0033ed60,
                            f: ee::s11::Cell_FindRailRecursive,
                        }
                        .call(c, 0x003388ac)?;
                        break 'b_3388ac;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_3388b0;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
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

    const CELL: u32 = DATA + 0x4000;
    const POINT: u32 = DATA + 0x1200;
    const OUT_A: u32 = DATA + 0x1300;
    const OUT_B: u32 = DATA + 0x1400;
    const TREE: u32 = DATA + 0x6000;
    const LEAVES: u32 = DATA + 0x10000;
    const ITEMS: u32 = DATA + 0x8000;

    fn f32bits(v: f32) -> u32 {
        v.to_bits()
    }

    /// A cell (key at `+0`, tree at `+0x10`) holding a tree with an object
    /// list of rails / flex rails and null or leaf children; a query point
    /// and radius around it.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x7a48, 0);
        for k in 0..6u32 {
            c.bus
                .write32(crate::difftest::GAME_GP - 0x2d44 - 4 * k, f32bits(0.2));
        }
        c.bus.write32(CELL, 12);
        c.bus.write32(CELL + 4, r.next_u32() % 2);
        c.bus.write32(CELL + 8, r.next_u32() % 2);
        c.bus.write32(CELL + 0xc, r.next_u32() % 2);
        c.bus
            .write32(CELL + 0x10, if seed % 9 == 0 { 0 } else { TREE });
        for k in 0..8u32 {
            let present = r.next_u32() % 3 != 0;
            c.bus
                .write32(TREE + 4 * k, if present { LEAVES + 0x100 * k } else { 0 });
        }
        let n = r.next_u32() % 4;
        let mut head = 0;
        for k in 0..n {
            let it = ITEMS + 0x40 * k;
            c.bus.write32(it, head);
            c.bus.write32(it + 8, 1 + r.next_u32() % 3);
            c.bus.write32(it + 0xc, ITEMS + 0x1000 + 0x100 * k);
            head = it;
        }
        c.bus.write32(TREE + 0x28, head);
        for k in 0..3u32 {
            c.bus
                .write32(POINT + 4 * k, f32bits((r.next_u32() % 9000) as f32 - 500.0));
        }
        c.bus.write32(POINT + 12, f32bits(1.0));
        c.g[T0][0] = f32bits((r.next_u32() % 3000) as f32 + 50.0) as u64;
        c.mtc1(T0, 12);
        c.g[A0][0] = CELL as u64;
        c.g[A1][0] = POINT as u64;
        c.g[A2][0] = OUT_A as u64;
        c.g[A3][0] = OUT_B as u64;
        c.pc = 0x0033_7a48;
    }

    #[test]
    fn nash_find_nearest_rail_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s11::f_00337a48__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 10, "paths: {} {seen:?}", seen.len());
    }

    #[test]
    fn nash_find_nearest_rail_matches() {
        check(
            ee::s11::f_00337a48__generated,
            nash_find_nearest_rail,
            200,
            &setup,
        );
    }
}
