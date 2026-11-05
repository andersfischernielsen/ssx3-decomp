//! Staging wave 203 (oc226): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::addu;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// IrradianceRef_GetEntry (0x2ee010)

/// The global irradiance table (`D_005047F8`).
const IRRADIANCE_TABLE: u32 = 0x0050_47f8;
/// `IrradianceRef::index` (+0) and `IrradianceRef::key` (+4); a reference is
/// resolved once its key is `RESOLVED`.
const REF_INDEX: u32 = 0x0;
const REF_KEY: u32 = 0x4;
const RESOLVED: u64 = 0x0012_3400;
/// The default irradiance entry index (gp+0x12d4, `D_004A43C4`).
const GP_DEFAULT_IRRADIANCE: u32 = 0x12d4;

const IRRADIANCE_REF_RESOLVE: Callee = Callee {
    addr: 0x002e_dfd0,
    f: ee::s09::IrradianceRef_Resolve,
};
/// `Irradiance_GetEntry(table, index)` (cross-shard: through the table).
const IRRADIANCE_GET_ENTRY: u32 = 0x0038_abf8;

/// `Irradiance* IrradianceRef_GetEntry(IrradianceRef* ref)` (0x2ee010): the
/// irradiance entry a reference names. A resolved reference (key 0x123400)
/// uses its index; an unnamed one (empty name) the default entry; any other
/// is resolved by name first (`IrradianceRef_Resolve`, which writes the
/// index and the key) and then uses its index.
pub fn irradiance_ref_get_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = RESOLVED;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = this.off(REF_KEY).lw(c);
        if c.g[V1][0] == RESOLVED {
            c.g[A0][0] = 0x50_0000;
            c.cycles += 1;
            c.g[A1][0] = this.off(REF_INDEX).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = this.lb(c);
            let gp = c.g[GP][0] as u32;
            c.g[A1][0] = Addr(gp).off(GP_DEFAULT_IRRADIANCE).lw(c);
            if c.g[V0][0] == 0 {
                // Unnamed: the default entry.
                c.cycles += 2;
                c.g[A0][0] = 0x50_0000;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                IRRADIANCE_REF_RESOLVE.call(c, 0x002e_e050)?;
                c.g[A0][0] = 0x50_0000;
                c.cycles += 1;
                c.g[A1][0] = this.off(REF_INDEX).lw(c);
            }
        }
        c.cycles += 6;
        c.g[A0][0] = sx(IRRADIANCE_TABLE);
        call_ptr(c, IRRADIANCE_GET_ENTRY, 0x002e_e060)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_PopBlock (0x3c6de0)

/// The stream manager (`D_005157B8`): +0 event count, +4 events (8 bytes:
/// u16 type, u16 stream id, u32 value), +0x304 queue pointer per stream.
const SNDSTRM_MAN_HI: u64 = 0x51_0000;
const SNDSTRM_MAN: u32 = 0x0051_57b8;
const SNDSTRM_QUEUES: u32 = 0x304;
/// Stream queue fields.
const Q_WR: u32 = 0x8; // blocks written (consumed by every reader)
const Q_RD: u32 = 0xc; // blocks read by all readers
const Q_POS: u32 = 0x10; // u32 per reader: blocks read
const Q_RIDX: u32 = 0x28; // u16 per reader: ring read index
const Q_CNT: u32 = 0x34; // u16 per reader: blocks pending
const Q_SIZE: u32 = 0x40; // u16 ring size
const Q_WIDX: u32 = 0x44; // u16 ring write index
const Q_BYTES_A: u32 = 0x48;
const Q_BYTES_B: u32 = 0x4c;
const Q_NOTIFY: u32 = 0x54;
const Q_READERS: u32 = 0x62; // u8
/// Ring entries (0x20 bytes) start at +0x7c: +4 length (bit 31: flag),
/// +8 value per reader.
const Q_ENTRIES: u32 = 0x7c;

/// `u32 SNDSTRM_PopBlock(int id, int reader, int* len, int* flag)`
/// (0x3c6de0): if every reader has passed the write mark, advance it (and
/// post a "block done" event when the queue asks for one); then pop the
/// reader's next block: its length and flag go to `*len`/`*flag`, the reader
/// advances, and once all readers have passed the read mark the shared byte
/// counters move. Returns the block's value for this reader (0 becomes -1),
/// or 0 when the reader has nothing pending. Kept register-shaped: the two
/// loops check events at their back edges with every register live.
pub fn sndstrm_pop_block(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = SNDSTRM_MAN_HI;
    c.g[T5][0] = c.g[A0][0];
    c.g[T3][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[T5][0] as u32) << 2);
    c.g[V0][0] = sx(SNDSTRM_MAN);
    c.g[T2][0] = 1; // all readers past the write mark
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[T0][0] = Addr(c.g[V0][0] as u32).off(SNDSTRM_QUEUES).lw(c);
    let q = Addr(c.g[T0][0] as u32);
    c.g[V0][0] = q.off(Q_READERS).lbu(c);
    c.g[T4][0] = 1; // all readers past the read mark
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[T1][0] = c.g[V0][0];
        c.g[A0][0] = addu(c.g[T0][0], Q_POS as u64);
        loop {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[T1][0] = addu(c.g[T1][0], (-1i64) as u64);
            c.g[V0][0] = q.off(Q_WR).lw(c);
            c.g[A0][0] = addu(c.g[A0][0], 4);
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.g[T2][0] = 0;
            }
            if c.g[T1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_6e18);
            }
        }
    }
    c.cycles += 2;
    c.g[V1][0] = addu(c.g[T0][0], Q_CNT as u64);
    if c.g[T2][0] != 0 {
        // Advance the write mark.
        c.cycles += 7;
        c.g[V0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V1][0] = q.off(Q_NOTIFY).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 11) as u32);
        c.g[V0][0] = addu(c.g[V0][0], Q_ENTRIES as u64);
        c.g[T1][0] = addu(c.g[T0][0], c.g[V0][0]);
        if c.g[V1][0] != 0 {
            // Post event {type 1, id, value of reader 0}.
            c.cycles += 13;
            let man = Addr((c.g[T3][0] as u32).wrapping_add(0x57b8));
            c.g[V1][0] = man.lw(c);
            c.g[V0][0] = sx(man.0);
            c.g[V0][0] = addu(c.g[V0][0], 4);
            c.g[A0][0] = 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
            let ev = Addr(c.g[V1][0] as u32);
            ev.sh(c, c.g[A0][0]);
            ev.off(2).sh(c, c.g[T5][0]);
            c.g[A0][0] = Addr(c.g[T1][0] as u32).off(8).lw(c);
            ev.off(4).sw(c, c.g[A0][0]);
            c.g[V0][0] = man.lw(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            man.sw(c, c.g[V0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        q.off(Q_WIDX).sh(c, c.g[V0][0]);
        c.g[V1][0] = q.off(Q_WR).lw(c);
        c.g[V1][0] = addu(c.g[V1][0], 1);
        q.off(Q_WR).sw(c, c.g[V1][0]);
        c.g[A0][0] = q.off(Q_WIDX).lhu(c);
        c.g[V0][0] = q.off(Q_SIZE).lhu(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[A0][0] == 0 {
            q.off(Q_WIDX).sh(c, 0);
        }
        c.cycles += 1;
        c.g[V1][0] = addu(c.g[T0][0], Q_CNT as u64);
    }
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[T9][0] = c.g[V1][0];
    c.g[T5][0] = c.g[V0][0];
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lhu(c);
    let pending = c.g[A0][0] != 0;
    c.g[A0][0] = addu(c.g[T0][0], Q_RIDX as u64);
    if !pending {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    // Pop the reader's block.
    c.cycles += 28;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[T7][0] = c.g[A0][0];
    c.g[V0][0] = addu(c.g[T0][0], Q_POS as u64);
    c.g[A0][0] = addu(c.g[A0][0], c.g[T5][0]);
    c.g[T8][0] = c.g[A1][0];
    let ridx = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = ridx.lhu(c);
    c.g[A1][0] = addu(c.g[V0][0], c.g[A1][0]);
    c.g[T2][0] = c.g[V0][0];
    c.g[T1][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[T6][0] = 1;
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
    c.g[V1][0] = addu(c.g[V1][0], Q_ENTRIES as u64);
    c.g[T3][0] = addu(c.g[T0][0], c.g[V1][0]);
    let entry = Addr(c.g[T3][0] as u32);
    c.g[V0][0] = entry.off(4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
    Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]); // *len
    c.g[V1][0] = entry.off(4).lw(c);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 31) as u32);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]); // *flag
    c.g[V0][0] = ridx.lhu(c);
    c.g[V0][0] = addu(c.g[V0][0], 1);
    ridx.sh(c, c.g[V0][0]);
    let pos = Addr(c.g[A1][0] as u32);
    c.g[V1][0] = pos.lw(c);
    c.g[V1][0] = addu(c.g[V1][0], 1);
    pos.sw(c, c.g[V1][0]);
    // Have all readers passed the read mark? (stops at the first that has not)
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(c.g[T2][0] as u32).lw(c);
        c.g[T1][0] = addu(c.g[T1][0], 1);
        c.g[V0][0] = q.off(Q_RD).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] == 0 {
            c.g[T4][0] = 0;
        }
        let all_so_far = c.g[T4][0] == c.g[T6][0];
        c.g[T2][0] = addu(c.g[T2][0], 4);
        if !all_so_far {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = q.off(Q_READERS).lbu(c);
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_6f58);
        }
    }
    c.cycles += 2;
    c.g[A0][0] = addu(c.g[T7][0], c.g[T5][0]);
    if c.g[T4][0] != 0 {
        // Advance the read mark; move the block's bytes from A to B.
        c.cycles += 16;
        c.g[V0][0] = q.off(Q_RD).lw(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        q.off(Q_RD).sw(c, c.g[V0][0]);
        c.g[V1][0] = entry.off(4).lw(c);
        c.g[A0][0] = q.off(Q_BYTES_A).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        q.off(Q_BYTES_A).sw(c, c.g[A0][0]);
        c.g[V0][0] = entry.off(4).lw(c);
        c.g[V1][0] = q.off(Q_BYTES_B).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
        q.off(Q_BYTES_B).sw(c, c.g[V1][0]);
        c.g[A0][0] = addu(c.g[T7][0], c.g[T5][0]);
    }
    // Wrap the read index.
    c.cycles += 7;
    let ridx = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = ridx.lhu(c);
    c.g[V0][0] = q.off(Q_SIZE).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        ridx.sh(c, 0);
    }
    c.cycles += 10;
    c.g[A0][0] = addu(c.g[T9][0], c.g[T5][0]);
    c.g[A1][0] = addu(c.g[T3][0], c.g[T8][0]);
    let cnt = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = cnt.lhu(c);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = addu(c.g[V0][0], (-1i64) as u64);
    cnt.sh(c, c.g[V0][0]);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_SetMixPreset (0x2883b0)

