//! Staging module oc1110 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1110`.

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
    (0x003d_93c0, spch_assign_variations, "SPCH_AssignVariations"),
    (
        0x0040_3620,
        lib_mpeg_sequence_header,
        "libmpeg_SequenceHeader",
    ),
    (
        0x0040_3e00,
        lib_mpeg_setup_macroblock,
        "libmpeg_SetupMacroblock",
    ),
    (
        0x0040_4758,
        lib_mpeg_setup_pred_block,
        "libmpeg_SetupPredBlock",
    ),
    (0x0040_5b70, lib_mpeg_wait_bdec, "libmpeg_WaitBdec"),
];

/// `int SPCH_AssignVariations(Phrase* phrase, SelectionSet* sel, int thr)`
/// (0x3d93c0): picks one sample variation per entry of a speech phrase. Every
/// entry's current choice (`+6`) starts at its first one (`+4`); a combination
/// is accepted when its sample-usage percentage (the sizes `SPCH_DecodeSampleEntry`
/// reports, times 100 over the global total at `gp+0x1748`) stays within `thr`
/// (0 = unchecked) and, for a phrase with sentence parts (`phrase[5]`),
/// `SPCH_BuildSentencePart` accepts it. Otherwise the choices advance like an
/// odometer (last entry first, each wrapping to its first choice); the search
/// ends with the accepted flag, or 0 once every combination was tried.
pub fn spch_assign_variations(c: &mut Cpu) {
    guest(c, |c| {
        'b_3d9430: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[FP][0] = 0u64;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S7][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S6][0] = c.g[A0][0];
            c.sq(31, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A1][0];
            if t {
                break 'b_3d9430;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[S5][0];
                'l_3d9410: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d9410);
                        }
                        continue 'l_3d9410;
                    } else {
                        c.cycles += 1;
                        break 'b_3d9430;
                    }
                }
            }
        }
        'l_3d9430: loop {
            'b_3d94c4: {
                c.cycles += 2;
                let t = c.g[S7][0] == 0u64;
                c.g[S4][0] = 0x1;
                if t {
                    break 'b_3d94c4;
                } else {
                    'b_3d9498: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                        let t = c.g[V0][0] == 0u64;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_3d9498;
                        } else {
                            c.cycles += 4;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x62));
                            c.g[S0][0] = c.g[S5][0];
                            'l_3d9458: loop {
                                'b_3d948c: {
                                    c.cycles += 12;
                                    c.g[V0][0] = c.lh((c.g[S0][0] as u32));
                                    c.g[A2][0] = c.g[SP][0];
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V1][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x003d9480);
                                    c.g[A1][0] = c.lbu((c.g[V1][0] as u32));
                                    Callee {
                                        addr: 0x003db838,
                                        f: ee::s14::SPCH_DecodeSampleEntry,
                                    }
                                    .call(c, 0x003d9480)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                                    if t {
                                        break 'b_3d948c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[S3][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        break 'b_3d948c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S1][0] != 0u64;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d9458);
                                    }
                                    continue 'l_3d9458;
                                } else {
                                    break 'b_3d9498;
                                }
                            }
                        }
                    }
                    'b_3d94bc: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1748));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_3d94bc;
                        } else {
                            'b_3d94b0: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x64;
                                if c.g[V1][0] == 0u64 {
                                    break 'b_3d94b0;
                                } else {
                                    break 'b_3d94b0;
                                }
                            }
                            c.cycles += 3;
                            c.mult(0, 2, 19, 2);
                            c.div(0, 2, 3);
                            c.g[V0][0] = c.lo[0];
                            break 'b_3d94bc;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[S4][0] = 0u64;
                    }
                    break 'b_3d94c4;
                }
            }
            'b_3d9574: {
                'b_3d94f0: {
                    c.cycles += 2;
                    if c.g[S4][0] == 0u64 {
                        c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
                        break 'b_3d94f0;
                    } else {
                        'b_3d94e4: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x5));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_3d94e4;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x003d94e0);
                                c.g[A1][0] = c.g[S6][0];
                                Callee {
                                    addr: 0x003d8e58,
                                    f: ee::s14::SPCH_BuildSentencePart,
                                }
                                .call(c, 0x003d94e0)?;
                                if c.g[V0][0] == 0 {
                                    c.g[S4][0] = 0u64;
                                }
                                break 'b_3d94e4;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S4][0] != 0u64;

                        if t {
                            break 'b_3d9574;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lbu((c.g[S6][0] as u32).wrapping_add(0x4));
                            break 'b_3d94f0;
                        }
                    }
                }
                c.cycles += 12;
                c.g[T1][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x5));
                c.g[A1][0] = c.g[A0][0];
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6));
                'l_3d9520: loop {
                    'b_3d9568: {
                        'b_3d953c: {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = c.g[V0][0] & 0xff;
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.sb((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                            if t {
                                break 'b_3d953c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[T0][0] = 0x1;
                                if t {
                                    break 'b_3d9568;
                                } else {
                                    break 'b_3d953c;
                                }
                            }
                        }
                        'b_3d9558: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x8));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[A3][0] as i64) >= 0;
                            c.sb((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                            if t {
                                break 'b_3d9558;
                            } else {
                                c.cycles += 2;
                                c.g[T0][0] = 0x1;
                                c.g[T1][0] = 0x1;
                                break 'b_3d9558;
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
                        c.g[A0][0] = c.g[A1][0];
                        c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        break 'b_3d9568;
                    }
                    c.cycles += 2;
                    if c.g[T0][0] == 0u64 {
                        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x6));
                        if event_due(c) {
                            deliver_event_here(c, 0x003d9520);
                        }
                        continue 'l_3d9520;
                    } else {
                        c.cycles += 1;
                        c.g[FP][0] = c.g[T1][0];
                        break 'b_3d9574;
                    }
                }
            }
            'b_3d9584: {
                c.cycles += 2;
                let t = c.g[FP][0] != 0u64;
                c.g[V0][0] = c.g[S4][0];
                if t {
                    break 'b_3d9584;
                } else {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d9430);
                        }
                        continue 'l_3d9430;
                    } else {
                        break 'b_3d9584;
                    }
                }
            }
            c.cycles += 12;
            c.lq(31, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `libmpeg_SequenceHeader(dec)` (0x403620): parses an MPEG sequence header
/// from the bitstream: width/height (`dec+0x124/0x128`, `Error` when the width
/// field is 2801 or more), frame-rate and bit-rate words (`+0x134/0x138`), then
/// the intra and non-intra quantiser matrices (loaded from the stream through
/// the IPU, or the default tables), and the extension/user data. Then it
/// falls into the frame-buffer set-up of the decoder context at
/// `dec[0x858]->[0x40]`: macroblock dimensions (`+0x12c/0x130`) and, when the
/// picture size changed, the work-area allocations, `SetupFrameBuffers` and
/// the `SetPictureSize` calls (the last one a tail call).
pub fn lib_mpeg_sequence_header(c: &mut Cpu) {
    guest(c, |c| {
        'b_40366c: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[A1][0] = 0x20;
            c.sd((c.g[SP][0] as u32), c.g[S0][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x0040363c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xd4), 0u64);
            Callee {
                addr: 0x004072e8,
                f: ee::s15::libmpeg_GetBits,
            }
            .call(c, 0x0040363c)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[V0][0] = c.g[V0][0] & 0xfff;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 20);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x124), c.g[V1][0]);
            c.g[A0][0] = ((c.g[V0][0] as i64) < 2801i64) as u64;
            let t = c.g[A0][0] != 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x128), c.g[V0][0]);
            if t {
                break 'b_40366c;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0040366c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7cf8));
                Callee {
                    addr: 0x004034a8,
                    f: ee::s15::libmpeg_Error,
                }
                .call(c, 0x0040366c)?;
                break 'b_40366c;
            }
        }
        'b_4036e0: {
            'b_4036c8: {
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00403678);
                c.g[A1][0] = 0x1e;
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x00403678)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
                c.g[A1][0] = 0x1;
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 12);
                c.g[V0][0] = c.g[V0][0] & 0x3ff;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V1][0]);
                c.g[RA][0] = sx(0x0040369c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x138), c.g[V0][0]);
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x0040369c)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x840), c.g[V0][0]);
                if t {
                    break 'b_4036c8;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x004036ac);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036ac)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x004036b8);
                    c.g[A1][0] = 0x50000000;
                    Callee {
                        addr: 0x00406df0,
                        f: ee::s15::libmpeg_IpuCommand,
                    }
                    .call(c, 0x004036b8)?;
                    c.g[RA][0] = sx(0x004036c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036c0)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_4036e0;
                    } else {
                        break 'b_4036c8;
                    }
                }
            }
            c.cycles += 6;
            c.g[A2][0] = 0x450000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3200));
            c.g[RA][0] = sx(0x004036dc);
            c.g[A1][0] = 0x50000000;
            Callee {
                addr: 0x00403ad0,
                f: ee::s15::libmpeg_LoadIqMatrix,
            }
            .call(c, 0x004036dc)?;
            c.g[A0][0] = c.g[S0][0];
            break 'b_4036e0;
        }
        'b_403728: {
            'b_403714: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x004036e8);
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x004072e8,
                    f: ee::s15::libmpeg_GetBits,
                }
                .call(c, 0x004036e8)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x844), c.g[V0][0]);
                if t {
                    break 'b_403714;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x004036f8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x004036f8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00403704);
                    c.g[A1][0] = 0x58000000;
                    Callee {
                        addr: 0x00406df0,
                        f: ee::s15::libmpeg_IpuCommand,
                    }
                    .call(c, 0x00403704)?;
                    c.g[RA][0] = sx(0x0040370c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00406e20,
                        f: ee::s15::libmpeg_IpuWaitBusy,
                    }
                    .call(c, 0x0040370c)?;
                    let t = true;

                    if t {
                        break 'b_403728;
                    } else {
                        break 'b_403714;
                    }
                }
            }
            c.cycles += 5;
            c.g[A2][0] = 0x450000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3240));
            c.g[RA][0] = sx(0x00403728);
            c.g[A1][0] = 0x58000000;
            Callee {
                addr: 0x00403ad0,
                f: ee::s15::libmpeg_LoadIqMatrix,
            }
            .call(c, 0x00403728)?;
            break 'b_403728;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x00403730);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x004076f8,
            f: ee::s15::libmpeg_ExtensionAndUserData,
        }
        .call(c, 0x00403730)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        'b_4037b0: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
            c.g[A1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[S7][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[FP][0]);
            c.g[FP][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x848));
            if c.g[A2][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
                break 'b_4037b0;
            } else {
                c.cycles += 10;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x5;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x174), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x124));
                break 'b_4037b0;
            }
        }
        'b_4037e8: {
            'b_4037e0: {
                'b_4037dc: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    let t = c.g[A2][0] == 0u64;
                    c.sw((c.g[FP][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
                    if t {
                        break 'b_4037dc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x13c));
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
                        if t {
                            break 'b_4037e0;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                            let t = true;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            if t {
                                break 'b_4037e8;
                            } else {
                                break 'b_4037dc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x128));
                break 'b_4037e0;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
            break 'b_4037e8;
        }
        'b_403810: {
            c.cycles += 7;
            c.sw((c.g[FP][0] as u32).wrapping_add(0x130), c.g[V0][0]);
            c.g[S6][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x12c));
            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
            c.g[S7][0] = sx((c.g[V0][0] as u32) << 4);
            let t = c.g[S7][0] != c.g[V1][0];
            c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
            if t {
                break 'b_403810;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                let t = c.g[S6][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x528));
                if t {
                    c.cycles += 12;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
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
                    return Ok(());
                } else {
                    break 'b_403810;
                }
            }
        }
        c.cycles += 108;
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[S6][0]);
        c.g[S0][0] = 0x180;
        c.sw((c.g[A1][0] as u32), c.g[S7][0]);
        c.mult(0, 16, 22, 16);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x108));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x320));
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
        c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1e8));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x388));
        c.mult(0, 16, 23, 16);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.g[S4][0] = sx((c.g[FP][0] as u32).wrapping_add(0x250));
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3f0));
        c.g[S5][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2b8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x458));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4c0));
        c.g[RA][0] = sx(0x00403870);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        Callee {
            addr: 0x00402d78,
            f: ee::s15::libmpeg_WorkAreaRestore,
        }
        .call(c, 0x00403870)?;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00403884);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x00403884)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0040389c);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x0040389c)?;
        c.sw((c.g[FP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[RA][0] = sx(0x004038b4);
        c.g[A3][0] = 0x40;
        Callee {
            addr: 0x00402d88,
            f: ee::s15::libmpeg_WorkAreaAlloc,
        }
        .call(c, 0x004038b4)?;
        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.g[S3][0];
        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.g[S4][0];
        c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
        c.g[A2][0] = c.g[S5][0];
        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.sw((c.g[FP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xfc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x100));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x104));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[S6][0]);
        c.g[RA][0] = sx(0x00403904);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        Callee {
            addr: 0x004039f0,
            f: ee::s15::libmpeg_SetupFrameBuffers,
        }
        .call(c, 0x00403904)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403914);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403914)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403924);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403924)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403934);
        c.g[A2][0] = c.g[S6][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403934)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403944);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403944)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403954);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403954)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403964);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403964)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403974);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403974)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x00403984);
        c.g[A2][0] = c.g[S2][0];
        Callee {
            addr: 0x00403600,
            f: ee::s15::libmpeg_SetPictureSize,
        }
        .call(c, 0x00403984)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = 0x00403600;
        ee::s15::libmpeg_SetPictureSize(c);
        return Ok(());
    });
}

