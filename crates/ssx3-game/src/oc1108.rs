//! Staging module oc1108 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1108`.

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
        0x0040_2e10,
        libmpeg_get_picture_common,
        "libmpeg_GetPictureCommon",
    ),
    (
        0x0040_4b78,
        libmpeg_add_pred_and_store,
        "libmpeg_AddPredAndStore",
    ),
];

/// `int libmpeg_GetPictureCommon(Mpeg *m)` (0x00402e10): the MPEG picture
/// pump. With `d = m->dec` (+0x40) it clears `d[0]` and requires
/// `d[54] & 0x3f == 0` (else `libmpeg_Errorf`, returns -1); then loops: read
/// headers (`libmpeg_GetHeader`) until the picture type is 0, the header
/// counters (`d[93]`, `d[53]`) agree or `d[530]` is clear, and dispatch on the
/// type (0: flush the pending picture and set `d[0]`; 1/2/3/4: decode a
/// picture into the I/P/B slot counters `d[40..42]`). Returns 1 once `d[520]`
/// or `d[0]` is set. Converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libmpeg_get_picture_common(c: &mut Cpu) {
    if c.pc != 0x00402e10 {
        return ee::s15::f_00402e10__generated(c);
    }
    guest(c, |c| {
        'b_402f64: {
            'b_402e60: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.g[S3][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.g[S2][0] = 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[S0][0]);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
                c.g[V0][0] = c.g[A2][0] & 0x3f;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32), 0u64);
                if t {
                    break 'b_402e60;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00402e58);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7c68));
                    Callee {
                        addr: 0x00403470,
                        f: ee::s15::libmpeg_Errorf,
                    }
                    .call(c, 0x00402e58)?;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_402f64;
                    } else {
                        break 'b_402e60;
                    }
                }
            }
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x820), 0u64);
            c.g[V0][0] = 0xffffffffffffffff;
            'l_402e68: loop {
                'b_402ea0: {
                    c.cycles += 2;
                    let t = c.g[S2][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                    if t {
                        break 'b_402ea0;
                    } else {
                        'l_402e70: loop {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x00402e78);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00407520,
                                f: ee::s15::libmpeg_GetHeader,
                            }
                            .call(c, 0x00402e78)?;
                            c.g[S3][0] = c.g[V0][0];
                            let t = c.g[S3][0] == 0u64;
                            c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                            if t {
                                break 'b_402ea0;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd4));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                                if t {
                                    break 'b_402ea0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = (c.g[S3][0] < 0x5u64) as u64;
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00402e70);
                                        }
                                        continue 'l_402e70;
                                    } else {
                                        break 'b_402ea0;
                                    }
                                }
                            }
                        }
                    }
                }
                'b_402f48: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        break 'b_402f48;
                    } else {
                        'b_402f28: {
                            'b_402f04: {
                                'b_402ed4: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7cb0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00402ec0 => {
                                            c.cycles += 5;
                                            c.g[RA][0] = sx(0x00402ec8);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x00403240,
                                                f: ee::s15::libmpeg_FlushPendingPicture,
                                            }
                                            .call(c, 0x00402ec8)?;
                                            c.g[V1][0] = 0x1;
                                            let t = true;
                                            c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                                            if t {
                                                break 'b_402f48;
                                            } else {
                                                break 'b_402ed4;
                                            }
                                        }
                                        0x00402ed4 => {
                                            break 'b_402ed4;
                                        }
                                        0x00402f04 => {
                                            break 'b_402f04;
                                        }
                                        0x00402f28 => {
                                            break 'b_402f28;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00402e10, 0x00402f80);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), 0u64);
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), 0u64);
                                c.g[A1][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), 0u64);
                                c.g[RA][0] = sx(0x00402ef0);
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x94));
                                Callee {
                                    addr: 0x00403098,
                                    f: ee::s15::libmpeg_DecodePicture,
                                }
                                .call(c, 0x00402ef0)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                                if t {
                                    break 'b_402f48;
                                } else {
                                    break 'b_402f04;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00402f14);
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                            Callee {
                                addr: 0x00403098,
                                f: ee::s15::libmpeg_DecodePicture,
                            }
                            .call(c, 0x00402f14)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
                            if t {
                                break 'b_402f48;
                            } else {
                                break 'b_402f28;
                            }
                        }
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00402f38);
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                        Callee {
                            addr: 0x00403098,
                            f: ee::s15::libmpeg_DecodePicture,
                        }
                        .call(c, 0x00402f38)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
                        break 'b_402f48;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x820));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_402f64;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00402e68);
                        }
                        continue 'l_402e68;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_402f64;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void libmpeg_AddPredAndStore(Mpeg *m, int blk)` (0x00404b78): MPEG block
