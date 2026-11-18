//! Staging module oc411 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc411`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0030_dbd0, rider_model_setup, "RiderModel_Setup")];

/// `void RiderModel_Setup(RiderModel* self, int a1)` (0x30dbd0, DevvyDont:
/// `cAnimModel_compile`): finish a rider model after its parts were loaded.
/// Shrinks the part array to its count, sorts the parts, hands out mask bits and
/// matrix slots, allocates the matrix / index tables, registers the shape banks
/// with the graphics manager, compiles every part mesh for the four LOD channels
/// through the graphics manager's vtable (kept in `+0x40..`, `+0x44..`, `+0x80..`),
/// and builds the rest-pose matrices.
///
/// Fields: `+0` capacity, `+4` LOD count, `+8` part count, `+0xc` parts (0x58
/// each: `+0` index, `+4` bit offset, `+8` mesh offset, `+0xc` mesh slot, `+0x10`
/// mask, `+0x38` node array (0x50 each), `+0x44` node count, `+0x4c` mesh count),
/// `+0x14` max part index, `+0x18` the argument, `+0x1c`/`+0x20` index tables,
/// `+0x2c/+0x30/+0x34/+0x38` matrix arrays, `+0x60/+0x64` shape banks.
///
/// The body is a register-exact transcription of the generated control flow
/// (a goto-structured compiler output with many shared tails); calls go through
/// `call_ptr` and every unwind check is kept.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_variables,
    unused_labels,
    unused_parens,
    unused_mut
)]
pub fn rider_model_setup(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue; shrink the part array to its count when count < capacity.
        'b_30dce0: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x31c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x31a0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3190));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3180));
            c.g[S2][0] = 0x310000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3170));
            c.g[S3][0] = 0x490000;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x3130));
            c.g[S4][0] = 0x490000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3160));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3150));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3140));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x3120), c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[FP][0] = 0x490000;
            if t {
                break 'b_30dce0;
            } else {
                'b_30dccc: {
                    c.cycles += 13;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = 0x490000;
                    c.mult(0, 0, 3, 4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6390));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lo[0];
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0030dc48)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = 0u64;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if t {
                        break 'b_30dccc;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = 0x58;
                        c.mult(0, 2, 4, 6);
                        'l_30dc60: loop {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                            'l_30dc74: loop {
                                c.cycles += 11;
                                c.g[A3][0] = c.ld((c.g[A0][0] as u32));
                                c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                c.g[T2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sd((c.g[V0][0] as u32), c.g[A3][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030dc74);
                                    }
                                    continue 'l_30dc74;
                                } else {
                                    c.cycles += 11;
                                    c.g[T4][0] = c.ld((c.g[A0][0] as u32));
                                    c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.sd((c.g[V0][0] as u32), c.g[T4][0]);
                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.g[A1][0];
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.mult(0, 2, 4, 6);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0030dc60);
                                        }
                                        continue 'l_30dc60;
                                    } else {
                                        break 'b_30dccc;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_30dce0;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x0030dce0)?;
                    break 'b_30dce0;
                }
            }
        }
        // Sort the parts (qsort, RiderModel_CompareInt), size the part-index tables and
        // give every part its bit range in the 64-bit mask.
        'b_30de8c: {
            c.cycles += 56;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x2470));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0x58;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f4), 0u64);
            c.g[S0][0] = 0u64;
            call_ptr(c, 0x00418ef8, 0x0030dcfc)?;
            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x58;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
            c.mult(0, 3, 3, 2);
            c.g[T2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[T4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3104), c.g[T1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3108), c.g[T2][0]);
            c.g[A3][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3114), c.g[T4][0]);
            c.g[A2][0] = 0x100;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6370));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x58));
            c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x0030dd54)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f8), c.g[V0][0]);
            c.g[A1][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x003e6448, 0x0030dd74)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6360));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x0030dd90)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.g[A1][0] = 0xffffffffffff0000;
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x003e6448, 0x0030ddb0)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = 0u64;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if t {
                break 'b_30de8c;
            } else {
                c.cycles += 4;
                c.g[T1][0] = 0x40;
                c.g[T3][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[A2][0] = 0u64;
                'l_30ddd0: loop {
                    'b_30de78: {
                        'b_30de74: {
                            c.cycles += 38;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x44));
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                            c.g[V1][0] = c.g[T3][0] << (c.g[V1][0] & 63);
                            c.g[V1][0] = c.g[V1][0] >> (c.g[V0][0] & 63);
                            c.g[V1][0] = c.g[V1][0] << (c.g[T4][0] & 63);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[T4][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
                            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.sw((c.g[V0][0] as u32), c.g[T4][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f4), c.g[T4][0]);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            if t {
                                break 'b_30de74;
                            } else {
                                c.cycles += 3;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T0][0]);
                                let t = true;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_30de78;
                                } else {
                                    break 'b_30de74;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T2][0]);
                        break 'b_30de78;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x58));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ddd0);
                        }
                        continue 'l_30ddd0;
                    } else {
                        break 'b_30de8c;
                    }
                }
            }
        }
        // Mask of all used bits (+0x150), totals.
        'b_30df28: {
            c.cycles += 14;
            c.sd((c.g[S1][0] as u32).wrapping_add(0x150), 0u64);
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3118), c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x310c), c.g[A1][0]);
            c.g[S2][0] = sx((c.g[A2][0] as u32) << 5);
            c.g[S3][0] = sx((c.g[A2][0] as u32) << 4);
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S4][0] = sx((c.g[A2][0] as u32) << 2);
            if t {
                break 'b_30df28;
            } else {
                c.cycles += 3;
                c.g[T0][0] = 0x1;
                c.g[A2][0] = 0u64;
                'l_30ded0: loop {
                    'b_30df00: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] == 0u64;
                        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                        if t {
                            break 'b_30df00;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = c.g[T0][0] << (c.g[V0][0] & 63);
                            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            break 'b_30df00;
                        }
                    }
                    c.cycles += 10;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x150));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
                    let t = c.g[V1][0] != 0u64;
                    c.sd((c.g[S1][0] as u32).wrapping_add(0x150), c.g[A0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030ded0);
                        }
                        continue 'l_30ded0;
                    } else {
                        break 'b_30df28;
                    }
                }
            }
        }
        // Matrix arrays (+0x30, +0x34, +0x2c), initialised to identity.
        'b_30df64: {
            c.cycles += 13;
            c.sd((c.g[S1][0] as u32).wrapping_add(0x158), 0u64);
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6350));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
            call_ptr(c, 0x00317e30, 0x0030df48)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S5][0] as u32), c.g[A0][0]);
                break 'b_30df64;
            } else {
                c.cycles += 2;
                c.sw((c.g[S5][0] as u32), c.g[A0][0]);
                break 'b_30df64;
            }
        }
        'b_30df94: {
            c.cycles += 10;
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6350));
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030df78)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S6][0] as u32), c.g[A0][0]);
                break 'b_30df94;
            } else {
                c.cycles += 2;
                c.sw((c.g[S6][0] as u32), c.g[A0][0]);
                break 'b_30df94;
            }
        }
        'b_30dfc4: {
            c.cycles += 11;
            c.g[T4][0] = 0x490000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6340));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030dfac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S7][0] as u32), c.g[A0][0]);
                break 'b_30dfc4;
            } else {
                c.cycles += 1;
                c.sw((c.g[S7][0] as u32), c.g[A0][0]);
                break 'b_30dfc4;
            }
        }
        'b_30e060: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_30e060;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x500000;
                c.g[V1][0] = 0x500000;
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xdd0));
                'l_30dfe0: loop {
                    c.cycles += 32;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx((c.g[A2][0] as u32) << 6);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lqc2(1, (c.g[T0][0] as u32));
                    c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                    c.sqc2(1, (c.g[V0][0] as u32));
                    c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lqc2(1, (c.g[T0][0] as u32));
                    c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                    c.sqc2(1, (c.g[A0][0] as u32));
                    c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 5);
                    c.lq(4, (c.g[A3][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                    c.sq(4, (c.g[V1][0] as u32));
                    c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.lq(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                    let t = c.g[A1][0] != 0u64;
                    c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030dfe0);
                        }
                        continue 'l_30dfe0;
                    } else {
                        break 'b_30e060;
                    }
                }
            }
        }
        'b_30e0ac: {
            c.cycles += 11;
            c.g[T2][0] = 0x490000;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6330));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0030e078)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3104));
            if t {
                break 'b_30e0ac;
            } else {
                c.cycles += 1;
                'l_30e090: loop {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3104));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e090);
                        }
                        continue 'l_30e090;
                    } else {
                        break 'b_30e0ac;
                    }
                }
            }
        }
        'b_30e0e4: {
            c.cycles += 12;
            c.g[V0][0] = 0x490000;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6320));
            c.g[A2][0] = 0u64;
            c.sw((c.g[T4][0] as u32), c.g[A0][0]);
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00317e30, 0x0030e0c8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
            c.g[V0][0] = 0xffffffffffffffff;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3108));
                break 'b_30e0e4;
            } else {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3108));
                break 'b_30e0e4;
            }
        }
        'b_30e1a8: {
            c.cycles += 12;
            c.g[A2][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6310));
            c.g[A3][0] = 0u64;
            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x00317e30, 0x0030e100)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[T1][0] = 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_30e1a8;
            } else {
                c.cycles += 3;
                c.g[T6][0] = 0x58;
                c.g[T5][0] = 0x58;
                c.mult(0, 2, 4, 14);
                'l_30e120: loop {
                    'b_30e194: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[T0][0] = 0u64;
                        c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[T2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = (c.g[T2][0] as i64) <= 0;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                        if t {
                            break 'b_30e194;
                        } else {
                            c.cycles += 3;
                            c.mult(0, 11, 4, 13);
                            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                            'l_30e148: loop {
                                c.cycles += 19;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] =
                                    sx((c.g[T3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.g[A1][0] = ((c.g[T0][0] as i64) < (c.g[T2][0] as i64)) as u64;
                                c.sw((c.g[V0][0] as u32), c.g[T1][0]);
                                c.g[V1][0] = c.lhu((c.g[A3][0] as u32));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                                c.g[A0][0] = c.g[V1][0] & 0x1;
                                if c.g[A0][0] != 0 {
                                    c.g[T1][0] = c.g[A2][0];
                                }
                                c.g[V1][0] = c.g[V1][0] & 0x2;
                                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3));
                                let t = c.g[A1][0] != 0u64;
                                if c.g[V1][0] != 0 {
                                    c.g[T1][0] = c.g[V0][0];
                                }
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e148);
                                    }
                                    continue 'l_30e148;
                                } else {
                                    break 'b_30e194;
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[T4][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 2, 4, 14);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e120);
                        }
                        continue 'l_30e120;
                    } else {
                        break 'b_30e1a8;
                    }
                }
            }
        }
        // Per-part-mesh tables (+0x3c).
        'b_30e1c8: {
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x490000;
            if t {
                break 'b_30e1c8;
            } else {
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62f8));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x0030e1c4)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                break 'b_30e1c8;
            }
        }
        // Register the shape banks and look up their textures (PS2GraphicsMan).
        'b_30e260: {
            'b_30e25c: {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[S6][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                if t {
                    break 'b_30e25c;
                } else {
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[S5][0] = c.g[A2][0];
                    call_ptr(c, 0x00369890, 0x0030e1e4)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
                        break 'b_30e260;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V1][0] = 0u64;
                        if t {
                            break 'b_30e25c;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                            'l_30e200: loop {
                                'b_30e248: {
                                    c.cycles += 6;
                                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    let t = (c.g[S2][0] as i64) <= 0;
                                    c.g[S0][0] = 0u64;
                                    if t {
                                        break 'b_30e248;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S3][0] = c.g[A0][0];
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                                        'l_30e220: loop {
                                            c.cycles += 10;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A2][0] = c.g[SP][0];
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, 0x003b3d00, 0x0030e230)?;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] = c.g[SP][0];
                                            call_ptr(c, 0x003696c8, 0x0030e23c)?;
                                            c.g[V1][0] =
                                                ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                            if c.g[V1][0] != 0u64 {
                                                c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030e220);
                                                }
                                                continue 'l_30e220;
                                            } else {
                                                break 'b_30e248;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                c.g[V1][0] = c.g[S4][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e200);
                                    }
                                    continue 'l_30e200;
                                } else {
                                    break 'b_30e25c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
            break 'b_30e260;
        }
        c.cycles += 2;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30fc), 0u64);
        c.g[A0][0] = c.g[S6][0];
        // For each of the 4 LOD channels: compile every part mesh through the graphics
        // manager's vtable and keep the result.
        'l_30e268: loop {
            'b_30e424: {
                c.cycles += 8;
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x003698a0, 0x0030e270)?;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3110), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                if t {
                    break 'b_30e424;
                } else {
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x3100), c.g[T0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    'l_30e298: loop {
                        'b_30e414: {
                            'b_30e410: {
                                'b_30e3a4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S3][0] = 0u64;
                                    if t {
                                        break 'b_30e3a4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_30e410;
                                        } else {
                                            c.cycles += 7;
                                            c.g[T1][0] = 0x58;
                                            c.g[T2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                                            c.mult(0, 21, 16, 9);
                                            c.g[FP][0] = 0xc;
                                            c.g[S4][0] = sx((c.g[T2][0] as u32) << 3);
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            'l_30e2c8: loop {
                                                'b_30e388: {
                                                    c.cycles += 10;
                                                    c.g[A0][0] = sx((c.g[S3][0] as u32) << 4);
                                                    c.g[T1][0] = c.g[A0][0];
                                                    c.g[A3][0] = sx((c.g[S5][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[T0][0] = c.g[V1][0];
                                                    c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                        .wrapping_add((c.g[S4][0] as u32)));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                                                    let t = c.g[A1][0] == 0u64;
                                                    c.g[A2][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    if t {
                                                        break 'b_30e388;
                                                    } else {
                                                        'b_30e30c: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xc));
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.g[T2][0] = 0u64;
                                                            if t {
                                                                break 'b_30e30c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x3c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[T2][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                break 'b_30e30c;
                                                            }
                                                        }
                                                        c.cycles += 18;
                                                        c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                            .wrapping_add((c.g[T0][0] as u32)));
                                                        c.g[A3][0] = c.lw((c.g[S6][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S4][0] as u32)));
                                                        c.g[S2][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                        c.g[T0][0] = sx((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[A3][0] as u32)
                                                                .wrapping_add(0x368));
                                                        c.g[T3][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x36c));
                                                        c.g[S0][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[T1][0] as u32)));
                                                        c.g[A3][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f8));
                                                        c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        let t = c.g[T3][0] as u32;
                                                        c.g[T1][0] = c.g[S2][0];
                                                        call_ptr(c, t, 0x0030e340)?;
                                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                            .wrapping_add(0x30f0));
                                                        let t = c.g[A0][0] == 0u64;
                                                        c.g[T4][0] = 0x490000;
                                                        if t {
                                                            break 'b_30e388;
                                                        } else {
                                                            c.cycles += 13;
                                                            c.mult(0, 4, 4, 30);
                                                            c.g[A1][0] = sx((c.g[T4][0] as u32)
                                                                .wrapping_sub(0x62e8));
                                                            c.g[A2][0] = 0x20000000;
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317e30, 0x0030e368)?;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A1][0] = c.g[S2][0];
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x30f0));
                                                            c.mult(0, 0, 6, 30);
                                                            c.g[A2][0] = c.lo[0];
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x003e6574, 0x0030e384)?;
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30f0),
                                                                0u64,
                                                            );
                                                            break 'b_30e388;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0030e2c8);
                                                    }
                                                    continue 'l_30e2c8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    if t {
                                                        break 'b_30e414;
                                                    } else {
                                                        break 'b_30e3a4;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[V0][0] = 0x58;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3100));
                                c.mult(0, 4, 16, 2);
                                c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                let t = c.g[A1][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_30e410;
                                } else {
                                    'b_30e3ec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[T2][0] = 0u64;
                                        if t {
                                            break 'b_30e3ec;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[T2][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_30e3ec;
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
                                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30f0));
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x368));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x36c));
                                    c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0030e410)?;
                                    break 'b_30e410;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            break 'b_30e414;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = c.g[S7][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if event_due(c) {
                                deliver_event_here(c, 0x0030e298);
                            }
                            continue 'l_30e298;
                        } else {
                            break 'b_30e424;
                        }
                    }
                }
            }
            'b_30e494: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3110));
                if t {
                    break 'b_30e494;
                } else {
                    'b_30e490: {
                        c.cycles += 6;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f0));
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        let t = c.g[A0][0] == 0u64;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[A0][0]);
                        if t {
                            break 'b_30e490;
                        } else {
                            c.cycles += 18;
                            c.g[A3][0] = 0xc;
                            c.g[T0][0] = 0x490000;
                            c.mult(0, 4, 4, 7);
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x62e8));
                            c.g[A2][0] = 0x20000000;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317e30, 0x0030e46c)?;
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f0));
                            c.g[A0][0] = c.g[V0][0];
                            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                            c.g[V0][0] = 0xc;
                            c.mult(0, 0, 6, 2);
                            c.g[A2][0] = c.lo[0];
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x003e6574, 0x0030e48c)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30f0), 0u64);
                            break 'b_30e490;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3110));
                    break 'b_30e494;
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S6][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30fc), c.g[V1][0]);
            call_ptr(c, 0x003698a0, 0x0030e4a0)?;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30fc));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S6][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0030e268);
                }
                continue 'l_30e268;
            } else {
                'b_30e4c8: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f8));
                    if c.g[A2][0] == 0u64 {
                        c.g[A3][0] = 0x490000;
                        break 'b_30e4c8;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A2][0];
                        call_ptr(c, 0x00317e98, 0x0030e4c4)?;
                        c.g[A3][0] = 0x490000;
                        break 'b_30e4c8;
                    }
                }
                'b_30e50c: {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3118));
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x62d0));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0030e4dc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310c));
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3114));
                    if t {
                        break 'b_30e50c;
                    } else {
                        'l_30e4f0: loop {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3114));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e4f0);
                                }
                                continue 'l_30e4f0;
                            } else {
                                break 'b_30e50c;
                            }
                        }
                    }
                }
                // Compose the parts' rest matrices with their parents'.
                'b_30e748: {
                    c.cycles += 4;
                    c.sw((c.g[T0][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_30e748;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = 0x58;
                        c.g[T7][0] = 0x58;
                        c.mult(0, 3, 4, 16);
                        'l_30e528: loop {
                            'b_30e734: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[A3][0] = 0u64;
                                c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                let t = (c.g[V0][0] as i64) <= 0;
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                                if t {
                                    break 'b_30e734;
                                } else {
                                    c.cycles += 7;
                                    c.mult(0, 9, 4, 15);
                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                                    c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3010));
                                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3030));
                                    c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3020));
                                    c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30b0));
                                    c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3070));
                                    'l_30e560: loop {
                                        'b_30e71c: {
                                            'b_30e6dc: {
                                                c.cycles += 40;
                                                c.lq(2, (c.g[A2][0] as u32).wrapping_sub(0x10));
                                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                                                c.lq(3, (c.g[A2][0] as u32));
                                                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x3020));
                                                c.lqc2(4, (c.g[T4][0] as u32));
                                                c.lqc2(3, (c.g[T5][0] as u32));
                                                c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
                                                c.vu0().add(0b1110, 5, 4, Src::Vec(4));
                                                c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
                                                c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
                                                c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
                                                c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
                                                c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
                                                c.vu0().opmula(5, 4);
                                                c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
                                                c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
                                                c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
                                                c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
                                                c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
                                                c.vu0().suba(0b0010, 1, Src::Bc(6, X));
                                                c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
                                                c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
                                                c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
                                                c.vu0().adda(0b0010, 0, Src::Bc(8, X));
                                                c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
                                                c.vu0().adda(0b0100, 0, Src::Bc(8, X));
                                                c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
                                                c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
                                                c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
                                                c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
                                                c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
                                                c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
                                                c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
                                                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x30));
                                                c.sqc2(10, (c.g[T0][0] as u32));
                                                c.sqc2(11, (c.g[T0][0] as u32).wrapping_add(0x10));
                                                c.sqc2(12, (c.g[T0][0] as u32).wrapping_add(0x20));
                                                c.g[A0][0] =
                                                    c.lh((c.g[A2][0] as u32).wrapping_sub(0x1e));
                                                let t = (c.g[A0][0] as i64) < 0;
                                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
                                                if t {
                                                    break 'b_30e6dc;
                                                } else {
                                                    c.cycles += 55;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = sx((c.g[T1][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A3][0] as u32)));
                                                    c.lqc2(4, (c.g[A0][0] as u32));
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        6,
                                                        (c.g[A0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lqc2(8, (c.g[T0][0] as u32));
                                                    c.lqc2(
                                                        9,
                                                        (c.g[T0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        10,
                                                        (c.g[T0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        11,
                                                        (c.g[T0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                                                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                                                    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                                                    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                                                    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                                                    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                                                    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                                                    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                                                    c.sqc2(12, (c.g[T3][0] as u32));
                                                    c.sqc2(
                                                        13,
                                                        (c.g[T3][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        14,
                                                        (c.g[T3][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        15,
                                                        (c.g[T3][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lqc2(1, (c.g[T3][0] as u32));
                                                    c.lqc2(
                                                        2,
                                                        (c.g[T3][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[T3][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[T3][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.sqc2(1, (c.g[T2][0] as u32));
                                                    c.sqc2(
                                                        2,
                                                        (c.g[T2][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        3,
                                                        (c.g[T2][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        4,
                                                        (c.g[T2][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.lqc2(1, (c.g[T2][0] as u32));
                                                    c.lqc2(
                                                        2,
                                                        (c.g[T2][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lqc2(
                                                        3,
                                                        (c.g[T2][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.lqc2(
                                                        4,
                                                        (c.g[T2][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.sqc2(1, (c.g[V0][0] as u32));
                                                    c.sqc2(
                                                        2,
                                                        (c.g[V0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sqc2(
                                                        3,
                                                        (c.g[V0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.sqc2(
                                                        4,
                                                        (c.g[V0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    let t = true;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    if t {
                                                        break 'b_30e71c;
                                                    } else {
                                                        break 'b_30e6dc;
                                                    }
                                                }
                                            }
                                            c.cycles += 16;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[V0][0] = sx((c.g[T1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.lqc2(1, (c.g[T0][0] as u32));
                                            c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                                            c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                                            c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                                            c.sqc2(1, (c.g[V1][0] as u32));
                                            c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_30e71c;
                                        }
                                        c.cycles += 6;
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[T1][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                        c.g[V0][0] =
                                            ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0030e560);
                                            }
                                            continue 'l_30e560;
                                        } else {
                                            break 'b_30e734;
                                        }
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.g[T6][0];
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.mult(0, 3, 4, 16);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e528);
                                }
                                continue 'l_30e528;
                            } else {
                                break 'b_30e748;
                            }
                        }
                    }
                }
                // Normalise the part matrices.
                'b_30e81c: {
                    c.cycles += 3;
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                    let t = (c.g[T1][0] as i64) <= 0;
                    c.g[A2][0] = 0u64;
                    if t {
                        break 'b_30e81c;
                    } else {
                        c.cycles += 5;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 4);
                        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3010));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 5);
                        'l_30e768: loop {
                            c.cycles += 45;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[A2][0] as u32) << 6);
                            c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30f4));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[T2][0] as i64)) as u64;
                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x24));
                            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x24));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.mfc1(4, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x301c), 0u64);
                            c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.lqc2(4, (c.g[V1][0] as u32));
                            c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                            c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
                            c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                            c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3030));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x3020));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x3010));
                            let t = c.g[A1][0] != 0u64;
                            c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0030e768);
                                }
                                continue 'l_30e768;
                            } else {
                                break 'b_30e81c;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = 0u64;
                // Per bank: compile the shadow/vertex data through the vtable (+0x374).
                'l_30e830: loop {
                    'b_30e91c: {
                        c.cycles += 2;
                        let t = (c.g[A1][0] as i64) <= 0;
                        c.g[FP][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_30e91c;
                        } else {
                            c.cycles += 2;
                            c.g[S7][0] = 0x58;
                            'l_30e840: loop {
                                'b_30e908: {
                                    'b_30e904: {
                                        'b_30e8b8: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S0][0] = 0u64;
                                            if t {
                                                break 'b_30e8b8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                let t = (c.g[V0][0] as i64) <= 0;
                                                c.g[S4][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_30e904;
                                                } else {
                                                    c.cycles += 3;
                                                    c.mult(0, 19, 4, 23);
                                                    c.g[S2][0] = sx((c.g[S5][0] as u32) << 3);
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    'l_30e860: loop {
                                                        'b_30e89c: {
                                                            c.cycles += 8;
                                                            c.g[A0][0] =
                                                                sx((c.g[S0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[S2][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            c.g[A2][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                            let t = c.g[A1][0] == 0u64;
                                                            c.g[T0][0] = 0u64;
                                                            if t {
                                                                break 'b_30e89c;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[A3][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x370));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x374));
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[A0][0] = sx((c.g[S6][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x0030e89c)?;
                                                                break 'b_30e89c;
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[S0][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0030e860);
                                                            }
                                                            continue 'l_30e860;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_30e908;
                                                            } else {
                                                                break 'b_30e8b8;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 9;
                                        c.mult(0, 2, 4, 23);
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                        c.g[A2][0] = sx((c.g[S5][0] as u32) << 3);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A1][0] == 0u64;
                                        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_30e904;
                                        } else {
                                            c.cycles += 10;
                                            c.g[V1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                            c.g[T1][0] = c.g[V0][0];
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.g[T0][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x374));
                                            c.g[A2][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x40));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x0030e904)?;
                                            break 'b_30e904;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_30e908;
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030e840);
                                    }
                                    continue 'l_30e840;
                                } else {
                                    break 'b_30e91c;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S5][0] = c.g[FP][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 4i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0u64;
                        if event_due(c) {
                            deliver_event_here(c, 0x0030e830);
                        }
                        continue 'l_30e830;
                    } else {
                        'b_30e9b4: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
                            if t {
                                break 'b_30e9b4;
                            } else {
                                'b_30e9b0: {
                                    c.cycles += 14;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xe80));
                                    c.g[A0][0] = 0xc;
                                    c.g[A1][0] = 0x490000;
                                    c.g[A2][0] = 0u64;
                                    c.mult(0, 0, 2, 4);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62c0));
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
                                    c.g[A0][0] = c.lo[0];
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317e30, 0x0030e95c)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x12c));
                                    c.g[A1][0] = 0u64;
                                    let t = (c.g[V1][0] as i64) <= 0;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x130), c.g[V0][0]);
                                    if t {
                                        break 'b_30e9b0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x280));
                                        c.g[V1][0] = 0u64;
                                        'l_30e978: loop {
                                            c.cycles += 14;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x130));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                                            c.ldr(6, (c.g[A0][0] as u32));
                                            c.g[A3][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x7));
                                            c.sdr(6, (c.g[V0][0] as u32));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x12c));
                                            c.g[V0][0] =
                                                ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0030e978);
                                                }
                                                continue 'l_30e978;
                                            } else {
                                                break 'b_30e9b0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x31b0));
                                break 'b_30e9b4;
                            }
                        }
                        c.cycles += 11;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x31a0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3190));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3180));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3170));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3160));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3150));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3140));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x3130));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3120));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x31c0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, machine, mix, FAKE};
    use crate::difftest::{put_vec, DATA, GAME_GP};
    use crate::game::worldpainter::quadtree::tests::{heap_init, pool_allocator, run, MEM_HEAP};

    const BASE: u32 = DATA + 0x20_0000;
    const MODEL: u32 = BASE;
    const PARTS: u32 = BASE + 0x1000;
    const NODES: u32 = BASE + 0x2000;
    const LOD_TABLES: u32 = BASE + 0x4000;
    const MESH_DATA: u32 = BASE + 0x6000;
    const GRAPHICS_MAN: u32 = BASE + 0x8000;
    const MAN_VTABLE: u32 = BASE + 0x9000;
    const BANKS: u32 = BASE + 0xa000;

    /// The mesh compiler behind the graphics manager's vtable slot `+0x36c`
    /// reports how many triangles it wrote through its `t0` argument.
    fn compile_miss(c: &mut Cpu, target: u32) {
        if target == FAKE {
            let n = (mix(c.g[A1][0] ^ c.cycles) % 4) as u32;
            let out = c.g[T0][0] as u32;
            c.bus.write32(out, n);
        }
        fake_call(c, target)
    }

    /// `RiderModel_Setup`: 1 to 4 parts (in or out of index order, so the real
    /// `qsort` has work), each with 0 to 3 nodes, 0 to 2 meshes and per-LOD mesh
    /// tables; the part array at its capacity or larger (the shrink path with a
    /// real heap block); the argument zero or not (compile the first mesh only,
    /// or every mesh of every channel); 0 to 2 shape banks. The graphics
    /// manager's compile calls are fakes that return triangle counts and
    /// sometimes unwind. The texture-name lookup loop (non-zero argument with
    /// banks that have entries) is not reached: banks have no entries.
    #[test]
    fn rider_model_setup_matches() {
        covered(
            ssx3_ee::s10::f_0030dbd0__generated,
            rider_model_setup,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_dbd0, 6);
                pool_allocator(c);
                heap_init(c);
                c.hooks.miss = compile_miss;
                // Constant matrices the setup copies (0x4ff1a0.. and 0x4ff230..).
                for i in 0..4u32 {
                    let mut v = [0u32; 4];
                    v[i as usize] = 0x3f80_0000;
                    put_vec(c, 0x004f_f1a0 + 0x10 * i, v);
                }
                for i in 0..2u32 {
                    put_vec(
                        c,
                        0x004f_f230 + 0x10 * i,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
                for a in (MODEL..MODEL + 0x200).step_by(4) {
                    c.bus.write32(a, 0);
                }
                for a in (BASE + 0x1000..BASE + 0xb000).step_by(4) {
                    c.bus.write32(a, 0);
                }
                let count = 1 + seed % 4;
                let arg = (seed >> 2) & 1;
                let cap = if (seed >> 3) & 1 == 0 {
                    count + 1 + (r.next_u32() & 1)
                } else {
                    count
                };
                let reversed = (seed >> 4) & 1 == 1;
                let lods = 1 + (r.next_u32() & 1);
                let parts = if cap > count {
                    run(
                        c,
                        0x0032_0058,
                        ssx3_ee::s10::f_00320058,
                        &[MEM_HEAP as u64, 16, (0x58 * cap) as u64],
                    ) as u32
                } else {
                    PARTS
                };
                let mut nodes = [0u32; 4];
                for n in nodes.iter_mut().take(count as usize) {
                    *n = r.next_u32() % 4;
                }
                if nodes.iter().sum::<u32>() == 0 {
                    nodes[0] = 1;
                }
                let total: u32 = nodes.iter().sum();
                for i in 0..count {
                    let e = parts + 0x58 * i;
                    for k in (0..0x58).step_by(4) {
                        c.bus.write32(e + k, 0);
                    }
                    let index = (if reversed { count - 1 - i } else { i }) * 2;
                    c.bus.write32(e, index);
                    c.bus.write32(e + 0x44, nodes[i as usize]);
                    c.bus.write32(e + 0x4c, r.next_u32() % 3);
                    let node_base = NODES + 0x200 * i;
                    c.bus.write32(e + 0x38, node_base);
                    for j in 0..nodes[i as usize] {
                        let n = node_base + 0x50 * j;
                        for k in (0..0x50).step_by(4) {
                            let v = r.f32(2.0);
                            c.bus.write32(n + k, v);
                        }
                        let parent = (r.next_u32() % (total + 1)) as i32 - 1;
                        c.bus.write16(n + 0x12, parent as u16);
                        c.bus.write16(n + 0x14, (r.next_u32() & 3) as u16);
                    }
                    for ch in 0..4u32 {
                        let table = LOD_TABLES + (i * 4 + ch) * 0x40;
                        c.bus.write32(e + 0x1c + 8 * ch, table);
                        for j in 0..lods {
                            let data = if r.next_u32() % 3 == 0 { 0 } else { MESH_DATA };
                            c.bus.write32(table + 0x10 * j, data);
                            c.bus.write32(table + 0x10 * j + 4, r.next_u32() & 0xff);
                        }
                    }
                }
                c.bus.write32(MODEL, cap);
                c.bus.write32(MODEL + 4, lods);
                c.bus.write32(MODEL + 8, count);
                c.bus.write32(MODEL + 0xc, parts);
                let banks = r.next_u32() % 3;
                c.bus.write32(MODEL + 0x60, banks);
                for k in 0..banks {
                    c.bus.write32(MODEL + 0x64 + 4 * k, BANKS + 0x100 * k);
                }
                // The graphics manager: vtable object, name cache, no history buffer.
                c.bus.write32(GAME_GP + 0x2a90, GRAPHICS_MAN);
                c.bus.write32(GRAPHICS_MAN + 0x10d8, MAN_VTABLE);
                c.bus.write32(GRAPHICS_MAN + 0xe80, r.next_u32() % 4);
                for k in (0x280..0x2b0).step_by(4) {
                    c.bus.write32(GRAPHICS_MAN + k, r.next_u32());
                }
                for off in [0x368u32, 0x370] {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(MAN_VTABLE + off, delta as u16);
                }
                c.bus.write32(MAN_VTABLE + 0x36c, FAKE);
                c.bus.write32(MAN_VTABLE + 0x374, FAKE + 0x10);
                c.g[A0][0] = sx(MODEL);
                c.g[A1][0] = arg as u64;
                c.pc = 0x0030_dbd0;
            },
        );
    }
}
