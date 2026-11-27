//! Staging module oc1092 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1092`.

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
    0x003b_de08,
    sndenc_encode_channel_mode2,
    "SNDENC_EncodeChannelMode2",
)];

/// `SNDENC_EncodeChannelMode2(ch)` (0x3bde08): encode one channel of an audio
/// frame in the mode-2 layout. Finds the subband peaks of the shared encoder
/// state (D_0050AC6C), derives 24 scale-factor indices from them (peak times
/// the per-band factor; below 4.0 by a byte table, above by the float exponent
/// and a table lookup), then quantises the subbands in three groups (5 x 16,
/// 9 x 16 and 5 pairs of 16 values) and packs the results as bit fields into
/// the frame buffer (D_0050AC68->out + ch * 172 + 284).
///
/// Transcribed block by block from the generated body: the packing sequences
/// leave their scratch registers, byte for byte, in the call log.
pub fn sndenc_encode_channel_mode2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0x510000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = 0x510000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5394));
        c.g[S4][0] = c.g[V0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = sx(0x003bde64);
        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        call_ptr(c, 0x003c03e0, 0x003bde64)?; // SNDENC_FindSubbandPeaks
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = 0x450000;
        c.g[V1][0] = 0x450000;
        c.g[AT][0] = 0x40800000;
        c.mtc1(1, 3);
        c.g[AT][0] = 0x352f0000;
        c.g[AT][0] = c.g[AT][0] | 0x2507;
        c.mtc1(1, 2);
        c.g[A2][0] = c.g[S5][0];
        c.g[T2][0] = c.g[S4][0];
        c.g[T1][0] = c.g[S1][0];
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22c8));
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
        'l_3bde98: loop {
            'b_3bdf20: {
                'b_3bdedc: {
                    c.cycles += 12;
                    c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5394));
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.lwc1(0, (c.g[V1][0] as u32));
                    c.mul_s(1, 0, 1);
                    c.c_lt_s(1, 3);
                    let t = !c.fcc();

                    if t {
                        break 'b_3bdedc;
                    } else {
                        c.cycles += 5;
                        c.cvt_w_s(0, 1);
                        c.mfc1(2, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                        let t = true;
                        c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                        if t {
                            break 'b_3bdf20;
                        } else {
                            break 'b_3bdedc;
                        }
                    }
                }
                c.cycles += 16;
                c.mfc1(3, 1);
                c.g[V0][0] = c.g[V1][0];
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 2);
                c.cvt_w_s(4, 0);
                c.mfc1(3, 4);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b8));
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.cvt_s_w(0, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b7));
                if t {
                    break 'b_3bdf20;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[A1][0];
                    break 'b_3bdf20;
                }
            }
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S2][0] as u32)));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sb((c.g[V1][0] as u32), c.g[A0][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003bde98);
                }
                continue 'l_3bde98;
            } else {
                c.cycles += 11;
                c.g[V0][0] = 0xac;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
                c.mult(0, 2, 19, 2);
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5394));
                c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x450000;
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa00));
                c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
                c.g[S2][0] = 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11c));
                c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                'l_3bdf60: loop {
                    c.cycles += 138;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                    c.g[A2][0] = sx((c.g[S2][0] as u32) << 6);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5394));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.lwc1(12, (c.g[V0][0] as u32));
                    c.cvt_s_w(12, 12);
                    c.div_s(12, 0, 12);
                    c.g[RA][0] = sx(0x003bdf9c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    call_ptr(c, 0x003c0564, 0x003bdf9c)?; // SNDENC_Quantize10
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
                    c.g[A3][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 7);
                    c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xb));
                    c.g[V0][0] = c.g[V0][0] & 0xf8;
                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x35d));
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[A1][0] = c.g[A1][0] & 0x7;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V1][0]);
                    c.g[A2][0] = c.g[A2][0] & 0xf8;
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 9);
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.sb((c.g[S1][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 7);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 9);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V1][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[V0][0]);
                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xe));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 5);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xb), c.g[A1][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 7);
                    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35d), c.g[V0][0]);
                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x12));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x367));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 9);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.g[A1][0] = c.g[A1][0] & 0x7;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35c), c.g[V1][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x12));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35f), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x12));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35e), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x16));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x361), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x16));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 7);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x360), c.g[V0][0]);
                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 9);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x363), c.g[V1][0]);
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x362), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x365), c.g[V0][0]);
                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1e));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 5);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x364), c.g[V1][0]);
                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1e));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x367), c.g[A1][0]);
                    let t = c.g[A3][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003bdf60);
                        }
                        continue 'l_3bdf60;
                    } else {
                        'b_3be390: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x450000;
                            if t {
                                break 'b_3be390;
                            } else {
                                c.cycles += 1;
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x38a0));
                                'l_3be198: loop {
                                    c.cycles += 126;
                                    c.g[V1][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                                    c.g[A2][0] = sx((c.g[S2][0] as u32) << 6);
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5394));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                                    c.lwc1(12, (c.g[V0][0] as u32));
                                    c.cvt_s_w(12, 12);
                                    c.div_s(12, 0, 12);
                                    c.g[RA][0] = sx(0x003be1d4);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                    call_ptr(c, 0x003c05e0, 0x003be1d4)?; // SNDENC_Quantize8
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[A3][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
                                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 5);
                                    c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x9));
                                    c.g[V0][0] = c.g[V0][0] & 0xf8;
                                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x35d));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A1][0] = c.g[A1][0] & 0x7;
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V0][0]);
                                    c.g[A2][0] = c.g[A2][0] & 0xf8;
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[A0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[A0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[A0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[A1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35d), c.g[A2][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35c), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35f), c.g[A0][0]);
                                    c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x365));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                    c.g[A2][0] = c.g[A2][0] & 0x7;
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35e), c.g[A1][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x361), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x360), c.g[A0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x363), c.g[V1][0]);
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x362), c.g[A0][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x365), c.g[A2][0]);
                                    let t = c.g[A3][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003be198);
                                        }
                                        continue 'l_3be198;
                                    } else {
                                        break 'b_3be390;
                                    }
                                }
                            }
                        }
                        'b_3be6b4: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x450000;
                            if t {
                                break 'b_3be6b4;
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[S2][0] as u32) << 6);
                                c.g[S3][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 20);
                                c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x38a0));
                                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                                c.g[S5][0] = 0x510000;
                                'l_3be3b8: loop {
                                    c.cycles += 191;
                                    c.g[V0][0] = c.lbu((c.g[S3][0] as u32));
                                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 6);
                                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                                    c.lwc1(12, (c.g[V0][0] as u32));
                                    c.cvt_s_w(12, 12);
                                    c.div_s(12, 20, 12);
                                    c.g[RA][0] = sx(0x003be3e8);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    call_ptr(c, 0x003c0640, 0x003be3e8)?; // SNDENC_Quantize5
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
                                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.g[V0][0] & 0xf8;
                                    c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x35d));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A2][0] = c.g[A2][0] & 0x7;
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V0][0]);
                                    c.g[A3][0] = c.g[A3][0] & 0xf8;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[A0][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                                    c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35d), c.g[A3][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35c), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35f), c.g[V1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x35e), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.g[A0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x360));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x361), c.g[V0][0]);
                                    c.g[A0][0] = c.g[A0][0] & 0x7;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x360), c.g[A0][0]);
                                    c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                    c.lwc1(12, (c.g[V0][0] as u32));
                                    c.cvt_s_w(12, 12);
                                    c.div_s(12, 20, 12);
                                    c.g[RA][0] = sx(0x003be560);
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x80));
                                    call_ptr(c, 0x003c0640, 0x003be560)?; // SNDENC_Quantize5
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[A3][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
                                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
                                    c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0xb));
                                    c.g[V0][0] = c.g[V0][0] & 0xf8;
                                    c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x360));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A1][0] = c.g[A1][0] & 0x7;
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A2][0] = c.g[A2][0] & 0xf8;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0xb), c.g[A1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x360), c.g[A2][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 4);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x363), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x362), c.g[V1][0]);
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
                                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x365), c.g[V0][0]);
                                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.g[A0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x367));
                                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x364), c.g[V0][0]);
                                    c.g[A0][0] = c.g[A0][0] & 0x7;
                                    c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sb((c.g[S1][0] as u32).wrapping_add(0x367), c.g[A0][0]);
                                    let t = c.g[A3][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003be3b8);
                                        }
                                        continue 'l_3be3b8;
                                    } else {
                                        break 'b_3be6b4;
                                    }
                                }
                            }
                        }
                        c.cycles += 11;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA};

    const OC1092_P: u32 = DATA + 0x1000; // D_0050AC6C: subband state
    const OC1092_PEAKS: u32 = DATA + 0x3000; // its pA20 peak array
    const OC1092_Q: u32 = DATA + 0x4000; // D_0050AC68: frame state
    const OC1092_BLK: u32 = DATA + 0x5000; // per-channel block (scale factors at +4)
    const OC1092_OUT: u32 = DATA + 0x6000; // frame output buffer

    /// Encode a channel: random subband data and band factors, so the scale
    /// factor search takes both the byte-table and the exponent branch; real
    /// leaf callees (peaks, quantisers) and the game's real tables.
    #[test]
    fn sndenc_encode_channel_mode2_matches() {
        check(
            ssx3_ee::s13::f_003bde08__generated,
            sndenc_encode_channel_mode2,
            40,
            &|c, seed| {
                let mut r = Rng(0x1092 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                let ch = seed & 1;
                c.g[A0][0] = ch as u64;
                c.bus.write32(0x51_0000u32.wrapping_sub(0x5394), OC1092_P);
                c.bus.write32(0x51_0000u32.wrapping_sub(0x5398), OC1092_Q);
                c.bus.write32(OC1092_P + 0xa20, OC1092_PEAKS);
                c.bus.write32(OC1092_Q + 8, OC1092_OUT);
                c.bus.write32(OC1092_Q + 0x84 + 4 * ch, OC1092_BLK);
                // Subband data (24 x 16 floats) and per-band factors.
                let scale = [0.5f32, 4.0, 100.0, 1000.0][(seed % 4) as usize];
                for i in 0..24 * 16 {
                    let v = (r.next_u32() % 2000) as f32 / 1000.0 - 1.0;
                    c.bus.write32(OC1092_P + i * 4, (v * scale).to_bits());
                }
                for i in 0..24u32 {
                    let f = 0.1 + (r.next_u32() % 3000) as f32 / 1000.0;
                    c.bus.write32(OC1092_Q + 0xc + 4 * i, f.to_bits());
                }
            },
        );
    }
}
