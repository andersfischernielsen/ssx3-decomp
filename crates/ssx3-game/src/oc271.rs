//! Staging wave 209 (oc271): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::guest::Addr;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_6e98, object_save, "Object_Save"),
    (0x0011_1630, rider_base_enter_sub_state, "RiderBase_EnterSubState"),
    (0x003d_4950, path_track_get_queued_count, "PATH_TrackGetQueuedCount"),
    (
        0x0029_9368,
        ssx_audio_system_play_boost_empty_sound,
        "SSXAudioSystem_PlayBoostEmptySound",
    ),
    (
        0x0029_9e28,
        ssx_audio_system_on_race_position_change,
        "SSXAudioSystem_OnRacePositionChange",
    ),
    (0x003b_8218, snd_get_fx_params, "SND_GetFxParams"),
    (0x002a_bb38, bank_monitor_instance_reset, "BankMonitorInstance_Reset"),
    (0x003d_8780, spch_is_sample_allowed, "SPCH_IsSampleAllowed"),
];

// ---------------------------------------------------------------------------
// Object_Save

/// `void Object_Save(Object* self, Stream* stream)` (0x356e98): an object
/// saves exactly what its modifier-object base saves (`ModObj_Save`).
pub fn object_save(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0035_6b30,
            f: ee::s11::ModObj_Save,
        }),
        0x0035_6ea8,
    )
}

// ---------------------------------------------------------------------------
// RiderBase_EnterSubState

/// `RiderBase.subState`: the current sub-state id (0..13).
const RIDER_BASE_SUB_STATE: u32 = 0xde4;
/// `.rodata` jump table of the sub-state switch (14 case labels).
const ENTER_SUB_STATE_TABLE: u32 = 0x0045_6bd0;

/// The sub-state `Enter` methods: (case label, sub-object offset in the
/// rider base, callee, return address). `None` callee: a `jalr` through the
/// function table (cross-shard call).
const ENTER_SUB_STATE_ARMS: [(u32, u32, u32, Option<GuestFn>, u32); 12] = [
    (0x0011_1664, 0x1c0, 0x0013_1608, Some(ee::s00::RiderCruiseSub_Enter), 0x0011_166c),
    (0x0011_1674, 0x1d0, 0x0012_fc60, Some(ee::s00::RiderBoardPressSub_Enter), 0x0011_167c),
    (0x0011_1684, 0x1f0, 0x0012_e980, Some(ee::s00::RiderPrewindSub_Enter), 0x0011_168c),
    (0x0011_1694, 0x210, 0x0012_f620, Some(ee::s00::RiderNaturalAirSub_Enter), 0x0011_169c),
    (0x0011_16a4, 0x230, 0x0013_3128, None, 0x0011_16ac), // RiderSpinSub_Enter
    (0x0011_16b4, 0x290, 0x0012_be20, Some(ee::s00::RiderGateSub_Enter), 0x0011_16bc),
    (0x0011_16c4, 0x2b0, 0x0013_1d08, Some(ee::s00::RiderRailslideSub_Enter), 0x0011_16cc),
    (0x0011_16d4, 0x2c0, 0x0012_ca30, Some(ee::s00::RiderWipeoutSub_Enter), 0x0011_16dc),
    (0x0011_16e4, 0x350, 0x0012_f230, Some(ee::s00::RiderResetSub_Enter), 0x0011_16ec),
    (0x0011_16f4, 0x360, 0x0012_c5c8, Some(ee::s00::RiderFinishLineSub_Enter), 0x0011_16fc),
    (0x0011_1704, 0x370, 0x0013_28b0, None, 0x0011_170c), // RiderHandplantSub_Enter
    (0x0011_1714, 0x390, 0x0013_6268, None, 0x0011_171c), // RiderUberGrindSub_Enter
];

/// `void RiderBase_EnterSubState(RiderBase* r)` (0x111630): call the `Enter`
/// method of the rider's current sub-state (cruise, board press, prewind,
/// natural air, spin, gate, railslide, wipeout, reset, finish line,
/// handplant, uber grind; 3 and 13 have none) on its sub-object. Accepts
/// every `__entries` resume label.
pub fn rider_base_enter_sub_state(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0011_1630 => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(RIDER_BASE_SUB_STATE).lw(c);
                    let in_range = c.g[V1][0] < 0xe;
                    c.g[V0][0] = 0x45_0000;
                    pc = if in_range { 0x0011_164c } else { 0x0011_171c };
                }
                0x0011_164c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(ENTER_SUB_STATE_TABLE & 0xffff));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[A0][0] = target;
                    let target = target as u32;
                    let known = target == 0x0011_171c
                        || ENTER_SUB_STATE_ARMS.iter().any(|a| a.0 == target);
                    if !known {
                        c.jump_out(target, 0x0011_1630, 0x0011_1728);
                        return Ok(());
                    }
                    pc = target;
                }
                0x0011_171c => {
                    c.cycles += 1;
                    let sp = sp(c);
                    c.g[RA][0] = c.ld(sp);
                    pc = 0x0011_1720;
                }
                0x0011_1720 => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    let Some(&(_, off, addr, f, ret)) =
                        ENTER_SUB_STATE_ARMS.iter().find(|a| a.0 == pc)
                    else {
                        c.jump_out(pc, 0x0011_1630, 0x0011_1728);
                        return Ok(());
                    };
                    // The last arm (uber grind) falls through to 0x11171c.
                    let last = pc == 0x0011_1714;
                    c.cycles += if last { 2 } else { 4 };
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(off));
                    match f {
                        Some(f) => Callee { addr, f }.call(c, ret)?,
                        None => call_ptr(c, addr, ret)?,
                    }
                    if last {
                        pc = 0x0011_171c;
                    } else {
                        let sp = sp(c);
                        c.g[RA][0] = c.ld(sp);
                        pc = 0x0011_1720;
                    }
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// PATH_TrackGetQueuedCount

