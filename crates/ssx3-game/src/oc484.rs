//! Staging module oc484 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc484`.

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
    0x00338920,
    cell_gather_objects_recursive,
    "Cell_GatherObjectsRecursive",
)];

/// `Cell_GatherObjectsRecursive` (0x00338920): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn cell_gather_objects_recursive(c: &mut Cpu) {
    if c.pc != 0x00338920 {
        return ee::s11::f_00338920__generated(c);
    }
    guest(c, |c| {
        'b_339568: {
            'b_339564: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S3][0] = c.g[T0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S4][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S5][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S0][0] = c.g[T1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = c.g[S3][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S1][0] = c.g[A2][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A3][0];
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[RA][0] = sx(0x00338970);
                c.g[A0][0] = c.g[S4][0];
                Callee {
                    addr: 0x003309d8,
                    f: ee::s11::Cell_GatherObjects,
                }
                .call(c, 0x00338970)?;
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = 0xb;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if t {
                    break 'b_339564;
                } else {
                    'b_338a04: {
                        c.cycles += 28;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d58));
                        c.mtc1(2, 4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.lwc1(3, c.g[S2][0] as u32);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.sub_s(0, 0, 2);
                        c.add_s(1, 1, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                        c.g[S7][0] = 0u64;
                        c.mul_s(15, 0, 4);
                        c.mul_s(16, 1, 4);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        c.c_lt_s(3, 16);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                        if t {
                            break 'b_338a04;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(15, 0);
                            if c.fcc() {
                                c.g[S7][0] = 0x1;
                                break 'b_338a04;
                            } else {
                                break 'b_338a04;
                            }
                        }
                    }
                    'b_338a64: {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d54));
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, c.g[S2][0] as u32);
                        c.add_s(1, 1, 3);
                        c.mov_s(13, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(14, 1, 4);
                        c.c_lt_s(2, 14);
                        let t = !c.fcc();
                        c.mul_s(12, 0, 4);
                        if t {
                            break 'b_338a64;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(12, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338a64;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                break 'b_338a64;
                            }
                        }
                    }
                    'b_338a78: {
                        c.cycles += 2;
                        let t = c.g[S7][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_338a78;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_339568;
                            } else {
                                break 'b_338a78;
                            }
                        }
                    }
                    'b_338ad4: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d50));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
                        c.add_s(1, 1, 3);
                        c.mov_s(6, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(11, 1, 4);
                        c.c_lt_s(2, 11);
                        let t = !c.fcc();
                        c.mul_s(10, 0, 4);
                        if t {
                            break 'b_338ad4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(10, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338ad4;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                break 'b_338ad4;
                            }
                        }
                    }
                    'b_338b28: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d4c));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(9, 1, 4);
                        c.c_lt_s(6, 9);
                        let t = !c.fcc();
                        c.mul_s(8, 0, 4);
                        if t {
                            break 'b_338b28;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(8, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338b28;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                break 'b_338b28;
                            }
                        }
                    }
                    'b_338b40: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_338b40;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_339568;
                            } else {
                                break 'b_338b40;
                            }
                        }
                    }
                    'b_338b9c: {
                        c.cycles += 17;
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d48));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.g[A0][0] = c.g[V1][0];
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                        c.add_s(1, 1, 3);
                        c.mov_s(5, 2);
                        c.sub_s(0, 0, 3);
                        c.mul_s(7, 1, 4);
                        c.c_lt_s(2, 7);
                        let t = !c.fcc();
                        c.mul_s(3, 0, 4);
                        if t {
                            break 'b_338b9c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(3, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338b9c;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                break 'b_338b9c;
                            }
                        }
                    }
                    'b_338bf0: {
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d44));
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.add_s(1, 1, 2);
                        c.sub_s(0, 0, 2);
                        c.mul_s(1, 1, 4);
                        c.c_lt_s(5, 1);
                        let t = !c.fcc();
                        c.mul_s(2, 0, 4);
                        if t {
                            break 'b_338bf0;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(2, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338bf0;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                break 'b_338bf0;
                            }
                        }
                    }
                    'b_338c04: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_338c04;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_339568;
                            } else {
                                break 'b_338c04;
                            }
                        }
                    }
                    'b_338c28: {
                        c.cycles += 4;
                        c.c_lt_s(13, 15);
                        let t = !c.fcc();
                        c.g[S6][0] = 0u64;
                        if t {
                            break 'b_338c28;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(16, 0);
                            if c.fcc() {
                                c.g[S6][0] = 0x1;
                                break 'b_338c28;
                            } else {
                                break 'b_338c28;
                            }
                        }
                    }
                    'b_338c50: {
                        c.cycles += 4;
                        c.c_lt_s(13, 12);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        if t {
                            break 'b_338c50;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(14, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338c50;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                break 'b_338c50;
                            }
                        }
                    }
                    'b_338c78: {
                        c.cycles += 4;
                        c.c_lt_s(6, 10);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        if t {
                            break 'b_338c78;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(11, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338c78;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                break 'b_338c78;
                            }
                        }
                    }
                    'b_338c9c: {
                        c.cycles += 4;
                        c.c_lt_s(6, 8);
                        let t = !c.fcc();
                        c.g[FP][0] = 0u64;
                        if t {
                            break 'b_338c9c;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                            c.c_lt_s(9, 0);
                            if c.fcc() {
                                c.g[FP][0] = 0x1;
                                break 'b_338c9c;
                            } else {
                                break 'b_338c9c;
                            }
                        }
                    }
                    'b_338cc4: {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        if t {
                            break 'b_338cc4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(7, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338cc4;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                break 'b_338cc4;
                            }
                        }
                    }
                    'b_338cec: {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        if t {
                            break 'b_338cec;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x18));
                            c.c_lt_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_338cec;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                break 'b_338cec;
                            }
                        }
                    }
                    'b_338df8: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lw(c.g[S3][0] as u32);
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_338df8;
                        } else {
                            'b_338df4: {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_338df4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_338df4;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_338df8;
                                        } else {
                                            'b_338dd8: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_338dd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_338dd8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_338dd8;
                                                        } else {
                                                            'b_338d48: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x00338d30);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x00338d30)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338d48;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338d44);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338d44)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338d48;
                                                                }
                                                            }
                                                            'b_338d5c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338d5c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338d58);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338d58)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338d5c;
                                                                }
                                                            }
                                                            'b_338d70: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338d70;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338d6c);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338d6c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338d70;
                                                                }
                                                            }
                                                            'b_338d84: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338d84;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338d80);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338d80)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338d84;
                                                                }
                                                            }
                                                            'b_338d98: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338d98;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338d94);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338d94)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338d98;
                                                                }
                                                            }
                                                            'b_338dac: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338dac;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338da8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338da8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338dac;
                                                                }
                                                            }
                                                            'b_338dc0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338dc0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338dbc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338dbc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338dc0;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_338df8;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x00338dd0);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338dd0)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_338df8;
                                                                } else {
                                                                    break 'b_338dd8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x00338df4);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x00338df4)?;
                                            break 'b_338df4;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                            break 'b_338df8;
                        }
                    }
                    'b_338f08: {
                        'b_338f04: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_338f04;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_338f04;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_338f04;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_338f08;
                                        } else {
                                            'b_338ee8: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_338ee8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_338ee8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_338ee8;
                                                        } else {
                                                            'b_338e58: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x00338e40);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x00338e40)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338e58;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338e54);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338e54)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338e58;
                                                                }
                                                            }
                                                            'b_338e6c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338e6c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338e68);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338e68)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338e6c;
                                                                }
                                                            }
                                                            'b_338e80: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338e80;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338e7c);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338e7c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338e80;
                                                                }
                                                            }
                                                            'b_338e94: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338e94;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338e90);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338e90)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338e94;
                                                                }
                                                            }
                                                            'b_338ea8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338ea8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338ea4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338ea4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338ea8;
                                                                }
                                                            }
                                                            'b_338ebc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338ebc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338eb8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338eb8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338ebc;
                                                                }
                                                            }
                                                            'b_338ed0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338ed0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338ecc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338ecc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338ed0;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_338f08;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x00338ee0);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338ee0)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_338f08;
                                                                } else {
                                                                    break 'b_338ee8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x00338f04);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x00338f04)?;
                                            break 'b_338f04;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_338f08;
                    }
                    'b_339014: {
                        'b_339010: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_339010;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_339010;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_339010;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                                            break 'b_339014;
                                        } else {
                                            'b_338ff8: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[T0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_338ff8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[FP][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_338ff8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_338ff8;
                                                        } else {
                                                            'b_338f68: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x00338f50);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x00338f50)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338f68;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338f64);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338f64)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_338f68;
                                                                }
                                                            }
                                                            'b_338f7c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338f7c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338f78);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338f78)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_338f7c;
                                                                }
                                                            }
                                                            'b_338f90: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338f90;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338f8c);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338f8c)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_338f90;
                                                                }
                                                            }
                                                            'b_338fa4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338fa4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338fa0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338fa0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_338fa4;
                                                                }
                                                            }
                                                            'b_338fb8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338fb8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338fb4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338fb4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_338fb8;
                                                                }
                                                            }
                                                            'b_338fcc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338fcc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338fc8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338fc8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_338fcc;
                                                                }
                                                            }
                                                            'b_338fe0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338fe0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00338fdc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338fdc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_338fe0;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_339014;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x00338ff0);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00338ff0)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_339014;
                                                                } else {
                                                                    break 'b_338ff8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x00339010);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x00339010)?;
                                            break 'b_339010;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_339014;
                    }
                    'b_339120: {
                        'b_33911c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33911c;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33911c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33911c;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                            break 'b_339120;
                                        } else {
                                            'b_339104: {
                                                c.cycles += 2;
                                                let t = c.g[S6][0] == 0u64;
                                                c.g[T0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_339104;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[FP][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_339104;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_339104;
                                                        } else {
                                                            'b_339074: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x0033905c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x0033905c)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339074;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339070);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339070)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339074;
                                                                }
                                                            }
                                                            'b_339088: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_339088;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339084);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339084)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_339088;
                                                                }
                                                            }
                                                            'b_33909c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33909c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339098);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339098)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_33909c;
                                                                }
                                                            }
                                                            'b_3390b0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3390b0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003390ac);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003390ac)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3390b0;
                                                                }
                                                            }
                                                            'b_3390c4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3390c4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003390c0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003390c0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3390c4;
                                                                }
                                                            }
                                                            'b_3390d8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3390d8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003390d4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003390d4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3390d8;
                                                                }
                                                            }
                                                            'b_3390ec: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_3390ec;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003390e8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003390e8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_3390ec;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x4));
                                                            if t {
                                                                break 'b_339120;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x003390fc);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003390fc)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                if t {
                                                                    break 'b_339120;
                                                                } else {
                                                                    break 'b_339104;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x0033911c);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x0033911c)?;
                                            break 'b_33911c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        break 'b_339120;
                    }
                    'b_339230: {
                        'b_33922c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_33922c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33922c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_33922c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33922c;
                                        } else {
                                            'b_339214: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[T0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_339214;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[FP][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                                    if t {
                                                        break 'b_339214;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_339214;
                                                        } else {
                                                            'b_339184: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x0033916c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x0033916c)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339184;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339180);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339180)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339184;
                                                                }
                                                            }
                                                            'b_339198: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_339198;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339194);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339194)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_339198;
                                                                }
                                                            }
                                                            'b_3391ac: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3391ac;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003391a8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003391a8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3391ac;
                                                                }
                                                            }
                                                            'b_3391c0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3391c0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003391bc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003391bc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3391c0;
                                                                }
                                                            }
                                                            'b_3391d4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3391d4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003391d0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003391d0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3391d4;
                                                                }
                                                            }
                                                            'b_3391e8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3391e8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003391e4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003391e4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3391e8;
                                                                }
                                                            }
                                                            'b_3391fc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_3391fc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003391f8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003391f8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_3391fc;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_339230;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0033920c);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x0033920c)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_339230;
                                                                } else {
                                                                    break 'b_339214;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x0033922c);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x0033922c)?;
                                            break 'b_33922c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_339230;
                    }
                    'b_339340: {
                        'b_33933c: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_33933c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_33933c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_33933c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_33933c;
                                        } else {
                                            'b_339324: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[T0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_339324;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[FP][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_339324;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_339324;
                                                        } else {
                                                            'b_339294: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x0033927c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x0033927c)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339294;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339290);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339290)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_339294;
                                                                }
                                                            }
                                                            'b_3392a8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_3392a8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003392a4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003392a4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_3392a8;
                                                                }
                                                            }
                                                            'b_3392bc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3392bc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003392b8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003392b8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3392bc;
                                                                }
                                                            }
                                                            'b_3392d0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3392d0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003392cc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003392cc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3392d0;
                                                                }
                                                            }
                                                            'b_3392e4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3392e4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003392e0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003392e0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3392e4;
                                                                }
                                                            }
                                                            'b_3392f8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3392f8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003392f4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003392f4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_3392f8;
                                                                }
                                                            }
                                                            'b_33930c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33930c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339308);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339308)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33930c;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_339340;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0033931c);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x0033931c)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                if t {
                                                                    break 'b_339340;
                                                                } else {
                                                                    break 'b_339324;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x0033933c);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x0033933c)?;
                                            break 'b_33933c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        break 'b_339340;
                    }
                    'b_339454: {
                        'b_339450: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_339450;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                                if t {
                                    break 'b_339450;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_339450;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        if t {
                                            break 'b_339450;
                                        } else {
                                            'b_339434: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    break 'b_339434;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                                    if t {
                                                        break 'b_339434;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_339434;
                                                        } else {
                                                            'b_3393a4: {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x0033938c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                Callee {
                                                                    addr: 0x003309d8,
                                                                    f: ee::s11::Cell_GatherObjects,
                                                                }
                                                                .call(c, 0x0033938c)?;
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[S0][0] as u32);
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_3393a4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003393a0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003393a0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    break 'b_3393a4;
                                                                }
                                                            }
                                                            'b_3393b8: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_3393b8;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003393b4);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003393b4)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_3393b8;
                                                                }
                                                            }
                                                            'b_3393cc: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3393cc;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003393c8);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003393c8)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0xc));
                                                                    break 'b_3393cc;
                                                                }
                                                            }
                                                            'b_3393e0: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3393e0;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003393dc);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003393dc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    break 'b_3393e0;
                                                                }
                                                            }
                                                            'b_3393f4: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3393f4;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x003393f0);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003393f0)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    break 'b_3393f4;
                                                                }
                                                            }
                                                            'b_339408: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_339408;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339404);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339404)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    break 'b_339408;
                                                                }
                                                            }
                                                            'b_33941c: {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33941c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[RA][0] = sx(0x00339418);
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339418)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    break 'b_33941c;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_339454;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0033942c);
                                                                c.g[A1][0] = c.g[S1][0];
                                                                Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x0033942c)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                if t {
                                                                    break 'b_339454;
                                                                } else {
                                                                    break 'b_339434;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[T0][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x00339450);
                                            c.g[T1][0] = c.g[SP][0];
                                            Callee {
                                                addr: 0x00338920,
                                                f: ee::s11::Cell_GatherObjectsRecursive,
                                            }
                                            .call(c, 0x00339450)?;
                                            break 'b_339450;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        break 'b_339454;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S0][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_339564;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_339564;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                            if t {
                                break 'b_339564;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                if t {
                                    break 'b_339564;
                                } else {
                                    'b_339548: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                        if t {
                                            break 'b_339548;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                            if t {
                                                break 'b_339548;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_339548;
                                                } else {
                                                    'b_3394b8: {
                                                        c.cycles += 5;
                                                        c.g[RA][0] = sx(0x003394a0);
                                                        c.g[A1][0] = c.g[S0][0];
                                                        Callee {
                                                            addr: 0x003309d8,
                                                            f: ee::s11::Cell_GatherObjects,
                                                        }
                                                        .call(c, 0x003394a0)?;
                                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_3394b8;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x003394b4);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003394b4)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_3394b8;
                                                        }
                                                    }
                                                    'b_3394cc: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_3394cc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x003394c8);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003394c8)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_3394cc;
                                                        }
                                                    }
                                                    'b_3394e0: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_3394e0;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x003394dc);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003394dc)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            break 'b_3394e0;
                                                        }
                                                    }
                                                    'b_3394f4: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_3394f4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x003394f0);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x003394f0)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            break 'b_3394f4;
                                                        }
                                                    }
                                                    'b_339508: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_339508;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x00339504);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339504)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            break 'b_339508;
                                                        }
                                                    }
                                                    'b_33951c: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33951c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x00339518);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x00339518)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_33951c;
                                                        }
                                                    }
                                                    'b_339530: {
                                                        c.cycles += 2;
                                                        if c.g[A0][0] == 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_339530;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x0033952c);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            Callee { addr: 0x00340a40, f: ee::s11::Cell_GatherObjectsSubtree }.call(c, 0x0033952c)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_339530;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] == 0u64;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                    );
                                                    if t {
                                                        break 'b_339568;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x00339540);
                                                        c.g[A1][0] = c.g[S1][0];
                                                        Callee {
                                                            addr: 0x00340a40,
                                                            f: ee::s11::Cell_GatherObjectsSubtree,
                                                        }
                                                        .call(c, 0x00339540)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        if t {
                                                            break 'b_339568;
                                                        } else {
                                                            break 'b_339548;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[T0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00339564);
                                    c.g[T1][0] = c.g[SP][0];
                                    Callee {
                                        addr: 0x00338920,
                                        f: ee::s11::Cell_GatherObjectsRecursive,
                                    }
                                    .call(c, 0x00339564)?;
                                    break 'b_339564;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
            break 'b_339568;
        }
        c.cycles += 11;
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
        let mut r = machine(c, seed, 0x8920, 0);
        for k in 0..6u32 {
            c.bus
                .write32(crate::difftest::GAME_GP - 0x2d44 - 4 * k, f32bits(0.2));
        }
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
        c.pc = 0x0033_8920;
    }

    #[test]
    fn cell_gather_objects_recursive_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            let mut c = fresh();
            setup(&mut c, seed);
            let b = c.cycles;
            run_generated(&mut c, ee::s11::f_00338920__generated);
            seen.insert(c.cycles - b);
        }
        assert!(seen.len() >= 20, "paths: {} {seen:?}", seen.len());
    }

    #[test]
    fn cell_gather_objects_recursive_matches() {
        check(
            ee::s11::f_00338920__generated,
            cell_gather_objects_recursive,
            200,
            &setup,
        );
    }
}
