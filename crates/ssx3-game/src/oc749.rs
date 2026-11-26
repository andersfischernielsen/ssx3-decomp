//! Staging module oc749 (wave wave243): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc749`.

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
        0x001a_0100,
        fe_module_rider_preview_poll_models,
        "FEModule_RiderPreviewPollModels",
    ),
    (0x0028_b320, snd_bank_load, "SndBank_Load"),
    (
        0x0029_0358,
        audio_on_delayed_sound_pitched,
        "Audio_OnDelayedSoundPitched",
    ),
    (
        0x0029_17d0,
        ssx_audio_system_start_board_base_loop,
        "SSXAudioSystem_StartBoardBaseLoop",
    ),
];

/// `int FEModule_RiderPreviewPollModels(RiderPreview* self)` (the C++ `cFERider_isDoneLoadingModels`): once per poll, checks that every bolt file holding one of the rider's profile parts is loaded; when all are, builds each part's model from the bolt's MDF archive, adds it to the preview's anim model, releases the file and clears the slot. Returns 1 when done (or already done, +0xcbc set), else 0.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn fe_module_rider_preview_poll_models(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x001a0100);
    'b_1a032c: {
        'b_1a0328: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xcbc));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_1a0328;
            } else {
                'b_1a01cc: {
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x001a0144);
                    c.pc = 0x0014dc80;
                    call_indirect(c, 0x0014dc80);
                    if c.pc != 0x001a0144 {
                        return;
                    }
                    c.g[V1][0] = 0x1;
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001a0158);
                    c.g[A1][0] = 0x9;
                    c.pc = 0x0014dd58;
                    call_indirect(c, 0x0014dd58);
                    if c.pc != 0x001a0158 {
                        return;
                    }
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001a0168);
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                    c.pc = 0x0014ad28;
                    call_indirect(c, 0x0014ad28);
                    if c.pc != 0x001a0168 {
                        return;
                    }
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28c));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                    if t {
                        break 'b_1a01cc;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        c.g[S0][0] = c.g[S6][0];
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        'l_1a0190: loop {
                            'b_1a01c0: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[RA][0] = sx(0x001a01a0);
                                c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                c.pc = 0x0019cbe0;
                                ee::s03::f_0019cbe0(c);
                                if c.pc != 0x001a01a0 {
                                    return;
                                }
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1a01c0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[RA][0] = sx(0x001a01b4);
                                    c.g[A0][0] = c.g[S7][0];
                                    c.pc = 0x0019e238;
                                    ee::s03::f_0019e238(c);
                                    if c.pc != 0x001a01b4 {
                                        return;
                                    }
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                    if c.g[V0][0] == 0 {
                                        c.g[V1][0] = 0u64;
                                    }
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                    break 'b_1a01c0;
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001a0190);
                                }
                                continue 'l_1a0190;
                            } else {
                                break 'b_1a01cc;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t {
                    break 'b_1a032c;
                } else {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_1a0328;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                        'l_1a01e8: loop {
                            'b_1a0318: {
                                c.cycles += 11;
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[S1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                c.g[RA][0] = sx(0x001a0208);
                                c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.pc = 0x0019cbe0;
                                ee::s03::f_0019cbe0(c);
                                if c.pc != 0x001a0208 {
                                    return;
                                }
                                c.g[S4][0] = c.g[V0][0];
                                if (c.g[S4][0] as i64) < 0 {
                                    c.g[S0][0] = c.g[FP][0];
                                    break 'b_1a0318;
                                } else {
                                    'b_1a0240: {
                                        c.cycles += 9;
                                        c.g[RA][0] = sx(0x001a021c);
                                        c.g[A0][0] = c.g[SP][0];
                                        c.pc = 0x00314978;
                                        call_indirect(c, 0x00314978);
                                        if c.pc != 0x001a021c {
                                            return;
                                        }
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                        c.g[A0][0] = c.g[S7][0];
                                        c.g[RA][0] = sx(0x001a022c);
                                        c.g[A2][0] = c.g[S4][0];
                                        c.pc = 0x0019e2b0;
                                        ee::s03::f_0019e2b0(c);
                                        if c.pc != 0x001a022c {
                                            return;
                                        }
                                        c.g[S5][0] = c.g[V0][0];
                                        let t = c.g[S5][0] == 0u64;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_1a0240;
                                        } else {
                                            c.cycles += 2;
                                            c.g[RA][0] = sx(0x001a0240);
                                            c.g[A1][0] = c.g[S5][0];
                                            c.pc = 0x00314a40;
                                            call_indirect(c, 0x00314a40);
                                            if c.pc != 0x001a0240 {
                                                return;
                                            }
                                            break 'b_1a0240;
                                        }
                                    }
                                    'b_1a030c: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_1a030c;
                                        } else {
                                            c.cycles += 1;
                                            c.g[S3][0] = sx((c.g[S6][0] as u32)
                                                .wrapping_sub((c.g[S0][0] as u32)));
                                            'l_1a0250: loop {
                                                'b_1a0300: {
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                    c.g[RA][0] = sx(0x001a0260);
                                                    c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                                    c.pc = 0x0019cbe0;
                                                    ee::s03::f_0019cbe0(c);
                                                    if c.pc != 0x001a0260 {
                                                        return;
                                                    }
                                                    if c.g[V0][0] != c.g[S4][0] {
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_1a0300;
                                                    } else {
                                                        'b_1a02dc: {
                                                            c.cycles += 2;
                                                            if c.g[S5][0] == 0u64 {
                                                                c.g[A1][0] =
                                                                    c.lw((c.g[S2][0] as u32));
                                                                break 'b_1a02dc;
                                                            } else {
                                                                'b_1a02ac: {
                                                                    'b_1a029c: {
                                                                        c.cycles += 9;
                                                                        c.g[RA][0] = sx(0x001a0278);
                                                                        c.pc = 0x0014bdb8;
                                                                        call_indirect(
                                                                            c, 0x0014bdb8,
                                                                        );
                                                                        if c.pc != 0x001a0278 {
                                                                            return;
                                                                        }
                                                                        c.g[S0][0] = c.g[V0][0];
                                                                        c.g[A1][0] =
                                                                            c.lw(
                                                                                (c.g[S2][0] as u32)
                                                                            );
                                                                        c.g[A2][0] =
                                                                            c.lh(
                                                                                (c.g[S1][0] as u32)
                                                                            );
                                                                        c.g[RA][0] = sx(0x001a028c);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        c.pc = 0x0014d448;
                                                                        call_indirect(
                                                                            c, 0x0014d448,
                                                                        );
                                                                        if c.pc != 0x001a028c {
                                                                            return;
                                                                        }
                                                                        let t = (c.g[V0][0] as i64)
                                                                            >= 0;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        if t {
                                                                            break 'b_1a029c;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.g[S0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_1a02ac;
                                                                            } else {
                                                                                break 'b_1a029c;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = 0x38;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.mult(0, 2, 4, 2);
                                                                    c.g[S0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V0][0] as u32),
                                                                        ));
                                                                    break 'b_1a02ac;
                                                                }
                                                                c.cycles += 3;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                if c.g[A1][0] == 0u64 {
                                                                    c.g[A1][0] =
                                                                        c.lw((c.g[S2][0] as u32));
                                                                    break 'b_1a02dc;
                                                                } else {
                                                                    'b_1a02d8: {
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001a02c0);
                                                                        c.g[A0][0] = c.g[SP][0];
                                                                        c.pc = 0x00314c00;
                                                                        call_indirect(
                                                                            c, 0x00314c00,
                                                                        );
                                                                        if c.pc != 0x001a02c0 {
                                                                            return;
                                                                        }
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A2][0] = c.g[V0][0];
                                                                        if t {
                                                                            break 'b_1a02d8;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[A3][0] = c.lb((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x1));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[RA][0] =
                                                                                sx(0x001a02d8);
                                                                            c.g[A1][0] = 0u64;
                                                                            c.pc = 0x0030d8b8;
                                                                            call_indirect(
                                                                                c, 0x0030d8b8,
                                                                            );
                                                                            if c.pc != 0x001a02d8 {
                                                                                return;
                                                                            }
                                                                            break 'b_1a02d8;
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[A1][0] =
                                                                        c.lw((c.g[S2][0] as u32));
                                                                    break 'b_1a02dc;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = c.g[S7][0];
                                                        c.g[RA][0] = sx(0x001a02e8);
                                                        c.g[A2][0] = c.g[S4][0];
                                                        c.pc = 0x0019dc20;
                                                        ee::s03::f_0019dc20(c);
                                                        if c.pc != 0x001a02e8 {
                                                            return;
                                                        }
                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                        c.g[A3][0] = 0xffffffffffffffff;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[RA][0] = sx(0x001a02fc);
                                                        c.g[A2][0] = c.lh((c.g[S1][0] as u32));
                                                        c.pc = 0x0019cc30;
                                                        ee::s03::f_0019cc30(c);
                                                        if c.pc != 0x001a02fc {
                                                            return;
                                                        }
                                                        c.g[S3][0] =
                                                            sx((c.g[S3][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_1a0300;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S3][0] != 0u64;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001a0250);
                                                    }
                                                    continue 'l_1a0250;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    break 'b_1a030c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x001a0314);
                                    c.g[A1][0] = 0x2;
                                    c.pc = 0x00314988;
                                    call_indirect(c, 0x00314988);
                                    if c.pc != 0x001a0314 {
                                        return;
                                    }
                                    c.g[S0][0] = c.g[FP][0];
                                    break 'b_1a0318;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001a01e8);
                                }
                                continue 'l_1a01e8;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_1a0328;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        break 'b_1a032c;
    }
    c.cycles += 11;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = t;
    return;
}

