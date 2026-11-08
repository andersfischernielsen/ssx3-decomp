//! Staging wave 213 (oc316): hand-written replacements for functions with no
//! C++ in the reference, ported with Ghidra's decompilation as the readable
//! aid. Each reproduces the generated body's registers, cycles, memory and
//! call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::guest::Addr;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_9830,
        spch_notify_play_callback,
        "SPCH_NotifyPlayCallback",
    ),
    (0x003c_6760, sndstrm_start_channel, "SNDSTRM_StartChannel"),
];
// ---------------------------------------------------------------------------
// SPCH_NotifyPlayCallback (0x3d9830)

/// The speech system's "sample started" callback (`void (*)(PlayInfo*)`,
/// 0 when nobody listens).
pub(crate) const SPCH_PLAY_CALLBACK: u32 = 0x0044_ff90;
/// The sentence slot table: 8-byte entries `{ i16 bank, i16 variation
/// (-1: none), u16 sample, u16 }`.
pub(crate) const SPCH_SENTENCE_SLOTS: u32 = 0x0045_070c;
/// gp offsets: the bank pointer table, the "mark played variations" flag and
/// the total length the percentage is taken of.
pub(crate) const SPCH_GP_BANKS: u32 = 0x1734;
pub(crate) const SPCH_GP_MARK_PLAYED: u32 = 0x1740;
pub(crate) const SPCH_GP_TOTAL_LEN: u32 = 0x1748;
/// Fields of a speech bank: flags (0x80 = has variation bits, low nibble =
/// header words), sample count, block multiplier, block size, id.
pub(crate) const SPCH_BANK_ID: u32 = 0x2;
pub(crate) const SPCH_BANK_FLAGS: u32 = 0x4;
pub(crate) const SPCH_BANK_COUNT: u32 = 0x5;
pub(crate) const SPCH_BANK_BLOCKS: u32 = 0x7;
pub(crate) const SPCH_BANK_BLOCK_SIZE: u32 = 0x8;

const SPCH_FIND_BANK_TYPE: Callee = Callee {
    addr: 0x003d_8330,
    f: ee::s14::f_003d8330,
};
const SPCH_FIND_BANK_USER_DATA: Callee = Callee {
    addr: 0x003d_81f0,
    f: ee::s14::f_003d81f0,
};
const SPCH_DECODE_SAMPLE_ENTRY: Callee = Callee {
    addr: 0x003d_b838,
    f: ee::s14::f_003db838,
};
const SPCH_HISTORY_PUSH: Callee = Callee {
    addr: 0x003d_6ee8,
    f: ee::s14::SPCH_HistoryPush,
};

