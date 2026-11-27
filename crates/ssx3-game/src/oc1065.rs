//! Staging module oc1065 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1065`.

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
        0x0038_ce20,
        ps2_bezier_man_emit_patch_type2,
        "PS2BezierMan_EmitPatchType2",
    ),
    (
        0x0038_d168,
        ps2_bezier_man_emit_patch_matrix_pass,
        "PS2BezierMan_EmitPatchMatrixPass",
    ),
    (0x0038_f9c0, tex_man_swizzle_t4_page, "TexMan_SwizzleT4Page"),
    (0x0038_fc48, tex_man_swizzle_t8_page, "TexMan_SwizzleT8Page"),
    (0x0038_fed0, tex_man_swizzle_t4, "TexMan_SwizzleT4"),
];

/// `void PS2BezierMan_EmitPatchType2(BezMan* self, int sub, Patch* patch, int* ext, uint* handle)`
/// (0x0038ce20): emits one 4x4-bezier ("type 2") patch into the GIF packet
/// list at `*handle`. Uploads the patch record to the scratchpad, tests its
/// bounds, sets the draw-state texture ids and the 0x180000 reflection/shadow
/// bits, writes the DMA packet (optional `ext` tag, then the program call),
/// queues a ring entry, then runs the matrix pass (reflection flag) and the
/// projector passes (shadow flag).
pub fn ps2_bezier_man_emit_patch_type2(c: &mut Cpu) {
    guest(c, |c| {
        'b_38ceb4: {
            c.cycles += 35;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = 0xc;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.mult(0, 2, 16, 2);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = 0x1b0;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S6][0] = c.g[A2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[FP][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = 0x2;
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[S6][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[RA][0] = sx(0x0038ce84);
            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
            Callee {
                addr: 0x0038f708,
                f: ee::s12::SprStream_Upload,
            }
            .call(c, 0x0038ce84)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x454));
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = 0x600000;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 8);
            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[S2][0] = c.g[A0][0] & c.g[V0][0];
            c.g[S2][0] = (0u64 < c.g[S2][0]) as u64;
            let t = c.g[S2][0] == 0u64;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            if t {
                break 'b_38ceb4;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13e8));
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = 0u64;
                }
                break 'b_38ceb4;
            }
        }
        'b_38cf74: {
            'b_38cf70: {
                'b_38cf68: {
                    'b_38cf64: {
                        'b_38cf54: {
                            'b_38cf40: {
                                c.cycles += 19;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[RA][0] = sx(0x0038cec0);
                                c.g[A1][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x0038ca08,
                                    f: ee::s12::PS2BezierMan_TestPatchBounds,
                                }
                                .call(c, 0x0038cec0)?;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A2][0] = 0x180000;
                                c.g[A1][0] = 0x100000;
                                c.g[A0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x1a0));
                                c.g[S7][0] = (0u64 < c.g[V0][0]) as u64;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13ec));
                                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                                if c.g[V0][0] != 0 {
                                    c.g[S7][0] = 0u64;
                                }
                                c.g[A0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x1a2));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                                if c.g[V1][0] == c.g[A1][0] {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                    break 'b_38cf40;
                                } else {
                                    'b_38cf2c: {
                                        'b_38cf1c: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = 0x80000;
                                            if t {
                                                break 'b_38cf1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                                if t {
                                                    break 'b_38cf2c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffff3;
                                                    if t {
                                                        break 'b_38cf68;
                                                    } else {
                                                        break 'b_38cf1c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[A2][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                            break 'b_38cf54;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                            if t {
                                                break 'b_38cf64;
                                            } else {
                                                break 'b_38cf2c;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V1][0] = 0xfffffffffffffff3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    let t = true;
                                    c.g[V0][0] = c.g[V0][0] | 0x8;
                                    if t {
                                        break 'b_38cf70;
                                    } else {
                                        break 'b_38cf40;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V1][0] = 0xfffffffffffffff3;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            if t {
                                break 'b_38cf70;
                            } else {
                                break 'b_38cf54;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.g[V0][0] | 0xc;
                        let t = true;
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        if t {
                            break 'b_38cf74;
                        } else {
                            break 'b_38cf64;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0xfffffffffffffff3;
                    break 'b_38cf68;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                break 'b_38cf70;
            }
            c.cycles += 1;
            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
            break 'b_38cf74;
        }
        'b_38cfe8: {
            c.cycles += 4;
            c.g[T2][0] = c.lw((c.g[S4][0] as u32));
            c.g[T3][0] = 0x30000000;
            let t = c.g[FP][0] == 0u64;
            c.g[T1][0] = c.g[T2][0] | c.g[T3][0];
            if t {
                break 'b_38cfe8;
            } else {
                c.cycles += 25;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
                c.sd((c.g[T1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = 0x6e010000;
                c.g[A0][0] = c.g[A0][0] | 0x80d0;
                c.g[V0][0] = 0x6c100000;
                c.g[V0][0] = c.g[V0][0] | 0x80c0;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A1][0] = 0x30000000;
                c.g[A1][0] = c.g[A1][0] | 0x10;
                c.g[V1][0] = c.lw((c.g[FP][0] as u32));
                c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] << 32;
                c.sd((c.g[T1][0] as u32).wrapping_add(0x18), 0u64);
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.sd((c.g[T1][0] as u32).wrapping_add(0x20), 0u64);
                c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                break 'b_38cfe8;
            }
        }
        'b_38d0e4: {
            c.cycles += 31;
            c.g[V0][0] = c.g[S0][0] << 32;
            c.g[V1][0] = 0x50000000;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A0][0] = 0x10000000;
            c.g[A0][0] = c.g[A0][0] | 0x1;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.g[V1][0] = 0x500000;
            c.sd((c.g[T1][0] as u32), c.g[V0][0]);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4960));
            c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A1][0] = 0x14000000;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x18), 0u64);
            c.g[A0][0] = 0xfff0000;
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
            c.sd((c.g[T1][0] as u32).wrapping_add(0x20), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[V1][0] = c.g[V1][0] << 32;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x30));
            c.g[T1][0] = c.g[T1][0] & c.g[A0][0];
            c.sw((c.g[S4][0] as u32), c.g[T1][0]);
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
            if t {
                break 'b_38d0e4;
            } else {
                c.cycles += 32;
                c.g[A1][0] = 0x60000;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A1][0] | 0x8000;
                c.g[V1][0] = 0xffffffffffffffe0;
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = c.g[A2][0] | c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[T0][0] as u32));
                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(3, (c.g[V0][0] as u32));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                break 'b_38d0e4;
            }
        }
        'b_38d114: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            let t = c.g[S2][0] == 0u64;
            c.sw((c.g[S4][0] as u32), c.g[V0][0]);
            if t {
                break 'b_38d114;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A3][0] = 0x2;
                c.g[T0][0] = c.g[S0][0];
                c.g[T1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0038d114);
                c.g[T2][0] = c.g[S4][0];
                Callee {
                    addr: 0x0038d168,
                    f: ee::s12::PS2BezierMan_EmitPatchMatrixPass,
                }
                .call(c, 0x0038d114)?;
                break 'b_38d114;
            }
        }
        'b_38d134: {
            c.cycles += 2;
            let t = c.g[S7][0] == 0u64;
            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            if t {
                break 'b_38d134;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S6][0];
                c.g[T1][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0038d134);
                c.g[A1][0] = 0x2;
                Callee {
                    addr: 0x0038d448,
                    f: ee::s12::PS2BezierMan_EmitPatchProjectorPasses,
                }
                .call(c, 0x0038d134)?;
                break 'b_38d134;
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
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

/// `void PS2BezierMan_EmitPatchMatrixPass(BezMan* self, void* cacheEntry, Patch* obj,
/// int type, void* progs, Patch* prog, uint* handle)` (0x0038d168): pushes a copy
/// of the current draw state, picks the vector data address for the patch type
/// (0: 0xc0 stride, 1: 432, 2: 0x300), sets texture ids and bit groups, writes the
/// GIF packet with the matrix copy and program call, queues a ring entry and pops
/// the draw state again. A leaf: no guest calls.
pub fn ps2_bezier_man_emit_patch_matrix_pass(c: &mut Cpu) {
    'b_38d224: {
        'b_38d218: {
            'b_38d210: {
                'b_38d204: {
                    c.cycles += 25;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[T4][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.sq(16, (c.g[SP][0] as u32));
                    c.g[T5][0] = c.g[A0][0];
                    c.g[T3][0] = c.g[A3][0];
                    c.g[S0][0] = c.g[T0][0];
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                    c.g[T9][0] = c.g[T1][0];
                    c.g[T6][0] = c.g[T2][0];
                    c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[V0][0] as u32));
                    c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
                    c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x23));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A3][0]);
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[T4][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                    let t = c.g[T3][0] == c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_38d204;
                    } else {
                        'b_38d1f8: {
                            'b_38d1e8: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T3][0] as i64) < 2i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_38d1e8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T3][0] == 0u64;
                                    c.g[A3][0] = 0u64;
                                    if t {
                                        break 'b_38d1f8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                                        if t {
                                            break 'b_38d224;
                                        } else {
                                            break 'b_38d1e8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[T3][0] == c.g[V0][0];
                            c.g[A3][0] = 0u64;
                            if t {
                                break 'b_38d210;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                                if t {
                                    break 'b_38d224;
                                } else {
                                    break 'b_38d1f8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0xc0;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1c4));
                        if t {
                            break 'b_38d218;
                        } else {
                            break 'b_38d204;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1b0;
                let t = true;
                c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1c8));
                if t {
                    break 'b_38d218;
                } else {
                    break 'b_38d210;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x300;
            c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1cc));
            break 'b_38d218;
        }
        c.cycles += 3;
        c.mult(0, 2, 4, 2);
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        break 'b_38d224;
    }
    'b_38d238: {
        c.cycles += 5;
        c.g[V0][0] = 0xffffffffffffffff;
        c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x1a4));
        if (c.g[V1][0] as i64) < 0 {
            c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x10a8));
            break 'b_38d238;
        } else {
            break 'b_38d238;
        }
    }
    'b_38d288: {
        'b_38d278: {
            c.cycles += 11;
            c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
            c.g[A1][0] = 0x400000;
            c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[V1][0]);
            c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            c.g[V0][0] = c.g[V0][0] | 0x30;
            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
            let t = c.g[V1][0] == 0u64;
            c.g[V1][0] = 0xfffffffffffff07f;
            if t {
                break 'b_38d278;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = true;
                c.g[V0][0] = c.g[V0][0] | 0x880;
                if t {
                    break 'b_38d288;
                } else {
                    break 'b_38d278;
                }
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x100;
        break 'b_38d288;
    }
    'b_38d424: {
        c.cycles += 70;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[T3][0] as u32) << 1);
        c.g[A2][0] = 0x78000000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[T8][0] = c.lw((c.g[T6][0] as u32));
        c.mult(0, 3, 3, 3);
        c.g[T7][0] = 0x30000000;
        c.g[A2][0] = c.g[A2][0] | 0x8040;
        c.g[A3][0] = c.g[A3][0] << 32;
        c.g[A0][0] = c.g[T8][0] | c.g[T7][0];
        c.g[T1][0] = 0x10000000;
        c.g[T1][0] = c.g[T1][0] | 0x1;
        c.g[T2][0] = 0x40400000;
        c.g[T2][0] = c.g[T2][0] | 0x4040;
        c.g[T2][0] = c.g[T2][0] << 16;
        c.g[T2][0] = c.g[T2][0] | 0x2000;
        c.g[T2][0] = c.g[T2][0] << 16;
        c.g[T3][0] = 0x10000000;
        c.g[T3][0] = c.g[T3][0] | 0x6;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[A1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
        c.g[T0][0] = 0x20000000;
        c.g[A3][0] = c.g[V0][0] | c.g[A3][0];
        c.g[V1][0] = 0x6c040000;
        c.g[V1][0] = c.g[V1][0] | 0x8080;
        c.g[V1][0] = c.g[V1][0] << 32;
        c.sd((c.g[A0][0] as u32), c.g[T1][0]);
        c.g[A2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x360));
        c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T2][0]);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
        c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x20), c.g[A3][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x30), c.g[T3][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x40), c.g[T0][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x28), 0u64);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
        c.lqc2(1, (c.g[A2][0] as u32));
        c.lqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.sqc2(1, (c.g[V0][0] as u32));
        c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = 0x14000000;
        c.sd((c.g[A0][0] as u32).wrapping_add(0x90), 0u64);
        c.g[A1][0] = 0xfff0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] << 32;
        c.sd((c.g[A0][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
        c.g[T1][0] = c.g[A0][0] & c.g[A1][0];
        c.sw((c.g[T6][0] as u32), c.g[T1][0]);
        c.g[A3][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x18f0));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[T0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        if t {
            break 'b_38d424;
        } else {
            c.cycles += 33;
            c.g[A1][0] = 0x60000;
            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[A1][0] | 0x8000;
            c.g[V1][0] = 0xffffffffffffffe0;
            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
            c.g[A2][0] = c.g[A2][0] | c.g[T7][0];
            c.g[V0][0] = c.g[V0][0] & 0x1f;
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[A3][0] as u32));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[T0][0] as u32));
            c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
            c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.sdr(3, (c.g[V0][0] as u32));
            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[V1][0] = 0x1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T8][0]);
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            break 'b_38d424;
        }
    }
    c.cycles += 9;
    c.g[V0][0] = c.lw((c.g[T6][0] as u32));
    c.lq(16, (c.g[SP][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sw((c.g[T6][0] as u32), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
    c.sw((c.g[T4][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `int TexMan_SwizzleT4Page(TexMan* self, int w, int h, uchar* src, uchar* dst)`
/// (0x0038f9c0): swizzles one 4-bit texture page: builds the block x/y maps from
/// the id table at 0x44c060, then for each 32x16 block copies the 16 source rows
/// into a block buffer, runs `TexMan_SwizzleT4Block` and copies the 8 output rows to
/// the page. Returns 0.
pub fn tex_man_swizzle_t4_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3d0));
        c.g[T1][0] = 0u64;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x310), c.g[V0][0]);
        c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x370));
        c.g[V0][0] = 0x450000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x308), c.g[T0][0]);
        c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3fa0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x3c0));
        c.g[T0][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x380));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x360));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x350));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x340));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x330), c.g[RA][0]);
        c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x300), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x304), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        'l_38fa30: loop {
            c.cycles += 4;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T5][0] as u32)));
            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            'l_38fa40: loop {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A3][0] as i64) < 8i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038fa40);
                    }
                    continue 'l_38fa40;
                } else {
                    c.cycles += 4;
                    c.g[T0][0] = c.g[S2][0];
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0038fa30);
                        }
                        continue 'l_38fa30;
                    } else {
                        'b_38fc10: {
                            c.cycles += 17;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            if c.g[V1][0] != 0 {
                                c.g[T2][0] = c.g[A2][0];
                            }
                            if c.g[V0][0] != 0 {
                                c.g[T3][0] = c.g[A1][0];
                            }
                            c.g[FP][0] = sx(((c.g[T2][0] as u32) as i32 >> 4) as u32);
                            c.g[S4][0] = sx(((c.g[T3][0] as u32) as i32 >> 5) as u32);
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038faa8);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00416210, 0x0038faa8)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038fab8);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00416210, 0x0038fab8)?;
                            let t = (c.g[FP][0] as i64) <= 0;
                            c.g[T0][0] = 0u64;
                            if t {
                                break 'b_38fc10;
                            } else {
                                c.cycles += 2;
                                c.g[A3][0] = 0u64;
                                'l_38fac8: loop {
                                    'b_38fc00: {
                                        c.cycles += 2;
                                        let t = (c.g[S4][0] as i64) <= 0;
                                        c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_38fc00;
                                        } else {
                                            c.cycles += 10;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x304));
                                            c.mult(0, 23, 8, 20);
                                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                            c.g[A0][0] = 0x450000;
                                            c.lo[0] = c.g[V1][0];
                                            c.g[S1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                                            c.g[V1][0] = 0x40;
                                            c.g[T0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x4020));
                                            c.madd(0, 22, 2, 3);
                                            c.g[V0][0] = sx((c.g[S7][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            'l_38faf8: loop {
                                                c.cycles += 8;
                                                c.g[V1][0] = sx((c.g[A3][0] as u32) << 4);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[T0][0] as u32)));
                                                c.g[S3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                                c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                                                c.g[A2][0] = 0xf;
                                                c.g[V0][0] = c.g[S5][0];
                                                'l_38fb18: loop {
                                                    c.cycles += 12;
                                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, (c.g[A0][0] as u32));
                                                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, (c.g[V0][0] as u32));
                                                    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0xf));
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    let t = (c.g[A2][0] as i64) >= 0;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0038fb18);
                                                        }
                                                        continue 'l_38fb18;
                                                    } else {
                                                        c.cycles += 22;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x300));
                                                        c.g[A1][0] = c.g[S5][0];
                                                        c.g[A2][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x0038fb5c);
                                                        c.sq(
                                                            8,
                                                            (c.g[SP][0] as u32).wrapping_add(0x320),
                                                        );
                                                        Callee {
                                                            addr: 0x0038f820,
                                                            f: ee::s12::TexMan_SwizzleT4Block,
                                                        }
                                                        .call(c, 0x0038fb5c)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x310));
                                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                        c.lq(
                                                            8,
                                                            (c.g[SP][0] as u32).wrapping_add(0x320),
                                                        );
                                                        c.g[A3][0] = c.g[S1][0];
                                                        c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[A2][0] = 0x7;
                                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                        c.g[A1][0] = sx((c.g[SP][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x308));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                        c.lo[0] = c.g[A0][0];
                                                        c.g[A0][0] = 0x100;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                                        c.madd(0, 2, 2, 4);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        'l_38fba0: loop {
                                                            c.cycles += 20;
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, (c.g[A3][0] as u32));
                                                            c.ldl(
                                                                9,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                9,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                3,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(5, (c.g[V0][0] as u32));
                                                            c.sdl(
                                                                9,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                9,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.sdr(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = (c.g[A2][0] as i64) >= 0;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x100));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x0038fba0,
                                                                    );
                                                                }
                                                                continue 'l_38fba0;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A3][0] = c.g[S3][0];
                                                                c.g[V0][0] = ((c.g[A3][0] as i64)
                                                                    < (c.g[S4][0] as i64))
                                                                    as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = sx((c.g[S7][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A3][0] as u32),
                                                                    ));
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0038faf8,
                                                                        );
                                                                    }
                                                                    continue 'l_38faf8;
                                                                } else {
                                                                    break 'b_38fc00;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[T0][0] = c.g[S2][0];
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A3][0] = 0u64;
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0038fac8);
                                        }
                                        continue 'l_38fac8;
                                    } else {
                                        break 'b_38fc10;
                                    }
                                }
                            }
                        }
                        c.cycles += 13;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                        c.g[V0][0] = 0u64;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x390));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x380));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x370));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x360));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x350));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x340));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x330));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3d0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `int TexMan_SwizzleT8Page(TexMan* self, int w, int h, uchar* src, uchar* dst)`