/// `void SndBank_Load(Bank* self, char* file, int align)` (the C++ `cBankInstance_Load`): unloads the bank, loads its data from disc (by allocation location: heap, preloaded buffer or stream memory), registers it with the sound driver and copies the header, then records the filename.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn snd_bank_load(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0028b320);
    'b_28b508: {
        'b_28b3ec: {
            'b_28b3c4: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x0028b350);
                c.g[S1][0] = 0u64;
                c.pc = 0x0028b528;
                ee::s07::f_0028b528(c);
                if c.pc != 0x0028b350 {
                    return;
                }
                c.g[RA][0] = sx(0x0028b358);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x0028b2d0;
                ee::s07::f_0028b2d0(c);
                if c.pc != 0x0028b358 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.sw((c.g[SP][0] as u32), 0u64);
                if t {
                    break 'b_28b3c4;
                } else {
                    'b_28b3a8: {
                        'b_28b3a0: {
                            'b_28b394: {
                                'b_28b384: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_28b384;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A2][0] = c.g[S2][0];
                                        if t {
                                            break 'b_28b394;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            if t {
                                                break 'b_28b3ec;
                                            } else {
                                                break 'b_28b384;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[A0][0] = c.g[S3][0];
                                if t {
                                    break 'b_28b3a0;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_28b3ec;
                                    } else {
                                        break 'b_28b394;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S3][0];
                            let t = true;
                            c.g[A1][0] = c.g[SP][0];
                            if t {
                                break 'b_28b3a8;
                            } else {
                                break 'b_28b3a0;
                            }
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = 0x100;
                        break 'b_28b3a8;
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0028b3b0);
                    c.pc = 0x003e1a70;
                    call_indirect(c, 0x003e1a70);
                    if c.pc != 0x0028b3b0 {
                        return;
                    }
                    c.g[S1][0] = c.g[V0][0];
                    if c.g[S1][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_28b508;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_28b3ec;
                        } else {
                            break 'b_28b3c4;
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x0028b3d4);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.pc = 0x003e1bb8;
            call_indirect(c, 0x003e1bb8);
            if c.pc != 0x0028b3d4 {
                return;
            }
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_28b508;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                break 'b_28b3ec;
            }
        }
        'b_28b420: {
            c.cycles += 3;
            c.g[S4][0] = 0x2;
            if c.g[V0][0] != c.g[S4][0] {
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_28b420;
            } else {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x524));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0028b41c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.pc = t;
                call_indirect(c, t);
                if c.pc != 0x0028b41c {
                    return;
                }
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_28b420;
            }
        }
        'b_28b4d8: {
            'b_28b4c0: {
                'b_28b4b8: {
                    'b_28b4b4: {
                        'b_28b4a0: {
                            'b_28b448: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0028b428);
                                c.g[A1][0] = c.g[S1][0];
                                c.pc = 0x003b6098;
                                call_indirect(c, 0x003b6098);
                                if c.pc != 0x0028b428 {
                                    return;
                                }
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x7;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x8;
                                if t {
                                    break 'b_28b448;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_28b4a0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_28b4c0;
                                        } else {
                                            break 'b_28b448;
                                        }
                                    }
                                }
                            }
                            'b_28b474: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0028b450);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.pc = 0x003b6528;
                                call_indirect(c, 0x003b6528);
                                if c.pc != 0x0028b450 {
                                    return;
                                }
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                let t = c.g[V0][0] != c.g[S4][0];
                                c.g[A2][0] = 0u64;
                                if t {
                                    break 'b_28b474;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x0028b46c);
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.pc = 0x003b6548;
                                    call_indirect(c, 0x003b6548);
                                    if c.pc != 0x0028b46c {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_28b4b8;
                                    } else {
                                        break 'b_28b474;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x0028b480);
                            c.g[A1][0] = c.g[S2][0];
                            c.pc = 0x00252fa0;
                            call_indirect(c, 0x00252fa0);
                            if c.pc != 0x0028b480 {
                                return;
                            }
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0028b490);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            c.pc = 0x003b6548;
                            call_indirect(c, 0x003b6548);
                            if c.pc != 0x0028b490 {
                                return;
                            }
                            c.g[RA][0] = sx(0x0028b498);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = 0x002523a8;
                            call_indirect(c, 0x002523a8);
                            if c.pc != 0x0028b498 {
                                return;
                            }
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S2][0]);
                            if t {
                                break 'b_28b4b4;
                            } else {
                                break 'b_28b4a0;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_28b4b8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            break 'b_28b4b4;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    break 'b_28b4b8;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_28b4d8;
                } else {
                    break 'b_28b4c0;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            if c.g[V1][0] == c.g[V0][0] {
                c.sw((c.g[S0][0] as u32), 0u64);
                break 'b_28b4d8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0028b4d4);
                c.g[A0][0] = c.g[S1][0];
                c.pc = 0x002523a8;
                call_indirect(c, 0x002523a8);
                if c.pc != 0x0028b4d4 {
                    return;
                }
                c.sw((c.g[S0][0] as u32), 0u64);
                break 'b_28b4d8;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x524));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0028b4f8);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x0028b4f8 {
            return;
        }
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = sx(0x0028b504);
        c.g[A1][0] = c.g[S3][0];
        c.pc = 0x004166f4;
        call_indirect(c, 0x004166f4);
        if c.pc != 0x0028b504 {
            return;
        }
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        break 'b_28b508;
    }
    c.cycles += 7;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t;
    return;
}

