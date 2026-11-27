//! Staging module oc1097 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1097`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `SND_DspBufferSource_Init` (the generated function, through its slot).
const DSP_BUFFER_SOURCE_INIT: Callee = Callee {
    addr: 0x003c_f798,
    f: ee::s14::SND_DspBufferSource_Init,
};
const SND_EE_FX_DISABLE: Callee = Callee {
    addr: 0x003c_c848,
    f: ee::s14::SND_EeFxDisable,
};
const SND_EE_FX_BUILD: Callee = Callee {
    addr: 0x003c_c070,
    f: ee::s14::SND_EeFxBuild,
};

/// `void SND_EeFxProcess(int n)` (0x3cbc58): the sound mixer's effects bus
/// pass for `n` samples. Clears the 6-entry `done` flags; if the wet bus
/// pointer (sndmix+0x14) is set it copies the wet buffer (`n*4` bytes) into
/// the work buffer at 0x515d80 and clears it. Otherwise, unless the work
/// buffer's use counter (+0x1800) reaches 3000, it initialises the DSP
/// buffer source and, per output channel not yet done, runs the channel
/// object's process function and mixes `wet` into the channel buffers named
/// by the route table, marking them done; finally it clears the work buffer.
/// Written register for register (state in `c.g`, as the generated body).
pub fn snd_ee_fx_process(c: &mut Cpu) {
    guest(c, |c| {
        'b_3cbe9c: {
            'b_3cbcd8: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[A1][0] = 0u64;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = 0x18;
                c.g[S5][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[RA][0] = sx(0x003cbc98);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00416210, 0x003cbc98)?;
                c.g[V1][0] = 0x510000;
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b40));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                let t = c.g[A0][0] == 0u64;
                c.g[S0][0] = 0x510000;
                if t {
                    break 'b_3cbcd8;
                } else {
                    c.cycles += 11;
                    c.g[S1][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d80));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003cbcc0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003c51e8, 0x003cbcc0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003cbcd0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00416210, 0x003cbcd0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    break 'b_3cbe9c;
                }
            }
            'b_3cbe98: {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                c.g[V0][0] = 0x510000;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d80));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1800));
                c.g[S6][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb4));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3000i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1800), c.g[V1][0]);
                if t {
                    break 'b_3cbe98;
                } else {
                    'b_3cbe84: {
                        c.cycles += 13;
                        c.g[V0][0] = 0x510000;
                        c.g[V1][0] = 0x510000;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d58));
                        c.g[A3][0] = sx((c.g[S5][0] as u32) << 2);
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d40));
                        c.g[A1][0] = 0x4;
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[A3][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x003cbd2c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, 0x003c72f8, 0x003cbd2c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x510000;
                        if t {
                            break 'b_3cbe84;
                        } else {
                            c.cycles += 12;
                            c.g[A0][0] = 0x510000;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7600));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5d44));
                            c.g[S2][0] = 0u64;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[RA][0] = sx(0x003cbd58);
                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                            DSP_BUFFER_SOURCE_INIT.call(c, 0x003cbd58)?;
                            c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x5));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S7][0] = c.g[S1][0];
                            if t {
                                break 'b_3cbe84;
                            } else {
                                c.cycles += 1;
                                c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                                'l_3cbd68: loop {
                                    'b_3cbe70: {
                                        c.cycles += 4;
                                        c.g[S4][0] =
                                            sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_3cbe70;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V1][0] = 0x510000;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5d48));
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                            let t = c.g[A0][0] == 0u64;
                                            c.g[A2][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_3cbe70;
                                            } else {
                                                c.cycles += 32;
                                                c.g[A3][0] = c.g[S6][0];
                                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                c.g[A1][0] = c.g[S5][0];
                                                c.g[T0][0] = c.g[FP][0];
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x003cbda8);
                                                c.g[S3][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0xd0));
                                                call_ptr(c, t, 0x003cbda8)?;
                                                c.g[S1][0] = 0u64;
                                                c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S7][0] as u32).wrapping_add(0x1e0));
                                                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 12);
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x003cbdcc);
                                                c.g[A1][0] = c.g[S6][0];
                                                call_ptr(c, t, 0x003cbdcc)?;
                                                c.g[V0][0] = 0x510000;
                                                c.g[A3][0] = 0x450000;
                                                c.g[A1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x5d58));
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_sub(0x1e9c));
                                                c.lo[0] = c.g[A3][0];
                                                c.g[V0][0] = 0xd8;
                                                c.g[A2][0] =
                                                    c.lbu((c.g[S7][0] as u32).wrapping_add(0x5));
                                                c.madd(0, 5, 5, 2);
                                                c.g[V1][0] = 0x24;
                                                c.g[A0][0] = 0x6;
                                                c.mult(0, 3, 6, 3);
                                                c.mult(0, 4, 18, 4);
                                                c.g[V0][0] = 0x1;
                                                c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                                                c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                let t = c.g[A2][0] == 0u64;
                                                c.g[S0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                if t {
                                                    break 'b_3cbe70;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[S4][0] = c.g[S3][0];
                                                    c.g[S2][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_sub(0xd0));
                                                    c.g[S3][0] = 0x1;
                                                    'l_3cbe20: loop {
                                                        'b_3cbe5c: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            if t {
                                                                break 'b_3cbe5c;
                                                            } else {
                                                                c.cycles += 12;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x1e0));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[S4][0] as u32,
                                                                    ));
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 12);
                                                                c.g[A2][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[A0][0] = c.g[S5][0];
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x003cbe4c);
                                                                c.g[A1][0] = c.g[S6][0];
                                                                call_ptr(c, t, 0x003cbe4c)?;
                                                                c.g[V0][0] =
                                                                    c.lb(c.g[S0][0] as u32);
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V1][0] as u32, c.g[S3][0]);
                                                                break 'b_3cbe5c;
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[V0][0] = c.lbu(
                                                            (c.g[S2][0] as u32).wrapping_add(0x5),
                                                        );
                                                        c.g[S1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] = ((c.g[S1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x003cbe20);
                                                            }
                                                            continue 'l_3cbe20;
                                                        } else {
                                                            break 'b_3cbe70;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lbu((c.g[S7][0] as u32).wrapping_add(0x5));
                                    c.g[S2][0] = c.g[FP][0];
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003cbd68);
                                        }
                                        continue 'l_3cbd68;
                                    } else {
                                        break 'b_3cbe84;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = 0x510000;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d80));
                    c.g[RA][0] = sx(0x003cbe98);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00416210, 0x003cbe98)?;
                    break 'b_3cbe98;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            break 'b_3cbe9c;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    });
}

