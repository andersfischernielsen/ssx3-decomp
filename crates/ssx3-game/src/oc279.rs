//! Staging wave 210 (oc279): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::guest::Addr;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_6310,
        ssx_audio_system_play_wipeout_impact,
        "SSXAudioSystem_PlayWipeoutImpact",
    ),
];

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayWipeoutImpact (0x296310)

/// `Rider` fields read by the wipeout impact.
mod wipeout_rider {
    /// Sound source position handed to the pushed sound (`&rider->...`).
    pub const SOURCE: u32 = 0x110;
    /// Rider state; 2 and 3 pick the fixed impact sample.
    pub const STATE: u32 = 0x438;
    /// The rider's controller object (embedded; vptr at +0).
    pub const CONTROLLER: u32 = 0x6c0;
    /// Impact speed (`float`), the input of the volume curve.
    pub const SPEED: u32 = 0x770;
    /// Listener index, is-human, is-local.
    pub const LISTENER: u32 = 0x870;
    pub const HUMAN: u32 = 0x874;
    pub const LOCAL: u32 = 0x87c;
    /// Controller vtable slots: a flag (true: check the view range) and the
    /// position.
    pub const VT_FLAG: u32 = 0x48;
    pub const VT_POSITION: u32 = 0x28;
}

/// Bank monitor push-queue tables (`queue = monitor + 0x1d8`).
mod wipeout_queue {
    pub const RECORDS: u32 = 0x20;
    pub const RECORD_SIZE: u64 = 0x18;
    pub const VOLUME: u32 = 0x38;
    pub const SLOTS: u32 = 0x40;
    pub const PARAMS: u32 = 0x70;
    /// Inline per-entry arrays of the monitor itself.
    pub const MON_SOURCE: u32 = 0x1fc;
    pub const MON_GAIN: u32 = 0x214;
    pub const MON_LISTENER: u32 = 0x23c;
    pub const MON_REPLAY_FLAG: u32 = 0x258;
    pub const MON_REPLAY_LISTENER: u32 = 0x25c;
}

/// Game-mode byte: 2 enables the replay check.
const WIPEOUT_MODE_BYTE: u32 = 0x0053_5c11;

/// `lw base+off` on whole registers.
#[inline(always)]
fn pwi_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `ri = count << shift` (count from `addr`), as the original's
/// `lw; sll; addu table` triple; returns the element address.
#[inline(always)]
fn pwi_index(c: &mut Cpu, ri: usize, rt: usize, count_at: u64, table: u64, shift: u32) -> u32 {
    c.g[ri][0] = pwi_lw(c, count_at, 0);
    c.g[rt][0] = table;
    c.g[ri][0] = sx((c.g[ri][0] as u32) << shift);
    c.g[ri][0] = sx((c.g[ri][0] as u32).wrapping_add(c.g[rt][0] as u32));
    c.g[ri][0] as u32
}

