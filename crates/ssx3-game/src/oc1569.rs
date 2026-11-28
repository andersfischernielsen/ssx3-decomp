//! Staging module oc1569 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1569`.

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
    (
        0x003a_19f8,
        cuitextscroll_set_string,
        "cUITextScroll_SetString",
    ),
    (0x003b_ed30, func_003bed30, "func_003BED30"),
    (0x003d_11b8, path_status_port, "PATH_status"),
];

/// `void cUITextScroll_SetString(cUITextScroll* self, const char* str)` (0x3a19f8):
/// replaces the scroll text's contents with `str`, word-wrapped into lines.
/// Calls the vtable method at `self->vtbl +0x28/+0x2c` (a `this` adjusted call),
/// fetches the font (`cUITextScroll_GetFont`; none: nothing happens), copies the
/// string into a freshly allocated UTF-16 buffer and walks it: `\\` (two
/// backslashes) and CR end a line (`cUITextScroll_AddLine`), any other character adds
/// `Font_GetCharWidthW(font, ch) * self->scale (+0x50)` to the running width;
/// a space remembers the last break point, and when the width exceeds the box
/// width (+0x60) the line is cut at the last space (or one character back)
/// and added. The rest is added at the end and the buffer freed.
///
/// Transcribed block for block from the generated body (every scratch register,
/// the callee-saved saves of s0-s7/fp/f20 and the cycle charges are kept).
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn cuitextscroll_set_string(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a1bc4: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A1][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = 0x1;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x003a1a48)?;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x003a12d0, 0x003a1a50)?;
            c.g[S7][0] = c.g[V0][0];
            if c.g[S7][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_3a1bc4;
            } else {
                'b_3a1b9c: {
                    c.cycles += 20;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S5][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00416810, 0x003a1a68)?;
                    c.g[S3][0] = 0u64;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4078));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    c.g[A2][0] = 0x100;
                    c.mtc1(0, 20);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x003a1a90)?;
                    c.g[S1][0] = 0u64;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2540, 0x003a1aa4)?;
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
                    if t {
                        break 'b_3a1b9c;
                    } else {
                        c.cycles += 1;
                        c.g[FP][0] = 0x5c;
                        'l_3a1ab0: loop {
                            'b_3a1b90: {
                                'b_3a1b1c: {
                                    'b_3a1b04: {
                                        'b_3a1ae4: {
                                            'b_3a1ae0: {
                                                c.cycles += 4;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[S2][0] as u32)));
                                                c.g[A0][0] = c.lhu((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] != c.g[FP][0];
                                                c.g[A1][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_3a1ae0;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                                    let t = c.g[V0][0] != c.g[A0][0];
                                                    c.g[S0][0] = sx((c.g[A1][0] as u32)
                                                        .wrapping_add((c.g[S2][0] as u32)));
                                                    if t {
                                                        break 'b_3a1ae4;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                                                        c.sh((c.g[V1][0] as u32), 0u64);
                                                        c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[A1][0] as u32)));
                                                        let t = true;
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2));
                                                        if t {
                                                            break 'b_3a1b04;
                                                        } else {
                                                            break 'b_3a1ae0;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[S2][0] as u32)));
                                            break 'b_3a1ae4;
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = 0xd;
                                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
                                        if c.g[V0][0] != c.g[V1][0] {
                                            c.g[A1][0] = c.lhu((c.g[S0][0] as u32));
                                            break 'b_3a1b1c;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                                            c.sh((c.g[S0][0] as u32), 0u64);
                                            c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                            break 'b_3a1b04;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[S5][0] = 0xffffffffffffffff;
                                    call_ptr(c, 0x003a1bf8, 0x003a1b10)?;
                                    c.mtc1(0, 20);
                                    let t = true;
                                    c.g[S1][0] = c.g[S3][0];
                                    if t {
                                        break 'b_3a1b90;
                                    } else {
                                        break 'b_3a1b1c;
                                    }
                                }
                                'b_3a1b8c: {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[S7][0];
                                    call_ptr(c, 0x00392c60, 0x003a1b24)?;
                                    c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x50));
                                    c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x60));
                                    c.mul_s(0, 0, 1);
                                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x20;
                                    c.add_s(20, 20, 0);
                                    c.c_lt_s(2, 20);
                                    let t = !c.fcc();
                                    if c.g[V0][0] == 0 {
                                        c.g[S5][0] = c.g[S3][0];
                                    }
                                    if t {
                                        break 'b_3a1b8c;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                                        if c.g[V0][0] == 0 {
                                            c.g[S5][0] = c.g[V1][0];
                                        }
                                        let t = (c.g[S5][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[S5][0] as u32) << 1);
                                        if t {
                                            break 'b_3a1b8c;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S2][0] as u32)));
                                            c.g[S3][0] = c.g[S5][0];
                                            c.sh((c.g[V0][0] as u32), 0u64);
                                            c.g[A1][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                            c.mtc1(0, 20);
                                            c.g[A0][0] = c.g[S4][0];
                                            call_ptr(c, 0x003a1bf8, 0x003a1b84)?;
                                            c.g[S5][0] = 0xffffffffffffffff;
                                            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                            break 'b_3a1b8c;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                break 'b_3a1b90;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003a1ab0);
                                }
                                continue 'l_3a1ab0;
                            } else {
                                break 'b_3a1b9c;
                            }
                        }
                    }
                }
                'b_3a1bb0: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == c.g[S3][0];
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_3a1bb0;
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = sx((c.g[S1][0] as u32) << 1);
                        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        call_ptr(c, 0x003a1bf8, 0x003a1bb0)?;
                        break 'b_3a1bb0;
                    }
                }
                c.cycles += 2;
                let t = c.g[S2][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t {
                    break 'b_3a1bc4;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317e98, 0x003a1bc0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    break 'b_3a1bc4;
                }
            }
        }
        c.cycles += 12;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

