//! Staging module oc739 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc739`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_a728,
        snd_load_queue_cancel_bank,
        "SndLoadQueue_CancelBank",
    ),
    (0x0029_0fd0, ssx_audio_system_v02, "SSXAudioSystem_v02"),
    (
        0x0029_3fc8,
        ssx_audio_system_calc_edge_loop_group2,
        "SSXAudioSystem_CalcEdgeLoopGroup2",
    ),
    (
        0x002b_6cf8,
        cworld_sound_compute_region_bounds,
        "cWorldSound_ComputeRegionBounds",
    ),
    (
        0x002e_b938,
        transition_effect_draw_rect,
        "TransitionEffect_DrawRect",
    ),
    (
        0x002f_b060,
        world_obj_factory_anim_delta,
        "WorldObjFactory_AnimDelta",
    ),
];

/// `void SndLoadQueue_CancelBank(Queue* q, int bank)` (the C++ `cAsyncSys_ASYNCSYS_Cancel`): removes `bank` from the load queue's rings
/// (+0x18c, +0x19c, +0x1ac, +0x1bc, each with head/tail/count words), compacting the rest;
/// finishes the current load when it is the cancelled one and starts the next.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn snd_load_queue_cancel_bank(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0028a728);
    'b_28aaec: {
        'b_28a78c: {
            'b_28a788: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] == 0u64;
                c.g[T3][0] = c.g[A1][0];
                if t {
                    break 'b_28a788;
                } else {
                    'b_28a76c: {
                        'b_28a758: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18c));
                            if t {
                                break 'b_28a758;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_28a76c;
                                } else {
                                    break 'b_28a758;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                        break 'b_28a76c;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[T3][0] {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                        break 'b_28a78c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[A2][0];
                        c.g[RA][0] = sx(0x0028a780);
                        c.g[A1][0] = 0u64;
                        c.pc = 0x00289df0;
                        ee::s07::f_00289df0(c);
                        if c.pc != 0x0028a780 {
                            return;
                        }
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_28aaec;
                        } else {
                            break 'b_28a788;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
            break 'b_28a78c;
        }
        'b_28a7a4: {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            if c.g[V0][0] == 0u64 {
                break 'b_28a7a4;
            } else {
                break 'b_28a7a4;
            }
        }
        'b_28aad8: {
            c.cycles += 4;
            c.div(0, 3, 2);
            c.g[V1][0] = c.hi[0];
            if (c.g[V1][0] as i64) <= 0 {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                break 'b_28aad8;
            } else {
                c.cycles += 3;
                c.g[T2][0] = c.g[V1][0];
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18c));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                'l_28a7c0: loop {
                    'b_28a800: {
                        'b_28a7d4: {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                            if c.g[V0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
                                break 'b_28a7d4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A1][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_28a800;
                                } else {
                                    break 'b_28a7d4;
                                }
                            }
                        }
                        'b_28a7fc: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x198), c.g[A1][0]);
                            if t {
                                break 'b_28a7fc;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x198), 0u64);
                                break 'b_28a7fc;
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_28a800;
                    }
                    'b_28aacc: {
                        'b_28aac8: {
                            'b_28a87c: {
                                c.cycles += 2;
                                if c.g[A1][0] != c.g[T3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                                    break 'b_28a87c;
                                } else {
                                    'b_28a82c: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                        let t = c.g[V0][0] == c.g[A0][0];
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_28a82c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x1a8),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), 0u64);
                                                break 'b_28a82c;
                                            } else {
                                                break 'b_28a82c;
                                            }
                                        }
                                    }
                                    'b_28a850: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                        let t = c.g[V0][0] == c.g[A0][0];
                                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_28a850;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                            c.sw(
                                                (c.g[A2][0] as u32).wrapping_add(0x1b8),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), 0u64);
                                                break 'b_28a850;
                                            } else {
                                                break 'b_28a850;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                    let t = c.g[V0][0] == c.g[A0][0];
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_28aac8;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[V0][0]);
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), 0u64);
                                            break 'b_28aac8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                                            if t {
                                                break 'b_28aacc;
                                            } else {
                                                break 'b_28a87c;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_28a890: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a890;
                                } else {
                                    break 'b_28a890;
                                }
                            }
                            'b_28a8d4: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x198));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                    break 'b_28a8d4;
                                } else {
                                    'b_28a8d0: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x194), c.g[V1][0]);
                                        if t {
                                            break 'b_28a8d0;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x194), 0u64);
                                            break 'b_28a8d0;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                    break 'b_28a8d4;
                                }
                            }
                            'b_28a928: {
                                'b_28a8f0: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x19c));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28a8f0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28a928;
                                        } else {
                                            break 'b_28a8f0;
                                        }
                                    }
                                }
                                'b_28a91c: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), c.g[A1][0]);
                                    if t {
                                        break 'b_28a91c;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1a8), 0u64);
                                        break 'b_28a91c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a8));
                                break 'b_28a928;
                            }
                            'b_28a93c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a93c;
                                } else {
                                    break 'b_28a93c;
                                }
                            }
                            'b_28a97c: {
                                c.cycles += 3;
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                    break 'b_28a97c;
                                } else {
                                    'b_28a978: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1a4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x19c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1a4), c.g[V1][0]);
                                        if t {
                                            break 'b_28a978;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1a4), 0u64);
                                            break 'b_28a978;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                    break 'b_28a97c;
                                }
                            }
                            'b_28a9d0: {
                                'b_28a998: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28a998;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28a9d0;
                                        } else {
                                            break 'b_28a998;
                                        }
                                    }
                                }
                                'b_28a9c4: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), c.g[A1][0]);
                                    if t {
                                        break 'b_28a9c4;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1b8), 0u64);
                                        break 'b_28a9c4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b8));
                                break 'b_28a9d0;
                            }
                            'b_28a9e4: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28a9e4;
                                } else {
                                    break 'b_28a9e4;
                                }
                            }
                            'b_28aa24: {
                                c.cycles += 3;
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    break 'b_28aa24;
                                } else {
                                    'b_28aa20: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1b4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1ac));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1b4), c.g[V1][0]);
                                        if t {
                                            break 'b_28aa20;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[A2][0] as u32).wrapping_add(0x1b4), 0u64);
                                            break 'b_28aa20;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                    break 'b_28aa24;
                                }
                            }
                            'b_28aa78: {
                                'b_28aa40: {
                                    c.cycles += 5;
                                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                    c.g[A0][0] = c.g[V1][0];
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_28aa40;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[T0][0] = 0xffffffffffffffff;
                                        if t {
                                            break 'b_28aa78;
                                        } else {
                                            break 'b_28aa40;
                                        }
                                    }
                                }
                                'b_28aa6c: {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[A1][0]);
                                    if t {
                                        break 'b_28aa6c;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), 0u64);
                                        break 'b_28aa6c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                c.g[T0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c8));
                                break 'b_28aa78;
                            }
                            'b_28aa8c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_28aa8c;
                                } else {
                                    break 'b_28aa8c;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.hi[0];
                            if c.g[A0][0] == c.g[A1][0] {
                                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                                break 'b_28aacc;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c4));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x1c4), c.g[V1][0]);
                                if t {
                                    break 'b_28aac8;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c4), 0u64);
                                    break 'b_28aac8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                        break 'b_28aacc;
                    }
                    c.cycles += 2;
                    if c.g[T2][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x194));
                        if event_due(c) {
                            deliver_event_here(c, 0x0028a7c0);
                        }
                        continue 'l_28a7c0;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        break 'b_28aad8;
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        if t {
            break 'b_28aaec;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028aae8);
            c.g[A0][0] = c.g[A2][0];
            c.pc = 0x0028a298;
            ee::s07::f_0028a298(c);
            if c.pc != 0x0028aae8 {
                return;
            }
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_28aaec;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `int SSXAudioSystem_v02(SSXAudio* self, int which)`: the trick-power sound level 0..3 of the focus rider.
/// 3 unless the rider is in motion state 1 and the air time left is at most 3.0 with positive trick power;
/// then 0, 1 or 2 by the trick power below 0.25 / 0.5.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn ssx_audio_system_v02(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x00290fd0);
    'b_2910c8: {
        'b_2910c4: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x00290ff4);
            c.g[S2][0] = c.g[A0][0];
            c.pc = 0x00285d98;
            ee::s07::f_00285d98(c);
            if c.pc != 0x00290ff4 {
                return;
            }
            c.g[S3][0] = 0x3;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00291004);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x0011fe98;
            call_indirect(c, 0x0011fe98);
            if c.pc != 0x00291004 {
                return;
            }
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = 0x1;
            let t = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                break 'b_2910c4;
            } else {
                'b_291040: {
                    'b_29103c: {
                        c.cycles += 8;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x788));
                        c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                        c.g[A1][0] = c.g[V1][0];
                        let t = c.g[A0][0] == c.g[A2][0];
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x6074));
                        if t {
                            break 'b_29103c;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[S3][0];

                            if t {
                                break 'b_291040;
                            } else {
                                break 'b_29103c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A3][0] = 0x1;
                    break 'b_291040;
                }
                'b_291060: {
                    'b_291058: {
                        c.cycles += 2;
                        let t = c.g[A3][0] == 0u64;

                        if t {
                            break 'b_291058;
                        } else {
                            c.cycles += 4;
                            c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0xa0));
                            c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x98));
                            let t = true;
                            c.sub_s(1, 0, 1);
                            if t {
                                break 'b_291060;
                            } else {
                                break 'b_291058;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[AT][0] = 0x40800000;
                    c.mtc1(1, 1);
                    break 'b_291060;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 0);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = c.g[S3][0];
                if t {
                    break 'b_2910c4;
                } else {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(0, 2);
                    let t = !c.fcc();
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_2910c8;
                    } else {
                        'b_2910c0: {
                            c.cycles += 6;
                            c.g[AT][0] = 0x3e800000;
                            c.mtc1(1, 0);
                            c.c_lt_s(2, 0);
                            let t = c.fcc();
                            c.g[S3][0] = 0u64;
                            if t {
                                break 'b_2910c0;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 0);
                                c.c_lt_s(2, 0);
                                let t = !c.fcc();
                                c.g[S3][0] = 0x2;
                                if t {
                                    break 'b_2910c0;
                                } else {
                                    c.cycles += 1;
                                    c.g[S3][0] = 0x1;
                                    break 'b_2910c0;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S3][0];
                        break 'b_2910c4;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        break 'b_2910c8;
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_CalcEdgeLoopGroup2(SSXAudio* self, Rider* rider, int* volume, int* pitch)`: smooths the
/// carve edging level per rider (fast fall, slow rise), sets volume 0x7f, pitch from the bend curve
/// (`Audio_EvalCurve5`) and scales the volume down at speeds under 0.1.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn ssx_audio_system_calc_edge_loop_group2(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x00293fc8);
    'b_294090: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S1][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x758));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c0));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x754));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x750));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        c.add_s(20, 1, 0);
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00294020);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x00294020 {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x45ac));
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x5990));
        c.sub_s(3, 2, 20);
        c.abs_s(0, 3);
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        if t {
            break 'b_294090;
        } else {
            'b_29406c: {
                c.cycles += 3;
                c.c_lt_s(20, 2);
                let t = !c.fcc();

                if t {
                    break 'b_29406c;
                } else {
                    c.cycles += 5;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.mul_s(0, 3, 0);
                    let t = true;
                    c.sub_s(20, 2, 0);
                    if t {
                        break 'b_294090;
                    } else {
                        break 'b_29406c;
                    }
                }
            }
            c.cycles += 4;
            c.c_lt_s(2, 20);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if t {
                break 'b_294090;
            } else {
                c.cycles += 5;
                c.sub_s(0, 20, 2);
                c.g[AT][0] = 0x3e000000;
                c.mtc1(1, 1);
                c.mul_s(0, 0, 1);
                c.add_s(20, 2, 0);
                break 'b_294090;
            }
        }
    }
    'b_294150: {
        c.cycles += 40;
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = 0x7f;
        c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x5990));
        c.mov_s(12, 21);
        c.sw((c.g[S1][0] as u32), c.g[V1][0]);
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d08));
        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[A3][0] as u32));
        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
        c.ldl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
        c.ldr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[SP][0] as u32));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x1f));
        c.ldr(2, (c.g[A3][0] as u32).wrapping_add(0x18));
        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x27));
        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x20));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x1f));
        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = sx(0x00294104);
        c.g[A0][0] = c.g[SP][0];
        c.pc = 0x00290b58;
        ee::s07::f_00290b58(c);
        if c.pc != 0x00294104 {
            return;
        }
        c.g[AT][0] = 0x45800000;
        c.mtc1(1, 1);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x45a8));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, (c.g[S3][0] as u32));
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x75c));
        c.c_lt_s(3, 2);
        let t = !c.fcc();
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        if t {
            break 'b_294150;
        } else {
            c.cycles += 8;
            c.lwc1(0, (c.g[S1][0] as u32));
            c.cvt_s_w(0, 0);
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 1);
            c.mul_s(0, 0, 3);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.swc1(1, (c.g[S1][0] as u32));
            break 'b_294150;
        }
    }
    c.cycles += 8;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

/// `void cWorldSound_ComputeRegionBounds(cWorldSound* self, Entry* entry, Vec3* lo, Vec3* hi)`: the world-space box of
/// a sound region: the union of its volume boxes (`cWorldSound_GetShapeBounds` per record), shifted by the
/// instance translation.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cworld_sound_compute_region_bounds(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002b6cf8);
    'b_2b6ea8: {
        c.cycles += 43;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4544));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S1][0] = c.g[A2][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S6][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.g[A3][0];
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[S3][0] = 0u64;
        c.swc1(0, (c.g[SP][0] as u32));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[S1][0] as u32));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4540));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[SP][0] as u32));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[S2][0] as u32));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[S4][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S4][0] as u32));
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let t = (c.g[A0][0] as i64) <= 0;
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        if t {
            break 'b_2b6ea8;
        } else {
            c.cycles += 3;
            c.g[FP][0] = 0x2;
            c.g[S7][0] = 0x3;
            c.g[A0][0] = c.g[S5][0];
            'l_2b6db0: loop {
                'b_2b6dd8: {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002b6dc0);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = 0x002b6f40;
                    ee::s08::f_002b6f40(c);
                    if c.pc != 0x002b6dc0 {
                        return;
                    }
                    c.lwc1(1, (c.g[SP][0] as u32));
                    c.lwc1(0, (c.g[S1][0] as u32));
                    c.c_lt_s(1, 0);
                    if c.fcc() {
                        c.swc1(1, (c.g[S1][0] as u32));
                        break 'b_2b6dd8;
                    } else {
                        break 'b_2b6dd8;
                    }
                }
                'b_2b6df0: {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.c_lt_s(1, 0);
                    if c.fcc() {
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                        break 'b_2b6df0;
                    } else {
                        break 'b_2b6df0;
                    }
                }
                'b_2b6e08: {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.c_lt_s(1, 0);
                    if c.fcc() {
                        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_2b6e08;
                    } else {
                        break 'b_2b6e08;
                    }
                }
                'b_2b6e20: {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[S2][0] as u32));
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.swc1(1, (c.g[S2][0] as u32));
                        break 'b_2b6e20;
                    } else {
                        break 'b_2b6e20;
                    }
                }
                'b_2b6e38: {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x4));
                        break 'b_2b6e38;
                    } else {
                        break 'b_2b6e38;
                    }
                }
                'b_2b6e50: {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_2b6e50;
                    } else {
                        break 'b_2b6e50;
                    }
                }
                'b_2b6e98: {
                    'b_2b6e94: {
                        'b_2b6e90: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_2b6e90;
                            } else {
                                'b_2b6e78: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2b6e78;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == 0u64 {
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                                            break 'b_2b6e94;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                            if t {
                                                break 'b_2b6e98;
                                            } else {
                                                break 'b_2b6e78;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[FP][0] {
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                                    break 'b_2b6e94;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[S7][0] {
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                        break 'b_2b6e94;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_2b6e98;
                                        } else {
                                            break 'b_2b6e90;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                        break 'b_2b6e94;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    break 'b_2b6e98;
                }
                c.cycles += 4;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.g[S5][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b6db0);
                    }
                    continue 'l_2b6db0;
                } else {
                    break 'b_2b6ea8;
                }
            }
        }
    }
    c.cycles += 37;
    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
    c.lwc1(1, (c.g[S1][0] as u32));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x40));
    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.add_s(1, 1, 0);
    c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.swc1(1, (c.g[S1][0] as u32));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x44));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.add_s(2, 2, 0);
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
    c.add_s(3, 3, 0);
    c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x40));
    c.lwc1(0, (c.g[S2][0] as u32));
    c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
    c.add_s(0, 0, 1);
    c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S2][0] as u32));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x44));
    c.add_s(2, 2, 1);
    c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
    c.add_s(3, 3, 0);
    c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x8));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = t;
    return;
}

/// `TransitionEffect_DrawRect`: fills a rectangle through a draw vtable call in a loop.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn transition_effect_draw_rect(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002eb938);
    c.cycles += 8;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
    c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
    c.g[T0][0] = c.g[A0][0];
    c.g[T2][0] = c.g[A1][0];
    c.g[V0][0] = 0x3;
    c.g[V1][0] = 0xffffffffffffffff;
    'l_2eb958: loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[A0][0] = 0xffffffffffffffff;
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x002eb958);
            }
            continue 'l_2eb958;
        } else {
            'b_2eba24: {
                'b_2eb9d4: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    if c.g[V1][0] == c.g[A0][0] {
                        c.lwc1(5, (c.g[A2][0] as u32));
                        break 'b_2eb9d4;
                    } else {
                        c.cycles += 20;
                        c.g[AT][0] = 0x43000000;
                        c.mtc1(1, 0);
                        c.lwc1(4, (c.g[A2][0] as u32));
                        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.mul_s(4, 4, 0);
                        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                        c.mul_s(2, 2, 0);
                        c.mul_s(3, 3, 0);
                        c.mul_s(1, 1, 0);
                        c.cvt_w_s(0, 2);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.cvt_w_s(0, 3);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                        c.cvt_w_s(0, 1);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.cvt_w_s(0, 4);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                        let t = true;
                        c.lwc1(2, (c.g[T0][0] as u32));
                        if t {
                            break 'b_2eba24;
                        } else {
                            break 'b_2eb9d4;
                        }
                    }
                }
                c.cycles += 20;
                c.g[AT][0] = 0x437f0000;
                c.mtc1(1, 0);
                c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 0);
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 4);
                c.mul_s(2, 2, 0);
                c.mul_s(3, 3, 0);
                c.mul_s(5, 5, 4);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.cvt_w_s(0, 2);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.cvt_w_s(0, 3);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.cvt_w_s(0, 5);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lwc1(2, (c.g[T0][0] as u32));
                break 'b_2eba24;
            }
            c.cycles += 58;
            c.g[A1][0] = 0x4;
            c.lwc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 4);
            c.g[A3][0] = 0u64;
            c.lwc1(3, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
            c.add_s(3, 2, 3);
            c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xc8));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xcc));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
            c.lq(9, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.add_s(1, 0, 1);
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x8));
            c.lq(8, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
            c.lq(2, (c.g[SP][0] as u32));
            c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(8, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(9, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lwc1(3, (c.g[T2][0] as u32).wrapping_add(0x4));
            c.lwc1(2, (c.g[T2][0] as u32).wrapping_add(0xc));
            c.lwc1(1, (c.g[T2][0] as u32));
            c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
            c.add_s(2, 3, 2);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.add_s(0, 1, 0);
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.swc1(1, (c.g[SP][0] as u32));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
            c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002ebb00);
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            call_indirect(c, t);
            if c.pc != 0x002ebb00 {
                return;
            }
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
            c.pc = t;
            return;
        }
    }
}

/// `WorldObjFactory_AnimDelta`: builds an animation-delta modifier record (template memcpy, parameter fill,
/// `AttachModifier`, allocation).
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn world_obj_factory_anim_delta(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002fb060);
    'b_2fb0ec: {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x24d0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = 0x500000;
        let t = c.g[V0][0] != 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        if t {
            break 'b_2fb0ec;
        } else {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c70));
            c.g[V1][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 0);
            c.g[A0][0] = 0x1;
            c.g[AT][0] = 0x41f00000;
            c.mtc1(1, 1);
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4c70), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
            c.sw((c.g[GP][0] as u32).wrapping_add(0x24d0), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
            c.sb((c.g[SP][0] as u32), c.g[A1][0]);
            break 'b_2fb0ec;
        }
    }
    'b_2fb170: {
        c.cycles += 11;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c70));
        c.g[A2][0] = 0x2c;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002fb110);
        c.g[S4][0] = 0x4d0000;
        c.pc = 0x003e6574;
        call_indirect(c, 0x003e6574);
        if c.pc != 0x002fb110 {
            return;
        }
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if t {
            break 'b_2fb170;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e70));
            c.g[A3][0] = 0x2;
            'l_2fb128: loop {
                'b_2fb164: {
                    'b_2fb160: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            break 'b_2fb160;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[A3][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2fb160;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                c.cvt_s_w(0, 0);
                                let t = true;
                                c.swc1(0, (c.g[A0][0] as u32));
                                if t {
                                    break 'b_2fb164;
                                } else {
                                    break 'b_2fb160;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                    break 'b_2fb164;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002fb128);
                    }
                    continue 'l_2fb128;
                } else {
                    break 'b_2fb170;
                }
            }
        }
    }
    'b_2fb1dc: {
        'b_2fb18c: {
            c.cycles += 4;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = !(0u64 | c.g[A2][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if t {
                break 'b_2fb18c;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                let t = true;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                if t {
                    break 'b_2fb1dc;
                } else {
                    break 'b_2fb18c;
                }
            }
        }
        'b_2fb1d8: {
            'b_2fb1b4: {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                if c.g[V1][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    break 'b_2fb1b4;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = 0u64;
                    if t {
                        break 'b_2fb1d8;
                    } else {
                        break 'b_2fb1b4;
                    }
                }
            }
            'b_2fb1d4: {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t {
                    break 'b_2fb1d4;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_2fb1d4;
                }
            }
            c.cycles += 1;
            c.g[A2][0] = c.g[V0][0];
            break 'b_2fb1d8;
        }
        c.cycles += 1;
        c.g[S0][0] = c.g[A2][0];
        break 'b_2fb1dc;
    }
    'b_2fb22c: {
        c.cycles += 2;
        let t = c.g[S0][0] == 0u64;

        if t {
            break 'b_2fb22c;
        } else {
            'b_2fb200: {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = 0x2;
                if t {
                    break 'b_2fb200;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002fb1f8);
                    c.g[A2][0] = c.g[S1][0];
                    c.pc = 0x002fae38;
                    ee::s10::f_002fae38(c);
                    if c.pc != 0x002fb1f8 {
                        return;
                    }
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_2fb22c;
                    } else {
                        break 'b_2fb200;
                    }
                }
            }
            c.cycles += 11;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = 0x70;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x69a8));
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x002fb218);
            c.g[A3][0] = 0u64;
            c.pc = 0x00317d70;
            ee::s10::f_00317d70(c);
            if c.pc != 0x002fb218 {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002fb22c);
            c.g[A3][0] = c.g[S1][0];
            c.pc = 0x00359228;
            call_indirect(c, 0x00359228);
            if c.pc != 0x002fb22c {
                return;
            }
            break 'b_2fb22c;
        }
    }
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[S2][0] as u32));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S2][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Control block of this module's gated callee fakes (in the machine's
    /// own RAM, away from the pool and the fakes' call log at DATA+0x100000).
    const OC739_CTRL: u32 = DATA + 0x20_0000;
    const OC739_MAGIC: u32 = 0x0739_0739;

    /// Gated fakes for callees with override slots: with the magic word at
    /// `OC739_CTRL` they return the word at `OC739_CTRL + off` in v0; with the
    /// gate off they run what the slot held before.
    macro_rules! oc739_fakes {
        ($($fake:ident = $off:expr, $shard:ident :: $slot:ident / $gen:ident;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC739_CTRL) != OC739_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    let v = c.bus.read32(OC739_CTRL + $off);
                    c.g[V0][0] = sx(v);
                    ret(c);
                }
            )*
            fn oc739_install() {
                $(
                    fakes::hook(
                        std::ptr::addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc739_fakes! {
        oc739_fake_find_rider = 0x04, s07::OVERRIDE_00285D98 / f_00285d98__generated;
        oc739_fake_main_state = 0x08, s00::OVERRIDE_0011FE98 / f_0011fe98__generated;
        oc739_fake_alloc = 0x10, s10::OVERRIDE_00317D70 / f_00317d70__generated;
        oc739_fake_delta_ctor = 0x14, s11::OVERRIDE_00359228 / f_00359228__generated;
        oc739_fake_attach = 0x18, s10::OVERRIDE_002FAE38 / f_002fae38__generated;
        oc739_fake_async_cancel = 0x20, s14::OVERRIDE_003DFAF0 / f_003dfaf0__generated;
        oc739_fake_async_result = 0x24, s14::OVERRIDE_003DF9D8 / f_003df9d8__generated;
        oc739_fake_async_load_into = 0x28, s14::OVERRIDE_003DF748 / f_003df748__generated;
        oc739_fake_async_load = 0x2c, s14::OVERRIDE_003DF690 / f_003df690__generated;
        oc739_fake_strcpy = 0x30, s15::OVERRIDE_004166F4 / f_004166f4__generated;
    }

    fn oc739_gate(c: &mut Cpu) {
        c.bus.write32(OC739_CTRL, OC739_MAGIC);
    }

    /// `SndLoadQueue_CancelBank`: four rings (size, data, head, tail at
    /// +0x18c/+0x19c/+0x1ac/+0x1bc), the busy word at +4. On `seed % 3 == 0`
    /// the current load is the cancelled bank and busy is set (finish path).
    #[test]
    fn snd_load_queue_cancel_bank_matches() {
        oc739_install();
        fakes::covered(
            ee::s07::f_0028a728__generated,
            snd_load_queue_cancel_bank,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28a728, 6);
                oc739_gate(c);
                for k in 0..8u32 {
                    let v = r.next_u32() % 2;
                    c.bus.write32(
                        OC739_CTRL + 0x20 + 4 * k,
                        if k < 2 { v } else { DATA + 0x5000 + 0x40 * k },
                    );
                }
                let this = DATA + 0x100;
                let id = 1 + r.next_u32() % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id as u64;
                let finish = seed % 3 == 0;
                let busy = if finish { 1 } else { r.next_u32() % 2 };
                c.bus.write32(this, DATA + 0x5000);
                c.bus.write32(this + 4, busy);
                for k in 0..4u32 {
                    let base = this + 0x18c + 0x10 * k;
                    let size = 2 + r.next_u32() % 5;
                    let data = DATA + 0x1000 + 0x40 * k;
                    let head = r.next_u32() % size;
                    let mut tail = r.next_u32() % size;
                    if (k == 0 || k == 3) && finish && head == tail {
                        tail = (tail + 1) % size;
                    }
                    c.bus.write32(base, size);
                    c.bus.write32(base + 4, data);
                    c.bus.write32(base + 8, head);
                    c.bus.write32(base + 0xc, tail);
                    for i in 0..size {
                        c.bus.write32(data + 4 * i, 1 + r.next_u32() % 3);
                    }
                    if k == 0 && finish {
                        c.bus.write32(data + 4 * tail, id);
                    }
                    if k == 3 && finish {
                        // The finish path calls the node virtuals for kinds 1 and 2.
                        c.bus.write32(data + 4 * tail, 1 + r.next_u32() % 2);
                    }
                }
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                let vt = DATA + 0x2000;
                c.bus.write32(this + 0x1d4, vt);
                for off in (0..0x40u32).step_by(4) {
                    let lh_slot = matches!(off, 0x10 | 0x18 | 0x20 | 0x28);
                    c.bus
                        .write32(vt + off, if lh_slot { 0 } else { fakes::FAKE });
                }
            },
        );
    }

    /// `SSXAudioSystem_v02`: the rider (fake `FindRider`), its motion state
    /// (fake `GetMainState`: 1 mostly), the air predictor's mode and times, and
    /// the per-player trick power.
    #[test]
    fn ssx_audio_system_v02_matches() {
        oc739_install();
        check(
            ee::s07::f_00290fd0__generated,
            ssx_audio_system_v02,
            300,
            &|c, seed| {
                let mut r = Rng(0x0029_0fd0 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc739_gate(c);
                let this = DATA;
                let rider = DATA + 0x1000;
                let air = DATA + 0x2000;
                let which = r.next_u32() % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = which as u64;
                c.bus.write32(OC739_CTRL + 4, rider);
                let state = if r.next_u32() % 6 == 0 { 2 } else { 1 };
                c.bus.write32(OC739_CTRL + 8, state);
                let powers = [0.0f32, -0.5, 0.1, 0.25, 0.3, 0.5, 0.9];
                let power = powers[(r.next_u32() % 7) as usize];
                c.bus.write32(this + 0x6074 + 4 * which, power.to_bits());
                c.bus.write32(rider + 0x788, air);
                c.bus.write32(air + 0xac, r.next_u32() % 4);
                let (total, cur) =
                    [(10.0f32, 8.0f32), (10.0, 7.0), (10.0, 6.0)][(r.next_u32() % 3) as usize];
                c.bus.write32(air + 0x98, total.to_bits());
                c.bus.write32(air + 0xa0, cur.to_bits());
            },
        );
    }

    /// `SSXAudioSystem_CalcEdgeLoopGroup2`: the rider's audio fields, the
    /// previous edging value of its AI index, the bend curve at 0x482d08 and
    /// the AI-index virtual (a fake, or the real leaf at 0x1406c0 returning 0).
    #[test]
    fn ssx_audio_system_calc_edge_loop_group2_matches() {
        fakes::covered(
            ee::s07::f_00293fc8__generated,
            ssx_audio_system_calc_edge_loop_group2,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x293fc8, 1);
                let this = DATA;
                let rider = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(DATA + 0x3000);
                c.g[A3][0] = sx(DATA + 0x3010);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x45ac), 0.1f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x45a8), 0.1f32.to_bits());
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(vt + 0x38, 0);
                if seed % 4 == 0 {
                    c.bus.write32(vt + 0x3c, 0x0014_06c0);
                } else {
                    c.bus.write32(vt + 0x3c, fakes::FAKE);
                    c.bus.write32(fakes::LOG_RESULTS, 1 + seed % 3);
                }
                let idx = if seed % 4 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write32(rider + 0x750, r.f32(1.0));
                c.bus.write32(rider + 0x754, r.f32(1.0));
                c.bus.write32(rider + 0x758, r.f32(1.2));
                let speed = match r.next_u32() % 3 {
                    0 => 0.05f32.to_bits(),
                    1 => (-0.2f32).to_bits(),
                    _ => r.f32(2.0),
                };
                c.bus.write32(rider + 0x75c, speed);
                let prev = if r.next_u32() % 5 == 0 {
                    // Close to the new level: the dead zone.
                    let lvl = f32::from_bits(c.bus.read32(rider + 0x750))
                        + f32::from_bits(c.bus.read32(rider + 0x754));
                    (lvl + 0.01).to_bits()
                } else {
                    r.f32(2.0)
                };
                c.bus.write32(this + 0x5990 + 4 * idx, prev);
                let curve = [0.0f32, 0.25, 0.5, 0.75, 1.0];
                for (i, t) in curve.iter().enumerate() {
                    c.bus.write32(0x0048_2d08 + 4 * i as u32, t.to_bits());
                    c.bus.write32(0x0048_2d08 + 0x14 + 4 * i as u32, r.f32(1.0));
                }
            },
        );
    }

    /// `cWorldSound_ComputeRegionBounds`: `nvol` volume records of the four
    /// kinds (0x1c, 0x30, 0x30, 0x18 bytes; kind 4 is an unknown one that does
    /// not advance), the instance translation at +0x40, and the 10000 limits
    /// next to gp. The shape callee is the real leaf.
    #[test]
    fn cworld_sound_compute_region_bounds_matches() {
        check(
            ee::s08::f_002b6cf8__generated,
            cworld_sound_compute_region_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x002b_6cf8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let entry = DATA + 0x100;
                let info = DATA + 0x200;
                let obj = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(entry);
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = sx(DATA + 0x410);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x4544), 10000.0f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x4540), (-10000.0f32).to_bits());
                c.bus.write32(entry + 0x10, info);
                c.bus.write32(entry + 0xc, obj);
                for i in 0..3 {
                    c.bus.write32(obj + 0x40 + 4 * i, r.f32(50.0));
                }
                let ntrig = r.next_u32() % 3;
                let nvol = (seed % 5) as i32 - 1;
                c.bus.write32(info, ntrig);
                c.bus.write32(info + 4, nvol as u32);
                let mut cur = info + 8 + 4 * ntrig;
                for i in 0..0x40u32 {
                    c.bus.write32(cur + 4 * i, r.f32(500.0));
                }
                for _ in 0..nvol.max(0) {
                    let kind = r.next_u32() % 5;
                    c.bus.write32(cur, kind);
                    cur += [0x1c, 0x30, 0x30, 0x18, 0][kind as usize];
                    // Later records start at the advanced cursor: random floats there.
                    for i in 0..0x10u32 {
                        c.bus.write32(cur + 4 * i + 4, r.f32(500.0));
                    }
                }
            },
        );
    }

    /// `TransitionEffect_DrawRect`: the rect/colour float records, the
    /// renderer object (+0xe84 with the half at +0x10 equal to -1 or not) and
    /// the draw virtual at +0x10d8/+0x23c (a fake).
    #[test]
    fn transition_effect_draw_rect_matches() {
        fakes::covered(
            ee::s09::f_002eb938__generated,
            transition_effect_draw_rect,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eb938, 1);
                let (pos, rect, color) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                let (gfx, scene, vt) = (DATA + 0x400, DATA + 0x500, DATA + 0x600);
                c.g[A0][0] = sx(pos);
                c.g[A1][0] = sx(rect);
                c.g[A2][0] = sx(color);
                c.f[12] = r.f32(100.0);
                for i in 0..4 {
                    c.bus.write32(pos + 4 * i, r.f32(100.0));
                    c.bus.write32(rect + 4 * i, r.f32(100.0));
                    c.bus.write32(color + 4 * i, r.f32(2.0));
                }
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0xe84, scene);
                c.bus
                    .write16(scene + 0x10, if seed % 2 == 0 { 0xffff } else { 3 });
                c.bus.write32(gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x238, 0);
                c.bus.write32(vt + 0x23c, fakes::FAKE);
            },
        );
    }

    /// `WorldObjFactory_AnimDelta` (the script function `MakeAnimDeltaFunc`):
    /// the init flag (`gp+0x24d0`), the parameter list (idx, val, type) with
    /// its type table at 0x445e70, the target found through the script
    /// manager (id -1) or the world's section table, and the node found,
    /// attached (fake result) or allocated (fake allocator and constructor).
    #[test]
    fn world_obj_factory_anim_delta_matches() {
        oc739_install();
        check(
            ee::s10::f_002fb060__generated,
            world_obj_factory_anim_delta,
            400,
            &|c, seed| {
                let mut r = Rng(0x002f_b060 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc739_gate(c);
                c.bus.write32(OC739_CTRL + 0x10, DATA + 0xb000);
                c.bus.write32(OC739_CTRL + 0x14, DATA + 0xb000);
                c.bus.write32(OC739_CTRL + 0x18, r.next_u32() % 2);
                let out = DATA + 0x7000;
                let params = DATA + 0x1000;
                let n = r.next_u32() % 4;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(params);
                c.bus.write32(GAME_GP + 0x24d0, r.next_u32() % 2);
                for i in 0..12 {
                    c.bus.write32(0x004f_b390 + 4 * i, r.next_u32());
                    c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
                }
                for i in 0..11 {
                    // Entry 0 (the instance id) is never a float: a float id would index far outside RAM.
                    let kind = if i == 0 {
                        r.next_u32() % 2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_5e70 + 4 * i, kind);
                }
                for i in 0..n {
                    let e = params + 16 * i;
                    let idx = if i == 0 && r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 10
                    };
                    let val = if idx == 0 {
                        (r.next_u32() % 3) << 8
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(e, idx);
                    c.bus.write32(e + 4, val);
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                // World -> section table -> instance record chain, and the script manager.
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                let section = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    DATA + 0x6000
                };
                c.bus.write32(DATA + 0x5000, section);
                c.bus.write32(DATA + 0x601c, DATA + 0x6100);
                for k in 0..3u32 {
                    let word = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        ((DATA + 0x8000) >> 2) << 8
                    };
                    c.bus.write32(DATA + 0x6100 + 4 * k, word);
                }
                c.bus.write32(
                    DATA + 0x800c,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x9000
                    },
                );
                c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
                c.bus.write32(
                    DATA + 0xa290,
                    if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        DATA + 0x8000
                    },
                );
            },
        );
    }
}
