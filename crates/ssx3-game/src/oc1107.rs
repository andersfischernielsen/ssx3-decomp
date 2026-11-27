//! Staging module oc1107 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1107`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0040_4050,
    lib_mpeg_form_predictions,
    "libmpeg_FormPredictions",
)];

/// `libmpeg_SetupPredBlock` (`0x0040_4758`).
pub(crate) const FP_SETUP_PRED_BLOCK: Callee = Callee {
    addr: 0x0040_4758,
    f: ee::s15::f_00404758,
};
/// `libmpeg_DualPrimeArithmetic` (`0x0040_5e30`).
pub(crate) const FP_DUAL_PRIME: Callee = Callee {
    addr: 0x0040_5e30,
    f: ee::s15::f_00405e30,
};
/// `libmpeg_Errorf` (`0x0040_3470`).
pub(crate) const FP_ERRORF: Callee = Callee {
    addr: 0x0040_3470,
    f: ee::s15::f_00403470,
};

/// `libmpeg_FormPredictions(dec, bx, by, mbtype, mtype, PMV, mvfs, dmv)`
/// (0x404050): MPEG-2 motion compensation of one macroblock. `dec` (a0) is the
/// decoder state, `bx`/`by` (a1/a2) the block position, `mbtype` (a3) the
/// macroblock type flags (bit 3 forward, bit 2 backward), `mtype` (t0) the
/// motion type (1 field, 2 frame, 3 dual prime), `PMV` (t1) the motion
/// vectors, `mvfs` (t2) the field selects, `dmv` (t3) the dual-prime deltas.
/// Clears the per-buffer flag at `dec + dec[0x810] * 0x140 + 0x6bc`, then
/// makes one or more `libmpeg_SetupPredBlock` calls for the forward and the
/// backward prediction (`Errorf` on an unknown motion type), averaging the
/// second one in.
pub fn lib_mpeg_form_predictions(c: &mut Cpu) {
    guest(c, |c| {
        'b_404538: {
            'b_4040cc: {
                c.cycles += 27;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.g[V1][0] = 0x140;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[FP][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S6][0]);
                c.g[FP][0] = c.g[T2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
                c.g[S6][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S4][0]);
                c.g[S5][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S3][0]);
                c.g[S4][0] = c.g[T0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S2][0]);
                c.g[S3][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                c.g[S2][0] = c.g[T1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.g[S0][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[S7][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x810));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                c.mult(0, 4, 2, 3);
                c.g[T4][0] = c.g[A3][0] & 0x8;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                let t = c.g[T4][0] != 0u64;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6bc), 0u64);
                if t {
                    break 'b_4040cc;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_404538;
                    } else {
                        break 'b_4040cc;
                    }
                }
            }
            'b_404534: {
                'b_404520: {
                    'b_404510: {
                        'b_4042e4: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                            c.g[V0][0] = 0x3;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = c.g[V1][0] ^ 0x2;
                            if t {
                                break 'b_4042e4;
                            } else {
                                'b_404128: {
                                    'b_4040f4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        if c.g[S4][0] == c.g[V0][0] {
                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                            break 'b_4040f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[T4][0] != 0u64;

                                            if t {
                                                break 'b_404128;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                break 'b_4040f4;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = 0u64;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                    c.g[A3][0] = 0u64;
                                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                    c.g[T0][0] = 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.g[T1][0] = 0x10;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                    c.g[T2][0] = c.g[S5][0];
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                    break 'b_404510;
                                }
                                'b_4041b4: {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] != c.g[S3][0];
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_4041b4;
                                    } else {
                                        c.cycles += 33;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A2][0] = c.lw((c.g[FP][0] as u32));
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        FP_SETUP_PRED_BLOCK.call(c, 0x0040416c)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A3][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        FP_SETUP_PRED_BLOCK.call(c, 0x004041ac)?;
                                        c.g[S0][0] = 0x1;
                                        break 'b_404534;
                                    }
                                }
                                'b_4042cc: {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] != c.g[V1][0];
                                    c.g[A1][0] = 0x490000;
                                    if t {
                                        break 'b_4042cc;
                                    } else {
                                        c.cycles += 68;
                                        c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A2][0] = c.g[T3][0];
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                        FP_DUAL_PRIME.call(c, 0x004041d8)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A2][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[A3][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404218)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                        c.g[A2][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[A3][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404254)?;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A2][0] = 0x1;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[A3][0] = 0x1;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.g[T3][0] = c.g[S6][0];
                                        FP_SETUP_PRED_BLOCK.call(c, 0x00404294)?;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                                        c.g[A2][0] = 0u64;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b8));
                                        c.g[A3][0] = 0x1;
                                        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                        c.g[T0][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T1][0] = 0x8;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                        break 'b_404510;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e38));
                                c.g[A2][0] = c.g[S4][0];
                                FP_ERRORF.call(c, 0x004042dc)?;
                                c.g[S0][0] = 0x1;
                                break 'b_404534;
                            }
                        }
                        'b_404334: {
                            'b_404330: {
                                c.cycles += 13;
                                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c8));
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
                                c.g[S7][0] = (c.g[V0][0] < 0x1u64) as u64;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                c.g[A2][0] = 0x2;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                c.g[S3][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[A1][0]);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                                let t = c.g[V0][0] != c.g[A2][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                if t {
                                    break 'b_404330;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_404334;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[FP][0] as u32));
                                        c.g[V0][0] = c.g[S7][0] ^ c.g[V0][0];
                                        c.g[S3][0] = (0u64 < c.g[V0][0]) as u64;
                                        break 'b_404330;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_404334;
                        }
                        'b_404504: {
                            'b_404384: {
                                'b_404348: {
                                    c.cycles += 2;
                                    if c.g[S4][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[FP][0] as u32));
                                        break 'b_404348;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[T4][0] != 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_404384;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = c.lw((c.g[FP][0] as u32));
                                            break 'b_404348;
                                        }
                                    }
                                }
                                c.cycles += 15;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.sw((c.g[SP][0] as u32), c.g[A2][0]);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                break 'b_404504;
                            }
                            'b_404454: {
                                c.cycles += 2;
                                let t = c.g[S4][0] != c.g[V0][0];
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_404454;
                                } else {
                                    'b_40440c: {
                                        'b_404408: {
                                            c.cycles += 24;
                                            c.g[V0][0] = c.lw((c.g[FP][0] as u32));
                                            c.g[A1][0] = sx((c.g[S3][0] as u32) << 3);
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                            c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A3][0] = 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                            c.g[T0][0] = 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                            c.g[T1][0] = 0x8;
                                            c.g[T2][0] = c.g[S5][0];
                                            c.g[T3][0] = c.g[S6][0];
                                            FP_SETUP_PRED_BLOCK.call(c, 0x004043e0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                            let t = c.g[V0][0] != c.g[S4][0];
                                            c.g[S3][0] = 0u64;
                                            if t {
                                                break 'b_404408;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V1][0] =
                                                    c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_40440c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[S3][0] = 0x1;
                                                    c.g[V0][0] = c.g[S7][0] ^ c.g[V1][0];
                                                    if c.g[V0][0] == 0 {
                                                        c.g[S3][0] = 0u64;
                                                    }
                                                    break 'b_40440c;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                                        break 'b_40440c;
                                    }
                                    c.cycles += 18;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A3][0] = 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[T0][0] = 0x8;
                                    c.sw((c.g[SP][0] as u32), c.g[A2][0]);
                                    c.g[T1][0] = 0x8;
                                    c.g[A2][0] = 0u64;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                    c.g[T2][0] = c.g[S5][0];
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                    break 'b_404510;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S4][0] != c.g[V0][0];
                            c.g[A1][0] = 0x490000;
                            if t {
                                break 'b_404520;
                            } else {
                                c.cycles += 42;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x120));
                                c.g[S3][0] = 0x1;
                                c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                                c.g[A2][0] = c.g[T3][0];
                                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0 {
                                    c.g[S3][0] = 0u64;
                                }
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                FP_DUAL_PRIME.call(c, 0x00404480)?;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                c.g[T0][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[T1][0] = 0x10;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                c.g[T2][0] = c.g[S5][0];
                                c.g[T3][0] = c.g[S6][0];
                                FP_SETUP_PRED_BLOCK.call(c, 0x004044c8)?;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                                if c.g[S7][0] != 0 {
                                    c.g[V0][0] = c.g[V1][0];
                                }
                                c.sw((c.g[SP][0] as u32), c.g[A0][0]);
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                c.g[V1][0] = 0x1;
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[A3][0] = 0u64;
                                break 'b_404504;
                            }
                        }
                        c.cycles += 3;
                        c.g[T0][0] = 0u64;
                        c.g[T1][0] = 0x10;
                        c.g[T2][0] = c.g[S5][0];
                        break 'b_404510;
                    }
                    c.cycles += 4;
                    c.g[T3][0] = c.g[S6][0];
                    FP_SETUP_PRED_BLOCK.call(c, 0x00404518)?;
                    c.g[S0][0] = 0x1;
                    break 'b_404534;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e58));
                c.g[A2][0] = c.g[S4][0];
                FP_ERRORF.call(c, 0x00404530)?;
                c.g[S0][0] = 0x1;
                break 'b_404534;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_404538;
        }
        'b_404728: {
            'b_404724: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x4;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_404724;
                } else {
                    'b_40461c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_40461c;
                        } else {
                            'b_40460c: {
                                'b_404594: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[S4][0] != c.g[V0][0];
                                    c.g[S3][0] = 0x1;
                                    if t {
                                        break 'b_404594;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[T2][0] = c.g[S5][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[T3][0] = c.g[S6][0];
                                        c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                        c.g[A2][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                        c.g[A3][0] = 0u64;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                        c.g[T0][0] = 0u64;
                                        c.g[T1][0] = 0x10;
                                        break 'b_40460c;
                                    }
                                }
                                c.cycles += 30;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A3][0] = 0u64;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1bc));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                c.g[T0][0] = 0u64;
                                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                c.g[T1][0] = 0x8;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[T2][0] = c.g[S5][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                c.g[T3][0] = c.g[S6][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                FP_SETUP_PRED_BLOCK.call(c, 0x004045d4)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.g[T2][0] = c.g[S5][0];
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                c.g[T3][0] = c.g[S6][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[A3][0] = 0x1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                c.g[T0][0] = 0u64;
                                c.g[T1][0] = 0x8;
                                c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
                                break 'b_40460c;
                            }
                            c.cycles += 4;
                            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                            FP_SETUP_PRED_BLOCK.call(c, 0x00404614)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    'b_404674: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_404674;
                        } else {
                            'b_404634: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_404634;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_404634;
                                }
                            }
                            c.cycles += 16;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[T2][0] = c.g[S5][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[T3][0] = c.g[S6][0];
                            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                            c.g[A3][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0x10;
                            FP_SETUP_PRED_BLOCK.call(c, 0x0040466c)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    'b_404714: {
                        c.cycles += 2;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[A1][0] = 0x490000;
                        if t {
                            break 'b_404714;
                        } else {
                            'b_40468c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_40468c;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_40468c;
                                }
                            }
                            'b_4046d4: {
                                c.cycles += 17;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[T0][0] = 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                c.g[T1][0] = 0x8;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                c.g[T2][0] = c.g[S5][0];
                                c.g[T3][0] = c.g[S6][0];
                                FP_SETUP_PRED_BLOCK.call(c, 0x004046c4)?;
                                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1cc));
                                    break 'b_4046d4;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1dc));
                                    break 'b_4046d4;
                                }
                            }
                            c.cycles += 16;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            c.g[T2][0] = c.g[S5][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[T3][0] = c.g[S6][0];
                            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                            c.g[A3][0] = 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                            c.g[T0][0] = 0x8;
                            c.g[T1][0] = 0x8;
                            FP_SETUP_PRED_BLOCK.call(c, 0x0040470c)?;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                            break 'b_404728;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e78));
                    c.g[A2][0] = c.g[S4][0];
                    FP_ERRORF.call(c, 0x00404724)?;
                    break 'b_404724;
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
            break 'b_404728;
        }
        c.cycles += 11;
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1107 callee fakes (in the machine's RAM).
    const FP_CTRL: u32 = DATA + 0xf_8000;
    const FP_MAGIC: u32 = 0x1107_c0de;

    static FP_PREV_SETUP: AtomicUsize = AtomicUsize::new(0);
    static FP_PREV_DUAL: AtomicUsize = AtomicUsize::new(0);
    static FP_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);

    /// `libmpeg_SetupPredBlock`, gated: logs the call and returns.
    fn fp_fake_setup(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_SETUP, ee::s15::f_00404758__generated);
        }
        fakes::fake_call(c, 0x0040_4758)
    }

    /// `libmpeg_DualPrimeArithmetic`, gated: logs the call, then fills the
    /// four-word `DMV` output (a1) from the call's inputs.
    fn fp_fake_dual(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_DUAL, ee::s15::f_00405e30__generated);
        }
        let out = c.g[A1][0] as u32;
        let h = fakes::mix(c.g[A2][0] ^ c.g[A3][0].rotate_left(9) ^ c.g[T0][0]);
        fakes::fake_call(c, 0x0040_5e30);
        for i in 0..4u32 {
            let v = fakes::mix(h + i as u64) as u32 % 64;
            c.bus.write32(out + 4 * i, v);
        }
    }

    /// `libmpeg_Errorf`, gated: logs the call and returns.
    fn fp_fake_errorf(c: &mut Cpu) {
        if c.bus.read32(FP_CTRL) != FP_MAGIC {
            return fakes::fall_back(c, &FP_PREV_ERRORF, ee::s15::f_00403470__generated);
        }
        fakes::fake_call(c, 0x0040_3470)
    }

    fn fp_hook_callees() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00404758),
            fp_fake_setup,
            &FP_PREV_SETUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00405E30),
            fp_fake_dual,
            &FP_PREV_DUAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            fp_fake_errorf,
            &FP_PREV_ERRORF,
        );
    }

    const FP_DEC: u32 = DATA + 0x1000;
    const FP_PMV: u32 = DATA + 0x4000;
    const FP_MVFS: u32 = DATA + 0x4100;
    const FP_DMV: u32 = DATA + 0x4200;

    /// The fixture: the seed walks the combinations of macroblock type (none,
    /// backward, forward, both), frame/field decoder mode (`dec+0x174`),
    /// picture structure (`dec+0x150`), motion type 0..4, the `dec+0x120`
    /// flag; the field selects, vectors and the
    /// reference pointers are random.
    fn fp_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x404050, 6);
        c.bus.write32(FP_CTRL, FP_MAGIC);
        let mut k = seed;
        let mut pick = |n: u32| {
            let v = k % n;
            k /= n;
            v
        };
        let mbtype = [0u32, 4, 8, 12][pick(4) as usize] | (r.next_u32() & 0x13) << 4;
        let frame_mode = [3u32, 1][pick(2) as usize];
        let structure = [1u32, 2, 3][pick(3) as usize];
        let mtype = pick(5);
        let flag120 = pick(2);
        fill(c, &mut r, FP_DEC, 0x900);
        c.bus.write32(FP_DEC + 0x174, frame_mode);
        c.bus.write32(FP_DEC + 0x150, structure);
        c.bus.write32(FP_DEC + 0x120, flag120);
        c.bus.write32(FP_DEC + 0x810, r.next_u32() % 3);
        for off in [0x1b8, 0x1bc, 0x1c8, 0x1cc, 0x1d8, 0x1dc] {
            c.bus
                .write32(FP_DEC + off, DATA + 0x6000 + 0x100 * (off & 0xff));
        }
        for i in 0..8 {
            c.bus.write32(FP_PMV + 4 * i, r.next_u32() % 64);
        }
        for i in 0..4 {
            c.bus.write32(FP_MVFS + 4 * i, r.next_u32() & 1);
        }
        c.g[A0][0] = FP_DEC as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = mbtype as u64;
        c.g[T0][0] = mtype as u64;
        c.g[T1][0] = FP_PMV as u64;
        c.g[T2][0] = FP_MVFS as u64;
        c.g[T3][0] = FP_DMV as u64;
    }

    /// `FormPredictions`: all the motion-compensation legs (frame and field
    /// pictures, forward/backward/both, dual prime, unknown motion type) with
    /// the three callees faked (gated) and unwinding at a random call.
    #[test]
    fn lib_mpeg_form_predictions_matches() {
        fp_hook_callees();
        check(
            ee::s15::f_00404050__generated,
            lib_mpeg_form_predictions,
            960,
            &fp_setup,
        );
        // Many distinct call sequences must really be reached.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..960 {
            let mut c = crate::difftest::fresh();
            fp_setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s15::f_00404050__generated);
            seen.insert(fakes::logged_returns(&mut c));
        }
        assert!(seen.len() >= 20, "only {} call sequences", seen.len());
    }
}