/// (0x0038fc48): the 8-bit twin of `TexMan_SwizzleT4Page` (16x16 blocks, source
/// pitch 0x80, id table 0x44c160, `TexMan_SwizzleT8Block`). Returns 0.
pub fn tex_man_swizzle_t8_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3d0));
        c.g[T1][0] = 0u64;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x310), c.g[V0][0]);
        c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x370));
        c.g[V0][0] = 0x450000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x308), c.g[T0][0]);
        c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3ea0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x3c0));
        c.g[T0][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.g[T3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x380));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x360));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x350));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x340));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x330), c.g[RA][0]);
        c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x300), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x304), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
        'l_38fcb8: loop {
            c.cycles += 4;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T5][0] as u32)));
            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            'l_38fcc8: loop {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A3][0] as i64) < 8i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038fcc8);
                    }
                    continue 'l_38fcc8;
                } else {
                    c.cycles += 4;
                    c.g[T0][0] = c.g[S2][0];
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0038fcb8);
                        }
                        continue 'l_38fcb8;
                    } else {
                        'b_38fe98: {
                            c.cycles += 17;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            if c.g[V1][0] != 0 {
                                c.g[T3][0] = c.g[A2][0];
                            }
                            if c.g[V0][0] != 0 {
                                c.g[T2][0] = c.g[A1][0];
                            }
                            c.g[FP][0] = sx(((c.g[T3][0] as u32) as i32 >> 4) as u32);
                            c.g[S4][0] = sx(((c.g[T2][0] as u32) as i32 >> 4) as u32);
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038fd30);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00416210, 0x0038fd30)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038fd40);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00416210, 0x0038fd40)?;
                            let t = (c.g[FP][0] as i64) <= 0;
                            c.g[T0][0] = 0u64;
                            if t {
                                break 'b_38fe98;
                            } else {
                                c.cycles += 2;
                                c.g[A3][0] = 0u64;
                                'l_38fd50: loop {
                                    'b_38fe88: {
                                        c.cycles += 2;
                                        let t = (c.g[S4][0] as i64) <= 0;
                                        c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_38fe88;
                                        } else {
                                            c.cycles += 10;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x304));
                                            c.mult(0, 23, 8, 20);
                                            c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
                                            c.g[A0][0] = 0x450000;
                                            c.lo[0] = c.g[V1][0];
                                            c.g[S1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                                            c.g[V1][0] = 0x80;
                                            c.g[T0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x3f20));
                                            c.madd(0, 22, 2, 3);
                                            c.g[V0][0] = sx((c.g[S7][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            'l_38fd80: loop {
                                                c.cycles += 8;
                                                c.g[V1][0] = sx((c.g[A3][0] as u32) << 4);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[T0][0] as u32)));
                                                c.g[S3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                                c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                                                c.g[A2][0] = 0xf;
                                                c.g[V0][0] = c.g[S5][0];
                                                'l_38fda0: loop {
                                                    c.cycles += 12;
                                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, (c.g[A0][0] as u32));
                                                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                    c.ldr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, (c.g[V0][0] as u32));
                                                    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0xf));
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    let t = (c.g[A2][0] as i64) >= 0;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x80));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0038fda0);
                                                        }
                                                        continue 'l_38fda0;
                                                    } else {
                                                        c.cycles += 22;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x300));
                                                        c.g[A1][0] = c.g[S5][0];
                                                        c.g[A2][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x0038fde4);
                                                        c.sq(
                                                            8,
                                                            (c.g[SP][0] as u32).wrapping_add(0x320),
                                                        );
                                                        Callee {
                                                            addr: 0x0038f930,
                                                            f: ee::s12::TexMan_SwizzleT8Block,
                                                        }
                                                        .call(c, 0x0038fde4)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x310));
                                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                        c.lq(
                                                            8,
                                                            (c.g[SP][0] as u32).wrapping_add(0x320),
                                                        );
                                                        c.g[A3][0] = c.g[S1][0];
                                                        c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[A2][0] = 0x7;
                                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                        c.g[A1][0] = sx((c.g[SP][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x308));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                        c.lo[0] = c.g[A0][0];
                                                        c.g[A0][0] = 0x100;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                                        c.madd(0, 2, 2, 4);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        'l_38fe28: loop {
                                                            c.cycles += 20;
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, (c.g[A3][0] as u32));
                                                            c.ldl(
                                                                9,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                9,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                3,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(5, (c.g[V0][0] as u32));
                                                            c.sdl(
                                                                9,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                9,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.sdr(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = (c.g[A2][0] as i64) >= 0;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x100));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x0038fe28,
                                                                    );
                                                                }
                                                                continue 'l_38fe28;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A3][0] = c.g[S3][0];
                                                                c.g[V0][0] = ((c.g[A3][0] as i64)
                                                                    < (c.g[S4][0] as i64))
                                                                    as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = sx((c.g[S7][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A3][0] as u32),
                                                                    ));
                                                                if t {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x0038fd80,
                                                                        );
                                                                    }
                                                                    continue 'l_38fd80;
                                                                } else {
                                                                    break 'b_38fe88;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[T0][0] = c.g[S2][0];
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A3][0] = 0u64;
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0038fd50);
                                        }
                                        continue 'l_38fd50;
                                    } else {
                                        break 'b_38fe98;
                                    }
                                }
                            }
                        }
                        c.cycles += 13;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                        c.g[V0][0] = 0u64;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x390));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x380));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x370));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x360));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x350));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x340));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x330));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3d0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// `int TexMan_SwizzleT4(TexMan* self, int w, int h, uchar* src, uchar* dst)`