/// `int func_003BED30(int mode, int rate, char* buf)` (0x3bed30): sets up the
/// state block at `*0x50AC68` (0x98 bytes carved from `buf`, 16-byte aligned;
/// the aligned arena start is stored at `0x50AC68`'s pointer cell). `rate`
/// (0..100) picks the per-slot table size `f04`: < 34: 0xe0, < 67: 0x160,
/// < 101: 0x1e0; negative or >= 101 returns -2. `mode` 1 or 2 picks the slot
/// count `f01` (4 or 5); any other mode returns -1. Each slot gets a block
/// header and a zeroed table of `f04` words, and the six words at +0x6c are
/// cleared. Then the 24 per-channel scales (+0x0c) and the divided table
/// (`0x44CA40[i] = 0x44C700[i] / k`) are filled in three ranges whose
/// constants (0.00195694715, 0.00787401572, 0.0666666701) depend on the mode
/// (mode 1: 14 + 10 entries, mode 2: 5 + 9 + 10). Returns 0.
///
/// Transcribed block for block from the generated body: a leaf (no calls), so
/// every register it leaves (the `AT` constants of the float loads included)
/// is reproduced.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn func_003bed30(c: &mut Cpu) {
    'b_3bed54: {
        c.cycles += 9;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = 0x510000;
        c.g[A2][0] = c.g[A2][0] | 0xf;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        c.g[T4][0] = c.g[A0][0];
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x5398), c.g[T0][0]);
        let t = (c.g[A1][0] as i64) >= 0;
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x99));
        if t {
            'b_3bed8c: {
                'b_3bed70: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 34i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x5398));
                    if t {
                        break 'b_3bed70;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0xe0;
                        if t {
                            break 'b_3bed8c;
                        } else {
                            break 'b_3bed70;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 67i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x160;
                if t {
                    break 'b_3bed8c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 101i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x5398));
                    if t {
                        break 'b_3bed54;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1e0;
                        break 'b_3bed8c;
                    }
                }
            }
            'b_3bedbc: {
                'b_3beda8: {
                    c.cycles += 4;
                    c.sh((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = 0x1;
                    let t = c.g[T4][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_3beda8;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x5398));
                        let t = true;
                        c.g[V0][0] = 0x4;
                        if t {
                            break 'b_3bedbc;
                        } else {
                            break 'b_3beda8;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[T4][0] == c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x5398));
                if t {
                    c.cycles += 1;
                    c.g[V0][0] = 0x5;
                    break 'b_3bedbc;
                } else {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.pc = t;
                    return;
                }
            }
            'b_3bee78: {
                c.cycles += 9;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x1), c.g[V0][0]);
                c.g[T3][0] = c.g[A3][0];
                c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_sub(0x5398));
                c.sb((c.g[V0][0] as u32), c.g[T4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = 0u64;
                if t {
                    break 'b_3bee78;
                } else {
                    c.cycles += 2;
                    c.g[T5][0] = c.g[T3][0];
                    c.g[A1][0] = c.lw((c.g[T5][0] as u32).wrapping_sub(0x5398));
                    'l_3bede8: loop {
                        'b_3bee60: {
                            c.cycles += 15;
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                            c.g[A0][0] = c.g[A0][0] | 0xf;
                            c.g[T1][0] = sx((c.g[A3][0] as u32) << 2);
                            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x84), c.g[T0][0]);
                            c.g[V0][0] = c.g[V0][0] | 0xf;
                            c.g[A2][0] = 0u64;
                            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x1), c.g[T0][0]);
                            c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            let t = c.g[V1][0] == 0u64;
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            if t {
                                break 'b_3bee60;
                            } else {
                                c.cycles += 3;
                                c.g[T2][0] = c.g[T1][0];
                                c.g[T1][0] = 0x510000;
                                c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
                                'l_3bee30: loop {
                                    c.cycles += 12;
                                    c.g[A1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[A1][0] as u32), 0u64);
                                    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
                                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003bee30);
                                        }
                                        continue 'l_3bee30;
                                    } else {
                                        break 'b_3bee60;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_sub(0x5398));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.lw((c.g[T5][0] as u32).wrapping_sub(0x5398));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x003bede8);
                            }
                            continue 'l_3bede8;
                        } else {
                            break 'b_3bee78;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = 0x510000;
            c.g[A3][0] = 0x5;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5398));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
            'l_3bee88: loop {
                c.cycles += 7;
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                let t = (c.g[A3][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003bee88);
                    }
                    continue 'l_3bee88;
                } else {
                    'b_3bf0e0: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[T4][0] != c.g[V0][0];
                        c.g[A3][0] = 0u64;
                        if t {
                            c.cycles += 14;
                            c.g[V0][0] = 0x510000;
                            c.g[V1][0] = 0x450000;
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                            c.g[A0][0] = 0x450000;
                            c.g[AT][0] = 0x3b000000;
                            c.g[AT][0] = c.g[AT][0] | 0x4020;
                            c.mtc1(1, 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x35c0));
                            c.g[AT][0] = 0x3c340000;
                            c.g[AT][0] = c.g[AT][0] | 0x3900;
                            c.mtc1(1, 1);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3900));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                            'l_3befc8: loop {
                                c.cycles += 10;
                                c.swc1(2, (c.g[A1][0] as u32));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = ((c.g[A3][0] as i64) < 5i64) as u64;
                                c.lwc1(0, (c.g[A0][0] as u32));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.div_s(0, 0, 1);
                                c.swc1(0, (c.g[V1][0] as u32));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003befc8);
                                    }
                                    continue 'l_3befc8;
                                } else {
                                    'b_3bf068: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[A3][0] as i64) < 14i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x510000;
                                        if t {
                                            break 'b_3bf068;
                                        } else {
                                            c.cycles += 17;
                                            c.g[V1][0] = 0x450000;
                                            c.g[A2][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                                            c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                                            c.g[V0][0] = 0x450000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_sub(0x35c0));
                                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x3900));
                                            c.g[V1][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[AT][0] = 0x3c010000;
                                            c.g[AT][0] = c.g[AT][0] | 0x204;
                                            c.mtc1(1, 2);
                                            c.g[AT][0] = 0x3d0f0000;
                                            c.g[AT][0] = c.g[AT][0] | 0x5c40;
                                            c.mtc1(1, 1);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            'l_3bf040: loop {
                                                c.cycles += 10;
                                                c.swc1(2, (c.g[A0][0] as u32));
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] = ((c.g[A3][0] as i64) < 14i64) as u64;
                                                c.lwc1(0, (c.g[A1][0] as u32));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                                c.div_s(0, 0, 1);
                                                c.swc1(0, (c.g[V1][0] as u32));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003bf040);
                                                    }
                                                    continue 'l_3bf040;
                                                } else {
                                                    break 'b_3bf068;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x510000;
                                    if t {
                                        break 'b_3bf0e0;
                                    } else {
                                        c.cycles += 17;
                                        c.g[V1][0] = 0x450000;
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                                        c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                                        c.g[V0][0] = 0x450000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x35c0));
                                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3900));
                                        c.g[V1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[AT][0] = 0x3d880000;
                                        c.g[AT][0] = c.g[AT][0] | 0x8889;
                                        c.mtc1(1, 2);
                                        c.g[AT][0] = 0x3e420000;
                                        c.g[AT][0] = c.g[AT][0] | 0x8f60;
                                        c.mtc1(1, 1);
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        'l_3bf0b8: loop {
                                            c.cycles += 10;
                                            c.swc1(2, (c.g[A0][0] as u32));
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                                            c.lwc1(0, (c.g[A1][0] as u32));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            c.div_s(0, 0, 1);
                                            c.swc1(0, (c.g[V1][0] as u32));
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003bf0b8);
                                                }
                                                continue 'l_3bf0b8;
                                            } else {
                                                break 'b_3bf0e0;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            c.cycles += 14;
                            c.g[V0][0] = 0x510000;
                            c.g[V1][0] = 0x450000;
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                            c.g[A0][0] = 0x450000;
                            c.g[AT][0] = 0x3b000000;
                            c.g[AT][0] = c.g[AT][0] | 0x4020;
                            c.mtc1(1, 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x35c0));
                            c.g[AT][0] = 0x3c340000;
                            c.g[AT][0] = c.g[AT][0] | 0x3900;
                            c.mtc1(1, 1);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3900));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                            'l_3beee8: loop {
                                c.cycles += 10;
                                c.swc1(2, (c.g[A1][0] as u32));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = ((c.g[A3][0] as i64) < 14i64) as u64;
                                c.lwc1(0, (c.g[A0][0] as u32));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.div_s(0, 0, 1);
                                c.swc1(0, (c.g[V1][0] as u32));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003beee8);
                                    }
                                    continue 'l_3beee8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x510000;
                                    if t {
                                        break 'b_3bf0e0;
                                    } else {
                                        c.cycles += 17;
                                        c.g[V1][0] = 0x450000;
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                                        c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
                                        c.g[V0][0] = 0x450000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x35c0));
                                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3900));
                                        c.g[V1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[AT][0] = 0x3c010000;
                                        c.g[AT][0] = c.g[AT][0] | 0x204;
                                        c.mtc1(1, 2);
                                        c.g[AT][0] = 0x3d0f0000;
                                        c.g[AT][0] = c.g[AT][0] | 0x5c40;
                                        c.mtc1(1, 1);
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        'l_3bef60: loop {
                                            c.cycles += 10;
                                            c.swc1(2, (c.g[A0][0] as u32));
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                                            c.lwc1(0, (c.g[A1][0] as u32));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                            c.div_s(0, 0, 1);
                                            c.swc1(0, (c.g[V1][0] as u32));
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003bef60);
                                                }
                                                continue 'l_3bef60;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[RA][0] as u32;
                                                c.g[V0][0] = 0u64;
                                                c.pc = t;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0u64;
                    let t = c.g[RA][0] as u32;
                    c.pc = t;
                    return;
                }
            }
        } else {
            break 'b_3bed54;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xfffffffffffffffe;
    c.pc = t;
    return;
}

