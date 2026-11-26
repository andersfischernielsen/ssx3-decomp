//! Staging module oc803 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc803`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x00206690,
    event_select_show_event_paths,
    "EventSelect_ShowEventPaths",
)];

/// `EventSelect_SetupPeakTexts` (0x00200de8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[cfg(test)] // not installed: its test runs away in the unslotted file-load chain
#[allow(dead_code)]
pub fn event_select_setup_peak_texts(c: &mut Cpu) {
    if c.pc != 0x00200de8 {
        return ee::s05::f_00200de8__generated(c);
    }
    guest(c, |c| {
        'b_200e64: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x340));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x300));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x320));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x330));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x310));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x2e0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x2a0), c.g[RA][0]);
            c.g[RA][0] = sx(0x00200e1c);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00200e1c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00200e28);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00200e28)?;
            c.g[A0][0] = 0x4a0000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x270), c.g[V0][0]);
            c.g[RA][0] = sx(0x00200e38);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25e0));
            call_ptr(c, 0x00317670, 0x00200e38)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x00200e44);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00200e44)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = 0x2;
            if t {
                break 'b_200e64;
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00200e64);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200e64)?;
                break 'b_200e64;
            }
        }
        'b_200e9c: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x00200e70);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25e8));
            call_ptr(c, 0x00317670, 0x00200e70)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x00200e7c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00200e7c)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_200e9c;
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00200e9c);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200e9c)?;
                break 'b_200e9c;
            }
        }
        'b_200ed0: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x00200ea8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25f0));
            call_ptr(c, 0x00317670, 0x00200ea8)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x00200eb4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00200eb4)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_200ed0;
            } else {
                c.cycles += 4;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00200ed0);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200ed0)?;
                break 'b_200ed0;
            }
        }
        'b_200f08: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x00200edc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x610));
            call_ptr(c, 0x00317670, 0x00200edc)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x00200ee8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00200ee8)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = 0x3;
            if t {
                break 'b_200f08;
            } else {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00200f08);
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200f08)?;
                break 'b_200f08;
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0u64;
        c.g[FP][0] = 0x470000;
        c.g[S7][0] = 0x470000;
        c.g[S6][0] = 0x470000;
        c.g[S5][0] = 0x1;
        c.g[S4][0] = 0x2;
        c.g[S2][0] = 0x3;
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        'l_200f28: loop {
            'b_20101c: {
                c.cycles += 50;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00200f3c);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x620));
                call_ptr(c, 0x00417828, 0x00200f3c)?;
                c.g[RA][0] = sx(0x00200f44);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00200f44)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00200f50);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00200f50)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00200f68);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200f68)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00200f78);
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x630));
                call_ptr(c, 0x00417828, 0x00200f78)?;
                c.g[RA][0] = sx(0x00200f80);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00200f80)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00200f8c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00200f8c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00200fa4);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200fa4)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00200fb4);
                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x648));
                call_ptr(c, 0x00417828, 0x00200fb4)?;
                c.g[RA][0] = sx(0x00200fbc);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00200fbc)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00200fc8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00200fc8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00200fe0);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00200fe0)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_20101c;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00201004);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00201004)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0020101c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0020101c)?;
                    break 'b_20101c;
                }
            }
            'b_201094: {
                c.cycles += 19;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020102c);
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x660));
                call_ptr(c, 0x00417828, 0x0020102c)?;
                c.g[RA][0] = sx(0x00201034);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x00201034)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00201040);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00201040)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S2][0]);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00201058);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                Callee {
                    addr: 0x00208438,
                    f: ee::s05::EventSelect_SetUnlockedColor,
                }
                .call(c, 0x00201058)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_201094;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0020107c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0020107c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00201094);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00201094)?;
                    break 'b_201094;
                }
            }
            c.cycles += 4;
            c.g[A2][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00200f28);
                }
                continue 'l_200f28;
            } else {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x274), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28c), c.g[V1][0]);
                'l_2010b0: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                    c.g[V1][0] = 0x360;
                    c.g[S6][0] = 0u64;
                    c.mult(0, 0, 2, 3);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lo[0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x290), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x284), c.g[V0][0]);
                    'l_2010d8: loop {
                        'b_20116c: {
                            'b_201104: {
                                c.cycles += 2;
                                let t = c.g[S6][0] != 0u64;
                                c.g[A2][0] = 0x1;
                                if t {
                                    break 'b_201104;
                                } else {
                                    c.cycles += 9;
                                    c.g[V0][0] = 0x470000;
                                    c.g[A1][0] = 0x4;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x278), c.g[A1][0]);
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x678));
                                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x678));
                                    c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                    let t = true;
                                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                                    if t {
                                        break 'b_20116c;
                                    } else {
                                        break 'b_201104;
                                    }
                                }
                            }
                            'b_201144: {
                                c.cycles += 2;
                                let t = c.g[S6][0] != c.g[A2][0];
                                c.g[V0][0] = 0x470000;
                                if t {
                                    break 'b_201144;
                                } else {
                                    c.cycles += 14;
                                    c.g[V0][0] = 0x470000;
                                    c.g[T0][0] = 0x5;
                                    c.g[A3][0] = c.g[V0][0];
                                    c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x688));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x688));
                                    c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x278), c.g[T0][0]);
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x14));
                                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                                    c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                    let t = true;
                                    c.sb((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A0][0]);
                                    if t {
                                        break 'b_20116c;
                                    } else {
                                        break 'b_201144;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[T1][0] = c.g[V0][0];
                            c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x6a0));
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x6a0));
                            c.g[V0][0] = 0x8;
                            c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x278), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            break 'b_20116c;
                        }
                        'b_20164c: {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x278));
                            c.g[S2][0] = 0u64;
                            let t = c.g[A0][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x294), c.g[V1][0]);
                            if t {
                                break 'b_20164c;
                            } else {
                                c.cycles += 18;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                                c.g[V0][0] = 0x1b0;
                                c.g[V1][0] = 0x21c;
                                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                c.mult(0, 0, 4, 2);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x288), c.g[A1][0]);
                                c.g[A0][0] = c.lo[0];
                                c.mult(0, 0, 2, 3);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x27c), c.g[A0][0]);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                                c.g[V0][0] = c.lo[0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x280), c.g[V0][0]);
                                c.g[V0][0] = 0x60;
                                c.mult(0, 23, 4, 2);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x27c));
                                'l_2011c8: loop {
                                    'b_2012ac: {
                                        'b_201260: {
                                            c.cycles += 9;
                                            c.g[A1][0] = 0x480000;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x7e30));
                                            c.lo[0] = c.g[V0][0];
                                            c.g[V0][0] = 0x6c;
                                            c.madd(0, 3, 18, 2);
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            c.g[V0][0] = c.g[V1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_201260;
                                            } else {
                                                c.cycles += 8;
                                                c.g[A2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x298),
                                                    c.g[A2][0],
                                                );
                                                c.g[A1][0] = (c.g[S6][0] < 0x2u64) as u64;
                                                c.g[S4][0] = sx((c.g[S2][0] as u32) << 2);
                                                c.g[S0][0] = sx((c.g[S6][0] as u32) << 5);
                                                c.g[S1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.g[S5][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x20));
                                                'l_20120c: loop {
                                                    c.cycles += 19;
                                                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(7, c.g[V1][0] as u32);
                                                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        9,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        6,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(7, c.g[A0][0] as u32);
                                                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        9,
                                                        (c.g[A0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        9,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        6,
                                                        (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        6,
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020120c);
                                                        }
                                                        continue 'l_20120c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_2012ac;
                                                        } else {
                                                            break 'b_201260;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x298), c.g[A3][0]);
                                        c.g[A1][0] = (c.g[S6][0] < 0x2u64) as u64;
                                        c.g[S4][0] = sx((c.g[S2][0] as u32) << 2);
                                        c.g[S0][0] = sx((c.g[S6][0] as u32) << 5);
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x20));
                                        'l_201280: loop {
                                            c.cycles += 11;
                                            c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                                            c.g[T1][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.g[A3][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.sd(c.g[A0][0] as u32, c.g[T0][0]);
                                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x18),
                                                c.g[A3][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00201280);
                                                }
                                                continue 'l_201280;
                                            } else {
                                                break 'b_2012ac;
                                            }
                                        }
                                    }
                                    'b_201394: {
                                        c.cycles += 8;
                                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.ldr(8, c.g[V1][0] as u32);
                                        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(8, c.g[A0][0] as u32);
                                        c.g[A3][0] = 0x1;
                                        let t = c.g[S6][0] != c.g[A3][0];
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                        if t {
                                            break 'b_201394;
                                        } else {
                                            'b_20137c: {
                                                'b_20134c: {
                                                    c.cycles += 10;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x280)
                                                        );
                                                    c.g[A2][0] = 0x480000;
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32)
                                                            .wrapping_sub(0x7920));
                                                    c.lo[0] = c.g[V0][0];
                                                    c.g[V0][0] = 0x6c;
                                                    c.madd(0, 3, 18, 2);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A2][0] as u32));
                                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[FP][0];
                                                    if t {
                                                        break 'b_20134c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        'l_2012f8: loop {
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(7, c.g[V1][0] as u32);
                                                            c.ldl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                9,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                9,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(7, c.g[A0][0] as u32);
                                                            c.sdl(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                9,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                9,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x002012f8,
                                                                    );
                                                                }
                                                                continue 'l_2012f8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;

                                                                if t {
                                                                    break 'b_20137c;
                                                                } else {
                                                                    break 'b_20134c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                                'l_201350: loop {
                                                    c.cycles += 11;
                                                    c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                                                    c.g[T0][0] =
                                                        c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.g[T1][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[A2][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        c.g[T0][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        c.g[T1][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        c.g[A2][0],
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00201350);
                                                        }
                                                        continue 'l_201350;
                                                    } else {
                                                        break 'b_20137c;
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.ldr(7, c.g[V1][0] as u32);
                                            c.g[T0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                            c.sdr(7, c.g[A0][0] as u32);
                                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                            break 'b_201394;
                                        }
                                    }
                                    'b_201464: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[S6][0] != c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x284));
                                        if t {
                                            break 'b_201464;
                                        } else {
                                            'b_20144c: {
                                                'b_20141c: {
                                                    c.cycles += 9;
                                                    c.g[A2][0] = 0x480000;
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32)
                                                            .wrapping_sub(0x72c8));
                                                    c.lo[0] = c.g[V0][0];
                                                    c.g[V0][0] = 0x6c;
                                                    c.madd(0, 3, 18, 2);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A2][0] as u32));
                                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[FP][0];
                                                    if t {
                                                        break 'b_20141c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        'l_2013c8: loop {
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(7, c.g[V1][0] as u32);
                                                            c.ldl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                9,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                9,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(7, c.g[A0][0] as u32);
                                                            c.sdl(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                9,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                9,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x002013c8,
                                                                    );
                                                                }
                                                                continue 'l_2013c8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;

                                                                if t {
                                                                    break 'b_20144c;
                                                                } else {
                                                                    break 'b_20141c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                                'l_201420: loop {
                                                    c.cycles += 11;
                                                    c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                                                    c.g[T0][0] =
                                                        c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.g[T1][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[A2][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        c.g[T0][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        c.g[T1][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        c.g[A2][0],
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00201420);
                                                        }
                                                        continue 'l_201420;
                                                    } else {
                                                        break 'b_20144c;
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.ldr(7, c.g[V1][0] as u32);
                                            c.g[T0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                            c.sdr(7, c.g[A0][0] as u32);
                                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                            break 'b_201464;
                                        }
                                    }
                                    'b_201488: {
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == 0u64;
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                                        if t {
                                            break 'b_201488;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x00201478);
                                            c.g[A1][0] = c.g[FP][0];
                                            Callee {
                                                addr: 0x00200b50,
                                                f: ee::s05::EventSelect_IsEventCompleted,
                                            }
                                            .call(c, 0x00201478)?;
                                            c.g[V1][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                            c.g[V1][0] =
                                                sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x140),
                                                c.g[V0][0],
                                            );
                                            break 'b_201488;
                                        }
                                    }
                                    'b_20153c: {
                                        'b_201538: {
                                            'b_201514: {
                                                c.cycles += 14;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[RA][0] = sx(0x0020149c);
                                                c.g[A3][0] = c.g[S2][0];
                                                call_ptr(c, 0x00417828, 0x0020149c)?;
                                                c.g[RA][0] = sx(0x002014a4);
                                                c.g[A0][0] = c.g[SP][0];
                                                call_ptr(c, 0x00317670, 0x002014a4)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                                                c.g[RA][0] = sx(0x002014b0);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x0039d860, 0x002014b0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                                                c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[S1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_201514;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x4c));
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                        .wrapping_add(c.g[S0][0] as u32));
                                                    if t {
                                                        break 'b_201514;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[S7][0] as u32));
                                                        c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        let t = c.g[S1][0] == 0u64;
                                                        c.sw(c.g[V0][0] as u32, 0u64);
                                                        if t {
                                                            break 'b_201538;
                                                        } else {
                                                            c.cycles += 14;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x002014f4);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x002014f4)?;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020150c);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0020150c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                            if t {
                                                                break 'b_20153c;
                                                            } else {
                                                                break 'b_201514;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x00201524);
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x274));
                                            Callee {
                                                addr: 0x00200d00,
                                                f: ee::s05::EventSelect_IsEventUnlocked,
                                            }
                                            .call(c, 0x00201524)?;
                                            c.g[V1][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                            c.g[V1][0] =
                                                sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                            break 'b_201538;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                        break 'b_20153c;
                                    }
                                    'b_201560: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[V0][0] ^ 0x17;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] =
                                            sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        if t {
                                            break 'b_201560;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[V0][0] =
                                                sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[RA][0] = sx(0x00201560);
                                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                            Callee {
                                                addr: 0x00208438,
                                                f: ee::s05::EventSelect_SetUnlockedColor,
                                            }
                                            .call(c, 0x00201560)?;
                                            break 'b_201560;
                                        }
                                    }
                                    'b_201638: {
                                        'b_20161c: {
                                            c.cycles += 4;
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.g[V0][0] = c.g[A1][0] ^ 0x17;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_20161c;
                                            } else {
                                                'b_2015b4: {
                                                    'b_2015a4: {
                                                        'b_20158c: {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = 0x1;
                                                            let t = c.g[V0][0] != c.g[V1][0];
                                                            c.g[S0][0] = sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x54));
                                                            if t {
                                                                break 'b_20158c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x00201584);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x270));
                                                                call_ptr(
                                                                    c, 0x00144c60, 0x00201584,
                                                                )?;
                                                                let t = true;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                if t {
                                                                    break 'b_2015a4;
                                                                } else {
                                                                    break 'b_20158c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2560));
                                                        c.g[RA][0] = sx(0x0020159c);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x004165a8, 0x0020159c)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_2015b4;
                                                        } else {
                                                            break 'b_2015a4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x002015ac);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_ptr(c, 0x003a0c50, 0x002015ac)?;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x18),
                                                        c.g[S2][0],
                                                    );
                                                    if t {
                                                        break 'b_201638;
                                                    } else {
                                                        break 'b_2015b4;
                                                    }
                                                }
                                                'b_201600: {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x48));
                                                    c.g[V0][0] = 0x4;
                                                    let t = c.g[A0][0] != c.g[V0][0];
                                                    c.g[A1][0] = 0x4a0000;
                                                    if t {
                                                        break 'b_201600;
                                                    } else {
                                                        c.cycles += 15;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x288));
                                                        c.g[RA][0] = sx(0x002015d0);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2590));
                                                        call_ptr(c, 0x002c2540, 0x002015d0)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x002015dc);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x270));
                                                        call_ptr(c, 0x00144c60, 0x002015dc)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x28c));
                                                        c.g[A2][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x002015ec);
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x288));
                                                        call_ptr(c, 0x002c26d0, 0x002015ec)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x28c));
                                                        c.g[RA][0] = sx(0x002015f8);
                                                        c.g[A0][0] = c.g[S1][0];
                                                        call_ptr(c, 0x003a0e90, 0x002015f8)?;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x18),
                                                            c.g[S2][0],
                                                        );
                                                        if t {
                                                            break 'b_201638;
                                                        } else {
                                                            break 'b_201600;
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[RA][0] = sx(0x00201608);
                                                call_ptr(c, 0x001fd190, 0x00201608)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00201614);
                                                c.g[A0][0] = c.g[S1][0];
                                                call_ptr(c, 0x003a0e90, 0x00201614)?;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x18),
                                                    c.g[S2][0],
                                                );
                                                if t {
                                                    break 'b_201638;
                                                } else {
                                                    break 'b_20161c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_201638;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00201638);
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00201638)?;
                                            break 'b_201638;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x298));
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x278));
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x27c));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002011c8);
                                        }
                                        continue 'l_2011c8;
                                    } else {
                                        break 'b_20164c;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x294));
                        c.g[V0][0] = ((c.g[S6][0] as i64) < 3i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002010d8);
                            }
                            continue 'l_2010d8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x274), c.g[A1][0]);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002010b0);
                                }
                                continue 'l_2010b0;
                            } else {
                                'b_2016a8: {
                                    c.cycles += 9;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00201674);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b8));
                                    call_ptr(c, 0x00317670, 0x00201674)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                                    c.g[RA][0] = sx(0x00201680);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x00201680)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    let t = c.g[S1][0] == 0u64;
                                    c.g[A1][0] = 0x4d0000;
                                    if t {
                                        break 'b_2016a8;
                                    } else {
                                        c.cycles += 7;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002016a8);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002016a8)?;
                                        break 'b_2016a8;
                                    }
                                }
                                'b_2016ec: {
                                    c.cycles += 9;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x002016b4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                                    call_ptr(c, 0x00317670, 0x002016b4)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                                    c.g[RA][0] = sx(0x002016c0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x002016c0)?;
                                    c.g[S1][0] = c.g[V0][0];
                                    let t = c.g[S1][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_2016ec;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        c.g[A1][0] = 0x4d0000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x002016ec);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002016ec)?;
                                        break 'b_2016ec;
                                    }
                                }
                                c.cycles += 12;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x330));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x320));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x310));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x300));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2f0));
                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2a0));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x340));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `EventSelect_RefreshPeakGoals` (0x00205098): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[cfg(test)] // not installed: its test runs away in the unslotted file-load chain