/// SSXAudioSystem fields.
const AUD_CHAN_OUT: u32 = 0x62c8; // 11 channels of 0x10: +0 out, -4 gain
const AUD_CHAN_GAIN: u32 = 0x62c4;
const AUD_MIX_OUT: u32 = 0x636c; // 10 floats: channel 4's out times a weight
const AUD_CUR_GAIN: u32 = 0x63e4; // float per channel
const AUD_FADING: u32 = 0x6410;
const AUD_PRESET: u32 = 0x646c;
/// Preset table: 0x28 bytes each, gains per channel (-1: unchanged) at
/// +0x647c.. and the fade time at +0x64a0.
const AUD_PRESETS: u32 = 0x647c;
const PRESET_SIZE: u64 = 0x28;
const PRESET_FADE: u32 = 0x64a0;
/// The ten mix weights (gp+0x43c..0x460, `D_004A352C`..).
const GP_MIX_WEIGHTS: u32 = 0x43c;
/// The channels a preset sets (5 is not part of a preset).
const PRESET_CHANNELS: [u64; 9] = [1, 2, 3, 4, 6, 7, 8, 9, 10];

const AUDIO_GET_PRESET_GAIN: Callee = Callee {
    addr: 0x0028_8370,
    f: ee::s07::SSXAudioSystem_GetPresetGain,
};
const AUDIO_SET_GAIN_TARGET: Callee = Callee {
    addr: 0x0028_87a8,
    f: ee::s07::SSXAudioSystem_SetGainTarget,
};

/// `void SSXAudioSystem_SetMixPreset(SSXAudioSystem* self, int id)`
/// (0x2883b0): switch to mix preset `id` (negative: 0). Unless it is already
/// current, look up the gain of channels 1-4 and 6-10 (`GetPresetGain`: the
/// preset's gain, or the current/target one where the preset says -1). With
/// no fade time the gains apply at once: each channel's output becomes gain
/// times level, and the ten mix outputs follow channel 4 times their
/// weights. With a fade time every channel gets a gain target and fade
/// rate (`SetGainTarget`), and the system is marked fading.
pub fn ssx_audio_system_set_mix_preset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S1, 0x90);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.g[A1][0] = 0;
        }
        c.g[V0][0] = c.g[A1][0];
        for (reg, off) in [
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x10);
        for i in 0..10 {
            frame.save_fpr(c, 29 - i, 0xf8 - 8 * i as u32);
        }
        let this = Addr(c.g[S1][0] as u32);
        c.g[V1][0] = this.off(AUD_PRESET).lw(c);
        c.g[S0][0] = PRESET_SIZE;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 73;
            this.off(AUD_PRESET).sw(c, c.g[V0][0]);
            c.mult(0, S0, V0, S0); // s0 = id * 0x28
                                   // Gains of the nine channels into f21..f29; the pointers to the
                                   // preset's entries stay in s2-s7, fp and two stack words.
            let ptr_regs = [S2, S3, S4, S5, S6, S7, FP];
            let rets = [
                0x0028_8440u32,
                0x0028_845c,
                0x0028_8478,
                0x0028_8494,
                0x0028_84b0,
                0x0028_84cc,
                0x0028_84e8,
                0x0028_8508,
                0x0028_8528,
            ];
            for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                if i > 0 {
                    c.mov_s(20 + i, 0);
                }
                c.g[V0][0] = addu(sx(this.0), (AUD_PRESETS + 4 * i as u32) as u64);
                c.g[A0][0] = c.g[S1][0];
                let entry = addu(c.g[V0][0], c.g[S0][0]);
                if i < 7 {
                    c.g[ptr_regs[i]][0] = entry;
                } else {
                    c.g[V0][0] = entry;
                    let a = frame.at(c, 4 * (i as u32 - 7));
                    c.sw(a, entry);
                }
                c.g[A1][0] = ch;
                c.g[A2][0] = Addr(entry as u32).lw(c);
                AUDIO_GET_PRESET_GAIN.call(c, rets[i])?;
            }
            c.mov_s(29, 0);
            c.g[S0][0] = addu(c.g[S1][0], c.g[S0][0]);
            let fade = Addr(c.g[S0][0] as u32).off(PRESET_FADE).read_fpu(c);
            abi::set_fpr(c, 20, fade);
            c.f[0] = 0;
            abi::set_fcc(c, fade.eq_s(FpuF32(0)));
            c.g[A0][0] = addu(c.g[S1][0], AUD_CHAN_OUT as u64);
            if fade.eq_s(FpuF32(0)) {
                // Apply at once.
                c.cycles += 12;
                for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                    let a = this.off(AUD_CUR_GAIN + 4 * ch as u32).0;
                    c.swc1(21 + i, a);
                }
                c.g[V1][0] = addu(c.g[S1][0], AUD_CUR_GAIN as u64);
                c.g[V0][0] = addu(c.g[S1][0], AUD_CHAN_GAIN as u64);
                c.g[A1][0] = 10;
                loop {
                    // out[k] = gain[k] * level[k], k = 0..10
                    c.cycles += 9;
                    let gain = Addr(c.g[V0][0] as u32).read_fpu(c);
                    c.g[A1][0] = addu(c.g[A1][0], (-1i64) as u64);
                    let level = Addr(c.g[V1][0] as u32).read_fpu(c);
                    c.g[V0][0] = addu(c.g[V0][0], 0x10);
                    c.g[V1][0] = addu(c.g[V1][0], 4);
                    let out = gain * level;
                    abi::set_fpr(c, 1, level);
                    abi::set_fpr(c, 0, out);
                    Addr(c.g[A0][0] as u32).write_fpu(c, out);
                    let more = (c.g[A1][0] as i64) >= 0;
                    c.g[A0][0] = addu(c.g[A0][0], 0x10);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_8578);
                    }
                }
                c.cycles += 32;
                let gp = Addr(c.g[GP][0] as u32);
                let m = this.off(AUD_CHAN_OUT + 4 * 0x10).read_fpu(c);
                // The weights' registers in the original: f8 f9 f10 f6 f7 f5
                // f4 f2 f1, and f3 (whose product lands in f0).
                let regs = [8usize, 9, 10, 6, 7, 5, 4, 2, 1, 3];
                for (k, &f) in regs.iter().enumerate() {
                    let w = gp.off(GP_MIX_WEIGHTS + 4 * k as u32).read_fpu(c);
                    let out = m * w;
                    this.off(AUD_MIX_OUT + 4 * k as u32).write_fpu(c, out);
                    if f == 3 {
                        abi::set_fpr(c, 3, w);
                        abi::set_fpr(c, 0, out);
                    } else {
                        abi::set_fpr(c, f, out);
                    }
                }
            } else {
                // Fade: a gain target per channel over `fade` seconds.
                c.cycles += 76;
                let rets = [
                    0x0028_863cu32,
                    0x0028_865c,
                    0x0028_867c,
                    0x0028_869c,
                    0x0028_86bc,
                    0x0028_86dc,
                    0x0028_86fc,
                    0x0028_8720,
                    0x0028_8744,
                ];
                for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                    let entry = if i < 7 {
                        c.g[ptr_regs[i]][0]
                    } else {
                        let a = frame.at(c, 4 * (i as u32 - 7));
                        c.g[V0][0] = c.lw(a);
                        c.g[V0][0]
                    };
                    let gain = Addr(entry as u32).lw(c);
                    c.mov_s(12, 21 + i);
                    c.mov_s(13, 20);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = ch;
                    // a2 = (gain != -1)
                    c.g[A2][0] = (0u64 < !gain) as u64;
                    AUDIO_SET_GAIN_TARGET.call(c, rets[i])?;
                }
                c.g[V0][0] = 1;
                this.off(AUD_FADING).sw(c, 1);
            }
        }
        c.cycles += 22;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        for (reg, off) in [
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x10);
        for i in 0..10 {
            frame.restore_fpr(c, 29 - i, 0xf8 - 8 * i as u32);
        }
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IntersectInfo_Evaluate (0x351538)

