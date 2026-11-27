//! Staging module oc1030 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1030`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::Flow;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::sound::soundsys::{
    audio_push_voice_and_play, boost_empty_store_entry, AUDIO_SYS_MONITOR, AUDIO_SYS_SND_ROOT,
    MONITOR_PUSH_COUNT, MONITOR_PUSH_QUEUE, RIDER_AUDIO_HUMAN, RIDER_AUDIO_LOCAL,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `lw base + off` (sign-extended word); the base is read before `c` is
/// borrowed.
macro_rules! oc1030_lw {
    ($c:expr, $base:expr, $off:expr) => {{
        let base = $base as u32;
        $c.lw(base.wrapping_add($off))
    }};
}

/// Epilogue of `SSXAudioSystem_OnTrickLanded`: restore s1..s6 and `ra`.
fn on_trick_landed_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 9;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore(c, S5, 0x20);
    frame.restore(c, S6, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void SSXAudioSystem::OnTrickLanded(Rider* rider, int ubers, int runUbers)`
/// (0x29b430; `a0` = system, `a1` = rider, `a2` = `runUbers`, `a3` = `ubers`).
/// Outside replays, for a human local rider, pushes the voice sample 0x67 of
/// bank 9 (the inlined `sndPlayVoice`), then, if the trick count `ubers` is
/// below 4 and `ubers + runUbers` reaches 4 but not 9, says the arcade phrase
/// (`ArcadeSay2133(this, rider, 1)`).
pub fn ssx_audio_system_on_trick_landed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A2][0];
        // SSXAudioSystem_IsReplayPlaying
        call_ptr(c, 0x0028_8ae0, 0x0029_b468)?;
        let replay = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x70);
        if !replay {
            on_trick_landed_body(c, frame)?;
        }
        on_trick_landed_epilogue(c, frame);
        Ok(())
    })
}

/// The part of `OnTrickLanded` after the replay check; the early `Ok` returns
/// leave through the caller's epilogue.
fn on_trick_landed_body(c: &mut Cpu, frame: Frame) -> Flow {
    // Human and local rider (+0x874, +0x87c > 0).
    c.cycles += 3;
    let rider = c.g[S4][0];
    c.g[V0][0] = oc1030_lw!(c, rider, RIDER_AUDIO_HUMAN);
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = oc1030_lw!(c, rider, RIDER_AUDIO_LOCAL);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    c.g[T1][0] = 0x18;
    if c.g[V1][0] != 0 {
        c.cycles += 151;
        c.g[S2][0] = 0x67; // the sample
        audio_push_voice_and_play(c, [0x0029_b560, 0x0029_b6bc, 0x0029_b6dc])?;
        c.g[V0][0] = ((c.g[S5][0] as i64) < 4) as u64;
        c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S6][0] as u32));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
            // Branch delay slot: s0 is restored before the ArcadeSay call
            // (which then saves the caller's s0).
            frame.restore(c, S0, 0x70);
            if c.g[V0][0] != 0 {
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 9) as u64;
            c.g[A0][0] = c.g[S3][0];
            if c.g[V0][0] == 0 {
                return Ok(());
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 0x1;
            Callee {
                addr: 0x002a_3de0,
                f: ee::s08::SSXAudioSystem_ArcadeSay2133,
            }
            .call(c, 0x0029_b70c)?;
        }
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x70);
    Ok(())
}

/// `void SSXAudioSystem::OnMissionStart()` (0x29d8e0): cancels a pending loop
/// effect (`+0x5fd4`: calls its vtable entry with 0x27 and clears
/// `+0x5fd0`/`+0x5fd4`), pushes the voice sample 0x6d of bank 9 and plays the
/// arcade phrase `ArcadeSay20A8(this, raceMan->field28, 1)`.
pub fn ssx_audio_system_on_mission_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[S3][0];
        c.g[V0][0] = oc1030_lw!(c, this, 0x5fd4);
        c.g[A1][0] = 0x27;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.g[V1][0] = oc1030_lw!(c, this, 0x5558);
            c.g[A2][0] = sx((this as u32).wrapping_add(0x118));
            let vt = c.g[V1][0] as u32;
            c.g[A0][0] = c.lh(vt.wrapping_add(0x20));
            c.g[V0][0] = c.lw(vt.wrapping_add(0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0029_d920)?;
            let this = c.g[S3][0] as u32;
            c.sw(this.wrapping_add(0x5fd0), 0);
            c.sw(this.wrapping_add(0x5fd4), 0);
        }
        c.cycles += 162;
        c.g[S2][0] = 0x6d; // the sample
        audio_push_voice_and_play(c, [0x0029_da00, 0x0029_db5c, 0x0029_db7c])?;
        // GetRaceMan()
        call_ptr(c, 0x0028_b1d8, 0x0029_db84)?;
        c.g[A1][0] = oc1030_lw!(c, c.g[V0][0], 0x28);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = 0x1;
        Callee {
            addr: 0x002a_3c00,
            f: ee::s08::SSXAudioSystem_ArcadeSay20A8,
        }
        .call(c, 0x0029_db94)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `PlayAnimEventSound50/51/52` (the "rider voice" sounds):
/// `void SSXAudioSystem::PlayAnimEventSoundNN(Rider* rider)` (`a0` = system,
/// `a1` = rider). For a human local rider it pushes sample `voice` of bank 9
/// on the bank monitor (the inlined `sndPlayVoice`: volume 100, pan 90, 50,
/// 127), clamps the volume byte, stores the channel gain pointer, the
/// rider's source (`rider + 0x110`) and listener index, and plays it
/// (`Audio_PlayPushedSound`). `rets` = return addresses of the memset, gain
/// lookup, listener lookup and play calls.
fn rider_voice_sound_oc1030(c: &mut Cpu, voice: u32, rets: [u32; 4]) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        let rider = c.g[S5][0];
        c.g[V0][0] = oc1030_lw!(c, rider, RIDER_AUDIO_HUMAN);
        c.g[S4][0] = c.g[A0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = oc1030_lw!(c, rider, RIDER_AUDIO_LOCAL);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[S2][0] = 0x18;
        if c.g[V1][0] != 0 {
            c.cycles += 154;
            let sys = c.g[S4][0];
            // New push-queue entry: count += 1, copy the 24-byte template.
            c.g[S0][0] = oc1030_lw!(c, sys, AUDIO_SYS_MONITOR);
            let monitor = c.g[S0][0] as u32;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[S1][0] = sx(monitor.wrapping_add(MONITOR_PUSH_QUEUE));
            let queue = c.g[S1][0] as u32;
            c.g[S3][0] = voice as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(monitor.wrapping_add(MONITOR_PUSH_COUNT), c.g[V0][0]);
            c.g[V1][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[V0][0] = c.lw(queue.wrapping_add(0x20));
            c.mult(0, V1, V1, S2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(V0, monitor.wrapping_add(0x1e3));
            c.ldr(V0, monitor.wrapping_add(0x1dc));
            c.ldl(A0, monitor.wrapping_add(0x1eb));
            c.ldr(A0, monitor.wrapping_add(0x1e4));
            c.ldl(A3, monitor.wrapping_add(0x1f3));
            c.ldr(A3, monitor.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(V0, record.wrapping_add(0x7));
            c.sdr(V0, record);
            c.sdl(A0, record.wrapping_add(0xf));
            c.sdr(A0, record.wrapping_add(0x8));
            c.sdl(A3, record.wrapping_add(0x17));
            c.sdr(A3, record.wrapping_add(0x10));
            // monitor->pushHandle[count] = 0 (inline array at +0x1fc).
            c.g[V0][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
            boost_empty_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
            // table 0x38 [count] = first byte of the entry's record.
            c.g[A3][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[A0][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[T0][0] = c.lw(queue.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, S2);
            c.g[V0][0] = c.lw(queue.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            boost_empty_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
            // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
            c.g[A0][0] = c.lw(monitor.wrapping_add(MONITOR_PUSH_COUNT));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, rets[0])?;

            // The entry's parameters: volume 100, pan 90, 50, 127, 1.
            c.g[A0][0] = 0x64;
            c.g[A1][0] = 0x5a;
            c.g[A2][0] = 0x32;
            c.g[A3][0] = 0x7f;
            boost_empty_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
            c.g[T0][0] = 0x1;
            let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
            boost_empty_store_entry(c, (V0, V1), 0x64, 2, 0, 0);
            boost_empty_store_entry(c, (V1, V0), 0x68, 2, 0, 0);
            boost_empty_store_entry(c, (V0, V1), 0x6c, 2, 0, 0);
            boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x4, 0);
            boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x0, a0);
            boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x8, a1);
            boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0xc, a2);
            boost_empty_store_entry(c, (V1, V0), 0x74, 2, 0, 0);
            boost_empty_store_entry(c, (V0, V1), 0x80, 2, 0, 0);
            boost_empty_store_entry(c, (V1, V0), 0x84, 2, 0, 0);
            boost_empty_store_entry(c, (V0, V1), 0x78, 2, 0, a3);
            boost_empty_store_entry(c, (V1, V0), 0x7c, 2, 0, t0);

            // sys->pushBank[count] = 0; sys->pushSample[count] = voice (s3).
            c.g[A0][0] = oc1030_lw!(c, c.g[S4][0], 0);
            c.g[V1][0] = oc1030_lw!(c, c.g[S4][0], 4);
            c.g[V0][0] = oc1030_lw!(c, c.g[A0][0], MONITOR_PUSH_COUNT);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0);
            c.g[V1][0] = oc1030_lw!(c, c.g[S4][0], 0);
            c.g[A0][0] = oc1030_lw!(c, c.g[S4][0], 8);
            c.g[V0][0] = oc1030_lw!(c, c.g[V1][0], MONITOR_PUSH_COUNT);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S3][0]);

            // Root monitor: volume[count] = 127, then the record's volume
            // byte = clamp(volume[count], 0, 127).
            c.g[V1][0] = oc1030_lw!(c, c.g[S4][0], AUDIO_SYS_SND_ROOT);
            c.g[A1][0] = oc1030_lw!(c, c.g[V1][0], 0);
            c.g[V0][0] = oc1030_lw!(c, c.g[A1][0], MONITOR_PUSH_COUNT);
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(MONITOR_PUSH_QUEUE));
            c.g[V1][0] = oc1030_lw!(c, c.g[A2][0], 0x38);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[A3][0]);
            c.g[A0][0] = oc1030_lw!(c, c.g[A1][0], MONITOR_PUSH_COUNT);
            c.g[V1][0] = oc1030_lw!(c, c.g[A2][0], 0x1c);
            c.g[A3][0] = oc1030_lw!(c, c.g[A2][0], 0x38);
            c.mult(0, A0, A0, S2);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = oc1030_lw!(c, c.g[A2][0], 0x20);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
            let below_top = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            // The current volume word: volume[queue->count].
            let vol_at = |c: &mut Cpu| -> u32 {
                let n = oc1030_lw!(c, c.g[A2][0], 0x1c);
                ((n as u32) << 2).wrapping_add(c.g[A3][0] as u32)
            };
            let mut negative = false;
            if below_top {
                c.cycles += 6;
                let at = vol_at(c);
                c.g[V0][0] = sx(at);
                c.g[V1][0] = c.lw(at);
                c.g[V0][0] = 0;
                negative = (c.g[V1][0] as i64) < 0;
            }
            if !negative {
                c.cycles += 7;
                let at = vol_at(c);
                c.g[V0][0] = sx(at);
                c.g[V1][0] = c.lw(at);
                c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0x7f;
                } else {
                    c.cycles += 4;
                    let at = vol_at(c);
                    c.g[V0][0] = sx(at);
                    c.g[V0][0] = c.lbu(at);
                }
            }
            c.cycles += 31;
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            // monitor->gain[count] = GetChannelGainPtr(sys, 5, 0)
            c.g[A2][0] = 0;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0x5;
            c.g[S0][0] = oc1030_lw!(c, c.g[S4][0], AUDIO_SYS_SND_ROOT);
            call_ptr(c, 0x0028_7968, rets[1])?;
            c.g[A3][0] = oc1030_lw!(c, c.g[S0][0], 0);
            c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x110));
            c.g[A1][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = oc1030_lw!(c, c.g[A3][0], MONITOR_PUSH_COUNT);
            c.g[A2][0] = oc1030_lw!(c, c.g[A3][0], 0x214);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            // monitor->source[count] = &rider->source (rider + 0x110)
            c.g[A2][0] = oc1030_lw!(c, c.g[S4][0], AUDIO_SYS_SND_ROOT);
            c.g[V1][0] = oc1030_lw!(c, c.g[A2][0], 0);
            c.g[V0][0] = oc1030_lw!(c, c.g[V1][0], MONITOR_PUSH_COUNT);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[T0][0]);
            // monitor->listener[count] = GetRiderListenerIndex(sys, rider)
            c.g[S0][0] = oc1030_lw!(c, c.g[S4][0], AUDIO_SYS_SND_ROOT);
            call_ptr(c, 0x0028_8a20, rets[2])?;
            c.g[A1][0] = oc1030_lw!(c, c.g[S0][0], 0);
            c.g[A0][0] = c.g[S4][0];
            c.g[V1][0] = oc1030_lw!(c, c.g[A1][0], MONITOR_PUSH_COUNT);
            c.g[A2][0] = oc1030_lw!(c, c.g[A1][0], 0x23c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[RA][0] = sx(rets[3]);
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            // Audio_PlayPushedSound(sys)
            call_ptr(c, 0x0029_06b8, rets[3])?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem::PlayAnimEventSound51(Rider*)` (0x29b968): rider
/// voice sample 0x50 (see [`rider_voice_sound_oc1030`]).
pub fn ssx_audio_system_play_anim_event_sound51(c: &mut Cpu) {
    rider_voice_sound_oc1030(
        c,
        0x50,
        [0x0029_ba80, 0x0029_bc70, 0x0029_bcb4, 0x0029_bcd4],
    );
}

/// `void SSXAudioSystem::PlayAnimEventSound50(Rider*)` (0x29bcf8): rider
/// voice sample 0x51.
pub fn ssx_audio_system_play_anim_event_sound50(c: &mut Cpu) {
    rider_voice_sound_oc1030(
        c,
        0x51,
        [0x0029_be10, 0x0029_c000, 0x0029_c044, 0x0029_c064],
    );
}

/// `void SSXAudioSystem::PlayAnimEventSound52(Rider*)` (0x29c088): rider
/// voice sample 0x52.
pub fn ssx_audio_system_play_anim_event_sound52(c: &mut Cpu) {
    rider_voice_sound_oc1030(
        c,
        0x52,
        [0x0029_c1a0, 0x0029_c390, 0x0029_c3d4, 0x0029_c3f4],
    );
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_b430,
        ssx_audio_system_on_trick_landed,
        "SSXAudioSystem_OnTrickLanded",
    ),
    (
        0x0029_b968,
        ssx_audio_system_play_anim_event_sound51,
        "SSXAudioSystem_PlayAnimEventSound51",
    ),
    (
        0x0029_bcf8,
        ssx_audio_system_play_anim_event_sound50,
        "SSXAudioSystem_PlayAnimEventSound50",
    ),
    (
        0x0029_c088,
        ssx_audio_system_play_anim_event_sound52,
        "SSXAudioSystem_PlayAnimEventSound52",
    ),
    (
        0x0029_d8e0,
        ssx_audio_system_on_mission_start,
        "SSXAudioSystem_OnMissionStart",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::sound::soundsys::tests::{audio, midi_bank};

    /// `OnTrickLanded` against the generated body: in and out of replays
    /// (real `IsReplayPlaying`), human and local rider or not, the push of
    /// sample 0x67 (real memset, gain lookup, `Audio_PlayPushedSound`), and
    /// `ubers` / `runUbers` below 4, reaching 4..8, or 9 and more (the real
    /// `ArcadeSay2133`, which stops early: speech off, or the rider's air
    /// value not negative).
    #[test]
    fn ssx_audio_system_on_trick_landed_matches() {
        check(
            ee::s08::f_0029b430__generated,
            ssx_audio_system_on_trick_landed,
            300,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_b430, 0x0029_b430);
                c.bus.write32(
                    audio::SYS + 0x635c,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0x3f80_0000
                    },
                );
                c.bus
                    .write32(audio::RIDER + 0x470, r.f32(4.0) & 0x7fff_ffff);
                let ubers = [-1i32, 0, 1, 3, 3, 4, 5, 8][(r.next_u32() % 8) as usize];
                let run = (r.next_u32() % 9) as i32;
                c.g[A2][0] = sx(run as u32);
                c.g[A3][0] = sx(ubers as u32);
            },
        );
    }

    /// `OnMissionStart`: a pending loop effect (its vtable call is a fake that
    /// sometimes unwinds) or not, the push of sample 0x6d, then
    /// `GetRaceMan()` and the real `ArcadeSay20A8` with and without a rider.
    #[test]
    fn ssx_audio_system_on_mission_start_matches() {
        const RACE_MAN: u32 = DATA + 0x27_0000;
        const VTABLE: u32 = DATA + 0x27_1000;
        check(
            ee::s08::f_0029d8e0__generated,
            ssx_audio_system_on_mission_start,
            300,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_d8e0, 0x0029_d8e0);
                c.bus.write32(audio::MODULE + 0xc, RACE_MAN);
                c.bus.write32(
                    RACE_MAN + 0x28,
                    if seed % 3 == 0 { 0 } else { audio::RIDER },
                );
                c.bus.write32(
                    audio::SYS + 0x635c,
                    if seed % 5 == 2 { 0 } else { 0x3f80_0000 },
                );
                c.bus
                    .write32(audio::RIDER + 0x470, r.f32(4.0) & 0x7fff_ffff);
                let pending = seed % 4 != 1;
                c.bus
                    .write32(audio::SYS + 0x5fd4, if pending { 1 + seed % 3 } else { 0 });
                c.bus.write32(audio::SYS + 0x5fd0, r.next_u32());
                c.bus.write32(audio::SYS + 0x5558, VTABLE);
                c.bus
                    .write16(VTABLE + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(VTABLE + 0x24, FAKE);
                if pending && seed % 7 == 0 {
                    // The vtable call unwinds.
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
            },
        );
    }

    /// The fixture of the rider-voice sounds: the MIDI bank structures of the
    /// other `SSXAudioSystem` sound tests, a rider with its human / local
    /// flags, and (for a third of the seeds) a volume table that aliases the
    /// bank's write cursor, so the stored volume 127 becomes the cursor and the
    /// clamp reads other values (negative, below 127, 127 and up).
    fn rider_voice_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        const RIDER: u32 = DATA + 0x3000;
        const BANK2: u32 = DATA + 0x700;
        let mut r = Rng(entry as u64 ^ (seed as u64) << 20);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.g[A1][0] = RIDER as u64;
        midi_bank(c, &mut r, seed);
        c.bus.write32(RIDER + 0x874, [1, 0, 2][(seed % 3) as usize]);
        c.bus
            .write32(RIDER + 0x87c, [0, 1, 3, 2, 0][(seed % 5) as usize]);
        if seed % 3 == 1 {
            let cursor = seed % 4;
            let volume = BANK2 + 0x1f4 - 4 * cursor;
            c.bus.write32(BANK2 + 0x1d8 + 0x38, volume);
            let word = match r.next_u32() % 4 {
                0 => r.next_u32() | 0x8000_0000,
                1 => r.next_u32() % 127,
                2 => 127,
                _ => 127 + r.next_u32() % 100,
            };
            c.bus.write32(volume + 127 * 4, word);
        }
    }

    /// `PlayAnimEventSound51`: nothing pending, count 0, and the play path
    /// (real memset, gain lookup, listener lookup and `Audio_PlayPushedSound`).
    #[test]
    fn ssx_audio_system_play_anim_event_sound51_matches() {
        check(
            ee::s08::f_0029b968__generated,
            ssx_audio_system_play_anim_event_sound51,
            240,
            &|c, seed| rider_voice_fixture(c, seed, 0x0029_b968),
        );
    }

    /// `PlayAnimEventSound50`, as above with sample 0x51.
    #[test]
    fn ssx_audio_system_play_anim_event_sound50_matches() {
        check(
            ee::s08::f_0029bcf8__generated,
            ssx_audio_system_play_anim_event_sound50,
            240,
            &|c, seed| rider_voice_fixture(c, seed, 0x0029_bcf8),
        );
    }

    /// `PlayAnimEventSound52`, as above with sample 0x52.
    #[test]
    fn ssx_audio_system_play_anim_event_sound52_matches() {
        check(
            ee::s08::f_0029c088__generated,
            ssx_audio_system_play_anim_event_sound52,
            240,
            &|c, seed| rider_voice_fixture(c, seed, 0x0029_c088),
        );
    }
}