#[allow(dead_code)]
pub fn event_select_refresh_peak_goals(c: &mut Cpu) {
    if c.pc != 0x00205098 {
        return ee::s05::f_00205098__generated(c);
    }
    guest(c, |c| {
        'b_205138: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4f0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x480));
            c.g[S6][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x4c0));
            c.g[A0][0] = 0x470000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x4d0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x4b0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x4a0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x490));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x470));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x460));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x450), c.g[RA][0]);
            c.g[RA][0] = sx(0x002050d4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x578));
            call_ptr(c, 0x00317670, 0x002050d4)?;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x002050e0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002050e0)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[A1][0] = 0x4a0000;
                break 'b_205138;
            } else {
                c.cycles += 17;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x00205104);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe50));
                call_ptr(c, 0x0039e8b8, 0x00205104)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x00205114);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe68));
                call_ptr(c, 0x0039e8b8, 0x00205114)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x00205124);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe80));
                call_ptr(c, 0x0039e8b8, 0x00205124)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[RA][0] = sx(0x00205134);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe98));
                call_ptr(c, 0x0039e8b8, 0x00205134)?;
                c.g[A1][0] = 0x4a0000;
                break 'b_205138;
            }
        }
        'b_205178: {
            'b_20516c: {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[S0][0] = c.g[A1][0];
                c.g[RA][0] = sx(0x00205148);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d8));
                call_ptr(c, 0x0039e948, 0x00205148)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_20516c;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00205164);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x558));
                    call_ptr(c, 0x0039e948, 0x00205164)?;
                    let t = true;

                    if t {
                        break 'b_205178;
                    } else {
                        break 'b_20516c;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
            c.g[RA][0] = sx(0x00205178);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x548));
            call_ptr(c, 0x0039e948, 0x00205178)?;
            break 'b_205178;
        }
        'b_2051e8: {
            'b_2051d4: {
                'b_2051b8: {
                    'b_2051a4: {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x00205180);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x25d8));
                        call_ptr(c, 0x00317670, 0x00205180)?;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                        c.g[RA][0] = sx(0x0020518c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x0020518c)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        let t = c.g[V1][0] != 0u64;
                        c.g[S0][0] = c.g[V0][0];
                        if t {
                            break 'b_2051a4;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xeb0));
                            if t {
                                break 'b_2051b8;
                            } else {
                                break 'b_2051a4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_2051d4;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xec0));
                        break 'b_2051b8;
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x002051c0);
                call_ptr(c, 0x00317670, 0x002051c0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002051cc);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x002051cc)?;
                let t = true;

                if t {
                    break 'b_2051e8;
                } else {
                    break 'b_2051d4;
                }
            }
            c.cycles += 5;
            c.g[RA][0] = sx(0x002051dc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xed0));
            call_ptr(c, 0x00317670, 0x002051dc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002051e8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x002051e8)?;
            break 'b_2051e8;
        }
        'b_205280: {
            c.cycles += 38;
            c.g[RA][0] = sx(0x002051f0);
            c.g[S1][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x002051f0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002051fc);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002051fc)?;
            c.g[RA][0] = sx(0x00205204);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x400), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x00205204)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00205210);
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x00205210)?;
            c.g[RA][0] = sx(0x00205218);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x404), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x00205218)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00205224);
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x00205224)?;
            c.g[RA][0] = sx(0x0020522c);
            c.g[S7][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x0020522c)?;
            c.g[A1][0] = 0xb;
            c.g[RA][0] = sx(0x00205238);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x00205238)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x408), c.g[V0][0]);
            c.g[RA][0] = sx(0x00205244);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014a080, 0x00205244)?;
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
            c.g[A1][0] = 0x470000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x40c), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6d8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[RA][0] = sx(0x00205264);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00417828, 0x00205264)?;
            c.g[RA][0] = sx(0x0020526c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00317670, 0x0020526c)?;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
            c.g[RA][0] = sx(0x00205278);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00205278)?;
            if c.g[V0][0] != 0u64 {
                c.g[S1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                break 'b_205280;
            } else {
                break 'b_205280;
            }
        }
        'b_205304: {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x470000;
            if t {
                break 'b_205304;
            } else {
                'b_2052e0: {
                    'b_2052c4: {
                        'b_2052ac: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != 0u64;
                            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            if t {
                                break 'b_2052ac;
                            } else {
                                c.cycles += 5;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                let t = true;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xee0));
                                if t {
                                    break 'b_2052c4;
                                } else {
                                    break 'b_2052ac;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] != c.g[V1][0];
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                        if t {
                            break 'b_2052e0;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[A1][0] = 0x470000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xef0));
                            break 'b_2052c4;
                        }
                    }
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002052cc);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x002052cc)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002052d8);
                    c.g[A0][0] = c.g[S6][0];
                    Callee {
                        addr: 0x00206690,
                        f: ee::s05::EventSelect_ShowEventPaths,
                    }
                    .call(c, 0x002052d8)?;
                    let t = true;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_205304;
                    } else {
                        break 'b_2052e0;
                    }
                }
                c.cycles += 9;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf00));
                c.g[RA][0] = sx(0x002052f4);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x002052f4)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00205300);
                c.g[A0][0] = c.g[S6][0];
                Callee {
                    addr: 0x00206690,
                    f: ee::s05::EventSelect_ShowEventPaths,
                }
                .call(c, 0x00205300)?;
                c.g[V0][0] = 0x470000;
                break 'b_205304;
            }
        }
        'b_206660: {
            'b_20537c: {
                c.cycles += 23;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf18));
                c.g[RA][0] = sx(0x00205314);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039e8b8, 0x00205314)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf28));
                c.g[RA][0] = sx(0x00205328);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039e8b8, 0x00205328)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf38));
                c.g[RA][0] = sx(0x0020533c);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x0039e8b8, 0x0020533c)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf48));
                c.g[RA][0] = sx(0x00205350);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0039e8b8, 0x00205350)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0x470000;
                if t {
                    break 'b_20537c;
                } else {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00205368);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf58));
                    call_ptr(c, 0x00317670, 0x00205368)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00205374);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00205374)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
                    if t {
                        break 'b_206660;
                    } else {
                        break 'b_20537c;
                    }
                }
            }
            'b_2053ac: {
                c.cycles += 8;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                c.g[T0][0] = 0u64;
                c.g[RA][0] = sx(0x00205394);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40c));
                call_ptr(c, 0x00157bf0, 0x00205394)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                if t {
                    break 'b_2053ac;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x002053a8);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e948, 0x002053a8)?;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    break 'b_2053ac;
                }
            }
            'b_2053d8: {
                c.cycles += 7;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                c.g[T0][0] = 0x1;
                c.g[RA][0] = sx(0x002053c0);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40c));
                call_ptr(c, 0x00157bf0, 0x002053c0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                if t {
                    break 'b_2053d8;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x002053d4);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0039e948, 0x002053d4)?;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    break 'b_2053d8;
                }
            }
            'b_205404: {
                c.cycles += 7;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                c.g[T0][0] = 0x2;
                c.g[RA][0] = sx(0x002053ec);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40c));
                call_ptr(c, 0x00157bf0, 0x002053ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                if t {
                    break 'b_205404;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00205400);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x0039e948, 0x00205400)?;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    break 'b_205404;
                }
            }
            'b_205430: {
                c.cycles += 7;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                c.g[T0][0] = 0x3;
                c.g[RA][0] = sx(0x00205418);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40c));
                call_ptr(c, 0x00157bf0, 0x00205418)?;
                let t = c.g[V0][0] == 0u64;
                c.g[FP][0] = 0x1;
                if t {
                    break 'b_205430;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x0020542c);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0039e948, 0x0020542c)?;
                    c.g[FP][0] = 0x1;
                    break 'b_205430;
                }
            }
            'b_206418: {
                'b_2063fc: {
                    'b_205dac: {
                        'b_205968: {
                            c.cycles += 2;
                            let t = c.g[S1][0] == c.g[FP][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_205968;
                            } else {
                                'b_205468: {
                                    'b_205450: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_205450;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = 0x4;
                                            if t {
                                                break 'b_205468;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
                                                if t {
                                                    break 'b_206660;
                                                } else {
                                                    break 'b_205450;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_205dac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[A0][0] = 0x470000;
                                        if t {
                                            break 'b_206418;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
                                            if t {
                                                break 'b_206660;
                                            } else {
                                                break 'b_205468;
                                            }
                                        }
                                    }
                                }
                                'b_20558c: {
                                    c.cycles += 54;
                                    c.g[A0][0] = 0x470000;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x410), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x00205478);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf70));
                                    call_ptr(c, 0x00317670, 0x00205478)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00205484);
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x003a1000, 0x00205484)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00205490);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf90));
                                    call_ptr(c, 0x00317670, 0x00205490)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                                    c.g[RA][0] = sx(0x0020549c);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x0020549c)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[S5][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x470000;
                                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xfa8));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x002054c0);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x002054c0)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x002054d0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x002054d0)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[RA][0] = sx(0x002054e4);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    call_ptr(c, 0x002c26d0, 0x002054e4)?;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[RA][0] = sx(0x002054f0);
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x003a0e90, 0x002054f0)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    c.g[A1][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00205500);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc0));
                                    call_ptr(c, 0x0039e8b8, 0x00205500)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    c.g[A1][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00205510);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfd0));
                                    call_ptr(c, 0x0039e8b8, 0x00205510)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    c.g[A1][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00205520);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfe0));
                                    call_ptr(c, 0x0039e8b8, 0x00205520)?;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    c.g[RA][0] = sx(0x00205530);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xff0));
                                    call_ptr(c, 0x0039e8b8, 0x00205530)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x410));
                                    if t {
                                        break 'b_20558c;
                                    } else {
                                        c.cycles += 19;
                                        c.g[V1][0] = 0x3;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                        c.g[A1][0] = 0x470000;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x410), c.g[V1][0]);
                                        c.g[RA][0] = sx(0x00205558);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1000));
                                        call_ptr(c, 0x0039e8b8, 0x00205558)?;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                        c.g[A1][0] = 0x470000;
                                        c.g[RA][0] = sx(0x00205568);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1010));
                                        call_ptr(c, 0x0039e8b8, 0x00205568)?;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                        c.g[A1][0] = 0x470000;
                                        c.g[RA][0] = sx(0x00205578);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1020));
                                        call_ptr(c, 0x0039e8b8, 0x00205578)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                        c.g[RA][0] = sx(0x00205588);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1030));
                                        call_ptr(c, 0x0039e8b8, 0x00205588)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x410));
                                        break 'b_20558c;
                                    }
                                }
                                'b_205908: {
                                    c.cycles += 3;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x418), 0u64);
                                    let t = c.g[A0][0] == 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x414), 0u64);
                                    if t {
                                        break 'b_205908;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        'l_2055a0: loop {
                                            'b_2056b4: {
                                                'b_205654: {
                                                    c.cycles += 12;
                                                    c.g[A0][0] = 0x1b0;
                                                    c.g[V0][0] = 0x6c;
                                                    c.g[A2][0] = 0x480000;
                                                    c.mult(0, 3, 3, 4);
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32)
                                                            .wrapping_sub(0x7e30));
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x418)
                                                        );
                                                    c.mult(0, 2, 4, 2);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[A2][0] as u32));
                                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x2b0));
                                                    if t {
                                                        break 'b_205654;
                                                    } else {
                                                        c.cycles += 12;
                                                        c.g[A3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x240));
                                                        c.g[T0][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x380));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x434),
                                                            c.g[A3][0],
                                                        );
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x43c),
                                                            c.g[T0][0],
                                                        );
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x430),
                                                            c.g[A0][0],
                                                        );
                                                        c.g[S4][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x1b0));
                                                        c.g[S7][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x1d0));
                                                        c.g[S3][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x320));
                                                        c.g[S5][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x350));
                                                        c.g[FP][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x370));
                                                        'l_205600: loop {
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(6, c.g[V1][0] as u32);
                                                            c.ldl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(6, c.g[A1][0] as u32);
                                                            c.sdl(
                                                                7,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                7,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.sdr(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00205600,
                                                                    );
                                                                }
                                                                continue 'l_205600;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;

                                                                if t {
                                                                    break 'b_2056b4;
                                                                } else {
                                                                    break 'b_205654;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 13;
                                                c.g[T0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x418));
                                                c.g[A2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x240));
                                                c.g[A3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x380));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x434),
                                                    c.g[A2][0],
                                                );
                                                c.g[T0][0] =
                                                    sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x43c),
                                                    c.g[A3][0],
                                                );
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x430),
                                                    c.g[T0][0],
                                                );
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                                c.g[S4][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                                                c.g[S7][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
                                                c.g[S3][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x320));
                                                c.g[S5][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x350));
                                                c.g[FP][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x370));
                                                'l_205688: loop {
                                                    c.cycles += 11;
                                                    c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                                                    c.g[A2][0] =
                                                        c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.g[A3][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[T0][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                                                    c.sd(
                                                        (c.g[A1][0] as u32).wrapping_add(0x8),
                                                        c.g[A2][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                                        c.g[A3][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                                        c.g[T0][0],
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00205688);
                                                        }
                                                        continue 'l_205688;
                                                    } else {
                                                        break 'b_2056b4;
                                                    }
                                                }
                                            }
                                            'b_2057b4: {
                                                'b_2056f4: {
                                                    c.cycles += 9;
                                                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[V1][0] as u32);
                                                    c.g[A0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                                                    c.sdr(2, c.g[A1][0] as u32);
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x8),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2c0)
                                                        );
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2b0)
                                                        );
                                                    if t {
                                                        break 'b_2056f4;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[RA][0] = sx(0x002056e0);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x400));
                                                        call_ptr(c, 0x00144c60, 0x002056e0)?;
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x002056ec);
                                                        c.g[A0][0] = c.g[S7][0];
                                                        call_ptr(c, 0x002c27c0, 0x002056ec)?;
                                                        let t = true;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x418));
                                                        if t {
                                                            break 'b_2057b4;
                                                        } else {
                                                            break 'b_2056f4;
                                                        }
                                                    }
                                                }
                                                'b_205720: {
                                                    c.cycles += 7;
                                                    c.g[S0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                                    c.g[T0][0] = 0x4a0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[T0][0] as u32)
                                                            .wrapping_add(0x2560));
                                                    c.g[RA][0] = sx(0x00205708);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x004165a8, 0x00205708)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_205720;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x00205718);
                                                        c.g[A0][0] = c.g[S7][0];
                                                        call_ptr(c, 0x002c2540, 0x00205718)?;
                                                        let t = true;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x418));
                                                        if t {
                                                            break 'b_2057b4;
                                                        } else {
                                                            break 'b_205720;
                                                        }
                                                    }
                                                }
                                                'b_20579c: {
                                                    c.cycles += 4;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2b8)
                                                        );
                                                    c.g[V0][0] = 0x4;
                                                    let t = c.g[A0][0] != c.g[V0][0];

                                                    if t {
                                                        break 'b_20579c;
                                                    } else {
                                                        c.cycles += 27;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x00205740);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2698));
                                                        call_ptr(c, 0x002c2540, 0x00205740)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x400));
                                                        c.g[RA][0] = sx(0x0020574c);
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b0));
                                                        call_ptr(c, 0x00144c60, 0x0020574c)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[S2][0] = c.g[V0][0];
                                                        c.g[A0][0] = 0x470000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8c));
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x1ab8));
                                                        c.g[S1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[V0][0] = c
                                                            .lh((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        c.g[RA][0] = sx(0x00205770);
                                                        c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        call_ptr(c, 0x00317670, 0x00205770)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x24));
                                                        c.g[A0][0] = c.g[S0][0];
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[RA][0] = sx(0x00205780);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, t, 0x00205780)?;
                                                        c.g[A2][0] = c.g[S2][0];
                                                        c.g[A3][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S7][0];
                                                        c.g[RA][0] = sx(0x00205794);
                                                        c.g[A1][0] = c.g[S3][0];
                                                        call_ptr(c, 0x002c26d0, 0x00205794)?;
                                                        let t = true;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x418));
                                                        if t {
                                                            break 'b_2057b4;
                                                        } else {
                                                            break 'b_20579c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x002057a4);
                                                call_ptr(c, 0x001fd190, 0x002057a4)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x002057b0);
                                                c.g[A0][0] = c.g[S7][0];
                                                call_ptr(c, 0x002c2508, 0x002057b0)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x418));
                                                break 'b_2057b4;
                                            }
                                            'b_205894: {
                                                'b_205884: {
                                                    c.cycles += 34;
                                                    c.g[V0][0] = 0x470000;
                                                    c.g[A1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1040));
                                                    c.g[RA][0] = sx(0x002057c4);
                                                    c.g[A0][0] = c.g[S4][0];
                                                    call_ptr(c, 0x00417828, 0x002057c4)?;
                                                    c.g[V1][0] = 0x470000;
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x418)
                                                        );
                                                    c.g[A1][0] =
                                                        sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x1058));
                                                    c.g[RA][0] = sx(0x002057d8);
                                                    c.g[A0][0] = c.g[S3][0];
                                                    call_ptr(c, 0x00417828, 0x002057d8)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S6][0] as u32).wrapping_add(0x2ec)
                                                        );
                                                    c.g[RA][0] = sx(0x002057e4);
                                                    c.g[A1][0] = c.g[S3][0];
                                                    call_ptr(c, 0x0039e948, 0x002057e4)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x418)
                                                        );
                                                    c.g[A1][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1068));
                                                    c.g[RA][0] = sx(0x002057f8);
                                                    c.g[A0][0] = c.g[S5][0];
                                                    call_ptr(c, 0x00417828, 0x002057f8)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S6][0] as u32).wrapping_add(0x2ec)
                                                        );
                                                    c.g[RA][0] = sx(0x00205804);
                                                    c.g[A1][0] = c.g[S5][0];
                                                    call_ptr(c, 0x0039e948, 0x00205804)?;
                                                    c.g[A2][0] = 0x4a0000;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x434)
                                                        );
                                                    c.g[RA][0] = sx(0x00205814);
                                                    c.g[A1][0] =
                                                        sx((c.g[A2][0] as u32)
                                                            .wrapping_add(0x2560));
                                                    call_ptr(c, 0x002c2540, 0x00205814)?;
                                                    c.g[A3][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2b8)
                                                        );
                                                    c.g[A1][0] = 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x404)
                                                        );
                                                    c.g[T0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2b0)
                                                        );
                                                    c.g[RA][0] = sx(0x0020582c);
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x40c)
                                                        );
                                                    call_ptr(c, 0x00158910, 0x0020582c)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffffffffffffffff;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.g[A3][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x414)
                                                        );
                                                    if t {
                                                        break 'b_205884;
                                                    } else {
                                                        c.cycles += 18;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[FP][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x23c0));
                                                        c.g[A3][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[RA][0] = sx(0x00205854);
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x414),
                                                            c.g[A3][0],
                                                        );
                                                        call_ptr(c, 0x002c2540, 0x00205854)?;
                                                        c.g[A1][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x00205860);
                                                        c.g[A0][0] = c.g[S6][0];
                                                        Callee {
                                                            addr: 0x00204ff0,
                                                            f: ee::s05::EventSelect_GetMedalText,
                                                        }
                                                        .call(c, 0x00205860)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x434));
                                                        c.g[A1][0] = c.g[FP][0];
                                                        c.g[RA][0] = sx(0x00205870);
                                                        c.g[A2][0] = c.g[V0][0];
                                                        call_ptr(c, 0x002c26d0, 0x00205870)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                        c.g[RA][0] = sx(0x0020587c);
                                                        c.g[A1][0] = c.g[S4][0];
                                                        call_ptr(c, 0x0039e948, 0x0020587c)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x430));
                                                        if t {
                                                            break 'b_205894;
                                                        } else {
                                                            break 'b_205884;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                                c.g[RA][0] = sx(0x00205890);
                                                c.g[A1][0] = c.g[S4][0];
                                                call_ptr(c, 0x0039e8b8, 0x00205890)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x430));
                                                break 'b_205894;
                                            }
                                            c.cycles += 29;
                                            c.g[T0][0] = 0x470000;
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x418));
                                            c.g[A1][0] =
                                                sx((c.g[T0][0] as u32).wrapping_add(0x1078));
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x43c));
                                            c.g[RA][0] = sx(0x002058ac);
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x418),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x00417828, 0x002058ac)?;
                                            c.g[RA][0] = sx(0x002058b4);
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x43c));
                                            call_ptr(c, 0x00317670, 0x002058b4)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                                            c.g[RA][0] = sx(0x002058c0);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x002058c0)?;
                                            c.g[V1][0] = 0x4a0000;
                                            c.g[S5][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x2550));
                                            c.g[RA][0] = sx(0x002058d4);
                                            c.g[A0][0] = c.g[FP][0];
                                            call_ptr(c, 0x002c2540, 0x002058d4)?;
                                            c.g[A3][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x434));
                                            c.g[A2][0] = c.g[S7][0];
                                            c.g[A0][0] = c.g[SP][0];
                                            c.g[RA][0] = sx(0x002058e8);
                                            c.g[A1][0] = c.g[FP][0];
                                            call_ptr(c, 0x002c26d0, 0x002058e8)?;
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[RA][0] = sx(0x002058f4);
                                            c.g[A1][0] = c.g[SP][0];
                                            call_ptr(c, 0x003a0e90, 0x002058f4)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x418));
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x410));
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002055a0);
                                                }
                                                continue 'l_2055a0;
                                            } else {
                                                break 'b_205908;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 24;
                                c.g[A3][0] = 0x470000;
                                c.g[RA][0] = sx(0x00205914);
                                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1088));
                                call_ptr(c, 0x00317670, 0x00205914)?;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00205920);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00205920)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S5][0] = c.g[V0][0];
                                c.g[T0][0] = 0x470000;
                                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10a0));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x00205944);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x00205944)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00205954);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x00205954)?;
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x414));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x410));
                                let t = true;
                                c.g[A0][0] = c.g[SP][0];
                                if t {
                                    break 'b_2063fc;
                                } else {
                                    break 'b_205968;
                                }
                            }
                        }
                        c.cycles += 52;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x00205974);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10b8));
                        call_ptr(c, 0x00317670, 0x00205974)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x41c), 0u64);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00205988);
                        c.g[S2][0] = 0u64;
                        call_ptr(c, 0x003a1000, 0x00205988)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x00205994);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf90));
                        call_ptr(c, 0x00317670, 0x00205994)?;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                        c.g[RA][0] = sx(0x002059a0);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002059a0)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S5][0] = c.g[V0][0];
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10d0));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x002059c4);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x002059c4)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002059d4);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x002059d4)?;
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x002059e8);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x002c26d0, 0x002059e8)?;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x002059f4);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0e90, 0x002059f4)?;
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3d0));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x340));
                        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3a0));
                        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3b0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x438), c.g[V0][0]);
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x440), c.g[V1][0]);
                        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x420), c.g[A0][0]);
                        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x424), c.g[A2][0]);
                        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x428), c.g[A3][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x42c), c.g[T0][0]);
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        'l_205a38: loop {
                            'b_205af4: {
                                'b_205ac0: {
                                    c.cycles += 11;
                                    c.g[A0][0] = 0x21c;
                                    c.g[V0][0] = 0x6c;
                                    c.mult(0, 2, 18, 2);
                                    c.mult(0, 3, 3, 4);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V0][0] = 0x480000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7920));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x438));
                                    if t {
                                        break 'b_205ac0;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        'l_205a6c: loop {
                                            c.cycles += 19;
                                            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                                            c.ldr(4, c.g[V1][0] as u32);
                                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                                            c.sdr(4, c.g[A1][0] as u32);
                                            c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                                            c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                                            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                                            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                                            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00205a6c);
                                                }
                                                continue 'l_205a6c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_205af4;
                                                } else {
                                                    break 'b_205ac0;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                'l_205ac8: loop {
                                    c.cycles += 11;
                                    c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00205ac8);
                                        }
                                        continue 'l_205ac8;
                                    } else {
                                        break 'b_205af4;
                                    }
                                }
                            }
                            'b_205bb8: {
                                'b_205b34: {
                                    c.cycles += 9;
                                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, c.g[V1][0] as u32);
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                                    c.sdr(2, c.g[A1][0] as u32);
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2a0));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
                                    if t {
                                        break 'b_205b34;
                                    } else {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00205b20);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400));
                                        call_ptr(c, 0x00144c60, 0x00205b20)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00205b2c);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x002c27c0, 0x00205b2c)?;
                                        let t = true;
                                        c.g[V1][0] = 0x470000;
                                        if t {
                                            break 'b_205bb8;
                                        } else {
                                            break 'b_205b34;
                                        }
                                    }
                                }
                                'b_205b60: {
                                    c.cycles += 7;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2a4));
                                    c.g[T0][0] = 0x4a0000;
                                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2560));
                                    c.g[RA][0] = sx(0x00205b48);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x004165a8, 0x00205b48)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        break 'b_205b60;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00205b58);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x002c2540, 0x00205b58)?;
                                        let t = true;
                                        c.g[V1][0] = 0x470000;
                                        if t {
                                            break 'b_205bb8;
                                        } else {
                                            break 'b_205b60;
                                        }
                                    }
                                }
                                'b_205ba0: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x298));
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_205ba0;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                                        c.g[RA][0] = sx(0x00205b7c);
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26a0));
                                        call_ptr(c, 0x002c2540, 0x00205b7c)?;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
                                        c.g[RA][0] = sx(0x00205b88);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400));
                                        call_ptr(c, 0x00144c60, 0x00205b88)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[RA][0] = sx(0x00205b98);
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                                        call_ptr(c, 0x002c26d0, 0x00205b98)?;
                                        let t = true;
                                        c.g[V1][0] = 0x470000;
                                        if t {
                                            break 'b_205bb8;
                                        } else {
                                            break 'b_205ba0;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x00205ba8);
                                call_ptr(c, 0x001fd190, 0x00205ba8)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00205bb4);
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x002c2508, 0x00205bb4)?;
                                c.g[V1][0] = 0x470000;
                                break 'b_205bb8;
                            }
                            'b_205ca0: {
                                'b_205c90: {
                                    c.cycles += 34;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1040));
                                    c.g[RA][0] = sx(0x00205bc8);
                                    c.g[A2][0] = c.g[S2][0];
                                    call_ptr(c, 0x00417828, 0x00205bc8)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x420));
                                    c.g[A3][0] = 0x470000;
                                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1058));
                                    c.g[RA][0] = sx(0x00205bdc);
                                    c.g[A2][0] = c.g[S2][0];
                                    call_ptr(c, 0x00417828, 0x00205bdc)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x420));
                                    c.g[RA][0] = sx(0x00205be8);
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    call_ptr(c, 0x0039e948, 0x00205be8)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x424));
                                    c.g[T0][0] = 0x470000;
                                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1068));
                                    c.g[RA][0] = sx(0x00205bfc);
                                    c.g[A2][0] = c.g[S2][0];
                                    call_ptr(c, 0x00417828, 0x00205bfc)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x424));
                                    c.g[RA][0] = sx(0x00205c08);
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                    call_ptr(c, 0x0039e948, 0x00205c08)?;
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S7][0];
                                    c.g[RA][0] = sx(0x00205c18);
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2560));
                                    call_ptr(c, 0x002c2540, 0x00205c18)?;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x298));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
                                    c.g[RA][0] = sx(0x00205c30);
                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40c));
                                    call_ptr(c, 0x00158910, 0x00205c30)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    let t = c.g[A1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_205c90;
                                    } else {
                                        c.cycles += 20;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c0));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23c0));
                                        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x3a0), c.g[A2][0]);
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x41c));
                                        c.sb((c.g[SP][0] as u32).wrapping_add(0x3a4), c.g[V1][0]);
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                        c.g[RA][0] = sx(0x00205c6c);
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x41c), c.g[A2][0]);
                                        Callee {
                                            addr: 0x00204ff0,
                                            f: ee::s05::EventSelect_GetMedalText,
                                        }
                                        .call(c, 0x00205c6c)?;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x428));
                                        c.g[A0][0] = c.g[S7][0];
                                        c.g[RA][0] = sx(0x00205c7c);
                                        c.g[A2][0] = c.g[V0][0];
                                        call_ptr(c, 0x002c27c0, 0x00205c7c)?;
                                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                        c.g[RA][0] = sx(0x00205c88);
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, 0x0039e948, 0x00205c88)?;
                                        let t = true;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x42c));
                                        if t {
                                            break 'b_205ca0;
                                        } else {
                                            break 'b_205c90;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                c.g[RA][0] = sx(0x00205c9c);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x0039e8b8, 0x00205c9c)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x42c));
                                break 'b_205ca0;
                            }
                            c.cycles += 43;
                            c.g[A3][0] = 0x470000;
                            c.g[A2][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00205cb0);
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1078));
                            call_ptr(c, 0x00417828, 0x00205cb0)?;
                            c.g[RA][0] = sx(0x00205cb8);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x42c));
                            call_ptr(c, 0x00317670, 0x00205cb8)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00205cc4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00205cc4)?;
                            c.g[T0][0] = 0x4a0000;
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[T0][0];
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x2550));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2550));
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x440));
                            c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = c.g[S7][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3d0), c.g[V1][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x3d4), c.g[V0][0]);
                            c.g[RA][0] = sx(0x00205cf8);
                            c.g[A2][0] = c.g[S4][0];
                            call_ptr(c, 0x002c27c0, 0x00205cf8)?;
                            c.g[A1][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00205d04);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x003a0e90, 0x00205d04)?;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00205d1c);
                            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00205d1c)?;
                            c.g[A3][0] = 0x470000;
                            c.g[A2][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10f0));
                            c.g[RA][0] = sx(0x00205d30);
                            c.g[A0][0] = c.g[FP][0];
                            call_ptr(c, 0x00417828, 0x00205d30)?;
                            c.g[S2][0] = c.g[S3][0];
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                            c.g[RA][0] = sx(0x00205d40);
                            c.g[A1][0] = c.g[FP][0];
                            call_ptr(c, 0x0039e948, 0x00205d40)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00205a38);
                                }
                                continue 'l_205a38;
                            } else {
                                c.cycles += 24;
                                c.g[T0][0] = 0x470000;
                                c.g[RA][0] = sx(0x00205d58);
                                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1088));
                                call_ptr(c, 0x00317670, 0x00205d58)?;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00205d64);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00205d64)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S5][0] = c.g[V0][0];
                                c.g[V0][0] = 0x470000;
                                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10a0));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[RA][0] = sx(0x00205d88);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                call_ptr(c, 0x00317670, 0x00205d88)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00205d98);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x00205d98)?;
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x41c));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[SP][0];
                                let t = true;
                                c.g[A3][0] = 0x5;
                                if t {
                                    break 'b_2063fc;
                                } else {
                                    break 'b_205dac;
                                }
                            }
                        }
                    }
                    'b_2060bc: {
                        'b_205f1c: {
                            c.cycles += 83;
                            c.g[A0][0] = 0x470000;
                            c.g[FP][0] = 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1108));
                            c.g[RA][0] = sx(0x00205dc0);
                            c.g[S4][0] = 0u64;
                            call_ptr(c, 0x00317670, 0x00205dc0)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00205dcc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00205dcc)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00205dd8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1128));
                            call_ptr(c, 0x00317670, 0x00205dd8)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00205de4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00205de4)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = 0x470000;
                            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1140));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x00205e08);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x00205e08)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00205e18);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x00205e18)?;
                            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00205e2c);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x002c26d0, 0x00205e2c)?;
                            c.g[A1][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00205e38);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x003a0e90, 0x00205e38)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[RA][0] = sx(0x00205e44);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x001543e0, 0x00205e44)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[S3][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00205e54);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00154368, 0x00205e54)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x470000;
                            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1158));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x00205e78);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x00205e78)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00205e88);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x00205e88)?;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00205e9c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x00205e9c)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00205ea8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1170));
                            call_ptr(c, 0x00317670, 0x00205ea8)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00205eb4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00205eb4)?;
                            c.g[A1][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00205ec0);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0e90, 0x00205ec0)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00205ecc);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1188));
                            call_ptr(c, 0x00317670, 0x00205ecc)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00205ed8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00205ed8)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                            c.g[RA][0] = sx(0x00205eec);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x00158890, 0x00205eec)?;
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            if t {
                                break 'b_205f1c;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[S4][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00205f14);
                                c.g[A0][0] =
                                    sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00205f14)?;
                                let t = true;

                                if t {
                                    break 'b_2060bc;
                                } else {
                                    break 'b_205f1c;
                                }
                            }
                        }
                        'b_206060: {
                            'b_206034: {
                                'b_205f7c: {
                                    c.cycles += 6;
                                    c.g[A2][0] = 0x1;
                                    c.g[RA][0] = sx(0x00205f28);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    call_ptr(c, 0x00158890, 0x00205f28)?;
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        break 'b_205f7c;
                                    } else {
                                        c.cycles += 18;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11a0));
                                        c.g[S4][0] = 0x1;
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00205f58);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x00205f58)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00205f68);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x00205f68)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        let t = true;
                                        c.g[A2][0] = 0u64;
                                        if t {
                                            break 'b_206034;
                                        } else {
                                            break 'b_205f7c;
                                        }
                                    }
                                }
                                'b_205fdc: {
                                    c.cycles += 6;
                                    c.g[A2][0] = 0x2;
                                    c.g[RA][0] = sx(0x00205f88);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    call_ptr(c, 0x00158890, 0x00205f88)?;
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        break 'b_205fdc;
                                    } else {
                                        c.cycles += 18;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11b8));
                                        c.g[S4][0] = 0x1;
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00205fb8);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x00205fb8)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00205fc8);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x00205fc8)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        let t = true;
                                        c.g[A2][0] = 0x1;
                                        if t {
                                            break 'b_206034;
                                        } else {
                                            break 'b_205fdc;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A2][0] = 0x3;
                                c.g[RA][0] = sx(0x00205fe8);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                call_ptr(c, 0x00158890, 0x00205fe8)?;
                                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_206060;
                                } else {
                                    c.cycles += 16;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11c8));
                                    c.g[S4][0] = 0x1;
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x00206014);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x00206014)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206024);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x00206024)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                    c.g[A2][0] = 0x2;
                                    break 'b_206034;
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x0020603c);
                            call_ptr(c, 0x00158890, 0x0020603c)?;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x0020604c);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x0020604c)?;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[RA][0] = sx(0x00206058);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x00206058)?;
                            let t = true;

                            if t {
                                break 'b_2060bc;
                            } else {
                                break 'b_206060;
                            }
                        }
                        c.cycles += 23;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11d8));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x0020607c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x0020607c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0020608c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x0020608c)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        c.g[RA][0] = sx(0x002060a0);
                        c.g[A2][0] = 0x3;
                        call_ptr(c, 0x00158890, 0x002060a0)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x002060b0);
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x002c26d0, 0x002060b0)?;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x002060bc);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0e90, 0x002060bc)?;
                        break 'b_2060bc;
                    }
                    'b_2060ec: {
                        'b_2060dc: {
                            c.cycles += 2;
                            let t = c.g[S4][0] == 0u64;
                            c.g[A1][0] = 0x470000;
                            if t {
                                break 'b_2060dc;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11e8));
                                c.g[RA][0] = sx(0x002060d4);
                                c.g[FP][0] = 0x1;
                                call_ptr(c, 0x0039e948, 0x002060d4)?;
                                let t = true;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                if t {
                                    break 'b_2060ec;
                                } else {
                                    break 'b_2060dc;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                        c.g[RA][0] = sx(0x002060e8);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11e8));
                        call_ptr(c, 0x0039e8b8, 0x002060e8)?;
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        break 'b_2060ec;
                    }
                    'b_206374: {
                        'b_2061d4: {
                            c.cycles += 49;
                            c.g[A0][0] = c.g[S7][0];
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x002060fc);
                            c.g[S4][0] = 0u64;
                            call_ptr(c, 0x001535c0, 0x002060fc)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[S3][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0020610c);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00153390, 0x0020610c)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x470000;
                            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1200));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x00206130);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x00206130)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00206140);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x00206140)?;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00206154);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x00206154)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00206160);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1218));
                            call_ptr(c, 0x00317670, 0x00206160)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x0020616c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0020616c)?;
                            c.g[A1][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00206178);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0e90, 0x00206178)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00206184);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1230));
                            call_ptr(c, 0x00317670, 0x00206184)?;
                            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00206190);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00206190)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[S5][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                            c.g[RA][0] = sx(0x002061a4);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x00158870, 0x002061a4)?;
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            if t {
                                break 'b_2061d4;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[S4][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002061cc);
                                c.g[A0][0] =
                                    sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002061cc)?;
                                let t = true;

                                if t {
                                    break 'b_206374;
                                } else {
                                    break 'b_2061d4;
                                }
                            }
                        }
                        'b_206318: {
                            'b_2062ec: {
                                'b_206234: {
                                    c.cycles += 6;
                                    c.g[A2][0] = 0x1;
                                    c.g[RA][0] = sx(0x002061e0);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    call_ptr(c, 0x00158870, 0x002061e0)?;
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        break 'b_206234;
                                    } else {
                                        c.cycles += 18;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11a0));
                                        c.g[S4][0] = 0x1;
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00206210);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x00206210)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00206220);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x00206220)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        let t = true;
                                        c.g[A2][0] = 0u64;
                                        if t {
                                            break 'b_2062ec;
                                        } else {
                                            break 'b_206234;
                                        }
                                    }
                                }
                                'b_206294: {
                                    c.cycles += 6;
                                    c.g[A2][0] = 0x2;
                                    c.g[RA][0] = sx(0x00206240);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    call_ptr(c, 0x00158870, 0x00206240)?;
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        break 'b_206294;
                                    } else {
                                        c.cycles += 18;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11b8));
                                        c.g[S4][0] = 0x1;
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00206270);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x00206270)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00206280);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x00206280)?;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                        let t = true;
                                        c.g[A2][0] = 0x1;
                                        if t {
                                            break 'b_2062ec;
                                        } else {
                                            break 'b_206294;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A2][0] = 0x3;
                                c.g[RA][0] = sx(0x002062a0);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                call_ptr(c, 0x00158870, 0x002062a0)?;
                                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_206318;
                                } else {
                                    c.cycles += 16;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11c8));
                                    c.g[S4][0] = 0x1;
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x002062cc);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x002062cc)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x002062dc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x002062dc)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                                    c.g[A2][0] = 0x2;
                                    break 'b_2062ec;
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x002062f4);
                            call_ptr(c, 0x00158870, 0x002062f4)?;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00206304);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x00206304)?;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[RA][0] = sx(0x00206310);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x00206310)?;
                            let t = true;

                            if t {
                                break 'b_206374;
                            } else {
                                break 'b_206318;
                            }
                        }
                        c.cycles += 23;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11d8));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x00206334);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x00206334)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00206344);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x00206344)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                        c.g[RA][0] = sx(0x00206358);
                        c.g[A2][0] = 0x3;
                        call_ptr(c, 0x00158870, 0x00206358)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00206368);
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x002c26d0, 0x00206368)?;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00206374);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0e90, 0x00206374)?;
                        break 'b_206374;
                    }
                    'b_2063a4: {
                        'b_206394: {
                            c.cycles += 2;
                            let t = c.g[S4][0] == 0u64;
                            c.g[A1][0] = 0x470000;
                            if t {
                                break 'b_206394;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1248));
                                c.g[RA][0] = sx(0x0020638c);
                                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                                call_ptr(c, 0x0039e948, 0x0020638c)?;
                                let t = true;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_2063a4;
                                } else {
                                    break 'b_206394;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                        c.g[RA][0] = sx(0x002063a0);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1248));
                        call_ptr(c, 0x0039e8b8, 0x002063a0)?;
                        c.g[A0][0] = 0x470000;
                        break 'b_2063a4;
                    }
                    c.cycles += 22;
                    c.g[RA][0] = sx(0x002063ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1260));
                    call_ptr(c, 0x00317670, 0x002063ac)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002063b8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002063b8)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10a0));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x002063dc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x002063dc)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002063ec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x002063ec)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[FP][0];
                    c.g[A3][0] = 0x2;
                    break 'b_2063fc;
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x00206404);
                call_ptr(c, 0x002c26d0, 0x00206404)?;
                c.g[A0][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x00206410);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a0e90, 0x00206410)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
                if t {
                    break 'b_206660;
                } else {
                    break 'b_206418;
                }
            }
            'b_206650: {
                c.cycles += 137;
                c.g[RA][0] = sx(0x00206420);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1278));
                call_ptr(c, 0x00317670, 0x00206420)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020642c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x0020642c)?;
                c.g[A0][0] = 0x470000;
                c.g[RA][0] = sx(0x00206438);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1298));
                call_ptr(c, 0x00317670, 0x00206438)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00206444);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00206444)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S5][0] = c.g[V0][0];
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12b0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x00206468);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x00206468)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00206478);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x00206478)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0020648c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x002c26d0, 0x0020648c)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00206498);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x003a0e90, 0x00206498)?;
                c.g[A0][0] = 0x470000;
                c.g[RA][0] = sx(0x002064a4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12c8));
                call_ptr(c, 0x00317670, 0x002064a4)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x002064b0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x002064b0)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x404));
                c.g[S5][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002064c0);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                call_ptr(c, 0x001588b0, 0x002064c0)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12e0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x002064e4);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x002064e4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x002064f4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, t, 0x002064f4)?;
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[S0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00206504);
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x00198af0, 0x00206504)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00206510);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x002c2540, 0x00206510)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00206520);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x002c26d0, 0x00206520)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x0020652c);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x003a0e90, 0x0020652c)?;
                c.g[A0][0] = 0x470000;
                c.g[RA][0] = sx(0x00206538);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12f0));
                call_ptr(c, 0x00317670, 0x00206538)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x00206544);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00206544)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x408));
                c.g[S5][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00206554);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001509f8, 0x00206554)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1300));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x00206578);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x00206578)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00206588);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, t, 0x00206588)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00206594);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00198af0, 0x00206594)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x002065a0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x002c2540, 0x002065a0)?;
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002065b4);
                c.g[S3][0] = 0u64;
                call_ptr(c, 0x002c26d0, 0x002065b4)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x002065c0);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x003a0e90, 0x002065c0)?;
                c.g[S2][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                c.g[A0][0] = 0x470000;
                if c.g[S2][0] == 0 {
                    c.g[S3][0] = c.g[FP][0];
                }
                c.g[RA][0] = sx(0x002065d4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1318));
                call_ptr(c, 0x00317670, 0x002065d4)?;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2e8));
                c.g[RA][0] = sx(0x002065e0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x002065e0)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S5][0] = c.g[V0][0];
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10a0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x00206604);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x00206604)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00206614);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x00206614)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00206628);
                c.g[A3][0] = 0x1;
                call_ptr(c, 0x002c26d0, 0x00206628)?;
                c.g[A0][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x00206634);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a0e90, 0x00206634)?;
                let t = (c.g[S3][0] as i64) <= 0;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_206650;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00206648);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1330));
                    call_ptr(c, 0x0039e948, 0x00206648)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
                    if t {
                        break 'b_206660;
                    } else {
                        break 'b_206650;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x2ec));
            c.g[RA][0] = sx(0x0020665c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1330));
            call_ptr(c, 0x0039e8b8, 0x0020665c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4e0));
            break 'b_206660;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x4d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x4c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x4b0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x480));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x450));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4f0));
        c.pc = t;
        return Ok(());
    });
}