/// IntersectInfo: +0 current segment index per animated channel, +0x40 the
/// curve set, +0x44 16 output floats.
const II_CURVE_SET: u32 = 0x40;
const II_OUT: u32 = 0x44;
/// Curve set: +0x18 mask of animated channels, +0x20 curve pointers (one per
/// animated channel).
const CS_MASK: u32 = 0x18;
const CS_CURVES: u32 = 0x20;
/// Curve: +0 segment count, +4 segments (0x18 bytes: cubic a b c d, then
/// the time range [start, end)).
const CURVE_SEGS: u32 = 0x4;
const SEG_SIZE: u64 = 0x18;
const SEG_START: u32 = 0x10;
const SEG_END: u32 = 0x14;

const INTERSECT_INFO_FIND_KEY_SEGMENT: Callee = Callee {
    addr: 0x0035_1a80,
    f: ee::s11::IntersectInfo_FindKeySegment,
};

/// `void IntersectInfo_Evaluate(IntersectInfo* self, float t)` (0x351538):
/// evaluate the animated channels of 16 at time `t`. For each channel whose
/// mask bit is set, the current cubic segment is used when `t` is in its
/// range, else `IntersectInfo_FindKeySegment` finds (and stores) the right
/// one; the output is `((a t + b) t + c) t + d`.
pub fn intersect_info_evaluate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        c.g[S0][0] = c.g[S3][0]; // current segment index of the channel
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 1; // channel mask bit
        frame.save_fpr(c, 20, 0x80);
        c.g[S2][0] = addu(c.g[S0][0], II_OUT as u64);
        frame.save(c, S1, 0x60);
        let t = abi::farg(c, 0);
        abi::set_fpr(c, 20, t);
        frame.save_ra(c, 0);
        c.g[S6][0] = SEG_SIZE;
        c.g[S4][0] = 0xf;
        let this = Addr(c.g[S3][0] as u32);
        c.g[V0][0] = this.off(II_CURVE_SET).lw(c);
        c.g[S1][0] = Addr(c.g[V0][0] as u32).off(CS_CURVES).lw(c);
        loop {
            c.cycles += 8;
            c.g[V1][0] = this.off(II_CURVE_SET).lw(c);
            c.g[S4][0] = addu(c.g[S4][0], (-1i64) as u64);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[A2][0] = 0;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(CS_MASK).lw(c);
            c.g[V0][0] &= c.g[S5][0];
            let animated = c.g[V0][0] != 0;
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 1);
            if animated {
                c.cycles += 13;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = c.g[A1][0];
                c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CURVE_SEGS).lw(c);
                abi::set_fpr(c, 12, t);
                c.mult(0, V0, V0, S6);
                c.g[A1][0] = c.g[S0][0];
                c.g[S1][0] = addu(c.g[S1][0], 4);
                c.g[V0][0] = addu(c.g[V1][0], c.g[V0][0]);
                let seg = Addr(c.g[V0][0] as u32);
                let start = seg.off(SEG_START).read_fpu(c);
                abi::set_fpr(c, 0, start);
                let after_start = start.le(t);
                abi::set_fcc(c, after_start);
                c.g[S0][0] = addu(c.g[S0][0], 4);
                if after_start {
                    c.cycles += 5;
                    let end = seg.off(SEG_END).read_fpu(c);
                    abi::set_fpr(c, 0, end);
                    let before_end = t.lt(end);
                    abi::set_fcc(c, before_end);
                    if before_end {
                        c.g[A2][0] = 1;
                    }
                }
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.cycles += 3;
                    INTERSECT_INFO_FIND_KEY_SEGMENT.call(c, 0x0035_1600)?;
                }
                let seg = Addr(c.g[V0][0] as u32);
                let a = seg.read_fpu(c);
                c.cycles += 10;
                let b = seg.off(4).read_fpu(c);
                let cc = seg.off(8).read_fpu(c);
                let d = seg.off(0xc).read_fpu(c);
                let v = ((a * t + b) * t + cc) * t + d;
                abi::set_fpr(c, 1, b);
                abi::set_fpr(c, 2, cc);
                abi::set_fpr(c, 3, d);
                abi::set_fpr(c, 0, v);
                Addr(c.g[S2][0] as u32).write_fpu(c, v);
            }
            c.cycles += 2;
            let more = (c.g[S4][0] as i64) >= 0;
            c.g[S2][0] = addu(c.g[S2][0], 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_1588);
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_FindJumpMarker (0x10b980)

/// ComputerActor: +0x18 the rider; the jump marker's take-off and landing
/// points go to +0xe50 and +0xe60.
const CA_RIDER: u32 = 0x18;
const CA_JUMP_FROM: u32 = 0xe50;
const CA_JUMP_TO: u32 = 0xe60;
/// Rider: track position (+0x4c0 across, +0x4c4 along), +0xab8 the track.
const RIDER_TRACK_X: u32 = 0x4c0;
const RIDER_TRACK_DIST: u32 = 0x4c4;
const RIDER_TRACK: u32 = 0xab8;
/// Track vtable (+0x34): `QueryMarkers(buf, max, x, dist)` at slot +8.
const TRACK_VTABLE: u32 = 0x34;
/// Marker records (0x10): type, flags, start and end distance.
const MARKER_JUMP: u64 = 0x10;
/// Speed units per flag step (gp-0x7e08: 27.77778, 100 km/h in m/s / 1).
const GP_MARKER_SPEED_UNIT: u32 = 0x7e08;

/// `Path_GetPointAtDistance(Vec4* out, Path* path, int* clamped, float d)`
/// (cross-shard: through the table).
const PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;

