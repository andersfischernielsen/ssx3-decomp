//! Staging module oc1062 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1062`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_64c0,
        c_speed_boost_streamer_fx_dtor,
        "cSpeedBoostStreamerFX_dtor",
    ),
    (
        0x0036_8170,
        tex_man_emit_image_upload,
        "TexMan_EmitImageUpload",
    ),
];

/// `cSpeedBoostStreamerFX_dtor` (0x002e64c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_speed_boost_streamer_fx_dtor(c: &mut Cpu) {
    if c.pc != 0x002e64c0 {
        return ee::s09::f_002e64c0__generated(c);
    }
    guest(c, |c| {
        'b_2e65dc: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.g[V0][0] = 0x490000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ab0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[A0][0] = 0x460000;
            c.g[A3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[V1][0] = 0x490000;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7a68));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_2e65dc;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2e6524: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld((c.g[A1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[V1][0] as u32), c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e6524);
                        }
                        continue 'l_2e6524;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ab0));
                        c.g[A2][0] = c.ld((c.g[A1][0] as u32));
                        c.sd((c.g[V1][0] as u32), c.g[A2][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7a68));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A1][0] = c.g[T0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3b0));
                        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2e6588: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] != c.g[T1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002e6588);
                                }
                                continue 'l_2e6588;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7a68));
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T0][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2e65dc;
                            }
                        }
                    }
                }
            }
        }
        'b_2e65f0: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3ac));
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2e65f0;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002e65f0);
                call_ptr(c, 0x00317e98, 0x002e65f0)?;
                break 'b_2e65f0;
            }
        }
        'b_2e6618: {
            c.cycles += 6;
            c.mtc1(0, 12);
            c.g[RA][0] = sx(0x002e65fc);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            call_ptr(c, 0x0037c720, 0x002e65fc)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if t {
                break 'b_2e6618;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x002e6614);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0011b978, 0x002e6614)?;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                break 'b_2e6618;
            }
        }
        'b_2e662c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                break 'b_2e662c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002e6628);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x002e6628)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                break 'b_2e662c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    });
}