/// `PathTrack.flags`: bits 14..18 hold the number of queued entries.
const PATH_TRACK_FLAGS: u32 = 0x20;
/// `PathTrack.slot`: the PATH platform slot of the track.
const PATH_TRACK_SLOT: u32 = 0x54;

/// The queued-entry count field of a PATH track's flags word.
fn path_track_queued_slots(flags: u64) -> u64 {
    sx((flags as u32) >> 14) & 0x1f
}

/// `int PATH_TrackGetQueuedCount(PathTrack* t)` (0x3d4950): sum the
/// non-negative values of the track's queued entries
/// (`PATH_PlatGetQueuedEntryValue(t->slot, i)` for each of the
/// `(t->flags >> 14) & 31` entries; negative ones are errors and skipped).
pub fn path_track_get_queued_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0; // the sum
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0]; // the track
        let flags = Addr(c.g[S2][0] as u32).off(PATH_TRACK_FLAGS).lw(c);
        c.g[V0][0] = path_track_queued_slots(flags);
        c.g[S1][0] = 0; // the entry index
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S2][0] as u32).off(PATH_TRACK_SLOT).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                // PATH_PlatGetQueuedEntryValue
                call_ptr(c, 0x002a_eaa8, 0x003d_4990)?;
                let value = c.g[V0][0];
                let flags = Addr(c.g[S2][0] as u32).off(PATH_TRACK_FLAGS).lw(c);
                let negative = (value as i64) < 0;
                c.g[A0][0] = negative as u64;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(value as u32));
                c.g[V1][0] = (c.g[S1][0] < path_track_queued_slots(flags)) as u64;
                let more = c.g[V1][0] != 0;
                if !negative {
                    c.g[S0][0] = c.g[V0][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_4980);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayBoostEmptySound

/// `SSXAudioSystem.bankMonitor` (+0): the monitor whose push queue the
/// sound goes through.
const AUDIO_SYS_MONITOR: u32 = 0x0;
/// `SSXAudioSystem.pushBank` / `.pushSample` (+4 / +8): per-queue-entry
/// word arrays.
const AUDIO_SYS_PUSH_BANK: u32 = 0x4;
const AUDIO_SYS_PUSH_SAMPLE: u32 = 0x8;
/// `SSXAudioSystem.sndRoot` (+0x118): `*root` is the sound bank monitor.
const AUDIO_SYS_SND_ROOT: u32 = 0x118;
/// `Rider.isHuman` / `Rider.isLocal`.
const RIDER_AUDIO_HUMAN: u32 = 0x874;
const RIDER_AUDIO_LOCAL: u32 = 0x87c;
/// The bank monitor's push queue (`monitor + 0x1d8`) and its entry count
/// (`queue + 0x1c`, i.e. `monitor + 0x1f4`).
const MONITOR_PUSH_QUEUE: u32 = 0x1d8;
const MONITOR_PUSH_COUNT: u32 = 0x1f4;

/// One "store `value` into word table `queue + table` at the current queue
/// entry" step of the inlined push: `ri = count << shift; rt = table;
/// ri += rt; *(ri + extra) = value` (the original's register pair).
fn boost_empty_store_entry(
    c: &mut Cpu,
    (ri, rt): (usize, usize),
    table: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    let monitor = c.g[S0][0] as u32;
    let queue = c.g[S1][0] as u32;
    c.g[ri][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[rt][0] = Addr(queue).off(table).lw(c);
    c.g[ri][0] = sx((c.g[ri][0] as u32) << shift);
    c.g[ri][0] = sx((c.g[ri][0] as u32).wrapping_add(c.g[rt][0] as u32));
    Addr(c.g[ri][0] as u32).off(extra).sw(c, value);
}

/// `void SSXAudioSystem_PlayBoostEmptySound(SSXAudioSystem* sys, Rider*
/// rider)` (0x299368): outside replays, for a human local rider, cancel the
/// pending boost-empty timers and push sound 0x6c of bank 9 (volume 100,
/// pan 90, 50, 127) on the monitor's push queue, then play it
/// (`Audio_PlayPushedSound`).
pub fn ssx_audio_system_play_boost_empty_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0]; // rider
        // SSXAudioSystem_IsReplayPlaying
        call_ptr(c, 0x0028_8ae0, 0x0029_938c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let rider = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_HUMAN).lw(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_LOCAL).lw(c);
                c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            let play = c.g[A1][0] != 0;
            c.g[V0][0] = 0x4a_0000;
            if play {
                c.cycles += 155;
                boost_empty_push_and_play(c)?;
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The inlined push of `PlayBoostEmptySound` (v0 = 0x4a0000 on entry).
fn boost_empty_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    let sys = c.g[S3][0] as u32;
    // BankMonitor_KillTimersByCallback(*sys->sndRoot + 0x1d8, callback key
    // at 0x4a36a8).
    c.g[V1][0] = Addr(sys).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x36af));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x36a8));
    c.g[S2][0] = 0x6c; // the sample
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MONITOR_PUSH_QUEUE));
    Callee {
        addr: 0x002a_dde0,
        f: ee::s08::BankMonitor_KillTimersByCallback,
    }
    .call(c, 0x0029_93cc)?;
    audio_push_voice_and_play(c, [0x0029_94a0, 0x0029_95fc, 0x0029_961c])
}