/// `bool ComputerActorRider_FindJumpMarker(ComputerActor* self, float*
/// speed, int* below, int* f3, int* f2, int* f1, int* f0)` (0x10b980): ask
/// the track for up to 12 markers in the 300 units ahead of the rider; for
/// the first jump marker (type 0x10) report its speed (flags >> 4, times
/// 27.78) and flag bits, whether it starts at or behind the rider, and store
/// the track points of its start and end. False when there is none.
pub fn computer_actor_rider_find_jump_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x180);
        c.g[AT][0] = 0x4396_0000; // 300.0
        c.mtc1(AT, 0);
        frame.save(c, S1, 0x160);
        frame.save(c, S4, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x120);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x110);
        c.g[S5][0] = c.g[T1][0];
        frame.save(c, S7, 0x100);
        c.g[S6][0] = c.g[T0][0];
        frame.save(c, FP, 0xf0);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S0, 0x170);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S2, 0x150);
        c.g[A1][0] = c.g[SP][0]; // marker buffer at sp
        frame.save(c, S3, 0x140);
        frame.save_ra(c, 0xe0);
        let this = Addr(c.g[S1][0] as u32);
        c.g[V0][0] = this.off(CA_RIDER).lw(c);
        let below_ptr = frame.at(c, 0xd0);
        c.sw(below_ptr, c.g[A2][0]);
        c.g[A2][0] = 0xc;
        let rider = Addr(c.g[V0][0] as u32);
        c.g[S3][0] = rider.off(RIDER_TRACK).lw(c);
        let dist = rider.off(RIDER_TRACK_DIST).read_fpu(c);
        c.g[V1][0] = Addr(c.g[S3][0] as u32).off(TRACK_VTABLE).lw(c);
        let x = rider.off(RIDER_TRACK_X).read_fpu(c);
        abi::set_fpr(c, 12, x);
        abi::set_fpr(c, 13, dist + FpuF32(c.f[0]));
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(8).lh(c);
        c.g[V0][0] = vt.off(0xc).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = addu(c.g[S3][0], c.g[A0][0]);
        call_ptr(c, method, 0x0010_ba00)?;
        c.g[A1][0] = c.g[V0][0];
        let count = c.g[A1][0] as i64;
        c.g[A0][0] = 0;
        let mut found = false;
        if count > 0 {
            c.cycles += 5;
            let gp = c.g[GP][0] as u32;
            let unit = Addr(gp.wrapping_sub(GP_MARKER_SPEED_UNIT)).read_fpu(c);
            abi::set_fpr(c, 1, unit);
            c.g[A2][0] = MARKER_JUMP;
            c.g[S2][0] = addu(c.g[SP][0], 0xc0); // point result
            c.g[V1][0] = c.g[SP][0]; // marker
            c.g[S0][0] = addu(c.g[SP][0], 0xc); // marker's end distance
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let is_jump = c.g[V0][0] == c.g[A2][0];
                c.g[A0][0] = addu(c.g[A0][0], 1);
                if is_jump {
                    c.cycles += 24;
                    let flags = Addr(c.g[V1][0] as u32).off(4).lw(c);
                    c.g[V1][0] = flags;
                    let start = Addr((c.g[S0][0] as u32).wrapping_sub(4)).read_fpu(c);
                    abi::set_fpr(c, 12, start);
                    let fl = flags as u32 as i32;
                    c.g[V0][0] = sx((fl >> 4) as u32);
                    let speed = FpuF32::from_i32(fl >> 4) * unit;
                    abi::set_fpr(c, 0, speed);
                    c.g[A1][0] = sx((fl >> 3) as u32) & 1;
                    c.g[V0][0] = sx((fl >> 2) as u32) & 1;
                    c.g[A0][0] = sx((fl >> 1) as u32) & 1;
                    c.g[V1][0] &= 1;
                    Addr(c.g[FP][0] as u32).write_fpu(c, speed);
                    Addr(c.g[S7][0] as u32).sw(c, c.g[A1][0]);
                    Addr(c.g[S6][0] as u32).sw(c, c.g[V0][0]);
                    Addr(c.g[S5][0] as u32).sw(c, c.g[A0][0]);
                    Addr(c.g[S4][0] as u32).sw(c, c.g[V1][0]);
                    c.g[V0][0] = this.off(CA_RIDER).lw(c);
                    let now = Addr(c.g[V0][0] as u32).off(RIDER_TRACK_DIST).read_fpu(c);
                    abi::set_fpr(c, 0, now);
                    let behind = start.le(now);
                    abi::set_fcc(c, behind);
                    c.g[A2][0] = 1;
                    if !behind {
                        c.cycles += 1;
                        c.g[A2][0] = 0;
                    }
                    c.cycles += 17;
                    let below = frame.at(c, 0xd0);
                    c.g[V0][0] = c.lw(below);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                    c.g[A2][0] = 0;
                    call_ptr(c, PATH_GET_POINT_AT_DISTANCE, 0x0010_baa8)?;
                    let point = frame.at(c, 0xc0);
                    c.lq(V0, point);
                    c.g[A0][0] = c.g[S2][0];
                    let end = Addr(c.g[S0][0] as u32).read_fpu(c);
                    abi::set_fpr(c, 12, end);
                    c.g[A1][0] = c.g[S3][0];
                    c.sq(V0, this.off(CA_JUMP_FROM).0);
                    c.g[A2][0] = 0;
                    call_ptr(c, PATH_GET_POINT_AT_DISTANCE, 0x0010_bac4)?;
                    let point = frame.at(c, 0xc0);
                    c.lq(V1, point);
                    c.g[V0][0] = 1;
                    c.sq(V1, this.off(CA_JUMP_TO).0);
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = addu(c.g[V1][0], 0x10);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = addu(c.g[S0][0], 0x10);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_ba20);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x170);
        frame.restore(c, S1, 0x160);
        frame.restore(c, S2, 0x150);
        frame.restore(c, S3, 0x140);
        frame.restore(c, S4, 0x130);
        frame.restore(c, S5, 0x120);
        frame.restore(c, S6, 0x110);
        frame.restore(c, S7, 0x100);
        frame.restore(c, FP, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AI_UpdateRaceTimeLimit (0x12bb20)

/// AI manager fields.
const AI_STATE: u32 = 0x0; // 5: racing, 3: waiting for the session
const AI_TIMER: u32 = 0x8; // race ticks
const AI_COUNTDOWN: u32 = 0x18; // ticks left after the first finisher; -1 done
const AI_RIDERS: u32 = 0x40; // actor pointers
const AI_RIDER_COUNT: u32 = 0x7c;
/// Actor +0x18 is the rider; rider +0x470 the finish time (>= 0 when
/// finished), +0x480 the "forced finish" flag.
const ACTOR_RIDER: u32 = 0x18;
const RIDER_FINISH_TIME: u32 = 0x470;
const RIDER_FORCED_FINISH: u32 = 0x480;
/// World (gp-0x848): +0x78 the BE interface, +0x84 the session (+0x214
/// its state, +0x28 the replay).
const GP_WORLD: u32 = 0x848;
const GP_GAME_COMM: u32 = 0x204; // gp-0x204
const SESSION_STATE: u32 = 0x214;
const SESSION_REPLAY: u32 = 0x28;
/// Globals: race active flag and race mode byte.
const RACE_ACTIVE: u32 = 0x0053_4b30;
const RACE_MODE: u32 = 0x0053_5c12;
/// The last tick at which a first finisher starts a countdown, the hard
/// time limit, and the longest countdown (2 minutes).
const LAST_COUNTDOWN_START: i64 = 0xd32b;
const TIME_LIMIT: u64 = 0xd3a4;
const MAX_COUNTDOWN: u64 = 0x1c20;

const BE_GET_INTERFACE: u32 = 0x0014_dd58; // BELibrary_GetInterface (table)
const BE_GET_INSTANCE: u32 = 0x0014_dc80; // BELibrary_GetInstance (table)
const REPLAY_STOP_RECORDING: u32 = 0x0026_fa50; // (table)
const GAME_COMM_SEND_SYNC_CHECK: u32 = 0x0025_8f20; // (table)
const AI_GET_SYNC_CHECKSUM: Callee = Callee {
    addr: 0x0012_b420,
    f: ee::s00::AI_GetSyncChecksum,
};
const RIDER_ON_FINISH_RACE: Callee = Callee {
    addr: 0x0012_5108,
    f: ee::s00::Rider_OnFinishRace,
};
const AI_ALL_PLAYERS_FINISHED: Callee = Callee {
    addr: 0x0012_a250,
    f: ee::s00::AI_AllPlayersFinished,
};

/// Force every rider that has not finished (finish time < 0) to finish
/// (`Rider_OnFinishRace` with the forced flag set). s2 counts, s0 walks the
/// actors, s3 = 1, f20 = 0. Loop head `head`, call return `ret`.
fn ai_force_finish_all(c: &mut Cpu, head: u32, ret: u32) -> crate::call::Flow {
    let this = Addr(c.g[S1][0] as u32);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
        let t = Addr(c.g[V0][0] as u32).off(RIDER_FINISH_TIME).read_fpu(c);
        abi::set_fpr(c, 0, t);
        let finished = FpuF32(c.f[20]).le(t);
        abi::set_fcc(c, finished);
        c.g[V1][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 5;
            Addr(c.g[V0][0] as u32)
                .off(RIDER_FORCED_FINISH)
                .sw(c, c.g[S3][0]);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
            RIDER_ON_FINISH_RACE.call(c, ret)?;
        }
        c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
        c.cycles += 4;
        c.g[S2][0] = addu(c.g[S2][0], 1);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = addu(c.g[S0][0], 4);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// What AI_UpdateRaceTimeLimit's time-limit checks end with.
#[derive(Clone, Copy)]
enum Limit {
    /// The unfinished riders were forced to finish: stop the replay.
    Stop,
    /// Nothing more to do (one join cycle).
    Join,
    /// Straight to the replay check.
    Tail,
}

/// `void AI_UpdateRaceTimeLimit(AI* self)` (0x12bb20): the race time limit.
/// Every second (60 ticks) a sync checksum is taken. While racing (state 5,
/// or 3 with the session in state 3) in race modes 0 and 4: once someone
/// finishes before tick 0xd32b a countdown starts (up to 2 minutes, never
/// past the limit); when it runs out, or at the hard limit 0xd3a4 (if not
/// everyone has finished), every unfinished rider is forced to finish, the
/// replay stops recording and the countdown is marked done (-1). Then,
/// unless the replay is in states 1-9, the sync check is sent every second.
pub fn ai_update_race_time_limit(c: &mut Cpu) {
    /// How the function leaves: through the s0 and s1 reloads (one cycle
    /// each), the s1 reload only (s0 was reloaded on the way), or neither.
    enum Exit {
        Full,
        S1Only,
        Plain,
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let world_slot = Addr(gp.wrapping_sub(GP_WORLD));
        c.g[V0][0] = world_slot.lw(c);
        frame.save(c, S1, 0x40);
        c.g[A1][0] = 7;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x78).lw(c);
        call_ptr(c, BE_GET_INTERFACE, 0x0012_bb54)?;
        let this = Addr(c.g[S1][0] as u32);
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = Addr(RACE_ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3c;
        let exit = 'body: {
            if !active {
                break 'body Exit::Full;
            }
            c.cycles += 3;
            c.g[A0][0] = this.off(AI_TIMER).lw(c);
            c.cycles += 4;
            c.div(0, A0, V0);
            c.g[V1][0] = c.hi[0];
            c.g[S4][0] = 0; // sync checksum
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                AI_GET_SYNC_CHECKSUM.call(c, 0x0012_bb88)?;
                c.g[S4][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[A1][0] = this.off(AI_STATE).lw(c);
            c.g[V0][0] = 5;
            let racing = c.g[A1][0] == 5;
            c.g[V0][0] = 3;
            if !racing {
                c.cycles += 2;
                let waiting = c.g[A1][0] == 3;
                frame.restore(c, S0, 0x50);
                if !waiting {
                    break 'body Exit::S1Only;
                }
                c.cycles += 5;
                c.g[V0][0] = world_slot.lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_STATE).lw(c);
                if c.g[A0][0] != c.g[A1][0] {
                    frame.restore(c, S1, 0x40);
                    break 'body Exit::Plain;
                }
            }
            c.cycles += 9;
            call_ptr(c, BE_GET_INSTANCE, 0x0012_bbc0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_INTERFACE, 0x0012_bbcc)?;
            c.g[V1][0] = 0x53_0000;
            c.g[V1][0] = Addr(RACE_MODE).lb(c);
            let mut timed_mode = c.g[V1][0] == 0;
            c.g[V0][0] = 4;
            if !timed_mode {
                c.cycles += 2;
                timed_mode = c.g[V1][0] == 4;
                c.g[V0][0] = world_slot.lw(c);
            }
            let limit = if !timed_mode {
                Limit::Tail
            } else {
                c.cycles += 3;
                c.g[V0][0] = this.off(AI_COUNTDOWN).lw(c);
                if (c.g[V0][0] as i64) >= 0 {
                    ai_countdown(c, this)?
                } else {
                    ai_time_limit(c, this, gp)?
                }
            };
            match limit {
                Limit::Tail => {}
                Limit::Stop | Limit::Join => {
                    if let Limit::Stop = limit {
                        // Time is up: stop the replay, countdown done.
                        c.cycles += 5;
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_REPLAY).lw(c);
                        call_ptr(c, REPLAY_STOP_RECORDING, 0x0012_bda4)?;
                        c.g[V0][0] = u64::MAX;
                        this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = world_slot.lw(c);
                }
            }
            // Send the sync check every second unless the replay is busy
            // (states 1-9).
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_REPLAY).lw(c);
            c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let busy = c.g[A1][0] != 0;
            frame.restore(c, S0, 0x50);
            if busy {
                break 'body Exit::S1Only;
            }
            c.cycles += 4;
            c.g[A1][0] = this.off(AI_TIMER).lw(c);
            c.g[V0][0] = 0x3c;
            c.cycles += 4;
            c.div(0, A1, V0);
            c.g[V1][0] = c.hi[0];
            frame.restore(c, S0, 0x50);
            if c.g[V1][0] != 0 {
                break 'body Exit::S1Only;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(gp.wrapping_sub(GP_GAME_COMM)).lw(c);
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, GAME_COMM_SEND_SYNC_CHECK, 0x0012_bdfc)?;
            Exit::Full
        };
        match exit {
            Exit::Full => {
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
                c.cycles += 1;
                frame.restore(c, S1, 0x40);
            }
            Exit::S1Only => {
                c.cycles += 1;
                frame.restore(c, S1, 0x40);
            }
            Exit::Plain => {}
        }
        c.cycles += 7;
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The running countdown (v0 = countdown >= 0): count it down; at zero
/// force everyone to finish.
fn ai_countdown(c: &mut Cpu, this: Addr) -> Result<Limit, crate::call::Unwound> {
    c.cycles += 2;
    let idle = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = addu(c.g[V0][0], (-1i64) as u64);
    if idle {
        return Ok(Limit::Join);
    }
    c.cycles += 2;
    let left = (c.g[V0][0] as i64) > 0;
    this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
    if left {
        return Ok(Limit::Join);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[20] = 0;
        c.g[S3][0] = 1;
        c.g[S0][0] = addu(c.g[S1][0], AI_RIDERS as u64);
        ai_force_finish_all(c, 0x0012_bc18, 0x0012_bc50)?;
        c.cycles += 2;
    } else {
        c.cycles += 1;
    }
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_sub(GP_WORLD)).lw(c);
    Ok(Limit::Stop)
}

/// No countdown yet (v0 < 0): start one when someone has finished before
/// tick 0xd32b; at the hard limit force everyone to finish.
fn ai_time_limit(c: &mut Cpu, this: Addr, gp: u32) -> Result<Limit, crate::call::Unwound> {
    let world_slot = Addr(gp.wrapping_sub(GP_WORLD));
    c.g[V1][0] = this.off(AI_TIMER).lw(c);
    c.cycles += 4;
    c.g[V0][0] = (LAST_COUNTDOWN_START < c.g[V1][0] as i64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        AI_ALL_PLAYERS_FINISHED.call(c, 0x0012_bc84)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                ai_start_countdown(c, this, gp);
                c.cycles += 1;
            }
        }
        c.g[V1][0] = this.off(AI_TIMER).lw(c);
    }
    // The hard limit.
    c.cycles += 4;
    c.g[V0][0] = ((TIME_LIMIT as i64 - 1) < c.g[V1][0] as i64) as u64;
    let over = c.g[V0][0] != 0;
    c.g[V0][0] = world_slot.lw(c);
    if !over {
        return Ok(Limit::Tail);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    AI_ALL_PLAYERS_FINISHED.call(c, 0x0012_bd28)?;
    let all = c.g[V0][0] != 0;
    c.g[V0][0] = world_slot.lw(c);
    if all {
        return Ok(Limit::Tail);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[20] = 0;
        c.g[S3][0] = 1;
        c.g[S0][0] = addu(c.g[S1][0], AI_RIDERS as u64);
        ai_force_finish_all(c, 0x0012_bd48, 0x0012_bd80)?;
    }
    c.cycles += 1;
    c.g[V0][0] = world_slot.lw(c);
    Ok(Limit::Stop)
}