/// `EventSelect_ShowEventPaths` (0x00206690): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn event_select_show_event_paths(c: &mut Cpu) {
    if c.pc != 0x00206690 {
        return ee::s05::f_00206690__generated(c);
    }
    guest(c, |c| {
        'b_207410: {
            'b_206a34: {
                'b_20686c: {
                    c.cycles += 112;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 6);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5520));
                    c.g[T4][0] = 0x470000;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x551c));
                    c.g[T3][0] = 0x470000;
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5518));
                    c.g[T2][0] = 0x470000;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5514));
                    c.g[T1][0] = 0x470000;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5510));
                    c.g[T0][0] = 0x4a0000;
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x550c));
                    c.g[A3][0] = 0x4a0000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[A2][0] = 0x4a0000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[V1][0] = 0x4a0000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[V0][0] = 0x4a0000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
                    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.swc1(6, c.g[SP][0] as u32);
                    c.g[T7][0] = sx((c.g[T4][0] as u32).wrapping_add(0x13a8));
                    c.ldl(5, (c.g[T7][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[T7][0] as u32);
                    c.ldl(13, (c.g[T7][0] as u32).wrapping_add(0xf));
                    c.ldr(13, (c.g[T7][0] as u32).wrapping_add(0x8));
                    c.ldl(14, (c.g[T7][0] as u32).wrapping_add(0x17));
                    c.ldr(14, (c.g[T7][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x2f));
                    c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.sdl(14, (c.g[SP][0] as u32).wrapping_add(0x37));
                    c.sdr(14, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[T6][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1420));
                    c.ldl(5, (c.g[T6][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[T6][0] as u32);
                    c.ldl(12, (c.g[T6][0] as u32).wrapping_add(0xf));
                    c.ldr(12, (c.g[T6][0] as u32).wrapping_add(0x8));
                    c.ldl(13, (c.g[T6][0] as u32).wrapping_add(0x17));
                    c.ldr(13, (c.g[T6][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x47));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sdl(12, (c.g[SP][0] as u32).wrapping_add(0x4f));
                    c.sdr(12, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x57));
                    c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1488));
                    c.ldl(15, (c.g[T4][0] as u32).wrapping_add(0x7));
                    c.ldr(15, c.g[T4][0] as u32);
                    c.ldl(5, (c.g[T4][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[T4][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x10));
                    c.sdl(15, (c.g[SP][0] as u32).wrapping_add(0x67));
                    c.sdr(15, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x6f));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x68));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T3][0]);
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x14a0));
                    c.ldl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(13, c.g[A1][0] as u32);
                    c.g[T6][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0x87));
                    c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T6][0]);
                    c.g[T4][0] = sx((c.g[T0][0] as u32).wrapping_add(0x26a8));
                    c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[T4][0] as u32);
                    c.sdl(9, (c.g[SP][0] as u32).wrapping_add(0x97));
                    c.sdr(9, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x26b0));
                    c.ldl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(13, c.g[A1][0] as u32);
                    c.sdl(13, (c.g[SP][0] as u32).wrapping_add(0xa7));
                    c.sdr(13, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x26b8));
                    c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[T2][0] as u32);
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xb7));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x26c0));
                    c.ldl(11, (c.g[T6][0] as u32).wrapping_add(0x7));
                    c.ldr(11, c.g[T6][0] as u32);
                    c.sdl(11, (c.g[SP][0] as u32).wrapping_add(0xc7));
                    c.sdr(11, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26c8));
                    c.ldl(15, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(15, c.g[A2][0] as u32);
                    c.sdl(15, (c.g[SP][0] as u32).wrapping_add(0xd7));
                    c.sdr(15, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x0020683c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x0020683c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00206848);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00206848)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        break 'b_20686c;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00206860);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2560));
                        call_ptr(c, 0x004165a8, 0x00206860)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = 0x470000;
                            break 'b_206a34;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            break 'b_20686c;
                        }
                    }
                }
                'b_2069ac: {
                    'b_206924: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                        if t {
                            break 'b_206924;
                        } else {
                            'b_2068a0: {
                                'b_206890: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x2;
                                        break 'b_206890;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                        if t {
                                            break 'b_2068a0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            if t {
                                                break 'b_207410;
                                            } else {
                                                break 'b_206890;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_2069ac;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                    if t {
                                        break 'b_207410;
                                    } else {
                                        break 'b_2068a0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S3][0] = 0x5;
                            'l_2068a8: loop {
                                'b_206910: {
                                    c.cycles += 8;
                                    c.g[RA][0] = sx(0x002068b0);
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x002068b0)?;
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                    c.g[RA][0] = sx(0x002068bc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x002068bc)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    if c.g[S0][0] == 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_206910;
                                    } else {
                                        'b_2068f8: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[RA][0] = sx(0x002068d4);
                                            c.g[A1][0] = c.g[S2][0];
                                            call_ptr(c, 0x004165a8, 0x002068d4)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if t {
                                                break 'b_2068f8;
                                            } else {
                                                c.cycles += 7;
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x002068f0);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x002068f0)?;
                                                let t = true;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_206910;
                                                } else {
                                                    break 'b_2068f8;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0020690c);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0020690c)?;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_206910;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[S3][0] as i64) >= 0;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002068a8);
                                    }
                                    continue 'l_2068a8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                    if t {
                                        break 'b_207410;
                                    } else {
                                        break 'b_206924;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = 0x5;
                    'l_206930: loop {
                        'b_206998: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x00206938);
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            call_ptr(c, 0x00317670, 0x00206938)?;
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00206944);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00206944)?;
                            c.g[S0][0] = c.g[V0][0];
                            if c.g[S0][0] == 0u64 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_206998;
                            } else {
                                'b_206980: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[RA][0] = sx(0x0020695c);
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x004165a8, 0x0020695c)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_206980;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00206978);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00206978)?;
                                        let t = true;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_206998;
                                        } else {
                                            break 'b_206980;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00206994);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00206994)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_206998;
                            }
                        }
                        c.cycles += 3;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[S3][0] as i64) >= 0;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00206930);
                            }
                            continue 'l_206930;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_207410;
                            } else {
                                break 'b_2069ac;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S3][0] = 0x4;
                'l_2069b8: loop {
                    'b_206a20: {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x002069c0);
                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                        call_ptr(c, 0x00317670, 0x002069c0)?;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                        c.g[RA][0] = sx(0x002069cc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002069cc)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_206a20;
                        } else {
                            'b_206a08: {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                c.g[RA][0] = sx(0x002069e4);
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x004165a8, 0x002069e4)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_206a08;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206a00);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206a00)?;
                                    let t = true;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                    if t {
                                        break 'b_206a20;
                                    } else {
                                        break 'b_206a08;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00206a1c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00206a1c)?;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_206a20;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002069b8);
                        }
                        continue 'l_2069b8;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                        if t {
                            break 'b_207410;
                        } else {
                            break 'b_206a34;
                        }
                    }
                }
            }
            'b_20740c: {
                'b_206af0: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206a40);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14b0));
                    call_ptr(c, 0x004165a8, 0x00206a40)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206af0;
                    } else {
                        c.cycles += 4;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206a58: loop {
                            'b_206ad8: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206a68);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206a68)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206a74);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206a74)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206ad8;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x2;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206a9c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206a9c)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                    'l_206aa0: loop {
                                        'b_206ad0: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206aac);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206aac)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206ad0;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206acc);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206acc)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206ad0;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206aa0);
                                            }
                                            continue 'l_206aa0;
                                        } else {
                                            break 'b_206ad8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206a58);
                                }
                                continue 'l_206a58;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206af0;
                                }
                            }
                        }
                    }
                }
                'b_206ba8: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206afc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14c0));
                    call_ptr(c, 0x004165a8, 0x00206afc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206ba8;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206b10: loop {
                            'b_206b90: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206b20);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206b20)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206b2c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206b2c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206b90;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206b54);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206b54)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                                    'l_206b58: loop {
                                        'b_206b88: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206b64);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206b64)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206b88;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206b84);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206b84)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206b88;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206b58);
                                            }
                                            continue 'l_206b58;
                                        } else {
                                            break 'b_206b90;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206b10);
                                }
                                continue 'l_206b10;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206ba8;
                                }
                            }
                        }
                    }
                }
                'b_206c60: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206bb4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14d0));
                    call_ptr(c, 0x004165a8, 0x00206bb4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206c60;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206bc8: loop {
                            'b_206c48: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206bd8);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206bd8)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206be4);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206be4)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206c48;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206c0c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206c0c)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                                    'l_206c10: loop {
                                        'b_206c40: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206c1c);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206c1c)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206c40;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206c3c);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206c3c)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206c40;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206c10);
                                            }
                                            continue 'l_206c10;
                                        } else {
                                            break 'b_206c48;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206bc8);
                                }
                                continue 'l_206bc8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206c60;
                                }
                            }
                        }
                    }
                }
                'b_206d18: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206c6c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14e0));
                    call_ptr(c, 0x004165a8, 0x00206c6c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206d18;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206c80: loop {
                            'b_206d00: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206c90);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206c90)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206c9c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206c9c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206d00;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206cc4);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206cc4)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                                    'l_206cc8: loop {
                                        'b_206cf8: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206cd4);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206cd4)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206cf8;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206cf4);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206cf4)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206cf8;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206cc8);
                                            }
                                            continue 'l_206cc8;
                                        } else {
                                            break 'b_206d00;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206c80);
                                }
                                continue 'l_206c80;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206d18;
                                }
                            }
                        }
                    }
                }
                'b_206dd0: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206d24);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14f0));
                    call_ptr(c, 0x004165a8, 0x00206d24)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206dd0;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206d38: loop {
                            'b_206db8: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206d48);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206d48)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206d54);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206d54)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206db8;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206d7c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206d7c)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                    'l_206d80: loop {
                                        'b_206db0: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206d8c);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206d8c)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206db0;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206dac);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206dac)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206db0;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206d80);
                                            }
                                            continue 'l_206d80;
                                        } else {
                                            break 'b_206db8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206d38);
                                }
                                continue 'l_206d38;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206dd0;
                                }
                            }
                        }
                    }
                }
                'b_206e88: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206ddc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1500));
                    call_ptr(c, 0x004165a8, 0x00206ddc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206e88;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        'l_206df0: loop {
                            'b_206e70: {
                                c.cycles += 10;
                                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[S1][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x00206e00);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206e00)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206e0c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206e0c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206e70;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[S3][0] = c.g[S1][0];
                                    c.g[S2][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206e34);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206e34)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                                    'l_206e38: loop {
                                        'b_206e68: {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                            c.g[RA][0] = sx(0x00206e44);
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            call_ptr(c, 0x004165a8, 0x00206e44)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206e68;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00206e64);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00206e64)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_206e68;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00206e38);
                                            }
                                            continue 'l_206e38;
                                        } else {
                                            break 'b_206e70;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S5][0];
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206df0);
                                }
                                continue 'l_206df0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_206e88;
                                }
                            }
                        }
                    }
                }
                'b_206f44: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206e94);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1510));
                    call_ptr(c, 0x004165a8, 0x00206e94)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206f44;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S2][0] = 0x5;
                        'l_206ea8: loop {
                            'b_206edc: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00206eb0);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206eb0)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206ebc);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206ebc)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206edc;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206edc);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206edc)?;
                                    break 'b_206edc;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206ea8);
                                }
                                continue 'l_206ea8;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ddb));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x144));
                                'l_206ef8: loop {
                                    'b_206f2c: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x00206f00);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x00206f00)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x00206f0c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x00206f0c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_206f2c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00206f2c);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00206f2c)?;
                                            break 'b_206f2c;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00206ef8);
                                        }
                                        continue 'l_206ef8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_206f44;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_206ffc: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00206f50);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1520));
                    call_ptr(c, 0x004165a8, 0x00206f50)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_206ffc;
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S2][0] = 0x5;
                        'l_206f60: loop {
                            'b_206f94: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00206f68);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00206f68)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00206f74);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00206f74)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_206f94;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00206f94);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00206f94)?;
                                    break 'b_206f94;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00206f60);
                                }
                                continue 'l_206f60;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x78cb));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b0));
                                'l_206fb0: loop {
                                    'b_206fe4: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x00206fb8);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x00206fb8)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x00206fc4);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x00206fc4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_206fe4;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00206fe4);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00206fe4)?;
                                            break 'b_206fe4;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00206fb0);
                                        }
                                        continue 'l_206fb0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_206ffc;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_207060: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00207008);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1530));
                    call_ptr(c, 0x004165a8, 0x00207008)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_207060;
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S2][0] = 0x5;
                        'l_207018: loop {
                            'b_20704c: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00207020);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00207020)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x0020702c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0020702c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = c.g[SP][0];
                                if t {
                                    break 'b_20704c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0020704c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0020704c)?;
                                    break 'b_20704c;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00207018);
                                }
                                continue 'l_207018;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_207060;
                                }
                            }
                        }
                    }
                }
                'b_20711c: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0020706c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1548));
                    call_ptr(c, 0x004165a8, 0x0020706c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_20711c;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S2][0] = 0x5;
                        'l_207080: loop {
                            'b_2070b4: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00207088);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00207088)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00207094);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00207094)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_2070b4;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x002070b4);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x002070b4)?;
                                    break 'b_2070b4;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00207080);
                                }
                                continue 'l_207080;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7c2b));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x144));
                                'l_2070d0: loop {
                                    'b_207104: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x002070d8);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x002070d8)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x002070e4);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x002070e4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_207104;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00207104);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00207104)?;
                                            break 'b_207104;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002070d0);
                                        }
                                        continue 'l_2070d0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_20711c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_2071d4: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00207128);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1558));
                    call_ptr(c, 0x004165a8, 0x00207128)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_2071d4;
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S2][0] = 0x5;
                        'l_207138: loop {
                            'b_20716c: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00207140);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00207140)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x0020714c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0020714c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_20716c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0020716c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0020716c)?;
                                    break 'b_20716c;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00207138);
                                }
                                continue 'l_207138;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76af));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b0));
                                'l_207188: loop {
                                    'b_2071bc: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x00207190);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x00207190)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x0020719c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0020719c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_2071bc;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002071bc);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x002071bc)?;
                                            break 'b_2071bc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00207188);
                                        }
                                        continue 'l_207188;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_2071d4;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_207238: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002071e0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1568));
                    call_ptr(c, 0x004165a8, 0x002071e0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_207238;
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S2][0] = 0x5;
                        'l_2071f0: loop {
                            'b_207224: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x002071f8);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x002071f8)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00207204);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00207204)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = c.g[SP][0];
                                if t {
                                    break 'b_207224;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00207224);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00207224)?;
                                    break 'b_207224;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002071f0);
                                }
                                continue 'l_2071f0;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                if t {
                                    break 'b_20740c;
                                } else {
                                    break 'b_207238;
                                }
                            }
                        }
                    }
                }
                'b_2072f4: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00207244);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1580));
                    call_ptr(c, 0x004165a8, 0x00207244)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_2072f4;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S2][0] = 0x4;
                        'l_207258: loop {
                            'b_20728c: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00207260);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00207260)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x0020726c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0020726c)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_20728c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0020728c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0020728c)?;
                                    break 'b_20728c;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00207258);
                                }
                                continue 'l_207258;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7a7b));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd8));
                                'l_2072a8: loop {
                                    'b_2072dc: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x002072b0);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x002072b0)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x002072bc);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x002072bc)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_2072dc;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002072dc);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x002072dc)?;
                                            break 'b_2072dc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002072a8);
                                        }
                                        continue 'l_2072a8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_2072f4;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'b_2073ac: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00207300);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1590));
                    call_ptr(c, 0x004165a8, 0x00207300)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        break 'b_2073ac;
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S2][0] = 0x4;
                        'l_207310: loop {
                            'b_207344: {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x00207318);
                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                call_ptr(c, 0x00317670, 0x00207318)?;
                                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                c.g[RA][0] = sx(0x00207324);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00207324)?;
                                c.g[S0][0] = c.g[V0][0];
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_207344;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00207344);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00207344)?;
                                    break 'b_207344;
                                }
                            }
                            c.cycles += 3;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00207310);
                                }
                                continue 'l_207310;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x480000;
                                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7493));
                                c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1b0));
                                'l_207360: loop {
                                    'b_207394: {
                                        c.cycles += 8;
                                        c.g[RA][0] = sx(0x00207368);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x00207368)?;
                                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                                        c.g[RA][0] = sx(0x00207374);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x00207374)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A1][0] = c.g[SP][0];
                                        if t {
                                            break 'b_207394;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00207394);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00207394)?;
                                            break 'b_207394;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00207360);
                                        }
                                        continue 'l_207360;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                                        if t {
                                            break 'b_207410;
                                        } else {
                                            break 'b_2073ac;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002073b8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15a0));
                call_ptr(c, 0x004165a8, 0x002073b8)?;
                let t = c.g[V0][0] != 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                if t {
                    break 'b_20740c;
                } else {
                    c.cycles += 2;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = 0x4;
                    'l_2073c8: loop {
                        'b_2073fc: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x002073d0);
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            call_ptr(c, 0x00317670, 0x002073d0)?;
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x002073dc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x002073dc)?;
                            c.g[S0][0] = c.g[V0][0];
                            let t = c.g[S0][0] == 0u64;
                            c.g[A1][0] = c.g[SP][0];
                            if t {
                                break 'b_2073fc;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002073fc);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002073fc)?;
                                break 'b_2073fc;
                            }
                        }
                        c.cycles += 3;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[S2][0] as i64) >= 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002073c8);
                            }
                            continue 'l_2073c8;
                        } else {
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                            break 'b_20740c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
            break 'b_207410;
        }
        c.cycles += 8;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        return Ok(());
    });
}