/// `int SPCH_NotifyPlayCallback(Sentence* sentence (a0), BankKey* key (a1),
/// int slot (a2))` (0x3d9830): when a play callback is installed, look up the
/// key's bank type and user data, then for each of the sentence's
/// `flags[4] >> 2` slot entries (from `slot`): clear the sample's
/// "variation pending" bit (when the bank has variation bits and marking is
/// on), decode the sample entry, and if it decodes, report `{bank, start
/// offset (+ variation * block bytes), length, key, user data, bank id,
/// type}` to the callback and push the sample into the bank's history.
/// Returns the summed length in percent of the total (gp+0x1748), or 0.
pub fn spch_notify_play_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x450000;
        frame.save(c, FP, 0xc0);
        frame.save(c, S7, 0xb0);
        c.g[FP][0] = 0x450000;
        frame.save(c, S5, 0x90);
        c.g[S7][0] = 0; // summed length
        frame.save(c, S1, 0x50);
        c.g[S5][0] = c.g[A1][0]; // key
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A2][0]; // slot
        c.g[V1][0] = Addr(SPCH_PLAY_CALLBACK).lw(c);
        c.g[S0][0] = c.g[A0][0]; // sentence
        frame.save(c, RA, 0xd0);
        c.g[V0][0] = 0;
        frame.save(c, S6, 0xa0);
        frame.save(c, S4, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S2, 0x60);
        let user_data = frame.at(c, 0x34);
        Addr(user_data).sw(c, 0);
        if c.g[V1][0] != 0 {
            c.cycles += 10;
            c.g[A1][0] = sx(frame.at(c, 0x30));
            c.g[A0][0] = c.g[S5][0];
            SPCH_FIND_BANK_TYPE.call(c, 0x003d_9890)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx(frame.at(c, 0x34));
            SPCH_FIND_BANK_USER_DATA.call(c, 0x003d_989c)?;
            c.g[V1][0] = Addr(r32(c, S0)).off(SPCH_BANK_FLAGS).lbu(c);
            c.g[A0][0] = sx((c.g[V1][0] as u32) >> 2);
            c.g[V0][0] = 0x450000;
            if c.g[A0][0] != 0 {
                c.cycles += 5;
                c.g[V1][0] = sx(r32(c, S1) << 3);
                c.g[V0][0] = sx(SPCH_SENTENCE_SLOTS);
                c.g[S4][0] = c.g[A0][0]; // entries left
                c.g[S2][0] = sx(SPCH_SENTENCE_SLOTS.wrapping_add(r32(c, V1))); // entry
                c.g[S6][0] = u64::MAX; // -1
                loop {
                    spch_notify_mark_played(c);
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[SP][0];
                    SPCH_DECODE_SAMPLE_ENTRY.call(c, 0x003d_9958)?;
                    if c.g[V0][0] != 0 {
                        spch_notify_report(c)?;
                    }
                    c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
                    c.cycles += 2;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(8));
                    if c.g[S4][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_98c0);
                    }
                }
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_TOTAL_LEN)).lw(c);
            c.g[V0][0] = 0;
            if c.g[V1][0] != 0 {
                c.cycles += 6;
                c.g[V0][0] = 0x64;
                c.mult(0, V0, S7, V0); // total * 100
                c.div(0, V0, V1);
                c.g[V0][0] = c.lo[0];
            }
        }
        c.cycles += 12;
        frame.restore(c, RA, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Loop head of [`spch_notify_play_callback`]: load the entry's bank (s0)
/// and sample (s1); when the bank keeps variation bits (flags 0x80) and
/// marking is on, clear the sample's bit in the bank's bit array (after the
/// header: `(count * (words + 2) + 15) & 0x3ffc + words * 4 + 1`).
fn spch_notify_mark_played(c: &mut Cpu) {
    c.cycles += 9;
    let entry = Addr(r32(c, S2));
    c.g[S3][0] = entry.lh(c);
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_BANKS)).lw(c);
    c.g[V1][0] = sx(r32(c, S3) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[S0][0] = Addr(r32(c, V1)).lw(c);
    let bank = Addr(r32(c, S0));
    c.g[A2][0] = bank.off(SPCH_BANK_FLAGS).lbu(c);
    c.g[V0][0] = c.g[A2][0] & 0x80;
    c.g[S1][0] = entry.off(4).lhu(c);
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(SPCH_GP_MARK_PLAYED)).lw(c);
    c.g[A2][0] &= 0xf;
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 22;
    c.g[V1][0] = bank.off(SPCH_BANK_COUNT).lbu(c);
    c.g[A1][0] = sx(r32(c, A2).wrapping_add(2));
    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S1][0] as i64)) as u64;
    c.mult(0, V1, V1, A1); // count * (words + 2)
                           // sample / 8 (rounded toward zero, as `(s < 0 ? s + 7 : s) >> 3`)
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(7));
    if c.g[V0][0] != 0 {
        c.g[A0][0] = c.g[S1][0];
    }
    c.g[A2][0] = sx(r32(c, A2) << 2);
    c.g[A0][0] = sx((r32(c, A0) as i32 >> 3) as u32);
    c.g[A1][0] = 1;
    c.g[V0][0] = sx(r32(c, A0) << 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xf));
    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(r32(c, V0))); // bit index
    c.g[V1][0] &= 0x3ffc;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
    c.g[A1][0] = sx(r32(c, A1) << (r32(c, V0) & 31));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    c.g[A1][0] = !c.g[A1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    let bits = Addr(r32(c, V1));
    c.g[V0][0] = bits.lbu(c) & c.g[A1][0];
    bits.sb(c, c.g[V0][0]);
}