/// For each finished rider (while the game comm is not in a menu or
/// paused, +0x8c/+0x48 both 0) set the countdown to the time left until the
/// limit, at most 2 minutes. v0 = rider count.
fn ai_start_countdown(c: &mut Cpu, this: Addr, gp: u32) {
    c.cycles += 6;
    c.f[1] = 0;
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = Addr(gp.wrapping_sub(GP_GAME_COMM)).lw(c);
    c.g[T1][0] = TIME_LIMIT;
    c.g[T0][0] = MAX_COUNTDOWN;
    c.g[A1][0] = addu(c.g[S1][0], AI_RIDERS as u64);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
        let t = Addr(c.g[V1][0] as u32).off(RIDER_FINISH_TIME).read_fpu(c);
        abi::set_fpr(c, 0, t);
        let finished = FpuF32(c.f[1]).le(t);
        abi::set_fcc(c, finished);
        c.g[A0][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        c.g[A2][0] = addu(c.g[A2][0], (-1i64) as u64);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            let comm = Addr(c.g[A3][0] as u32);
            c.g[V0][0] = comm.off(0x8c).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = comm.off(0x48).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 5;
                    c.g[V0][0] = this.off(AI_TIMER).lw(c);
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V0][0] as i64) < 7201) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.g[T0][0];
                    }
                    this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
                }
            }
        }
        c.cycles += 2;
        let more = c.g[A2][0] != 0;
        c.g[A1][0] = addu(c.g[A1][0], 4);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_bcb0);
        }
    }
}

