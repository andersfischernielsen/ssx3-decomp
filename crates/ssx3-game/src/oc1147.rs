//! Staging module oc1147 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1147`.

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
    (0x003c_3358, snd_mutex_delete, "SND_MutexDelete"),
    (0x003c_4318, snd_channel_get_position, "SND_ChannelGetPosition"),
    (0x003c_4ec8, snd_spu_mem_get_free, "SNDI_SpuMemGetFree"),
    (0x003c_5068, snd_channel_add_dsp, "SND_ChannelAddDsp"),
    (0x003c_5128, snd_channel_remove_dsp, "SND_ChannelRemoveDsp"),
    (
        0x003c_51e0,
        snd_get_output_latency_impl_return_zero,
        "SND_GetOutputLatencyImpl_ReturnZero",
    ),
    (0x003c_52f8, snd_set_config28, "SND_SetConfig28"),
    (0x003c_5330, snd_get_config28, "SND_GetConfig28"),
];

/// The SND block at `0x50ad00` (the generated code reaches it as
/// `0x510000 - 0x5300`): the mutex handle at +0x1b10, the SPU block counters
/// at +0x1a98 (count), +0x1a9a, +0x1a9c, +0x1a9e and the list pointer at +0x1aa0.
pub(crate) const SND_SPU_BLOCK_STATE: u32 = 0x0050_ad00;
/// The SND channel table at `0x50a8e8` (`0x510000 - 0x5718`): the channel
/// array base at +0x1f0; a channel record has its DSP list at +4 (halfwords),
/// flags at +0x24 (bit 2 = has DSP slots) and its count at +0x23.
pub(crate) const SND_CHANNEL_TABLE_STATE: u32 = 0x0050_a8e8;

/// `void SND_MutexDelete(void)` (0x3c3358; `SNDI_mutexfree` in the reference):
/// `DeleteSema` on the mutex handle the SND block keeps at +0x1b10.
pub fn snd_mutex_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = SND_SPU_BLOCK_STATE as u64;
        c.g[A0][0] = Addr(SND_SPU_BLOCK_STATE + 0x1b10).lw(c);
        call_ptr(c, 0x0042_3db0, 0x003c_3374)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_GetOutputLatencyImpl_ReturnZero(void)` (0x3c51e0): a leaf that
/// reports no output latency.
pub fn snd_get_output_latency_impl_return_zero(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `int SND_SetConfig28(SndOptions28* in)` (0x3c52f8): reads the current
/// options into a stack copy with `SND_GetConfig`, stores the halfword at
/// +0x28 of that copy from `in` (in `s0`), and hands the copy back to
/// `SND_SetConfig`. Returns 0.
pub fn snd_set_config28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x003b_4dd8,
            f: ee::s13::SND_GetConfig,
        }
        .call(c, 0x003c_5310)?;
        let s0 = c.g[S0][0];
        Addr(frame.at(c, 0x28)).sh(c, s0);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x003b_4fc0,
            f: ee::s13::SND_SetConfig,
        }
        .call(c, 0x003c_531c)?;
        frame.restore(c, S0, 0xf0);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_GetConfig28(SndOptions28* out)` (0x3c5330): reads the options