/// A decoded entry of [`spch_notify_play_callback`]: build the play info at
/// sp+0x10 and hand it to the callback, then push the sample into the bank's
/// history. Decode output at sp: `{ length, start, ... }`.
fn spch_notify_report(c: &mut Cpu) -> crate::call::Flow {
    let sp = sp(c);
    c.cycles += 8;
    let bank = Addr(r32(c, S0));
    c.g[V0][0] = bank.off(SPCH_BANK_BLOCKS).lbu(c);
    c.g[V1][0] = bank.off(SPCH_BANK_BLOCK_SIZE).lhu(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.g[A0][0] = Addr(r32(c, S2)).off(2).lh(c); // variation
    c.g[V0][0] = sx(r32(c, V0) << 8);
    c.g[A1][0] = Addr(sp.wrapping_add(4)).lw(c); // start
    let no_variation = c.g[A0][0] == c.g[S6][0];
    c.mult(0, V1, V1, V0); // block bytes
    if !no_variation {
        c.cycles += 2;
        c.mult(0, V0, A0, V1);
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    }
    c.cycles += 22;
    c.g[V0][0] = Addr(sp).lw(c); // length
    c.g[A0][0] = sx(sp.wrapping_add(0x10));
    Addr(sp.wrapping_add(0x14)).sw(c, c.g[A1][0]);
    Addr(sp.wrapping_add(0x10)).sw(c, c.g[S3][0]);
    c.g[S7][0] = sx(r32(c, S7).wrapping_add(r32(c, V0)));
    Addr(sp.wrapping_add(0x18)).sw(c, c.g[V0][0]);
    // The key (4 bytes, maybe unaligned) to sp+0x1c.
    let key = r32(c, S5);
    c.lwl(V1, key.wrapping_add(3));
    c.lwr(V1, key);
    c.swl(V1, sp.wrapping_add(0x1f));
    c.swr(V1, sp.wrapping_add(0x1c));
    c.g[V0][0] = Addr(r32(c, FP).wrapping_sub(0x70)).lw(c); // the callback
    c.g[V1][0] = Addr(sp.wrapping_add(0x30)).lw(c); // bank type
    c.g[A1][0] = Addr(sp.wrapping_add(0x34)).lw(c); // user data
    Addr(sp.wrapping_add(0x28)).sw(c, c.g[V1][0]);
    Addr(sp.wrapping_add(0x20)).sw(c, c.g[A1][0]);
    c.g[V1][0] = bank.off(SPCH_BANK_ID).lhu(c);
    let callback = r32(c, V0);
    Addr(sp.wrapping_add(0x24)).sw(c, c.g[V1][0]);
    call_ptr(c, callback, 0x003d_99d0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    SPCH_HISTORY_PUSH.call(c, 0x003d_99dc)
}

// ---------------------------------------------------------------------------
// SNDSTRM_StartChannel (0x3c6760)

/// The sound streams' channel records (`StreamChannel*[]`, by stream index).
pub(crate) const SNDSTRM_CHANNELS: u32 = 0x0051_5abc;
/// The sound system's state block (`D_0050A8E8`; +0x1f0: the voice/channel
/// array, 0x8c bytes per channel).
pub(crate) const SNDSTRM_SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const SNDSTRM_SND_CHANNELS: u32 = 0x1f0;
pub(crate) const SNDSTRM_SND_CHANNEL_SIZE: u32 = 0x8c;
/// Stream channel fields: result/handle word, per-voice stream offsets
/// (+0x10 words, +0x28 and +0x34 halves), voice count byte, max voice, the
/// copied stream header word and per-voice stream parameters (+0x64 words).
pub(crate) const SNDSTRM_CH_RESULT: u32 = 0x0;
pub(crate) const SNDSTRM_CH_MAX_VOICE: u32 = 0x42;
pub(crate) const SNDSTRM_CH_HEADER: u32 = 0x60;
pub(crate) const SNDSTRM_CH_VOICES: u32 = 0x62;
pub(crate) const SNDSTRM_CH_PARAMS: u32 = 0x64;
/// Stack frame slots: the voice-group request (sp+0: b3 = voice count,
/// +0x18 = capability flags), the group voice range (0xd0, 0xd4), the
/// allocation handle (0xd8), the group index (0xdc), the stream index (0xe0).
pub(crate) const SNDSTRM_START_FRAME: u32 = 0x190;

const SNDSTRM_GET_GROUP_VOICE_RANGE: Callee = Callee {
    addr: 0x003c_0c18,
    f: ee::s13::SND_GetGroupVoiceRange,
};
const SNDSTRM_ALLOC_VOICES: Callee = Callee {
    addr: 0x003b_a0b0,
    f: ee::s13::SND_AllocVoices,
};
const SNDSTRM_FIND_MAX_VOICE: Callee = Callee {
    addr: 0x003c_6580,
    f: ee::s13::SND_ChannelFindMaxVoice,
};
const SNDSTRM_CALC_PITCH: Callee = Callee {
    addr: 0x003b_a938,
    f: ee::s13::SND_ChannelCalcPitch,
};
const SNDSTRM_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::SND_ChannelCalcVolume,
};
const SNDSTRM_CALC_SEND: Callee = Callee {
    addr: 0x003b_a740,
    f: ee::s13::SND_ChannelCalcSend,
};
const SNDSTRM_CHANNEL_RELEASE: Callee = Callee {
    addr: 0x003b_a550,
    f: ee::s13::SND_ChannelRelease,
};
const SNDSTRM_NEXT_VOICE_GROUP: Callee = Callee {
    addr: 0x003c_6158,
    f: ee::s13::SND_NextVoiceGroup,
};
/// `SND_ChannelStartStreamVoices` (called through the function table).
const SNDSTRM_START_STREAM_VOICES: u32 = 0x003c_e1b0;

/// `reg + off` as an address.
#[inline]
fn sndstrm_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(r32(c, reg).wrapping_add(off))
}

