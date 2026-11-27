//! Staging module oc1099 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1099`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Global holding the SNDENC encoder state pointer (`0x510000 - 0x5398`).
pub(crate) const SNDENC_STATE: u32 = 0x0050_CC68;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_f3e0, sndenc_copy_block, "SNDENC_CopyBlock"),
    (0x003b_f440, sndenc_pcm_to_float, "SNDENC_PcmToFloat"),
    (0x003b_f4c0, sndenc_window_filter4, "SNDENC_WindowFilter4"),
    (0x003b_f5a0, sndenc_window_filter6, "SNDENC_WindowFilter6"),
    (0x003b_f6b0, sndenc_window_filter8, "SNDENC_WindowFilter8"),
    (0x003b_f800, sndenc_filter_stage1, "SNDENC_FilterStage1"),
];

/// `void SNDENC_PutBits(int value, int n)` (0x3bcae8): appends the low `n`
/// bits of `value` to the encoder's bit buffer. The state (`*SNDENC_STATE`)
/// holds the bit position `pos` (u16 at +6) and the byte buffer pointer (+8);
/// each pass fills the byte at `(pos >> 3) ^ 1` (odd/even order).
#[allow(dead_code)]
pub fn sndenc_put_bits(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T1][0] = c.g[A1][0];
    c.g[V0][0] = sx(0x20u32.wrapping_sub(c.g[T1][0] as u32));
    let shift = (c.g[V0][0] as u32) & 31;
    c.g[T3][0] = sx((c.g[A0][0] as u32) << shift);
    let n_le_0 = (c.g[T1][0] as i64) <= 0;
    c.g[T3][0] = sx((c.g[T3][0] as u32) >> shift);
    if n_le_0 {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[T2][0] = 0x51_0000;
    c.g[T4][0] = 0x8;
    loop {
        // Byte index of the next bit: (pos >> 3) with the odd/even flip.
        c.cycles += 6;
        c.g[V0][0] = c.lw(SNDENC_STATE);
        c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x6));
        c.g[T0][0] = sx((c.g[V1][0] as u32) >> 3);
        c.g[V0][0] = c.g[T0][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        } else {
            c.cycles += 1;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
        }
        // Free bits in that byte; clear it when it is untouched.
        c.cycles += 6;
        c.g[V1][0] = c.lw(SNDENC_STATE);
        c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
        c.g[V0][0] &= 0x7;
        c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        let room_not_8 = c.g[A3][0] != c.g[T4][0];
        c.g[V0][0] = c.lw(SNDENC_STATE);
        if !room_not_8 {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.sb(c.g[V0][0] as u32, 0u64);
            c.g[V0][0] = c.lw(SNDENC_STATE);
        }
        // Place as many bits as fit in this byte.
        c.cycles += 18;
        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
        c.g[A2][0] = c.g[A3][0];
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        if c.g[V1][0] == 0 {
            c.g[A2][0] = c.g[T1][0];
        }
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = sx((c.g[T3][0] as u32) >> ((c.g[A0][0] as u32) & 31));
        c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A3][0] as u32) & 31));
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sb(c.g[A1][0] as u32, c.g[V1][0]);
        // Advance the bit position by the bits placed.
        c.g[A0][0] = c.lw(SNDENC_STATE);
        c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x6));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let more = (c.g[T1][0] as i64) > 0;
        c.sh((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_cb08);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `void SNDENC_EncodeLfe(...)` (0x3bd078): LFE channel encoder. Runs the LFE
/// decimator (`SNDENC_LfeDecimate`) into a stack buffer, finds the peak of its
/// eight outputs, scales them by a table value and stores eight bytes.
#[allow(dead_code)]
pub fn sndenc_encode_lfe(c: &mut Cpu) {
    c.cycles += 22;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V0][0] = 0x51_0000;
    c.g[V1][0] = c.lw(SNDENC_STATE);
    c.g[A3][0] = 0x45_0000;
    let frame = c.g[SP][0] as u32;
    c.sq(16, frame.wrapping_add(0x40));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x2300));
    c.sq(17, frame.wrapping_add(0x30));
    c.g[S0][0] = c.g[A2][0];
    c.swc1(21, frame.wrapping_add(0x58));
    c.g[S1][0] = c.g[A1][0];
    c.swc1(20, frame.wrapping_add(0x50));
    c.g[A1][0] = c.g[SP][0];
    c.sd(frame.wrapping_add(0x20), c.g[RA][0]);
    c.mtc1(0, 21);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c));
    if (Callee {
        addr: 0x003c_06b0,
        f: ee::s13::f_003c06b0,
    })
    .call(c, 0x003b_d0b8)
    .is_err()
    {
        return;
    }
    c.g[A3][0] = 0x45_0000;
    c.g[V0][0] = c.g[SP][0];
    c.mov_s(20, 21);
    c.g[A1][0] = 0x7;
    c.lwc1(0, c.g[V0][0] as u32);
    // Running max into f21 and running min into f20 over the eight outputs.
    loop {
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.c_lt_s(21, 0);
        let not_below = !c.fcc();
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        if not_below {
            c.cycles += 4;
            c.c_lt_s(0, 20);
            if c.fcc() {
                c.mov_s(20, 0);
            }
        } else {
            c.cycles += 2;
            c.mov_s(21, 0);
        }
        c.cycles += 2;
        if (c.g[A1][0] as i64) < 0 {
            break;
        }
        c.lwc1(0, c.g[V0][0] as u32);
        if event_due(c) {
            deliver_event_here(c, 0x003b_d0d0);
        }
    }
    // Scale the peak against 4.0 and the table: the wider branch maps the
    // peak through the table at A3-0x38a0.
    c.cycles += 5;
    c.neg_s(20, 20);
    c.c_lt_s(21, 20);
    if c.fcc() {
        c.mov_s(21, 20);
    }
    c.cycles += 6;
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(1, 0);
    c.c_lt_s(21, 0);
    let below_four = c.fcc();
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x38a0));
    if below_four {
        c.cycles += 7;
        c.cvt_w_s(0, 21);
        c.mfc1(3, 0);
        c.g[V0][0] = 0x45_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22c8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = c.lb(c.g[V1][0] as u32);
    } else {
        c.cycles += 19;
        c.mfc1(3, 21);
        c.g[AT][0] = 0x352f_0000;
        c.g[AT][0] |= 0x2507;
        c.mtc1(1, 1);
        c.g[V0][0] = c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1);
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b8));
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.lwc1(0, c.g[V0][0] as u32);
        c.cvt_s_w(0, 0);
        c.c_lt_s(21, 0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b7));
        if c.fcc() {
            c.cycles += 1;
            c.g[A1][0] = c.g[A2][0];
        }
    }
    c.cycles += 15;
    c.sb(c.g[S1][0] as u32, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x38a0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[AT][0] = 0x41e4_0000;
    c.g[AT][0] |= 0x9249;
    c.mtc1(1, 2);
    c.lwc1(1, c.g[V0][0] as u32);
    c.cvt_s_w(1, 1);
    c.g[A1][0] = 0;
    c.g[A2][0] = c.g[SP][0];
    c.div_s(0, 0, 1);
    c.mul_s(1, 0, 2);
    // Eight output bytes: each float times the scale, stored at S0 + index.
    loop {
        c.cycles += 10;
        c.lwc1(0, c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        c.mul_s(0, 0, 1);
        c.g[V1][0] = ((c.g[A1][0] as i64) < 8) as u64;
        c.cvt_w_s(2, 0);
        c.mfc1(2, 2);
        let more = c.g[V1][0] != 0;
        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_d1d8);
            }
            continue;
        }
        break;
    }
    c.cycles += 7;
    let frame = c.g[SP][0] as u32;
    c.lq(16, frame.wrapping_add(0x40));
    c.lq(17, frame.wrapping_add(0x30));
    c.g[RA][0] = c.ld(frame.wrapping_add(0x20));
    c.lwc1(21, frame.wrapping_add(0x58));
    c.lwc1(20, frame.wrapping_add(0x50));
    c.g[SP][0] = sx(frame.wrapping_add(0x60));
    ret(c);
}