/// The push-and-play sequence the compiler inlined into the
/// `SSXAudioSystem` sound triggers: a new entry on the bank monitor's push
/// queue (`sys->monitor + 0x1d8`) for sample `s2` of bank 9 (volume 100,
/// pan 90, 50, 127), its gain pointer, then `Audio_PlayPushedSound(sys)`.
/// `s3` = sys; `rets` = the return addresses of the memset, gain lookup and
/// play calls. Leaves v0 = the play handle.
pub(crate) fn audio_push_voice_and_play(c: &mut Cpu, rets: [u32; 3]) -> crate::call::Flow {
    // New queue entry: count += 1, copy the 24-byte template into the
    // entry's record.
    c.g[S0][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_MONITOR).lw(c);
    let monitor = c.g[S0][0] as u32;
    c.g[T1][0] = 0x18;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(MONITOR_PUSH_QUEUE));
    let queue = c.g[S1][0] as u32;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor).off(MONITOR_PUSH_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, monitor.wrapping_add(0x1e3));
    c.ldr(2, monitor.wrapping_add(0x1dc));
    c.ldl(4, monitor.wrapping_add(0x1eb));
    c.ldr(4, monitor.wrapping_add(0x1e4));
    c.ldl(7, monitor.wrapping_add(0x1f3));
    c.ldr(7, monitor.wrapping_add(0x1ec));
    let record = c.g[V1][0] as u32;
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->pushHandle[count] = 0 (inline array at +0x1fc).
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x1fc).sw(c, 0);
    boost_empty_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [count] = the first byte of the entry's record
    // (sign-extended; t1 = 0x18 is the record size).
    c.g[A3][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    boost_empty_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, rets[0])?;

    // The entry's parameters.
    c.g[A3][0] = 0x64; // volume
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    boost_empty_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 0x1;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0x9; // bank / channel kind
    c.g[A2][0] = 0;
    boost_empty_store_entry(c, (V1, V0), 0x64, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x68, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x6c, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x4, 0);
    let (a3, t0, t1, t2, t3) = (c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x0, a3);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x8, t0);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0xc, t1);
    boost_empty_store_entry(c, (V0, V1), 0x74, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x80, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x84, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x78, 2, 0, t2);
    boost_empty_store_entry(c, (V0, V1), 0x7c, 2, 0, t3);

    // sys->pushBank[count] = 0; sys->pushSample[count] = sample (s2).
    for (field, value) in [(AUDIO_SYS_PUSH_BANK, 0), (AUDIO_SYS_PUSH_SAMPLE, c.g[S2][0])] {
        let sys = c.g[S3][0] as u32;
        c.g[V1][0] = Addr(sys).off(AUDIO_SYS_MONITOR).lw(c);
        c.g[A3][0] = Addr(sys).off(field).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MONITOR_PUSH_COUNT).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        Addr(c.g[V0][0] as u32).sw(c, value);
    }

    // monitor->pushGain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 9, 0).
    c.g[S0][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_SND_ROOT).lw(c);
    call_ptr(c, 0x0028_7968, rets[1])?;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = c.g[S3][0];
    let monitor = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A2][0] = Addr(monitor).off(0x214).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // Audio_PlayPushedSound(sys)
    call_ptr(c, 0x0029_06b8, rets[2])
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_OnRacePositionChange

/// `SSXRaceMan.humanCount` / `.cpuCount`? (`+0x7c`, `+0x80`): the two
/// counts whose sum is the number of racers.
const RACE_MAN_COUNT_A: u32 = 0x7c;
const RACE_MAN_COUNT_B: u32 = 0x80;
/// `GameModule.raceType` (+0x214): 4 is a race.
const GAME_MODULE_RACE_TYPE: u32 = 0x214;
/// The audio settings byte at 0x535c10 (non-zero: check the race type).
const AUDIO_RACE_TYPE_CHECK: u32 = 0x0053_5c10;