// ---------------------------------------------------------------------------
// __floatsidf (0x412550)

const PACK_D: Callee = Callee {
    addr: 0x0041_1b08,
    f: ee::s15::__pack_d,
};
/// libgcc `fp_number_type` on the stack: class, sign, exponent, fraction.
const FP_CLASS: u32 = 0x0;
const FP_SIGN: u32 = 0x4;
const FP_EXP: u32 = 0x8;
const FP_FRAC: u32 = 0x10;
const CLASS_ZERO: u64 = 2;
const CLASS_NUMBER: u64 = 3;
/// Fraction bits (FRACBITS + NGARDS = 60): the fraction is normalised so
/// its top set bit is bit 60.
const FRAC_EXP: u64 = 0x3c;

/// `double __floatsidf(int i)` (0x412550, libgcc soft-float): build the
/// unpacked number (zero, or sign and magnitude with exponent 60,
/// normalised) and pack it with `__pack_d`. INT_MIN returns -2^31 directly.
pub fn floatsidf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        let num = Addr(sp(c));
        c.g[V0][0] = CLASS_NUMBER;
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 31); // sign
        frame.save_ra(c, 0x20);
        num.off(FP_CLASS).sw(c, c.g[V0][0]);
        num.off(FP_SIGN).sw(c, c.g[V1][0]);
        if c.g[A0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = CLASS_ZERO;
            num.off(FP_CLASS).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = FRAC_EXP;
            num.off(FP_EXP).sw(c, c.g[V0][0]);
            if c.g[V1][0] == 0 {
                c.cycles += 1;
                num.off(FP_FRAC).sd(c, c.g[A0][0]);
            } else {
                c.cycles += 3;
                let int_min = c.g[A0][0] == 0xffff_ffff_8000_0000;
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
                if int_min {
                    // -2^31 as a double.
                    c.cycles += 4;
                    c.g[V0][0] = 0xc1e0 << 48;
                    frame.restore_ra(c, 0x20);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                num.off(FP_FRAC).sd(c, c.g[V0][0]);
            }
            // Normalise: shift the fraction up to bit 60.
            c.cycles += 6;
            c.g[A1][0] = num.off(FP_FRAC).ld(c);
            c.g[V0][0] = u64::MAX >> 4;
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            c.g[A0][0] = num.off(FP_EXP).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A2][0] = u64::MAX >> 4;
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = c.g[A1][0] << 1;
                    c.g[A0][0] = addu(c.g[A0][0], (-1i64) as u64);
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_25d0);
                    }
                }
                c.cycles += 2;
                num.off(FP_EXP).sw(c, c.g[A0][0]);
                num.off(FP_FRAC).sd(c, c.g[V1][0]);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[SP][0];
        PACK_D.call(c, 0x0041_25fc)?;
        frame.restore_ra(c, 0x20);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_SetSlew0Target (0x113e80)

/// Rider: +0x1e0 velocity, slew 0 (lean) value +0x1f0, rate +0x1f4,
/// target +0x1f8, +0x438 the main state.
const RIDER_VELOCITY: u32 = 0x1e0;
const RIDER_SLEW0: u32 = 0x1f0;
const RIDER_SLEW0_RATE: u32 = 0x1f4;
const RIDER_SLEW0_TARGET: u32 = 0x1f8;
const RIDER_MAIN_STATE: u32 = 0x438;
/// gp-relative constants: full-lean speed (694.54), rate per unit of
/// distance (12.0), minimum and maximum rate (0.1, 14.0), the rate factor
/// in states 2, 3 and 13 (0.756), seconds per tick (1/60).
const GP_SLEW0_FULL_SPEED: u32 = 0x7c40;
const GP_SLEW0_RATE_SCALE: u32 = 0x7c3c;
const GP_SLEW0_RATE_MIN: u32 = 0x7c38;
const GP_SLEW0_RATE_MAX: u32 = 0x7c34;
const GP_SLEW0_STATE_FACTOR: u32 = 0x7c30;
const GP_SLEW0_DT: u32 = 0x7c2c;

