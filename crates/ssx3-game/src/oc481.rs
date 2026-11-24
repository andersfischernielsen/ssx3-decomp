//! Staging module oc481 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc481`.

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
    0x00331e90,
    overlap_list_gather_objects_and_instances,
    "OverlapList_GatherObjectsAndInstances",
)];

/// `OverlapList_GatherObjectsAndInstances` (0x00331e90): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn overlap_list_gather_objects_and_instances(c: &mut Cpu) {
    if c.pc != 0x00331e90 {
        return ee::s11::f_00331e90__generated(c);
    }
    guest(c, |c| {
        'b_3320e4: {
            'b_3320e0: {
                c.cycles += 37;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.g[A2][0] = 0x40;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S4][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[FP][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0xc));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x8));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x110));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x10c));
                c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[S4][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[S4][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.sw(c.g[V0][0] as u32, 0u64);
                c.g[S5][0] = c.lw(c.g[FP][0] as u32);
                c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
                let t = c.g[S1][0] == 0u64;
                c.g[S0][0] = c.g[SP][0];
                if t {
                    break 'b_3320e0;
                } else {
                    'b_3320bc: {
                        c.cycles += 7;
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00331f34);
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x00328360, 0x00331f34)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_3320bc;
                        } else {
                            'b_332018: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x00331f4c);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00331f4c)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                if c.g[A2][0] == 0u64 {
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    break 'b_332018;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    'l_331f60: loop {
                                        'b_331fe0: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_331fe0;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_331fe0;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_331fe0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_331fe0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_331fe0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_331fe0;
                                                                } else {
                                                                    break 'b_331fe0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_33200c: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33200c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33200c;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00331f60);
                                            }
                                            continue 'l_331f60;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            break 'b_332018;
                                        }
                                    }
                                }
                            }
                            'b_33202c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_33202c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332028);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332028)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_33202c;
                                }
                            }
                            'b_332040: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332040;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033203c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033203c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332040;
                                }
                            }
                            'b_332054: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332054;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332050);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332050)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332054;
                                }
                            }
                            'b_332068: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332068;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332064);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332064)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332068;
                                }
                            }
                            'b_33207c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_33207c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332078);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332078)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_33207c;
                                }
                            }
                            'b_332090: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332090;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033208c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033208c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332090;
                                }
                            }
                            'b_3320a4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3320a4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003320a0);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003320a0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3320a4;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                                break 'b_3320e4;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003320b4);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x003320b4)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                                if t {
                                    break 'b_3320e4;
                                } else {
                                    break 'b_3320bc;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
                        break 'b_3320e4;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x003320e0);
                        c.g[T1][0] = c.g[S5][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x003320e0)?;
                        break 'b_3320e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
            break 'b_3320e4;
        }
        'b_3322d4: {
            'b_3322d0: {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x64));
                c.g[S6][0] = sx((c.g[S5][0] as u32).wrapping_add(0x28));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                c.g[S7][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3c));
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x78));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8c));
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                if t {
                    break 'b_3322d0;
                } else {
                    'b_3322ac: {
                        c.cycles += 8;
                        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x21));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00332128);
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x00328360, 0x00332128)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_3322ac;
                        } else {
                            'b_332204: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00332140);
                                c.g[A1][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00332140)?;
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_332204;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_332150: loop {
                                        'b_3321d0: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_3321d0;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_3321d0;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_3321d0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_3321d0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_3321d0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_3321d0;
                                                                } else {
                                                                    break 'b_3321d0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_3321fc: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_3321fc;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_3321fc;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00332150);
                                            }
                                            continue 'l_332150;
                                        } else {
                                            break 'b_332204;
                                        }
                                    }
                                }
                            }
                            'b_33221c: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                    break 'b_33221c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332218);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332218)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                    break 'b_33221c;
                                }
                            }
                            'b_332230: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    break 'b_332230;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033222c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033222c)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    break 'b_332230;
                                }
                            }
                            'b_332244: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    break 'b_332244;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332240);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332240)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    break 'b_332244;
                                }
                            }
                            'b_332258: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_332258;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332254);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332254)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_332258;
                                }
                            }
                            'b_33226c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    break 'b_33226c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332268);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332268)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    break 'b_33226c;
                                }
                            }
                            'b_332280: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    break 'b_332280;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033227c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033227c)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    break 'b_332280;
                                }
                            }
                            'b_332294: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                    break 'b_332294;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332290);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332290)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                    break 'b_332294;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x38));
                                break 'b_3322d4;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003322a4);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x003322a4)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x38));
                                if t {
                                    break 'b_3322d4;
                                } else {
                                    break 'b_3322ac;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x38));
                        break 'b_3322d4;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T0][0] = c.g[S2][0];
                        c.g[T1][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003322d0);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x003322d0)?;
                        break 'b_3322d0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x38));
            break 'b_3322d4;
        }
        'b_33249c: {
            'b_332498: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x22));
                if t {
                    break 'b_332498;
                } else {
                    'b_332474: {
                        c.cycles += 7;
                        c.g[S1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x003322ec);
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x00328360, 0x003322ec)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_332474;
                        } else {
                            'b_3323cc: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00332304);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00332304)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_3323cc;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_332318: loop {
                                        'b_332398: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_332398;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_332398;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_332398;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_332398;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_332398;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_332398;
                                                                } else {
                                                                    break 'b_332398;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_3323c4: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_3323c4;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_3323c4;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00332318);
                                            }
                                            continue 'l_332318;
                                        } else {
                                            break 'b_3323cc;
                                        }
                                    }
                                }
                            }
                            'b_3323e4: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3323e4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003323e0);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003323e0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3323e4;
                                }
                            }
                            'b_3323f8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3323f8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003323f4);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003323f4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3323f8;
                                }
                            }
                            'b_33240c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_33240c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332408);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332408)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_33240c;
                                }
                            }
                            'b_332420: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332420;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033241c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033241c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332420;
                                }
                            }
                            'b_332434: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_332434;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332430);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332430)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_332434;
                                }
                            }
                            'b_332448: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332448;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332444);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332444)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332448;
                                }
                            }
                            'b_33245c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_33245c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332458);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332458)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_33245c;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4c));
                                break 'b_33249c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0033246c);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x0033246c)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4c));
                                if t {
                                    break 'b_33249c;
                                } else {
                                    break 'b_332474;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4c));
                        break 'b_33249c;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[T1][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00332498);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x00332498)?;
                        break 'b_332498;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4c));
            break 'b_33249c;
        }
        'b_332664: {
            'b_332660: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x23));
                if t {
                    break 'b_332660;
                } else {
                    'b_33263c: {
                        c.cycles += 7;
                        c.g[S1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x003324b4);
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x00328360, 0x003324b4)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_33263c;
                        } else {
                            'b_332594: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x003324cc);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x003324cc)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_332594;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_3324e0: loop {
                                        'b_332560: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_332560;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_332560;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_332560;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_332560;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_332560;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_332560;
                                                                } else {
                                                                    break 'b_332560;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_33258c: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33258c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33258c;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003324e0);
                                            }
                                            continue 'l_3324e0;
                                        } else {
                                            break 'b_332594;
                                        }
                                    }
                                }
                            }
                            'b_3325ac: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3325ac;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003325a8);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003325a8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_3325ac;
                                }
                            }
                            'b_3325c0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3325c0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003325bc);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003325bc)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_3325c0;
                                }
                            }
                            'b_3325d4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3325d4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003325d0);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003325d0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_3325d4;
                                }
                            }
                            'b_3325e8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3325e8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003325e4);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003325e4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3325e8;
                                }
                            }
                            'b_3325fc: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3325fc;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003325f8);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003325f8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3325fc;
                                }
                            }
                            'b_332610: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332610;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033260c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033260c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332610;
                                }
                            }
                            'b_332624: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_332624;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332620);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332620)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_332624;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x60));
                                break 'b_332664;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00332634);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332634)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x60));
                                if t {
                                    break 'b_332664;
                                } else {
                                    break 'b_33263c;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x60));
                        break 'b_332664;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[T1][0] = c.g[S7][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00332660);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x00332660)?;
                        break 'b_332660;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x60));
            break 'b_332664;
        }
        'b_33282c: {
            'b_332828: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                if t {
                    break 'b_332828;
                } else {
                    'b_332804: {
                        c.cycles += 8;
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00332680);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00328360, 0x00332680)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_332804;
                        } else {
                            'b_33275c: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00332698);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00332698)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_33275c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_3326a8: loop {
                                        'b_332728: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_332728;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_332728;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_332728;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_332728;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_332728;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_332728;
                                                                } else {
                                                                    break 'b_332728;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_332754: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_332754;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_332754;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003326a8);
                                            }
                                            continue 'l_3326a8;
                                        } else {
                                            break 'b_33275c;
                                        }
                                    }
                                }
                            }
                            'b_332774: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_332774;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332770);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332770)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_332774;
                                }
                            }
                            'b_332788: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332788;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332784);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332784)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332788;
                                }
                            }
                            'b_33279c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_33279c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332798);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332798)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_33279c;
                                }
                            }
                            'b_3327b0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3327b0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003327ac);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003327ac)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_3327b0;
                                }
                            }
                            'b_3327c4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3327c4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003327c0);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003327c0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_3327c4;
                                }
                            }
                            'b_3327d8: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3327d8;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003327d4);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003327d4)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3327d8;
                                }
                            }
                            'b_3327ec: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3327ec;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003327e8);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003327e8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3327ec;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x74));
                                break 'b_33282c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003327fc);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x003327fc)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x74));
                                if t {
                                    break 'b_33282c;
                                } else {
                                    break 'b_332804;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x74));
                        break 'b_33282c;
                    } else {
                        c.cycles += 7;
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00332828);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x00332828)?;
                        break 'b_332828;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x74));
            break 'b_33282c;
        }
        'b_3329f4: {
            'b_3329f0: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                if t {
                    break 'b_3329f0;
                } else {
                    'b_3329cc: {
                        c.cycles += 8;
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x25));
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00332848);
                        c.g[A0][0] = c.g[V1][0];
                        call_ptr(c, 0x00328360, 0x00332848)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_3329cc;
                        } else {
                            'b_332924: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00332860);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00332860)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_332924;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_332870: loop {
                                        'b_3328f0: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_3328f0;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_3328f0;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_3328f0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_3328f0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_3328f0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_3328f0;
                                                                } else {
                                                                    break 'b_3328f0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_33291c: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33291c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_33291c;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00332870);
                                            }
                                            continue 'l_332870;
                                        } else {
                                            break 'b_332924;
                                        }
                                    }
                                }
                            }
                            'b_33293c: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_33293c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332938);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332938)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_33293c;
                                }
                            }
                            'b_332950: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332950;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033294c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033294c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332950;
                                }
                            }
                            'b_332964: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332964;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332960);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332960)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332964;
                                }
                            }
                            'b_332978: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332978;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332974);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332974)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332978;
                                }
                            }
                            'b_33298c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_33298c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332988);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332988)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_33298c;
                                }
                            }
                            'b_3329a0: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3329a0;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0033299c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x0033299c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_3329a0;
                                }
                            }
                            'b_3329b4: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3329b4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x003329b0);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x003329b0)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_3329b4;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x88));
                                break 'b_3329f4;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x003329c4);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x003329c4)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x88));
                                if t {
                                    break 'b_3329f4;
                                } else {
                                    break 'b_3329cc;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x88));
                        break 'b_3329f4;
                    } else {
                        c.cycles += 7;
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003329f0);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x003329f0)?;
                        break 'b_3329f0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x88));
            break 'b_3329f4;
        }
        'b_332bbc: {
            'b_332bb8: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_332bb8;
                } else {
                    'b_332b94: {
                        c.cycles += 8;
                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x26));
                        c.g[A1][0] = c.g[S4][0];
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00332a10);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00328360, 0x00332a10)?;
                        c.g[V1][0] = c.g[V0][0];
                        let t = c.g[V1][0] != 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_332b94;
                        } else {
                            'b_332aec: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00332a28);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x003309d8,
                                    f: ee::s11::Cell_GatherObjects,
                                }
                                .call(c, 0x00332a28)?;
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_332aec;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    'l_332a38: loop {
                                        'b_332ab8: {
                                            c.cycles += 6;
                                            c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                            c.lwc1(0, c.g[V0][0] as u32);
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();
                                            c.g[V1][0] = 0u64;
                                            if t {
                                                break 'b_332ab8;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                                c.c_le_s(0, 1);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_332ab8;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x168),
                                                    );
                                                    c.c_le_s(1, 0);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_332ab8;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                        );
                                                        c.c_le_s(0, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_332ab8;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x16c),
                                                            );
                                                            c.c_le_s(1, 0);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_332ab8;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[A2][0] as u32)
                                                                        .wrapping_add(0x160),
                                                                );
                                                                c.c_le_s(0, 1);
                                                                if c.fcc() {
                                                                    c.g[V1][0] = 0x1;
                                                                    break 'b_332ab8;
                                                                } else {
                                                                    break 'b_332ab8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_332ae4: {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_332ae4;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                                break 'b_332ae4;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A2][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00332a38);
                                            }
                                            continue 'l_332a38;
                                        } else {
                                            break 'b_332aec;
                                        }
                                    }
                                }
                            }
                            'b_332b04: {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_332b04;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b00);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b00)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    break 'b_332b04;
                                }
                            }
                            'b_332b18: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332b18;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b14);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b14)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_332b18;
                                }
                            }
                            'b_332b2c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332b2c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b28);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b28)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                    break 'b_332b2c;
                                }
                            }
                            'b_332b40: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332b40;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b3c);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b3c)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    break 'b_332b40;
                                }
                            }
                            'b_332b54: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_332b54;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b50);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b50)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                    break 'b_332b54;
                                }
                            }
                            'b_332b68: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332b68;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b64);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b64)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_332b68;
                                }
                            }
                            'b_332b7c: {
                                c.cycles += 2;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_332b7c;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x00332b78);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00340c20,
                                        f: ee::s11::Cell_GatherObjInstSubtree,
                                    }
                                    .call(c, 0x00332b78)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    break 'b_332b7c;
                                }
                            }
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x9c));
                                break 'b_332bbc;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00332b8c);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332b8c)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x9c));
                                if t {
                                    break 'b_332bbc;
                                } else {
                                    break 'b_332b94;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x9c));
                        break 'b_332bbc;
                    } else {
                        c.cycles += 7;
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00332bb8);
                        c.g[A3][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0033a3d8,
                            f: ee::s11::Cell_GatherObjInstRecursive,
                        }
                        .call(c, 0x00332bb8)?;
                        break 'b_332bb8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x9c));
            break 'b_332bbc;
        }
        'b_332d7c: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
            if t {
                break 'b_332d7c;
            } else {
                'b_332d5c: {
                    c.cycles += 8;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x27));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x00332bd8);
                    c.g[A0][0] = c.g[V1][0];
                    call_ptr(c, 0x00328360, 0x00332bd8)?;
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_332d5c;
                    } else {
                        'b_332cb4: {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00332bf0);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x003309d8,
                                f: ee::s11::Cell_GatherObjects,
                            }
                            .call(c, 0x00332bf0)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_332cb4;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                'l_332c00: loop {
                                    'b_332c80: {
                                        c.cycles += 6;
                                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x164));
                                        c.lwc1(0, c.g[V0][0] as u32);
                                        c.c_le_s(0, 1);
                                        let t = !c.fcc();
                                        c.g[V1][0] = 0u64;
                                        if t {
                                            break 'b_332c80;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x158));
                                            c.c_le_s(0, 1);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_332c80;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x168));
                                                c.c_le_s(1, 0);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_332c80;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x14),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_add(0x15c),
                                                    );
                                                    c.c_le_s(0, 1);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_332c80;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[A2][0] as u32).wrapping_add(0x16c),
                                                        );
                                                        c.c_le_s(1, 0);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_332c80;
                                                        } else {
                                                            c.cycles += 6;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x160),
                                                            );
                                                            c.c_le_s(0, 1);
                                                            if c.fcc() {
                                                                c.g[V1][0] = 0x1;
                                                                break 'b_332c80;
                                                            } else {
                                                                break 'b_332c80;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    'b_332cac: {
                                        c.cycles += 2;
                                        if c.g[V1][0] == 0u64 {
                                            c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                            break 'b_332cac;
                                        } else {
                                            c.cycles += 9;
                                            c.g[A1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                            c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                                            c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                                            break 'b_332cac;
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[A2][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00332c00);
                                        }
                                        continue 'l_332c00;
                                    } else {
                                        break 'b_332cb4;
                                    }
                                }
                            }
                        }
                        'b_332ccc: {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_332ccc;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332cc8);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332cc8)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_332ccc;
                            }
                        }
                        'b_332ce0: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_332ce0;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332cdc);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332cdc)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_332ce0;
                            }
                        }
                        'b_332cf4: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_332cf4;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332cf0);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332cf0)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_332cf4;
                            }
                        }
                        'b_332d08: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                break 'b_332d08;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332d04);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332d04)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                break 'b_332d08;
                            }
                        }
                        'b_332d1c: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                break 'b_332d1c;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332d18);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332d18)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                break 'b_332d1c;
                            }
                        }
                        'b_332d30: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                break 'b_332d30;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332d2c);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332d2c)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                break 'b_332d30;
                            }
                        }
                        'b_332d44: {
                            c.cycles += 2;
                            if c.g[A0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                break 'b_332d44;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x00332d40);
                                c.g[A1][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00340c20,
                                    f: ee::s11::Cell_GatherObjInstSubtree,
                                }
                                .call(c, 0x00332d40)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                break 'b_332d44;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;

                        if t {
                            break 'b_332d7c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00332d54);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00340c20,
                                f: ee::s11::Cell_GatherObjInstSubtree,
                            }
                            .call(c, 0x00332d54)?;
                            let t = true;

                            if t {
                                break 'b_332d7c;
                            } else {
                                break 'b_332d5c;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                if t {
                    break 'b_332d7c;
                } else {
                    c.cycles += 6;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S4][0];
                    c.g[T0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00332d7c);
                    c.g[A2][0] = c.g[A1][0];
                    Callee {
                        addr: 0x0033a3d8,
                        f: ee::s11::Cell_GatherObjInstRecursive,
                    }
                    .call(c, 0x00332d7c)?;
                    break 'b_332d7c;
                }
            }
        }
        c.cycles += 15;
        c.g[V0][0] = 0xffffffffffff0000;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x210), c.g[V0][0]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fresh, run_generated, DATA};

    const SELF: u32 = DATA;
    const GRID: u32 = DATA + 0x4000;
    const BOX: u32 = DATA + 0x1200;
    const TREES: u32 = DATA + 0x6000;
    const LEAVES: u32 = DATA + 0x10000;

    fn f32bits(v: f32) -> u32 {
        v.to_bits()
    }

    /// The eight grid cells (key at `+0`, tree at `+0x10`, stride 0x14),
    /// trees with null or leaf children, and a box that classifies
    /// differently against each cell.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1e90, 0);
        for k in 0..6u32 {
            c.bus
                .write32(crate::difftest::GAME_GP - 0x2d44 - 4 * k, f32bits(0.2));
        }
        c.bus.write32(SELF, GRID);
        c.bus.write32(GRID + 0xa0, r.next_u32() & 0xff);
        for i in 0..8u32 {
            let cell = GRID + 0x14 * i;
            c.bus.write32(cell, 12);
            c.bus.write32(cell + 4, i & 1);
            c.bus.write32(cell + 8, (i >> 1) & 1);
            c.bus.write32(cell + 0xc, (i >> 2) & 1);
            let tree = TREES + 0x400 * i;
            c.bus
                .write32(cell + 0x10, if r.next_u32() % 5 == 0 { 0 } else { tree });
            for k in 0..8u32 {
                let present = r.next_u32() % 3 != 0;
                c.bus.write32(
                    tree + 4 * k,
                    if present {
                        LEAVES + 0x100 * (8 * i + k)
                    } else {
                        0
                    },
                );
            }
        }
        let mut mins = [0f32; 3];
        for k in 0..3u32 {
            mins[k as usize] = (r.next_u32() % 10000) as f32 - 2000.0;
            c.bus.write32(BOX + 4 * k, f32bits(mins[k as usize]));
        }
        for k in 0..3u32 {
            let span = if seed % 3 == 0 {
                30000.0
            } else {
                (r.next_u32() % 6000) as f32
            };
            c.bus
                .write32(BOX + 0x10 + 4 * k, f32bits(mins[k as usize] + span));
        }
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = BOX as u64;
        c.pc = 0x0033_1e90;
    }

    #[test]
    fn overlap_list_gather_objects_and_instances_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s11::f_00331e90__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 20, "paths: {} {seen:?}", seen.len());
    }

    #[test]
    fn overlap_list_gather_objects_and_instances_matches() {
        check(
            ee::s11::f_00331e90__generated,
            overlap_list_gather_objects_and_instances,
            200,
            &setup,
        );
    }
}