/// reconstruction. Runs the two per-component function-pointer tables of
/// block `blk` (stride 0x140) `count` times, reports an error if the block
/// has both flags set (`libmpeg_Error`), then adds the prediction (second
/// source) to the 24 residual quad-words (or just clamps them, without a
/// prediction) with packed halfword add/min/max and stores them as packed
/// bytes. Converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn libmpeg_add_pred_and_store(c: &mut Cpu) {
    if c.pc != 0x00404b78 {
        return ee::s15::f_00404b78__generated(c);
    }
    guest(c, |c| {
        'b_404c7c: {
            'b_404c78: {
                'b_404c6c: {
                    'b_404c5c: {
                        c.cycles += 19;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                        c.g[V0][0] = 0x140;
                        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                        c.mult(0, 5, 5, 2);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
                        c.g[S4][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6c8));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6bc));
                        if t {
                            break 'b_404c5c;
                        } else {
                            c.cycles += 5;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            let t = (c.g[V1][0] as i64) <= 0;
                            c.g[S5][0] = 0u64;
                            if t {
                                break 'b_404c6c;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                                c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5b8));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c8));
                                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                                'l_404bf0: loop {
                                    c.cycles += 25;
                                    c.g[S1][0] = 0x140;
                                    c.g[S3][0] = 0x1c;
                                    c.g[S0][0] = sx((c.g[S5][0] as u32) << 2);
                                    c.mult(0, 17, 2, 17);
                                    c.mult(1, 19, 21, 19);
                                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x590));
                                    c.g[V0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    c.g[S2][0] =
                                        sx((c.g[S4][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00404c28);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00404c28)?;
                                    c.g[S0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xb8));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00404c3c);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                                    call_ptr(c, t, 0x00404c3c)?;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[S1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00404bf0);
                                        }
                                        continue 'l_404bf0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                                        if t {
                                            break 'b_404c7c;
                                        } else {
                                            break 'b_404c5c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                    c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        break 'b_404c78;
                    } else {
                        break 'b_404c6c;
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6c0));
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b8));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_404c78;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32));
            break 'b_404c7c;
        }
        'b_404cb0: {
            c.cycles += 7;
            c.g[V0][0] = 0x140;
            c.mult(0, 4, 3, 2);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if t {
                break 'b_404cb0;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6cc));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = 0x490000;
                if t {
                    break 'b_404cb0;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00404cb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e98));
                    Callee {
                        addr: 0x004034a8,
                        f: ee::s15::libmpeg_Error,
                    }
                    .call(c, 0x00404cb0)?;
                    break 'b_404cb0;
                }
            }
        }
        'b_405ae8: {
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32));
            c.g[V0][0] = 0x140;
            c.mult(0, 5, 3, 2);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if t {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6cc));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                if t {
                    c.cycles += 16;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x81c));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x594));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.cycles += 4;
                    c.g[T4][0] = 0x18;
                    c.g[T2][0] = 0x400000;
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b30));
                    c.lq(11, (c.g[T2][0] as u32));
                    'l_405a98: loop {
                        c.cycles += 17;
                        c.lq(8, (c.g[A1][0] as u32));
                        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
                        c.lq(13, (c.g[A2][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.lq(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.paddh(8, 8, 13);
                        c.lq(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.pminh(8, 8, 11);
                        c.paddh(9, 9, 2);
                        c.pmaxh(8, 8, 0);
                        c.pminh(9, 9, 11);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        c.pmaxh(9, 9, 0);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        c.ppacb(10, 9, 8);
                        let t = c.g[T4][0] != 0u64;
                        c.sq(10, (c.g[A0][0] as u32).wrapping_sub(0x10));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00405a98);
                            }
                            continue 'l_405a98;
                        } else {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.pc = t;
                            return Ok(());
                        }
                    }
                } else {
                    c.cycles += 14;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x81c));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    break 'b_405ae8;
                }
            } else {
                c.cycles += 16;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x594));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_405ae8;
            }
        }
        c.cycles += 4;
        c.g[T4][0] = 0x18;
        c.g[T2][0] = 0x400000;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b30));
        c.lq(11, (c.g[T2][0] as u32));
        'l_405af8: loop {
            c.cycles += 12;
            c.lq(8, (c.g[A1][0] as u32));
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
            c.pminh(8, 8, 11);
            c.lq(9, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.pmaxh(8, 8, 0);
            c.pminh(9, 9, 11);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            c.pmaxh(9, 9, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            c.ppacb(10, 9, 8);
            let t = c.g[T4][0] != 0u64;
            c.sq(10, (c.g[A0][0] as u32).wrapping_sub(0x10));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00405af8);
                }
                continue 'l_405af8;
            } else {
                c.cycles += 8;
                let t = c.g[RA][0] as u32;
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word, result mailboxes and call counters of this module's fakes.
    pub(crate) const OC1108_CTRL: u32 = DATA + 0xf_e000;
    pub(crate) const OC1108_MAGIC: u32 = 0x4d50_3147;
    /// Header types the `libmpeg_GetHeader` fake returns, in call order.
    pub(crate) const OC1108_HDR: u32 = OC1108_CTRL + 0x100;
    /// Results of the `libmpeg_DecodePicture` fake, in call order.
    pub(crate) const OC1108_DEC: u32 = OC1108_CTRL + 0x200;

    pub(crate) static OC1108_PREV_ERRORF: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1108_PREV_HDR: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1108_PREV_FLUSH: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1108_PREV_DEC: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1108_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);

    /// Log the call; the header and decode fakes answer from the mailboxes
    /// (the list is exhausted: 0), the decode fake may set `d[520]`.
    pub(crate) fn oc1108_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let (counter, list) = match addr {
            0x0040_7520 => (OC1108_CTRL + 0x20, OC1108_HDR),
            0x0040_3098 => (OC1108_CTRL + 0x24, OC1108_DEC),
            _ => return,
        };
        let n = c.bus.read32(counter);
        c.bus.write32(counter, n + 1);
        let v = if n < 8 { c.bus.read32(list + 4 * n) } else { 0 };
        c.g[V0][0] = sx(v);
        if addr == 0x0040_3098 && n == c.bus.read32(OC1108_CTRL + 0x30) {
            let dec = c.bus.read32(OC1108_CTRL + 0x34);
            c.bus.write32(dec + 0x820, 1);
        }
    }

    macro_rules! oc1108_gated {
        ($name:ident, $addr:expr, $prev:ident, $gen:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1108_CTRL) != OC1108_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1108_fake(c, $addr)
            }
        };
    }
    oc1108_gated!(
        oc1108_fake_errorf,
        0x0040_3470,
        OC1108_PREV_ERRORF,
        ee::s15::f_00403470__generated
    );
    oc1108_gated!(
        oc1108_fake_hdr,
        0x0040_7520,
        OC1108_PREV_HDR,
        ee::s15::f_00407520__generated
    );
    oc1108_gated!(
        oc1108_fake_flush,
        0x0040_3240,
        OC1108_PREV_FLUSH,
        ee::s15::f_00403240__generated
    );
    oc1108_gated!(
        oc1108_fake_dec,
        0x0040_3098,
        OC1108_PREV_DEC,
        ee::s15::f_00403098__generated
    );
    oc1108_gated!(
        oc1108_fake_error,
        0x0040_34a8,
        OC1108_PREV_ERROR,
        ee::s15::f_004034a8__generated
    );

    pub(crate) fn oc1108_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403470),
            oc1108_fake_errorf,
            &OC1108_PREV_ERRORF,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00407520),
            oc1108_fake_hdr,
            &OC1108_PREV_HDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403240),
            oc1108_fake_flush,
            &OC1108_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00403098),
            oc1108_fake_dec,
            &OC1108_PREV_DEC,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1108_fake_error,
            &OC1108_PREV_ERROR,
        );
    }

    pub(crate) const OC1108_MPEG: u32 = DATA + 0x2000;
    pub(crate) const OC1108_DECODER: u32 = DATA + 0x3000;

    /// `GetPictureCommon`: the error exit (`d[54] & 0x3f`), header loops
    /// (types 0-5, matching counters, `d[530]` set), decodes returning -1 or 0
    /// (-1 re-dispatches the same type), the exit through `d[520]` set by a
    /// decode, and unwinds in the fakes.
    #[test]
    fn libmpeg_get_picture_common_matches() {
        oc1108_install_fakes();
        check(
            ee::s15::f_00402e10__generated,
            libmpeg_get_picture_common,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x402e10, 5);
                elf_image(c);
                c.bus.write32(OC1108_CTRL, OC1108_MAGIC);
                c.bus.write32(OC1108_MPEG + 0x40, OC1108_DECODER);
                let d = OC1108_DECODER;
                let bad = seed % 11 == 0;
                c.bus
                    .write32(d + 0xd8, if bad { 0x41 } else { 0x40 * (seed % 2) });
                let counter = r.next_u32() % 3;
                c.bus.write32(d + 0xd4, counter);
                c.bus
                    .write32(d + 0x174, if seed % 4 == 0 { counter } else { counter + 1 });
                c.bus.write32(d + 0x848, (seed % 3 == 0) as u32);
                for k in 0..3u32 {
                    c.bus.write32(d + 0x94 + 4 * k, r.next_u32());
                }
                for n in 0..8u32 {
                    let t = if n < 1 + seed % 6 {
                        r.next_u32() % 6
                    } else {
                        0
                    };
                    c.bus.write32(OC1108_HDR + 4 * n, t);
                    let res = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        0
                    };
                    c.bus.write32(OC1108_DEC + 4 * n, res);
                }
                c.bus.write32(
                    OC1108_CTRL + 0x30,
                    if seed % 5 == 0 { r.next_u32() % 3 } else { 99 },
                );
                c.bus.write32(OC1108_CTRL + 0x34, d);
                c.g[A0][0] = sx(OC1108_MPEG);
                c.pc = 0x0040_2e10;
            },
        );
    }

    pub(crate) const OC1108_SRC: u32 = DATA + 0x6000;
    pub(crate) const OC1108_PRED: u32 = DATA + 0x7000;
    pub(crate) const OC1108_DST: u32 = DATA + 0x8000;

    /// `AddPredAndStore`: blocks 0-2, no / some table calls (counts -1, 0, 1-4
    /// through the fake method address), both flag combinations (error report)
    /// and the clamp-only against the add-prediction tails.
    #[test]
    fn libmpeg_add_pred_and_store_matches() {
        oc1108_install_fakes();
        check(
            ee::s15::f_00404b78__generated,
            libmpeg_add_pred_and_store,
            400,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x404b78, 9);
                elf_image(c);
                c.bus.write32(OC1108_CTRL, OC1108_MAGIC);
                let m = OC1108_MPEG;
                let idx = seed % 3;
                let blk = 0x140 * idx;
                for b in 0..3u32 {
                    let o = 0x140 * b;
                    for k in 0..4u32 {
                        c.bus.write32(m + 0x5b8 + o + 4 * k, fakes::FAKE + 4 * k);
                        c.bus
                            .write32(m + 0x5c8 + o + 4 * k, fakes::FAKE + 0x10 + 4 * k);
                    }
                    let count = [0xffff_ffffu32, 0, 1, 3, 4][(r.next_u32() % 5) as usize];
                    c.bus.write32(m + 0x6bc + o, count);
                    c.bus.write32(m + 0x6c8 + o, (seed / 3 % 2) as u32);
                    c.bus.write32(m + 0x6c0 + o, (seed / 6 % 2) as u32);
                    c.bus.write32(m + 0x6cc + o, (r.next_u32() % 2) as u32);
                    c.bus.write32(m + 0x594 + o, OC1108_PRED);
                    c.bus.write32(m + 0x6b8 + o, OC1108_DST);
                }
                c.bus.write32(m + 0x81c, OC1108_SRC);
                for k in 0..0xc0u32 {
                    c.bus.write32(OC1108_SRC + 4 * k, r.next_u32());
                    c.bus.write32(OC1108_PRED + 4 * k, r.next_u32());
                }
                let _ = blk;
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(idx);
                c.pc = 0x0040_4b78;
            },
        );
    }
}