/// `void SSXAudioSystem_PlayWipeoutImpact(SSXAudioSystem* sys, Rider* rider)`
/// (0x296310): the impact sound of a wipeout. Unless the rider's controller
/// flag is clear and the rider is local, the rider must be within 100 units
/// of a view (`FindViewInRange`). The volume comes from a five-key curve over
/// the impact speed (clamped to 0..127); the sample is fixed (bank 10,
/// sample 9) in rider states 2/3, else a random one of 0x30..0x33. A new
/// entry goes on the bank monitor's push queue (volume 100, pan 90, 50,
/// 127), with its gain (channel kind 5), the rider as source and listener,
/// and for a human local rider outside replays (mode byte 2) the replay
/// listener; then `Audio_PlayPushedSound(sys)`.
pub fn ssx_audio_system_play_wipeout_impact(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S4, 0x80);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S3][0] = c.g[A0][0]; // sys
        c.g[S5][0] = c.g[A1][0]; // rider
        c.g[S0][0] = sx(r32(c, S5).wrapping_add(wipeout_rider::CONTROLLER));
        if pwi_in_range(c)? {
            pwi_volume_and_sample(c)?;
            pwi_replay_check(c)?;
            pwi_push_entry(c)?;
            pwi_finish_entry(c)?;
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0029_06b8, 0x0029_6838)?; // Audio_PlayPushedSound
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The range gate: false when no view is within 100 units of the rider.
/// Leaves v0 = 0x480000 when it passes.
fn pwi_in_range(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use wipeout_rider::*;
    let controller = c.g[S0][0];
    // controller->flag()
    let vt = pwi_lw(c, c.g[S5][0], CONTROLLER);
    c.g[V1][0] = vt;
    let m = VSlot::read(c, vt, VT_FLAG);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(controller);
    call_ptr(c, m.func as u32, 0x0029_635c)?;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = pwi_lw(c, c.g[S5][0], CONTROLLER);
    } else {
        c.cycles += 3;
        let local = pwi_lw(c, c.g[S5][0], LOCAL) != 0;
        c.g[V0][0] = 0x48_0000;
        if local {
            return Ok(true);
        }
        c.cycles += 1;
        c.g[V1][0] = pwi_lw(c, c.g[S5][0], CONTROLLER);
    }
    // FindViewInRange(sys, controller->position(), 100.0)
    c.cycles += 11;
    let vt = c.g[V1][0];
    let m = VSlot::read(c, vt, VT_POSITION);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(c.g[S0][0]);
    call_ptr(c, m.func as u32, 0x0029_6384)?;
    c.g[AT][0] = 0x42c8_0000;
    c.f[12] = 0x42c8_0000; // 100.0
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x0029_6398)?;
    let out = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = 0x48_0000;
    Ok(!out)
}

/// Volume (s4) from the speed curve, sample bank / sample (s7 / s6).
fn pwi_volume_and_sample(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_rider::*;
    c.cycles += 36;
    let speed = pwi_lw(c, c.g[S5][0], SPEED);
    c.f[12] = speed as u32;
    // The curve at 0x482da0 (five keys, five values), by value on the
    // stack (five doublewords).
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2da0));
    let src = c.g[A3][0] as u32;
    let dst = sp(c);
    for (i, r) in [V1, A1, A2, V1, A1].into_iter().enumerate() {
        let off = 8 * i as u32;
        c.g[r][0] = c.ld(src.wrapping_add(off));
        c.sd(dst.wrapping_add(off), c.g[r][0]);
    }
    c.g[A0][0] = sx(dst);
    call_ptr(c, 0x0029_0b58, 0x0029_6400)?; // Audio_EvalCurve5
    c.g[S7][0] = 0;
    c.cvt_w_s(1, 0);
    c.mfc1(S4, 1);
    c.g[A0][0] = 0x7f;
    c.g[V1][0] = u64::MAX;
    let state = pwi_lw(c, c.g[S5][0], STATE);
    c.g[A2][0] = state;
    c.g[A1][0] = 2;
    c.g[V0][0] = ((c.g[S4][0] as i64) < 128) as u64;
    if c.g[V0][0] == 0 {
        c.g[S4][0] = 0x7f;
    }
    c.g[V1][0] = (-1i64 < c.g[S4][0] as i64) as u64;
    if c.g[V1][0] == 0 {
        c.g[S4][0] = 0;
    }
    let fixed = if state == 2 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        if state != 3 {
            c.g[V0][0] = pwi_lw(c, c.g[S3][0], 0x118);
            false
        } else {
            true
        }
    };
    if fixed {
        c.cycles += 3;
        c.g[S7][0] = 0xa;
        c.g[S6][0] = 0x9;
        return Ok(());
    }
    // Audio_Rand(monitor->queue head) -> 0x30 + rand15 * 3 / 0x7fff
    c.cycles += 8;
    c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
    c.g[A0][0] = pwi_lw(c, c.g[V1][0], 0x1d8);
    Callee {
        addr: 0x002a_df60,
        f: ee::s08::Audio_Rand,
    }
    .call(c, 0x0029_6454)?;
    c.g[V0][0] &= 0x7fff;
    c.g[A0][0] = 0x7fff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let (q, _) = crate::abi::divu(c, c.g[V1][0] as u32, 0x7fff);
    c.g[V1][0] = sx(q);
    c.g[S6][0] = sx(q.wrapping_add(0x30));
    Ok(())
}