/// `int PATH_status(u32 id, PathStatus* out)` (0x3d11b8): fills `out` (12
/// words) with the playback status of path voice `id`: finds the track
/// (`PATH_FindTrack`) and its map (`PATH_FindMap(id & 0xff000000)`), asks
/// `PATH_PlatGetStatus` for the position (`off`, `len`) and then looks the
/// returned status up in the track's table (+0xfc, 0x10-byte entries with the key
/// at +0xc) to read the node format word from the path player (`0x44F420`):
/// samples per channel (`total / chans`) and per frame (`/ (chans * bytes)`)
/// and the remainders of `len` by them go to out+0x10/+0x14/+0x1c/+0x20. With the
/// debug flag (0x44F43C & 0x100) set and no table entry matching, the
/// miss is printed. Returns the status (or -8 / -18).
///
/// Resumable dispatcher: the generated body is a `match pc` loop whose back
/// edges (0x3d1218, 0x3d12e8) are resume points after an event unwinds the
/// thread, so the whole match is kept and starts at `c.pc`.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn path_status_port(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003d11b8 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[V0][0] = 0x450000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                    c.g[S3][0] = 0xfffffffffffffff8;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    if t {
                        pc = 0x003d1210;
                        continue;
                    }
                    pc = 0x003d11f0;
                    continue;
                }
                0x003d11f0 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = c.g[V1][0];
                    if t {
                        pc = 0x003d1210;
                        continue;
                    }
                    pc = 0x003d1200;
                    continue;
                }
                0x003d1200 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d1208);
                    c.g[A0][0] = 0xffffffffffffffff;
                    Callee {
                        addr: 0x003d5450,
                        f: ee::s14::PATH_SelectMap,
                    }
                    .call(c, 0x003d1208)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003d1230;
                        continue;
                    }
                    pc = 0x003d1210;
                    continue;
                }
                0x003d1210 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffee;
                    if t {
                        pc = 0x003d144c;
                        continue;
                    }
                    pc = 0x003d1218;
                    continue;
                }
                0x003d1218 => {
                    c.cycles += 2;

                    if c.g[A3][0] == 0u64 {
                        pc = 0x003d1220;
                        continue;
                    }
                    pc = 0x003d1220;
                    continue;
                }
                0x003d1220 => {
                    c.cycles += 4;
                    c.divu(0, 2, 7);
                    c.g[V1][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    if t {
                        pc = 0x003d1440;
                        continue;
                    }
                    pc = 0x003d1230;
                    continue;
                }
                0x003d1230 => {
                    c.cycles += 17;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                    c.g[S4][0] = 0x450000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003d1268);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    Callee {
                        addr: 0x003d5290,
                        f: ee::s14::PATH_FindTrack,
                    }
                    .call(c, 0x003d1268)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] == 0u64 {
                        c.sw((c.g[S5][0] as u32).wrapping_sub(0xbd0), 0u64);
                        pc = 0x003d1444;
                        continue;
                    }
                    pc = 0x003d1274;
                    continue;
                }
                0x003d1274 => {
                    c.cycles += 5;
                    c.g[A0][0] = 0xffffffffff000000;
                    c.g[RA][0] = sx(0x003d1280);
                    c.g[A0][0] = c.g[S2][0] & c.g[A0][0];
                    Callee {
                        addr: 0x003d4bb0,
                        f: ee::s14::PATH_FindMap,
                    }
                    .call(c, 0x003d1280)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[SP][0];
                    if t {
                        pc = 0x003d1440;
                        continue;
                    }
                    pc = 0x003d1288;
                    continue;
                }
                0x003d1288 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    c.g[RA][0] = sx(0x003d1298);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x002af2f0, 0x003d1298)?;
                    c.g[S3][0] = c.g[V0][0];
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x003d1440;
                        continue;
                    }
                    pc = 0x003d12a4;
                    continue;
                }
                0x003d12a4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x36));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 14);
                    c.g[V0][0] = c.g[V0][0] & 0x1f;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x003d141c;
                        continue;
                    }
                    pc = 0x003d12d0;
                    continue;
                }
                0x003d12d0 => {
                    c.cycles += 6;
                    c.g[T1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0xbe0));
                    c.g[T3][0] = 0xf00000;
                    c.g[T2][0] = 0u64;
                    c.g[T4][0] = 0xf000000;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xfc));
                    pc = 0x003d12e8;
                    continue;
                }
                0x003d12e8 => {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[A2][0] as u32) << 4);
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        pc = 0x003d1404;
                        continue;
                    }
                    pc = 0x003d12fc;
                    continue;
                }
                0x003d12fc => {
                    c.cycles += 18;
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[S0][0] as u32), c.g[A1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xfc));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lh((c.g[A1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V1][0] & c.g[T3][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.g[A3][0];
                    if t {
                        pc = 0x003d13d4;
                        continue;
                    }
                    pc = 0x003d1344;
                    continue;
                }
                0x003d1344 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 20);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.divu(0, 7, 2);

                    if c.g[V0][0] == c.g[T2][0] {
                        pc = 0x003d1358;
                        continue;
                    }
                    pc = 0x003d1358;
                    continue;
                }
                0x003d1358 => {
                    c.cycles += 9;
                    c.g[V1][0] = c.lo[0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[A1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 20);
                    c.g[A0][0] = c.g[A1][0] & c.g[T4][0];
                    c.g[A2][0] = c.g[V0][0] & 0xf;
                    c.divu(0, 7, 6);

                    if c.g[A2][0] == c.g[T2][0] {
                        pc = 0x003d137c;
                        continue;
                    }
                    pc = 0x003d137c;
                    continue;
                }
                0x003d137c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lo[0];
                    let t = c.g[A0][0] == 0u64;
                    c.g[T0][0] = c.g[V1][0];
                    if t {
                        pc = 0x003d13d4;
                        continue;
                    }
                    pc = 0x003d1388;
                    continue;
                }
                0x003d1388 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 24);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.mult(0, 2, 2, 6);
                    c.divu(0, 7, 2);

                    if c.g[V0][0] == c.g[T2][0] {
                        pc = 0x003d13a0;
                        continue;
                    }
                    pc = 0x003d13a0;
                    continue;
                }
                0x003d13a0 => {
                    c.cycles += 11;
                    c.g[V1][0] = c.lo[0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 24);
                    c.g[V1][0] = c.g[V1][0] & 0xf;
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.mult(0, 2, 2, 3);
                    c.divu(0, 7, 2);

                    if c.g[V0][0] == c.g[T2][0] {
                        pc = 0x003d13cc;
                        continue;
                    }
                    pc = 0x003d13cc;
                    continue;
                }
                0x003d13cc => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lo[0];
                    c.g[A3][0] = c.g[A0][0];
                    pc = 0x003d13d4;
                    continue;
                }
                0x003d13d4 => {
                    c.cycles += 2;

                    if c.g[T0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                        pc = 0x003d13f4;
                        continue;
                    }
                    pc = 0x003d13dc;
                    continue;
                }
                0x003d13dc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));

                    if c.g[T0][0] == c.g[T2][0] {
                        pc = 0x003d13e8;
                        continue;
                    }
                    pc = 0x003d13e8;
                    continue;
                }
                0x003d13e8 => {
                    c.cycles += 3;
                    c.divu(0, 2, 8);
                    c.g[V1][0] = c.hi[0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    pc = 0x003d13f4;
                    continue;
                }
                0x003d13f4 => {
                    c.cycles += 2;

                    if c.g[A3][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        if back_edge_unwinds(c, 0x003d1218) {
                            return Ok(());
                        }
                        pc = 0x003d1218;
                        continue;
                    }
                    pc = 0x003d13fc;
                    continue;
                }
                0x003d13fc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                    if t {
                        pc = 0x003d1440;
                        continue;
                    }
                    pc = 0x003d1404;
                    continue;
                }
                0x003d1404 => {
                    c.cycles += 6;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 14);
                    c.g[V0][0] = c.g[V0][0] & 0x1f;
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xfc));
                        if back_edge_unwinds(c, 0x003d12e8) {
                            return Ok(());
                        }
                        pc = 0x003d12e8;
                        continue;
                    }
                    pc = 0x003d141c;
                    continue;
                }
                0x003d141c => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbc4));
                    c.g[V0][0] = c.g[V0][0] & 0x100;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x490000;
                    if t {
                        pc = 0x003d1440;
                        continue;
                    }
                    pc = 0x003d1430;
                    continue;
                }
                0x003d1430 => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b10));
                    c.g[RA][0] = sx(0x003d1440);
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x003d1658,
                        f: ee::s14::PATH_DebugPrintf,
                    }
                    .call(c, 0x003d1440)?;
                    pc = 0x003d1440;
                    continue;
                }
                0x003d1440 => {
                    c.cycles += 1;
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0xbd0), 0u64);
                    pc = 0x003d1444;
                    continue;
                }
                0x003d1444 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S3][0];
                    c.sw((c.g[S4][0] as u32).wrapping_sub(0xbe0), 0u64);
                    pc = 0x003d144c;
                    continue;
                }
                0x003d144c => {
                    c.cycles += 9;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d11b8, 0x003d1470);
                    return Ok(());
                }
            }
        }
    });
}