/// through `SND_GetConfig` into a stack copy and stores its halfword at +0x28
/// to `*out` (a word). Returns 0.
pub fn snd_get_config28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xe0);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x003b_4dd8,
            f: ee::s13::SND_GetConfig,
        }
        .call(c, 0x003c_5348)?;
        c.g[V1][0] = Addr(frame.at(c, 0x28)).lhu(c);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0xe0);
        let v = c.g[V1][0];
        let dst = Addr(c.g[S0][0] as u32);
        dst.sw(c, v);
        frame.restore(c, S0, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// `void SNDI_SpuMemGetFree(int* out)` (0x3c4ec8; `SNDPLATFORM_memlargestunused`
/// in the reference). With no SPU blocks listed it clamps the one free gap
/// (`SND_SpuMemClampBlock` on the block at +0x1a9a/+0x1a9c) and reports it;
/// otherwise it walks the list of four-byte entries at +0x1aa0, clamps each
/// gap, keeps the largest in `s2` and stores it through `out` (`s1`). The
/// stored value is the largest gap shifted left by 6 (the epilogue's
/// `*out <<= 6`).
pub fn snd_spu_mem_get_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S1, 0x70);
        c.g[A3][0] = sx(0x0051_0000u32.wrapping_sub(0x5300));
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(SND_SPU_BLOCK_STATE + 0x1a98).lhu(c);
        c.g[S6][0] = c.g[V0][0];
        let mut store = true;
        if c.g[V1][0] == 0 {
            // No listed blocks: clamp the one free gap.
            c.cycles += 12;
            c.g[A2][0] = Addr(SND_SPU_BLOCK_STATE + 0x1a9a).lhu(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = Addr(SND_SPU_BLOCK_STATE + 0x1a9c).lhu(c);
            c.g[A1][0] = c.g[SP][0] | 0x4;
            c.g[V1][0] = c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
            let v = c.g[V1][0];
            Addr(frame.at(c, 0)).sw(c, v);
            let v = c.g[V0][0];
            Addr(frame.at(c, 4)).sw(c, v);
            Callee {
                addr: 0x003c_4af8,
                f: ee::s13::SND_SpuMemClampBlock,
            }
            .call(c, 0x003c_4f2c)?;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[S2][0] = Addr(frame.at(c, 4)).lw(c);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(SND_SPU_BLOCK_STATE + 0x1a98).lhu(c);
            c.g[S0][0] = 0;
            let count_zero = c.g[V0][0] == 0;
            c.g[S3][0] = sx(frame.at(c, 4));
            if !count_zero {
                c.cycles += 2;
                c.g[S5][0] = c.g[A3][0];
                c.g[S4][0] = 0xffff_ffff_ffff_fffc;
                loop {
                    c.cycles += 4;
                    c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x1aa0)).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    let first = c.g[S0][0] == 0;
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if first {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = c.g[S3][0];
                        let v = c.g[V0][0];
                        Addr(frame.at(c, 0)).sw(c, v);
                        c.g[V1][0] = Addr(c.g[A2][0] as u32).lhu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x141));
                        let v = c.g[V1][0];
                        Addr(frame.at(c, 4)).sw(c, v);
                    } else {
                        c.cycles += 10;
                        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        c.g[A1][0] = c.g[S3][0];
                        c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(2)).lhu(c);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[V1][0] = Addr(c.g[A3][0] as u32).lhu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let v = c.g[V1][0];
                        Addr(frame.at(c, 0)).sw(c, v);
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        let v = c.g[V0][0];
                        Addr(frame.at(c, 4)).sw(c, v);
                    }
                    // Clamp the gap, keep the largest.
                    c.cycles += 6;
                    Callee {
                        addr: 0x003c_4af8,
                        f: ee::s13::SND_SpuMemClampBlock,
                    }
                    .call(c, 0x003c_4fb0)?;
                    c.g[V1][0] = Addr(frame.at(c, 4)).lw(c);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
                    if !t {
                        c.cycles += 2;
                        c.g[S2][0] = c.g[V1][0];
                        let v = c.g[V0][0];
                        let dst = Addr(c.g[S1][0] as u32);
                        dst.sw(c, v);
                    }
                    // Next entry.
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x1a98)).lhu(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_4f50);
                    }
                }
            }
            // The last gap, clamped again, then the store.
            c.cycles += 19;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5300));
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1aa0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1a9e)).lhu(c);
            c.g[A1][0] = c.g[S3][0];
            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(2)).lhu(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            let v = c.g[V0][0];
            Addr(frame.at(c, 0)).sw(c, v);
            let v = c.g[A2][0];
            Addr(frame.at(c, 4)).sw(c, v);
            Callee {
                addr: 0x003c_4af8,
                f: ee::s13::SND_SpuMemClampBlock,
            }
            .call(c, 0x003c_5018)?;
            c.g[V1][0] = Addr(frame.at(c, 4)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
            if t {
                store = false;
            } else {
                c.cycles += 1;
                c.g[S2][0] = c.g[V1][0];
            }
        }
        if store {
            c.cycles += 1;
            let v = c.g[V0][0];
            let dst = Addr(c.g[S1][0] as u32);
            dst.sw(c, v);
        }
        // Epilogue: `*out <<= 6`, the result is `s2 << 6`.
        c.cycles += 14;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 6);
        frame.restore(c, S0, 0x80);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
        frame.restore(c, S2, 0x60);
        let v = c.g[V1][0];
        let dst = Addr(c.g[S1][0] as u32);
        dst.sw(c, v);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int SND_ChannelGetPosition(int ch)` (0x3c4318; `SNDPLATFORM_getcurframe`
/// in the reference): the playback position of a channel's DSP slot. Flag
/// bit 3 gives the position from the slot table, else bit 8 gives the
/// register-only answer; with neither, `SND_EeVoiceGetPosition` is asked.
pub fn snd_channel_get_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[A2][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x8c;
        frame.save_ra(c, 0);
        c.mult(0, 4, 6, 4);
        c.g[V0][0] = 0x0051_0000;
        c.g[A3][0] = sx(0x0051_0000u32.wrapping_sub(0x5718));
        c.g[A1][0] = 0x0051_0000;
        c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x1f0)).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 5);
        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4468));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lhu(c);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.g[V1][0] & 0x8;
        // The two paths that return through the shared `ra` restore below.
        let shared_exit = if c.g[V0][0] != 0 {
            c.cycles += 9;
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 0x1c;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x658)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.mult(0, 2, 2, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
            true
        } else {
            c.g[V0][0] = c.g[V1][0] & 0x100;
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x30)).lbu(c);
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x4d8)).lw(c);
                true
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x2f)).lbu(c);
                c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x30)).lbu(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                call_ptr(c, 0x003c_8a40, 0x003c_439c)?;
                frame.restore_ra(c, 0);
                false
            }
        };
        if shared_exit {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void SND_ChannelAddDsp(int ch, int dsp)` (0x3c5068; `SNDPLATFORM_filteradd`
/// in the reference): when the channel has DSP slots (flag bit 2), calls
/// `SND_EeVoiceAddDsp(dsp, entry)` for each entry of its list (count at +0x23).
/// Returns 0, or -7 when the channel has no DSP slots.
pub fn snd_channel_add_dsp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x8c;
        frame.save(c, S2, 0x30);
        c.mult(0, 4, 4, 2);
        frame.save(c, S4, 0x10);
        c.g[V1][0] = 0x0051_0000;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = sx(0x0051_0000u32.wrapping_sub(0x5718));
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1f0)).lw(c);
        c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x24)).lhu(c);
        c.g[V0][0] &= 0x4;
        let flagged = c.g[V0][0] != 0;
        c.g[S4][0] = c.g[A1][0];
        if flagged {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x23)).lbu(c);
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !empty {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x2f)).lbu(c);
                loop {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x30)).lbu(c);
                    c.g[A2][0] = c.g[S4][0];
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lh(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    call_ptr(c, 0x003c_8e28, 0x003c_50ec)?;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x23)).lbu(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x2f)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_50c8);
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_fff9;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SND_ChannelRemoveDsp(int ch, int dsp)` (0x3c5128; the counterpart of
/// [`snd_channel_add_dsp`]): calls `SND_EeVoiceRemoveDsp(dsp, entry)` for each
/// entry of a channel with DSP slots. Returns 0.
pub fn snd_channel_remove_dsp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x8c;
        frame.save(c, S2, 0x30);
        c.mult(0, 4, 4, 2);
        frame.save(c, S4, 0x10);
        c.g[V1][0] = 0x0051_0000;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = sx(0x0051_0000u32.wrapping_sub(0x5718));
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1f0)).lw(c);
        c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x24)).lhu(c);
        c.g[V0][0] &= 0x4;
        let flagged = c.g[V0][0] != 0;
        c.g[S4][0] = c.g[A1][0];
        if flagged {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x23)).lbu(c);
            let empty = c.g[V0][0] == 0;
            c.g[S1][0] = 0;
            if !empty {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x2f)).lbu(c);
                loop {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x30)).lbu(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lh(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    call_ptr(c, 0x003c_8ec0, 0x003c_51a8)?;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x23)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x2f)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_5188);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The callee fakes answer only while this word of the machine holds
    /// the magic (otherwise the body they replaced runs).
    pub(crate) const OC1147_CTRL: u32 = DATA + 0x0022_1100;
    pub(crate) const OC1147_MAGIC: u32 = 0x1147_0272;

    pub(crate) static OC1147_PREV_DELETE_SEMA: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_GET_CONFIG: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_SET_CONFIG: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_CLAMP_BLOCK: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_VOICE_POSITION: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_VOICE_ADD_DSP: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1147_PREV_VOICE_REMOVE_DSP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1147_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1147_CTRL) == OC1147_MAGIC
    }

    /// `DeleteSema`: logs the call, returns to its caller.
    pub(crate) fn oc1147_fake_delete_sema(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(
                c,
                &OC1147_PREV_DELETE_SEMA,
                ee::s15::f_00423db0__generated,
            );
        }
        fakes::fake_call(c, 0x0042_3db0);
    }

    pub(crate) fn oc1147_fake_get_config(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(c, &OC1147_PREV_GET_CONFIG, ee::s13::f_003b4dd8__generated);
        }
        fakes::fake_call(c, 0x003b_4dd8);
    }

    pub(crate) fn oc1147_fake_set_config(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(c, &OC1147_PREV_SET_CONFIG, ee::s13::f_003b4fc0__generated);
        }
        fakes::fake_call(c, 0x003b_4fc0);
    }

    pub(crate) fn oc1147_fake_clamp_block(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(
                c,
                &OC1147_PREV_CLAMP_BLOCK,
                ee::s13::f_003c4af8__generated,
            );
        }
        fakes::fake_call(c, 0x003c_4af8);
    }

    pub(crate) fn oc1147_fake_voice_position(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(
                c,
                &OC1147_PREV_VOICE_POSITION,
                ee::s14::f_003c8a40__generated,
            );
        }
        fakes::fake_call(c, 0x003c_8a40);
    }

    pub(crate) fn oc1147_fake_voice_add_dsp(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(
                c,
                &OC1147_PREV_VOICE_ADD_DSP,
                ee::s14::f_003c8e28__generated,
            );
        }
        fakes::fake_call(c, 0x003c_8e28);
    }

    pub(crate) fn oc1147_fake_voice_remove_dsp(c: &mut Cpu) {
        if !oc1147_gated(c) {
            return fakes::fall_back(
                c,
                &OC1147_PREV_VOICE_REMOVE_DSP,
                ee::s14::f_003c8ec0__generated,
            );
        }
        fakes::fake_call(c, 0x003c_8ec0);
    }

    /// Hooks the seven callee fakes (once per process).
    pub(crate) fn oc1147_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DB0),
            oc1147_fake_delete_sema,
            &OC1147_PREV_DELETE_SEMA,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4DD8),
            oc1147_fake_get_config,
            &OC1147_PREV_GET_CONFIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4FC0),
            oc1147_fake_set_config,
            &OC1147_PREV_SET_CONFIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C4AF8),
            oc1147_fake_clamp_block,
            &OC1147_PREV_CLAMP_BLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003C8A40),
            oc1147_fake_voice_position,
            &OC1147_PREV_VOICE_POSITION,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003C8E28),
            oc1147_fake_voice_add_dsp,
            &OC1147_PREV_VOICE_ADD_DSP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003C8EC0),
            oc1147_fake_voice_remove_dsp,
            &OC1147_PREV_VOICE_REMOVE_DSP,
        );
    }

    /// A machine for a test whose callees are faked: scrambled registers,
    /// the fakes installed and switched on.
    pub(crate) fn oc1147_machine(c: &mut Cpu, seed: u32, salt: u64) {
        oc1147_install_fakes();
        fakes::machine(c, seed, salt, 2);
        c.bus.write32(OC1147_CTRL, OC1147_MAGIC);
    }

    #[test]
    fn snd_mutex_delete_matches() {
        check(
            ee::s13::f_003c3358__generated,
            snd_mutex_delete,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_3358);
                c.bus.write32(
                    SND_SPU_BLOCK_STATE + 0x1b10,
                    DATA + 0x2000 + 0x40 * (seed % 4),
                );
            },
        );
    }

    #[test]
    fn snd_get_output_latency_impl_return_zero_matches() {
        check(
            ee::s13::f_003c51e0__generated,
            snd_get_output_latency_impl_return_zero,
            20,
            &|c, seed| {
                let mut r = Rng(0x003c_51e0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_set_config28_matches() {
        check(
            ee::s13::f_003c52f8__generated,
            snd_set_config28,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_52f8);
                c.g[A0][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn snd_get_config28_matches() {
        check(
            ee::s13::f_003c5330__generated,
            snd_get_config28,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_5330);
                c.g[A0][0] = sx(DATA + 0x2100 + 0x10 * (seed % 3));
                c.bus.write16(DATA + 0x2100 + 0x10 * (seed % 3) + 0x28, seed as u16);
            },
        );
    }

    #[test]
    fn snd_spu_mem_get_free_matches() {
        check(
            ee::s13::f_003c4ec8__generated,
            snd_spu_mem_get_free,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_4ec8);
                // Count 0 takes the single-gap path, 1..3 walk the list.
                c.bus.write16(SND_SPU_BLOCK_STATE + 0x1a98, (seed % 4) as u16);
                c.bus.write16(SND_SPU_BLOCK_STATE + 0x1a9a, ((seed / 4) % 16) as u16);
                c.bus.write16(SND_SPU_BLOCK_STATE + 0x1a9c, (4 + (seed / 64) % 16) as u16);
                c.bus.write16(SND_SPU_BLOCK_STATE + 0x1a9e, ((seed / 8) % 8) as u16);
                c.bus.write32(SND_SPU_BLOCK_STATE + 0x1aa0, DATA + 0x5000);
                for i in 0..4u32 {
                    let e = DATA + 0x5000 + 4 * i;
                    c.bus.write16(e, (seed.wrapping_mul(37) ^ (i * 101)) as u16 & 0x3ff);
                    c.bus.write16(e + 2, (seed.wrapping_mul(11) + i * 53) as u16 & 0x3ff);
                }
                c.g[A0][0] = sx(DATA + 0x6000);
            },
        );
    }

    #[test]
    fn snd_channel_get_position_matches() {
        check(
            ee::s13::f_003c4318__generated,
            snd_channel_get_position,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_4318);
                let ch = seed % 2;
                c.g[A0][0] = sx(ch);
                c.bus.write32(SND_CHANNEL_TABLE_STATE + 0x1f0, DATA + 0x3000);
                // Flag word: bit 3 (table), bit 8 (register path), neither (call).
                let flags = [0x8u32, 0x100, 0][((seed / 2) % 3) as usize];
                c.bus.write16(DATA + 0x3000 + 0x8c * ch + 0x24, flags as u16);
                c.bus.write8(SND_CHANNEL_TABLE_STATE + 0x2f, (seed % 5) as u8);
                c.bus.write8(SND_CHANNEL_TABLE_STATE + 0x30, (seed % 7) as u8);
            },
        );
    }

    /// Channel record for the DSP tests: base pointer at the channel table,
    /// flags and count in the record, eight list halfwords.
    pub(crate) fn oc1147_dsp_setup(c: &mut Cpu, seed: u32) {
        let ch = seed % 2;
        c.g[A0][0] = sx(ch);
        c.bus.write32(SND_CHANNEL_TABLE_STATE + 0x1f0, DATA + 0x3000);
        let rec = DATA + 0x3000 + 0x8c * ch;
        let kind = (seed / 2) % 3;
        c.bus.write16(rec + 0x24, if kind == 0 { 0 } else { 4 });
        let count = if kind == 2 { 1 + (seed / 6) % 3 } else { 0 };
        c.bus.write8(rec + 0x23, count as u8);
        for k in 0..8u32 {
            c.bus.write16(rec + 4 + 2 * k, (seed.wrapping_mul(3) + k) as u16);
        }
        c.bus.write8(SND_CHANNEL_TABLE_STATE + 0x2f, (seed % 5) as u8);
        c.bus.write8(SND_CHANNEL_TABLE_STATE + 0x30, (seed % 7) as u8);
    }

    #[test]
    fn snd_channel_add_dsp_matches() {
        check(
            ee::s13::f_003c5068__generated,
            snd_channel_add_dsp,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_5068);
                oc1147_dsp_setup(c, seed);
            },
        );
    }

    #[test]
    fn snd_channel_remove_dsp_matches() {
        check(
            ee::s13::f_003c5128__generated,
            snd_channel_remove_dsp,
            200,
            &|c, seed| {
                oc1147_machine(c, seed, 0x003c_5128);
                oc1147_dsp_setup(c, seed);
            },
        );
    }
}
