//! Staging module oc1094 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1094`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003c_c070, snd_ee_fx_build, "SND_EeFxBuild")];

/// `void SND_EeFxBuild(int rate)` (0x003cc070): builds the EE-side DSP effect
/// graph from the loaded effect bank. Allocates the output-channel table
/// (`D_00515D48`, 7 words) and the node table (`D_00515D44`), then for every
/// node record (kind byte, output channel byte, 16-bit big-endian parameters)
/// allocates and initialises the filter object for its kind (pass-through,
/// lowpass, FIR low/high/band-pass, resonator, gain, all-pass, sum, block
/// delay), maps the node to its output channel through `SND_EeFxLookupParam`
/// (kind 9 with bank version >= 11 maps two channels, one per nibble), then
/// connects the nodes by the link list and counts the records into
/// `D_00517600`. The body keeps the generated block structure (resume
/// points after events); every register, cycle and call is as generated.
pub fn snd_ee_fx_build(c: &mut Cpu) {
    if c.pc != 0x003cc070 {
        return ee::s14::f_003cc070__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003cc070;
        loop {
            match pc {
                0x003cc070 => {
                    c.cycles += 50;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.g[V0][0] = 0x510000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x75c0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = 0x510000;
                    c.g[S6][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A0][0] = 0x1c;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S1][0] = 0x510000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = 0x510000;
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S5][0] = 0u64;
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x003cc0c4);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5d40), c.g[V0][0]);
                    call_ptr(c, 0x003b5c60, 0x003cc0c4)?;
                    c.g[A2][0] = 0x1c;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003cc0d8);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x5d48), c.g[V0][0]);
                    call_ptr(c, 0x00416210, 0x003cc0d8)?;
                    c.g[S1][0] = 0x510000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5d58));
                    c.g[V1][0] = 0x510000;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5d40));
                    c.g[V0][0] = 0x510000;
                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x75f8));
                    c.g[A0][0] = sx((c.g[FP][0] as u32) << 2);
                    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x75fc));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = 0x510000;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = 0x510000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5d4c), c.g[A2][0]);
                    c.g[A1][0] = 0x4;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x5d50), c.g[A3][0]);
                    c.g[RA][0] = sx(0x003cc120);
                    c.g[S4][0] = c.g[S1][0];
                    call_ptr(c, 0x003c72f8, 0x003cc120)?;
                    c.g[S7][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003cc12c);
                    c.g[A0][0] = sx((c.g[S7][0] as u32) << 2);
                    call_ptr(c, 0x003b5c60, 0x003cc12c)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x5d44), c.g[V0][0]);
                    let t = (c.g[S7][0] as i64) <= 0;
                    c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5d4c));
                    if t {
                        pc = 0x003cc50c;
                        continue;
                    }
                    pc = 0x003cc138;
                    continue;
                }
                0x003cc138 => {
                    c.cycles += 29;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32));
                    c.g[S1][0] = 0x450000;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x17b8));
                    c.g[S0][0] = sx((c.g[S5][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S3][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[RA][0] = sx(0x003cc15c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003b5c60, 0x003cc15c)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A1][0] = 0u64;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x003cc18c);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00416210, 0x003cc18c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003cc19c);
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x00416210, 0x003cc19c)?;
                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x003cc42c;
                        continue;
                    }
                    pc = 0x003cc1ac;
                    continue;
                }
                0x003cc1ac => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x59a0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003cc1c4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc1d4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x003cf7b0,
                        f: ee::s14::SND_DspPassThru_Init,
                    }
                    .call(c, 0x003cc1d4)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc1dc;
                    continue;
                }
                0x003cc1dc => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc1f4);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc1f4)?;
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[A1][0] = 0x2;
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003cc210);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6));
                    call_ptr(c, 0x003c72f8, 0x003cc210)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc21c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    Callee {
                        addr: 0x003ca870,
                        f: ee::s14::SND_DspLowpass_Init,
                    }
                    .call(c, 0x003cc21c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc228);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003ca8a0,
                        f: ee::s14::SND_DspLowpass_SetParams,
                    }
                    .call(c, 0x003cc228)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc230;
                    continue;
                }
                0x003cc230 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc248);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc248)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc260);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    Callee {
                        addr: 0x003cee88,
                        f: ee::s14::SND_DspFirLowpass_Init,
                    }
                    .call(c, 0x003cc260)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc26c);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003ceed0,
                        f: ee::s14::SND_DspFirLowpass_SetCutoff,
                    }
                    .call(c, 0x003cc26c)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc274;
                    continue;
                }
                0x003cc274 => {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc28c);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc28c)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc2a4);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    Callee {
                        addr: 0x003ca990,
                        f: ee::s14::SND_DspFirHighpass_Init,
                    }
                    .call(c, 0x003cc2a4)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc2b0);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003ca9d8,
                        f: ee::s14::SND_DspFirHighpass_SetCutoff,
                    }
                    .call(c, 0x003cc2b0)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc2b8;
                    continue;
                }
                0x003cc2b8 => {
                    c.cycles += 22;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc2d0);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc2d0)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[A1][0] = 0x2;
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc2e4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6));
                    call_ptr(c, 0x003c72f8, 0x003cc2e4)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc2fc);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    Callee {
                        addr: 0x003cefb0,
                        f: ee::s14::SND_DspFirBandpass_Init,
                    }
                    .call(c, 0x003cc2fc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc308);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003ceff8,
                        f: ee::s14::SND_DspFirBandpass_SetRange,
                    }
                    .call(c, 0x003cc308)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc310;
                    continue;
                }
                0x003cc310 => {
                    c.cycles += 26;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc328);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc328)?;
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003cc344);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x003c72f8, 0x003cc344)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[A1][0] = 0x2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc358);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6));
                    call_ptr(c, 0x003c72f8, 0x003cc358)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc364);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    Callee {
                        addr: 0x003cf498,
                        f: ee::s14::SND_DspResonator_Init,
                    }
                    .call(c, 0x003cc364)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc370);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003cf4d0,
                        f: ee::s14::SND_DspResonator_SetParams,
                    }
                    .call(c, 0x003cc370)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc378;
                    continue;
                }
                0x003cc378 => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc390);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc390)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc39c);
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    Callee {
                        addr: 0x003cf128,
                        f: ee::s14::SND_DspGain_Init,
                    }
                    .call(c, 0x003cc39c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc3a8);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003cf158,
                        f: ee::s14::SND_DspGain_SetGain,
                    }
                    .call(c, 0x003cc3a8)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc3b0;
                    continue;
                }
                0x003cc3b0 => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc3c8);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003c72f8, 0x003cc3c8)?;
                    c.g[V1][0] = sx((c.g[S6][0] as u32) << 8);
                    c.g[A1][0] = 0x2;
                    c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x003cc3e0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    call_ptr(c, 0x003c72f8, 0x003cc3e0)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc3f0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    Callee {
                        addr: 0x003cf998,
                        f: ee::s14::SND_DspAllpass_Init,
                    }
                    .call(c, 0x003cc3f0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003cc3fc);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x003cf9d8,
                        f: ee::s14::SND_DspAllpass_SetParams,
                    }
                    .call(c, 0x003cc3fc)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc404;
                    continue;
                }
                0x003cc404 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc414);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x003cf2c0,
                        f: ee::s14::SND_DspSum_Init,
                    }
                    .call(c, 0x003cc414)?;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003cc430;
                        continue;
                    }
                    pc = 0x003cc41c;
                    continue;
                }
                0x003cc41c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003cc42c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x003cf6f8,
                        f: ee::s14::SND_DspBlockDelay_Init,
                    }
                    .call(c, 0x003cc42c)?;
                    pc = 0x003cc42c;
                    continue;
                }
                0x003cc42c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003cc430;
                    continue;
                }
                0x003cc430 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x9;
                    if t {
                        pc = 0x003cc4e4;
                        continue;
                    }
                    pc = 0x003cc438;
                    continue;
                }
                0x003cc438 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003cc4ac;
                        continue;
                    }
                    pc = 0x003cc444;
                    continue;
                }
                0x003cc444 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x510000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = (c.g[V0][0] < 0xbu64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x510000;
                    if t {
                        pc = 0x003cc4ac;
                        continue;
                    }
                    pc = 0x003cc45c;
                    continue;
                }
                0x003cc45c => {
                    c.cycles += 20;
                    c.g[S0][0] = 0x510000;
                    c.g[S1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d58));
                    c.g[A2][0] = c.g[S1][0] & 0xf;
                    c.g[RA][0] = sx(0x003cc478);
                    c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                    Callee {
                        addr: 0x003cbc20,
                        f: ee::s14::SND_EeFxLookupParam,
                    }
                    .call(c, 0x003cc478)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = 0x510000;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5d48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A2][0] = sx((c.g[S1][0] as u32) >> 4);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = 0x510000;
                    c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d58));
                    if t {
                        pc = 0x003cc4bc;
                        continue;
                    }
                    pc = 0x003cc4ac;
                    continue;
                }
                0x003cc4ac => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x510000;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d58));
                    c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x5b45));
                    c.g[A2][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003cc4bc;
                    continue;
                }
                0x003cc4bc => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x003cc4c4);
                    Callee {
                        addr: 0x003cbc20,
                        f: ee::s14::SND_EeFxLookupParam,
                    }
                    .call(c, 0x003cc4c4)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[A0][0] = 0x510000;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5d48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                    pc = 0x003cc4e4;
                    continue;
                }
                0x003cc4e4 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32));
                    c.g[A0][0] = 0x450000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1790));
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A1][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x003cc138) {
                            return Ok(());
                        }
                        pc = 0x003cc138;
                        continue;
                    }
                    pc = 0x003cc50c;
                    continue;
                }
                0x003cc50c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x510000;
                    c.g[S5][0] = 0u64;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d50));
                    let t = true;
                    c.g[S2][0] = sx((c.g[FP][0] as u32) << 2);
                    if t {
                        pc = 0x003cc568;
                        continue;
                    }
                    pc = 0x003cc520;
                    continue;
                }
                0x003cc520 => {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x003cc528);
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x003c72f8, 0x003cc528)?;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[S0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x003cc544);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    call_ptr(c, 0x003c72f8, 0x003cc544)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5d44));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x9));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[RA][0] = sx(0x003cc568);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x003c90c0,
                        f: ee::s14::SND_DspNode_Connect,
                    }
                    .call(c, 0x003cc568)?;
                    pc = 0x003cc568;
                    continue;
                }
                0x003cc568 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0x510000;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d40));
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x003cc580);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, 0x003c72f8, 0x003cc580)?;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        if back_edge_unwinds(c, 0x003cc520) {
                            return Ok(());
                        }
                        pc = 0x003cc520;
                        continue;
                    }
                    pc = 0x003cc58c;
                    continue;
                }
                0x003cc58c => {
                    c.cycles += 14;
                    c.g[A0][0] = 0x510000;
                    c.g[V0][0] = 0xbb8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d80));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x1800), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003cc5a8);
                    c.g[A2][0] = 0x800;
                    call_ptr(c, 0x00416210, 0x003cc5a8)?;
                    c.g[V0][0] = 0x510000;
                    c.g[A0][0] = 0x510000;
                    c.g[S2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5d4c));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7600), 0u64);
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x450000;
                    if t {
                        pc = 0x003cc5fc;
                        continue;
                    }
                    pc = 0x003cc5c4;
                    continue;
                }
                0x003cc5c4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x510000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1790));
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7600));
                    pc = 0x003cc5d8;
                    continue;
                }
                0x003cc5d8 => {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x7600), c.g[V1][0]);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lbu((c.g[S2][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7600));
                    if t {
                        if back_edge_unwinds(c, 0x003cc5d8) {
                            return Ok(());
                        }
                        pc = 0x003cc5d8;
                        continue;
                    }
                    pc = 0x003cc5fc;
                    continue;
                }
                0x003cc5fc => {
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
                }
                _ => {
                    c.jump_out(pc, 0x003cc070, 0x003cc62c);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and bump-allocator cursor of this test's `SND_MemAlloc` fake.
    const OC1094_CTRL: u32 = DATA + 0x2_0000;
    const OC1094_MAGIC: u32 = 0x1094_f00d;
    const OC1094_ARENA_NEXT: u32 = OC1094_CTRL + 4;
    /// Allocations (filter objects, tables) come from here.
    const OC1094_ARENA: u32 = DATA + 0x20_0000;
    /// Node records, links.
    const OC1094_RECORDS: u32 = DATA + 0x4_0000;
    const OC1094_LINKS: u32 = DATA + 0x5_0000;
    /// Effect bank header (`D_005175C0`), channel count, records/links
    /// pointers, the byte `SND_EeFxLookupParam` is given as its second argument.
    const OC1094_HEADER: u32 = 0x0051_75c0;
    const OC1094_CHANNELS: u32 = 0x0051_5d58;
    const OC1094_RECORDS_PTR: u32 = 0x0051_75f8;
    const OC1094_LINKS_PTR: u32 = 0x0051_75fc;
    const OC1094_LOOKUP_ARG: u32 = 0x0051_5b45;

    static OC1094_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// `SND_MemAlloc` with the gate on: a 16-byte aligned bump allocator.
    fn oc1094_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC1094_CTRL) != OC1094_MAGIC {
            return fakes::fall_back(c, &OC1094_PREV_ALLOC, ee::s13::f_003b5c60__generated);
        }
        let size = (c.g[A0][0] as u32).wrapping_add(0xf) & !0xf;
        let at = c.bus.read32(OC1094_ARENA_NEXT);
        c.bus.write32(OC1094_ARENA_NEXT, at + size);
        c.g[V0][0] = at as u64;
        c.cycles += 9;
        ret(c);
    }

    /// Record stride per kind (`D_0044E870`).
    const OC1094_STRIDE: [u32; 10] = [4, 8, 4, 4, 8, 8, 4, 8, 4, 4];

    fn oc1094_be(c: &mut Cpu, at: u32, n: u32, v: u32) {
        for i in 0..n {
            c.bus.write8(at + i, (v >> (8 * (n - 1 - i))) as u8);
        }
    }

    /// A bank of 0..6 nodes (all kinds over the seeds, record kind 0 ends the
    /// final count early), 0..3 links, version 10 or 11, 1..6 channels.
    fn oc1094_setup(c: &mut Cpu, seed: u32) {
        let mut r: Rng = fakes::machine(c, seed, 0x3c_c070, 0);
        elf_image(c);
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B5C60),
            oc1094_fake_alloc,
            &OC1094_PREV_ALLOC,
        );
        c.bus.write32(OC1094_CTRL, OC1094_MAGIC);
        c.bus.write32(OC1094_ARENA_NEXT, OC1094_ARENA);
        c.pc = 0x003c_c070;
        c.g[A0][0] = if seed % 2 == 0 { 44100 } else { 48000 };

        let channels = 1 + r.next_u32() % 6;
        let version = 10 + seed % 2;
        let count = if seed % 8 == 0 {
            0
        } else {
            1 + (seed / 2 + r.next_u32()) % 6
        };
        let links = if count == 0 { 0 } else { r.next_u32() % 4 };
        c.bus.write32(OC1094_CHANNELS, channels);
        c.bus.write32(OC1094_RECORDS_PTR, OC1094_RECORDS);
        c.bus.write32(OC1094_LINKS_PTR, OC1094_LINKS);
        c.bus
            .write8(OC1094_LOOKUP_ARG, (1 + r.next_u32() % 6) as u8);
        c.bus.write8(OC1094_HEADER + 2, version as u8);
        let last = channels - 1;
        oc1094_be(c, OC1094_HEADER + 8 + 4 * last, 4, count);
        oc1094_be(c, OC1094_HEADER + 0x20 + 4 * last, 4, links);
        let mut at = OC1094_RECORDS;
        for i in 0..count {
            let kind = (seed / 3 + 3 * i + r.next_u32() % 3) % 10;
            c.bus.write8(at, kind as u8);
            let ch = match r.next_u32() % 3 {
                0 => 0,
                _ if kind == 9 && version >= 11 => {
                    (1 + r.next_u32() % 6) << 4 | (1 + r.next_u32() % 6)
                }
                _ => 1 + r.next_u32() % 6,
            };
            c.bus.write8(at + 1, ch as u8);
            for b in 2..OC1094_STRIDE[kind as usize] {
                c.bus.write8(at + b, (r.next_u32() % 0x300) as u8);
            }
            at += OC1094_STRIDE[kind as usize];
        }
        for i in 0..links {
            let l = OC1094_LINKS + 12 * i;
            oc1094_be(c, l, 4, r.next_u32() % count);
            oc1094_be(c, l + 4, 4, r.next_u32() % count);
            c.bus.write8(l + 8, (1 + r.next_u32() % 3) as u8 & 3);
            c.bus.write8(l + 9, (1 + r.next_u32() % 3) as u8 & 3);
        }
    }

    /// `SND_EeFxBuild`: the effect graph builder against the generated body,
    /// with real callees (filter inits, `SND_ReadBigEndian`, memset,
    /// `SND_EeFxLookupParam`, `SND_DspNode_Connect`) and a bump-allocator fake
    /// for `SND_MemAlloc`.
    #[test]
    fn snd_ee_fx_build_matches() {
        check(
            ee::s14::f_003cc070__generated,
            snd_ee_fx_build,
            240,
            &oc1094_setup,
        );
        let mut paths = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = crate::difftest::fresh();
            oc1094_setup(&mut c, seed);
            let before = c.cycles;
            ee::s14::f_003cc070__generated(&mut c);
            paths.insert(c.cycles - before);
        }
        assert!(paths.len() >= 40, "paths {}", paths.len());
    }
}
