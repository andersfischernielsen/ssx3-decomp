//! Staging module oc1076 (wave wave266): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1076`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `cDynamicSprayFX::~cDynamicSprayFX(int flags)` (0x2df1c8): destructor of the
/// board spray effect. `a0` this, `a1` flags (bit 0: delete this, bit 1: also run
/// the owning rider's destructor). Points the rider's three table pointers
/// (`+0x6e8`, `+0x6d0`, `+0x6c0`) at the effect's static tables first; with flags
/// 0 it also builds stack copies of the two tables patched with this object's
/// offset (the compiler's inlined base-class unwinding) and points the rider at
/// those. Then it destroys the 0x210-byte emitters of the array at `this+0x28`
/// (count at `-0x10`, back to front, through their vtable slot `+0x20/+0x24`),
/// frees that array, the buffer at `this+0x2c` and the one at `this+0x7c`
/// (`EA_MemFree`), calls `Rider_dtor(*this, 0)` for flag 2 and
/// `__builtin_delete(this)` for flag 1. Kept as the generated block structure
/// (labels are the generated block addresses) so every register and cycle
/// matches.
#[allow(unreachable_code, unused_assignments, unused_variables)]
pub fn cdynamic_spray_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2df2ec: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
            c.g[V0][0] = 0x490000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ca0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[T0][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[A0][0] = 0x460000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
            c.g[A3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S1][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[V1][0] = 0x490000;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32));
            c.g[T3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            let t = c.g[T0][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_2df2ec;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2df234: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld((c.g[A1][0] as u32));
                    c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd((c.g[V1][0] as u32), c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002df234);
                        }
                        continue 'l_2df234;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca0));
                        c.g[A2][0] = c.ld((c.g[A1][0] as u32));
                        c.sd((c.g[V1][0] as u32), c.g[A2][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                        c.g[A1][0] = c.g[T1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
                        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2df298: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] != c.g[T2][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002df298);
                                }
                                continue 'l_2df298;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = c.ld((c.g[A2][0] as u32));
                                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2df2ec;
                            }
                        }
                    }
                }
            }
        }
        'b_2df348: {
            c.cycles += 4;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[S3][0] = c.g[T0][0] & 0x1;
            let t = c.g[A1][0] == 0u64;
            c.g[S2][0] = c.g[T0][0] & 0x2;
            if t {
                break 'b_2df348;
            } else {
                'b_2df340: {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = 0x210;
                    c.mult(0, 4, 3, 2);
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if c.g[A1][0] == c.g[S0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                        break 'b_2df340;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
                        'l_2df318: loop {
                            c.cycles += 9;
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002df330);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002df330)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                            if c.g[V0][0] != c.g[S0][0] {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
                                if event_due(c) {
                                    deliver_event_here(c, 0x002df318);
                                }
                                continue 'l_2df318;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                break 'b_2df340;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x002df348);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x002df348)?;
                break 'b_2df348;
            }
        }
        'b_2df360: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                break 'b_2df360;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002df35c);
                call_ptr(c, 0x00317e98, 0x002df35c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                break 'b_2df360;
            }
        }
        'b_2df370: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2df370;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002df370);
                call_ptr(c, 0x00317e98, 0x002df370)?;
                break 'b_2df370;
            }
        }
        'b_2df384: {
            c.cycles += 2;
            let t = c.g[S2][0] == 0u64;

            if t {
                break 'b_2df384;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                c.g[RA][0] = sx(0x002df384);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0011b978, 0x002df384)?;
                break 'b_2df384;
            }
        }
        'b_2df398: {
            c.cycles += 2;
            let t = c.g[S3][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
            if t {
                break 'b_2df398;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002df394);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00317e50, 0x002df394)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_2df398;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    })
}

/// `void PS2BezierMan_EmitPatchProjectorPasses(self, int kind, void *tbl,
/// sPatch *patch, unsigned lights, unsigned *handle)` (0x38d448): for every light
/// `k < gm->numLights` whose bit is set in `lights` (`t0`), emits one projector
/// pass for the patch: sets the render state's texture (`gm+0xe84` top state,
/// halfword `+0x12` from the light's texture at `gm+0x13f0+0xa0k+0x94`) and
/// its type bits to 3, then writes a 0x70-byte DMA record at `*handle` (uncached
/// alias): the light's 4x4 projection matrix (VU0 `lqc2/sqc2` from
/// `gm+0x1440+0xa0k`) and the VU1 start address taken from the program table
/// `tbl + 8*patch[1] + 0x30`; the record is then also queued in the draw ring
/// (`gm+0x18f0`, up to 2600 entries) by copying the 0x14 bytes of the state's
/// top state. `a2` tbl, `a3` patch, `t0` lights, `t1` handle. A leaf: no calls.
/// Kept as the generated block structure so every register and cycle matches.
#[allow(unreachable_code, unused_assignments, unused_variables)]
pub fn ps2_bezier_man_emit_patch_projector_passes(c: &mut Cpu) {
    'b_38d60c: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[T5][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.g[T0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(30, (c.g[SP][0] as u32));
        c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        if t {
            break 'b_38d60c;
        } else {
            c.cycles += 18;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[T0][0] = 0xfff0000;
            c.g[T9][0] = 0x60000;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[FP][0] = 0xffffffffffffffcf;
            c.g[S7][0] = 0x10000000;
            c.g[S7][0] = c.g[S7][0] | 0x6;
            c.g[S6][0] = 0x20000000;
            c.g[S5][0] = 0x6c040000;
            c.g[S5][0] = c.g[S5][0] | 0x8080;
            c.g[S5][0] = c.g[S5][0] << 32;
            c.g[S4][0] = 0x14000000;
            c.g[T0][0] = c.g[T0][0] | 0xffff;
            c.g[T9][0] = c.g[T9][0] | 0x8000;
            c.g[S3][0] = 0xffffffffffffffe0;
            c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1440));
            c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x13f0));
            'l_38d4d0: loop {
                'b_38d5f4: {
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[S2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0x30000000;
                    if t {
                        break 'b_38d5f4;
                    } else {
                        'b_38d5e8: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x94));
                            c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            c.g[T6][0] = c.lw((c.g[T1][0] as u32));
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                            c.g[V1][0] = c.g[T6][0] | c.g[A2][0];
                            c.g[A0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                            c.g[V0][0] = c.g[V0][0] | 0x30;
                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                            c.sd((c.g[V1][0] as u32), c.g[S7][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S6][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
                            c.lqc2(1, (c.g[T8][0] as u32));
                            c.lqc2(2, (c.g[T8][0] as u32).wrapping_add(0x10));
                            c.lqc2(3, (c.g[T8][0] as u32).wrapping_add(0x20));
                            c.lqc2(4, (c.g[T8][0] as u32).wrapping_add(0x30));
                            c.sqc2(1, (c.g[A1][0] as u32));
                            c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                            c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x60), 0u64);
                            c.g[S0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
                            c.g[T4][0] = c.g[V1][0] & c.g[T0][0];
                            c.sw((c.g[T1][0] as u32), c.g[T4][0]);
                            c.g[A3][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x18f0));
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            if t {
                                break 'b_38d5e8;
                            } else {
                                c.cycles += 29;
                                c.g[A1][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[T9][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                                c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
                                c.g[V1][0] = c.g[V1][0] & 0x1f;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                c.ldl(3, (c.g[T2][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[T2][0] as u32));
                                c.ldl(4, (c.g[T2][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[T2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[V0][0] as u32));
                                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[S0][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T6][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                                break 'b_38d5e8;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[T1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[T1][0] as u32), c.g[V0][0]);
                        break 'b_38d5f4;
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = (c.g[T5][0] < c.g[V0][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0xa0));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038d4d0);
                    }
                    continue 'l_38d4d0;
                } else {
                    break 'b_38d60c;
                }
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, (c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002df1c8, cdynamic_spray_fx_dtor, "cDynamicSprayFX_dtor"),
    (
        0x0038d448,
        ps2_bezier_man_emit_patch_projector_passes,
        "PS2BezierMan_EmitPatchProjectorPasses",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1076 callee fakes (in the machine's RAM).
    const OC1076_CTRL: u32 = DATA + 0xf_8000;
    const OC1076_MAGIC: u32 = 0x1076_c0de;

    static OC1076_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1076_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1076_PREV_RIDER_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemFree` (0x317e98), gated: logs the call (all registers) and returns.
    fn oc1076_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1076_CTRL) != OC1076_MAGIC {
            return fakes::fall_back(c, &OC1076_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        fakes::fake_call(c, 0x0031_7e98)
    }

    /// `__builtin_delete` (0x317e50), gated.
    fn oc1076_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1076_CTRL) != OC1076_MAGIC {
            return fakes::fall_back(c, &OC1076_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50)
    }

    /// `Rider_dtor` (0x11b978), gated.
    fn oc1076_fake_rider_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1076_CTRL) != OC1076_MAGIC {
            return fakes::fall_back(c, &OC1076_PREV_RIDER_DTOR, ee::s00::f_0011b978__generated);
        }
        fakes::fake_call(c, 0x0011_b978)
    }

    fn oc1076_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1076_fake_free,
            &OC1076_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1076_fake_delete,
            &OC1076_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc1076_fake_rider_dtor,
            &OC1076_PREV_RIDER_DTOR,
        );
    }

    /// `cDynamicSprayFX::~cDynamicSprayFX`: flags 0 (stack table copies), 1
    /// (delete), 2 (rider dtor), 3, and ones with other bits; the emitter array
    /// absent or with 0..3 elements (virtual dtor through a fake vtable slot);
    /// the two buffers present or absent. `EA_MemFree`, `Rider_dtor` and
    /// `__builtin_delete` are gated fakes that log their registers; fakes
    /// unwind at a random call.
    #[test]
    fn cdynamic_spray_fx_dtor_matches() {
        const THIS: u32 = DATA + 0x100;
        const RIDER: u32 = DATA + 0x1000;
        const ARR: u32 = DATA + 0x2010;
        const VT: u32 = DATA + 0x4000;
        oc1076_hook_callees();
        check(
            ee::s09::f_002df1c8__generated,
            cdynamic_spray_fx_dtor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2df1c8, 9);
                elf_image(c);
                c.bus.write32(OC1076_CTRL, OC1076_MAGIC);
                let flags = [0u32, 1, 2, 3, 4, 7][(seed % 6) as usize];
                fill(c, &mut r, THIS, 0x20);
                c.bus.write32(THIS, RIDER);
                // Emitter array: count at -0x10, elements of 0x210 bytes.
                let n = r.next_u32() % 4;
                let has_arr = seed % 5 != 3;
                c.bus.write32(THIS + 0x28, if has_arr { ARR } else { 0 });
                c.bus.write32(ARR - 0x10, n);
                c.bus.write16(VT + 0x20, (r.next_u32() % 0x40 * 4) as u16);
                c.bus.write32(VT + 0x24, FAKE);
                for i in 0..n {
                    c.bus.write32(ARR + 0x210 * i + 0x1f8, VT);
                }
                c.bus
                    .write32(THIS + 0x2c, if seed % 3 == 1 { 0 } else { DATA + 0x6000 });
                c.bus
                    .write32(THIS + 0x7c, if seed % 4 == 2 { 0 } else { DATA + 0x7000 });
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = flags as u64;
            },
        );
    }

    /// `PS2BezierMan_EmitPatchProjectorPasses`: 0..4 lights with random bit
    /// masks, the program table entry flag 0..3, the draw ring below and at its
    /// 2600-entry limit (no ring entry), and an empty light count.
    #[test]
    fn ps2_bezier_man_emit_patch_projector_passes_matches() {
        const GM: u32 = DATA + 0x3_0000;
        const TOP: u32 = DATA + 0x3_4000;
        const TABLE: u32 = DATA + 0x3_8000;
        const ENTRY: u32 = DATA + 0x3_9000;
        const HANDLE: u32 = DATA + 0x2_2000;
        const RING: u32 = DATA + 0x4_0000;
        check(
            ee::s12::f_0038d448__generated,
            ps2_bezier_man_emit_patch_projector_passes,
            300,
            &|c, seed| {
                let mut r = Rng(0x0038_d448 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0x2a90, GM);
                fill(c, &mut r, GM + 0x13ec, 0xc0);
                c.bus.write32(
                    GM + 0x13ec,
                    if seed % 9 == 4 { 0 } else { r.next_u32() % 5 },
                );
                c.bus.write32(GM + 0xe84, TOP);
                c.bus.write32(GM + 0x18f0, RING);
                fill(c, &mut r, TOP, 8);
                c.bus.write32(
                    RING,
                    if seed % 7 == 3 {
                        2600
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(RING + 0x69cc0, r.next_u32());
                c.bus.write32(RING + 0x69cc4, r.next_u32());
                fill(c, &mut r, TABLE, 0x80);
                c.bus.write32(ENTRY + 4, r.next_u32() % 4);
                c.bus
                    .write32(HANDLE, DATA + 0x3_c000 + (r.next_u32() & 0xff0));
                c.g[A2][0] = TABLE as u64;
                c.g[A3][0] = ENTRY as u64;
                c.g[T0][0] = (if seed % 5 == 0 { 0xf } else { r.next_u32() }) as u64;
                c.g[T1][0] = HANDLE as u64;
            },
        );
    }
}