/// Struct offsets of the cUITextScroll `SetString` reads (test fixtures).
#[cfg(test)]
pub(crate) const OC1569_SCROLL_VTBL: u32 = 0x08;
#[cfg(test)]
pub(crate) const OC1569_SCROLL_SCALE: u32 = 0x50;
#[cfg(test)]
pub(crate) const OC1569_SCROLL_WIDTH: u32 = 0x60;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (0 = fall back to the real callee).
    const OC1569_CTRL: u32 = DATA + 0x36_0000;
    const OC1569_MAGIC: u32 = 0x1569_0a01;
    /// Forced-result mailboxes, one word per callee address (`addr & 0xfffc`).
    const OC1569_MAIL: u32 = DATA + 0x37_0000;

    fn oc1569_mail(addr: u32) -> u32 {
        OC1569_MAIL + (addr & 0xfffc)
    }

    /// Second mailbox of a callee: f0 result (`Font_GetCharWidthW`) or, for
    /// `PATH_PlatGetStatus`, the `off` (+4) and `len` (+8) it writes out.
    fn oc1569_mail2(addr: u32) -> u32 {
        oc1569_mail(addr) + 0x1_0000
    }

    /// Force the fake for `addr` to return `v` (sign-extended; 0 forces zero).
    fn oc1569_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc1569_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs like `fakes::fake_call` (and may unwind), then
    /// applies the forced results unless it unwound.
    fn oc1569_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        let forced = c.bus.read32(oc1569_mail(addr));
        let m2 = oc1569_mail2(addr);
        let (w0, w1, w2) = (c.bus.read32(m2), c.bus.read32(m2 + 4), c.bus.read32(m2 + 8));
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if forced != 0 {
            c.g[V0][0] = sx(forced.wrapping_sub(3));
        }
        if addr == 0x0039_2c60 && w0 != 0 {
            c.f[0] = w0;
        }
        if addr == 0x002a_f2f0 {
            c.bus.write32(a1, w1);
            c.bus.write32(a2, w2);
        }
    }

    macro_rules! oc1569_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1569_CTRL) != OC1569_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1569_fake_call(c, $addr);
                }
            )*
            fn oc1569_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1569_fakes! {
        (oc1569_fake_3a12d0, OC1569_PREV_3A12D0, ee::s13::OVERRIDE_003A12D0, ee::s13::f_003a12d0__generated, 0x003a_12d0),
        (oc1569_fake_3a1bf8, OC1569_PREV_3A1BF8, ee::s13::OVERRIDE_003A1BF8, ee::s13::f_003a1bf8__generated, 0x003a_1bf8),
        (oc1569_fake_392c60, OC1569_PREV_392C60, ee::s13::OVERRIDE_00392C60, ee::s13::f_00392c60__generated, 0x0039_2c60),
        (oc1569_fake_317e30, OC1569_PREV_317E30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, 0x0031_7e30),
        (oc1569_fake_317e98, OC1569_PREV_317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98),
        (oc1569_fake_3d5450, OC1569_PREV_3D5450, ee::s14::OVERRIDE_003D5450, ee::s14::f_003d5450__generated, 0x003d_5450),
        (oc1569_fake_3d5290, OC1569_PREV_3D5290, ee::s14::OVERRIDE_003D5290, ee::s14::f_003d5290__generated, 0x003d_5290),
        (oc1569_fake_3d4bb0, OC1569_PREV_3D4BB0, ee::s14::OVERRIDE_003D4BB0, ee::s14::f_003d4bb0__generated, 0x003d_4bb0),
        (oc1569_fake_2af2f0, OC1569_PREV_2AF2F0, ee::s08::OVERRIDE_002AF2F0, ee::s08::f_002af2f0__generated, 0x002a_f2f0),
        (oc1569_fake_3d1658, OC1569_PREV_3D1658, ee::s14::OVERRIDE_003D1658, ee::s14::f_003d1658__generated, 0x003d_1658),
    }

    const SCROLL: u32 = DATA + 0x1000;
    const SCROLL_VT: u32 = DATA + 0x1100;
    const STR: u32 = DATA + 0x2000;
    const BUF: u32 = DATA + 0x3000;

    /// `cUITextScroll_SetString`: strings with spaces, CR and double backslashes
    /// (wrapping at several box widths), no font, empty text and no buffer, with
    /// the callees faked (the vtable call, `GetFont`, `AddLine`,
    /// `Font_GetCharWidthW`, allocator) and sometimes unwinding.
    #[test]
    fn cuitextscroll_set_string_matches() {
        oc1569_install_fakes();
        covered(
            ee::s13::f_003a19f8__generated,
            cuitextscroll_set_string,
            400,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x3a19f8, 7);
                c.pc = 0x003a_19f8;
                c.bus.write32(OC1569_CTRL, OC1569_MAGIC);
                c.g[A0][0] = SCROLL as u64;
                c.g[A1][0] = STR as u64;
                c.bus.write32(SCROLL + OC1569_SCROLL_VTBL, SCROLL_VT);
                c.bus.write16(SCROLL_VT + 0x28, (8 * (seed % 3)) as u16);
                c.bus.write32(SCROLL_VT + 0x2c, fakes::FAKE + 0x40);
                let scale = [1.0f32, 0.5, 2.0][(seed / 3 % 3) as usize];
                let limit = (6 + seed % 19) as f32;
                c.bus.write32(SCROLL + OC1569_SCROLL_SCALE, scale.to_bits());
                c.bus.write32(SCROLL + OC1569_SCROLL_WIDTH, limit.to_bits());
                // The glyph width every `Font_GetCharWidthW` call returns.
                let width = (1 + seed % 7) as f32;
                c.bus.write32(oc1569_mail2(0x0039_2c60), width.to_bits());
                // Font: none for one seed in eleven.
                if seed % 11 == 0 {
                    oc1569_force(c, 0x003a_12d0, 0);
                }
                // The text: spaces, CR, backslashes and letters.
                let empty = seed % 13 == 1;
                let len = if empty { 0 } else { 1 + r.next_u32() % 30 };
                for i in 0..len {
                    let ch = match r.next_u32() % 8 {
                        0 | 1 => b' ',
                        2 => b'\\',
                        3 if seed % 5 == 0 => b'\r',
                        _ => b'a' + (r.next_u32() % 26) as u8,
                    };
                    c.bus.write8(STR + i, ch);
                }
                if seed % 4 == 0 && len > 5 {
                    c.bus.write8(STR + 2, b'\\');
                    c.bus.write8(STR + 3, b'\\');
                }
                c.bus.write8(STR + len, 0);
                // The allocator: a real buffer, or none for the empty text.
                oc1569_force(c, 0x0031_7e30, if empty { 0 } else { BUF });
            },
        );
    }

    const MIX_BUF: u32 = DATA + 0x1_0000;
    /// Source table of the divided scales (`0x44C700`), 24 floats.
    const MIX_SRC: u32 = 0x0044_c700;

    /// `func_003BED30`: every rate bracket and mode, an unaligned buffer.
    #[test]
    fn func_003bed30_matches() {
        check(
            ee::s13::f_003bed30__generated,
            func_003bed30,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3bed30, 0);
                c.g[A0][0] = [1u64, 2, 0, 3, 1, 2][(seed % 6) as usize];
                let rate: i64 =
                    [-5, 0, 10, 33, 34, 50, 66, 67, 90, 100, 101, 200][(seed / 6 % 12) as usize];
                c.g[A1][0] = rate as u64;
                c.g[A2][0] = (MIX_BUF + r.next_u32() % 32) as u64;
                for i in 0..24 {
                    c.bus.write32(MIX_SRC + 4 * i, r.f32(100.0));
                }
            },
        );
    }

    /// The globals `PATH_status` reads (`0x44F42C` flag byte, `0x44F430` busy word,
    /// `0x44F420` the path player, `0x44F43C` debug flags).
    const PS_ON: u32 = 0x0044_f42c;
    const PS_BUSY: u32 = 0x0044_f430;
    const PS_PLAYER: u32 = 0x0044_f420;
    const PS_FLAGS: u32 = 0x0044_f43c;
    const PS_OUT: u32 = DATA + 0x1000;
    const PS_TRACK: u32 = DATA + 0x2000;
    const PS_TABLE: u32 = DATA + 0x3000;
    const PS_PLAYER_BLOCK: u32 = DATA + 0x5000;
    const PS_HDR: u32 = DATA + 0x6000;
    const PS_IDX: u32 = DATA + 0x7000;

    /// `PATH_status`: not ready / busy, no track or map, a failing status call,
    /// table with and without a matching key (node formats with zero and non-zero
    /// channel/byte nibbles), the debug print, and fakes that unwind.
    #[test]
    fn path_status_matches() {
        oc1569_install_fakes();
        covered(
            ee::s14::f_003d11b8__generated,
            path_status_port,
            400,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3d11b8, 6);
                c.pc = 0x003d_11b8;
                c.bus.write32(OC1569_CTRL, OC1569_MAGIC);
                c.g[A0][0] = (0x0100_0000 | (r.next_u32() & 0xff_ffff)) as u64;
                c.g[A1][0] = PS_OUT as u64;
                c.bus.write8(PS_ON, (seed % 9 != 0) as u8);
                c.bus.write32(PS_BUSY, (seed % 10 == 1) as u32);
                c.bus.write32(PS_PLAYER, PS_PLAYER_BLOCK);
                c.bus
                    .write32(PS_FLAGS, if seed % 2 == 0 { 0x100 } else { 0 });
                // PATH_SelectMap, FindTrack, FindMap results.
                if seed % 17 == 2 {
                    oc1569_force(c, 0x003d_5450, 0);
                }
                oc1569_force(c, 0x003d_5290, if seed % 19 == 3 { 0 } else { PS_TRACK });
                if seed % 23 == 4 {
                    oc1569_force(c, 0x003d_4bb0, 0);
                }
                // The status call: a key in 0..4, sometimes negative.
                let key = r.next_u32() % 4;
                let status = if seed % 29 == 5 { 0xffff_fffe } else { key };
                oc1569_force(c, 0x002a_f2f0, status);
                let m2 = oc1569_mail2(0x002a_f2f0);
                c.bus.write32(m2 + 4, r.next_u32() & 0xffff);
                c.bus.write32(m2 + 8, r.next_u32() & 0xffff);
                // The track: node count n (0..3), table of {a, .., key}.
                let n = r.next_u32() % 4;
                c.bus.write32(
                    PS_TRACK + 0x20,
                    (n << 14) | (r.next_u32() & 0x3fff) | (r.next_u32() & 0x1fff) << 19,
                );
                c.bus.write32(PS_TRACK + 0x54, r.next_u32());
                c.bus.write16(PS_TRACK + 0x36, r.next_u32() as u16);
                c.bus.write32(PS_TRACK + 0xfc, PS_TABLE);
                for i in 0..n {
                    let e = PS_TABLE + 0x10 * i;
                    c.bus.write32(e, r.next_u32() % 4);
                    c.bus.write32(e + 8, r.next_u32());
                    let hit = seed % 3 != 0 && i == n - 1;
                    c.bus
                        .write32(e + 0xc, if hit { key } else { r.next_u32() % 4 });
                }
                // The path player: header base +0x34, index array +0x38.
                c.bus.write32(PS_PLAYER_BLOCK + 0x34, PS_HDR);
                c.bus.write32(PS_PLAYER_BLOCK + 0x38, PS_IDX);
                for i in 0..4 {
                    c.bus.write16(PS_IDX + 2 * i, (r.next_u32() % 4) as u16);
                    let chans = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 15
                    };
                    let bytes = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 15
                    };
                    c.bus.write32(
                        PS_HDR + 4 * i + 4,
                        (bytes << 24) | (chans << 20) | (r.next_u32() & 0xf_ffff),
                    );
                }
            },
        );
    }
}
