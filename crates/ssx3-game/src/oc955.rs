//! Staging module oc955 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc955`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001e_5b80, f_001e5b80, "OVScreenFreestyleStandings_Init")];

/// `OVScreenFreestyleStandings_Init` (0x001e5b80): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn f_001e5b80(c: &mut Cpu) {
    if c.pc != 0x001e5b80 {
        return ee::s04::f_001e5b80__generated(c);
    }
    guest(c, |c| {
        'b_1e5bec: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x260));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x210));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x240));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x230));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x220));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ca0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x1c0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x250));
            c.g[RA][0] = sx(0x001e5bc0);
            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e5bc0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001e5bd4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e5bd4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1e5bec;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e5bec);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e5bec)?;
                break 'b_1e5bec;
            }
        }
        'b_1e5c24: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001e5bfc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001e5bfc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001e5c10);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e5c10)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e5c24;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e5c24);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e5c24)?;
                break 'b_1e5c24;
            }
        }
        'b_1e5d40: {
            'b_1e5d14: {
                'b_1e5d0c: {
                    'b_1e5cfc: {
                        c.cycles += 39;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x001e5c30);
                        c.g[S0][0] = 0x530000;
                        call_ptr(c, 0x0020a380, 0x001e5c30)?;
                        c.g[RA][0] = sx(0x001e5c38);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                        call_ptr(c, 0x0014dc80, 0x001e5c38)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e5c44);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001e5c44)?;
                        c.g[RA][0] = sx(0x001e5c4c);
                        c.g[S1][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x001e5c4c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e5c58);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001e5c58)?;
                        c.g[RA][0] = sx(0x001e5c60);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x190), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001e5c60)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e5c6c);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0014dd58, 0x001e5c6c)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x194), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                        c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                        c.g[RA][0] = sx(0x001e5c90);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x198), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001e5c90)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e5c9c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001e5c9c)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001e5ca8);
                        c.g[S2][0] = c.g[V0][0];
                        call_ptr(c, 0x00144bf0, 0x001e5ca8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e5cb4);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x002c2540, 0x001e5cb4)?;
                        c.g[V0][0] = 0x2;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[A3][0] = 0u64;
                        if t {
                            break 'b_1e5cfc;
                        } else {
                            'b_1e5cf0: {
                                'b_1e5cdc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1e5cdc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_1e5cf0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                            if t {
                                                break 'b_1e5d40;
                                            } else {
                                                break 'b_1e5cdc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1e5d0c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                    if t {
                                        break 'b_1e5d40;
                                    } else {
                                        break 'b_1e5cf0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                            if t {
                                break 'b_1e5d14;
                            } else {
                                break 'b_1e5cfc;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        break 'b_1e5d14;
                    } else {
                        break 'b_1e5d0c;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                break 'b_1e5d14;
            }
            c.cycles += 11;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = sx(0x001e5d28);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            call_ptr(c, 0x00317670, 0x001e5d28)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.g[S0][0];
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e5d38);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, t, 0x001e5d38)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1e5d40;
        }
        'b_1e5de4: {
            'b_1e5dcc: {
                'b_1e5d9c: {
                    c.cycles += 19;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                    c.g[RA][0] = sx(0x001e5d54);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c27c0, 0x001e5d54)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e5d60);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x001e5d60)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001e5d6c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x00317670, 0x001e5d6c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001e5d78);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001e5d78)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_1e5d9c;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1e5dcc;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = 0x4a0000;
                            if t {
                                break 'b_1e5de4;
                            } else {
                                break 'b_1e5d9c;
                            }
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = 0x470000;
                c.g[RA][0] = sx(0x001e5da8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c30));
                call_ptr(c, 0x00317670, 0x001e5da8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e5db4);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003a1000, 0x001e5db4)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x001e5dc4);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2150));
                call_ptr(c, 0x0039e8b8, 0x001e5dc4)?;
                let t = true;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1e5de4;
                } else {
                    break 'b_1e5dcc;
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001e5dd4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c18));
            call_ptr(c, 0x00317670, 0x001e5dd4)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001e5de0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001e5de0)?;
            c.g[A0][0] = 0x4a0000;
            break 'b_1e5de4;
        }
        'b_1e5e24: {
            c.cycles += 11;
            c.g[RA][0] = sx(0x001e5dec);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2158));
            call_ptr(c, 0x00317670, 0x001e5dec)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001e5df8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001e5df8)?;
            c.g[A0][0] = 0x530000;
            c.g[S2][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x470000;
            if t {
                break 'b_1e5e24;
            } else {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e5e18);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c00));
                call_ptr(c, 0x00317670, 0x001e5e18)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001e5e24);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e5e24)?;
                break 'b_1e5e24;
            }
        }
        'b_1e5e48: {
            c.cycles += 4;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
            c.g[V0][0] = 0x1;
            let t = c.g[A2][0] != c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x19c), 0u64);
            if t {
                break 'b_1e5e48;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x70));
                c.g[A3][0] = c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] ^ 0x3;
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = 0u64;
                }
                c.sw((c.g[SP][0] as u32).wrapping_add(0x19c), c.g[A3][0]);
                break 'b_1e5e48;
            }
        }
        'b_1e61cc: {
            c.cycles += 17;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x001e5e58);
            c.g[S7][0] = 0x1;
            call_ptr(c, 0x00146e98, 0x001e5e58)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x001e5e6c);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1a4), 0u64);
            call_ptr(c, 0x00146e98, 0x001e5e6c)?;
            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1a0), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1b4), c.g[T0][0]);
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
            c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
            if t {
                break 'b_1e61cc;
            } else {
                c.cycles += 17;
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66d0));
                c.g[A2][0] = 0x530000;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = 0x530000;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6730));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6708));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1ac), c.g[V0][0]);
                c.g[S5][0] = c.g[FP][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1a8), c.g[V1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1b8), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1bc), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1b0), c.g[A3][0]);
                'l_1e5ed0: loop {
                    'b_1e5f80: {
                        'b_1e5f78: {
                            'b_1e5f28: {
                                c.cycles += 18;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b8));
                                c.g[T0][0] = 0x470000;
                                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1be8));
                                c.g[A2][0] = c.g[S7][0];
                                c.g[A0][0] = c.g[S5][0];
                                c.g[RA][0] = sx(0x001e5eec);
                                c.g[S3][0] = c.lw((c.g[V0][0] as u32));
                                call_ptr(c, 0x00417828, 0x001e5eec)?;
                                c.g[S0][0] = 0u64;
                                c.g[RA][0] = sx(0x001e5ef8);
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x00317670, 0x001e5ef8)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001e5f04);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001e5f04)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1bc));
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                let t = c.g[A2][0] != c.g[V1][0];
                                c.g[S2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1e5f28;
                                } else {
                                    c.cycles += 4;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1ac));
                                    c.g[S0][0] = 0x1;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                    if t {
                                        break 'b_1e5f78;
                                    } else {
                                        break 'b_1e5f28;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[T0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.g[V0][0] = 0x2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                            if t {
                                break 'b_1e5f80;
                            } else {
                                c.cycles += 9;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                                c.g[A2][0] = 0x530000;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6730));
                                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1bc));
                                c.g[A0][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                let t = c.g[A3][0] != c.g[V1][0];
                                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                                if t {
                                    break 'b_1e5f80;
                                } else {
                                    c.cycles += 5;
                                    c.g[T0][0] = 0x530000;
                                    c.g[S0][0] = 0x1;
                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x66d0));
                                    c.g[V1][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                    break 'b_1e5f78;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 0u64;
                        }
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        break 'b_1e5f80;
                    }
                    'b_1e6030: {
                        'b_1e6004: {
                            'b_1e6000: {
                                c.cycles += 21;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                                c.g[V1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V0][0] = c.g[V1][0];
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001e5f98);
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0014eec8, 0x001e5f98)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001e5fa4);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x003a0c50, 0x001e5fa4)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A2][0] = c.g[S7][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1bd8));
                                c.g[RA][0] = sx(0x001e5fb8);
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x00417828, 0x001e5fb8)?;
                                c.g[RA][0] = sx(0x001e5fc0);
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x00317670, 0x001e5fc0)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001e5fcc);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001e5fcc)?;
                                let t = c.g[S0][0] == 0u64;
                                c.g[S2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1e6000;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                                    c.g[V1][0] = 0x1;
                                    if c.g[V0][0] != c.g[V1][0] {
                                        c.g[A0][0] = c.g[S6][0];
                                        break 'b_1e6004;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A2][0] = 0x4a0000;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001e5ff4);
                                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2160));
                                        call_ptr(c, 0x003a0c50, 0x001e5ff4)?;
                                        c.g[A3][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1a4), c.g[A3][0]);
                                        if t {
                                            break 'b_1e6030;
                                        } else {
                                            break 'b_1e6000;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S6][0];
                            break 'b_1e6004;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x001e6010);
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x00238590, 0x001e6010)?;
                        c.g[T0][0] = 0x4a0000;
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2138));
                        c.g[RA][0] = sx(0x001e6024);
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x00417828, 0x001e6024)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e6030);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                        call_ptr(c, 0x003a0c50, 0x001e6030)?;
                        break 'b_1e6030;
                    }
                    'b_1e61a4: {
                        'b_1e61a0: {
                            'b_1e617c: {
                                'b_1e60a0: {
                                    c.cycles += 26;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = c.g[S7][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1bc8));
                                    c.g[RA][0] = sx(0x001e6044);
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x00417828, 0x001e6044)?;
                                    c.g[RA][0] = sx(0x001e604c);
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x00317670, 0x001e604c)?;
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001e6058);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001e6058)?;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = c.g[S7][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1bb8));
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[RA][0] = sx(0x001e6070);
                                    c.g[S2][0] = c.g[V0][0];
                                    call_ptr(c, 0x00417828, 0x001e6070)?;
                                    c.g[RA][0] = sx(0x001e6078);
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x00317670, 0x001e6078)?;
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001e6084);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001e6084)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[V1][0] = 0x2;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                                    let t = c.g[V0][0] == c.g[V1][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                                    if t {
                                        break 'b_1e60a0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[A3][0] = 0x4a0000;
                                        if t {
                                            break 'b_1e617c;
                                        } else {
                                            break 'b_1e60a0;
                                        }
                                    }
                                }
                                'b_1e60dc: {
                                    c.cycles += 2;
                                    let t = c.g[S0][0] == 0u64;
                                    c.g[A2][0] = 0x4a0000;
                                    if t {
                                        break 'b_1e60dc;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001e60b4);
                                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2160));
                                        call_ptr(c, 0x003a0c50, 0x001e60b4)?;
                                        c.g[A3][0] = 0x1;
                                        c.g[A1][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1a4), c.g[A3][0]);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001e60d4);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001e60d4)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1e61a4;
                                        } else {
                                            break 'b_1e60dc;
                                        }
                                    }
                                }
                                c.cycles += 32;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[RA][0] = sx(0x001e60ec);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x00238590, 0x001e60ec)?;
                                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                                c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                                c.g[RA][0] = sx(0x001e60fc);
                                c.g[A2][0] = c.g[V0][0];
                                call_ptr(c, 0x00417828, 0x001e60fc)?;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                                c.g[RA][0] = sx(0x001e6108);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x003a0c50, 0x001e6108)?;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[RA][0] = sx(0x001e6118);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x00238590, 0x001e6118)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x001e612c);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x00238590, 0x001e612c)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                                c.g[RA][0] = sx(0x001e613c);
                                c.g[A2][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x00417828, 0x001e613c)?;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.g[RA][0] = sx(0x001e6148);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a0c50, 0x001e6148)?;
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1bc));
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                    break 'b_1e61a4;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                                    let t = c.g[A0][0] == 0u64;
                                    c.g[A2][0] = 0x4a0000;
                                    if t {
                                        break 'b_1e61a0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001e6174);
                                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2168));
                                        call_ptr(c, 0x003a0c50, 0x001e6174)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1e61a4;
                                        } else {
                                            break 'b_1e617c;
                                        }
                                    }
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001e6188);
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2168));
                            call_ptr(c, 0x003a0c50, 0x001e6188)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e61a0);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e61a0)?;
                            break 'b_1e61a0;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                        break 'b_1e61a4;
                    }
                    c.cycles += 10;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b8));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1bc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x1bc), c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001e5ed0);
                        }
                        continue 'l_1e5ed0;
                    } else {
                        break 'b_1e61cc;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x530000;
        c.g[S1][0] = 0xffffffffffffffff;
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
        c.g[S0][0] = 0x1;
        c.g[S3][0] = 0x5;
        c.g[S2][0] = 0x1;
        c.g[A2][0] = 0x470000;
        'l_1e61e8: loop {
            'b_1e6260: {
                'b_1e6254: {
                    'b_1e6250: {
                        c.cycles += 15;
                        c.g[A0][0] = c.g[FP][0];
                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1ba8));
                        c.g[RA][0] = sx(0x001e61f8);
                        c.g[A2][0] = c.g[S0][0];
                        call_ptr(c, 0x00417828, 0x001e61f8)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                        c.g[RA][0] = sx(0x001e6204);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00146e98, 0x001e6204)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = c.g[S3][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                        let t = c.g[A3][0] != c.g[S2][0];
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = c.g[V1][0];
                        }
                        if t {
                            break 'b_1e6250;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            let t = c.g[V0][0] != c.g[S0][0];
                            c.g[A0][0] = c.g[S4][0];
                            if t {
                                break 'b_1e6254;
                            } else {
                                c.cycles += 3;
                                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                                let t = c.g[T0][0] != 0u64;

                                if t {
                                    break 'b_1e6254;
                                } else {
                                    c.cycles += 5;
                                    c.g[S1][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001e6248);
                                    c.g[A1][0] = c.g[FP][0];
                                    call_ptr(c, 0x0039e948, 0x001e6248)?;
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1e6260;
                                    } else {
                                        break 'b_1e6250;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S4][0];
                    break 'b_1e6254;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001e625c);
                c.g[A1][0] = c.g[FP][0];
                call_ptr(c, 0x0039e8b8, 0x001e625c)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                break 'b_1e6260;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x470000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001e61e8);
                }
                continue 'l_1e61e8;
            } else {
                'b_1e62b8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x470000;
                    if t {
                        break 'b_1e62b8;
                    } else {
                        c.cycles += 16;
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1ba8));
                        c.g[RA][0] = sx(0x001e6288);
                        c.g[A0][0] = c.g[FP][0];
                        call_ptr(c, 0x00417828, 0x001e6288)?;
                        c.g[RA][0] = sx(0x001e6290);
                        c.g[A0][0] = c.g[FP][0];
                        call_ptr(c, 0x00317670, 0x001e6290)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e629c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001e629c)?;
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e62ac);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b98));
                        call_ptr(c, 0x00317670, 0x001e62ac)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001e62b8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001e62b8)?;
                        break 'b_1e62b8;
                    }
                }
                'b_1e6384: {
                    'b_1e6380: {
                        'b_1e6324: {
                            c.cycles += 25;
                            c.g[V0][0] = 0x470000;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1b60));
                            c.g[V0][0] = 0x470000;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1b78));
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(6, (c.g[V1][0] as u32));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x147));
                            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x140));
                            c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x14f));
                            c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x148));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[T0][0]);
                            c.g[RA][0] = sx(0x001e62fc);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x0039e8b8, 0x001e62fc)?;
                            c.g[V0][0] = 0x470000;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1b50));
                            c.g[RA][0] = sx(0x001e6310);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039e8b8, 0x001e6310)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                            let t = c.g[A0][0] == 0u64;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1e6324;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                if t {
                                    break 'b_1e6380;
                                } else {
                                    break 'b_1e6324;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                        let t = c.g[A2][0] != c.g[V0][0];
                        c.g[V0][0] = 0x440000;
                        if t {
                            break 'b_1e6384;
                        } else {
                            'b_1e6370: {
                                c.cycles += 11;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                                c.g[RA][0] = sx(0x001e633c);
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x00146e98, 0x001e633c)?;
                                c.g[A3][0] = 0x530000;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6730));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                c.g[A0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                                if c.g[A0][0] == 0u64 {
                                    c.g[A1][0] = c.g[S0][0];
                                    break 'b_1e6370;
                                } else {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001e6368);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x0039e948, 0x001e6368)?;
                                    let t = true;
                                    c.g[V0][0] = 0x440000;
                                    if t {
                                        break 'b_1e6384;
                                    } else {
                                        break 'b_1e6370;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001e6378);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0039e948, 0x001e6378)?;
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[T0][0]);
                            break 'b_1e6380;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x440000;
                    break 'b_1e6384;
                }
                c.cycles += 5;
                c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b4));
                c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d10));
                c.g[S1][0] = c.g[FP][0];
                c.g[S2][0] = 0u64;
                c.g[S5][0] = 0x470000;
                'l_1e6398: loop {
                    'b_1e6468: {
                        c.cycles += 13;
                        c.g[V0][0] = 0x470000;
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1b40));
                        c.g[RA][0] = sx(0x001e63ac);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00417828, 0x001e63ac)?;
                        c.g[RA][0] = sx(0x001e63b4);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00317670, 0x001e63b4)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e63c0);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001e63c0)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                        if t {
                            break 'b_1e6468;
                        } else {
                            'b_1e643c: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_1e643c;
                                } else {
                                    'b_1e6434: {
                                        c.cycles += 10;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S7][0] as u32)));
                                        c.g[RA][0] = sx(0x001e63e8);
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x001e63e8)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001e63f4);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a1000, 0x001e63f4)?;
                                        if c.g[S2][0] != 0u64 {
                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                            break 'b_1e6434;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                                            c.g[V0][0] = 0x1;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                break 'b_1e6434;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S6][0] as u32).wrapping_add(0x70));
                                                c.g[V0][0] = 0x3;
                                                if c.g[V1][0] != c.g[V0][0] {
                                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                    break 'b_1e6434;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[RA][0] = sx(0x001e6424);
                                                    c.g[A0][0] =
                                                        sx((c.g[S5][0] as u32)
                                                            .wrapping_sub(0x1b30));
                                                    call_ptr(c, 0x00317670, 0x001e6424)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001e6430);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x003a1000, 0x001e6430)?;
                                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                    break 'b_1e6434;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                    if t {
                                        break 'b_1e6468;
                                    } else {
                                        break 'b_1e643c;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e6450);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e6450)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e6468);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e6468)?;
                            break 'b_1e6468;
                        }
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001e6398);
                        }
                        continue 'l_1e6398;
                    } else {
                        c.cycles += 12;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x250));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x240));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x230));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x220));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x210));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x200));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1c0));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x260));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, elf_image, fresh, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (own gate, own magic).
    pub(crate) const OC955_CTRL: u32 = DATA + 0xe_8000;
    pub(crate) const OC955_MAGIC: u32 = 0x0955_f00d;
    /// What the `BENewPlayerInt_GetHumanRider` fake returns for argument 0 and 1.
    const OC955_HUMAN: u32 = OC955_CTRL + 0x20;
    /// What the two `UIEngine_CreateScreen` fake calls return.
    const OC955_SCREEN: u32 = OC955_CTRL + 0x28;
    /// The n-th `UIScreen_FindObject` call that finds nothing.
    const OC955_MISSING: u32 = OC955_CTRL + 0x30;
    /// Per-target call counters of the fakes.
    const OC955_COUNTS: u32 = OC955_CTRL + 0x400;
    const OC955_SELF: u32 = DATA + 0x1000;
    const OC955_WORLD: u32 = DATA + 0x2000;
    const OC955_SETUP: u32 = DATA + 0x3000;
    /// Found UI objects: two of them, `0x200` apart, sharing one vtable.
    const OC955_OBJ: u32 = DATA + 0x4000;
    const OC955_VT: u32 = DATA + 0x4800;
    const OC955_SCREEN_OBJ: u32 = DATA + 0x5000;

    const OC955_TARGETS: [u32; 19] = [
        0x0014_4bf0,
        0x0014_6e98,
        0x0014_dc80,
        0x0014_dd58,
        0x0014_eec8,
        0x0020_a380,
        0x0023_8590,
        0x002c_2540,
        0x002c_27c0,
        0x0031_7670,
        0x0039_7cc0,
        0x0039_c870,
        0x0039_d860,
        0x0039_e8b8,
        0x0039_e948,
        0x003a_0c50,
        0x003a_0e90,
        0x003a_1000,
        0x0041_7828,
    ];

    /// Logged fake with scripted results: the human rider ids, the created
    /// screens (or none) and the objects `UIScreen_FindObject` finds.
    pub(crate) fn oc955_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        let idx = OC955_TARGETS.iter().position(|&t| t == target).unwrap() as u32;
        let n = c.bus.read32(OC955_COUNTS + 4 * idx);
        c.bus.write32(OC955_COUNTS + 4 * idx, n + 1);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0014_6e98 => {
                let v = c.bus.read32(OC955_HUMAN + 4 * (a1 & 1));
                c.g[V0][0] = v as u64;
            }
            0x0039_7cc0 => {
                let v = c.bus.read32(OC955_SCREEN + 4 * n.min(1));
                c.g[V0][0] = v as u64;
            }
            0x0039_d860 => {
                let miss = c.bus.read32(OC955_MISSING);
                c.g[V0][0] = if n == miss {
                    0
                } else {
                    (OC955_OBJ + 0x200 * (n & 1)) as u64
                };
            }
            _ => {}
        }
    }

    macro_rules! oc955_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC955_CTRL) != OC955_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc955_fake(c, $addr);
            }
        };
    }

    oc955_gated!(
        oc955_f_144bf0,
        OC955_P_144BF0,
        0x0014_4bf0,
        ee::s01::f_00144bf0__generated
    );
    oc955_gated!(
        oc955_f_146e98,
        OC955_P_146E98,
        0x0014_6e98,
        ee::s01::f_00146e98__generated
    );
    oc955_gated!(
        oc955_f_14dc80,
        OC955_P_14DC80,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc955_gated!(
        oc955_f_14dd58,
        OC955_P_14DD58,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc955_gated!(
        oc955_f_14eec8,
        OC955_P_14EEC8,
        0x0014_eec8,
        ee::s01::f_0014eec8__generated
    );
    oc955_gated!(
        oc955_f_20a380,
        OC955_P_20A380,
        0x0020_a380,
        ee::s05::f_0020a380__generated
    );
    oc955_gated!(
        oc955_f_238590,
        OC955_P_238590,
        0x0023_8590,
        ee::s06::f_00238590__generated
    );
    oc955_gated!(
        oc955_f_2c2540,
        OC955_P_2C2540,
        0x002c_2540,
        ee::s08::f_002c2540__generated
    );
    oc955_gated!(
        oc955_f_2c27c0,
        OC955_P_2C27C0,
        0x002c_27c0,
        ee::s08::f_002c27c0__generated
    );
    oc955_gated!(
        oc955_f_317670,
        OC955_P_317670,
        0x0031_7670,
        ee::s10::f_00317670__generated
    );
    oc955_gated!(
        oc955_f_397cc0,
        OC955_P_397CC0,
        0x0039_7cc0,
        ee::s13::f_00397cc0__generated
    );
    oc955_gated!(
        oc955_f_39c870,
        OC955_P_39C870,
        0x0039_c870,
        ee::s13::f_0039c870__generated
    );
    oc955_gated!(
        oc955_f_39d860,
        OC955_P_39D860,
        0x0039_d860,
        ee::s13::f_0039d860__generated
    );
    oc955_gated!(
        oc955_f_39e8b8,
        OC955_P_39E8B8,
        0x0039_e8b8,
        ee::s13::f_0039e8b8__generated
    );
    oc955_gated!(
        oc955_f_39e948,
        OC955_P_39E948,
        0x0039_e948,
        ee::s13::f_0039e948__generated
    );
    oc955_gated!(
        oc955_f_3a0c50,
        OC955_P_3A0C50,
        0x003a_0c50,
        ee::s13::f_003a0c50__generated
    );
    oc955_gated!(
        oc955_f_3a0e90,
        OC955_P_3A0E90,
        0x003a_0e90,
        ee::s13::f_003a0e90__generated
    );
    oc955_gated!(
        oc955_f_3a1000,
        OC955_P_3A1000,
        0x003a_1000,
        ee::s13::f_003a1000__generated
    );
    oc955_gated!(
        oc955_f_417828,
        OC955_P_417828,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    pub(crate) fn oc955_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
            oc955_f_144bf0,
            &OC955_P_144BF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc955_f_146e98,
            &OC955_P_146E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc955_f_14dc80,
            &OC955_P_14DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc955_f_14dd58,
            &OC955_P_14DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EEC8),
            oc955_f_14eec8,
            &OC955_P_14EEC8,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc955_f_20a380,
            &OC955_P_20A380,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00238590),
            oc955_f_238590,
            &OC955_P_238590,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2540),
            oc955_f_2c2540,
            &OC955_P_2C2540,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C27C0),
            oc955_f_2c27c0,
            &OC955_P_2C27C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc955_f_317670,
            &OC955_P_317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc955_f_397cc0,
            &OC955_P_397CC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc955_f_39c870,
            &OC955_P_39C870,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc955_f_39d860,
            &OC955_P_39D860,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E8B8),
            oc955_f_39e8b8,
            &OC955_P_39E8B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E948),
            oc955_f_39e948,
            &OC955_P_39E948,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc955_f_3a0c50,
            &OC955_P_3A0C50,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc955_f_3a0e90,
            &OC955_P_3A0E90,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc955_f_3a1000,
            &OC955_P_3A1000,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc955_f_417828,
            &OC955_P_417828,
        );
    }

    fn oc955_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The seed picks the standings screen's inputs: the race state (1 = race
    /// result, 2 = final), the event mode byte, the 2-player flag, the
    /// winner flag, the AI count, the standing tables, the two human riders,
    /// whether a screen is created, which object lookup fails.
    fn oc955_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc955_install_fakes();
        fakes::machine(c, seed, 0x1e5b80, calls);
        elf_image(c);
        let h = fakes::mix(seed as u64 * 0x9e37 + 5);
        let pick = |shift: u32, n: u32| ((h >> shift) as u32) % n;
        oc955_w(c, OC955_CTRL, OC955_MAGIC);
        c.pc = 0x001e_5b80;
        c.g[A0][0] = OC955_SELF as u64;
        oc955_w(c, OC955_SELF + 0x10, DATA + 0x6000);
        // world: setup pointer at +0xc0, localisation object at +0x8c.
        oc955_w(c, crate::difftest::GAME_GP.wrapping_sub(0x848), OC955_WORLD);
        oc955_w(c, OC955_WORLD + 0xc0, OC955_SETUP);
        oc955_w(c, OC955_WORLD + 0x8c, OC955_SCREEN_OBJ);
        oc955_w(c, OC955_SCREEN_OBJ + 4, OC955_OBJ);
        for i in 0..0x80u32 {
            let w = fakes::mix(h + i as u64) as u32;
            oc955_w(c, OC955_SETUP + 4 * i, w & 7);
        }
        oc955_w(c, OC955_SETUP, pick(0, 4));
        oc955_w(c, OC955_SETUP + 0x10, pick(4, 6));
        oc955_w(
            c,
            OC955_SETUP + 0x70,
            if pick(8, 2) == 0 { 3 } else { pick(9, 3) },
        );
        // the race globals (mode byte +0x48, flag +0x49; standing tables)
        c.bus.write8(0x0053_5bc8 + 0x48, pick(11, 4) as u8);
        c.bus.write8(0x0053_5bc8 + 0x49, pick(13, 3) as u8);
        for i in 0..8u32 {
            oc955_w(c, 0x0053_6708 + 4 * i, (i + pick(15, 8)) % 8);
            oc955_w(c, 0x0053_6730 + 4 * i, pick(18 + i, 7));
            oc955_w(c, 0x0053_66d0 + 4 * i, pick(30 + i, 2));
        }
        oc955_w(c, OC955_HUMAN, pick(20, 3));
        oc955_w(c, OC955_HUMAN + 4, pick(22, 3));
        let none = pick(24, 5) == 0;
        oc955_w(c, OC955_SCREEN, if none { 0 } else { OC955_SCREEN_OBJ });
        oc955_w(
            c,
            OC955_SCREEN + 4,
            if pick(26, 5) == 0 {
                0
            } else {
                OC955_SCREEN_OBJ
            },
        );
        oc955_w(
            c,
            OC955_MISSING,
            if pick(28, 4) == 0 {
                pick(33, 40)
            } else {
                0x7fff_ffff
            },
        );
        // objects: [+4] self, [+8] vtable; vtable methods are machine fakes.
        for i in 0..2u32 {
            let o = OC955_OBJ + 0x200 * i;
            oc955_w(c, o + 4, OC955_OBJ);
            oc955_w(c, o + 8, OC955_VT);
        }
        for (k, off) in [0x24u32, 0x44, 0x4c].into_iter().enumerate() {
            oc955_w(c, OC955_VT + off, FAKE + 0x10 * (k as u32 + 1));
        }
        c.bus.write16(OC955_OBJ + 0x20, 0);
    }

    /// `OVScreenFreestyleStandings_Init(self)`: builds the standings screen:
    /// two screens, the title from the event mode, the result rows (names,
    /// points, bonus, totals), the position marker, the hint buttons.
    #[test]
    fn ov_screen_freestyle_standings_init_matches() {
        check(
            ee::s04::f_001e5b80__generated,
            f_001e5b80,
            600,
            &|c, seed| oc955_setup(c, seed, 150),
        );
    }

    /// The fixture reaches many distinct paths of the generated body.
    #[test]
    fn ov_screen_freestyle_standings_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..600 {
            let mut c = fresh();
            oc955_setup(&mut c, seed, 0);
            let before = c.cycles;
            ee::s04::f_001e5b80__generated(&mut c);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 40, "paths {} {seen:?}", seen.len());
    }
}