/// fp = human local rider in mode 2 and no replay playing; s0 = monitor.
fn pwi_replay_check(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_rider::*;
    c.cycles += 4;
    c.g[V0][0] = pwi_lw(c, c.g[S5][0], HUMAN);
    c.g[FP][0] = 0;
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = pwi_lw(c, c.g[S5][0], LOCAL);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 10;
        call_ptr(c, 0x0014_dc80, 0x0029_64a0)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_64ac)?; // BELibrary_GetInterface
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = Addr(WIPEOUT_MODE_BYTE).lb(c);
        if c.g[V0][0] == 2 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0028_8ae0, 0x0029_64c8)?; // IsReplayPlaying
            c.g[FP][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0);
    Ok(())
}

/// A new push-queue entry on the monitor (s0; s1 = its queue): count += 1,
/// copy the record template, clear the entry, the default parameters, and
/// the sample (s7 / s6) on the system's push tables.
fn pwi_push_entry(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::sound::soundsys::boost_empty_store_entry as store;
    use wipeout_queue::*;
    c.cycles += 153;
    let monitor = c.g[S0][0];
    let count_at = sx((monitor as u32).wrapping_add(0x1f4));
    c.g[S2][0] = RECORD_SIZE;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[V0][0] = pwi_lw(c, monitor, 0x1f4);
    c.g[S1][0] = sx((monitor as u32).wrapping_add(0x1d8));
    let queue = c.g[S1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(count_at as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = pwi_lw(c, count_at, 0);
    c.g[V0][0] = pwi_lw(c, queue, RECORDS);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // Record template (monitor + 0x1dc, 24 bytes, unaligned).
    let m = monitor as u32;
    c.ldl(2, m.wrapping_add(0x1e3));
    c.ldr(2, m.wrapping_add(0x1dc));
    c.ldl(4, m.wrapping_add(0x1eb));
    c.ldr(4, m.wrapping_add(0x1e4));
    c.ldl(7, m.wrapping_add(0x1f3));
    c.ldr(7, m.wrapping_add(0x1ec));
    let record = c.g[V1][0] as u32;
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->source[count] = 0 (inline).
    c.g[V0][0] = pwi_lw(c, count_at, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(m.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(MON_SOURCE).sw(c, 0);
    store(c, (V1, V0), 0x34, 2, 0, 0);
    // volume[count] = the record's first byte (sign-extended).
    c.g[A3][0] = pwi_lw(c, count_at, 0);
    c.g[A0][0] = pwi_lw(c, count_at, 0);
    c.g[T0][0] = pwi_lw(c, queue, RECORDS);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = pwi_lw(c, queue, VOLUME);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    store(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count], 0, 8)
    c.g[A0][0] = pwi_lw(c, count_at, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((queue as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SLOTS));
    call_ptr(c, 0x0041_6210, 0x0029_65a0)?;

    // Parameters: volume 100, pan 90, 50, 127, 1.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    c.g[T0][0] = 0x1;
    store(c, (V1, V0), 0x60, 2, 0, 0);
    store(c, (V0, V1), 0x64, 2, 0, 0);
    store(c, (V1, V0), 0x68, 2, 0, 0);
    store(c, (V0, V1), 0x6c, 2, 0, 0);
    store(c, (V1, V0), PARAMS, 4, 0x4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    store(c, (V0, V1), PARAMS, 4, 0x0, a0);
    store(c, (V1, V0), PARAMS, 4, 0x8, a1);
    store(c, (V0, V1), PARAMS, 4, 0xc, a2);
    store(c, (V1, V0), 0x74, 2, 0, 0);
    store(c, (V0, V1), 0x80, 2, 0, 0);
    store(c, (V1, V0), 0x84, 2, 0, 0);
    store(c, (V0, V1), 0x78, 2, 0, a3);
    store(c, (V1, V0), 0x7c, 2, 0, t0);

    // sys->pushBank[count] = s7; sys->pushSample[count] = s6.
    let sys = c.g[S3][0];
    c.g[A0][0] = pwi_lw(c, sys, 0);
    c.g[V1][0] = pwi_lw(c, sys, 0x4);
    let at = pwi_index(c, V0, V1, sx((c.g[A0][0] as u32).wrapping_add(0x1f4)), c.g[V1][0], 2);
    Addr(at).sw(c, c.g[S7][0]);
    c.g[V1][0] = pwi_lw(c, sys, 0);
    c.g[A0][0] = pwi_lw(c, sys, 0x8);
    let at = pwi_index(c, V0, A0, sx((c.g[V1][0] as u32).wrapping_add(0x1f4)), c.g[A0][0], 2);
    Addr(at).sw(c, c.g[S6][0]);

    // root monitor: volume[count] = s4, then the record's volume byte =
    // clamp(volume[count], 0, 127).
    c.g[V1][0] = pwi_lw(c, sys, 0x118);
    c.g[A1][0] = pwi_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = pwi_lw(c, c.g[A1][0], 0x1f4);
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], VOLUME);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S4][0]);
    c.g[A0][0] = pwi_lw(c, c.g[A1][0], 0x1f4);
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], 0x1c);
    c.g[A3][0] = pwi_lw(c, c.g[A2][0], VOLUME);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = pwi_lw(c, c.g[A2][0], RECORDS);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    // The current volume word: queue->volume[queue->count].
    let vol_at = |c: &mut Cpu| -> u64 {
        let n = pwi_lw(c, c.g[A2][0], 0x1c);
        sx(((n as u32) << 2).wrapping_add(c.g[A3][0] as u32))
    };
    let mut clamp_top = true;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
        c.g[V0][0] = 0;
        clamp_top = (c.g[V1][0] as i64) >= 0;
    }
    if clamp_top {
        c.cycles += 7;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = vol_at(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
        }
    }
    Ok(())
}