/// `int libmpeg_SetupMacroblock(dec, mbAddr, b, flags, mtype)` (0x403e00): splits
/// the macroblock address by the picture width in macroblocks (`dec+0x12c`)
/// into column and row, waits for the IPU-side SPR DMA (`0x1000d400` bit 8),
/// and for a non-intra block (`flags & 1` clear) forms the predictions
/// (`Errorf` and 0 for a bad motion type 0 or above 3, else `FormPredictions`,
/// the DMA wait again, `FetchRefBlocks`). Then it records the per-buffer
/// entry of `dec[0x6b8..0x6c8 + cur * 0x140]`: done flag, two flags and the
/// macroblock's buffer address in the current frame (picture structure
/// `dec+0x174`: 3 frame, 2 the `+0x1e0` field buffer, else `+0x1d0`).
pub fn lib_mpeg_setup_macroblock(c: &mut Cpu) {
    guest(c, |c| {
        'b_403e50: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
            c.g[FP][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
            c.g[S5][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
            c.g[V1][0] = c.g[S5][0] & 0x1;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
            c.div(0, 5, 2);
            if c.g[V0][0] == 0u64 {
                break 'b_403e50;
            } else {
                break 'b_403e50;
            }
        }
        'b_404020: {
            'b_403f44: {
                'b_403eb0: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[V1][0] = c.hi[0];
                    c.g[S7][0] = c.lo[0];
                    c.g[S6][0] = c.g[V1][0];
                    c.g[A2][0] = sx((c.g[S7][0] as u32) << 4);
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S6][0] as u32) << 4);
                    if t {
                        break 'b_403eb0;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
                        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
                        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
                        'l_403e80: loop {
                            c.cycles += 7;
                            c.g[V0][0] = 0x10000000;
                            c.g[V0][0] = c.g[V0][0] | 0xd400;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
                            c.g[V1][0] = c.g[V1][0] & 0x1;
                            let t = c.g[V1][0] != 0u64;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00403e80);
                                }
                                continue 'l_403e80;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x140;
                                c.mult(0, 3, 4, 2);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                let t = true;
                                c.sw((c.g[V0][0] as u32), 0u64);
                                if t {
                                    break 'b_403f44;
                                } else {
                                    break 'b_403eb0;
                                }
                            }
                        }
                    }
                }
                'b_403ee4: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_403ee4;
                    } else {
                        c.cycles += 9;
                        c.g[A1][0] = 0x490000;
                        c.g[A2][0] = c.g[T0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e10));
                        c.g[RA][0] = sx(0x00403ed4);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00403470,
                            f: ee::s15::libmpeg_Errorf,
                        }
                        .call(c, 0x00403ed4)?;
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = 0u64;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                        if t {
                            break 'b_404020;
                        } else {
                            break 'b_403ee4;
                        }
                    }
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x00403eec);
                c.g[A3][0] = c.g[S5][0];
                Callee {
                    addr: 0x00404050,
                    f: ee::s15::libmpeg_FormPredictions,
                }
                .call(c, 0x00403eec)?;
                c.g[V1][0] = 0x10000000;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c4));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b8));
                c.g[V1][0] = c.g[V1][0] | 0xd400;
                'l_403f08: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00403f08);
                        }
                        continue 'l_403f08;
                    } else {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x00403f2c);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00408988,
                            f: ee::s15::libmpeg_FetchRefBlocks,
                        }
                        .call(c, 0x00403f2c)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[V1][0] = 0x140;
                        c.g[A0][0] = 0x1;
                        c.mult(0, 5, 2, 3);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                        break 'b_403f44;
                    }
                }
            }
            'b_403f84: {
                'b_403f74: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[FP][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        break 'b_403f74;
                    } else {
                        'b_403f70: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S5][0] & 0x2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x140;
                            if t {
                                break 'b_403f70;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                                c.mult(0, 4, 2, 3);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                let t = true;
                                c.sw((c.g[V0][0] as u32), c.g[FP][0]);
                                if t {
                                    break 'b_403f84;
                                } else {
                                    break 'b_403f70;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        break 'b_403f74;
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = 0x140;
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.sw((c.g[V0][0] as u32), 0u64);
                break 'b_403f84;
            }
            'b_40401c: {
                'b_403fe0: {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                    c.g[A3][0] = 0x140;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                    c.g[V1][0] = 0x3;
                    c.mult(0, 4, 2, 7);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_403fe0;
                    } else {
                        c.cycles += 13;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                        c.g[A2][0] = 0x180;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                        c.mult(0, 3, 4, 7);
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                        c.mult(0, 5, 22, 2);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                        c.mult(0, 2, 2, 6);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        let t = true;
                        c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                        if t {
                            break 'b_40401c;
                        } else {
                            break 'b_403fe0;
                        }
                    }
                }
                'b_403fec: {
                    c.cycles += 2;
                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d0));
                        break 'b_403fec;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e0));
                        break 'b_403fec;
                    }
                }
                c.cycles += 12;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = 0x180;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                c.g[A1][0] = 0x140;
                c.mult(0, 7, 22, 3);
                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                c.mult(0, 7, 4, 5);
                c.mult(0, 3, 3, 6);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                break 'b_40401c;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_404020;
        }
        c.cycles += 12;
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
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `libmpeg_SetupPredBlock(dec, ref, a2, a3, t0, h, bx, by, dx, dy, fld, avg)`
/// (0x404758): appends the luma and the chroma prediction block descriptors
/// for one motion vector (`dx`, `dy` in half pixels, at the stack) to the
/// current buffer entry of `dec` (`dec[0x6bc + cur * 0x140]` holds the count
/// `k`): block address inside the reference picture, the fraction-dependent
/// row split (`+8`/`+0xc` rows before/after the macroblock edge), buffer
/// pointers, and the interpolation-routine selectors from the tables at
/// `0x453280`/`0x4532a0`. Leaf, 8 stack arguments.
pub fn lib_mpeg_setup_pred_block(c: &mut Cpu) {
    'b_404818: {
        'b_404810: {
            c.cycles += 42;
            c.g[T8][0] = c.g[A0][0];
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x6bc));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[FP][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
            c.g[V1][0] = 0x140;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[FP][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.g[T6][0] = c.g[T1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S1][0] = c.g[A2][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[S0][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S7][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x81c));
            c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[A0][0] = 0x1c;
            c.g[T5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T2][0]);
            c.mult(0, 2, 2, 3);
            c.sw((c.g[SP][0] as u32), c.g[A3][0]);
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[T2][0] = sx(((c.g[S6][0] as u32) as i32 >> 1) as u32);
            c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb8));
            c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[S7][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x590));
            c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.mult(0, 4, 23, 4);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
            c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            let t = c.g[T5][0] == 0u64;
            c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            if t {
                break 'b_404810;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                let t = true;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                if t {
                    break 'b_404818;
                } else {
                    break 'b_404810;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        break 'b_404818;
    }
    'b_4048e0: {
        'b_4048dc: {
            'b_4048a8: {
                c.cycles += 22;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
                c.g[S4][0] = sx(((c.g[T0][0] as u32) as i32 >> 4) as u32);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
                c.mult(0, 5, 20, 5);
                c.g[S5][0] = sx(((c.g[A2][0] as u32) as i32 >> 4) as u32);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                c.sw((c.g[T4][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 4);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.sw((c.g[T4][0] as u32), c.g[A0][0]);
                c.g[T9][0] = c.g[S2][0] & 0x1;
                let t = c.g[T9][0] == 0u64;
                c.g[S3][0] = c.g[S6][0] & 0x1;
                if t {
                    break 'b_4048a8;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                        break 'b_4048dc;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0x10;
                        c.g[V1][0] =
                            sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] =
                            sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[T6][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        if t {
                            break 'b_4048e0;
                        } else {
                            break 'b_4048a8;
                        }
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T6][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                break 'b_4048dc;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x10;
                c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                let t = true;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                if t {
                    break 'b_4048e0;
                } else {
                    break 'b_4048dc;
                }
            }
        }
        c.cycles += 1;
        c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
        break 'b_4048e0;
    }
    'b_4049a8: {
        'b_404994: {
            c.cycles += 38;
            c.g[A1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
            c.g[V0][0] = 0x140;
            c.g[A3][0] = 0x600;
            c.g[A0][0] = sx((c.g[S3][0] as u32) << 1);
            c.mult(0, 3, 5, 2);
            c.mult(0, 7, 23, 7);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc8));
            c.g[T1][0] = sx((c.g[A2][0] as u32) << 4);
            c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x300));
            c.g[A2][0] = 0x10;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T8][0] as u32)));
            c.g[V0][0] = sx((c.g[S2][0] as u32) >> 31);
            c.g[V1][0] = sx((c.g[S6][0] as u32) >> 31);
            c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x590));
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[S3][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[T1][0] as u32)));
            c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[T2][0] as u32)));
            c.g[S6][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
            c.g[A0][0] = c.g[A3][0] | c.g[A0][0];
            c.g[A0][0] = c.g[A0][0] | c.g[T9][0];
            c.g[A3][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.sw((c.g[T4][0] as u32).wrapping_add(0x14), c.g[T1][0]);
            c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
            c.sw((c.g[T4][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[T4][0] as u32).wrapping_add(0x18), c.g[T2][0]);
            let t = c.g[T5][0] == 0u64;
            c.g[T1][0] = sx(((c.g[T6][0] as u32) as i32 >> 1) as u32);
            if t {
                break 'b_404994;
            } else {
                c.cycles += 7;
                c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
                c.g[A0][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                let t = true;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t {
                    break 'b_4049a8;
                } else {
                    break 'b_404994;
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
        c.g[V1][0] = sx(((c.g[T3][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        break 'b_4049a8;
    }
    'b_404a5c: {
        'b_404a58: {
            'b_404a24: {
                c.cycles += 17;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                c.g[T2][0] = sx(((c.g[A2][0] as u32) as i32 >> 3) as u32);
                c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[T2][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[S3][0] = c.g[S3][0] & 0x1;
                c.g[A3][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x200));
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                c.g[T9][0] = c.g[S6][0] & 0x1;
                c.sw((c.g[T7][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                let t = c.g[T9][0] == 0u64;
                c.sw((c.g[T7][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_404a24;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 8i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                        break 'b_404a58;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = 0x8;
                        c.g[V1][0] =
                            sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] =
                            sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = true;
                        c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        if t {
                            break 'b_404a5c;
                        } else {
                            break 'b_404a24;
                        }
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[T5][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 9i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                break 'b_404a58;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x8;
                c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.sw((c.g[T7][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                let t = true;
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                if t {
                    break 'b_404a5c;
                } else {
                    break 'b_404a58;
                }
            }
        }
        c.cycles += 1;
        c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
        break 'b_404a5c;
    }
    c.cycles += 70;
    c.g[V0][0] = 0x8;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[T5][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub((c.g[S5][0] as u32)));
    c.sw((c.g[T7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A1][0] = 0x140;
    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x450000;
    c.g[T0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x810));
    c.g[T2][0] = 0x180;
    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3280));
    c.mult(0, 8, 8, 5);
    c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.mult(0, 5, 4, 10);
    c.mult(0, 13, 12, 10);
    c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10));
    c.g[T4][0] = c.lw((c.g[V1][0] as u32));
    c.g[A3][0] = sx((c.g[S7][0] as u32) << 2);
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
    c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T3][0] as u32)));
    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x5b8), c.g[T4][0]);
    c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x400));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
    c.g[T0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x100));
    c.mult(0, 12, 11, 10);
    c.g[A1][0] = 0x450000;
    c.g[T1][0] = c.lw((c.g[FP][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x32a0));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.sw((c.g[T7][0] as u32).wrapping_add(0x14), c.g[A0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.sw((c.g[T7][0] as u32).wrapping_add(0x18), c.g[V1][0]);
    c.g[A0][0] = sx((c.g[T8][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[T1][0] as u32)));
    c.g[A1][0] = c.lw((c.g[T0][0] as u32));
    c.g[V1][0] = c.g[A0][0];
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x598), c.g[T1][0]);
    c.g[A3][0] = c.g[V1][0];
    c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
    c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x5c8), c.g[A2][0]);
    c.sw((c.g[A3][0] as u32).wrapping_add(0x5a8), c.g[T3][0]);
    c.sw((c.g[T0][0] as u32), c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = t;
    return;
}

/// `int libmpeg_WaitBdec(dec)` (0x405b70): waits for the IPU decode to finish.
/// Loops while the IPU-from DMA has quadwords queued (`0x1000b020`) and no
/// error is flagged in `IPU_CTRL` (`0x10002010`, ECD = bit 14), feeding the
/// stalled to-IPU DMA through the callback `dec[0x858]` (`DispatchCallback`,
/// command 1). With `dec[0x10a0]` set it aborts instead (interrupts off, DMA
/// registers cleared, `IPU_CTRL = 0x40000000`, returns 0). Otherwise it
/// stores `IPU_TOP` and the bit-position remainder (`dec+0x838/0x83c`) and,
/// when ECD is set, reports the error (`Error`, callbacks 2 and 3), resets the
/// DMA and returns 0; else 1.
pub fn lib_mpeg_wait_bdec(c: &mut Cpu) {
    guest(c, |c| {
        'b_405c38: {
            'b_405c34: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S2][0]);
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S6][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S5][0]);
                c.g[S6][0] = 0x1;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                c.g[RA][0] = sx(0x00405ba0);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S0][0]);
                Callee {
                    addr: 0x00406e20,
                    f: ee::s15::libmpeg_IpuWaitBusy,
                }
                .call(c, 0x00405ba0)?;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0xb020;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_405c34;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[V0][0] | 0x2010;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.g[V1][0] & 0x4000;
                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
                        break 'b_405c38;
                    } else {
                        c.cycles += 10;
                        c.g[S4][0] = 0x10000000;
                        c.g[S3][0] = 0x10000000;
                        c.g[S1][0] = 0x10000000;
                        c.g[S0][0] = 0x10000000;
                        c.g[S4][0] = c.g[S4][0] | 0xb420;
                        c.g[S3][0] = c.g[S3][0] | 0xb400;
                        c.g[S5][0] = 0x1;
                        c.g[S1][0] = c.g[S1][0] | 0xb020;
                        c.g[S0][0] = c.g[S0][0] | 0x2010;
                        'l_405bf0: loop {
                            'b_405c18: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_405c18;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                    c.g[V0][0] = c.g[V0][0] & 0x100;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[SP][0];
                                    if t {
                                        break 'b_405c18;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                                        c.g[RA][0] = sx(0x00405c18);
                                        c.sw((c.g[SP][0] as u32), c.g[S5][0]);
                                        Callee {
                                            addr: 0x00402c30,
                                            f: ee::s15::libmpeg_DispatchCallback,
                                        }
                                        .call(c, 0x00405c18)?;
                                        break 'b_405c18;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
                                break 'b_405c38;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                c.g[V0][0] = c.g[V0][0] & 0x4000;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00405bf0);
                                    }
                                    continue 'l_405bf0;
                                } else {
                                    break 'b_405c34;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10a0));
            break 'b_405c38;
        }
        'b_405de8: {
            'b_405cd8: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x10000000;
                if t {
                    break 'b_405cd8;
                } else {
                    'b_405ca8: {
                        c.cycles += 23;
                        c.g[RA][0] = sx(0x00405c48);
                        Callee {
                            addr: 0x0042c078,
                            f: ee::s15::DIntr,
                        }
                        .call(c, 0x00405c48)?;
                        c.g[A3][0] = 0x10000000;
                        c.g[A0][0] = 0x10000;
                        c.g[A3][0] = c.g[A3][0] | 0xf520;
                        c.g[T0][0] = 0x10000000;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                        c.g[T0][0] = c.g[T0][0] | 0xf590;
                        c.g[A1][0] = 0x10000000;
                        c.g[A2][0] = 0x10000000;
                        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                        c.g[A1][0] = c.g[A1][0] | 0xb400;
                        c.sw((c.g[T0][0] as u32), c.g[V1][0]);
                        c.g[A2][0] = c.g[A2][0] | 0xb000;
                        c.sw((c.g[A1][0] as u32), 0u64);
                        c.g[A0][0] = 0xfffffffffffe0000;
                        c.sw((c.g[A2][0] as u32), 0u64);
                        c.g[A0][0] = c.g[A0][0] | 0xffff;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                        c.sw((c.g[T0][0] as u32), c.g[V1][0]);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x10000000;
                        if t {
                            break 'b_405ca8;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x00405ca4);
                            Callee {
                                addr: 0x0042c0c0,
                                f: ee::s15::EIntr,
                            }
                            .call(c, 0x00405ca4)?;
                            c.g[V0][0] = 0x10000000;
                            break 'b_405ca8;
                        }
                    }
                    c.cycles += 12;
                    c.g[V1][0] = 0x10000000;
                    c.g[V0][0] = c.g[V0][0] | 0xb020;
                    c.g[V1][0] = c.g[V1][0] | 0xb420;
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A0][0] = 0x10000000;
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A0][0] = c.g[A0][0] | 0x2010;
                    c.g[V1][0] = 0x40000000;
                    c.g[V0][0] = 0u64;
                    c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        break 'b_405de8;
                    } else {
                        break 'b_405cd8;
                    }
                }
            }
            'b_405d18: {
                'b_405d14: {
                    c.cycles += 8;
                    c.g[A0][0] = 0x10000000;
                    c.g[A0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x2030));
                    c.g[V0][0] = c.g[V0][0] | 0x2020;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.g[A0][0] << 32;
                    c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x838), c.g[V1][0]);
                    if t {
                        break 'b_405d14;
                    } else {
                        'b_405d0c: {
                            c.cycles += 3;
                            c.g[V1][0] = c.g[V0][0] & 0x1f;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x20;
                            if t {
                                break 'b_405d0c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                if t {
                                    break 'b_405d18;
                                } else {
                                    break 'b_405d0c;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_405d18;
                        } else {
                            break 'b_405d14;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x20;
                break 'b_405d18;
            }
            'b_405de0: {
                c.cycles += 7;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x83c), c.g[V0][0]);
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x2010;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.g[V1][0] & 0x4000;
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = 0x490000;
                if t {
                    break 'b_405de0;
                } else {
                    'b_405dd4: {
                        c.cycles += 37;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00405d40);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7eb0));
                        Callee {
                            addr: 0x004034a8,
                            f: ee::s15::libmpeg_Error,
                        }
                        .call(c, 0x00405d40)?;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00405d58);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        Callee {
                            addr: 0x00402c30,
                            f: ee::s15::libmpeg_DispatchCallback,
                        }
                        .call(c, 0x00405d58)?;
                        c.g[V1][0] = 0x40000000;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x858));
                        c.g[AT][0] = 0x10000000;
                        c.sw((c.g[AT][0] as u32).wrapping_add(0x2010), c.g[V1][0]);
                        c.g[V0][0] = 0x3;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        c.g[RA][0] = sx(0x00405d78);
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x00402c30,
                            f: ee::s15::libmpeg_DispatchCallback,
                        }
                        .call(c, 0x00405d78)?;
                        c.g[RA][0] = sx(0x00405d80);
                        Callee {
                            addr: 0x0042c078,
                            f: ee::s15::DIntr,
                        }
                        .call(c, 0x00405d80)?;
                        c.g[A2][0] = 0x10000000;
                        c.g[A3][0] = 0x10000000;
                        c.g[A2][0] = c.g[A2][0] | 0xf520;
                        c.g[T0][0] = 0x10000;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        c.g[A3][0] = c.g[A3][0] | 0xf590;
                        c.g[A0][0] = 0x10000000;
                        c.g[A1][0] = 0xfffffffffffe0000;
                        c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
                        c.g[A0][0] = c.g[A0][0] | 0xb000;
                        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                        c.sw((c.g[A0][0] as u32), 0u64);
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x10000000;
                        if t {
                            break 'b_405dd4;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x00405dd0);
                            Callee {
                                addr: 0x0042c0c0,
                                f: ee::s15::EIntr,
                            }
                            .call(c, 0x00405dd0)?;
                            c.g[V0][0] = 0x10000000;
                            break 'b_405dd4;
                        }
                    }
                    c.cycles += 3;
                    c.g[S6][0] = 0u64;
                    c.g[V0][0] = c.g[V0][0] | 0xb020;
                    c.sw((c.g[V0][0] as u32), 0u64);
                    break 'b_405de0;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S6][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_405de8;
        }
        c.cycles += 9;
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP, SP as STACK_TOP};
    use std::ptr::addr_of_mut;

    /// Gate word of the oc1110 callee fakes (in the machine's RAM): with it
    /// unset every fake runs the body it replaced.
    const OC1110_CTRL: u32 = DATA + 0xf_8100;
    const OC1110_MAGIC: u32 = 0x1110_c0de;
    /// Mailbox words of the fakes (`DATA + 0xf_8110 + 4 * i`).
    const OC1110_MB: u32 = DATA + 0xf_8110;
    /// GetBits results: 32-bit header, 30-bit header, first and second 1-bit flag, 1-bit calls so far.
    const MB_HDR1: u32 = 0;
    const MB_HDR2: u32 = 1;
    const MB_FLAG1: u32 = 2;
    const MB_FLAG2: u32 = 3;
    const MB_BITS1: u32 = 4;
    /// WaitBdec callback fake: calls so far, the call that acts, what it does
    /// (0: clears the IPU-from DMA qwc, else raises ECD), and DIntr's result.
    const MB_CB_COUNT: u32 = 8;
    const MB_CB_AT: u32 = 9;
    const MB_CB_KIND: u32 = 10;
    const MB_DINTR: u32 = 12;
    /// AssignVariations fakes: BuildSentencePart result mask and counter,
    /// DecodeSampleEntry counter, size seed and result mask.
    const MB_BUILD_MASK: u32 = 16;
    const MB_BUILD_N: u32 = 17;
    const MB_DECODE_N: u32 = 18;
    const MB_DECODE_SEED: u32 = 19;
    const MB_DECODE_MASK: u32 = 20;

    fn mb_rd(c: &mut Cpu, i: u32) -> u32 {
        c.bus.read32(OC1110_MB + 4 * i)
    }

    fn mb_wr(c: &mut Cpu, i: u32, v: u32) {
        c.bus.write32(OC1110_MB + 4 * i, v);
    }

    /// Gated fakes that only log the call (all registers, cycles, target).
    macro_rules! oc1110_simple_fakes {
        ($(($fake:ident, $slot:ident, $gen:ident, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::s15::$gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1110_hook_simple() {
                $(fakes::hook(addr_of_mut!(ee::s15::$slot), $fake, fakes::prev_cell($fake));)*
            }
        };
    }

    oc1110_simple_fakes!(
        (
            oc1110_fake_error,
            OVERRIDE_004034A8,
            f_004034a8__generated,
            0x0040_34a8
        ),
        (
            oc1110_fake_ipu_wait,
            OVERRIDE_00406E20,
            f_00406e20__generated,
            0x0040_6e20
        ),
        (
            oc1110_fake_ipu_cmd,
            OVERRIDE_00406DF0,
            f_00406df0__generated,
            0x0040_6df0
        ),
        (
            oc1110_fake_load_iq,
            OVERRIDE_00403AD0,
            f_00403ad0__generated,
            0x0040_3ad0
        ),
        (
            oc1110_fake_ext_user,
            OVERRIDE_004076F8,
            f_004076f8__generated,
            0x0040_76f8
        ),
        (
            oc1110_fake_wa_restore,
            OVERRIDE_00402D78,
            f_00402d78__generated,
            0x0040_2d78
        ),
        (
            oc1110_fake_wa_alloc,
            OVERRIDE_00402D88,
            f_00402d88__generated,
            0x0040_2d88
        ),
        (
            oc1110_fake_setup_fb,
            OVERRIDE_004039F0,
            f_004039f0__generated,
            0x0040_39f0
        ),
        (
            oc1110_fake_set_pic_size,
            OVERRIDE_00403600,
            f_00403600__generated,
            0x0040_3600
        ),
        (
            oc1110_fake_errorf,
            OVERRIDE_00403470,
            f_00403470__generated,
            0x0040_3470
        ),
        (
            oc1110_fake_form_pred,
            OVERRIDE_00404050,
            f_00404050__generated,
            0x0040_4050
        ),
        (
            oc1110_fake_fetch_ref,
            OVERRIDE_00408988,
            f_00408988__generated,
            0x0040_8988
        ),
        (
            oc1110_fake_eintr,
            OVERRIDE_0042C0C0,
            f_0042c0c0__generated,
            0x0042_c0c0
        ),
    );

    /// `libmpeg_GetBits(dec, n)`: logs, then returns the mailbox value for `n`
    /// (32: header word, 30: second header word, 1: the two matrix flags).
    fn oc1110_fake_get_bits(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_get_bits),
                ee::s15::f_004072e8__generated,
            );
        }
        let bits = c.g[A1][0] as u32;
        fakes::fake_call(c, 0x0040_72e8);
        let v = match bits {
            0x20 => mb_rd(c, MB_HDR1),
            0x1e => mb_rd(c, MB_HDR2),
            _ => {
                let n = mb_rd(c, MB_BITS1);
                mb_wr(c, MB_BITS1, n + 1);
                mb_rd(c, MB_FLAG1 + (n & 1))
            }
        };
        c.g[V0][0] = v as u64;
    }

    /// `libmpeg_DispatchCallback(ctx, cmd)`: logs, counts the call and, on the
    /// mailbox's call, finishes the IPU wait: clears the from-IPU qwc, or
    /// raises ECD (an invalid VDEC) in `IPU_CTRL`.
    fn oc1110_fake_dispatch(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_dispatch),
                ee::s15::f_00402c30__generated,
            );
        }
        fakes::fake_call(c, 0x0040_2c30);
        let n = mb_rd(c, MB_CB_COUNT) + 1;
        mb_wr(c, MB_CB_COUNT, n);
        if n == mb_rd(c, MB_CB_AT) {
            if mb_rd(c, MB_CB_KIND) == 0 {
                c.bus.write32(0x1000_b020, 0);
            } else {
                c.bus.write32(0x1000_2000, 0x3400_0000);
            }
        }
    }

    /// `DIntr()`: logs, returns the mailbox's "interrupts were enabled" word.
    fn oc1110_fake_dintr(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_dintr),
                ee::s15::f_0042c078__generated,
            );
        }
        fakes::fake_call(c, 0x0042_c078);
        let v = mb_rd(c, MB_DINTR);
        c.g[V0][0] = v as u64;
    }

    /// `SPCH_DecodeSampleEntry(sample, idx, int* size)`: logs, stores a size
    /// (0..39) through a2 and returns a flag from the mailbox's mask.
    fn oc1110_fake_decode_entry(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_decode_entry),
                ee::s14::f_003db838__generated,
            );
        }
        let out = c.g[A2][0] as u32;
        fakes::fake_call(c, 0x003d_b838);
        let n = mb_rd(c, MB_DECODE_N);
        mb_wr(c, MB_DECODE_N, n + 1);
        let seed = mb_rd(c, MB_DECODE_SEED) as u64;
        let size = (fakes::mix(seed + n as u64) % 40) as u32;
        c.bus.write32(out, size);
        let mask = mb_rd(c, MB_DECODE_MASK);
        c.g[V0][0] = ((mask >> (n & 31)) & 1) as u64;
    }

    /// `SPCH_BuildSentencePart(sel, phrase)`: logs, returns a flag from the
    /// mailbox's mask.
    fn oc1110_fake_build_part(c: &mut Cpu) {
        if c.bus.read32(OC1110_CTRL) != OC1110_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1110_fake_build_part),
                ee::s14::f_003d8e58__generated,
            );
        }
        fakes::fake_call(c, 0x003d_8e58);
        let n = mb_rd(c, MB_BUILD_N);
        mb_wr(c, MB_BUILD_N, n + 1);
        let mask = mb_rd(c, MB_BUILD_MASK);
        c.g[V0][0] = ((mask >> (n & 31)) & 1) as u64;
    }

    fn oc1110_hook_all() {
        oc1110_hook_simple();
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004072E8),
            oc1110_fake_get_bits,
            fakes::prev_cell(oc1110_fake_get_bits),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1110_fake_dispatch,
            fakes::prev_cell(oc1110_fake_dispatch),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042C078),
            oc1110_fake_dintr,
            fakes::prev_cell(oc1110_fake_dintr),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DB838),
            oc1110_fake_decode_entry,
            fakes::prev_cell(oc1110_fake_decode_entry),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D8E58),
            oc1110_fake_build_part,
            fakes::prev_cell(oc1110_fake_build_part),
        );
    }

    /// A machine with the fakes on, the mailbox cleared.
    fn oc1110_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1110_hook_all();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1110_CTRL, OC1110_MAGIC);
        for i in 0..24 {
            mb_wr(c, i, 0);
        }
        r
    }

    const OC1110_DEC: u32 = DATA + 0x1000;

    // ------------------------------------------------------------------
    // SPCH_AssignVariations

    const AV_PHRASE: u32 = DATA + 0x1000;
    const AV_SEL: u32 = DATA + 0x2000;
    const AV_TABLE: u32 = DATA + 0x3000;

    fn av_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x3d93c0, 8);
        let n = 1 + seed % 3;
        fill(c, &mut r, AV_PHRASE, 8);
        fill(c, &mut r, AV_SEL, 0x60);
        fill(c, &mut r, AV_TABLE, 16);
        c.bus
            .write8(AV_PHRASE + 4, (n << 2 | r.next_u32() & 3) as u8);
        c.bus.write8(AV_PHRASE + 5, (seed / 3 % 2) as u8);
        for i in 0..n {
            let e = AV_SEL + 8 * i;
            c.bus.write16(e, r.next_u32() as u16 % 8);
            c.bus.write8(e + 4, (r.next_u32() % 4) as u8);
            c.bus.write8(e + 5, (1 + r.next_u32() % 3) as u8);
        }
        c.bus.write32(GAME_GP + 0x1734, AV_TABLE);
        let total = [0u32, 50, 200, 1000][(seed / 6 % 4) as usize];
        c.bus.write32(GAME_GP + 0x1748, total);
        let thr = [0u32, 5, 30, 100, 1000][(seed / 24 % 5) as usize];
        let mut all = |k: u32| if seed % k == 0 { !0 } else { r.next_u32() };
        let build = all(4);
        let decode = all(5);
        mb_wr(c, MB_BUILD_MASK, build);
        mb_wr(c, MB_DECODE_MASK, decode);
        mb_wr(c, MB_DECODE_SEED, r.next_u32());
        c.g[A0][0] = AV_PHRASE as u64;
        c.g[A1][0] = AV_SEL as u64;
        c.g[A2][0] = thr as u64;
    }

    /// Phrases of 1 to 3 entries with 1 to 3 choices each, with and without
    /// the usage limit and sentence parts: the odometer runs through every
    /// combination until one is accepted or all were tried; fakes unwind.
    #[test]
    fn spch_assign_variations_matches() {
        check_lohi(
            ee::s14::f_003d93c0__generated,
            spch_assign_variations,
            480,
            &av_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_SequenceHeader

    /// Sequence header bits and the previous picture size (`dec[0x858][0..8]`),
    /// the decoder context being `dec` itself (`dec[0x858][0x40] = dec`).
    fn seq_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x403620, 30);
        fill(c, &mut r, OC1110_DEC, 0x600);
        let b = DATA + 0x4000;
        fill(c, &mut r, b, 0x20);
        let height = 16 + r.next_u32() % 700;
        let w = if seed % 7 == 0 {
            2801 + r.next_u32() % 100
        } else {
            16 + r.next_u32() % 700
        };
        mb_wr(
            c,
            MB_HDR1,
            height << 20 | w << 8 | (r.next_u32() & 0xff) | 1,
        );
        mb_wr(c, MB_HDR2, r.next_u32() | 1);
        mb_wr(c, MB_FLAG1, seed / 2 % 2);
        mb_wr(c, MB_FLAG2, seed / 4 % 2);
        c.bus.write32(OC1110_DEC + 0x858, b);
        c.bus.write32(b + 0x40, OC1110_DEC);
        let flag848 = seed / 8 % 2;
        c.bus
            .write32(OC1110_DEC + 0x848, flag848 * (1 + r.next_u32() % 5));
        let f13c = if flag848 == 0 { 1 } else { seed / 16 % 2 };
        c.bus.write32(OC1110_DEC + 0x13c, f13c);
        // The size the header will give: S7 = mbw << 4, S6 = mbh << 4.
        let mbw = (height + 15) >> 4;
        let mbh = if f13c != 0 {
            (w + 15) >> 4
        } else {
            ((w + 31) >> 5) << 1
        };
        let (s7, s6) = (mbw << 4, mbh << 4);
        let (w0, w1) = match seed / 32 % 4 {
            0 => (s7, s6),
            1 => (s7, s6 + 16),
            2 => (s7 + 16, s6),
            _ => (r.next_u32() % 900, r.next_u32() % 900),
        };
        c.bus.write32(b, w0);
        c.bus.write32(b + 4, w1);
        c.g[A0][0] = OC1110_DEC as u64;
    }

    /// Both quantiser-matrix sources, the width limit, an unchanged picture
    /// size (early return), a changed one (allocations, frame buffers and the
    /// `SetPictureSize` tail call); fakes unwind at random calls.
    #[test]
    fn lib_mpeg_sequence_header_matches() {
        check_lohi(
            ee::s15::f_00403620__generated,
            lib_mpeg_sequence_header,
            512,
            &seq_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_SetupMacroblock

    fn smb_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x403e00, 3);
        fill(c, &mut r, OC1110_DEC, 0x600);
        c.bus.write32(OC1110_DEC + 0x12c, 1 + r.next_u32() % 30);
        c.bus.write32(OC1110_DEC + 0x810, seed % 3);
        c.bus
            .write32(OC1110_DEC + 0x174, [1u32, 2, 3][(seed / 3 % 3) as usize]);
        for (i, off) in [0x1c0u32, 0x1d0, 0x1e0].into_iter().enumerate() {
            let p = DATA + 0x3000 + 0x40 * i as u32;
            c.bus.write32(OC1110_DEC + off, p);
            c.bus.write32(p, r.next_u32());
            c.bus.write32(p + 0x10, r.next_u32() % 50);
        }
        let flags = [0u32, 1, 2, 3][(seed / 9 % 4) as usize] | (r.next_u32() & 0xc);
        c.g[A0][0] = OC1110_DEC as u64;
        c.g[A1][0] = (r.next_u32() % 3000) as u64;
        c.g[A2][0] = [0u32, 1, 2][(seed / 36 % 3) as usize] as u64;
        c.g[A3][0] = flags as u64;
        c.g[T0][0] = (seed / 108 % 6) as u64;
    }

    /// Intra (no prediction) and inter macroblocks, motion types 0..5 (0, 4
    /// and 5 report the error), the three picture structures, the flag
    /// combinations of the `b == 1 && flags & 2` field.
    #[test]
    fn lib_mpeg_setup_macroblock_matches() {
        check_lohi(
            ee::s15::f_00403e00__generated,
            lib_mpeg_setup_macroblock,
            648,
            &smb_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_SetupPredBlock

    fn spb_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1110_4758 ^ (seed as u64) << 9);
        scramble(c, &mut r);
        fill(c, &mut r, OC1110_DEC, 0x400);
        let cur = seed % 3;
        let k = seed / 3 % 4;
        c.bus.write32(OC1110_DEC + 0x810, cur);
        c.bus.write32(OC1110_DEC + 0x6bc + cur * 0x140, k);
        let rf = DATA + 0x3000;
        c.bus.write32(rf, r.next_u32());
        c.bus.write32(rf + 0x10, 1 + r.next_u32() % 40);
        for i in 0..8 {
            c.bus.write32(0x0045_3280 + 4 * i, r.next_u32());
            c.bus.write32(0x0045_32a0 + 4 * i, r.next_u32());
        }
        let signed = |r: &mut Rng| (r.next_u32() % 81) as i32 - 40;
        c.bus.write32(STACK_TOP, signed(&mut r) as u32);
        c.bus.write32(STACK_TOP + 8, signed(&mut r) as u32);
        c.bus.write32(STACK_TOP + 0x10, seed / 12 % 2);
        c.bus.write32(STACK_TOP + 0x18, seed / 24 % 2);
        c.g[A0][0] = OC1110_DEC as u64;
        c.g[A1][0] = rf as u64;
        c.g[A2][0] = (r.next_u32() % 24) as u64;
        c.g[A3][0] = (r.next_u32() % 2) as u64;
        c.g[T0][0] = (r.next_u32() % 8) as u64;
        c.g[T1][0] = (1 + r.next_u32() % 16) as u64;
        c.g[T2][0] = (r.next_u32() % 100) as u64;
        c.g[T3][0] = (r.next_u32() % 100) as u64;
    }

    /// Luma and chroma block descriptors for every vector parity, frame and
    /// field prediction, both buffer entries and block counts.
    #[test]
    fn lib_mpeg_setup_pred_block_matches() {
        check_lohi(
            ee::s15::f_00404758__generated,
            lib_mpeg_setup_pred_block,
            1200,
            &spb_setup,
        );
    }

    // ------------------------------------------------------------------
    // libmpeg_WaitBdec

    /// `seed % 6`: the IPU state the wait starts in (idle, ECD with and
    /// without queued DMA, queued DMA ended by the callback at the 1st or 2nd
    /// call or by an error raised in the callback); then the abort flag
    /// `dec[0x10a0]` and `DIntr`'s result.
    fn wbd_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1110_machine(c, seed, 0x405b70, 8);
        fill(c, &mut r, OC1110_DEC, 0x440);
        let mode = seed % 6;
        let abort = seed / 6 % 2;
        c.bus
            .write32(OC1110_DEC + 0x10a0, abort * (1 + r.next_u32() % 9));
        mb_wr(c, MB_DINTR, seed / 12 % 2);
        let queued = mode >= 2;
        c.bus
            .write32(0x1000_b020, if queued { 1 + r.next_u32() % 20 } else { 0 });
        if mode == 1 || mode == 2 {
            c.bus.write32(0x1000_2000, 0x3400_0000);
        }
        match mode {
            3 => {
                mb_wr(c, MB_CB_AT, 1);
                mb_wr(c, MB_CB_KIND, 0);
            }
            4 => {
                mb_wr(c, MB_CB_AT, 2);
                mb_wr(c, MB_CB_KIND, 0);
            }
            5 => {
                mb_wr(c, MB_CB_AT, 1);
                mb_wr(c, MB_CB_KIND, 1);
            }
            _ => {}
        }
        c.g[A0][0] = OC1110_DEC as u64;
    }

    /// The IPU wait loop (ended by the callback clearing the DMA, or by ECD),
    /// the abort path with `DIntr` returning 0 and 1, and the ECD report with
    /// its two callbacks. Not reached: a to-IPU DMA already queued
    /// (`0x1000b420`) or running (`0x1000b400` bit 8) inside the loop (the loop
    /// would never end on the test machine) and IPU_TOP with bit 63 set (the
    /// model's TOP reads zero in the high word).
    #[test]
    fn lib_mpeg_wait_bdec_matches() {
        check(
            ee::s15::f_00405b70__generated,
            lib_mpeg_wait_bdec,
            288,
            &wbd_setup,
        );
    }
}