/// `void SNDENC_CopyBlock(src, dst, count)` (0x3bf3e0): copies `count >> 5`
/// blocks of 128 bytes through the VU0 quad registers. The loop is
/// do-while, so `count` must be at least 32.
pub fn sndenc_copy_block(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 5);
    loop {
        c.cycles += 20;
        let src = c.g[A0][0] as u32;
        let dst = c.g[A1][0] as u32;
        for k in 0..8u32 {
            c.lqc2((k + 1) as usize, src.wrapping_add(0x10 * k));
        }
        for k in 0..8u32 {
            c.sqc2((k + 1) as usize, dst.wrapping_add(0x10 * k));
        }
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = sx(src.wrapping_add(0x80));
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx(dst.wrapping_add(0x80));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f3e4);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `void SNDENC_PcmToFloat(src, dst, count)` (0x3bf440): converts `count >> 4`
/// blocks of sixteen 16-bit samples to floats through the VU0 unit.
pub fn sndenc_pcm_to_float(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 4);
    loop {
        c.cycles += 28;
        c.g[A3][0] = c.ld(c.g[A0][0] as u32);
        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
        c.g[T2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
        c.pextlh(7, 7, 0);
        c.pextlh(8, 8, 0);
        c.pextlh(9, 9, 0);
        c.pextlh(10, 10, 0);
        c.psraw(7, 7, 8);
        c.psraw(8, 8, 8);
        c.psraw(9, 9, 8);
        c.psraw(10, 10, 8);
        c.qmtc2(7, 1);
        c.qmtc2(8, 2);
        c.qmtc2(9, 3);
        c.qmtc2(10, 4);
        c.vu0().itof(0, 0b1111, 1, 1);
        c.vu0().itof(0, 0b1111, 2, 2);
        c.vu0().itof(0, 0b1111, 3, 3);
        c.vu0().itof(0, 0b1111, 4, 4);
        let out = c.g[A1][0] as u32;
        c.sqc2(1, out);
        c.sqc2(2, out.wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(3, out.wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
        c.sqc2(4, out.wrapping_add(0x30));
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx(out.wrapping_add(0x40));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f444);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `SNDENC_WindowFilter4` (0x3bf4c0): four-tap window filter over sixteen
/// 4-lane rows (`a0` coefficients, `a1` history, `a2` output).
pub fn sndenc_window_filter4(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = 0x10;
    loop {
        c.cycles += 51;
        let hist = c.g[A1][0] as u32;
        let coef = c.g[A0][0] as u32;
        for k in 0..4u32 {
            c.lqc2((k + 1) as usize, hist.wrapping_add(0x100 * k));
        }
        for k in 0..10u32 {
            c.lqc2((k + 5) as usize, coef.wrapping_add(0x80 * k));
        }
        c.vu0().mula(0b1111, 1, Src::Vec(5));
        c.vu0().madda(0b1111, 2, Src::Vec(7));
        c.vu0().madda(0b1111, 3, Src::Vec(9));
        c.vu0().madd(0b1111, 5, 4, Src::Vec(11));
        c.vu0().mula(0b1111, 1, Src::Vec(6));
        c.vu0().madda(0b1111, 2, Src::Vec(8));
        c.vu0().madda(0b1111, 3, Src::Vec(10));
        c.vu0().madd(0b1111, 6, 4, Src::Vec(12));
        c.vu0().mula(0b1111, 1, Src::Vec(7));
        c.vu0().madda(0b1111, 2, Src::Vec(9));
        c.vu0().madda(0b1111, 3, Src::Vec(11));
        c.vu0().madd(0b1111, 7, 4, Src::Vec(13));
        c.vu0().mula(0b1111, 1, Src::Vec(8));
        c.vu0().madda(0b1111, 2, Src::Vec(10));
        c.vu0().madda(0b1111, 3, Src::Vec(12));
        c.vu0().madd(0b1111, 8, 4, Src::Vec(14));
        c.qmfc2(7, 5);
        c.qmfc2(8, 6);
        c.qmfc2(9, 7);
        c.qmfc2(10, 8);
        c.pextlw(11, 8, 7);
        c.pextuw(8, 8, 7);
        c.pextlw(12, 10, 9);
        c.pextuw(10, 10, 9);
        c.pcpyld(7, 12, 11);
        c.pcpyud(9, 11, 12);
        c.pcpyld(11, 10, 8);
        c.pcpyud(12, 8, 10);
        let out = c.g[A2][0] as u32;
        c.sq(7, out);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sq(9, out.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(11, out.wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.sq(12, out.wrapping_add(0x30));
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(out.wrapping_add(0x40));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f4c4);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `SNDENC_WindowFilter6` (0x3bf5a0): six-tap window filter, sixteen rows.
pub fn sndenc_window_filter6(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = 0x10;
    loop {
        c.cycles += 65;
        let hist = c.g[A1][0] as u32;
        let coef = c.g[A0][0] as u32;
        for k in 0..6u32 {
            c.lqc2((k + 1) as usize, hist.wrapping_add(0x100 * k));
        }
        for k in 0..14u32 {
            c.lqc2((k + 7) as usize, coef.wrapping_add(0x80 * k));
        }
        c.vu0().mula(0b1111, 1, Src::Vec(7));
        c.vu0().madda(0b1111, 2, Src::Vec(9));
        c.vu0().madda(0b1111, 3, Src::Vec(11));
        c.vu0().madda(0b1111, 4, Src::Vec(13));
        c.vu0().madda(0b1111, 5, Src::Vec(15));
        c.vu0().madd(0b1111, 7, 6, Src::Vec(17));
        c.vu0().mula(0b1111, 1, Src::Vec(8));
        c.vu0().madda(0b1111, 2, Src::Vec(10));
        c.vu0().madda(0b1111, 3, Src::Vec(12));
        c.vu0().madda(0b1111, 4, Src::Vec(14));
        c.vu0().madda(0b1111, 5, Src::Vec(16));
        c.vu0().madd(0b1111, 8, 6, Src::Vec(18));
        c.vu0().mula(0b1111, 1, Src::Vec(9));
        c.vu0().madda(0b1111, 2, Src::Vec(11));
        c.vu0().madda(0b1111, 3, Src::Vec(13));
        c.vu0().madda(0b1111, 4, Src::Vec(15));
        c.vu0().madda(0b1111, 5, Src::Vec(17));
        c.vu0().madd(0b1111, 9, 6, Src::Vec(19));
        c.vu0().mula(0b1111, 1, Src::Vec(10));
        c.vu0().madda(0b1111, 2, Src::Vec(12));
        c.vu0().madda(0b1111, 3, Src::Vec(14));
        c.vu0().madda(0b1111, 4, Src::Vec(16));
        c.vu0().madda(0b1111, 5, Src::Vec(18));
        c.vu0().madd(0b1111, 10, 6, Src::Vec(20));
        c.qmfc2(7, 7);
        c.qmfc2(8, 8);
        c.qmfc2(9, 9);
        c.qmfc2(10, 10);
        c.pextlw(11, 8, 7);
        c.pextuw(8, 8, 7);
        c.pextlw(12, 10, 9);
        c.pextuw(10, 10, 9);
        c.pcpyld(7, 12, 11);
        c.pcpyud(9, 11, 12);
        c.pcpyld(11, 10, 8);
        c.pcpyud(12, 8, 10);
        let out = c.g[A2][0] as u32;
        c.sq(7, out);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sq(9, out.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(11, out.wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.sq(12, out.wrapping_add(0x30));
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(out.wrapping_add(0x40));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f5a4);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `SNDENC_WindowFilter8` (0x3bf6b0): eight-tap window filter, sixteen rows.
pub fn sndenc_window_filter8(c: &mut Cpu) {
    c.cycles += 1;
    c.g[V0][0] = 0x10;
    loop {
        c.cycles += 79;
        let hist = c.g[A1][0] as u32;
        let coef = c.g[A0][0] as u32;
        for k in 0..8u32 {
            c.lqc2((k + 1) as usize, hist.wrapping_add(0x100 * k));
        }
        for k in 0..18u32 {
            c.lqc2((k + 9) as usize, coef.wrapping_add(0x80 * k));
        }
        c.vu0().mula(0b1111, 1, Src::Vec(9));
        c.vu0().madda(0b1111, 2, Src::Vec(11));
        c.vu0().madda(0b1111, 3, Src::Vec(13));
        c.vu0().madda(0b1111, 4, Src::Vec(15));
        c.vu0().madda(0b1111, 5, Src::Vec(17));
        c.vu0().madda(0b1111, 6, Src::Vec(19));
        c.vu0().madda(0b1111, 7, Src::Vec(21));
        c.vu0().madd(0b1111, 9, 8, Src::Vec(23));
        c.vu0().mula(0b1111, 1, Src::Vec(10));
        c.vu0().madda(0b1111, 2, Src::Vec(12));
        c.vu0().madda(0b1111, 3, Src::Vec(14));
        c.vu0().madda(0b1111, 4, Src::Vec(16));
        c.vu0().madda(0b1111, 5, Src::Vec(18));
        c.vu0().madda(0b1111, 6, Src::Vec(20));
        c.vu0().madda(0b1111, 7, Src::Vec(22));
        c.vu0().madd(0b1111, 10, 8, Src::Vec(24));
        c.vu0().mula(0b1111, 1, Src::Vec(11));
        c.vu0().madda(0b1111, 2, Src::Vec(13));
        c.vu0().madda(0b1111, 3, Src::Vec(15));
        c.vu0().madda(0b1111, 4, Src::Vec(17));
        c.vu0().madda(0b1111, 5, Src::Vec(19));
        c.vu0().madda(0b1111, 6, Src::Vec(21));
        c.vu0().madda(0b1111, 7, Src::Vec(23));
        c.vu0().madd(0b1111, 11, 8, Src::Vec(25));
        c.vu0().mula(0b1111, 1, Src::Vec(12));
        c.vu0().madda(0b1111, 2, Src::Vec(14));
        c.vu0().madda(0b1111, 3, Src::Vec(16));
        c.vu0().madda(0b1111, 4, Src::Vec(18));
        c.vu0().madda(0b1111, 5, Src::Vec(20));
        c.vu0().madda(0b1111, 6, Src::Vec(22));
        c.vu0().madda(0b1111, 7, Src::Vec(24));
        c.vu0().madd(0b1111, 12, 8, Src::Vec(26));
        c.qmfc2(7, 9);
        c.qmfc2(8, 10);
        c.qmfc2(9, 11);
        c.qmfc2(10, 12);
        c.pextlw(11, 8, 7);
        c.pextuw(8, 8, 7);
        c.pextlw(12, 10, 9);
        c.pextuw(10, 10, 9);
        c.pcpyld(7, 12, 11);
        c.pcpyud(9, 11, 12);
        c.pcpyld(11, 10, 8);
        c.pcpyud(12, 8, 10);
        let out = c.g[A2][0] as u32;
        c.sq(7, out);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sq(9, out.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sq(11, out.wrapping_add(0x20));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.sq(12, out.wrapping_add(0x30));
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(out.wrapping_add(0x40));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f6b4);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

/// `SNDENC_FilterStage1` (0x3bf800): one butterfly stage over four 4-lane
/// blocks, working forward from `a0` and backward from `a0 + 0x1f0`.
pub fn sndenc_filter_stage1(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f0));
    loop {
        c.cycles += 48;
        let fwd = c.g[A0][0] as u32;
        let bwd = c.g[A1][0] as u32;
        c.lqc2(9, fwd.wrapping_add(0x200));
        c.lqc2(10, fwd.wrapping_add(0x210));
        c.lqc2(11, fwd.wrapping_add(0x220));
        c.lqc2(12, fwd.wrapping_add(0x230));
        c.lqc2(13, bwd.wrapping_add(0x1d0));
        c.lqc2(14, bwd.wrapping_add(0x1e0));
        c.lqc2(15, bwd.wrapping_add(0x1f0));
        c.lqc2(16, bwd.wrapping_add(0x200));
        c.lqc2(1, fwd);
        c.lqc2(2, fwd.wrapping_add(0x10));
        c.lqc2(3, fwd.wrapping_add(0x20));
        c.lqc2(4, fwd.wrapping_add(0x30));
        c.lqc2(5, bwd.wrapping_sub(0x30));
        c.lqc2(6, bwd.wrapping_sub(0x20));
        c.lqc2(7, bwd.wrapping_sub(0x10));
        c.lqc2(8, bwd);
        c.vu0().add(0b1111, 16, 9, Src::Vec(16));
        c.vu0().add(0b1111, 15, 10, Src::Vec(15));
        c.vu0().add(0b1111, 14, 11, Src::Vec(14));
        c.vu0().add(0b1111, 13, 12, Src::Vec(13));
        c.vu0().sub(0b1111, 9, 1, Src::Vec(8));
        c.vu0().sub(0b1111, 10, 2, Src::Vec(7));
        c.vu0().sub(0b1111, 11, 3, Src::Vec(6));
        c.vu0().sub(0b1111, 12, 4, Src::Vec(5));
        c.vu0().sub(0b1111, 8, 8, Src::Vec(1));
        c.vu0().sub(0b1111, 7, 7, Src::Vec(2));
        c.vu0().sub(0b1111, 6, 6, Src::Vec(3));
        c.vu0().sub(0b1111, 5, 5, Src::Vec(4));
        c.vu0().sub(0b1111, 1, 9, Src::Vec(16));
        c.vu0().sub(0b1111, 2, 10, Src::Vec(15));
        c.vu0().sub(0b1111, 3, 11, Src::Vec(14));
        c.vu0().sub(0b1111, 4, 12, Src::Vec(13));
        c.vu0().sub(0b1111, 8, 8, Src::Vec(16));
        c.vu0().sub(0b1111, 7, 7, Src::Vec(15));
        c.vu0().sub(0b1111, 6, 6, Src::Vec(14));
        c.vu0().sub(0b1111, 5, 5, Src::Vec(13));
        c.sqc2(1, fwd);
        c.sqc2(2, fwd.wrapping_add(0x10));
        c.sqc2(3, fwd.wrapping_add(0x20));
        c.sqc2(4, fwd.wrapping_add(0x30));
        c.sqc2(5, bwd.wrapping_sub(0x30));
        c.sqc2(6, bwd.wrapping_sub(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sqc2(7, bwd.wrapping_sub(0x10));
        c.g[A0][0] = sx(fwd.wrapping_add(0x40));
        c.sqc2(8, bwd);
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx(bwd.wrapping_sub(0x40));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_f808);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{elf_image, scramble, Rng, DATA};

    /// Fills `len` bytes at `base` with random words.
    fn fill(c: &mut Cpu, r: &mut Rng, base: u32, len: u32) {
        for k in (0..len).step_by(4) {
            c.bus.write32(base + k, r.next_u32());
        }
    }

    #[test]
    fn sndenc_copy_block_matches() {
        check(
            ssx3_ee::s13::f_003bf3e0__generated,
            sndenc_copy_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf3_e0 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x400);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(32 * (1 + seed % 4) + r.next_u32() % 32);
                c.pc = 0x003b_f3e0;
            },
        );
    }

    #[test]
    fn sndenc_pcm_to_float_matches() {
        check(
            ssx3_ee::s13::f_003bf440__generated,
            sndenc_pcm_to_float,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf4_40 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x400);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(16 * (1 + seed % 4) + r.next_u32() % 16);
                c.pc = 0x003b_f440;
            },
        );
    }

    #[test]
    fn sndenc_window_filter4_matches() {
        check(
            ssx3_ee::s13::f_003bf4c0__generated,
            sndenc_window_filter4,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf4_c0 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x900);
                fill(c, &mut r, DATA + 0x5000, 0x900);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x6000);
                c.pc = 0x003b_f4c0;
            },
        );
    }

    #[test]
    fn sndenc_window_filter6_matches() {
        check(
            ssx3_ee::s13::f_003bf5a0__generated,
            sndenc_window_filter6,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf5_a0 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x900);
                fill(c, &mut r, DATA + 0x5000, 0x900);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x6000);
                c.pc = 0x003b_f5a0;
            },
        );
    }

    #[test]
    fn sndenc_window_filter8_matches() {
        check(
            ssx3_ee::s13::f_003bf6b0__generated,
            sndenc_window_filter8,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf6_b0 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x900);
                fill(c, &mut r, DATA + 0x5000, 0x900);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x6000);
                c.pc = 0x003b_f6b0;
            },
        );
    }

    #[test]
    fn sndenc_filter_stage1_matches() {
        check(
            ssx3_ee::s13::f_003bf800__generated,
            sndenc_filter_stage1,
            200,
            &|c, seed| {
                let mut r = Rng(0x3bf8_00 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x4000, 0x420);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.pc = 0x003b_f800;
            },
        );
    }
}