/// `void Rider_SetSlew0Target(Rider* rider, float target)` (0x113e80): set
/// the lean slew target, scaled down below full-lean speed, and its rate:
/// the distance to the current value times 12, clamped to [0.1, 14] (the
/// lower bound wins when above it; reduced in states 2, 3 and 13), per tick.
pub fn rider_set_slew0_target(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    let gp = c.g[GP][0] as u32;
    let lit = |o: u32| Addr(gp.wrapping_sub(o));
    c.cycles += 18;
    let full = lit(GP_SLEW0_FULL_SPEED).read_fpu(c);
    abi::set_fpr(c, 1, full);
    // |velocity| (VU0 macro mode, as the original).
    c.lqc2(3, rider.off(RIDER_VELOCITY).0);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    let speed = FpuF32(c.g[V0][0] as u32);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    let one = FpuF32(0x3f80_0000);
    abi::set_fpr(c, 2, one);
    let ratio = speed.div(full);
    abi::set_fpr(c, 0, ratio);
    let slow = ratio.lt(one);
    abi::set_fcc(c, slow);
    let mut target = abi::farg(c, 0);
    if slow {
        target = target * ratio;
        abi::set_fpr(c, 12, target);
    }
    c.cycles += 11;
    let cur = rider.off(RIDER_SLEW0).read_fpu(c);
    let scale = lit(GP_SLEW0_RATE_SCALE).read_fpu(c);
    let lo = lit(GP_SLEW0_RATE_MIN).read_fpu(c);
    let hi = lit(GP_SLEW0_RATE_MAX).read_fpu(c);
    abi::set_fpr(c, 3, hi);
    abi::set_fpr(c, 1, lo);
    let d = (target - cur).abs() * scale;
    abi::set_fpr(c, 0, d);
    let above = lo.le(d);
    abi::set_fcc(c, above);
    let mut rate = lo;
    if above {
        c.cycles += 1;
        rate = d.min(hi);
    }
    abi::set_fpr(c, 2, rate);
    c.cycles += 5;
    c.g[V1][0] = rider.off(RIDER_MAIN_STATE).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(2));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let mut slowed = c.g[V0][0] != 0;
    if !slowed {
        c.cycles += 3;
        c.g[V0][0] = 0xd;
        slowed = c.g[V1][0] == c.g[V0][0];
    }
    if slowed {
        c.cycles += 2;
        let k = lit(GP_SLEW0_STATE_FACTOR).read_fpu(c);
        abi::set_fpr(c, 0, k);
        rate = rate * k;
    }
    abi::set_fpr(c, 1, rate);
    c.cycles += 5;
    let dt = lit(GP_SLEW0_DT).read_fpu(c);
    rider.off(RIDER_SLEW0_TARGET).write_fpu(c, target);
    let per_tick = rate * dt;
    abi::set_fpr(c, 0, per_tick);
    rider.off(RIDER_SLEW0_RATE).write_fpu(c, per_tick);
    abi::ret(c);
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_e010,
        irradiance_ref_get_entry,
        "IrradianceRef_GetEntry",
    ),
    (0x003c_6de0, sndstrm_pop_block, "SNDSTRM_PopBlock"),
    (
        0x0028_83b0,
        ssx_audio_system_set_mix_preset,
        "SSXAudioSystem_SetMixPreset",
    ),
    (
        0x0035_1538,
        intersect_info_evaluate,
        "IntersectInfo_Evaluate",
    ),
    (
        0x0010_b980,
        computer_actor_rider_find_jump_marker,
        "ComputerActorRider_FindJumpMarker",
    ),
    (
        0x0012_bb20,
        ai_update_race_time_limit,
        "AI_UpdateRaceTimeLimit",
    ),
    (0x0041_2550, floatsidf, "__floatsidf"),
    (0x0011_3e80, rider_set_slew0_target, "Rider_SetSlew0Target"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{
        check, check_lohi, check_paths, fill, put_words, scramble, Rng, DATA, GAME_GP,
    };

    /// SNDSTRM_PopBlock: readers behind/past the write and read marks, the
    /// notify event, ring wrap of both indices, nothing pending, value 0.
    #[test]
    fn sndstrm_pop_block_matches() {
        check(ee::s13::f_003c6de0__generated, sndstrm_pop_block, 300, &|c, seed| {
            let mut r = Rng(0x3c6d_e000 ^ seed as u64);
            scramble(c, &mut r);
            let id = r.next_u32() % 4;
            let readers = if seed % 11 == 0 {
                0
            } else {
                1 + r.next_u32() % 4
            };
            let reader = r.next_u32() % readers.max(1);
            c.g[A0][0] = id as u64;
            c.g[A1][0] = reader as u64;
            c.g[A2][0] = sx(DATA + 0x1000);
            c.g[A3][0] = sx(DATA + 0x1004);
            let q = DATA;
            fill(c, &mut r, q, 0x80);
            put_words(c, SNDSTRM_MAN, &[r.next_u32() % 6]);
            c.bus.write32(SNDSTRM_MAN + SNDSTRM_QUEUES + 4 * id, q);
            c.bus.write8(q + Q_READERS, readers as u8);
            let size = 2 + r.next_u32() % 6;
            c.bus.write16(q + Q_SIZE, size as u16);
            // Write index at the end of the ring now and then (wraps).
            let widx = if seed % 4 == 0 {
                size - 1
            } else {
                r.next_u32() % size
            };
            c.bus.write16(q + Q_WIDX, widx as u16);
            let wr = 10 + r.next_u32() % 4;
            let rd = 10 + r.next_u32() % 4;
            c.bus.write32(q + Q_WR, wr);
            c.bus.write32(q + Q_RD, rd);
            let ahead = seed % 3 == 0;
            for i in 0..4 {
                let p = if ahead {
                    14 + r.next_u32() % 2
                } else {
                    9 + r.next_u32() % 7
                };
                c.bus.write32(q + Q_POS + 4 * i, p);
                let ri = if seed % 5 == 0 {
                    size - 1
                } else {
                    r.next_u32() % size
                };
                c.bus.write16(q + Q_RIDX + 2 * i, ri as u16);
                let cnt = if seed % 7 == 0 { 0 } else { r.next_u32() % 3 };
                c.bus.write16(q + Q_CNT + 2 * i, cnt as u16);
            }
            c.bus
                .write32(q + Q_NOTIFY, if seed % 2 == 0 { 0 } else { 1 });
            for e in 0..8 {
                let entry = q + Q_ENTRIES + 0x20 * e;
                if r.next_u32() % 4 == 0 {
                    c.bus.write32(entry + 8 + 4 * reader, 0);
                }
            }
        });
    }

    /// SSXAudioSystem_SetMixPreset: same preset (no-op), negative id,
    /// immediate apply (fade 0) and fading, gains of -1 (keep current).
    #[test]
    fn ssx_audio_system_set_mix_preset_matches() {
        check_lohi(
            ee::s07::f_002883b0__generated,
            ssx_audio_system_set_mix_preset,
            240,
            &|c, seed| {
                let mut r = Rng(0x2883_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                c.g[A0][0] = sx(this);
                for k in 0..0x50 {
                    c.bus.write32(this + 0x62c0 + 4 * k, r.f32(4.0));
                }
                for k in 0..0x30 {
                    c.bus.write32(this + 0x6440 + 4 * k, r.f32(4.0));
                }
                for k in 0..10 {
                    c.bus.write32(GAME_GP + GP_MIX_WEIGHTS + 4 * k, r.f32(2.0));
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x4614), r.f32(2.0));
                c.bus.write32(this + AUD_FADING, r.next_u32() % 2);
                let id = (r.next_u32() % 4) as i32 - if seed % 6 == 0 { 3 } else { 0 };
                c.g[A1][0] = sx(id as u32);
                let cur = if seed % 8 == 0 {
                    id.max(0) as u32
                } else {
                    r.next_u32() % 5
                };
                c.bus.write32(this + AUD_PRESET, cur);
                for p in 0..4u32 {
                    let e = this + AUD_PRESETS + p * PRESET_SIZE as u32;
                    for k in 0..9 {
                        let g = if r.next_u32() % 3 == 0 {
                            u32::MAX
                        } else {
                            r.f32(2.0)
                        };
                        c.bus.write32(e + 4 * k, g);
                    }
                    let fade = if seed % 2 == 0 {
                        0
                    } else {
                        r.f32(3.0) & 0x7fff_ffff
                    };
                    c.bus.write32(e + 0x24, fade);
                }
            },
        );
    }

    /// IntersectInfo_Evaluate: random channel masks, `t` inside the current
    /// segment, in the next one, elsewhere and outside all of them (the real
    /// FindKeySegment searches).
    #[test]
    fn intersect_info_evaluate_matches() {
        check(
            ee::s11::f_00351538__generated,
            intersect_info_evaluate,
            240,
            &|c, seed| {
                let mut r = Rng(0x3515_3800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let set = DATA + 0x200;
                let curves = DATA + 0x300;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + II_CURVE_SET, set);
                let mask = match seed % 4 {
                    0 => 0xffff,
                    1 => 0,
                    _ => r.next_u32() & 0xffff,
                };
                c.bus.write32(set + CS_MASK, mask);
                c.bus.write32(set + CS_CURVES, curves);
                for i in 0..16 {
                    let curve = DATA + 0x400 + 0x10 * i;
                    let segs = DATA + 0x1000 + 0x200 * i;
                    let count = 1 + r.next_u32() % 5;
                    c.bus.write32(curves + 4 * i, curve);
                    c.bus.write32(curve, count);
                    c.bus.write32(curve + CURVE_SEGS, segs);
                    for j in 0..count {
                        let seg = segs + 0x18 * j;
                        for k in 0..4 {
                            c.bus.write32(seg + 4 * k, r.f32(3.0));
                        }
                        c.bus.write32(seg + SEG_START, (j as f32).to_bits());
                        c.bus.write32(seg + SEG_END, (j as f32 + 1.0).to_bits());
                    }
                    c.bus.write32(this + 4 * i, r.next_u32() % count);
                    c.bus.write32(this + II_OUT + 4 * i, r.next_u32());
                }
                let t = match seed % 5 {
                    0 => (r.next_u32() % 6) as f32,
                    1 => -0.5,
                    _ => (r.next_u32() % 600) as f32 / 100.0,
                };
                c.f[12] = t.to_bits();
            },
        );
    }

    /// ComputerActorRider_FindJumpMarker: the track's marker query is a fake
    /// (forced count; a real return-0 method for "none"), the buffer it
    /// "fills" is written beforehand; jump markers at random places or
    /// absent, before and behind the rider; the real Path_GetPointAtDistance
    /// on a short path.
    #[test]
    fn computer_actor_rider_find_jump_marker_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::SP as STACK;
        covered(
            ee::s00::f_0010b980__generated,
            computer_actor_rider_find_jump_marker,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10b9_80, 1);
                let this = DATA;
                let rider = DATA + 0x1000;
                let track = DATA + 0x2000;
                let vt = DATA + 0x3000;
                let nodes = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x5000);
                c.g[A2][0] = sx(DATA + 0x5004);
                c.g[A3][0] = sx(DATA + 0x5008);
                c.g[T0][0] = sx(DATA + 0x500c);
                c.g[T1][0] = sx(DATA + 0x5010);
                c.g[T2][0] = sx(DATA + 0x5014);
                c.bus.write32(this + CA_RIDER, rider);
                c.bus.write32(rider + RIDER_TRACK, track);
                c.bus.write32(rider + RIDER_TRACK_X, r.f32(50.0));
                c.bus.write32(
                    rider + RIDER_TRACK_DIST,
                    (100.0 + (r.next_u32() % 200) as f32).to_bits(),
                );
                c.bus.write32(track + TRACK_VTABLE, vt);
                let delta = (r.next_u32() % 8) * 4;
                c.bus.write16(vt + 8, delta as u16);
                // Path: start point, node count, nodes.
                for k in 0..3 {
                    c.bus.write32(track + 0xc + 4 * k, r.f32(100.0));
                }
                c.bus.write32(track + 8, r.next_u32() % 3);
                c.bus.write32(track + 0x18, nodes);
                for k in 0..12 {
                    c.bus.write32(nodes + 4 * k, r.f32(400.0));
                }
                c.bus
                    .write32(GAME_GP - GP_MARKER_SPEED_UNIT, 27.777_78f32.to_bits());
                if seed % 9 == 0 {
                    c.bus.write32(vt + 0xc, 0x0011_25b8); // Rider_ReturnZero: none
                } else {
                    c.bus.write32(vt + 0xc, FAKE);
                    c.bus.write32(LOG_RESULTS, 1 + r.next_u32() % 12);
                }
                // The marker records the query writes (buffer at the new sp).
                let buf = STACK - 0x180;
                let jump_at = if seed % 4 == 0 { 99 } else { r.next_u32() % 12 };
                for i in 0..12 {
                    let m = buf + 0x10 * i;
                    let ty = if i == jump_at {
                        0x10
                    } else {
                        r.next_u32() % 0x10
                    };
                    c.bus.write32(m, ty);
                    c.bus.write32(m + 4, r.next_u32() >> (r.next_u32() % 32));
                    c.bus
                        .write32(m + 8, (50.0 + (r.next_u32() % 300) as f32).to_bits());
                    c.bus
                        .write32(m + 0xc, (100.0 + (r.next_u32() % 400) as f32).to_bits());
                }
            },
        );
    }

    /// AI_UpdateRaceTimeLimit: inactive race, states 5/3/other (session
    /// state matching or not), race modes 0/4/other, countdown idle,
    /// running, expiring, not started (start it, hard limit, all finished),
    /// replay busy or not, every-second ticks. BELibrary_GetInterface's
    /// method is a fake; the other callees are real on short paths.
    #[test]
    fn ai_update_race_time_limit_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s00::f_0012bb20__generated,
            ai_update_race_time_limit,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12bb_20, 2);
                let gp = GAME_GP;
                let this = DATA;
                let world = DATA + 0x6000;
                let be_obj = DATA + 0x7000;
                let be_vt = DATA + 0x7100;
                let comm = DATA + 0x8000;
                let sess = DATA + 0x9000;
                let replay = DATA + 0x9400;
                c.g[A0][0] = sx(this);
                c.bus.write32(gp - GP_WORLD, world);
                c.bus.write32(world + 0x78, be_obj);
                c.bus.write32(world + 0x84, sess);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, be_obj);
                c.bus.write32(be_obj + 4, be_vt);
                c.bus.write16(be_vt + 8, 0);
                c.bus.write32(be_vt + 0xc, FAKE);
                c.bus.write32(gp - 0x1f04, DATA + 0x7800); // BELibrary instance
                c.bus.write32(gp - GP_GAME_COMM, comm);
                c.bus.write32(comm + 0x8c, (r.next_u32() % 4 == 0) as u32);
                c.bus.write32(comm + 0x48, (r.next_u32() % 4 == 0) as u32);
                c.bus.write32(sess + 0xc, sess + 0x500);
                c.bus.write32(sess + 0x500, 0); // session idle: no sync send
                c.bus.write32(
                    sess + SESSION_STATE,
                    if r.next_u32() % 2 == 0 { 3 } else { 4 },
                );
                c.bus.write32(sess + SESSION_REPLAY, replay);
                c.bus
                    .write32(replay, [0, 5, 14, 20][(r.next_u32() % 4) as usize]);
                c.bus.write32(RACE_ACTIVE, (seed % 13 != 0) as u32);
                c.bus
                    .write8(RACE_MODE, [0u8, 4, 0, 4, 2][(r.next_u32() % 5) as usize]);
                c.bus.write32(
                    this + AI_STATE,
                    [5, 5, 5, 3, 1][(r.next_u32() % 5) as usize],
                );
                let timer = match r.next_u32() % 5 {
                    0 => 60 * (1 + r.next_u32() % 800),
                    1 => 0xd32b - (r.next_u32() % 3),
                    2 => 0xd3a4 + (r.next_u32() % 3) * 60,
                    3 => 0xd32c + r.next_u32() % 0x70,
                    _ => r.next_u32() % 0xd3c0,
                };
                c.bus.write32(this + AI_TIMER, timer);
                let countdown = match seed % 5 {
                    0 | 1 => u32::MAX,
                    2 => 0,
                    3 => 1,
                    _ => 2 + r.next_u32() % 100,
                };
                c.bus.write32(this + AI_COUNTDOWN, countdown);
                c.bus.write32(this + 0x78, 0); // no riders for the sync checksum
                let count = r.next_u32() % 4;
                c.bus.write32(this + AI_RIDER_COUNT, count);
                for i in 0..4 {
                    let actor = DATA + 0x100 + 0x10 * i;
                    let rider = DATA + 0x1000 + 0x1000 * i;
                    c.bus.write32(this + AI_RIDERS + 4 * i, actor);
                    c.bus.write32(actor + ACTOR_RIDER, rider);
                    let t = if r.next_u32() % 2 == 0 {
                        (-1.0f32).to_bits()
                    } else {
                        r.f32(100.0) & 0x7fff_ffff
                    };
                    c.bus.write32(rider + RIDER_FINISH_TIME, t);
                    c.bus.write32(rider + RIDER_FORCED_FINISH, 0);
                }
            },
        );
    }

    /// __floatsidf: zero, positive, negative, INT_MIN, small and large
    /// magnitudes (long and short normalisation), with the real __pack_d.
    #[test]
    fn floatsidf_matches() {
        check(ee::s15::f_00412550__generated, floatsidf, 300, &|c, seed| {
            let mut r = Rng(0x4125_5000 ^ seed as u64);
            scramble(c, &mut r);
            let v = match seed % 6 {
                0 => 0,
                1 => i32::MIN,
                2 => (r.next_u32() % 16) as i32 - 8,
                3 => r.next_u32() as i32,
                4 => i32::MAX - (r.next_u32() % 4) as i32,
                _ => ((r.next_u32() >> (r.next_u32() % 31)) as i32)
                    .wrapping_mul(if r.next_u32() & 1 == 0 { 1 } else { -1 }),
            };
            c.g[A0][0] = sx(v as u32);
        });
    }

    /// Rider_SetSlew0Target: below and above full-lean speed, rate below,
    /// inside and above its bounds, states 2/3/13 and others.
    #[test]
    fn rider_set_slew0_target_matches() {
        check(
            ee::s00::f_00113e80__generated,
            rider_set_slew0_target,
            300,
            &|c, seed| {
                let mut r = Rng(0x113e_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let rider = DATA;
                c.g[A0][0] = sx(rider);
                let v = if seed % 3 == 0 { 900.0 } else { 300.0 };
                for k in 0..4 {
                    c.bus.write32(rider + RIDER_VELOCITY + 4 * k, r.f32(v));
                }
                c.f[12] = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
                c.bus.write32(rider + RIDER_SLEW0, r.f32(2.0));
                put_words(
                    c,
                    GAME_GP - GP_SLEW0_FULL_SPEED,
                    &[
                        694.538_8f32.to_bits(),
                        12.002_403f32.to_bits(),
                        0.1f32.to_bits(),
                        14.001_209f32.to_bits(),
                        0.756_134f32.to_bits(),
                        (1.0f32 / 60.0).to_bits(),
                    ],
                );
                let state = [2, 3, 13, 0, 1, 4, 12, 14][(r.next_u32() % 8) as usize];
                c.bus.write32(rider + RIDER_MAIN_STATE, state);
            },
        );
    }

    /// IrradianceRef_GetEntry: resolved, unnamed and named references (the
    /// real `IrradianceRef_Resolve` searches a small name table; the empty
    /// name or a miss decides the index it writes).
    #[test]
    fn irradiance_ref_get_entry_matches() {
        check(
            ee::s09::f_002ee010__generated,
            irradiance_ref_get_entry,
            240,
            &|c, seed| {
                let mut r = Rng(0x2ee0_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                // Irradiance table: {?, count, entries (0xa0 each), names (8 each)}.
                let names = DATA + 0x2000;
                let count = 1 + r.next_u32() % 3;
                put_words(c, IRRADIANCE_TABLE, &[0, count, DATA + 0x4000, names]);
                fill(c, &mut r, names, 8);
                if seed % 5 == 0 {
                    c.bus.write8(names, 0); // first name empty
                }
                c.bus
                    .write32(GAME_GP + GP_DEFAULT_IRRADIANCE, r.next_u32() % 4);
                c.bus.write32(this + REF_INDEX, r.next_u32() % 5);
                match seed % 3 {
                    0 => c.bus.write32(this + REF_KEY, RESOLVED as u32),
                    1 => {
                        c.bus.write32(this + REF_KEY, r.next_u32());
                        c.bus.write8(this, 0);
                    }
                    _ => {
                        c.bus.write32(this + REF_KEY, r.next_u32());
                        c.bus.write8(this, 1 + (r.next_u32() % 0x7f) as u8);
                        c.bus.write8(this + 1, 0);
                    }
                }
            },
        );
    }
}