/// `void SND_EeFxLoad(int outputRate, const u8* data)` (0x3cc630): loads an
/// effects-bus description. Disables the current one, copies the 0x38-byte
/// header to 0x5175c0, finds the number of used output channels (last
/// non-zero count in the header; stored at 0x515d58/0x515d54), walks the
/// per-channel node lists (byte-indexed size table at 0x44e870, 12 bytes per
/// route) to record two cursors (0x5175f8, 0x5175fc), builds the bus
/// (`SND_EeFxBuild(outputRate, data)`) and installs `SND_EeFxProcess` in the
/// mixer (sndmix+0x10, flag at +0xf).
///
/// State stays in `c.g` as in the generated body: a back-edge event that
/// unwinds leaves it ready for the generated body to resume (`c.pc` is then
/// a loop head, which is delegated to `f_003cc630__generated`).
pub fn snd_ee_fx_load(c: &mut Cpu) {
    if c.pc != 0x003c_c630 {
        return ee::s14::f_003cc630__generated(c);
    }
    guest(c, |c| {
        c.cycles += 35;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = c.g[A1][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x38));
        SND_EE_FX_DISABLE.call(c, 0x003c_c66c)?;
        c.g[S0][0] = 0x510000;
        c.g[V0][0] = 0x1;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x75c0));
        c.g[A3][0] = 0x510000;
        c.g[V1][0] = 0x450000;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[V1][0] as u32).wrapping_sub(0x1768), c.g[V0][0]);
        c.g[A1][0] = c.g[S6][0];
        c.g[S4][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5b40));
        c.g[A2][0] = 0x38;
        call_ptr(c, 0x003c_51e8, 0x003c_c698)?;
        c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x4;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[S1][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x003c_72f8, 0x003c_c6b4)?;
        let last_nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = 0x510000;
        if last_nonzero {
            // 0x3cc6bc: all output channels are used
            c.cycles += 6;
            c.g[V1][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
            c.g[S4][0] = c.g[S1][0];
            c.g[S5][0] = 0x510000;
            c.g[S1][0] = c.g[V0][0];
            c.sw((c.g[V0][0] as u32).wrapping_add(0x5d58), c.g[V1][0]);
        } else {
            // 0x3cc6d4: trim trailing channels with a zero count
            c.cycles += 3;
            c.g[S1][0] = 0x510000;
            c.g[S5][0] = 0x510000;
            loop {
                // 0x3cc6e4
                c.cycles += 9;
                c.g[V0][0] = 0x510000;
                c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x75c8));
                c.g[A1][0] = 0x4;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[S4][0] = c.g[S0][0];
                call_ptr(c, 0x003c_72f8, 0x003c_c700)?;
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if !zero {
                    break;
                }
                if event_due(c) {
                    c.pc = 0x003c_c6e0;
                    if c.event() {
                        return Ok(());
                    }
                }
                // 0x3cc6e0
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            }
            // 0x3cc708
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x5d58), c.g[V0][0]);
        }
        // 0x3cc70c
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5d58));
        c.g[S1][0] = 0u64;
        let no_channels = (c.g[S3][0] as i64) <= 0;
        c.sw((c.g[S5][0] as u32).wrapping_add(0x5d54), c.g[V0][0]);
        if !no_channels {
            // 0x3cc71c
            c.cycles += 5;
            c.g[V0][0] = 0x510000;
            c.g[FP][0] = 0x450000;
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x75c8));
            c.g[S7][0] = sx((c.g[S5][0] as u32).wrapping_add(0x18));
            loop {
                // 0x3cc730
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A1][0] = 0x4;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x003c_72f8, 0x003c_c744)?;
                c.g[V1][0] = c.g[V0][0];
                let none = (c.g[V1][0] as i64) <= 0;
                c.g[A1][0] = c.g[V1][0];
                if !none {
                    // 0x3cc750: skip `count` nodes
                    c.cycles += 2;
                    c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1790));
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu(c.g[S2][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        let more = c.g[A1][0] != 0;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            c.pc = 0x003c_c758;
                            if c.event() {
                                return Ok(());
                            }
                        }
                    }
                }
                // 0x3cc774: skip `routes * 12` bytes
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[A1][0] = 0x4;
                call_ptr(c, 0x003c_72f8, 0x003c_c780)?;
                c.g[A0][0] = 0xc;
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.mult(0, 4, 2, 4);
                let again = c.g[V1][0] != 0;
                c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                if !again {
                    break;
                }
                if event_due(c) {
                    c.pc = 0x003c_c730;
                    if c.event() {
                        return Ok(());
                    }
                }
            }
        }
        // 0x3cc794
        c.cycles += 10;
        c.g[V0][0] = 0x510000;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x75c8));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x75f8), c.g[S2][0]);
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x003c_72f8, 0x003c_c7b0)?;
        c.g[V1][0] = c.g[V0][0];
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = 0x450000;
        if !none {
            // 0x3cc7bc
            c.cycles += 3;
            c.g[A1][0] = c.g[V1][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1790));
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[S2][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let more = c.g[A1][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    c.pc = 0x003c_c7c8;
                    if c.event() {
                        return Ok(());
                    }
                }
            }
        }
        // 0x3cc7e4
        c.cycles += 24;
        c.g[A0][0] = 0x510000;
        c.g[A1][0] = c.g[S6][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x75fc), c.g[S2][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        SND_EE_FX_BUILD.call(c, 0x003c_c7f8)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x3d0000;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b40));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x43a8));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[A0][0] = 0x1;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0xf), c.g[A0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_bc58, snd_ee_fx_process, "SND_EeFxProcess"),
    (0x003c_c630, snd_ee_fx_load, "SND_EeFxLoad"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fakes, fresh, run_generated, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word and mailbox of this module's fakes (own words, in the machine's RAM).
    pub(crate) const OC1097_GATE: u32 = DATA + 0x20_0000;
    pub(crate) const OC1097_MAGIC: u32 = 0x1097_f00d;
    const OC1097_SNDMIX: u32 = 0x0051_5b40;
    const OC1097_BUF: u32 = 0x0051_5d80;

    static OC1097_PREV_DSP: AtomicUsize = AtomicUsize::new(0);
    static OC1097_PREV_DISABLE: AtomicUsize = AtomicUsize::new(0);
    static OC1097_PREV_BUILD: AtomicUsize = AtomicUsize::new(0);

    /// Record a call (a0, a1, ra, cycles) in the mailbox at `slot` and return a
    /// fixed result: the argument registers must match between the two runs.
    fn oc1097_record(c: &mut Cpu, slot: u32) {
        let n = c.bus.read32(slot);
        c.bus.write32(slot, n + 1);
        let p = slot + 0x10 + 0x20 * (n & 7);
        let (a0, a1, ra, cy) = (c.g[A0][0], c.g[A1][0], c.g[RA][0], c.cycles);
        c.bus.write64(p, a0);
        c.bus.write64(p + 8, a1);
        c.bus.write64(p + 16, ra);
        c.bus.write64(p + 24, cy);
        c.cycles += 5;
        c.g[V0][0] = 0x77;
        c.g[V1][0] = 0x88;
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.pc = ra as u32;
    }

    fn oc1097_fake_dsp(c: &mut Cpu) {
        if c.bus.read32(OC1097_GATE) != OC1097_MAGIC {
            return fakes::fall_back(c, &OC1097_PREV_DSP, ee::s14::f_003cf798__generated);
        }
        oc1097_record(c, OC1097_GATE + 0x100)
    }
    fn oc1097_fake_disable(c: &mut Cpu) {
        if c.bus.read32(OC1097_GATE) != OC1097_MAGIC {
            return fakes::fall_back(c, &OC1097_PREV_DISABLE, ee::s14::f_003cc848__generated);
        }
        oc1097_record(c, OC1097_GATE + 0x400)
    }
    fn oc1097_fake_build(c: &mut Cpu) {
        if c.bus.read32(OC1097_GATE) != OC1097_MAGIC {
            return fakes::fall_back(c, &OC1097_PREV_BUILD, ee::s14::f_003cc070__generated);
        }
        oc1097_record(c, OC1097_GATE + 0x700)
    }

    fn oc1097_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CF798),
            oc1097_fake_dsp,
            &OC1097_PREV_DSP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CC848),
            oc1097_fake_disable,
            &OC1097_PREV_DISABLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CC070),
            oc1097_fake_build,
            &OC1097_PREV_BUILD,
        );
    }

    /// Big-endian 32-bit store (what `SNDI_getb(p, 4)` reads back).
    fn oc1097_be32(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v.swap_bytes());
    }

    /// Effects bus pass: wet path (seed%6==0), counter at the limit (1), no
    /// route table entry (2), otherwise random channel objects and routes.
    fn oc1097_process_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1097_0001, 10);
        c.bus.write32(OC1097_GATE, OC1097_MAGIC);
        oc1097_install_fakes();
        let n = 1 + r.next_u32() % 48;
        c.g[A0][0] = n as u64;
        let kind = seed % 6;
        let wet = if kind == 0 { DATA + 0x4000 } else { 0 };
        c.bus.write32(OC1097_SNDMIX + 0x14, wet);
        c.bus.write32(OC1097_SNDMIX + 0xb0, DATA + 0x1000);
        c.bus.write32(OC1097_SNDMIX + 0xb4, DATA + 0x2000);
        let channels = if seed % 11 == 0 { 0 } else { r.next_u32() % 7 };
        c.bus.write8(OC1097_SNDMIX + 5, channels as u8);
        c.bus.write32(OC1097_SNDMIX + 0x1e0, fakes::FAKE + 0x100);
        for i in 0..6u32 {
            c.bus
                .write32(OC1097_SNDMIX + 0xd0 + 4 * i, DATA + 0x6000 + 0x100 * i);
        }
        let counter = if kind == 1 { 2999 } else { r.next_u32() % 100 };
        c.bus.write32(OC1097_BUF + 0x1800, counter);
        let idx = r.next_u32() % 4;
        c.bus.write32(0x0051_5d58, idx);
        let table = DATA + 0x3000;
        c.bus.write32(0x0051_5d40, table);
        let used = if kind == 2 { 0 } else { 1 + r.next_u32() % 3 };
        oc1097_be32(c, table + 4 + 4 * idx, used);
        let sources = DATA + 0x3100;
        c.bus.write32(0x0051_5d44, sources);
        let sel = r.next_u32() % 4;
        c.bus.write32(0x0051_7600, sel);
        c.bus.write32(sources + 4 * sel, DATA + 0x7000);
        let objs = DATA + 0x3200;
        c.bus.write32(0x0051_5d48, objs);
        for i in 0..6u32 {
            let obj = DATA + 0x3300 + 0x10 * i;
            let has = r.next_u32() % 3 != 0;
            c.bus.write32(objs + 4 + 4 * i, if has { obj } else { 0 });
            c.bus.write32(obj, fakes::FAKE + 0x10 * i);
        }
        // Route table: D_0044E164[idx] (0xd8 bytes), + channels*0x24 + i*6 + j.
        let route = 0x0044_e164 + idx * 0xd8 + channels * 0x24;
        for ij in 0..36u32 {
            let v = if r.next_u32() % 3 == 0 {
                0xff
            } else {
                r.next_u32() % 6
            };
            c.bus.write8(route + ij, v as u8);
        }
    }

    /// `SND_EeFxProcess`: differential test against the generated body, with
    /// LO/HI (the route address is a `madd`), and a check that the channel
    /// objects' process calls and both mixes were reached.
    #[test]
    fn snd_ee_fx_process_matches() {
        let seeds = 240;
        check_lohi(
            ee::s14::f_003cbc58__generated,
            snd_ee_fx_process,
            seeds,
            &oc1097_process_setup,
        );
        let mut cycles = std::collections::BTreeSet::new();
        let mut rets = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            oc1097_process_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003cbc58__generated);
            cycles.insert(c.cycles - before);
            rets.extend(fakes::logged_returns(&mut c));
        }
        for ret in [0x003c_bda8u32, 0x003c_bdcc, 0x003c_be4c] {
            assert!(
                rets.contains(&ret),
                "no seed reached the call returning to {ret:#x}"
            );
        }
        assert!(cycles.len() >= 8, "only {} distinct paths", cycles.len());
    }

    /// Effects bus load: all channels used (seed%4==0) or trailing channels
    /// with a zero count trimmed; random node streams and counts.
    fn oc1097_load_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1097_0002, 0);
        c.bus.write32(OC1097_GATE, OC1097_MAGIC);
        oc1097_install_fakes();
        c.pc = 0x003c_c630;
        let data = DATA + 0x5000;
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = data as u64;
        let n = 1 + r.next_u32() % 6;
        c.bus.write8(OC1097_SNDMIX + 5, n as u8);
        let all_used = seed % 4 == 0 || n == 1;
        for i in 0..6u32 {
            let v = r.next_u32() % 3;
            let v = if i + 1 >= n && !all_used { 0 } else { v };
            oc1097_be32(c, data + 8 + 4 * i, v);
            let routes = r.next_u32() % 4;
            oc1097_be32(c, data + 0x20 + 4 * i, routes);
        }
        if all_used {
            oc1097_be32(c, data + 8 + 4 * (n - 1), 1 + r.next_u32() % 2);
        } else {
            let k = r.next_u32() % (n - 1);
            oc1097_be32(c, data + 8 + 4 * k, 1 + r.next_u32() % 2);
        }
        for b in 0..0x300u32 {
            let v = r.next_u32() % 8;
            c.bus.write8(data + 0x38 + b, v as u8);
        }
        for b in 0..8u32 {
            c.bus.write32(0x0044_e870 + 4 * b, 4 + 4 * (b % 5));
        }
    }

    /// `SND_EeFxLoad`: differential test (resume entries are delegated to the
    /// generated body; event unwinds at its back edges are not driven here).
    #[test]
    fn snd_ee_fx_load_matches() {
        let seeds = 240;
        check_lohi(
            ee::s14::f_003cc630__generated,
            snd_ee_fx_load,
            seeds,
            &oc1097_load_setup,
        );
        let mut cycles = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            oc1097_load_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003cc630__generated);
            cycles.insert(c.cycles - before);
        }
        assert!(cycles.len() >= 8, "only {} distinct paths", cycles.len());
    }
}