/// `void SSXAudioSystem_OnRacePositionChange(SSXAudioSystem* sys, Rider*
/// rider, Rider* passed, Rider* passer, int limit, int position)`
/// (0x299e28): in a race with at least two racers, for the human local
/// rider (or the rider `FindRider(sys, -1)` returns), play the "position
/// change" sting (sample 0x6d) when the rider moved into first place and
/// it is not already tracked or a replay, then have the riders taunt each
/// other (`RiderSay207D(sys, rider, passed)` when `position < limit`, else
/// `RiderSay207D(sys, passer, rider)`).
pub fn ssx_audio_system_on_race_position_change(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A1][0]; // rider
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[T1][0]; // position
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A3][0]; // passer
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[A2][0]; // passed
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[T0][0]; // limit
        // BELibrary_GetInterface(BELibrary_GetInstance(), 0)
        call_ptr(c, 0x0014_dc80, 0x0029_9e70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_9e7c)?;
        'out: {
            c.g[V1][0] = 0x53_0000;
            c.g[V0][0] = Addr(AUDIO_RACE_TYPE_CHECK).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x002a_4290,
                    f: ee::s08::SSXAudioSystem_IsRaceType4,
                }
                .call(c, 0x0029_9e94)?;
                let ok = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x90);
                if !ok {
                    break 'out;
                }
            }
            c.cycles += 6;
            // SSXAudioSystem_GetGameModule
            call_ptr(c, 0x0028_b1c8, 0x0029_9ea4)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(GAME_MODULE_RACE_TYPE).lw(c);
            c.g[V0][0] = 0x4;
            let race = c.g[V1][0] == c.g[V0][0];
            frame.restore(c, S0, 0x90);
            if !race {
                break 'out;
            }
            c.cycles += 3;
            let rider = c.g[S4][0] as u32;
            c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_HUMAN).lw(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_LOCAL).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                // SSXAudioSystem_FindRider(sys, -1)
                call_ptr(c, 0x0028_5d98, 0x0029_9ed8)?;
                let same = c.g[V0][0] == c.g[S4][0];
                frame.restore(c, S0, 0x90);
                if !same {
                    break 'out;
                }
            }
            c.cycles += 9;
            // SSXAudioSystem_GetRaceMan (twice)
            call_ptr(c, 0x0028_b1d8, 0x0029_9ee8)?;
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(RACE_MAN_COUNT_A).lw(c);
            call_ptr(c, 0x0028_b1d8, 0x0029_9ef0)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(RACE_MAN_COUNT_B).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            if c.g[S0][0] != 0 {
                frame.restore(c, S0, 0x90);
                break 'out;
            }
            /// `position < limit` (slt s5, fp).
            fn ahead(c: &Cpu) -> u64 {
                ((c.g[S5][0] as i64) < (c.g[FP][0] as i64)) as u64
            }
            c.cycles += 2;
            c.g[V0][0] = ahead(c);
            if c.g[S5][0] == 0 {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x0029_5028,
                    f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
                }
                .call(c, 0x0029_9f1c)?;
                let playing = c.g[V0][0] != 0;
                c.g[V0][0] = ahead(c);
                if !playing {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    // SSXAudioSystem_IsReplayPlaying
                    call_ptr(c, 0x0028_8ae0, 0x0029_9f2c)?;
                    let replay = c.g[V0][0] != 0;
                    c.g[V0][0] = ahead(c);
                    if !replay {
                        c.cycles += 160;
                        c.g[S2][0] = 0x6d; // the sample
                        audio_push_voice_and_play(c, [0x0029_a00c, 0x0029_a168, 0x0029_a188])?;
                        // SSXAudioSystem_TrackSound(sys, 1, handle,
                        // (*sys->sndRoot)->time + 200, 0, 0)
                        c.g[V1][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_SND_ROOT).lw(c);
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0x260).lw(c);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc8));
                        Callee {
                            addr: 0x0029_5628,
                            f: ee::s08::SSXAudioSystem_TrackSound,
                        }
                        .call(c, 0x0029_a1b0)?;
                        c.g[V0][0] = ahead(c);
                    }
                }
            }
            let say = Callee {
                addr: 0x002a_0560,
                f: ee::s08::SSXAudioSystem_RiderSay207D,
            };
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[S7][0] != 0 {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S7][0];
                    say.call(c, 0x0029_a1d0)?;
                    frame.restore(c, S0, 0x90);
                    break 'out;
                }
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[S6][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S4][0];
                    say.call(c, 0x0029_a1ec)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_GetFxParams

