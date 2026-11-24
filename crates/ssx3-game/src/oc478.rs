//! Staging module oc478 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc478`.

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
    0x0033ed60,
    cell_find_rail_recursive,
    "Cell_FindRailRecursive",
)];

/// `Cell_FindRailRecursive` (0x0033ed60): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn cell_find_rail_recursive(c: &mut Cpu) {
    if c.pc != 0x0033ed60 {
        return ee::s11::f_0033ed60__generated(c);
    }
    guest(c, |c| {
        'b_33ee6c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S0][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S2][0] = c.g[T1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[FP][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x28));
            let t = c.g[S1][0] == 0u64;
            c.g[S4][0] = c.g[A3][0];
            if t {
                break 'b_33ee6c;
            } else {
                c.cycles += 5;
                c.g[S7][0] = 0x2;
                c.g[S6][0] = 0x3;
                c.g[S5][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                'l_33edc0: loop {
                    'b_33ee64: {
                        'b_33edf4: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S7][0];

                            if t {
                                break 'b_33edf4;
                            } else {
                                c.cycles += 11;
                                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                                c.g[A2][0] = c.g[A1][0];
                                c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                c.g[T1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                c.g[T2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x0033edec);
                                c.g[T3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x0035c698,
                                    f: ee::s11::RailModifier_FindClosestPoint,
                                }
                                .call(c, 0x0033edec)?;
                                let t = true;
                                c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                if t {
                                    break 'b_33ee64;
                                } else {
                                    break 'b_33edf4;
                                }
                            }
                        }
                        'b_33ee28: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[S6][0];

                            if t {
                                break 'b_33ee28;
                            } else {
                                c.cycles += 11;
                                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                                c.g[A2][0] = c.g[A1][0];
                                c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                c.g[T1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                c.g[T2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x0033ee20);
                                c.g[T3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x00348290,
                                    f: ee::s11::FlexRail_FindClosestPoint,
                                }
                                .call(c, 0x0033ee20)?;
                                let t = true;
                                c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                if t {
                                    break 'b_33ee64;
                                } else {
                                    break 'b_33ee28;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[S5][0] {
                            c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                            break 'b_33ee64;
                        } else {
                            'b_33ee60: {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                let t = c.g[V1][0] == 0u64;
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_33ee60;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                    c.g[A2][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                    c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x0033ee60);
                                    c.g[T1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                    Callee {
                                        addr: 0x00335128,
                                        f: ee::s11::Rail_FindNearestPoint,
                                    }
                                    .call(c, 0x0033ee60)?;
                                    break 'b_33ee60;
                                }
                            }
                            c.cycles += 1;
                            c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                            break 'b_33ee64;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x0033edc0);
                        }
                        continue 'l_33edc0;
                    } else {
                        break 'b_33ee6c;
                    }
                }
            }
        }
        'b_3400ac: {
            'b_3400a8: {
                c.cycles += 4;
                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_3400a8;
                } else {
                    'b_33ef04: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cc4));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, c.g[S4][0] as u32);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_33ef04;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33ef04;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_33ef04;
                            }
                        }
                    }
                    'b_33ef64: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cc0));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, c.g[S4][0] as u32);
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_33ef64;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33ef64;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_33ef64;
                            }
                        }
                    }
                    'b_33ef7c: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_33ef7c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                break 'b_3400ac;
                            } else {
                                break 'b_33ef7c;
                            }
                        }
                    }
                    'b_33efd8: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cbc));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_33efd8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33efd8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_33efd8;
                            }
                        }
                    }
                    'b_33f02c: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cb8));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_33f02c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f02c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_33f02c;
                            }
                        }
                    }
                    'b_33f044: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_33f044;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                break 'b_3400ac;
                            } else {
                                break 'b_33f044;
                            }
                        }
                    }
                    'b_33f0a0: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2cb4));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_33f0a0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f0a0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_33f0a0;
                            }
                        }
                    }
                    'b_33f0f4: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2cb0));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_33f0f4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f0f4;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_33f0f4;
                            }
                        }
                    }
                    'b_33f108: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        if t {
                            break 'b_33f108;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                break 'b_3400ac;
                            } else {
                                break 'b_33f108;
                            }
                        }
                    }
                    'b_33f130: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_33f130;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f130;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_33f130;
                            }
                        }
                    }
                    'b_33f158: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_33f158;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f158;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_33f158;
                            }
                        }
                    }
                    'b_33f180: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                        if t {
                            break 'b_33f180;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f180;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                break 'b_33f180;
                            }
                        }
                    }
                    'b_33f1a8: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                        if t {
                            break 'b_33f1a8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f1a8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                break 'b_33f1a8;
                            }
                        }
                    }
                    'b_33f1d0: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), 0u64);
                        if t {
                            break 'b_33f1d0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f1d0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                break 'b_33f1d0;
                            }
                        }
                    }
                    'b_33f1f8: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                        if t {
                            break 'b_33f1f8;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_33f1f8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                                break 'b_33f1f8;
                            }
                        }
                    }
                    'b_33f3c4: {
                        'b_33f3c0: {
                            c.cycles += 3;
                            c.g[S2][0] = c.lw(c.g[FP][0] as u32);
                            let t = c.g[S2][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            if t {
                                break 'b_33f3c0;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                if t {
                                    break 'b_33f3c0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                    if t {
                                        break 'b_33f3c0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        if t {
                                            break 'b_33f3c0;
                                        } else {
                                            'b_33f3a8: {
                                                'b_33f3a4: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    if t {
                                                        break 'b_33f3a4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        if t {
                                                            break 'b_33f3a4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33f3a8;
                                                            } else {
                                                                'b_33f300: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33f300;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33f250: loop {
                                                                            'b_33f2f4: {
                                                                                'b_33f284: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f284;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f27c);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033f27c)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f2f4;
                                                                                        } else {
                                                                                            break 'b_33f284;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33f2b8: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f2b8;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f2b0);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033f2b0)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f2f4;
                                                                                        } else {
                                                                                            break 'b_33f2b8;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f2f4;
                                                                                } else {
                                                                                    'b_33f2f0: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33f2f0;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033f2f0);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033f2f0)?;
                                                                                            break 'b_33f2f0;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f2f4;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033f250);
                                                                                }
                                                                                continue 'l_33f250;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33f300;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33f314: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f314;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f310);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f310)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f314;
                                                                    }
                                                                }
                                                                'b_33f328: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f328;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f324);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f324)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f328;
                                                                    }
                                                                }
                                                                'b_33f33c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f33c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f338);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f338)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f33c;
                                                                    }
                                                                }
                                                                'b_33f350: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f350;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f34c);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f34c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f350;
                                                                    }
                                                                }
                                                                'b_33f364: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f364;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f360);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f360)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f364;
                                                                    }
                                                                }
                                                                'b_33f378: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f378;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f374);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f374)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f378;
                                                                    }
                                                                }
                                                                'b_33f38c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f38c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f388);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f388)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f38c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33f3c4;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033f39c);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f39c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33f3c4;
                                                                    } else {
                                                                        break 'b_33f3a4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33f3a8;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033f3c0);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033f3c0)?;
                                            break 'b_33f3c0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33f3c4;
                    }
                    'b_33f59c: {
                        'b_33f598: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33f598;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                if t {
                                    break 'b_33f598;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                    if t {
                                        break 'b_33f598;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        if t {
                                            break 'b_33f598;
                                        } else {
                                            'b_33f580: {
                                                'b_33f57c: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    if t {
                                                        break 'b_33f57c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        if t {
                                                            break 'b_33f57c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33f580;
                                                            } else {
                                                                'b_33f4d8: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33f4d8;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33f428: loop {
                                                                            'b_33f4cc: {
                                                                                'b_33f45c: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f45c;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f454);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033f454)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f4cc;
                                                                                        } else {
                                                                                            break 'b_33f45c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33f490: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f490;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f488);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033f488)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f4cc;
                                                                                        } else {
                                                                                            break 'b_33f490;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f4cc;
                                                                                } else {
                                                                                    'b_33f4c8: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33f4c8;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033f4c8);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033f4c8)?;
                                                                                            break 'b_33f4c8;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f4cc;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033f428);
                                                                                }
                                                                                continue 'l_33f428;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33f4d8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33f4ec: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f4ec;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f4e8);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f4e8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f4ec;
                                                                    }
                                                                }
                                                                'b_33f500: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f500;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f4fc);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f4fc)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f500;
                                                                    }
                                                                }
                                                                'b_33f514: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f514;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f510);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f510)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f514;
                                                                    }
                                                                }
                                                                'b_33f528: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f528;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f524);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f524)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f528;
                                                                    }
                                                                }
                                                                'b_33f53c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f53c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f538);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f538)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f53c;
                                                                    }
                                                                }
                                                                'b_33f550: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f550;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f54c);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f54c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f550;
                                                                    }
                                                                }
                                                                'b_33f564: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f564;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f560);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f560)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f564;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33f59c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033f574);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f574)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33f59c;
                                                                    } else {
                                                                        break 'b_33f57c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33f580;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033f598);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033f598)?;
                                            break 'b_33f598;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33f59c;
                    }
                    'b_33f774: {
                        'b_33f770: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33f770;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33f770;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                    if t {
                                        break 'b_33f770;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        if t {
                                            break 'b_33f770;
                                        } else {
                                            'b_33f758: {
                                                'b_33f754: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    if t {
                                                        break 'b_33f754;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        if t {
                                                            break 'b_33f754;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33f758;
                                                            } else {
                                                                'b_33f6b0: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33f6b0;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33f600: loop {
                                                                            'b_33f6a4: {
                                                                                'b_33f634: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f634;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f62c);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033f62c)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f6a4;
                                                                                        } else {
                                                                                            break 'b_33f634;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33f668: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f668;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f660);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033f660)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f6a4;
                                                                                        } else {
                                                                                            break 'b_33f668;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f6a4;
                                                                                } else {
                                                                                    'b_33f6a0: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33f6a0;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033f6a0);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033f6a0)?;
                                                                                            break 'b_33f6a0;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f6a4;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033f600);
                                                                                }
                                                                                continue 'l_33f600;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33f6b0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33f6c4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f6c4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f6c0);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f6c0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f6c4;
                                                                    }
                                                                }
                                                                'b_33f6d8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f6d8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f6d4);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f6d4)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f6d8;
                                                                    }
                                                                }
                                                                'b_33f6ec: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f6ec;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f6e8);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f6e8)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f6ec;
                                                                    }
                                                                }
                                                                'b_33f700: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f700;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f6fc);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f6fc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f700;
                                                                    }
                                                                }
                                                                'b_33f714: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f714;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f710);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f710)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f714;
                                                                    }
                                                                }
                                                                'b_33f728: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f728;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f724);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f724)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f728;
                                                                    }
                                                                }
                                                                'b_33f73c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f73c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f738);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f738)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f73c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33f774;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033f74c);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f74c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33f774;
                                                                    } else {
                                                                        break 'b_33f754;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33f758;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033f770);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033f770)?;
                                            break 'b_33f770;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33f774;
                    }
                    'b_33f94c: {
                        'b_33f948: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33f948;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33f948;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                    if t {
                                        break 'b_33f948;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        if t {
                                            break 'b_33f948;
                                        } else {
                                            'b_33f930: {
                                                'b_33f92c: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    if t {
                                                        break 'b_33f92c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        if t {
                                                            break 'b_33f92c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33f930;
                                                            } else {
                                                                'b_33f888: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33f888;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33f7d8: loop {
                                                                            'b_33f87c: {
                                                                                'b_33f80c: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f80c;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f804);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033f804)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f87c;
                                                                                        } else {
                                                                                            break 'b_33f80c;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33f840: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f840;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f838);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033f838)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33f87c;
                                                                                        } else {
                                                                                            break 'b_33f840;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f87c;
                                                                                } else {
                                                                                    'b_33f878: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33f878;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033f878);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033f878)?;
                                                                                            break 'b_33f878;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33f87c;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033f7d8);
                                                                                }
                                                                                continue 'l_33f7d8;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33f888;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33f89c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f89c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f898);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f898)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33f89c;
                                                                    }
                                                                }
                                                                'b_33f8b0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f8b0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f8ac);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f8ac)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33f8b0;
                                                                    }
                                                                }
                                                                'b_33f8c4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f8c4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f8c0);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f8c0)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33f8c4;
                                                                    }
                                                                }
                                                                'b_33f8d8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f8d8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f8d4);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f8d4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33f8d8;
                                                                    }
                                                                }
                                                                'b_33f8ec: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f8ec;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f8e8);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f8e8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33f8ec;
                                                                    }
                                                                }
                                                                'b_33f900: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f900;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f8fc);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f8fc)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33f900;
                                                                    }
                                                                }
                                                                'b_33f914: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f914;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033f910);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f910)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33f914;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_33f94c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033f924);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033f924)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    if t {
                                                                        break 'b_33f94c;
                                                                    } else {
                                                                        break 'b_33f92c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33f930;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033f948);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033f948)?;
                                            break 'b_33f948;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_33f94c;
                    }
                    'b_33fb24: {
                        'b_33fb20: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33fb20;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33fb20;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                    if t {
                                        break 'b_33fb20;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_33fb20;
                                        } else {
                                            'b_33fb08: {
                                                'b_33fb04: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    if t {
                                                        break 'b_33fb04;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        if t {
                                                            break 'b_33fb04;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33fb08;
                                                            } else {
                                                                'b_33fa60: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33fa60;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33f9b0: loop {
                                                                            'b_33fa54: {
                                                                                'b_33f9e4: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33f9e4;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033f9dc);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033f9dc)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fa54;
                                                                                        } else {
                                                                                            break 'b_33f9e4;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33fa18: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33fa18;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033fa10);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033fa10)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fa54;
                                                                                        } else {
                                                                                            break 'b_33fa18;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fa54;
                                                                                } else {
                                                                                    'b_33fa50: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33fa50;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033fa50);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033fa50)?;
                                                                                            break 'b_33fa50;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fa54;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033f9b0);
                                                                                }
                                                                                continue 'l_33f9b0;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33fa60;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33fa74: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fa74;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fa70);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fa70)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fa74;
                                                                    }
                                                                }
                                                                'b_33fa88: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fa88;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fa84);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fa84)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fa88;
                                                                    }
                                                                }
                                                                'b_33fa9c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fa9c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fa98);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fa98)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fa9c;
                                                                    }
                                                                }
                                                                'b_33fab0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fab0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033faac);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033faac)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fab0;
                                                                    }
                                                                }
                                                                'b_33fac4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fac4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fac0);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fac0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fac4;
                                                                    }
                                                                }
                                                                'b_33fad8: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fad8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fad4);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fad4)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fad8;
                                                                    }
                                                                }
                                                                'b_33faec: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33faec;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fae8);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fae8)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33faec;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33fb24;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033fafc);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fafc)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33fb24;
                                                                    } else {
                                                                        break 'b_33fb04;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33fb08;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033fb20);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033fb20)?;
                                            break 'b_33fb20;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33fb24;
                    }
                    'b_33fcfc: {
                        'b_33fcf8: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33fcf8;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_33fcf8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                    if t {
                                        break 'b_33fcf8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_33fcf8;
                                        } else {
                                            'b_33fce0: {
                                                'b_33fcdc: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x38));
                                                    if t {
                                                        break 'b_33fcdc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        if t {
                                                            break 'b_33fcdc;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33fce0;
                                                            } else {
                                                                'b_33fc38: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33fc38;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33fb88: loop {
                                                                            'b_33fc2c: {
                                                                                'b_33fbbc: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33fbbc;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033fbb4);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033fbb4)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fc2c;
                                                                                        } else {
                                                                                            break 'b_33fbbc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33fbf0: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33fbf0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033fbe8);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033fbe8)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fc2c;
                                                                                        } else {
                                                                                            break 'b_33fbf0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fc2c;
                                                                                } else {
                                                                                    'b_33fc28: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33fc28;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033fc28);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033fc28)?;
                                                                                            break 'b_33fc28;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fc2c;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033fb88);
                                                                                }
                                                                                continue 'l_33fb88;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33fc38;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33fc4c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fc4c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fc48);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fc48)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fc4c;
                                                                    }
                                                                }
                                                                'b_33fc60: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fc60;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fc5c);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fc5c)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fc60;
                                                                    }
                                                                }
                                                                'b_33fc74: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fc74;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fc70);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fc70)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fc74;
                                                                    }
                                                                }
                                                                'b_33fc88: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fc88;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fc84);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fc84)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fc88;
                                                                    }
                                                                }
                                                                'b_33fc9c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fc9c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fc98);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fc98)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fc9c;
                                                                    }
                                                                }
                                                                'b_33fcb0: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fcb0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fcac);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fcac)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fcb0;
                                                                    }
                                                                }
                                                                'b_33fcc4: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33fcc4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fcc0);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fcc0)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33fcc4;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_33fcfc;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033fcd4);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fcd4)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    if t {
                                                                        break 'b_33fcfc;
                                                                    } else {
                                                                        break 'b_33fcdc;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33fce0;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033fcf8);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033fcf8)?;
                                            break 'b_33fcf8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_33fcfc;
                    }
                    'b_33fed4: {
                        'b_33fed0: {
                            c.cycles += 4;
                            c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S2][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_33fed0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                if t {
                                    break 'b_33fed0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                    if t {
                                        break 'b_33fed0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_33fed0;
                                        } else {
                                            'b_33feb8: {
                                                'b_33feb4: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                                    if t {
                                                        break 'b_33feb4;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        if t {
                                                            break 'b_33feb4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                            if t {
                                                                break 'b_33feb8;
                                                            } else {
                                                                'b_33fe10: {
                                                                    c.cycles += 3;
                                                                    c.g[S1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    if c.g[S1][0] == 0u64 {
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33fe10;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.g[S7][0] = 0x2;
                                                                        c.g[S6][0] = 0x3;
                                                                        c.g[S5][0] = 0x1;
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        'l_33fd60: loop {
                                                                            'b_33fe04: {
                                                                                'b_33fd94: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S7]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33fd94;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033fd8c);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033fd8c)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fe04;
                                                                                        } else {
                                                                                            break 'b_33fd94;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_33fdc8: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[S6]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_33fdc8;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            11;
                                                                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                        c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[A1]
                                                                                            [0];
                                                                                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                        c.g[RA][0] = sx(0x0033fdc0);
                                                                                        c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                        Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033fdc0)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[S1][0] = c.lw(c.g[S1][0] as u32);
                                                                                        if t {
                                                                                            break 'b_33fe04;
                                                                                        } else {
                                                                                            break 'b_33fdc8;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                if c.g[V0][0]
                                                                                    != c.g[S5][0]
                                                                                {
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fe04;
                                                                                } else {
                                                                                    'b_33fe00: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            == 0u64;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_33fe00;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                            c.g[RA][0] = sx(0x0033fe00);
                                                                                            c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033fe00)?;
                                                                                            break 'b_33fe00;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[S1][0] = c
                                                                                        .lw(
                                                                                        c.g[S1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    break 'b_33fe04;
                                                                                }
                                                                            }
                                                                            c.cycles += 2;
                                                                            if c.g[S1][0] != 0u64 {
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0033fd60);
                                                                                }
                                                                                continue 'l_33fd60;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[S2][0]
                                                                                        as u32);
                                                                                break 'b_33fe10;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_33fe24: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fe24;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe20);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe20)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        break 'b_33fe24;
                                                                    }
                                                                }
                                                                'b_33fe38: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fe38;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe34);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe34)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        break 'b_33fe38;
                                                                    }
                                                                }
                                                                'b_33fe4c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fe4c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe48);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe48)?;
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        break 'b_33fe4c;
                                                                    }
                                                                }
                                                                'b_33fe60: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fe60;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe5c);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe5c)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        break 'b_33fe60;
                                                                    }
                                                                }
                                                                'b_33fe74: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fe74;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe70);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe70)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        break 'b_33fe74;
                                                                    }
                                                                }
                                                                'b_33fe88: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fe88;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe84);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe84)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        break 'b_33fe88;
                                                                    }
                                                                }
                                                                'b_33fe9c: {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == 0u64 {
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33fe9c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x0033fe98);
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fe98)?;
                                                                        c.g[A0][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        break 'b_33fe9c;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_33fed4;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0033feac);
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033feac)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    if t {
                                                                        break 'b_33fed4;
                                                                    } else {
                                                                        break 'b_33feb4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                break 'b_33feb8;
                                            }
                                            c.cycles += 6;
                                            c.g[T0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S0][0];
                                            c.g[A3][0] = c.g[S4][0];
                                            c.g[RA][0] = sx(0x0033fed0);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x0033ed60,
                                                f: ee::s11::Cell_FindRailRecursive,
                                            }
                                            .call(c, 0x0033fed0)?;
                                            break 'b_33fed0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_33fed4;
                    }
                    c.cycles += 4;
                    c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S2][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_3400a8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        if t {
                            break 'b_3400a8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                            if t {
                                break 'b_3400a8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_3400a8;
                                } else {
                                    'b_340090: {
                                        'b_34008c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                                            if t {
                                                break 'b_34008c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                                                if t {
                                                    break 'b_34008c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                    if t {
                                                        break 'b_340090;
                                                    } else {
                                                        'b_33ffe8: {
                                                            c.cycles += 3;
                                                            c.g[S1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x28));
                                                            if c.g[S1][0] == 0u64 {
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S2][0] as u32);
                                                                break 'b_33ffe8;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[S5][0] = 0x2;
                                                                c.g[S4][0] = 0x3;
                                                                c.g[S3][0] = 0x1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                'l_33ff38: loop {
                                                                    'b_33ffdc: {
                                                                        'b_33ff6c: {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[S5][0];

                                                                            if t {
                                                                                break 'b_33ff6c;
                                                                            } else {
                                                                                c.cycles += 11;
                                                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[A3][0] = c
                                                                                    .lw(c.g[S0][0]
                                                                                        as u32);
                                                                                c.g[A2][0] =
                                                                                    c.g[A1][0];
                                                                                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0033ff64);
                                                                                c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                Callee { addr: 0x0035c698, f: ee::s11::RailModifier_FindClosestPoint }.call(c, 0x0033ff64)?;
                                                                                let t = true;
                                                                                c.g[S1][0] = c
                                                                                    .lw(c.g[S1][0]
                                                                                        as u32);
                                                                                if t {
                                                                                    break 'b_33ffdc;
                                                                                } else {
                                                                                    break 'b_33ff6c;
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_33ffa0: {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V0][0]
                                                                                != c.g[S4][0];

                                                                            if t {
                                                                                break 'b_33ffa0;
                                                                            } else {
                                                                                c.cycles += 11;
                                                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                                                c.g[A3][0] = c
                                                                                    .lw(c.g[S0][0]
                                                                                        as u32);
                                                                                c.g[A2][0] =
                                                                                    c.g[A1][0];
                                                                                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0033ff98);
                                                                                c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                Callee { addr: 0x00348290, f: ee::s11::FlexRail_FindClosestPoint }.call(c, 0x0033ff98)?;
                                                                                let t = true;
                                                                                c.g[S1][0] = c
                                                                                    .lw(c.g[S1][0]
                                                                                        as u32);
                                                                                if t {
                                                                                    break 'b_33ffdc;
                                                                                } else {
                                                                                    break 'b_33ffa0;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] != c.g[S3][0]
                                                                        {
                                                                            c.g[S1][0] = c
                                                                                .lw(c.g[S1][0]
                                                                                    as u32);
                                                                            break 'b_33ffdc;
                                                                        } else {
                                                                            'b_33ffd8: {
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                c.g[V1][0] = c.g
                                                                                    [V1][0]
                                                                                    & c.g[A0][0];
                                                                                let t = c.g[V1][0]
                                                                                    == 0u64;
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                if t {
                                                                                    break 'b_33ffd8;
                                                                                } else {
                                                                                    c.cycles += 6;
                                                                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                                                    c.g[A2][0] = c
                                                                                        .lw(
                                                                                        c.g[S0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                    c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0033ffd8,
                                                                                    );
                                                                                    c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                    Callee { addr: 0x00335128, f: ee::s11::Rail_FindNearestPoint }.call(c, 0x0033ffd8)?;
                                                                                    break 'b_33ffd8;
                                                                                }
                                                                            }
                                                                            c.cycles += 1;
                                                                            c.g[S1][0] = c
                                                                                .lw(c.g[S1][0]
                                                                                    as u32);
                                                                            break 'b_33ffdc;
                                                                        }
                                                                    }
                                                                    c.cycles += 2;
                                                                    if c.g[S1][0] != 0u64 {
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[S1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0033ff38,
                                                                            );
                                                                        }
                                                                        continue 'l_33ff38;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        break 'b_33ffe8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_33fffc: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33fffc;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0033fff8);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0033fff8)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                break 'b_33fffc;
                                                            }
                                                        }
                                                        'b_340010: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_340010;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0034000c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0034000c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_340010;
                                                            }
                                                        }
                                                        'b_340024: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_340024;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x00340020);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x00340020)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                break 'b_340024;
                                                            }
                                                        }
                                                        'b_340038: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_340038;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x00340034);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x00340034)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                break 'b_340038;
                                                            }
                                                        }
                                                        'b_34004c: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_34004c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x00340048);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x00340048)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_34004c;
                                                            }
                                                        }
                                                        'b_340060: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_340060;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0034005c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x0034005c)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_340060;
                                                            }
                                                        }
                                                        'b_340074: {
                                                            c.cycles += 2;
                                                            if c.g[A0][0] == 0u64 {
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_340074;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x00340070);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee { addr: 0x003411b8, f: ee::s11::Cell_FindRailSubtree }.call(c, 0x00340070)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_340074;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            break 'b_3400ac;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x00340084);
                                                            c.g[A1][0] = c.g[S0][0];
                                                            Callee {
                                                                addr: 0x003411b8,
                                                                f: ee::s11::Cell_FindRailSubtree,
                                                            }
                                                            .call(c, 0x00340084)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xe0),
                                                            );
                                                            if t {
                                                                break 'b_3400ac;
                                                            } else {
                                                                break 'b_34008c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                        break 'b_340090;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[T0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x003400a8);
                                    c.g[T1][0] = c.g[SP][0];
                                    Callee {
                                        addr: 0x0033ed60,
                                        f: ee::s11::Cell_FindRailRecursive,
                                    }
                                    .call(c, 0x003400a8)?;
                                    break 'b_3400a8;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            break 'b_3400ac;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
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

    const CTX: u32 = DATA + 0x1000;
    const CTX2: u32 = DATA + 0x1100;
    const BOX: u32 = DATA + 0x1200;
    const NODE: u32 = DATA + 0x2000;
    const CELL: u32 = DATA + 0x1300;
    const LEAF: u32 = DATA + 0x3000;
    const ITEMS: u32 = DATA + 0x8000;

    fn f32bits(v: f32) -> u32 {
        v.to_bits()
    }

    /// A level-12 cell whose eight children are null or leaves (level 11),
    /// an object list of rail / flex-rail / other items on the node, and a
    /// box that overlaps some child cells and contains others.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xed60, 0);
        c.bus.write32(GAME_GP_02CC4, f32bits(0.2));
        for i in 0..0x40u32 {
            c.bus.write32(CTX + 4 * i, r.next_u32() & 0x3f);
            c.bus.write32(CTX2 + 4 * i, r.next_u32() & 0x3f);
        }
        c.bus.write32(CELL, if seed % 7 == 0 { 11 } else { 12 });
        for k in 1..4u32 {
            c.bus.write32(CELL + 4 * k, r.next_u32() % 3);
        }
        let lo = |r: &mut crate::difftest::Rng| f32bits((r.next_u32() % 9000) as f32 - 1000.0);
        let mut mins = [0f32; 3];
        for k in 0..3u32 {
            mins[k as usize] = (r.next_u32() % 9000) as f32 - 1000.0;
            c.bus.write32(BOX + 4 * k, f32bits(mins[k as usize]));
        }
        let _ = lo;
        for k in 0..3u32 {
            let span = if seed % 3 == 0 {
                20000.0
            } else {
                (r.next_u32() % 7000) as f32
            };
            c.bus
                .write32(BOX + 0x10 + 4 * k, f32bits(mins[k as usize] + span));
        }
        for k in 0..8u32 {
            let leaf = LEAF + 0x100 * k;
            let present = r.next_u32() % 4 != 0;
            c.bus.write32(NODE + 4 * k, if present { leaf } else { 0 });
        }
        // Object list: 0..3 items.
        let n = r.next_u32() % 4;
        let mut head = 0;
        for k in 0..n {
            let it = ITEMS + 0x40 * k;
            c.bus.write32(it, head);
            c.bus.write32(it + 8, 1 + r.next_u32() % 3);
            c.bus.write32(it + 0xc, ITEMS + 0x1000 + 0x100 * k);
            head = it;
        }
        c.bus.write32(NODE + 0x28, head);
        c.g[A0][0] = (seed % 2) as u64;
        c.g[A1][0] = CTX as u64;
        c.g[A2][0] = CTX2 as u64;
        c.g[A3][0] = BOX as u64;
        c.g[T0][0] = NODE as u64;
        c.g[T1][0] = CELL as u64;
        c.pc = 0x0033_ed60;
    }

    const GAME_GP_02CC4: u32 = crate::difftest::GAME_GP - 0x2cc4;

    #[test]
    fn cell_find_rail_recursive_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s11::f_0033ed60__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 20, "paths: {} {seen:?}", seen.len());
    }

    #[test]
    fn cell_find_rail_recursive_matches() {
        check(
            ee::s11::f_0033ed60__generated,
            cell_find_rail_recursive,
            200,
            &setup,
        );
    }
}