/// `int SNDSTRM_StartChannel(int stream (a0), StreamHeader* hdr (a1),
/// StreamInfo* info (a2), StreamParams* params (a3))` (0x3c6760): reset the
/// stream's channel record (copy the header word, clear the per-voice
/// offsets, copy the per-voice parameters from `info + 0x18`), then walk the
/// voice groups that suit the request: allocate `hdr[2]` voices in the
/// group's range (-9 into the record when that fails, then try the next
/// group); on success set up the sound channel (pitch, volume, pan, send,
/// envelope fields from `info` and `params`), calculate pitch, volume and
/// send, and start the stream voices; if that fails release the voices and
/// store the error. Returns the record's result word.
///
/// The original is dispatcher-shaped: its loops' event checks can unwind,
/// and the function is re-entered at the loop heads 0x3c6818 (record reset),
/// 0x3c6858 (next voice group), 0x3c6988 (envelope copy) and 0x3c6a40
/// (voice release); all loop state is in registers.
pub fn sndstrm_start_channel(c: &mut Cpu) {
    let frame = Frame {
        size: SNDSTRM_START_FRAME,
    };
    let mut pc = c.pc;
    loop {
        match pc {
            0x003c_6760 => {
                c.cycles += 41;
                Frame::open(c, SNDSTRM_START_FRAME);
                c.g[V0][0] = 0x510000;
                Addr(frame.at(c, 0xe0)).sw(c, c.g[A0][0]);
                c.g[V0][0] = sx(0x0051_57b8);
                frame.save(c, S0, 0x180);
                frame.save(c, S5, 0x130);
                c.g[V1][0] = sx(r32(c, A0) << 2);
                frame.save(c, S6, 0x120);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                frame.save(c, S7, 0x110);
                c.g[S6][0] = c.g[A1][0]; // header
                frame.save(c, S1, 0x170);
                c.g[S7][0] = c.g[A2][0]; // info
                frame.save(c, S2, 0x160);
                c.g[S5][0] = c.g[A3][0]; // params
                frame.save(c, S3, 0x150);
                c.g[S0][0] = 0;
                frame.save(c, S4, 0x140);
                frame.save(c, FP, 0x100);
                frame.save_ra(c, 0xf0);
                c.g[A0][0] = sndstrm_at(c, S6, 2).lbu(c);
                c.g[A1][0] = sndstrm_at(c, S7, 8).lhu(c);
                c.g[S3][0] = sndstrm_at(c, V0, 0x304).lw(c); // the channel record
                c.g[FP][0] = sndstrm_at(c, S6, 2).lbu(c); // voice count
                                                          // The voice-group request at sp.
                Addr(frame.at(c, 0x3)).sb(c, c.g[A0][0]);
                Addr(frame.at(c, 0x18)).sh(c, c.g[A1][0]);
                Addr(frame.at(c, 0xdc)).sw(c, 0);
                // The header word (maybe unaligned) into the record.
                let (hdr, rec) = (r32(c, S6), r32(c, S3));
                c.lwl(V0, hdr.wrapping_add(3));
                c.lwr(V0, hdr);
                c.swl(V0, rec.wrapping_add(SNDSTRM_CH_HEADER + 3));
                c.swr(V0, rec.wrapping_add(SNDSTRM_CH_HEADER));
                let rec = Addr(rec);
                rec.off(0x4).sw(c, 0);
                rec.off(0x48).sw(c, 0);
                rec.off(0x4c).sw(c, 0);
                rec.off(0xc).sw(c, 0);
                rec.off(0x8).sw(c, 0);
                rec.off(0x46).sh(c, 0);
                c.g[V0][0] = rec.off(SNDSTRM_CH_VOICES).lbu(c);
                rec.off(0x44).sh(c, 0);
                c.g[A3][0] = sx(rec.0.wrapping_add(SNDSTRM_CH_PARAMS));
                if c.g[V0][0] == 0 {
                    pc = 0x003c_6a68;
                } else {
                    c.cycles += 5;
                    c.g[A2][0] = sx(r32(c, S7).wrapping_add(0x18));
                    c.g[A1][0] = sx(rec.0.wrapping_add(0x28));
                    c.g[A0][0] = sx(rec.0.wrapping_add(0x34));
                    c.g[V1][0] = sx(rec.0.wrapping_add(0x10));
                    pc = 0x003c_6818;
                }
            }
            // Record reset loop: clear the voice's offsets, copy its
            // parameter word.
            0x003c_6818 => {
                c.cycles += 14;
                sndstrm_at(c, V1, 0).sw(c, 0);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                sndstrm_at(c, A0, 0).sh(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                sndstrm_at(c, A1, 0).sh(c, 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(2));
                c.g[V0][0] = sndstrm_at(c, A2, 0).lw(c);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                sndstrm_at(c, A3, 0).sw(c, c.g[V0][0]);
                c.g[V0][0] = sndstrm_at(c, S3, SNDSTRM_CH_VOICES).lbu(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x003c_6818) {
                        return;
                    }
                    pc = 0x003c_6818;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = sx(frame.at(c, 0xdc));
                    pc = 0x003c_6a6c;
                }
            }
            // A voice group (s2): its voice range, then allocate.
            0x003c_6858 => {
                c.cycles += 12;
                c.g[A2][0] = sx(frame.at(c, 0xd4));
                c.g[A0][0] = c.g[S2][0];
                if SNDSTRM_GET_GROUP_VOICE_RANGE.call(c, 0x003c_6864).is_err() {
                    return;
                }
                c.g[A3][0] = Addr(frame.at(c, 0xd0)).lw(c);
                c.g[A0][0] = c.g[FP][0];
                c.g[T0][0] = Addr(frame.at(c, 0xd4)).lw(c);
                c.g[A1][0] = 0x65; // priority
                c.g[A2][0] = sx(frame.at(c, 0xd8));
                if SNDSTRM_ALLOC_VOICES.call(c, 0x003c_687c).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0]; // the sound channel
                c.g[V0][0] = Addr(frame.at(c, 0xd8)).lw(c); // its handle
                if (c.g[S1][0] as i64) >= 0 {
                    c.cycles += 59;
                    if sndstrm_setup_channel(c).is_err() {
                        return;
                    }
                    pc = if c.g[V0][0] == 0 {
                        0x003c_69a8
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = sx(r32(c, T1).wrapping_add(0x50));
                        0x003c_6988
                    };
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = (-9i64) as u64;
                    sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[V0][0]);
                    c.g[A0][0] = sx(frame.at(c, 0xdc));
                    pc = 0x003c_6a6c;
                }
            }
            // Envelope copy loop: info+0xc halves into channel+0x50.
            0x003c_6988 => {
                c.cycles += 8;
                c.g[V0][0] = sndstrm_at(c, V1, 0).lhu(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
                sndstrm_at(c, A0, 0).sh(c, c.g[V0][0]);
                c.g[V0][0] = sndstrm_at(c, S6, 2).lbu(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x003c_6988) {
                        return;
                    }
                    pc = 0x003c_6988;
                } else {
                    pc = 0x003c_69a8;
                }
            }
            0x003c_69a8 => {
                c.cycles += 30;
                if sndstrm_start_voices(c, frame).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                if (c.g[S2][0] as i64) >= 0 {
                    pc = 0x003c_6a80;
                } else {
                    // Failed: release the allocated voices.
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    if c.g[FP][0] == 0 {
                        pc = 0x003c_6a64;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = SNDSTRM_SND_CHANNEL_SIZE as u64;
                        c.mult(0, S1, S1, V0);
                        c.g[V0][0] = 0x510000;
                        c.g[S4][0] = sx(SNDSTRM_SND_STATE);
                        c.g[V0][0] = sndstrm_at(c, S4, SNDSTRM_SND_CHANNELS).lw(c);
                        pc = 0x003c_6a40;
                    }
                }
            }
            // Release loop: SND_ChannelRelease(channel->voices[i]).
            0x003c_6a40 => {
                c.cycles += 9;
                c.g[V1][0] = sx(r32(c, S0) << 1);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sndstrm_at(c, V0, 4).lh(c);
                if SNDSTRM_CHANNEL_RELEASE.call(c, 0x003c_6a58).is_err() {
                    return;
                }
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = sndstrm_at(c, S4, SNDSTRM_SND_CHANNELS).lw(c);
                    if back_edge_unwinds(c, 0x003c_6a40) {
                        return;
                    }
                    pc = 0x003c_6a40;
                } else {
                    pc = 0x003c_6a64;
                }
            }
            0x003c_6a64 => {
                c.cycles += 1;
                sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[S2][0]);
                pc = 0x003c_6a68;
            }
            0x003c_6a68 => {
                c.cycles += 1;
                c.g[A0][0] = sx(frame.at(c, 0xdc));
                pc = 0x003c_6a6c;
            }
            // Next voice group for the request at sp.
            0x003c_6a6c => {
                c.cycles += 5;
                c.g[A1][0] = c.g[SP][0];
                if SNDSTRM_NEXT_VOICE_GROUP.call(c, 0x003c_6a74).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                if c.g[S2][0] != 0 {
                    c.g[A1][0] = sx(frame.at(c, 0xd0));
                    if back_edge_unwinds(c, 0x003c_6858) {
                        return;
                    }
                    pc = 0x003c_6858;
                } else {
                    pc = 0x003c_6a80;
                }
            }
            0x003c_6a80 => {
                c.cycles += 13;
                c.g[V0][0] = sndstrm_at(c, S3, SNDSTRM_CH_RESULT).lw(c);
                frame.restore(c, S0, 0x180);
                frame.restore(c, S1, 0x170);
                frame.restore(c, S2, 0x160);
                frame.restore(c, S3, 0x150);
                frame.restore(c, S4, 0x140);
                frame.restore(c, S5, 0x130);
                frame.restore(c, S6, 0x120);
                frame.restore(c, S7, 0x110);
                frame.restore(c, FP, 0x100);
                frame.restore_ra(c, 0xf0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003c_6760, 0x003c_6ab4);
                return;
            }
        }
    }
}