/// (0x0038fed0): swizzles a whole 4-bit texture: checks that `w` and `h` are
/// powers of two from 1024 down (else returns -1), clears two 8 KiB buffers, then for
/// every 128x128 tile copies its rows out of `src`, runs `TexMan_SwizzleT4Page` and
/// copies the result rows into `dst`. Returns 0.
pub fn tex_man_swizzle_t4(c: &mut Cpu) {
    guest(c, |c| {
        'b_38ff3c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40d0));
            c.g[V0][0] = 0x400;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40b0));
            c.g[S0][0] = c.g[A1][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4000), c.g[A0][0]);
            c.g[S1][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40a0));
            c.g[A0][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x4090));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x4080));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x4070));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x4060));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x4050));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x4040));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x4030), c.g[RA][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4004), c.g[A3][0]);
            let t = c.g[S0][0] == c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4008), c.g[T0][0]);
            if t {
                break 'b_38ff3c;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0x400;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                'l_38ff28: loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 11i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] =
                        sx(((c.g[V1][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
                    if t {
                        break 'b_38ff3c;
                    } else {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x0038ff28);
                            }
                            continue 'l_38ff28;
                        } else {
                            break 'b_38ff3c;
                        }
                    }
                }
            }
        }
        'b_390168: {
            'b_38ffa0: {
                'b_38ff98: {
                    c.cycles += 3;
                    c.g[V0][0] = 0xb;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_38ff98;
                    } else {
                        'b_38ff8c: {
                            c.cycles += 10;
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038ff54);
                            c.g[A2][0] = 0x2000;
                            call_ptr(c, 0x00416210, 0x0038ff54)?;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2000));
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0038ff64);
                            c.g[A2][0] = 0x2000;
                            call_ptr(c, 0x00416210, 0x0038ff64)?;
                            c.g[V0][0] = 0x400;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_38ff8c;
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = 0x400;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                'l_38ff78: loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A0][0] as i64) < 11i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32
                                        >> ((c.g[A0][0] as u32) & 31))
                                        as u32);
                                    if t {
                                        break 'b_38ff8c;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S1][0] != c.g[V0][0] {
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0038ff78);
                                            }
                                            continue 'l_38ff78;
                                        } else {
                                            break 'b_38ff8c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0xb;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[T0][0] = 0xffffffffffffffff;
                        if t {
                            break 'b_38ffa0;
                        } else {
                            break 'b_38ff98;
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_390168;
                } else {
                    break 'b_38ffa0;
                }
            }
            'b_390008: {
                'b_38fffc: {
                    c.cycles += 14;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7e));
                    c.g[A0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7e));
                    c.g[A1][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[A2][0];
                    }
                    if c.g[A1][0] != 0 {
                        c.g[V1][0] = c.g[A3][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = c.g[V1][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x400c), c.g[V0][0]);
                    if t {
                        break 'b_38fffc;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                        c.g[V1][0] = sx((c.g[S0][0] as u32) >> 31);
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = c.g[S0][0];
                        }
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 2) as u32);
                        c.g[S7][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                        let t = true;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4014), c.g[A0][0]);
                        if t {
                            break 'b_390008;
                        } else {
                            break 'b_38fffc;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x20;
                c.g[S7][0] = 0x40;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4014), c.g[V0][0]);
                break 'b_390008;
            }
            'b_39002c: {
                'b_390024: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x80;
                    if t {
                        break 'b_390024;
                    } else {
                        c.cycles += 3;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4010), c.g[S1][0]);
                        let t = true;
                        c.g[S5][0] = sx((c.g[S1][0] as u32) << 1);
                        if t {
                            break 'b_39002c;
                        } else {
                            break 'b_390024;
                        }
                    }
                }
                c.cycles += 2;
                c.g[S5][0] = 0x100;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4010), c.g[A1][0]);
                break 'b_39002c;
            }
            'b_390164: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_390164;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = sx((c.g[FP][0] as u32) << 7);
                    c.g[V1][0] = sx((c.g[FP][0] as u32) << 5);
                    c.mult(0, 5, 23, 2);
                    c.mult(0, 0, 21, 3);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4018), c.g[A1][0]);
                    c.g[V0][0] = c.lo[0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4020), c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    'l_390058: loop {
                        'b_390150: {
                            c.cycles += 3;
                            c.g[S3][0] = 0u64;
                            let t = (c.g[FP][0] as i64) <= 0;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4028), c.g[V1][0]);
                            if t {
                                break 'b_390150;
                            } else {
                                c.cycles += 15;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4008));
                                c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2000));
                                c.lo[0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4020));
                                c.madd(0, 0, 2, 4);
                                c.g[V0][0] = c.lo[0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4024), c.g[V0][0]);
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4004));
                                c.lo[0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4018));
                                c.madd(0, 0, 2, 4);
                                c.g[V0][0] = c.lo[0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x401c), c.g[V0][0]);
                                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                'l_3900a0: loop {
                                    'b_3900e8: {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x401c));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x402c), c.g[V1][0]);
                                        c.g[S2][0] = c.g[SP][0];
                                        c.lo[0] = c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4010));
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.madd(0, 17, 23, 19);
                                        if t {
                                            break 'b_3900e8;
                                        } else {
                                            c.cycles += 3;
                                            c.mult(0, 20, 23, 30);
                                            c.g[S0][0] = c.g[V1][0];
                                            'l_3900c8: loop {
                                                c.cycles += 8;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A2][0] = c.g[S7][0];
                                                c.g[RA][0] = sx(0x003900dc);
                                                c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S4][0] as u32)));
                                                call_ptr(c, 0x0041605c, 0x003900dc)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003900c8);
                                                    }
                                                    continue 'l_3900c8;
                                                } else {
                                                    break 'b_3900e8;
                                                }
                                            }
                                        }
                                    }
                                    'b_390140: {
                                        c.cycles += 12;
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4000));
                                        c.g[A1][0] = 0x80;
                                        c.g[A2][0] = 0x80;
                                        c.g[A3][0] = c.g[SP][0];
                                        c.g[T0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x00390104);
                                        c.g[S2][0] = c.g[S6][0];
                                        Callee {
                                            addr: 0x0038f9c0,
                                            f: ee::s12::TexMan_SwizzleT4Page,
                                        }
                                        .call(c, 0x00390104)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4024));
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4014));
                                        c.lo[0] = c.g[V0][0];
                                        let t = (c.g[V1][0] as i64) <= 0;
                                        c.madd(0, 17, 21, 19);
                                        if t {
                                            break 'b_390140;
                                        } else {
                                            c.cycles += 2;
                                            c.mult(0, 19, 21, 30);
                                            c.g[S0][0] = c.g[V1][0];
                                            'l_390120: loop {
                                                c.cycles += 8;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] = c.g[S5][0];
                                                c.g[RA][0] = sx(0x00390134);
                                                c.g[S1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[S3][0] as u32)));
                                                call_ptr(c, 0x0041605c, 0x00390134)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x100));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00390120);
                                                    }
                                                    continue 'l_390120;
                                                } else {
                                                    break 'b_390140;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x402c));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[FP][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003900a0);
                                        }
                                        continue 'l_3900a0;
                                    } else {
                                        break 'b_390150;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4028));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x400c));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00390058);
                            }
                            continue 'l_390058;
                        } else {
                            break 'b_390164;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_390168;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40c0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40b0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40a0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x4090));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x4080));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x4070));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x4060));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x4050));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x4040));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x4030));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40d0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, fakes, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes, their magic, and the forced
    /// results (`+4`: scratchpad upload result, `+8`: patch bounds result).
    const OC1065_CTRL: u32 = DATA + 0x20_0000;
    const OC1065_MAGIC: u32 = 0x1065_ab01;
    /// Bezier manager (`self`), graphics manager (`gp+0x2a90`), draw state stack
    /// (`gfx+0xe84`), ring (`gfx+0x18f0`), patch (`sPatch`), its copy, the
    /// handle word, the packet buffer and the program table.
    const OC1065_SELF: u32 = DATA + 0x100;
    const OC1065_GFX: u32 = DATA + 0x2000;
    const OC1065_DSTATE: u32 = DATA + 0x3000;
    const OC1065_PATCH: u32 = DATA + 0x400;
    const OC1065_HANDLE: u32 = DATA + 0x500;
    const OC1065_EXT: u32 = DATA + 0x520;
    const OC1065_COPY: u32 = DATA + 0x8000;
    const OC1065_RING: u32 = DATA + 0x10_000;
    const OC1065_CACHE: u32 = DATA + 0x2_0000;
    const OC1065_PROGS: u32 = DATA + 0x3_0000;
    const OC1065_PACKETS: u32 = DATA + 0x4_0000;

    /// The callee fake: logs like `fakes::fake_call`, then forces the result
    /// the function acts on.
    fn oc1065_fake_call(c: &mut Cpu, addr: u32) {
        let up = c.bus.read32(OC1065_CTRL + 4);
        let bounds = c.bus.read32(OC1065_CTRL + 8);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0038_f708 => c.g[V0][0] = sx(up),
            0x0038_ca08 => c.g[V0][0] = sx(bounds),
            _ => {}
        }
    }

    macro_rules! oc1065_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1065_CTRL) != OC1065_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1065_fake_call(c, $addr);
                }
            )*
            fn oc1065_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1065_fakes!(
        (
            oc1065_fake_upload,
            OC1065_PREV_UPLOAD,
            ee::s12::OVERRIDE_0038F708,
            ee::s12::f_0038f708__generated,
            0x0038_f708
        ),
        (
            oc1065_fake_bounds,
            OC1065_PREV_BOUNDS,
            ee::s12::OVERRIDE_0038CA08,
            ee::s12::f_0038ca08__generated,
            0x0038_ca08
        ),
        (
            oc1065_fake_projector,
            OC1065_PREV_PROJECTOR,
            ee::s12::OVERRIDE_0038D448,
            ee::s12::f_0038d448__generated,
            0x0038_d448
        ),
    );

    /// The structures both bezier emitters read: the graphics manager with its
    /// draw state stack and ring, the patch and its copy (flags chosen by
    /// `seed`), the handle word and the program table.
    fn oc1065_bezier_world(c: &mut Cpu, r: &mut difftest::Rng, seed: u32) {
        difftest::elf_image(c);
        let gp = difftest::GAME_GP;
        difftest::fill(c, r, OC1065_SELF, 0x180);
        difftest::fill(c, r, OC1065_GFX, 0x80);
        difftest::fill(c, r, OC1065_DSTATE, 0x20);
        difftest::fill(c, r, OC1065_PATCH, 0x10);
        difftest::fill(c, r, OC1065_COPY, 0x80);
        difftest::fill(c, r, OC1065_EXT, 4);
        c.bus.write32(gp + 0x2a90, OC1065_GFX);
        c.bus.write32(OC1065_GFX + 0xe84, OC1065_DSTATE);
        c.bus.write32(OC1065_GFX + 0x18f0, OC1065_RING);
        c.bus.write32(OC1065_GFX + 0x10a8, r.next_u32() & 0x7fff);
        // The ring: a count below or at the 2600-entry limit, and its two keys.
        let count = if seed % 5 == 0 {
            2600 + r.next_u32() % 50
        } else {
            r.next_u32() % 2600
        };
        c.bus.write32(OC1065_RING, count);
        c.bus.write32(OC1065_RING + 0x69cc0, r.next_u32());
        c.bus.write32(OC1065_RING + 0x69cc4, r.next_u32());
        // gp flags: projector / reflection kill switches.
        c.bus.write32(gp + 0x13e8, (seed / 3 % 2) * r.next_u32());
        c.bus.write32(gp + 0x13ec, (seed / 7 % 2) * r.next_u32());
        // The patch copy: the 0x180000 mode bits and the reflection bits.
        let mode = [0u32, 0x80000, 0x100000, 0x180000][(seed / 2 % 4) as usize];
        let refl = [0u32, 0x200000, 0x400000, 0x600000][(seed / 5 % 4) as usize];
        c.bus
            .write32(OC1065_COPY + 0xc, mode | refl | (r.next_u32() & 0x7fff));
        // The patch: object, program index.
        c.bus.write32(OC1065_PATCH, OC1065_COPY);
        c.bus.write32(OC1065_PATCH + 4, r.next_u32() % 8);
        c.bus.write32(OC1065_PATCH + 8, r.next_u32() % 16);
        // The manager: pointers the pass adds offsets to.
        c.bus.write32(OC1065_SELF + 0x30, OC1065_CACHE);
        c.bus.write32(OC1065_SELF + 0x454, OC1065_PROGS);
        // The handle word: the next packet position (28-bit, 16-byte aligned).
        c.bus.write32(
            OC1065_HANDLE,
            (OC1065_PACKETS + (r.next_u32() % 0x80) * 16) & 0x0fff_ffff,
        );
    }

    /// `PS2BezierMan_EmitPatchType2` against the generated body: all four
    /// 0x180000 mode cases, with and without the `ext` tag and the reflection
    /// and projector passes, ring full or not, callees unwinding.
    #[test]
    fn ps2_bezier_man_emit_patch_type2_matches() {
        oc1065_install_fakes();
        fakes::covered(
            ee::s12::f_0038ce20__generated,
            ps2_bezier_man_emit_patch_type2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1065_0001, 3);
                oc1065_bezier_world(c, &mut r, seed);
                c.bus.write32(OC1065_CTRL, OC1065_MAGIC);
                c.bus.write32(OC1065_CTRL + 4, OC1065_COPY);
                c.bus
                    .write32(OC1065_CTRL + 8, [0, 1, 7][(seed / 3 % 3) as usize]);
                c.g[A0][0] = sx(OC1065_SELF);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(OC1065_PATCH);
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { sx(OC1065_EXT) };
                c.g[T0][0] = sx(OC1065_HANDLE);
            },
        );
    }

    /// `PS2BezierMan_EmitPatchMatrixPass` against the generated body: patch
    /// types 0, 1, 2 and the default (3, -1, 5), both bit-group cases, negative
    /// and positive texture ids, ring entries recorded or skipped.
    #[test]
    fn ps2_bezier_man_emit_patch_matrix_pass_matches() {
        difftest::check_lohi(
            ee::s12::f_0038d168__generated,
            ps2_bezier_man_emit_patch_matrix_pass,
            240,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1065_0002, 0);
                oc1065_bezier_world(c, &mut r, seed);
                let ty = [0u32, 1, 2, 3, u32::MAX, 5][(seed % 6) as usize];
                c.g[A0][0] = sx(OC1065_SELF);
                c.g[A1][0] = sx(OC1065_CACHE);
                c.g[A2][0] = sx(OC1065_COPY);
                c.g[A3][0] = sx(ty);
                c.g[T0][0] = sx(OC1065_PROGS);
                c.g[T1][0] = sx(OC1065_PATCH);
                c.g[T2][0] = sx(OC1065_HANDLE);
            },
        );
    }

    /// Source, destination and `self` of the swizzle tests.
    const OC1065_SRC: u32 = DATA + 0x1000;
    const OC1065_DST: u32 = DATA + 0x8000;
    const OC1065_TEX: u32 = DATA + 0x100;

    /// Common page-swizzle fixture: random source and destination pages, the
    /// game image (id tables), width and height from the case list.
    fn oc1065_page_setup(c: &mut Cpu, seed: u32, salt: u64, cases: &[(i32, i32)]) {
        let mut r = fakes::machine(c, seed, salt, 2);
        difftest::elf_image(c);
        difftest::fill(c, &mut r, OC1065_SRC, 0x1400);
        difftest::fill(c, &mut r, OC1065_DST, 0x2000);
        let (w, h) = cases[seed as usize % cases.len()];
        c.g[A0][0] = sx(OC1065_TEX);
        c.g[A1][0] = sx(w as u32);
        c.g[A2][0] = sx(h as u32);
        c.g[A3][0] = sx(OC1065_SRC);
        c.g[T0][0] = sx(OC1065_DST);
    }

    /// `TexMan_SwizzleT4Page` against the generated body: full 128x128 pages,
    /// smaller and larger ones, empty and negative sizes (the clamps), the two
    /// memsets unwinding.
    #[test]
    fn tex_man_swizzle_t4_page_matches() {
        const CASES: [(i32, i32); 9] = [
            (0x80, 0x80),
            (0x80, 0x80),
            (0x40, 0x20),
            (0x20, 0x10),
            (0, 0),
            (-1, -1),
            (-20, 0x40),
            (0x100, 0x40),
            (0x80, 0x40),
        ];
        crate::difftest::check(
            ee::s12::f_0038f9c0__generated,
            tex_man_swizzle_t4_page,
            120,
            &|c, seed| oc1065_page_setup(c, seed, 0x1065_0003, &CASES),
        );
    }

    /// `TexMan_SwizzleT8Page` against the generated body (the 8-bit twin).
    #[test]
    fn tex_man_swizzle_t8_page_matches() {
        const CASES: [(i32, i32); 9] = [
            (0x80, 0x80),
            (0x80, 0x80),
            (0x40, 0x40),
            (0x20, 0x10),
            (0, 0),
            (-1, -1),
            (-20, 0x40),
            (0x80, 0x40),
            (0x40, 0x80),
        ];
        crate::difftest::check(
            ee::s12::f_0038fc48__generated,
            tex_man_swizzle_t8_page,
            120,
            &|c, seed| oc1065_page_setup(c, seed, 0x1065_0004, &CASES),
        );
    }

    /// `TexMan_SwizzleT4` against the generated body: single-tile and
    /// multi-tile textures (nx/ny 1 and more), a bad width and a bad height
    /// (return -1), the memsets, row copies and the page call unwinding.
    #[test]
    fn tex_man_swizzle_t4_matches() {
        const CASES: [(i32, i32); 14] = [
            (0x80, 0x80),
            (0x100, 0x80),
            (0x40, 0x40),
            (0x80, 0x40),
            (0x100, 0x100),
            (0x200, 0x100),
            (0x20, 0x20),
            (0x77, 0x80),
            (0x80, 0x55),
            (0x400, 0x80),
            (0x80, 0x200),
            (1, 1),
            (2, 1),
            (0x100, 0x200),
        ];
        crate::difftest::check(
            ee::s12::f_0038fed0__generated,
            tex_man_swizzle_t4,
            140,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1065_0005, 12);
                difftest::elf_image(c);
                // The tile and output buffers live on the stack, below `sp`.
                let sp = c.g[SP][0] as u32;
                difftest::fill(c, &mut r, sp - 0x40d0, 0x1000);
                difftest::fill(c, &mut r, OC1065_DST, 0x400);
                let (w, h) = CASES[seed as usize % CASES.len()];
                c.g[A0][0] = sx(OC1065_TEX);
                c.g[A1][0] = sx(w as u32);
                c.g[A2][0] = sx(h as u32);
                c.g[A3][0] = sx(OC1065_SRC);
                c.g[T0][0] = sx(OC1065_DST);
            },
        );
    }
}