/// `WorldPainter_LoadViewState` (0x002ef2d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(dead_code)]
pub fn world_painter_load_view_state(c: &mut Cpu) {
    if c.pc != 0x002ef2d8 {
        return ee::s09::f_002ef2d8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x500000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a0));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        'l_2ef300: loop {
            c.cycles += 19;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x208));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002ef31c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x002ef31c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf0));
            c.g[A2][0] = c.lw((c.g[A0][0] as u32));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x208));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002ef340);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x002ef340)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                if event_due(c) {
                    deliver_event_here(c, 0x002ef300);
                }
                continue 'l_2ef300;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `TexMan_EmitImageUpload` (0x00368170): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn tex_man_emit_image_upload(c: &mut Cpu) {
    if c.pc != 0x00368170 {
        return ee::s12::f_00368170__generated(c);
    }
    guest(c, |c| {
        'b_368268: {
            'b_368264: {
                'b_368260: {
                    'b_368228: {
                        c.cycles += 43;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[FP][0] = c.g[A0][0];
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                        c.g[S7][0] = 0x8;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[A1][0] = 0x1;
                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        c.g[V0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A2][0] = 0xa;
                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                        c.g[A0][0] = c.g[V0][0] >> 26;
                        c.g[V1][0] = c.g[V0][0] >> 30;
                        c.g[V0][0] = c.g[V0][0] >> 20;
                        c.g[A0][0] = c.g[A0][0] & 0xf;
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V1][0] = c.g[V1][0] & 0xf;
                        c.g[S6][0] = c.g[V0][0] << 32;
                        c.g[S6][0] = ((c.g[S6][0] as i64) >> 32) as u64;
                        c.g[A0][0] = c.g[A0][0] << 32;
                        c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.g[V1][0] << 32;
                        c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                        c.g[T9][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                        c.g[T8][0] = sx((c.g[A1][0] as u32) << ((c.g[A0][0] as u32) & 31));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                        c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        let t = c.g[A3][0] != c.g[A2][0];
                        c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        if t {
                            break 'b_368228;
                        } else {
                            c.cycles += 3;
                            c.g[S6][0] = 0u64;
                            let t = true;
                            c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                            if t {
                                break 'b_368260;
                            } else {
                                break 'b_368228;
                            }
                        }
                    }
                    'b_368240: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x9;
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[V0][0] = 0xc;
                        if t {
                            break 'b_368240;
                        } else {
                            c.cycles += 3;
                            c.g[S6][0] = 0x1b;
                            let t = true;
                            c.g[S7][0] = 0x6;
                            if t {
                                break 'b_368264;
                            } else {
                                break 'b_368240;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        break 'b_368268;
                    } else {
                        'b_36825c: {
                            c.cycles += 2;
                            let t = c.g[T8][0] != c.g[T9][0];
                            c.g[S6][0] = 0u64;
                            if t {
                                break 'b_36825c;
                            } else {
                                c.cycles += 3;
                                c.g[T8][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                                let t = true;
                                c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 2) as u32);
                                if t {
                                    break 'b_368264;
                                } else {
                                    break 'b_36825c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 2) as u32);
                        break 'b_368260;
                    }
                }
                c.cycles += 1;
                c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                break 'b_368264;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            break 'b_368268;
        }
        'b_3684a0: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) < 0;
            c.g[S3][0] = 0u64;
            if t {
                break 'b_3684a0;
            } else {
                'l_368270: loop {
                    'b_368330: {
                        'b_368328: {
                            c.cycles += 30;
                            c.g[V1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = 0x1;
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.g[V1][0] >> 30;
                            c.g[S2][0] = 0x100;
                            c.g[V1][0] = c.g[V1][0] >> 26;
                            c.g[V0][0] = c.g[V0][0] & 0xf;
                            c.g[V1][0] = c.g[V1][0] & 0xf;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                            c.g[V1][0] = c.g[V1][0] << 32;
                            c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                            c.g[T3][0] = sx(((c.g[V0][0] as u32) as i32
                                >> ((c.g[S3][0] as u32) & 31))
                                as u32);
                            c.g[T2][0] = sx(((c.g[A0][0] as u32) as i32
                                >> ((c.g[S3][0] as u32) & 31))
                                as u32);
                            c.g[T4][0] = c.g[A3][0];
                            c.mult(0, 3, 10, 11);
                            c.g[A0][0] = 0xff;
                            c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x4));
                            c.g[A2][0] = (c.g[T0][0] < 0x1du64) as u64;
                            c.mult(0, 3, 3, 7);
                            c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
                            if c.g[A1][0] != 0 {
                                c.g[V0][0] = c.g[V1][0];
                            }
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[A2][0] == 0u64;
                            if c.g[A0][0] != 0 {
                                c.g[S2][0] = c.g[V0][0];
                            }
                            if t {
                                break 'b_368328;
                            } else {
                                'b_36831c: {
                                    'b_368310: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x490000;
                                        c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x22c0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00368304 => {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x80;
                                                let t = true;
                                                c.g[T1][0] = 0x80;
                                                if t {
                                                    break 'b_368330;
                                                } else {
                                                    break 'b_368310;
                                                }
                                            }
                                            0x00368310 => {
                                                break 'b_368310;
                                            }
                                            0x0036831c => {
                                                break 'b_36831c;
                                            }
                                            0x00368328 => {
                                                break 'b_368328;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00368170, 0x003684f0);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x80;
                                    let t = true;
                                    c.g[T1][0] = 0x40;
                                    if t {
                                        break 'b_368330;
                                    } else {
                                        break 'b_36831c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x40;
                                let t = true;
                                c.g[T1][0] = 0x40;
                                if t {
                                    break 'b_368330;
                                } else {
                                    break 'b_368328;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x40;
                        c.g[T1][0] = 0x20;
                        break 'b_368330;
                    }
                    c.cycles += 92;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V1][0] = ((c.g[T3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[T2][0];
                    }
                    if c.g[V1][0] == 0 {
                        c.g[T1][0] = c.g[T3][0];
                    }
                    c.mult(0, 9, 4, 9);
                    c.g[A1][0] = sx(((c.g[S2][0] as u32) as i32 >> 4) as u32);
                    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x3f));
                    c.g[T0][0] = 0x1;
                    c.g[A0][0] = 0u64;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = c.g[A1][0];
                    }
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] << 48;
                    c.g[A3][0] = c.g[S4][0] << 32;
                    c.g[V0][0] = 0x30000000;
                    c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
                    c.mult(0, 9, 9, 12);
                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                    if c.g[A0][0] != 0 {
                        c.g[T0][0] = c.g[A1][0];
                    }
                    c.g[V1][0] = 0x8000;
                    c.g[V1][0] = c.g[V1][0] << 44;
                    c.g[V0][0] = c.g[S6][0] << 56;
                    c.g[T0][0] = c.g[T0][0] | c.g[V1][0];
                    c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                    c.por(3, 0, 0);
                    c.g[V1][0] = 0x10000000;
                    c.g[V1][0] = c.g[V1][0] | 0x6;
                    c.sq(3, (c.g[S0][0] as u32));
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7));
                    c.g[T3][0] = c.g[S5][0] << 32;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[T1][0];
                    }
                    c.g[T2][0] = c.g[T9][0] << 32;
                    c.g[T3][0] = c.g[A2][0] | c.g[T3][0];
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[T2][0] = c.g[T8][0] | c.g[T2][0];
                    c.g[T7][0] = 0xe;
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x4;
                    c.g[A1][0] = 0x50;
                    c.g[A2][0] = 0x51;
                    c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
                    c.g[T4][0] = 0x52;
                    c.g[T5][0] = 0x53;
                    c.g[T6][0] = 0x10000000;
                    c.g[T6][0] = c.g[T6][0] | 0x2;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.g[A0][0] << 32;
                    c.g[A0][0] = c.g[A0][0] | 0x1;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A3][0]);
                    c.g[A2][0] = 0x10000000;
                    c.g[A2][0] = c.g[A2][0] << 32;
                    c.g[A2][0] = c.g[A2][0] | 0x8001;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x40), c.g[T2][0]);
                    c.g[A1][0] = 0x3f;
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x48), c.g[T4][0]);
                    c.g[V1][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[S7][0] as u32) & 31)) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x58), c.g[T5][0]);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x60), c.g[T0][0]);
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x70), c.g[T3][0]);
                    c.g[T8][0] = sx(((c.g[T8][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x80), c.g[T6][0]);
                    c.g[T9][0] = sx(((c.g[T9][0] as u32) as i32 >> 1) as u32);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T7][0]);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x88), 0u64);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x98), c.g[T7][0]);
                    c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    }
                    c.sd((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
                    c.sd((c.g[S0][0] as u32).wrapping_add(0x90), c.g[A0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00368270);
                        }
                        continue 'l_368270;
                    } else {
                        break 'b_3684a0;
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[A0][0] = c.g[FP][0];
        c.g[RA][0] = sx(0x003684ac);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x00368138,
            f: ee::s12::TexMan_EmitPath3Window,
        }
        .call(c, 0x003684ac)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sw((c.g[V0][0] as u32), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::{elf_image, fresh, run_generated, DATA};

    const OC1062_SELF: u32 = DATA + 0x1000;
    const OC1062_OBJ: u32 = DATA + 0x2000;

    /// `cSpeedBoostStreamerFX_dtor(self, flags)`: resets the vtables of the
    /// owner, copies the base vtable block when `flags == 0`, releases the
    /// streamer when `+0x3ac` is set, and frees `self` for flag bit 0.
    fn oc1062_dtor_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = machine(c, seed, 0x2e64c0, 4);
        elf_image(c);
        c.bus.write32(OC1062_SELF, OC1062_OBJ);
        c.bus.write32(
            OC1062_SELF + 0x3ac,
            if (seed / 4) % 2 == 0 { 0 } else { 0x1234 },
        );
        c.g[A0][0] = OC1062_SELF as u64;
        c.g[A1][0] = [0u64, 0, 1, 2, 3, 0][(seed % 6) as usize];
        let mode = schedule(c, &mut r, seed, 0x002e_64c0);
        if mode != Events::Off {
            c.next_event = c.cycles;
        }
        mode
    }

    /// Against the generated body: both the `flags == 0` copy loops and the
    /// flag paths, with event delivery in both loops.
    #[test]
    fn c_speed_boost_streamer_fx_dtor_matches() {
        check_events(
            ssx3_ee::s09::f_002e64c0__generated,
            c_speed_boost_streamer_fx_dtor,
            120,
            &oc1062_dtor_setup,
        );
    }

    #[test]
    fn c_speed_boost_streamer_fx_dtor_paths() {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            oc1062_dtor_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ssx3_ee::s09::f_002e64c0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 4, "only {} distinct paths", totals.len());
    }

    /// `WorldPainter_LoadViewState(view)`: calls a virtual function of the
    /// object held by each of the six 0xf0-byte entries of the painter table.
    fn oc1062_view_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = machine_rt(c, seed, 0x2ef2d8, 4);
        elf_image(c);
        let base = 0x0050_0000u32 - 0x5c90;
        for i in 0..6u32 {
            let ent = base + 0xf0 * i;
            let obj = DATA + 0x1000 + 0x100 * i;
            let inner = DATA + 0x3000 + 0x100 * i;
            let vt = DATA + 0x5000 + 0x100 * i;
            c.bus.write32(ent + 0x20, obj);
            c.bus.write32(obj, inner);
            c.bus.write32(inner + 4, vt);
            c.bus.write16(vt + 0x208, (r.next_u32() % 8 * 4) as u16);
            let f = if (seed + i) % 2 == 0 {
                0x001406c0
            } else {
                0x0031_7e98
            };
            c.bus.write32(vt + 0x20c, f);
        }
        c.g[A0][0] = (DATA + 0x7000) as u64;
        let mode = schedule(c, &mut r, seed, 0x002e_f2d8);
        if mode != Events::Off {
            c.next_event = c.cycles;
        }
        mode
    }

    /// `TexMan_EmitImageUpload(man, index, window, gif)`: emits the GIF upload
    /// packets (one 0xb0-byte block per mip level) for the image, whose
    /// format, size logs and bits per pixel pick the block shapes.
    fn oc1062_upload_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = machine(c, seed, 0x368170, 4);
        elf_image(c);
        let man = DATA + 0x100;
        let img = DATA + 0x800;
        let idx = seed % 3;
        c.bus.write32(man + idx * 4 + 8, img);
        let fmt = [0xau32, 9, 0xc, 0xc, 1, 0xa][((seed / 3) % 6) as usize];
        let wlog = r.next_u32() % 8;
        let hlog = if seed % 5 < 2 { wlog } else { r.next_u32() % 8 };
        let bits = ((wlog as u64) << 30)
            | ((hlog as u64) << 26)
            | (((r.next_u32() % 64) as u64) << 20)
            | (r.next_u32() as u64 & 0xf_ffff)
            | (((r.next_u32() & 0xf) as u64) << 34);
        let bpp = [4u32, 8, 16, 24, 32, 5, 12, 20, 28, 0, 3, 40][(r.next_u32() % 12) as usize];
        c.bus.write32(img + 0xc, fmt);
        c.bus.write64(img + 0x38, bits);
        c.bus.write32(img + 0x18, bpp);
        c.bus.write32(img + 0x1c, r.next_u32());
        c.bus.write32(img + 0x28, r.next_u32());
        let mips = if seed % 7 == 6 {
            u32::MAX
        } else {
            r.next_u32() % 4
        };
        c.bus.write32(img + 0x24, mips);
        // The window cursor cell and the GIF cursor cell.
        c.bus.write32(DATA + 0x1000, DATA + 0x6000);
        c.bus.write32(DATA + 0x1008, DATA + 0x4000);
        c.g[A0][0] = man as u64;
        c.g[A1][0] = idx as u64;
        c.g[A2][0] = (DATA + 0x1000) as u64;
        c.g[A3][0] = (DATA + 0x1008) as u64;
        let mode = schedule(c, &mut r, seed, 0x0036_8170);
        if mode != Events::Off {
            c.next_event = c.cycles;
        }
        mode
    }

    #[test]
    fn tex_man_emit_image_upload_matches() {
        check_events(
            ssx3_ee::s12::f_00368170__generated,
            tex_man_emit_image_upload,
            240,
            &oc1062_upload_setup,
        );
    }

    #[test]
    fn tex_man_emit_image_upload_paths() {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = fresh();
            oc1062_upload_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ssx3_ee::s12::f_00368170__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 8, "only {} distinct paths", totals.len());
    }
}