/// `EventSelect_RefreshEventList` (0x00207430): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[cfg(test)] // not installed: its test runs away in the unslotted file-load chain
#[allow(dead_code)]
pub fn event_select_refresh_event_list(c: &mut Cpu) {
    if c.pc != 0x00207430 {
        return ee::s05::f_00207430__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00207430;
        loop {
            match pc {
                0x00207430 => {
                    c.cycles += 23;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xbc0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb70));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xb50));
                    c.g[A0][0] = 0x470000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xba0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb80));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb60));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb40));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xb30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xb20), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0020746c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x578));
                    call_ptr(c, 0x00317670, 0x0020746c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00207478);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00207478)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x002074a0;
                        continue;
                    }
                    pc = 0x0020748c;
                    continue;
                }
                0x0020748c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002074a0);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002074a0)?;
                    pc = 0x002074a0;
                    continue;
                }
                0x002074a0 => {
                    c.cycles += 27;
                    c.g[RA][0] = sx(0x002074a8);
                    call_ptr(c, 0x0014dc80, 0x002074a8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002074b4);
                    c.g[A1][0] = 0xa;
                    call_ptr(c, 0x0014dd58, 0x002074b4)?;
                    c.g[RA][0] = sx(0x002074bc);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb00), c.g[V0][0]);
                    call_ptr(c, 0x0014dc80, 0x002074bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002074c8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x002074c8)?;
                    c.g[RA][0] = sx(0x002074d0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb04), c.g[V0][0]);
                    call_ptr(c, 0x0014dc80, 0x002074d0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002074dc);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x002074dc)?;
                    c.g[RA][0] = sx(0x002074e4);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb08), c.g[V0][0]);
                    call_ptr(c, 0x0014dc80, 0x002074e4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002074f0);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, 0x0014dd58, 0x002074f0)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb0c), c.g[V0][0]);
                    c.g[RA][0] = sx(0x002074fc);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x0014a080, 0x002074fc)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xb10), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2cc));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    if t {
                        pc = 0x00207530;
                        continue;
                    }
                    pc = 0x0020750c;
                    continue;
                }
                0x0020750c => {
                    c.cycles += 9;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x00207518);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15b8));
                    call_ptr(c, 0x0039e8b8, 0x00207518)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207528);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2600));
                    call_ptr(c, 0x0039e8b8, 0x00207528)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        pc = 0x00207550;
                        continue;
                    }
                    pc = 0x00207530;
                    continue;
                }
                0x00207530 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x0020753c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15b8));
                    call_ptr(c, 0x0039e948, 0x0020753c)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x0020754c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2600));
                    call_ptr(c, 0x0039e948, 0x0020754c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    pc = 0x00207550;
                    continue;
                }
                0x00207550 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S0][0] = 0x4a0000;
                    if t {
                        pc = 0x00207598;
                        continue;
                    }
                    pc = 0x0020755c;
                    continue;
                }
                0x0020755c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207568);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x25d8));
                    call_ptr(c, 0x0039e948, 0x00207568)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x470000;
                    if t {
                        pc = 0x0020758c;
                        continue;
                    }
                    pc = 0x00207574;
                    continue;
                }
                0x00207574 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207584);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x558));
                    call_ptr(c, 0x0039e948, 0x00207584)?;
                    let t = true;

                    if t {
                        pc = 0x00207598;
                        continue;
                    }
                    pc = 0x0020758c;
                    continue;
                }
                0x0020758c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207598);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x548));
                    call_ptr(c, 0x0039e948, 0x00207598)?;
                    pc = 0x00207598;
                    continue;
                }
                0x00207598 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x002075a0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x25d8));
                    call_ptr(c, 0x00317670, 0x002075a0)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002075ac);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002075ac)?;
                    c.g[FP][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[V0][0] = 0x480000;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7e30));
                    c.g[V0][0] = c.g[A1][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        pc = 0x00207624;
                        continue;
                    }
                    pc = 0x002075cc;
                    continue;
                }
                0x002075cc => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                    pc = 0x002075d0;
                    continue;
                }
                0x002075d0 => {
                    c.cycles += 19;
                    c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[A1][0] as u32);
                    c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, c.g[A0][0] as u32);
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x002075d0) {
                            return Ok(());
                        }
                        pc = 0x002075d0;
                        continue;
                    }
                    pc = 0x0020761c;
                    continue;
                }
                0x0020761c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    if t {
                        pc = 0x00207658;
                        continue;
                    }
                    pc = 0x00207624;
                    continue;
                }
                0x00207624 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
                    pc = 0x00207628;
                    continue;
                }
                0x00207628 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207628) {
                            return Ok(());
                        }
                        pc = 0x00207628;
                        continue;
                    }
                    pc = 0x00207654;
                    continue;
                }
                0x00207654 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    pc = 0x00207658;
                    continue;
                }
                0x00207658 => {
                    c.cycles += 9;
                    c.g[S2][0] = 0u64;
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[A1][0] as u32);
                    c.g[T1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A0][0] as u32);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    if t {
                        pc = 0x002078bc;
                        continue;
                    }
                    pc = 0x0020767c;
                    continue;
                }
                0x0020767c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x00207698;
                        continue;
                    }
                    pc = 0x00207688;
                    continue;
                }
                0x00207688 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[T1][0] = 0x4a0000;
                    if t {
                        pc = 0x002076a8;
                        continue;
                    }
                    pc = 0x00207690;
                    continue;
                }
                0x00207690 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00207c2c;
                        continue;
                    }
                    pc = 0x00207698;
                    continue;
                }
                0x00207698 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[T1][0] = 0x4a0000;
                    if t {
                        pc = 0x00207abc;
                        continue;
                    }
                    pc = 0x002076a0;
                    continue;
                }
                0x002076a0 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00207c2c;
                        continue;
                    }
                    pc = 0x002076a8;
                    continue;
                }
                0x002076a8 => {
                    c.cycles += 24;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x15d0));
                    c.g[RA][0] = sx(0x002076c4);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x002076c4)?;
                    c.g[RA][0] = sx(0x002076cc);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x002076cc)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002076d8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002076d8)?;
                    c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    c.g[A1][0] = 0x1b0;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[V0][0] = 0x6c;
                    c.mult(0, 2, 18, 2);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x7e30));
                    c.mult(0, 3, 3, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.g[A0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x00207764;
                        continue;
                    }
                    pc = 0x00207708;
                    continue;
                }
                0x00207708 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    pc = 0x00207710;
                    continue;
                }
                0x00207710 => {
                    c.cycles += 19;
                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[A0][0] as u32);
                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.ldr(3, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A1][0] as u32);
                    c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.sdr(3, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.sdr(6, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207710) {
                            return Ok(());
                        }
                        pc = 0x00207710;
                        continue;
                    }
                    pc = 0x0020775c;
                    continue;
                }
                0x0020775c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        pc = 0x0020779c;
                        continue;
                    }
                    pc = 0x00207764;
                    continue;
                }
                0x00207764 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    pc = 0x0020776c;
                    continue;
                }
                0x0020776c => {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld(c.g[A0][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0020776c) {
                            return Ok(());
                        }
                        pc = 0x0020776c;
                        continue;
                    }
                    pc = 0x00207798;
                    continue;
                }
                0x00207798 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    pc = 0x0020779c;
                    continue;
                }
                0x0020779c => {
                    c.cycles += 15;
                    c.g[S1][0] = 0x4;
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[A0][0] as u32);
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[A1][0] as u32);
                    c.g[V1][0] = 0x3;
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S0][0] = 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.g[V1][0];
                    }
                    c.g[S5][0] = 0x60;
                    c.g[S3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x140));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    pc = 0x002077d8;
                    continue;
                }
                0x002077d8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    if t {
                        pc = 0x00207868;
                        continue;
                    }
                    pc = 0x002077e0;
                    continue;
                }
                0x002077e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x470000;
                    if t {
                        pc = 0x0020786c;
                        continue;
                    }
                    pc = 0x002077ec;
                    continue;
                }
                0x002077ec => {
                    c.cycles += 23;
                    c.g[V0][0] = 0x470000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207800);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15e8));
                    call_ptr(c, 0x00417828, 0x00207800)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x0020780c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e948, 0x0020780c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                    c.g[RA][0] = sx(0x0020781c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x0020781c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    c.mult(0, 4, 4, 21);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    if t {
                        pc = 0x00207858;
                        continue;
                    }
                    pc = 0x00207848;
                    continue;
                }
                0x00207848 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00207850);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e948, 0x00207850)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x002078a8;
                        continue;
                    }
                    pc = 0x00207858;
                    continue;
                }
                0x00207858 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00207860);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x00207860)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x002078a8;
                        continue;
                    }
                    pc = 0x00207868;
                    continue;
                }
                0x00207868 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x470000;
                    pc = 0x0020786c;
                    continue;
                }
                0x0020786c => {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15e8));
                    c.g[RA][0] = sx(0x0020787c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x0020787c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207888);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x00207888)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                    c.g[RA][0] = sx(0x00207898);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207898)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x002078a4);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x002078a4)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x002078a8;
                    continue;
                }
                0x002078a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x002077d8) {
                            return Ok(());
                        }
                        pc = 0x002077d8;
                        continue;
                    }
                    pc = 0x002078b4;
                    continue;
                }
                0x002078b4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[T1][0] = 0x4a0000;
                    if t {
                        pc = 0x00207c2c;
                        continue;
                    }
                    pc = 0x002078bc;
                    continue;
                }
                0x002078bc => {
                    c.cycles += 25;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1608));
                    c.g[RA][0] = sx(0x002078d8);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x002078d8)?;
                    c.g[RA][0] = sx(0x002078e0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x002078e0)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002078ec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002078ec)?;
                    c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    c.g[A1][0] = 0x21c;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[V0][0] = 0x6c;
                    c.mult(0, 2, 18, 2);
                    c.g[V1][0] = 0x480000;
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[A0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0020797c;
                        continue;
                    }
                    pc = 0x00207920;
                    continue;
                }
                0x00207920 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    pc = 0x00207928;
                    continue;
                }
                0x00207928 => {
                    c.cycles += 17;
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[A0][0] as u32);
                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.ldr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sdl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[A1][0] as u32);
                    c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    pc = 0x0020796c;
                    continue;
                }
                0x0020796c => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207928) {
                            return Ok(());
                        }
                        pc = 0x00207928;
                        continue;
                    }
                    pc = 0x00207974;
                    continue;
                }
                0x00207974 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x002079b0;
                        continue;
                    }
                    pc = 0x0020797c;
                    continue;
                }
                0x0020797c => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    pc = 0x00207984;
                    continue;
                }
                0x00207984 => {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld(c.g[A0][0] as u32);
                    c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207984) {
                            return Ok(());
                        }
                        pc = 0x00207984;
                        continue;
                    }
                    pc = 0x002079b0;
                    continue;
                }
                0x002079b0 => {
                    c.cycles += 10;
                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A1][0] as u32);
                    c.g[S0][0] = 0u64;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[S3][0] = 0x60;
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x140));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                    pc = 0x002079d8;
                    continue;
                }
                0x002079d8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    if t {
                        pc = 0x00207a68;
                        continue;
                    }
                    pc = 0x002079e0;
                    continue;
                }
                0x002079e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[T0][0] = 0x470000;
                    if t {
                        pc = 0x00207a6c;
                        continue;
                    }
                    pc = 0x002079ec;
                    continue;
                }
                0x002079ec => {
                    c.cycles += 23;
                    c.g[A3][0] = 0x470000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x15e8));
                    c.g[RA][0] = sx(0x00207a00);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207a00)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207a0c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e948, 0x00207a0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                    c.g[RA][0] = sx(0x00207a1c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207a1c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    c.mult(0, 4, 4, 19);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    if t {
                        pc = 0x00207a58;
                        continue;
                    }
                    pc = 0x00207a48;
                    continue;
                }
                0x00207a48 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00207a50);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e948, 0x00207a50)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00207aa8;
                        continue;
                    }
                    pc = 0x00207a58;
                    continue;
                }
                0x00207a58 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00207a60);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x00207a60)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00207aa8;
                        continue;
                    }
                    pc = 0x00207a68;
                    continue;
                }
                0x00207a68 => {
                    c.cycles += 1;
                    c.g[T0][0] = 0x470000;
                    pc = 0x00207a6c;
                    continue;
                }
                0x00207a6c => {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x15e8));
                    c.g[RA][0] = sx(0x00207a7c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207a7c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207a88);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x00207a88)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                    c.g[RA][0] = sx(0x00207a98);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207a98)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207aa4);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0039e8b8, 0x00207aa4)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x00207aa8;
                    continue;
                }
                0x00207aa8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x002079d8) {
                            return Ok(());
                        }
                        pc = 0x002079d8;
                        continue;
                    }
                    pc = 0x00207ab4;
                    continue;
                }
                0x00207ab4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[T1][0] = 0x4a0000;
                    if t {
                        pc = 0x00207c2c;
                        continue;
                    }
                    pc = 0x00207abc;
                    continue;
                }
                0x00207abc => {
                    c.cycles += 25;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00207ad8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1628));
                    call_ptr(c, 0x00417828, 0x00207ad8)?;
                    c.g[RA][0] = sx(0x00207ae0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00207ae0)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00207aec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00207aec)?;
                    c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    c.g[A1][0] = 0x360;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[V0][0] = 0x6c;
                    c.mult(0, 2, 18, 2);
                    c.g[V1][0] = 0x480000;
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72c8));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[A0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    if t {
                        pc = 0x00207b80;
                        continue;
                    }
                    pc = 0x00207b20;
                    continue;
                }
                0x00207b20 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    pc = 0x00207b2c;
                    continue;
                }
                0x00207b2c => {
                    c.cycles += 19;
                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A0][0] as u32);
                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(3, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A2][0] as u32);
                    c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.sdr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[A2][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207b2c) {
                            return Ok(());
                        }
                        pc = 0x00207b2c;
                        continue;
                    }
                    pc = 0x00207b78;
                    continue;
                }
                0x00207b78 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00207bb8;
                        continue;
                    }
                    pc = 0x00207b80;
                    continue;
                }
                0x00207b80 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    pc = 0x00207b8c;
                    continue;
                }
                0x00207b8c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A0][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00207b8c) {
                            return Ok(());
                        }
                        pc = 0x00207b8c;
                        continue;
                    }
                    pc = 0x00207bb8;
                    continue;
                }
                0x00207bb8 => {
                    c.cycles += 10;
                    c.ldl(9, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[A2][0] as u32);
                    c.g[S0][0] = c.g[A1][0];
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[S1][0] = 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    pc = 0x00207be0;
                    continue;
                }
                0x00207be0 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207bec);
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                    call_ptr(c, 0x00417828, 0x00207bec)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207bf8);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e8b8, 0x00207bf8)?;
                    c.g[T0][0] = 0x470000;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x15e8));
                    c.g[RA][0] = sx(0x00207c0c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x00207c0c)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00207c1c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e8b8, 0x00207c1c)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x00207be0) {
                            return Ok(());
                        }
                        pc = 0x00207be0;
                        continue;
                    }
                    pc = 0x00207c28;
                    continue;
                }
                0x00207c28 => {
                    c.cycles += 1;
                    c.g[T1][0] = 0x4a0000;
                    pc = 0x00207c2c;
                    continue;
                }
                0x00207c2c => {
                    c.cycles += 24;
                    c.g[RA][0] = sx(0x00207c34);
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x26d0));
                    call_ptr(c, 0x00317670, 0x00207c34)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00207c40);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00207c40)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00207c50);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb04));
                    call_ptr(c, 0x00144c60, 0x00207c50)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00207c5c);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003a0c50, 0x00207c5c)?;
                    c.g[V0][0] = 0x470000;
                    c.g[RA][0] = sx(0x00207c68);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1640));
                    call_ptr(c, 0x00317670, 0x00207c68)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00207c74);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00207c74)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11));
                    c.g[V1][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S7][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    if t {
                        pc = 0x00207cbc;
                        continue;
                    }
                    pc = 0x00207c8c;
                    continue;
                }
                0x00207c8c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S7][0] = 0xc;
                    if t {
                        pc = 0x00207f58;
                        continue;
                    }
                    pc = 0x00207c9c;
                    continue;
                }
                0x00207c9c => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x00207ca8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1650));
                    call_ptr(c, 0x00317670, 0x00207ca8)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00207cb4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00207cb4)?;
                    let t = true;
                    c.g[V0][0] = (c.g[S7][0] < 0xdu64) as u64;
                    if t {
                        pc = 0x00207f5c;
                        continue;
                    }
                    pc = 0x00207cbc;
                    continue;
                }
                0x00207cbc => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x102));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2560));
                    c.g[RA][0] = sx(0x00207cd0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004165a8, 0x00207cd0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        pc = 0x00207cec;
                        continue;
                    }
                    pc = 0x00207cd8;
                    continue;
                }
                0x00207cd8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xc;
                    if t {
                        pc = 0x00207f40;
                        continue;
                    }
                    pc = 0x00207ce4;
                    continue;
                }
                0x00207ce4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00207e90;
                        continue;
                    }
                    pc = 0x00207cec;
                    continue;
                }
                0x00207cec => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x00207d38;
                        continue;
                    }
                    pc = 0x00207cfc;
                    continue;
                }
                0x00207cfc => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[A1][0] = 0x60;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = 0x470000;
                        pc = 0x00207e8c;
                        continue;
                    }
                    pc = 0x00207d2c;
                    continue;
                }
                0x00207d2c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1680));
                    if t {
                        pc = 0x00207e90;
                        continue;
                    }
                    pc = 0x00207d38;
                    continue;
                }
                0x00207d38 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6));
                    if t {
                        pc = 0x00207d7c;
                        continue;
                    }
                    pc = 0x00207d40;
                    continue;
                }
                0x00207d40 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    c.g[A1][0] = 0x60;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = 0x470000;
                        pc = 0x00207e8c;
                        continue;
                    }
                    pc = 0x00207d70;
                    continue;
                }
                0x00207d70 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1698));
                    if t {
                        pc = 0x00207e90;
                        continue;
                    }
                    pc = 0x00207d7c;
                    continue;
                }
                0x00207d7c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        pc = 0x00207e50;
                        continue;
                    }
                    pc = 0x00207d88;
                    continue;
                }
                0x00207d88 => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x60;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x00207e8c;
                        continue;
                    }
                    pc = 0x00207db4;
                    continue;
                }
                0x00207db4 => {
                    c.cycles += 39;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207dc8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2590));
                    call_ptr(c, 0x002c2540, 0x00207dc8)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[RA][0] = sx(0x00207dd8);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb04));
                    call_ptr(c, 0x00144c60, 0x00207dd8)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207de8);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x002c26d0, 0x00207de8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16b0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00207e0c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00207e0c)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00207e1c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x00207e1c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00207e28);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2508, 0x00207e28)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207e3c);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002c26d0, 0x00207e3c)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00207e48);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0d00, 0x00207e48)?;
                    let t = true;
                    c.g[V0][0] = 0xc;
                    if t {
                        pc = 0x00207f40;
                        continue;
                    }
                    pc = 0x00207e50;
                    continue;
                }
                0x00207e50 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x9));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    if t {
                        pc = 0x00207f3c;
                        continue;
                    }
                    pc = 0x00207e60;
                    continue;
                }
                0x00207e60 => {
                    c.cycles += 10;
                    c.g[A1][0] = 0x60;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.mult(0, 4, 4, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c0));
                    if t {
                        pc = 0x00207eac;
                        continue;
                    }
                    pc = 0x00207e88;
                    continue;
                }
                0x00207e88 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x470000;
                    pc = 0x00207e8c;
                    continue;
                }
                0x00207e8c => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1668));
                    pc = 0x00207e90;
                    continue;
                }
                0x00207e90 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00207e98);
                    call_ptr(c, 0x00317670, 0x00207e98)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00207ea4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00207ea4)?;
                    let t = true;
                    c.g[V0][0] = 0xc;
                    if t {
                        pc = 0x00207f40;
                        continue;
                    }
                    pc = 0x00207eac;
                    continue;
                }
                0x00207eac => {
                    c.cycles += 36;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207ebc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x26a0));
                    call_ptr(c, 0x002c2540, 0x00207ebc)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[RA][0] = sx(0x00207ecc);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb04));
                    call_ptr(c, 0x00144c60, 0x00207ecc)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207edc);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x002c26d0, 0x00207edc)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16b0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00207f00);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00207f00)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00207f10);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x00207f10)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00207f1c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2508, 0x00207f1c)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00207f30);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002c26d0, 0x00207f30)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00207f3c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0d00, 0x00207f3c)?;
                    pc = 0x00207f3c;
                    continue;
                }
                0x00207f3c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xc;
                    pc = 0x00207f40;
                    continue;
                }
                0x00207f40 => {
                    c.cycles += 2;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[V0][0] = (c.g[S7][0] < 0xdu64) as u64;
                    if t {
                        pc = 0x00207f5c;
                        continue;
                    }
                    pc = 0x00207f48;
                    continue;
                }
                0x00207f48 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x00207f54);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb04));
                    call_ptr(c, 0x00145870, 0x00207f54)?;
                    c.g[S7][0] = c.g[V0][0];
                    pc = 0x00207f58;
                    continue;
                }
                0x00207f58 => {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[S7][0] < 0xdu64) as u64;
                    pc = 0x00207f5c;
                    continue;
                }
                0x00207f5c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        pc = 0x00208054;
                        continue;
                    }
                    pc = 0x00207f64;
                    continue;
                }
                0x00207f64 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S7][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x17c0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00207f7c => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x470000;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1ab8));
                    if t {
                        pc = 0x00207fa8;
                        continue;
                    }
                    pc = 0x00207f88;
                    continue;
                }
                0x00207f88 => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x470000;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1c68));
                    if t {
                        pc = 0x00207fa8;
                        continue;
                    }
                    pc = 0x00207f94;
                    continue;
                }
                0x00207f94 => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x470000;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        pc = 0x00207fa8;
                        continue;
                    }
                    pc = 0x00207fa0;
                    continue;
                }
                0x00207fa0 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0x470000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x16c8));
                    pc = 0x00207fa8;
                    continue;
                }
                0x00207fa8 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x00207fb0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00207fb0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00207fbc);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003a1000, 0x00207fbc)?;
                    c.g[RA][0] = sx(0x00207fc4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00207fc4)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x00207fd0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00207fd0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        pc = 0x00208058;
                        continue;
                    }
                    pc = 0x00207fd8;
                    continue;
                }
                0x00207fd8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a10));
                    if t {
                        pc = 0x00207ff8;
                        continue;
                    }
                    pc = 0x00207fe4;
                    continue;
                }
                0x00207fe4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16d8));
                    if t {
                        pc = 0x00207ff8;
                        continue;
                    }
                    pc = 0x00207ff0;
                    continue;
                }
                0x00207ff0 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16e8));
                    pc = 0x00207ff8;
                    continue;
                }
                0x00207ff8 => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x00208000);
                    call_ptr(c, 0x00317670, 0x00208000)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020800c);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003a1000, 0x0020800c)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0020801c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x26d8));
                    call_ptr(c, 0x003a0c50, 0x0020801c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        pc = 0x00208058;
                        continue;
                    }
                    pc = 0x00208024;
                    continue;
                }
                0x00208024 => {
                    c.cycles += 12;
                    c.g[S0][0] = 0x470000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x16f8));
                    c.g[RA][0] = sx(0x00208034);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00208034)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208040);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003a1000, 0x00208040)?;
                    c.g[RA][0] = sx(0x00208048);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x00208048)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x00208054);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00208054)?;
                    pc = 0x00208054;
                    continue;
                }
                0x00208054 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    pc = 0x00208058;
                    continue;
                }
                0x00208058 => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x2;
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    if t {
                        pc = 0x00208074;
                        continue;
                    }
                    pc = 0x00208064;
                    continue;
                }
                0x00208064 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00208070);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x125));
                    Callee {
                        addr: 0x00206690,
                        f: ee::s05::EventSelect_ShowEventPaths,
                    }
                    .call(c, 0x00208070)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));
                    pc = 0x00208074;
                    continue;
                }
                0x00208074 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[T1][0] = 0x470000;
                    if t {
                        pc = 0x002081cc;
                        continue;
                    }
                    pc = 0x0020807c;
                    continue;
                }
                0x0020807c => {
                    c.cycles += 78;
                    c.g[V1][0] = 0x470000;
                    c.g[RA][0] = sx(0x00208088);
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1710));
                    call_ptr(c, 0x00317670, 0x00208088)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00208094);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00208094)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002080a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1720));
                    call_ptr(c, 0x00317670, 0x002080a4)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002080b0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x002080b0)?;
                    c.g[A2][0] = 0x470000;
                    c.g[RA][0] = sx(0x002080bc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1738));
                    call_ptr(c, 0x00317670, 0x002080bc)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002080c8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002080c8)?;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb00));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2398));
                    c.g[RA][0] = sx(0x002080e4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00153520, 0x002080e4)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002080f4);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb00));
                    call_ptr(c, 0x00153350, 0x002080f4)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208108);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x00208108)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00208114);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x003a0c50, 0x00208114)?;
                    c.g[A3][0] = 0x470000;
                    c.g[RA][0] = sx(0x00208120);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1748));
                    call_ptr(c, 0x00317670, 0x00208120)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x0020812c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0020812c)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020813c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1758));
                    call_ptr(c, 0x00317670, 0x0020813c)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00208148);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00208148)?;
                    c.g[T0][0] = 0x470000;
                    c.g[RA][0] = sx(0x00208154);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1770));
                    call_ptr(c, 0x00317670, 0x00208154)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00208160);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00208160)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208170);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb00));
                    call_ptr(c, 0x001542e0, 0x00208170)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208180);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb00));
                    call_ptr(c, 0x001542a0, 0x00208180)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00208194);
                    c.g[A3][0] = c.g[V0][0];
                    call_ptr(c, 0x00417828, 0x00208194)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002081a0);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0c50, 0x002081a0)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x002081ac);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb00));
                    call_ptr(c, 0x001542a0, 0x002081ac)?;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    if t {
                        pc = 0x002083a8;
                        continue;
                    }
                    pc = 0x002081b4;
                    continue;
                }
                0x002081b4 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002081c4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x26e0));
                    call_ptr(c, 0x003a0c50, 0x002081c4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    if t {
                        pc = 0x002083a8;
                        continue;
                    }
                    pc = 0x002081cc;
                    continue;
                }
                0x002081cc => {
                    c.cycles += 37;
                    c.g[V0][0] = 0x470000;
                    c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1710));
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1738));
                    c.g[RA][0] = sx(0x002081e4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e948, 0x002081e4)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x002081f0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0039e948, 0x002081f0)?;
                    c.g[RA][0] = sx(0x002081f8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317670, 0x002081f8)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00208204);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00208204)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208214);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1780));
                    call_ptr(c, 0x00317670, 0x00208214)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00208220);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00208220)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb08));
                    c.g[A1][0] = 0u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb10));
                    c.g[RA][0] = sx(0x00208238);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    call_ptr(c, 0x00158910, 0x00208238)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00208244);
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x00317670, 0x00208244)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x00208250);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00208250)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x0020827c;
                        continue;
                    }
                    pc = 0x00208260;
                    continue;
                }
                0x00208260 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00208268);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x00204ff0,
                        f: ee::s05::EventSelect_GetMedalText,
                    }
                    .call(c, 0x00208268)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00208274);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0e90, 0x00208274)?;
                    let t = true;
                    c.g[V1][0] = 0x470000;
                    if t {
                        pc = 0x00208298;
                        continue;
                    }
                    pc = 0x0020827c;
                    continue;
                }
                0x0020827c => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x00208288);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1790));
                    call_ptr(c, 0x00317670, 0x00208288)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00208294);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00208294)?;
                    c.g[V1][0] = 0x470000;
                    pc = 0x00208298;
                    continue;
                }
                0x00208298 => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x002082a0);
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1748));
                    call_ptr(c, 0x00317670, 0x002082a0)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002082ac);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002082ac)?;
                    c.g[A2][0] = 0x470000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002082bc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1770));
                    call_ptr(c, 0x00317670, 0x002082bc)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x2e8));
                    c.g[RA][0] = sx(0x002082c8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002082c8)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb50));

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0x470000;
                        pc = 0x0020833c;
                        continue;
                    }
                    pc = 0x002082d8;
                    continue;
                }
                0x002082d8 => {
                    c.cycles += 19;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002082e4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17a0));
                    call_ptr(c, 0x00317670, 0x002082e4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x002082f0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x002082f0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0c));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[RA][0] = sx(0x00208304);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    call_ptr(c, 0x001550e8, 0x00208304)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208310);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0020a1a8,
                        f: ee::s05::FE_FormatMinSec,
                    }
                    .call(c, 0x00208310)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020831c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003a0c50, 0x0020831c)?;
                    let t = c.g[S0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    if t {
                        pc = 0x002083a8;
                        continue;
                    }
                    pc = 0x00208324;
                    continue;
                }
                0x00208324 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00208334);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2108));
                    call_ptr(c, 0x003a0c50, 0x00208334)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    if t {
                        pc = 0x002083a8;
                        continue;
                    }
                    pc = 0x0020833c;
                    continue;
                }
                0x0020833c => {
                    c.cycles += 22;
                    c.g[RA][0] = sx(0x00208344);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17b0));
                    call_ptr(c, 0x00317670, 0x00208344)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00208350);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x00208350)?;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0c));
                    c.g[RA][0] = sx(0x00208364);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
                    call_ptr(c, 0x001550e8, 0x00208364)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                    c.g[RA][0] = sx(0x00208380);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x00208380)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0020838c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003a0c50, 0x0020838c)?;
                    let t = c.g[S1][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    if t {
                        pc = 0x002083a8;
                        continue;
                    }
                    pc = 0x00208394;
                    continue;
                }
                0x00208394 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002083a4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2108));
                    call_ptr(c, 0x003a0c50, 0x002083a4)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xbb0));
                    pc = 0x002083a8;
                    continue;
                }
                0x002083a8 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xba0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xb50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xb30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xbc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00207430, 0x002083d4);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc803 file-load fakes (in the machine's own RAM).
    const OC803_CTRL: u32 = DATA + 0x20_0000;
    const OC803_MAGIC: u32 = 0x0803_c0de;

    macro_rules! oc803_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC803_CTRL) != OC803_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc803_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    // The file-load chain (EAFile_AllocRequest spins on test data): stop it
    // at the slotted entry points.
    oc803_fakes!(
        (
            oc803_fake_3de670,
            OC803_PREV_3DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003d_e670
        ),
        (
            oc803_fake_3e18c8,
            OC803_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc803_fake_3e1908,
            OC803_PREV_3E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
        (
            oc803_fake_3e22f0,
            OC803_PREV_3E22F0,
            ee::s14::OVERRIDE_003E22F0,
            ee::s14::f_003e22f0__generated,
            0x003e_22f0
        ),
    );

    #[test]
    fn event_select_show_event_paths_matches() {
        check(
            ee::s05::f_00206690__generated,
            event_select_show_event_paths,
            200,
            &|c, seed| {
                oc803_install_fakes();
                machine(c, seed, 0x6690, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(OC803_CTRL, OC803_MAGIC);
                c.pc = 0x00206690;
            },
        );
    }
}