/// `SndFxParams* SND_GetFxParams(int voice, unsigned flags)` (0x3b8218):
/// the 20-byte parameter record of `voice` in the table of the effect the
/// flags select (`flags & 0x71c`: 8, 4 or 0x100), or null for any other
/// effect.
pub fn snd_get_fx_params(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] &= 0x71c;
    let kind = c.g[A1][0];
    c.g[V1][0] = 0x14; // record size
    // The record of `voice` (a0) in the table at `0x510000 - neg`.
    fn snd_fx_record(c: &mut Cpu, neg: u32) {
        c.mult(0, 3, 4, 3);
        c.g[V0][0] = sx(0x0051_0000u32.wrapping_sub(neg));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    }
    if kind == 0x8 {
        c.cycles += 5;
        snd_fx_record(c, 0x54f8);
    } else {
        c.cycles += 3;
        if (kind as i64) < 9 {
            c.cycles += 2;
            if kind == 0x4 {
                c.cycles += 4;
                snd_fx_record(c, 0x5520);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 3;
            if kind == 0x100 {
                c.cycles += 5;
                snd_fx_record(c, 0x550c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_Reset

/// `BankMonitorInstance` fields cleared by `Reset`.
const BMI_RESET_KIND: u32 = 0x0;
const BMI_RESET_FADING: u32 = 0x54;
const BMI_RESET_TIMER: u32 = 0x58;
const BMI_RESET_PRIORITY: u32 = 0xb0;
const BMI_RESET_KEY: u32 = 0xb4;
const BMI_RESET_ID: u32 = 0xb8;

/// `void BankMonitorInstance_Reset(BankMonitorInstance* self)` (0x2abb38):
/// fade the instance out, then free it: kind 0, not fading, no timer,
/// priority 0x8000, key -1, id 0xffff.
pub fn bank_monitor_instance_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002a_c180,
            f: ee::s08::BankMonitorInstance_FadeOut,
        }
        .call(c, 0x002a_bb4c)?;
        let inst = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = 0x8000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = 0xffff;
        inst.off(BMI_RESET_PRIORITY).sw(c, c.g[V1][0]);
        inst.off(BMI_RESET_ID).sw(c, c.g[V0][0]);
        inst.off(BMI_RESET_KEY).sw(c, c.g[A0][0]);
        inst.off(BMI_RESET_KIND).sw(c, 0);
        inst.off(BMI_RESET_FADING).sw(c, 0);
        inst.off(BMI_RESET_TIMER).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_IsSampleAllowed

/// `int SPCH_IsSampleAllowed(SpchEvent* ev, SpchBank* bank, int choice)`
/// (0x3d8780): whether `choice` may be played for the speech event. The
/// bank header holds the bits per choice (+5), a flags byte (+4: bit 7 =
/// per-slot allow masks present, low nibble = slot count), an extra-bytes
/// count (+6) and the sample count (+0xa); the bit arrays follow the
/// header at `(slots + 2) * 4 + 0xf & 0x3ffc`. With a sample selected
/// (`ev+2 >= 0`) the choice bit of that sample's row must be set; then, if
/// the bank has slot masks, the choice bit of the slot's mask decides.
/// Out-of-range choices are refused.
pub fn spch_is_sample_allowed(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A1][0]; // bank
    c.g[T2][0] = c.g[A2][0]; // choice
    let bank = c.g[T0][0] as u32;
    c.g[A3][0] = Addr(bank).off(0x5).lbu(c);
    let out_of_range = (c.g[A3][0] as i64) < (c.g[T2][0] as i64);
    c.g[A1][0] = 0x1;
    'done: {
        if out_of_range {
            c.cycles += 2;
            c.g[A1][0] = 0;
            break 'done;
        }
        'slots: {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x2).lh(c); // sample
            c.g[A2][0] = u64::MAX;
            if c.g[V1][0] == c.g[A2][0] {
                c.g[A2][0] = Addr(bank).off(0x4).lbu(c);
                break 'slots;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(bank).off(0xa).lhu(c);
            let in_range = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = in_range as u64;
            c.g[A0][0] = 0;
            if in_range {
                // The sample's row of choice bits.
                c.cycles += 14;
                c.mult(0, 2, 7, 3); // v0 = bits per row * sample
                c.g[A0][0] = Addr(bank).off(0x4).lbu(c);
                c.g[T4][0] = 0;
                c.g[V1][0] = c.g[A0][0] & 0xf;
                c.g[A0][0] &= 0x80;
                c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                c.mult(0, 2, 7, 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
                c.g[V0][0] &= 0x3ffc;
                c.g[V0][0] = sx(bank.wrapping_add(c.g[V0][0] as u32));
                let masks = c.g[A0][0] != 0;
                c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if masks {
                    // Skip the slot masks: (bits + 7) / 8 + 1 bytes.
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xe));
                    c.g[A0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V1][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(bank).off(0x6).lbu(c);
                c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T4][0] as u32));
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                }
                // Test bit t1 of the bit array at a3.
                c.cycles += 11;
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[T1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[T1][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V1][0] &= 0xff;
                c.g[A0][0] &= c.g[V1][0];
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[A0][0];
            if c.g[A1][0] == 0 {
                break 'done;
            }
            c.cycles += 1;
            c.g[A2][0] = Addr(bank).off(0x4).lbu(c);
        }
        // The slot masks (flags bit 7): the choice bit of slot `flags & 15`.
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0] & 0x80;
        c.g[A2][0] &= 0xf;
        if c.g[V0][0] == 0 {
            break 'done;
        }
        c.cycles += 21;
        c.g[V0][0] = Addr(bank).off(0x5).lbu(c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2));
        c.g[V1][0] = u64::MAX;
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[T2][0];
        }
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[V0][0] &= 0x3ffc;
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(bank.wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] &= 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1).lbu(c);
        c.g[A1][0] &= c.g[V1][0];
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{check, elf_image, fill, DATA, GAME_GP};

    // ------------------------------------------------------------ Object_Save

    /// `Object_Save`: the same fixture as `ModObj_Save` (with and without a
    /// modifier block; the stream's write slots are fakes, some of which
    /// unwind); the real `ModObj_Save` runs under the wrapper.
    #[test]
    fn object_save_matches() {
        use crate::difftest::fakes::vtable;
        covered(ee::s11::f_00356e98__generated, object_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_6e98, 7);
            c.pc = 0x0035_6e98;
            let (this, stream, block) = (DATA + 0x100, DATA + 0x400, DATA + 0x800);
            fill(c, &mut r, this, 0x10);
            c.bus
                .write32(this + 0x1c, if seed % 3 == 0 { 0 } else { block });
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    // ------------------------------------------------ RiderBase_EnterSubState

    /// `RiderBase_EnterSubState`: the real `.rodata` switch table (from the
    /// ELF image) picks the arm from `rider + 0xde4`; every sub-state 0..13
    /// and out-of-range ids (14, large, negative). The rider base and its
    /// sub-objects are zeroed, with a zeroed world and BELibrary singleton,
    /// so the real `Enter` methods take their short paths; null method
    /// pointers they call through land in the fakes.
    #[test]
    fn rider_base_enter_sub_state_matches() {
        check(ee::s00::f_00111630__generated, rider_base_enter_sub_state, 34, &|c, seed| {
            machine(c, seed, 0x0011_1630, 0);
            elf_image(c);
            c.pc = 0x0011_1630;
            c.g[SP][0] = (crate::difftest::SP - 0x200) as u64;
            for i in 0..0x400 {
                c.bus.write32(DATA + i * 4, 0);
            }
            c.bus.write32(GAME_GP - 0x848, DATA + 0x1000); // world
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1100); // BELibrary
            // Sub states 2, 6, 11 and 12 are not driven: their real Enter
            // methods run away on the zeroed rider (AnimController_PlayAnimImpl
            // at 0x312138, the zeroed-heap consolidate at 0x31ef40). Their arms
            // are the same table call as the others.
            let state = match seed % 17 {
                2 | 6 | 11 | 12 => 13,
                s @ 0..=13 => s,
                14 => 14,
                15 => 0x100,
                _ => 0xffff_fff0,
            };
            c.bus.write32(DATA + RIDER_BASE_SUB_STATE, state);
            if state < 14 {
                assert_ne!(c.bus.read32(ENTER_SUB_STATE_TABLE + 4 * state), 0);
            }
            c.g[A0][0] = DATA as u64;
        });
    }

    // ----------------------------------------------- PATH_TrackGetQueuedCount

    /// `PATH_TrackGetQueuedCount`: 0..7 queued entries (count field among
    /// random flag bits) over the real `PATH_PlatGetQueuedEntryValue` with
    /// the same monitor fixture as its own test: empty and filled monitor
    /// slots (an empty one gives -8: skipped), stale status, sub handles
    /// with the sign bit (negative values skipped).
    #[test]
    fn path_track_get_queued_count_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::game::sound::bankmonitor::{
            PQ_MON_STALE, PQ_MON_SUBS, PQ_MON_SUB_COUNT, PQ_MON_TABLE, PQ_MON_VALUE,
            PQ_SUB_HANDLE, PQ_SUB_VALUE,
        };
        const TRACK: u32 = DATA + 0x20_0000;
        const BANKS: u32 = DATA + 0x21_0000;
        const SUBS: u32 = DATA + 0x21_1000;
        check(ee::s14::f_003d4950__generated, path_track_get_queued_count, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_4950, 0);
            c.pc = 0x003d_4950;
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            for i in 0..4u32 {
                let e = PQ_MON_TABLE + 4 * i;
                if (seed / 8 + i) % 5 == 0 {
                    c.bus.write32(e, 0);
                    continue;
                }
                let bank = BANKS + 0x100 * i;
                let subs = SUBS + 0x100 * i;
                c.bus.write32(e, bank);
                c.bus.write32(bank, u32::MAX); // no stream: refresh skips it
                c.bus.write8(bank + PQ_MON_STALE, ((seed + i) % 4 == 0) as u8);
                c.bus.write8(bank + PQ_MON_SUB_COUNT, (r.next_u32() % 8) as u8);
                c.bus.write32(bank + PQ_MON_SUBS, subs);
                c.bus.write32(bank + PQ_MON_VALUE, r.next_u32());
                for j in 0..8u32 {
                    let h = if r.next_u32() % 3 == 0 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() >> 1
                    };
                    c.bus.write32(subs + 0xc * j + PQ_SUB_HANDLE, h);
                    c.bus.write32(subs + 0xc * j + PQ_SUB_VALUE, r.next_u32() >> 4);
                }
            }
            let count = seed % 8;
            let flags = (r.next_u32() & !(0x1f << 14)) | count << 14;
            c.bus.write32(TRACK + PATH_TRACK_FLAGS, flags);
            c.bus.write32(TRACK + PATH_TRACK_SLOT, (seed / 2) % 4);
            c.g[A0][0] = sx(TRACK);
        });
    }

    // ------------------------------------------------------ SSXAudioSystem

    /// Audio-system test world: the audio system, its bank monitor (push
    /// queue tables zeroed), a rider, the game chain for
    /// `SSXAudioSystem_IsReplayPlaying`, fake SND lock hooks.
    pub(crate) mod audio {
        use super::*;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::Rng;

        pub const SYS: u32 = DATA + 0x20_0000;
        pub const MONITOR: u32 = DATA + 0x21_0000;
        pub const ROOT: u32 = DATA + 0x22_0000;
        pub const TABLES: u32 = DATA + 0x23_0000;
        pub const RIDER: u32 = DATA + 0x24_0000;
        pub const GAME: u32 = DATA + 0x25_0000;
        pub const MODULE: u32 = DATA + 0x25_1000;
        pub const REPLAY: u32 = DATA + 0x25_2000;

        /// The machine (fakes, no unwinds), the ELF image, and the world;
        /// `count` entries already in the push queue (0..=2).
        pub fn world(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
            let mut r = machine(c, seed, salt, 0);
            elf_image(c);
            c.pc = entry;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x26_0000); // BELibrary
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE); // SND_Lock hook
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10); // SND_Unlock hook
            // Replay state: not in game, no replay, or replay states 0/5/12/20.
            c.bus.write32(GAME_GP - 0x848, GAME);
            let in_game = seed % 7 != 0;
            c.bus.write32(GAME + 0x84, if in_game { MODULE } else { 0 });
            c.bus.write32(MODULE + 0x28, if seed % 5 == 1 { 0 } else { REPLAY });
            let replay = [0u32, 0, 5, 12, 20][(seed / 3 % 5) as usize];
            c.bus.write32(REPLAY, replay);
            // The system and its monitor.
            c.bus.write32(SYS + AUDIO_SYS_MONITOR, MONITOR);
            c.bus.write32(SYS + AUDIO_SYS_PUSH_BANK, TABLES + 0x1000);
            c.bus.write32(SYS + AUDIO_SYS_PUSH_SAMPLE, TABLES + 0x1100);
            c.bus.write32(SYS + AUDIO_SYS_SND_ROOT, ROOT);
            c.bus.write32(ROOT, MONITOR);
            for i in 0..0x400 {
                c.bus.write32(MONITOR + 4 * i, 0);
            }
            c.bus.write32(MONITOR + MONITOR_PUSH_COUNT, seed / 2 % 3);
            fill(c, &mut r, MONITOR + 0x1dc, 6); // 24-byte entry template
            // Queue tables (queue + 0x20 .. + 0x84), zeroed.
            for (i, off) in [0x20u32, 0x34, 0x38, 0x3c]
                .into_iter()
                .chain((0x60..=0x84).step_by(4))
                .enumerate()
            {
                c.bus
                    .write32(MONITOR + MONITOR_PUSH_QUEUE + off, TABLES + 0x100 * i as u32);
            }
            for i in 0..0x1200 / 4 {
                c.bus.write32(TABLES + 4 * i, 0);
            }
            // Bank-loaded table: zero (no bank: SND_Play fails at once).
            c.bus.write32(MONITOR + 0xacc, TABLES + 0x1180);
            // Channel gains (SSXAudioSystem_GetChannelGainPtr).
            fill(c, &mut r, SYS + 0x6300, 0x40);
            c.bus.write32(SYS + 0x6358, r.f32(1.0));
            // The rider.
            c.bus.write32(RIDER + RIDER_AUDIO_HUMAN, (seed % 4 != 1) as u32);
            c.bus.write32(RIDER + RIDER_AUDIO_LOCAL, if seed % 6 == 2 { 0 } else { 1 });
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
            r
        }
    }

    /// `SSXAudioSystem_PlayBoostEmptySound`: in and out of replays, human
    /// and local or not; the play path runs the real timer kill, memset,
    /// gain lookup and `Audio_PlayPushedSound` (no bank loaded: `SND_Play`
    /// fails and the pushed entry is dropped again).
    #[test]
    fn ssx_audio_system_play_boost_empty_sound_matches() {
        check(
            ee::s08::f_00299368__generated,
            ssx_audio_system_play_boost_empty_sound,
            120,
            &|c, seed| {
                audio::world(c, seed, 0x29_9368, 0x0029_9368);
            },
        );
    }

    /// `SSXAudioSystem_OnRacePositionChange`: race-type check byte on/off
    /// (the real `IsRaceType4`), no game module / not a race, two racers
    /// or fewer, a human local rider or not (the real `FindRider`), first
    /// place or not (the sting: real tracked-sound ring, empty, the real
    /// push queue and `Audio_PlayPushedSound` with no bank, `TrackSound`),
    /// position ahead of or behind the limit with and without the other
    /// rider (the real `RiderSay207D`, mostly on its speech-off early out).
    #[test]
    fn ssx_audio_system_on_race_position_change_matches() {
        use crate::difftest::fakes::FAKE;
        check(
            ee::s08::f_00299e28__generated,
            ssx_audio_system_on_race_position_change,
            200,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_9e28, 0x0029_9e28);
                // BELibrary_GetInterface: interface table entry 0 with a
                // fake method.
                let iface = DATA + 0x26_1000;
                let vt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x40);
                c.bus.write8(AUDIO_RACE_TYPE_CHECK, (seed % 3 == 0) as u8);
                c.bus.write8(0x0053_5c12, if seed % 9 == 0 { 2 } else { 4 });
                c.bus.write32(
                    audio::MODULE + GAME_MODULE_RACE_TYPE,
                    if seed % 11 == 5 { 2 } else { 4 },
                );
                let race_man = DATA + 0x25_3000;
                c.bus.write32(audio::MODULE + 0xc, race_man);
                c.bus.write32(race_man + RACE_MAN_COUNT_A, 1 + seed % 2);
                c.bus.write32(race_man + RACE_MAN_COUNT_B, if seed % 13 == 4 { 0 } else { 2 });
                // Tracked-sound ring: capacity 8, empty.
                c.bus.write32(audio::SYS + 0x597c, 8);
                c.bus.write32(audio::SYS + 0x5984, 3);
                c.bus.write32(audio::SYS + 0x5988, 3);
                // (*sndRoot)->time.
                c.bus.write32(audio::MONITOR + 0x260, r.next_u32() % 10000);
                // RiderSay207D: speech mode 2 (its early out) mostly.
                let speech = DATA + 0x26_2000;
                c.bus.write32(GAME_GP - 0x84c, speech);
                c.bus.write32(speech + 0x550, if seed % 10 == 7 { 0 } else { 2 });
                // Arguments: passed / passer riders, limit, position.
                let other = DATA + 0x24_1000;
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(other) };
                c.g[A3][0] = if seed % 5 == 0 { 0 } else { sx(other + 0x1000) };
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = if seed % 2 == 0 { 0 } else { (r.next_u32() % 4) as u64 };
            },
        );
    }

    /// `SND_GetFxParams`: each effect kind (8, 4, 0x100), other kinds below
    /// and above 9, random high flag bits, any voice.
    #[test]
    fn snd_get_fx_params_matches() {
        use crate::difftest::check_lohi;
        check_lohi(ee::s13::f_003b8218__generated, snd_get_fx_params, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_8218, 0);
            c.pc = 0x003b_8218;
            let kind = [0x8u32, 0x4, 0x100, 0x0, 0x10, 0x18, 0x200, 0x71c][(seed % 8) as usize];
            let noise = if seed % 3 == 0 { r.next_u32() & !0x71c } else { 0 };
            c.g[A1][0] = sx(kind | noise);
            c.g[A0][0] = if seed % 5 == 0 { r.next_u32() as u64 } else { (r.next_u32() % 64) as u64 };
        });
    }

    /// `BankMonitorInstance_Reset`: the `FadeOut` fixture (already fading,
    /// or kind 1 / 2 / other; null SND lock callbacks are fakes, some
    /// unwind; a handle that finds no channel), with the real `FadeOut`.
    #[test]
    fn bank_monitor_instance_reset_matches() {
        covered(ee::s08::f_002abb38__generated, bank_monitor_instance_reset, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_bb38, 4);
            c.pc = 0x002a_bb38;
            let inst = DATA;
            c.g[A0][0] = inst as u64;
            c.bus.write32(inst + 0x8, r.next_u32()); // playing
            c.bus.write32(inst + 0x54, if seed % 5 == 0 { 1 + r.next_u32() % 3 } else { 0 });
            let kind = match seed % 4 {
                0 | 1 => 1,
                2 => 2,
                _ => r.next_u32() % 6,
            };
            c.bus.write32(inst, kind);
            let handle = if r.next_u32() % 2 == 0 { 0xffff_ffff } else { r.next_u32() & 0x7fff_ffff };
            c.bus.write32(inst + 0x4, handle);
        });
    }

    /// `SPCH_IsSampleAllowed`: choices in and out of range (negative too),
    /// no sample selected (-1), samples in and out of range, flags with and
    /// without slot masks, extra bytes or none, random bit arrays.
    #[test]
    fn spch_is_sample_allowed_matches() {
        use crate::difftest::check_lohi;
        const EV: u32 = DATA + 0x100;
        const BANK: u32 = DATA + 0x1000;
        check_lohi(ee::s14::f_003d8780__generated, spch_is_sample_allowed, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_8780, 0);
            c.pc = 0x003d_8780;
            fill(c, &mut r, BANK, 0x100);
            let bits = r.next_u32() % 24;
            c.bus.write8(BANK + 5, bits as u8);
            let flags = (r.next_u32() % 16) | if seed % 2 == 0 { 0x80 } else { 0 };
            c.bus.write8(BANK + 4, flags as u8);
            c.bus.write8(BANK + 6, if seed % 3 == 0 { 0 } else { (r.next_u32() % 5) as u8 });
            let samples = r.next_u32() % 8;
            c.bus.write16(BANK + 0xa, samples as u16);
            let sample: u16 = match seed % 5 {
                0 => 0xffff,
                1 => (samples + r.next_u32() % 3) as u16,
                _ => (r.next_u32() % samples.max(1)) as u16,
            };
            c.bus.write16(EV + 2, sample);
            let choice = match seed % 7 {
                0 => bits + 1 + r.next_u32() % 4,
                1 => 0u32.wrapping_sub(1 + r.next_u32() % 8), // small negative
                _ => r.next_u32() % (bits + 1),
            };
            c.g[A0][0] = sx(EV);
            c.g[A1][0] = sx(BANK);
            c.g[A2][0] = sx(choice);
        });
    }
}