/// Allocation succeeded (s1 = sound channel, v0 = handle): store the handle,
/// find the channel's max voice, and fill the channel's fields from `info`
/// (s7) and `params` (s5). Leaves t1 = the channel, v1 = info + 0xc and
/// v0 = the voice count (whether the envelope copy runs).
fn sndstrm_setup_channel(c: &mut Cpu) -> crate::call::Flow {
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = 0;
    sndstrm_at(c, S3, SNDSTRM_CH_RESULT).sw(c, c.g[V0][0]);
    c.g[A1][0] = sndstrm_at(c, S6, 2).lbu(c);
    SNDSTRM_FIND_MAX_VOICE.call(c, 0x003c_68ac)?;
    c.g[A3][0] = SNDSTRM_SND_CHANNEL_SIZE as u64;
    sndstrm_at(c, S3, SNDSTRM_CH_MAX_VOICE).sb(c, c.g[V0][0]);
    c.mult(0, V1, S1, A3); // channel * 0x8c
    c.g[A1][0] = u64::MAX;
    c.g[A3][0] = 0x510000;
    c.g[A0][0] = 0x7fff0000;
    c.g[A3][0] = sx(SNDSTRM_SND_STATE);
    c.g[A2][0] = 0x7f0000;
    c.g[V0][0] = sndstrm_at(c, A3, SNDSTRM_SND_CHANNELS).lw(c);
    c.g[A0][0] |= 0xffff;
    c.g[A3][0] = 1;
    c.g[T0][0] = 0x64;
    c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    let ch = Addr(r32(c, T1));
    let info = Addr(r32(c, S7));
    let params = Addr(r32(c, S5));
    ch.off(0x4c).sh(c, 0);
    c.g[V1][0] = params.off(0x8).lhu(c);
    ch.off(0x12).sh(c, c.g[A1][0]);
    ch.off(0x1c).sh(c, c.g[V1][0]);
    c.g[V0][0] = info.lhu(c);
    ch.off(0x84).sh(c, c.g[V0][0]); // base pitch (cents)
    c.g[V1][0] = params.off(0xc).lhu(c);
    ch.off(0x30).sw(c, 0);
    ch.off(0x88).sh(c, c.g[V1][0]);
    c.g[V0][0] = params.lb(c);
    ch.off(0x44).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    ch.off(0x40).sw(c, c.g[A2][0]);
    ch.off(0x38).sw(c, c.g[V0][0]); // volume << 16
    ch.off(0x3c).sw(c, 0);
    c.g[V0][0] = info.off(0x3).lbu(c);
    ch.off(0x2b).sb(c, c.g[V0][0]);
    c.g[V1][0] = params.off(0x1).lbu(c);
    ch.off(0x5c).sb(c, c.g[A3][0]);
    ch.off(0x64).sb(c, c.g[V1][0]);
    ch.off(0x5d).sb(c, 0);
    ch.off(0x5e).sb(c, 0);
    c.g[V0][0] = info.off(0x5).lbu(c);
    ch.off(0x60).sb(c, c.g[V0][0]);
    c.g[V1][0] = params.off(0x4).lbu(c);
    ch.off(0x5f).sb(c, c.g[V1][0]);
    c.g[V0][0] = params.off(0x5).lbu(c);
    ch.off(0x61).sb(c, c.g[V0][0]);
    c.g[V1][0] = info.off(0x6).lb(c);
    ch.off(0x26).sb(c, 0);
    c.mult(0, V1, V1, T0); // pitch range * 100
    ch.off(0x27).sb(c, 0);
    ch.off(0x82).sh(c, c.g[V1][0]);
    for off in [0x6c, 0x70, 0x74, 0x78, 0x7c] {
        ch.off(off).sw(c, 0);
    }
    c.g[V0][0] = sndstrm_at(c, S6, 2).lbu(c);
    c.g[V1][0] = sx(info.0.wrapping_add(0xc));
    Ok(())
}