/// `bool Audio_OnDelayedSoundPitched(void* ev, Request* req)` (the C++ `cBankSys_SFX_Add`-style voice push): claims the next voice slot of the bank manager (global at gp+0x574), fills its tables from the request (bank, patch, volume clamped to 0..0x7f, pitch) and plays it. Returns 1.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn audio_on_delayed_sound_pitched(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x00290358);
    'b_29065c: {
        'b_290630: {
            c.cycles += 176;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.g[A2][0] = 0x8;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0x18;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32));
            c.g[S4][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[S5][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 19);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[V1][0] as u32));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 19);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x00290460);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210;
            call_indirect(c, 0x00416210);
            if c.pc != 0x00290460 {
                return;
            }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[A1][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A2][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A3][0] = 0x7f;
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[T0][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[T0][0]);
            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S4][0]);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S5][0]);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x24));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.mult(0, 4, 4, 19);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if t {
                break 'b_290630;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_29065c;
                } else {
                    break 'b_290630;
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
        if c.g[V1][0] == 0u64 {
            c.g[V0][0] = 0x7f;
            break 'b_29065c;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
            break 'b_29065c;
        }
    }
    c.cycles += 22;
    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[A2][0] = 0x18;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
    c.g[A3][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f8));
    c.mult(0, 2, 2, 6);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x00290688);
    c.sh((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A3][0]);
    c.pc = 0x002906b8;
    ee::s07::f_002906b8(c);
    if c.pc != 0x00290688 {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = 0x1;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_StartBoardBaseLoop(SSXAudioSystem* self, Rider* rider)`: for a human local rider, claims the next voice slot of the bank manager, sets up the board base loop (surface bank, volume channel, listener, radius vanish), marks replay playback when in the replay game path, and plays it.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn ssx_audio_system_start_board_base_loop(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002917d0);
    'b_291810: {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V1][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S5][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x874));
        let t = c.g[V0][0] == 0u64;
        c.g[S4][0] = c.g[A0][0];
        if t {
            break 'b_291810;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x87c));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            break 'b_291810;
        }
    }
    'b_291c64: {
        c.cycles += 2;
        let t = c.g[V1][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        if t {
            break 'b_291c64;
        } else {
            'b_291850: {
                c.cycles += 10;
                c.g[RA][0] = sx(0x00291820);
                c.g[S6][0] = 0u64;
                c.pc = 0x0014dc80;
                call_indirect(c, 0x0014dc80);
                if c.pc != 0x00291820 {
                    return;
                }
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0029182c);
                c.g[A1][0] = 0u64;
                c.pc = 0x0014dd58;
                call_indirect(c, 0x0014dd58);
                if c.pc != 0x0029182c {
                    return;
                }
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = 0x2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                if c.g[V0][0] != c.g[A0][0] {
                    c.g[A0][0] = c.g[S4][0];
                    break 'b_291850;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00291848);
                    c.g[A0][0] = c.g[S4][0];
                    c.pc = 0x00288ae0;
                    ee::s07::f_00288ae0(c);
                    if c.pc != 0x00291848 {
                        return;
                    }
                    c.g[S6][0] = (c.g[V0][0] < 0x1u64) as u64;
                    c.g[A0][0] = c.g[S4][0];
                    break 'b_291850;
                }
            }
            'b_291b08: {
                'b_291adc: {
                    c.cycles += 157;
                    c.g[RA][0] = sx(0x00291858);
                    c.g[A1][0] = 0u64;
                    c.pc = 0x002917b8;
                    ee::s07::f_002917b8(c);
                    if c.pc != 0x00291858 {
                        return;
                    }
                    c.g[S2][0] = 0x18;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 18);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.sdr(2, (c.g[V1][0] as u32));
                    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                    c.mult(0, 4, 4, 18);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[RA][0] = sx(0x00291930);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00416210;
                    call_indirect(c, 0x00416210);
                    if c.pc != 0x00291930 {
                        return;
                    }
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[A1][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A3][0] = 0x7f;
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[T0][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[T0][0]);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 18);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if t {
                        break 'b_291adc;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        let t = (c.g[V1][0] as i64) < 0;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_291b08;
                        } else {
                            break 'b_291adc;
                        }
                    }
                }
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                if c.g[V1][0] == 0u64 {
                    c.g[V0][0] = 0x7f;
                    break 'b_291b08;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                    break 'b_291b08;
                }
            }
            'b_291c58: {
                c.cycles += 68;
                c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0x6;
                c.g[RA][0] = sx(0x00291b20);
                c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.pc = 0x00287968;
                ee::s07::f_00287968(c);
                if c.pc != 0x00291b20 {
                    return;
                }
                c.g[A3][0] = c.lw((c.g[S0][0] as u32));
                c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x110));
                c.g[T1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x214));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[T0][0]);
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x3688));
                c.ldl(2, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[T0][0] as u32));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[SP][0] as u32));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
                c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x238));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[S5][0]);
                c.g[RA][0] = sx(0x00291bc0);
                c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.pc = 0x00288a20;
                ee::s07::f_00288a20(c);
                if c.pc != 0x00291bc0 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[AT][0] = 0x42c80000;
                c.mtc1(1, 12);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x23c));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 13);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x00291bf8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                c.pc = 0x002a9988;
                call_indirect(c, 0x002a9988);
                if c.pc != 0x00291bf8 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                let t = c.g[S6][0] == 0u64;
                c.sw((c.g[V0][0] as u32), 0u64);
                if t {
                    break 'b_291c58;
                } else {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0x1;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x258));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x870));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x25c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                    break 'b_291c58;
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x00291c60);
            c.g[A0][0] = c.g[S4][0];
            c.pc = 0x002906b8;
            ee::s07::f_002906b8(c);
            if c.pc != 0x00291c60 {
                return;
            }
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_291c64;
        }
    }
    c.cycles += 9;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check_lohi, scramble, Rng, DATA, GAME_GP};

    /// Control block of this module's gated callee fakes (in the machine's own
    /// RAM, away from the pool and the fakes' call log at DATA+0x100000).
    const OC749_CTRL: u32 = DATA + 0x21_0000;
    const OC749_MAGIC: u32 = 0x0749_0749;

    /// Gate on, and maybe make one of the slot fakes unwind (`CTRL+4` holds the
    /// 1-based index of the fake call that unwinds, 0: none).
    fn oc749_gate(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC749_CTRL, OC749_MAGIC);
        let at = if r.next_u32() % 5 == 0 {
            1 + r.next_u32() % 6
        } else {
            0
        };
        c.bus.write32(OC749_CTRL + 4, at);
    }

    /// The result a slot fake returns for callee `addr` (v0).
    fn oc749_res(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(OC749_CTRL + 0x1000 + (addr & 0xffff), v);
    }

    /// Per-id/slot result tables of the bolt-file fakes (8 words each).
    const OC749_MODEL_SLOT: u32 = OC749_CTRL + 0x100;
    const OC749_IS_LOADED: u32 = OC749_CTRL + 0x140;
    const OC749_GET_DATA: u32 = OC749_CTRL + 0x180;
    const OC749_FIND_BOLT: u32 = OC749_CTRL + 0x1c0;
    /// Size the file-loading fake stores through its out pointer.
    const OC749_LOAD_SIZE: u32 = OC749_CTRL + 0x20;

    fn oc749_unwind(c: &mut Cpu) -> bool {
        let at = c.bus.read32(OC749_CTRL + 4);
        let n = c.bus.read32(OC749_CTRL + 8) + 1;
        c.bus.write32(OC749_CTRL + 8, n);
        if at != 0 && n == at {
            c.pc = 0xdead_0000 | n << 4;
            return true;
        }
        false
    }

    fn oc749_fake_body(c: &mut Cpu, addr: u32) {
        let a2 = c.g[A2][0] as u32;
        let v = match addr {
            0x19cbe0 => c.bus.read32(OC749_MODEL_SLOT + 4 * (a2 & 7)),
            0x19e238 => c.bus.read32(OC749_IS_LOADED + 4 * (a2 & 7)),
            0x19e2b0 => c.bus.read32(OC749_GET_DATA + 4 * (a2 & 7)),
            0x14d448 => c.bus.read32(OC749_FIND_BOLT + 4 * (a2 & 7)),
            _ => c.bus.read32(OC749_CTRL + 0x1000 + (addr & 0xffff)),
        };
        if addr == 0x3e1a70 {
            let out = c.g[A1][0] as u32;
            let size = c.bus.read32(OC749_LOAD_SIZE);
            c.bus.write32(out, size);
        }
        c.g[V0][0] = sx(v);
    }

    /// Gated fakes for the callees with override slots: with the magic word
    /// at `OC749_CTRL` they return per-callee results (and may unwind); with
    /// the gate off they run what the slot held before.
    macro_rules! oc749_fakes {
        ($($fake:ident = $addr:expr, $shard:ident :: $slot:ident / $gen:ident;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC749_CTRL) != OC749_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    if oc749_unwind(c) {
                        return;
                    }
                    oc749_fake_body(c, $addr);
                    ret(c);
                }
            )*
            fn oc749_install() {
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

    oc749_fakes! {
        oc749_fake_14dc80 = 0x14dc80, s01::OVERRIDE_0014DC80 / f_0014dc80__generated;
        oc749_fake_14dd58 = 0x14dd58, s01::OVERRIDE_0014DD58 / f_0014dd58__generated;
        oc749_fake_14ad28 = 0x14ad28, s01::OVERRIDE_0014AD28 / f_0014ad28__generated;
        oc749_fake_14bdb8 = 0x14bdb8, s01::OVERRIDE_0014BDB8 / f_0014bdb8__generated;
        oc749_fake_14d448 = 0x14d448, s01::OVERRIDE_0014D448 / f_0014d448__generated;
        oc749_fake_19cbe0 = 0x19cbe0, s03::OVERRIDE_0019CBE0 / f_0019cbe0__generated;
        oc749_fake_19e238 = 0x19e238, s03::OVERRIDE_0019E238 / f_0019e238__generated;
        oc749_fake_19e2b0 = 0x19e2b0, s03::OVERRIDE_0019E2B0 / f_0019e2b0__generated;
        oc749_fake_19cc30 = 0x19cc30, s03::OVERRIDE_0019CC30 / f_0019cc30__generated;
        oc749_fake_314c00 = 0x314c00, s10::OVERRIDE_00314C00 / f_00314c00__generated;
        oc749_fake_30d8b8 = 0x30d8b8, s10::OVERRIDE_0030D8B8 / f_0030d8b8__generated;
        oc749_fake_314988 = 0x314988, s10::OVERRIDE_00314988 / f_00314988__generated;
        oc749_fake_314978 = 0x314978, s10::OVERRIDE_00314978 / f_00314978__generated;
        oc749_fake_314a40 = 0x314a40, s10::OVERRIDE_00314A40 / f_00314a40__generated;
        oc749_fake_28b528 = 0x28b528, s07::OVERRIDE_0028B528 / f_0028b528__generated;
        oc749_fake_28b2d0 = 0x28b2d0, s07::OVERRIDE_0028B2D0 / f_0028b2d0__generated;
        oc749_fake_2906b8 = 0x2906b8, s07::OVERRIDE_002906B8 / f_002906b8__generated;
        oc749_fake_288ae0 = 0x288ae0, s07::OVERRIDE_00288AE0 / f_00288ae0__generated;
        oc749_fake_2917b8 = 0x2917b8, s07::OVERRIDE_002917B8 / f_002917b8__generated;
        oc749_fake_287968 = 0x287968, s07::OVERRIDE_00287968 / f_00287968__generated;
        oc749_fake_288a20 = 0x288a20, s07::OVERRIDE_00288A20 / f_00288a20__generated;
        oc749_fake_3e1a70 = 0x3e1a70, s14::OVERRIDE_003E1A70 / f_003e1a70__generated;
        oc749_fake_3e1bb8 = 0x3e1bb8, s14::OVERRIDE_003E1BB8 / f_003e1bb8__generated;
        oc749_fake_3b6098 = 0x3b6098, s13::OVERRIDE_003B6098 / f_003b6098__generated;
        oc749_fake_3b6528 = 0x3b6528, s13::OVERRIDE_003B6528 / f_003b6528__generated;
        oc749_fake_3b6548 = 0x3b6548, s13::OVERRIDE_003B6548 / f_003b6548__generated;
        oc749_fake_252fa0 = 0x252fa0, s06::OVERRIDE_00252FA0 / f_00252fa0__generated;
        oc749_fake_2523a8 = 0x2523a8, s06::OVERRIDE_002523A8 / f_002523a8__generated;
        oc749_fake_4166f4 = 0x4166f4, s15::OVERRIDE_004166F4 / f_004166f4__generated;
        oc749_fake_416210 = 0x416210, s15::OVERRIDE_00416210 / f_00416210__generated;
        oc749_fake_2a9988 = 0x2a9988, s08::OVERRIDE_002A9988 / f_002a9988__generated;
    }

    /// A sound manager at `mgr`: its play-option tables (S1 = mgr+0x1d8,
    /// words +0x20..+0x88) point at zeroed 0x100-byte regions from `regions`,
    /// the words before the current index (+0x1c) are random.
    fn oc749_mgr(c: &mut Cpu, r: &mut Rng, mgr: u32, regions: u32, cur: u32) {
        let s1 = mgr + 0x1d8;
        for off in (4..0x1cu32).step_by(4) {
            c.bus.write32(s1 + off, r.next_u32());
        }
        c.bus.write32(s1 + 0x1c, cur);
        for off in (0x20..=0x88u32).step_by(4) {
            c.bus.write32(s1 + off, regions + (off - 0x20) * 0x40);
        }
    }

    /// `FEModule_RiderPreviewPollModels`: the preview's bolt list (+0x28c count,
    /// ids at +0x290), the bolt-file model-slot table, the loaded flags (some
    /// files not loaded: returns 0), archives (with and without a package), the
    /// bolt DB entries (file 0 or not, index -1 or valid) and the real
    /// `BoltFile_Release` on slots with a high refcount.
    #[test]
    fn fe_module_rider_preview_poll_models_matches() {
        oc749_install();
        check_lohi(
            ee::s03::f_001a0100__generated,
            fe_module_rider_preview_poll_models,
            300,
            &|c, seed| {
                let mut r = Rng(0x001a_0100 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc749_gate(c, &mut r);
                let this = DATA + 0x100;
                let lst = DATA + 0x1000;
                let db = DATA + 0x2000;
                let arr = DATA + 0x2100;
                let s7 = DATA + 0x30000;
                let world = DATA + 0x200;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32() % 2);
                c.bus.write32(this + 4, r.next_u32() % 3);
                c.bus.write32(this + 8, DATA + 0x180);
                c.bus.write32(
                    this + 0xcbc,
                    if seed % 9 == 0 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, s7 - 0x1a70);
                c.bus.write32(GAME_GP.wrapping_sub(0x1788), DATA + 0x400);
                for file in 0..2u32 {
                    for slot in 0..4u32 {
                        c.bus.write32(s7 + 0x12000 * file + 0x120 * slot + 0x114, 9);
                    }
                }
                c.bus.write32(lst + 0x28c, r.next_u32() % 4);
                for i in 0..4u32 {
                    let word = (r.next_u32() & 0xffff_0000) | (r.next_u32() % 8);
                    c.bus.write32(lst + 0x290 + 4 * i, word);
                }
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(OC749_MODEL_SLOT + 4 * k, slot);
                    c.bus
                        .write32(OC749_IS_LOADED + 4 * k, (r.next_u32() % 6 != 0) as u32);
                    let data = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        DATA + 0x6000
                    };
                    c.bus.write32(OC749_GET_DATA + 4 * k, data);
                    let idx = if r.next_u32() % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(OC749_FIND_BOLT + 4 * k, idx);
                }
                oc749_res(c, 0x14dc80, DATA + 0x500);
                oc749_res(c, 0x14dd58, DATA + 0x510);
                oc749_res(c, 0x14ad28, lst);
                oc749_res(c, 0x14bdb8, db);
                c.bus.write32(db + 4, arr);
                for e in 0..3u32 {
                    c.bus.write8(arr + 0x38 * e + 1, r.next_u32() as u8);
                    let file = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(arr + 0x38 * e + 0x18, file);
                }
                let part = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    DATA + 0x7000
                };
                oc749_res(c, 0x314c00, part);
                oc749_res(c, 0x30d8b8, r.next_u32());
            },
        );
    }

    /// `SndBank_Load`: by allocation location (+8: 0 heap, 1 preloaded buffer,
    /// 2 stream memory, 3 none), the file loaders (failing sometimes), the
    /// bank-add result (7: header copy, 8: keep data, other: free) and the two
    /// bank-manager virtual calls.
    #[test]
    fn snd_bank_load_matches() {
        oc749_install();
        fakes::covered(
            ee::s07::f_0028b320__generated,
            snd_bank_load,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b320, 2);
                oc749_gate(c, &mut r);
                let this = DATA + 0x100;
                let bank_mgr = DATA + 0x600;
                let p1 = DATA + 0x700;
                let q = DATA + 0x710;
                let vt = DATA + 0x800;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(this + 4, r.next_u32());
                c.bus.write32(this + 8, seed % 4);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write32(this + 0x10, DATA + 0xb00);
                c.bus.write32(this + 0x14, r.next_u32() % 0x1000);
                c.bus.write32(GAME_GP + 0x524, bank_mgr);
                c.bus.write32(bank_mgr + 0x1d8, p1);
                c.bus.write32(p1, q);
                c.bus.write32(q, vt);
                c.bus
                    .write16(vt + 0x30, if r.next_u32() % 2 == 0 { 0 } else { 0x10 });
                c.bus.write32(vt + 0x34, fakes::FAKE);
                c.bus
                    .write16(vt + 0x38, if r.next_u32() % 2 == 0 { 0 } else { 0x20 });
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x10);
                let data = if r.next_u32() % 6 == 0 {
                    0
                } else {
                    DATA + 0x900
                };
                oc749_res(c, 0x3e1a70, data);
                c.bus.write32(OC749_LOAD_SIZE, r.next_u32() % 0x1000);
                oc749_res(c, 0x3e1bb8, (r.next_u32() % 6 != 0) as u32);
                oc749_res(c, 0x3b6098, [7, 8, 3][(seed as usize / 4) % 3]);
                oc749_res(c, 0x3b6528, r.next_u32() % 0x400);
                oc749_res(c, 0x252fa0, DATA + 0xa00);
            },
        );
    }

    /// `Audio_OnDelayedSoundPitched`: pushes one voice record (a pitched
    /// delayed sound request) onto the bank manager's tables, clamping the
    /// volume from the request into 0..0x7f (values below 0, 0x7f and above
    /// are covered), then plays it.
    #[test]
    fn audio_on_delayed_sound_pitched_matches() {
        oc749_install();
        check_lohi(
            ee::s07::f_00290358__generated,
            audio_on_delayed_sound_pitched,
            300,
            &|c, seed| {
                let mut r = Rng(0x0029_0358 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc749_gate(c, &mut r);
                let g = DATA + 0x80;
                let mgr = DATA + 0x1000;
                let req = DATA + 0x100;
                c.bus.write32(GAME_GP + 0x574, g);
                c.bus.write32(g, mgr);
                c.bus.write32(g + 4, DATA + 0x3000);
                c.bus.write32(g + 8, DATA + 0x3100);
                oc749_mgr(c, &mut r, mgr, DATA + 0x10000, seed % 2);
                c.g[A1][0] = sx(req);
                for off in [0x18u32, 0x1c, 0x20, 0x28] {
                    c.bus.write32(req + off, r.next_u32());
                }
                let vol = [0xffff_fffd, 0, 50, 126, 127, 300][(seed as usize / 2) % 6];
                c.bus.write32(req + 0x24, vol);
            },
        );
    }

    /// `SSXAudioSystem_StartBoardBaseLoop`: for a human, local rider (rider
    /// +0x874 set, +0x87c > 0) pushes the board base loop voice onto the
    /// bank manager tables; in a replay game path (byte 0x535c11 == 2) with no
    /// replay playing it also sets the replay-start fields.
    #[test]
    fn ssx_audio_system_start_board_base_loop_matches() {
        oc749_install();
        check_lohi(
            ee::s07::f_002917d0__generated,
            ssx_audio_system_start_board_base_loop,
            300,
            &|c, seed| {
                let mut r = Rng(0x0029_17d0 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc749_gate(c, &mut r);
                let this = DATA + 0x100;
                let rider = DATA + 0x500;
                let p = DATA + 0x400;
                let mgr = DATA + 0x1000;
                let q = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rider);
                c.bus.write32(this, mgr);
                c.bus.write32(this + 4, DATA + 0x3000);
                c.bus.write32(this + 8, DATA + 0x3100);
                c.bus.write32(this + 0x118, p);
                c.bus.write32(p, q);
                oc749_mgr(c, &mut r, mgr, DATA + 0x10000, seed % 2);
                oc749_mgr(c, &mut r, q, DATA + 0x14000, (seed / 2) % 3);
                c.bus.write32(rider + 0x870, r.next_u32());
                c.bus.write32(rider + 0x874, (seed % 5 != 0) as u32);
                c.bus.write32(
                    rider + 0x87c,
                    [0, 1, 5, 0xffff_fff0][(seed as usize / 3) % 4],
                );
                c.bus.write8(0x0053_5c11, if seed % 3 == 0 { 1 } else { 2 });
                c.bus.write32(0x004a_3688, r.next_u32());
                c.bus.write32(0x004a_368c, r.next_u32());
                oc749_res(c, 0x288ae0, (seed % 4 == 0) as u32);
                oc749_res(c, 0x2917b8, r.next_u32());
                oc749_res(c, 0x287968, DATA + 0x3200);
                oc749_res(c, 0x288a20, r.next_u32());
                oc749_res(c, 0x14dc80, DATA + 0x500);
                oc749_res(c, 0x14dd58, DATA + 0x510);
            },
        );
    }
}
