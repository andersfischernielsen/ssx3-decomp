//! Staging module oc1101 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1101`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_0564, sndenc_quantize10, "SNDENC_Quantize10"),
    (0x003c_05e0, sndenc_quantize8, "SNDENC_Quantize8"),
    (0x003c_0640, sndenc_quantize5, "SNDENC_Quantize5"),
    (0x003c_06b0, sndenc_lfe_decimate, "SNDENC_LfeDecimate"),
    (0x003c_09b8, snd_save_spr_dma, "SND_SaveSprDma"),
    (0x003c_0a60, snd_restore_spr_dma, "SND_RestoreSprDma"),
    (0x003c_15d8, snd_str_cat, "SND_StrCat"),
    (0x003c_3450, snd_iop_set_effect, "SND_IopSetEffect"),
];

/// `SNDENC_Quantize10(in /*a0*/, out /*a1*/, scale /*f12*/)` (0x3c0564): scales
/// 16 floats by `scale` (VU0), truncates them to ints, packs them into 16-bit
/// lanes and masks each with 0x3ff. Writes two quadwords. Leaf, 29 cycles.
pub fn sndenc_quantize10(c: &mut Cpu) {
    c.cycles += 29;
    c.mfc1(6, 12);
    c.qmtc2(6, 5);
    c.lqc2(1, (c.g[A0][0] as u32));
    c.g[T2][0] = 0x3ff;
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.pcpyh(10, 10);
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.pcpyld(10, 10, 10);
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.vu0().mul(0b1111, 1, 1, Src::Bc(5, X));
    c.vu0().mul(0b1111, 2, 2, Src::Bc(5, X));
    c.vu0().mul(0b1111, 3, 3, Src::Bc(5, X));
    c.vu0().mul(0b1111, 4, 4, Src::Bc(5, X));
    c.vu0().ftoi(0, 0b1111, 1, 1);
    c.vu0().ftoi(0, 0b1111, 2, 2);
    c.vu0().ftoi(0, 0b1111, 3, 3);
    c.vu0().ftoi(0, 0b1111, 4, 4);
    c.qmfc2(6, 1);
    c.qmfc2(7, 2);
    c.qmfc2(8, 3);
    c.qmfc2(9, 4);
    c.ppach(6, 7, 6);
    c.ppach(8, 9, 8);
    c.pand(6, 6, 10);
    c.pand(8, 8, 10);
    c.sq(6, (c.g[A1][0] as u32));
    c.sq(8, (c.g[A1][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `SNDENC_Quantize8(in /*a0*/, out /*a1*/, scale /*f12*/)` (0x3c05e0): scales
/// 16 floats by `scale`, truncates to ints and packs them into one quadword of
/// 8-bit lanes. Leaf, 24 cycles.
pub fn sndenc_quantize8(c: &mut Cpu) {
    c.cycles += 24;
    c.mfc1(6, 12);
    c.qmtc2(6, 5);
    c.lqc2(1, (c.g[A0][0] as u32));
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.vu0().mul(0b1111, 1, 1, Src::Bc(5, X));
    c.vu0().mul(0b1111, 2, 2, Src::Bc(5, X));
    c.vu0().mul(0b1111, 3, 3, Src::Bc(5, X));
    c.vu0().mul(0b1111, 4, 4, Src::Bc(5, X));
    c.vu0().ftoi(0, 0b1111, 1, 1);
    c.vu0().ftoi(0, 0b1111, 2, 2);
    c.vu0().ftoi(0, 0b1111, 3, 3);
    c.vu0().ftoi(0, 0b1111, 4, 4);
    c.qmfc2(6, 1);
    c.qmfc2(7, 2);
    c.qmfc2(8, 3);
    c.qmfc2(9, 4);
    c.ppach(6, 7, 6);
    c.ppach(8, 9, 8);
    c.ppacb(6, 8, 6);
    c.sq(6, (c.g[A1][0] as u32));
    ret(c);
}

/// `SNDENC_Quantize5(in /*a0*/, out /*a1*/, scale /*f12*/)` (0x3c0640): as
/// `SNDENC_Quantize8`, with the lanes masked by 0x1f1f. One quadword out.
/// Leaf, 28 cycles.
pub fn sndenc_quantize5(c: &mut Cpu) {
    c.cycles += 28;
    c.mfc1(6, 12);
    c.qmtc2(6, 5);
    c.lqc2(1, (c.g[A0][0] as u32));
    c.g[T2][0] = 0x1f1f;
    c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.pcpyh(10, 10);
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.pcpyld(10, 10, 10);
    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.vu0().mul(0b1111, 1, 1, Src::Bc(5, X));
    c.vu0().mul(0b1111, 2, 2, Src::Bc(5, X));
    c.vu0().mul(0b1111, 3, 3, Src::Bc(5, X));
    c.vu0().mul(0b1111, 4, 4, Src::Bc(5, X));
    c.vu0().ftoi(0, 0b1111, 1, 1);
    c.vu0().ftoi(0, 0b1111, 2, 2);
    c.vu0().ftoi(0, 0b1111, 3, 3);
    c.vu0().ftoi(0, 0b1111, 4, 4);
    c.qmfc2(6, 1);
    c.qmfc2(7, 2);
    c.qmfc2(8, 3);
    c.qmfc2(9, 4);
    c.ppach(6, 7, 6);
    c.ppach(8, 9, 8);
    c.ppacb(6, 8, 6);
    c.pand(6, 6, 10);
    c.sq(6, (c.g[A1][0] as u32));
    ret(c);
}

/// `SNDENC_LfeDecimate(in /*a0*/ (s16*), out /*a1*/ (f32*), state /*a2*/ (6 f32),
/// coef /*a3*/ (13 f32))` (0x3c06b0): the LFE channel's decimating FIR. Each of
/// the 8 outputs is a 64-tap dot product of the input (advancing a0 by 0x80
/// bytes per output) with the coefficients, accumulated in the FPU accumulator
/// in four interleaved sums; the six state floats are kept at the end.
///
/// Transcribed from the generated body: the inner loop's back edge and the
/// outer loop's back edge are the two event-check points (0x3c0708, 0x3c0704).
pub fn sndenc_lfe_decimate(c: &mut Cpu) {
    c.cycles += 21;
    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
    c.mtc1(0, 0);
    c.lwc1(1, (c.g[A3][0] as u32));
    c.lwc1(2, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.lwc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.lwc1(4, (c.g[A3][0] as u32).wrapping_add(0xc));
    c.lwc1(5, (c.g[A3][0] as u32).wrapping_add(0x10));
    c.lwc1(6, (c.g[A3][0] as u32).wrapping_add(0x14));
    c.lwc1(7, (c.g[A3][0] as u32).wrapping_add(0x18));
    c.lwc1(8, (c.g[A3][0] as u32).wrapping_add(0x1c));
    c.lwc1(9, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.lwc1(10, (c.g[A3][0] as u32).wrapping_add(0x24));
    c.lwc1(11, (c.g[A3][0] as u32).wrapping_add(0x28));
    c.lwc1(12, (c.g[A3][0] as u32).wrapping_add(0x2c));
    c.lwc1(13, (c.g[A3][0] as u32).wrapping_add(0x30));
    c.lwc1(20, (c.g[A2][0] as u32));
    c.lwc1(21, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.lwc1(22, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(23, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.lwc1(24, (c.g[A2][0] as u32).wrapping_add(0x10));
    c.lwc1(25, (c.g[A2][0] as u32).wrapping_add(0x14));
    'outer: loop {
        c.cycles += 1;
        c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
        loop {
            c.cycles += 27;
            c.g[T4][0] = c.lh((c.g[A0][0] as u32));
            c.mtc1(12, 31);
            c.cvt_s_w(31, 31);
            c.mula_s(1, 31);
            c.madda_s(20, 2);
            c.madda_s(21, 3);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.madd_s(31, 0, 0);
            c.madda_s(20, 4);
            c.madda_s(21, 5);
            c.mov_s(20, 21);
            c.mov_s(21, 31);
            c.madda_s(22, 6);
            c.madda_s(23, 7);
            c.madd_s(31, 0, 0);
            c.madda_s(22, 8);
            c.madda_s(23, 9);
            c.mov_s(22, 23);
            c.mov_s(23, 31);
            c.madda_s(24, 10);
            c.madda_s(25, 11);
            c.madd_s(31, 0, 0);
            c.madda_s(24, 12);
            c.madda_s(25, 13);
            c.mov_s(24, 25);
            let more_taps = c.g[T1][0] != c.g[A0][0];
            c.mov_s(25, 31);
            if more_taps {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_0708);
                }
                continue;
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.madd_s(31, 0, 0);
            let more_outputs = c.g[T0][0] != c.g[A1][0];
            c.swc1(31, (c.g[A1][0] as u32).wrapping_sub(0x4));
            if more_outputs {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_0704);
                }
                continue 'outer;
            }
            c.cycles += 8;
            c.swc1(20, (c.g[A2][0] as u32));
            c.swc1(21, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.swc1(22, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.swc1(23, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.swc1(24, (c.g[A2][0] as u32).wrapping_add(0x10));
            c.swc1(25, (c.g[A2][0] as u32).wrapping_add(0x14));
            ret(c);
            return;
        }
    }
}

/// `SND_SaveSprDma(...)` (0x3c09b8): saves the DMA state of the sound system
/// to the scratchpad: reads the DMA channel registers (0x1000_d000 block) and
/// copies them into the sound globals at 0x50_ad00 + 0x1b1c.., then starts the
/// SPR-to-memory transfer through `SND_DmaFromSpr` (0x3c4a38), returning to
/// 0x3c0a54. A 0x10 frame holds `ra`.
pub fn snd_save_spr_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x1000_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xd010;
        c.g[T0][0] = 0x51_0000;
        c.g[V1][0] = 0x1000_0000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x5300));
        c.g[V1][0] = c.g[V1][0] | 0xd080;
        c.g[V0][0] = 0x1000_0000;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b1c), c.g[A0][0]);
        c.g[V0][0] = c.g[V0][0] | 0xd020;
        c.g[A1][0] = 0x1000_0000;
        c.g[T4][0] = 0x51_0000;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[A1][0] = c.g[A1][0] | 0xd410;
        c.g[T3][0] = 0x1000_0000;
        c.g[T2][0] = 0x1000_0000;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b20), c.g[A0][0]);
        c.g[T3][0] = c.g[T3][0] | 0xd430;
        c.g[T2][0] = c.g[T2][0] | 0xd480;
        c.g[T1][0] = 0x1000_0000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[T1][0] = c.g[T1][0] | 0xd420;
        c.g[A2][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x1aae));
        c.g[A3][0] = 0x1_0000;
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b24), c.g[V1][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 4);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
        c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_sub(0x56b4));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b28), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1b38));
        c.g[V0][0] = c.lw((c.g[T3][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b2c), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T2][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b30), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[T1][0] as u32));
        c.sw((c.g[T0][0] as u32).wrapping_add(0x1b34), c.g[V0][0]);
        Callee {
            addr: 0x003c_4a38,
            f: ee::s13::f_003c4a38,
        }
        .call(c, 0x003c_0a54)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SND_RestoreSprDma(...)` (0x3c0a60): the counterpart of `SND_SaveSprDma`.
/// Pushes the saved sound DMA state back to the channel registers through
/// `SND_DmaToSpr` (0x3c4978, returning to 0x3c0a90), then reloads the channel
/// addresses and counts. A 0x20 frame holds `ra` and s0.
pub fn snd_restore_spr_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x1_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x51_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x5300));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x56b4));
        c.g[A2][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1aae));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b38));
        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 4);
        Callee {
            addr: 0x003c_4978,
            f: ee::s13::f_003c4978,
        }
        .call(c, 0x003c_0a90)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b1c));
        c.g[V0][0] = 0x1000_0000;
        c.g[V0][0] = c.g[V0][0] | 0xd010;
        c.g[V1][0] = 0x1000_0000;
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0xd080;
        c.g[A3][0] = 0x1000_0000;
        c.g[T0][0] = 0x1000_0000;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b20));
        c.g[A3][0] = c.g[A3][0] | 0xd020;
        c.g[T0][0] = c.g[T0][0] | 0xd410;
        c.g[A1][0] = 0x1000_0000;
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        c.g[A1][0] = c.g[A1][0] | 0xd430;
        c.g[A0][0] = 0x1000_0000;
        c.g[A2][0] = 0x1000_0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b24));
        c.g[A0][0] = c.g[A0][0] | 0xd480;
        c.g[A2][0] = c.g[A2][0] | 0xd420;
        frame.restore_ra(c, 0);
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b28));
        c.sw((c.g[T0][0] as u32), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b2c));
        c.sw((c.g[A1][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b30));
        c.sw((c.g[A0][0] as u32), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b34));
        frame.restore(c, S0, 0x10);
        c.sw((c.g[A2][0] as u32), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `char *SND_StrCat(char *dst /*a0*/, const char *src /*a1*/)` (0x3c15d8):
/// appends `src` to `dst` (strcat). Walks `dst` to its terminator (the event
/// check at 0x3c15e8 sits on that walk), then copies `src` including the
/// terminator's place (the copy loop's event check at 0x3c1610), and writes the
/// final NUL. Leaves `v0` as the first byte of `src`, `v1` as the next byte,
/// the same as the generated body. Leaf, no callees.
pub fn snd_str_cat(c: &mut Cpu) {
    'find: {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
        if c.g[V0][0] == 0u64 {
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
            break 'find;
        }
        c.cycles += 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x003c_15e8);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
            break 'find;
        }
    }
    c.cycles += 2;
    let src_empty = c.g[V0][0] == 0u64;
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lbu(c);
    if !src_empty {
        loop {
            c.cycles += 7;
            let byte = c.g[V1][0];
            Addr(c.g[A0][0] as u32).sb(c, byte);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
            let more = c.g[V0][0] != 0u64;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lbu(c);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003c_1610);
                }
                continue;
            }
            break;
        }
    }
    c.cycles += 2;
    Addr(c.g[A0][0] as u32).sb(c, 0u64);
    ret(c);
}

/// `SND_IopSetEffect(a0 = effect index)` (0x3c3450): sets the sound-effect
/// request word in the IOP mailbox (0x515b40) from the effect in `a0` and the
/// flag at `*(s0 + 0x42)`, then sends the RPC `SND_IopCallRpc` (0x3c0b10).
///
/// With `a0 == 1` the request goes out only once, through the second RPC
/// after `SND_ReadBigEndian` (0x3c72f8) reads the 4-byte parameter. With any
/// other `a0` the first RPC (`a2 = 4`) is sent and the function returns.
/// A 0x20 frame holds `ra` and s0.
pub fn snd_iop_set_effect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x550c));
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20c));
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x42)).lbu(c);
        let flag_clear = c.g[V0][0] == 0u64;
        c.g[A2][0] = c.g[A0][0];
        if !flag_clear {
            c.cycles += 7;
            c.g[V0][0] = 0x51_0000;
            c.g[V1][0] = 0x3d_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b40));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3788));
            c.g[A0][0] = 0x1;
            let request = c.g[V1][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x10)).sw(c, request);
            Addr((c.g[V0][0] as u32).wrapping_add(0xf)).sb(c, 1);
        }
        // Block B: a0 == 1 skips the first RPC and goes on to block D.
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let is_one = c.g[A2][0] == c.g[V0][0];
        c.g[A0][0] = 0x2;
        if !is_one {
            c.cycles += 7;
            c.g[A1][0] = 0x45_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2290));
            c.g[A2][0] = 0x4;
            Callee {
                addr: 0x003c_0b10,
                f: ee::s13::f_003c0b10,
            }
            .call(c, 0x003c_34ac)?;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x42)).lbu(c);
            let flag_clear = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x51_0000;
            if !flag_clear {
                c.cycles += 2;
                Addr((c.g[V0][0] as u32).wrapping_add(0x5b4f)).sb(c, 0u64);
            }
        } else {
            // Block D: the flag again, then the state pointer is reloaded.
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x42)).lbu(c);
            let flag_clear = c.g[V0][0] == 0u64;
            c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
            if !flag_clear {
                c.cycles += 2;
                c.g[V0][0] = 0x51_0000;
                let mode = c.g[A2][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x5b4f)).sb(c, mode);
            }
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x4;
            Callee {
                addr: 0x003c_72f8,
                f: ee::s13::f_003c72f8,
            }
            .call(c, 0x003c_34e0)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = 0x2;
            Callee {
                addr: 0x003c_0b10,
                f: ee::s13::f_003c0b10,
            }
            .call(c, 0x003c_34f0)?;
        }
        // The common epilogue: both branches end here.
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate magic: a gated fake answers only when its own word holds this
    /// value in the test machine's RAM.
    const MAGIC: u32 = 0x1101_c0de;

    /// One gate word per fake, in an otherwise unused part of the test data area.
    const fn gate(k: u32) -> u32 {
        DATA + 0x21_0000 + 4 * k
    }
    const GATE_DMA_FROM: u32 = gate(0);
    const GATE_DMA_TO: u32 = gate(1);
    const GATE_IOP_RPC: u32 = gate(2);

    fn gates_on(c: &mut Cpu) {
        for g in [GATE_DMA_FROM, GATE_DMA_TO, GATE_IOP_RPC] {
            c.bus.write32(g, MAGIC);
        }
    }

    /// A gated fake: with its gate on, it logs the call and returns to the
    /// caller's `ra`; with it off, it runs the body the slot held before.
    macro_rules! gated {
        ($fake:ident, $prev:ident, $gate:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated!(
        oc1101_fake_dma_from,
        OC1101_PREV_DMA_FROM,
        GATE_DMA_FROM,
        0x003c_4a38,
        ee::s13::f_003c4a38__generated
    );
    gated!(
        oc1101_fake_dma_to,
        OC1101_PREV_DMA_TO,
        GATE_DMA_TO,
        0x003c_4978,
        ee::s13::f_003c4978__generated
    );
    gated!(
        oc1101_fake_iop_rpc,
        OC1101_PREV_IOP_RPC,
        GATE_IOP_RPC,
        0x003c_0b10,
        ee::s13::f_003c0b10__generated
    );

    /// Puts the gated fakes in their override slots (once per process).
    fn install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C4A38),
            oc1101_fake_dma_from,
            &OC1101_PREV_DMA_FROM,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C4978),
            oc1101_fake_dma_to,
            &OC1101_PREV_DMA_TO,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C0B10),
            oc1101_fake_iop_rpc,
            &OC1101_PREV_IOP_RPC,
        );
    }

    /// Scrambled machine with the call hooks, the gates on, and unwinding
    /// fakes some of the time.
    fn call_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 2);
        gates_on(c);
        r
    }

    /// Random input, coefficient and output areas for the quantisers.
    fn quant_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1101_0001 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.f[12] = r.f32(4.0);
        fill(c, &mut r, DATA, 0x10);
        fill(c, &mut r, DATA + 0x2000, 0x8);
    }

    #[test]
    fn sndenc_quantize10_matches() {
        install();
        check(
            ee::s13::f_003c0564__generated,
            sndenc_quantize10,
            60,
            &|c, seed| quant_fixture(c, seed),
        );
    }

    #[test]
    fn sndenc_quantize8_matches() {
        install();
        check(
            ee::s13::f_003c05e0__generated,
            sndenc_quantize8,
            60,
            &|c, seed| quant_fixture(c, seed),
        );
    }

    #[test]
    fn sndenc_quantize5_matches() {
        install();
        check(
            ee::s13::f_003c0640__generated,
            sndenc_quantize5,
            60,
            &|c, seed| quant_fixture(c, seed),
        );
    }

    #[test]
    fn sndenc_lfe_decimate_matches() {
        install();
        check(
            ee::s13::f_003c06b0__generated,
            sndenc_lfe_decimate,
            40,
            &|c, seed| {
                let mut r = Rng(0x1101_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x3000);
                // Eight outputs of 64 taps: 0x400 bytes of input.
                fill(c, &mut r, DATA, 0x100);
                fill(c, &mut r, DATA + 0x1000, 0x8);
                for k in 0..6 {
                    c.bus.write32(DATA + 0x2000 + 4 * k, r.f32(1.0));
                }
                for k in 0..13 {
                    c.bus.write32(DATA + 0x3000 + 4 * k, r.f32(1.0));
                }
            },
        );
    }

    #[test]
    fn snd_save_spr_dma_matches() {
        install();
        check(
            ee::s13::f_003c09b8__generated,
            snd_save_spr_dma,
            60,
            &|c, seed| {
                call_fixture(c, seed, 0x1101_0009);
            },
        );
    }

    #[test]
    fn snd_restore_spr_dma_matches() {
        install();
        check(
            ee::s13::f_003c0a60__generated,
            snd_restore_spr_dma,
            60,
            &|c, seed| {
                call_fixture(c, seed, 0x1101_000a);
            },
        );
    }

    #[test]
    fn snd_str_cat_matches() {
        install();
        check(
            ee::s13::f_003c15d8__generated,
            snd_str_cat,
            60,
            &|c, seed| {
                let mut r = Rng(0x1101_0015 ^ seed as u64);
                scramble(c, &mut r);
                let dst = DATA + 0x100;
                let src = DATA + 0x200;
                for i in 0..32 {
                    Addr(dst + i).sb(c, 0);
                    Addr(src + i).sb(c, 0);
                }
                // Destination length 0..3 and source length 0..2: both
                // empty and non-empty paths of each loop.
                for i in 0..(seed % 4) {
                    Addr(dst + i).sb(c, (0x41 + i) as u64);
                }
                for i in 0..((seed / 4) % 3) {
                    Addr(src + i).sb(c, (0x61 + i) as u64);
                }
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = sx(src);
            },
        );
    }

    #[test]
    fn snd_iop_set_effect_matches() {
        install();
        check(
            ee::s13::f_003c3450__generated,
            snd_iop_set_effect,
            60,
            &|c, seed| {
                let mut r = call_fixture(c, seed, 0x1101_3450);
                // The effect index: 1 takes the second-RPC path.
                c.g[A0][0] = u64::from(seed % 3);
                // The game globals: the state pointer at 0x50aaf8 and the
                // flag byte at its +0x42 (0x50a92a).
                Addr(0x0050_aaf8).sw(c, u64::from(DATA + 0x100));
                Addr(0x0050_a92a).sb(c, u64::from((seed / 3) % 2));
                fill(c, &mut r, DATA + 0x100, 0x8);
            },
        );
    }
}