/// Finish the channel (t1) setup, calculate pitch, volume and send, and
/// start its stream voices; v0 = the result.
fn sndstrm_start_voices(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let ch = Addr(r32(c, T1));
    let hdr = Addr(r32(c, S6));
    c.g[V0][0] = u64::MAX;
    ch.off(0x14).sw(c, 0);
    ch.off(0x18).sw(c, c.g[V0][0]);
    c.g[A0][0] = c.g[S1][0];
    c.g[V0][0] = hdr.lhu(c);
    ch.off(0x20).sh(c, c.g[V0][0]);
    c.g[V1][0] = hdr.off(0x3).lbu(c);
    ch.off(0x22).sb(c, c.g[V1][0]);
    c.g[V0][0] = hdr.off(0x2).lbu(c);
    ch.off(0x24).sh(c, c.g[S2][0]); // voice group
    ch.off(0x23).sb(c, c.g[V0][0]);
    ch.off(0x86).sh(c, 0);
    SNDSTRM_CALC_PITCH.call(c, 0x003c_69dc)?;
    c.g[A0][0] = c.g[S1][0];
    SNDSTRM_CALC_VOLUME.call(c, 0x003c_69e4)?;
    c.g[A0][0] = 0;
    c.g[A1][0] = c.g[S1][0];
    SNDSTRM_CALC_SEND.call(c, 0x003c_69f0)?;
    let params = Addr(r32(c, S5));
    c.g[A2][0] = params.off(0xe).lhu(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(frame.at(c, 0xe0)).lw(c); // stream
    c.g[T2][0] = c.g[S6][0];
    c.g[A3][0] = params.off(0x12).lhu(c);
    c.g[T3][0] = c.g[S7][0];
    c.g[T0][0] = params.off(0x14).lhu(c);
    c.g[T1][0] = params.off(0x16).lhu(c);
    call_ptr(c, SNDSTRM_START_STREAM_VOICES, 0x003c_6a14)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fill, DATA, GAME_GP};

    /// `SPCH_NotifyPlayCallback`: a sentence of 0-4 slot entries over four
    /// random banks (with and without variation bits, samples in and out of
    /// range so the decode fails or succeeds, variation -1 or not), the
    /// marking flag on/off, total 0 or not, no callback; the callback is a
    /// fake that may unwind.
    #[test]
    fn spch_notify_play_callback_matches() {
        covered(
            ee::s14::f_003d9830__generated,
            spch_notify_play_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d98_30, 4);
                c.pc = 0x003d_9830;
                let gp = GAME_GP;
                let callback = if seed % 9 == 0 { 0 } else { FAKE };
                c.bus.write32(SPCH_PLAY_CALLBACK, callback);
                // The key {u16, id lo, id hi} and the bank-type table at
                // 0x44ffb8 (8 x {bank*, user data}); some keys match entry 3.
                let key = DATA + 1 + (seed & 2); // maybe unaligned
                fill(c, &mut r, DATA, 4);
                for i in 0..8u32 {
                    let e = 0x0044_ffb8 + i * 8;
                    let p = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x400 + i * 0x10
                    };
                    c.bus.write32(e, p);
                    c.bus.write32(e + 4, r.next_u32());
                    if p != 0 {
                        fill(c, &mut r, p, 4);
                    }
                }
                if seed % 2 == 0 {
                    let p = DATA + 0x400 + 3 * 0x10;
                    c.bus.write32(0x0044_ffb8 + 3 * 8, p);
                    let (id_lo, id_hi) = (c.bus.read8(key + 2), c.bus.read8(key + 3));
                    c.bus.write8(p + 9, id_lo);
                    c.bus.write8(p + 8, id_hi);
                }
                // Banks: four pointers at gp+0x1734.
                let banks = DATA + 0x800;
                c.bus.write32(gp + SPCH_GP_BANKS, banks);
                for k in 0..4u32 {
                    let b = DATA + 0x10000 + k * 0x1000;
                    c.bus.write32(banks + 4 * k, b);
                    fill(c, &mut r, b, 0x200);
                    let flags = (r.next_u32() & 0x80) as u8 | (r.next_u32() % 4) as u8;
                    c.bus.write8(b + SPCH_BANK_FLAGS, flags);
                    c.bus
                        .write8(b + SPCH_BANK_COUNT, (1 + r.next_u32() % 8) as u8);
                    c.bus.write8(b + SPCH_BANK_BLOCKS, (r.next_u32() % 4) as u8);
                }
                c.bus
                    .write32(gp + SPCH_GP_MARK_PLAYED, if seed % 3 == 0 { 0 } else { 1 });
                let total = if seed % 5 == 0 {
                    0
                } else {
                    r.next_u32() % 5000 + 1
                };
                c.bus.write32(gp + SPCH_GP_TOTAL_LEN, total);
                // The sentence: flags byte at +4 (entry count << 2).
                let sentence = DATA + 0x100;
                let count = if seed % 11 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus
                    .write8(sentence + 4, (count << 2 | r.next_u32() & 3) as u8);
                let slot = r.next_u32() % 4;
                for i in 0..count {
                    let e = SPCH_SENTENCE_SLOTS + (slot + i) * 8;
                    c.bus.write16(e, (r.next_u32() % 4) as u16);
                    let variation = if r.next_u32() % 2 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write16(e + 2, variation as u16);
                    c.bus.write16(e + 4, (r.next_u32() % 10) as u16);
                }
                c.g[A0][0] = sx(sentence);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = slot as u64;
            },
        );
    }

    /// `SNDSTRM_StartChannel` fixture: a stream channel record (0-4 voice
    /// parameter words), a header with 1-3 voices (maybe unaligned), random
    /// info/params, and a voice-group table of 0-3 groups whose masks are 0
    /// (rejected), 0x8 (voices [0, n8)) or 0x100 (voices [n8, n8 + n100))
    /// over up to 8 sound channels, each free or busy with a priority the
    /// allocator will not steal (so SND_Stop is never reached). Groups with
    /// too few free voices fail (-9, next group); the first that fits sets
    /// the channel up. SND_ChannelStartStreamVoices always returns 0, so the
    /// release path (0x3c6a20..0x3c6a64) is not reachable with real callees.
    pub(crate) fn sndstrm_start_channel_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        let mut r = machine(c, seed, 0x3c67_60, 0);
        c.pc = 0x003c_6760;
        c.g[RA][0] = crate::difftest::RA as u64;
        let stream = r.next_u32() % 4;
        let rec = DATA + 0x100;
        fill(c, &mut r, rec, 0x40);
        c.bus.write32(SNDSTRM_CHANNELS + 4 * stream, rec);
        c.bus
            .write8(rec + SNDSTRM_CH_VOICES, (r.next_u32() % 5) as u8);
        let hdr = DATA + 0x400 + (seed & 3);
        fill(c, &mut r, DATA + 0x400, 4);
        let voices = 1 + r.next_u32() % 3;
        c.bus.write8(hdr + 2, voices as u8);
        let info = DATA + 0x500;
        fill(c, &mut r, info, 0x10);
        c.bus.write16(info + 8, 0); // request flags: any group
        c.bus.write16(info, (r.next_u32() % 4000) as u16); // pitch cents
        let params = DATA + 0x600;
        fill(c, &mut r, params, 0x10);
        // The voice groups.
        let state = SNDSTRM_SND_STATE;
        let groups = r.next_u32() % 4;
        c.bus.write8(state + 0x4c, groups as u8);
        for i in 0..4 {
            let m = match r.next_u32() % 4 {
                0 => 0,
                1 => 0x100,
                _ => 0x8,
            } | (r.next_u32() & 0x60);
            let m = if m & 0x108 == 0 { 0 } else { m };
            c.bus.write16(state + 0x4e + 2 * i, m as u16);
        }
        let n8 = r.next_u32() % 5;
        let n100 = r.next_u32() % 4;
        c.bus.write8(state + 0x30, n8 as u8);
        c.bus.write8(state + 0x2f, n100 as u8);
        c.bus.write8(state + 0x3c, r.next_u32() as u8);
        c.bus.write32(0x0044_c478, r.next_u32() & 0x7fff_ff00);
        // The sound channels.
        let chans = DATA + 0x10000;
        c.bus.write32(state + SNDSTRM_SND_CHANNELS, chans);
        for k in 0..8 {
            let ch = chans + k * SNDSTRM_SND_CHANNEL_SIZE;
            fill(c, &mut r, ch, SNDSTRM_SND_CHANNEL_SIZE / 4);
            let busy = r.next_u32() % 3 == 0;
            c.bus.write8(ch + 0x69, busy as u8);
            if busy {
                c.bus.write8(ch + 0x2a, 0x70);
            }
        }
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = sx(hdr);
        c.g[A2][0] = sx(info);
        c.g[A3][0] = sx(params);
        r
    }

    crate::difftest::events::driven!(sndstrm_start_channel_gen, ee::s13::f_003c6760__generated);
    crate::difftest::events::driven!(sndstrm_start_channel_hand, sndstrm_start_channel);

    #[test]
    fn sndstrm_start_channel_matches() {
        crate::difftest::check_lohi(
            ee::s13::f_003c6760__generated,
            sndstrm_start_channel,
            300,
            &|c, seed| {
                sndstrm_start_channel_setup(c, seed);
            },
        );
        crate::difftest::events::check_events(
            sndstrm_start_channel_gen,
            sndstrm_start_channel_hand,
            150,
            &|c, seed| {
                let mut r = sndstrm_start_channel_setup(c, seed);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, 0x003c_6760);
                // The first due event must land on this function's own back
                // edge: callees (SND_ChannelCalcPitch, ...) are dispatchers
                // whose unwind would resume at a pc that is not an entry of
                // ours. With 2-4 record words the record reset loop runs
                // before any call, and its first back edge (0x3c6818, ~60
                // cycles in) takes the event; delivering it reschedules the
                // next one far past this function.
                let record = DATA + 0x100;
                c.bus
                    .write8(record + SNDSTRM_CH_VOICES, (2 + r.next_u32() % 3) as u8);
                if mode != crate::difftest::events::Events::Off {
                    c.next_event = c.cycles + 1;
                }
                mode
            },
        );
    }
}