/// Store the volume byte, then the entry's gain, source, listener and (fp)
/// replay listener.
fn pwi_finish_entry(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_queue::*;
    c.cycles += 30;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
    // monitor->gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 5, 0)
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0x5;
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0x118);
    call_ptr(c, 0x0028_7968, 0x0029_6790)?;
    c.g[A3][0] = pwi_lw(c, c.g[S0][0], 0);
    c.g[T0][0] = sx(r32(c, S5).wrapping_add(wipeout_rider::SOURCE));
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[V1][0] = pwi_lw(c, c.g[A3][0], 0x1f4);
    c.g[A2][0] = pwi_lw(c, c.g[A3][0], MON_GAIN);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // monitor->source[count] = &rider->source (inline array).
    c.g[A2][0] = pwi_lw(c, c.g[S3][0], 0x118);
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], 0);
    c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V1][0] as u32).off(MON_SOURCE).sw(c, c.g[T0][0]);
    // monitor->listener[count] = SSXAudioSystem_GetRiderListenerIndex(sys, rider)
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0x118);
    call_ptr(c, 0x0028_8a20, 0x0029_67d4)?;
    c.g[A0][0] = pwi_lw(c, c.g[S0][0], 0);
    c.g[V1][0] = pwi_lw(c, c.g[A0][0], 0x1f4);
    c.g[A1][0] = pwi_lw(c, c.g[A0][0], MON_LISTENER);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    if c.g[FP][0] != 0 {
        // Live play of the local human: flag and listener for the replay.
        c.cycles += 16;
        for (table, listener) in [(MON_REPLAY_FLAG, false), (MON_REPLAY_LISTENER, true)] {
            c.g[A0][0] = pwi_lw(c, c.g[S3][0], 0x118);
            c.g[A1][0] = if listener {
                pwi_lw(c, c.g[S5][0], wipeout_rider::LISTENER)
            } else {
                1
            };
            c.g[V1][0] = pwi_lw(c, c.g[A0][0], 0);
            c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0x1f4);
            c.g[A0][0] = pwi_lw(c, c.g[V1][0], table);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Addr(c.g[V0][0] as u32).sw(c, c.g[A1][0]);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::check_lohi;
    use crate::difftest::fakes::{vtable, LOG_RESULTS, LOG_UNWIND_AT};
    use crate::difftest::{put_vec, DATA};
    use crate::game::sound::soundsys::tests::audio;

    /// Views for `FindViewInRange` / `GetListenerCount`.
    const PWI_VIEWS: u32 = DATA + 0x28_0000;
    const PWI_VIEW: u32 = DATA + 0x28_1000;
    /// The controller's position (the fakes' forced result).
    const PWI_POS: u32 = DATA + 0x28_2000;

    /// `SSXAudioSystem_PlayWipeoutImpact` on the shared audio world: the
    /// controller flag from a fake (non-zero) or the real zero leaf, local or
    /// not; no view / a far view (early out) or a view at the rider; the
    /// real curve or one spanning negative to >127 volumes; rider states 2/3
    /// (fixed sample) or the real `Audio_Rand`; human/local and mode byte 2
    /// (real `IsReplayPlaying`, in and out of replays); the real push,
    /// gain, listener lookup and `Audio_PlayPushedSound` (no bank). Some
    /// seeds unwind at a fake call.
    #[test]
    fn ssx_audio_system_play_wipeout_impact_matches() {
        check_lohi(
            ee::s08::f_00296310__generated,
            ssx_audio_system_play_wipeout_impact,
            150,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_6310, 0x0029_6310);
                let rider = audio::RIDER;
                // Controller vtable: position (0x28) and flag (0x48) fakes.
                let vt = vtable(c, &mut r, &[0x28, 0x48]);
                let flag_is_zero = seed % 4 == 3;
                if flag_is_zero {
                    c.bus.write32(vt + 0x4c, 0x0014_06c0); // returns 0
                }
                c.bus.write32(rider + wipeout_rider::CONTROLLER, vt);
                c.bus.write32(LOG_RESULTS, PWI_POS);
                c.bus.write32(LOG_RESULTS + 4, PWI_POS);
                // Views: one, at the rider or far away; none sometimes.
                let count = if !flag_is_zero && seed % 9 == 4 { 0 } else { 1 };
                c.bus.write32(audio::MODULE + 0x84, PWI_VIEWS);
                c.bus.write32(PWI_VIEWS + 0x10, count);
                c.bus.write32(PWI_VIEWS + 4, PWI_VIEW);
                let pos = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 1.0f32.to_bits()];
                put_vec(c, PWI_POS, pos);
                let far = seed % 8 == 5;
                let view = if far {
                    [1.0e4f32.to_bits(), pos[1], pos[2], pos[3]]
                } else {
                    pos
                };
                put_vec(c, PWI_VIEW + 0x20, view);
                // Volume curve: the game's, or keys 0..40 with values -50..300.
                if seed % 3 != 0 {
                    for (i, v) in [0.0f32, 10.0, 20.0, 30.0, 40.0, -50.0, 0.0, 100.0, 200.0, 300.0]
                        .into_iter()
                        .enumerate()
                    {
                        c.bus.write32(0x0048_2da0 + 4 * i as u32, v.to_bits());
                    }
                }
                let speed = match seed % 6 {
                    0 => r.edge_f32(),
                    _ => r.f32(45.0),
                };
                c.bus.write32(rider + wipeout_rider::SPEED, speed);
                c.bus.write32(rider + wipeout_rider::STATE, [2, 3, 0, 1, 7][(seed / 2 % 5) as usize]);
                c.bus.write32(rider + wipeout_rider::LISTENER, r.next_u32() % 4);
                c.bus.write8(WIPEOUT_MODE_BYTE, if seed % 5 < 3 { 2 } else { 1 });
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 4);
                }
            },
        );
    }
}
