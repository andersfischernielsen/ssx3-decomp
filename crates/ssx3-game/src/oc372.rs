//! Staging module oc372 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc372`.

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
    0x0029_8488,
    ssx_audio_system_play_rider_collision_sound,
    "SSXAudioSystem_PlayRiderCollisionSound",
)];

// ------------------------------------ SSXAudioSystem_PlayRiderCollisionSound

/// Rider: the embedded physics object (its vtable first: slot 0x10 = the
/// velocity vector, slot 0x28 = the world position).
pub(crate) const PRCS_RIDER_BODY: u32 = 0x6c0;

/// Rider: the "human" / "local" flags (both set: a local human rider).
pub(crate) const PRCS_RIDER_HUMAN: u32 = 0x874;

pub(crate) const PRCS_RIDER_LOCAL: u32 = 0x87c;

/// Rider: the sound source position handed to the push queue (`+0x110`).
pub(crate) const PRCS_RIDER_SOURCE: u32 = 0x110;

/// Audio system: the bank monitor (`+0`), the push bank/sample tables
/// (`+4`, `+8`), the sound root (`+0x118`, `**` = the monitor).
pub(crate) const PRCS_SYS_PUSH_BANK: u32 = 0x4;

pub(crate) const PRCS_SYS_PUSH_SAMPLE: u32 = 0x8;

pub(crate) const PRCS_SYS_SND_ROOT: u32 = 0x118;

/// Bank monitor: the push queue (`+0x1d8`), its entry count (`+0x1f4`),
/// the clock (`+0x260`).
pub(crate) const PRCS_MONITOR_QUEUE: u32 = 0x1d8;

pub(crate) const PRCS_MONITOR_COUNT: u32 = 0x1f4;

pub(crate) const PRCS_MONITOR_CLOCK: u32 = 0x260;

/// The tracked-sound key of rider collisions.
pub(crate) const PRCS_TRACK_KEY: u64 = 7;

/// `SSXAudioSystem_IsTrackedSoundPlaying` (0x295028).
const PRCS_IS_TRACKED_SOUND_PLAYING: Callee = Callee {
    addr: 0x0029_5028,
    f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
};

/// `SndPlayParams_SetRadiusVanish` (0x2a9988).
const PRCS_SET_RADIUS_VANISH: Callee = Callee {
    addr: 0x002a_9988,
    f: ee::s08::SndPlayParams_SetRadiusVanish,
};

/// `SSXAudioSystem_TrackSound` (0x295628).
const PRCS_TRACK_SOUND: Callee = Callee {
    addr: 0x0029_5628,
    f: ee::s08::SSXAudioSystem_TrackSound,
};

/// Rider a's (`s6`) physics virtual at `slot`; `a0` = adjusted `this`.
fn prcs_rider_body_call(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    c.g[V1][0] = Addr(r32(c, S6).wrapping_add(PRCS_RIDER_BODY)).lw(c);
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// One `table[count] (<< shift) + field = value` store of the push queue
/// (`s0` = monitor, `s1` = queue): `idx` ends as the address, `base` as the
/// table pointer.
fn prcs_queue_store(
    c: &mut Cpu,
    (idx, base): (usize, usize),
    table: u32,
    shift: u32,
    field: u32,
    value: u64,
) {
    c.g[idx][0] = Addr(r32(c, S0).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[base][0] = Addr(r32(c, S1).wrapping_add(table)).lw(c);
    c.g[idx][0] = sx(r32(c, idx) << shift);
    c.g[idx][0] = sx(r32(c, idx).wrapping_add(r32(c, base)));
    Addr(r32(c, idx).wrapping_add(field)).sw(c, value);
}

/// One `sys->table[monitor->count] = value` store (`s5` = sys): `t` ends as
/// the table pointer, `m` as the monitor.
fn prcs_sys_store(c: &mut Cpu, m: usize, t: usize, table: u32, value: u64) {
    c.g[m][0] = Addr(r32(c, S5)).lw(c);
    c.g[t][0] = Addr(r32(c, S5).wrapping_add(table)).lw(c);
    c.g[V0][0] = Addr(r32(c, m).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, t)));
    Addr(r32(c, V0)).sw(c, value);
}

/// `void SSXAudioSystem_PlayRiderCollisionSound(SSXAudioSystem* sys, Rider*
/// a, Rider* b)` (0x298488): when a listener view is within 100 m of rider
/// `a` (and one in front of it), one of the two riders is a local human and
/// no collision sound (tracked key 7) is still playing, push sample 0x36 of
/// bank 0 with a volume from the length of the riders' summed velocities
/// (curve table 1, clamped to 0..127), give it a radius of 100 and rider
/// `a`'s listener and source position, play it and track it until the
/// clock + 50. Frame 0xb0 (s0-s6, f20 = 100.0).
pub fn ssx_audio_system_play_rider_collision_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A2][0]; // rider b
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[A0][0]; // sys
        frame.save_fpr(c, 20, 0xa0);
        c.g[S6][0] = c.g[A1][0]; // rider a
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(r32(c, S6).wrapping_add(PRCS_RIDER_BODY));
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x20);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 20); // 100.0
                        // Some view within 100 m of rider a's position?
        prcs_rider_body_call(c, 0x28, 0x0029_84dc)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S5][0];
        c.mov_s(12, 20);
        call_ptr(c, 0x0028_8b40, 0x0029_84ec)?; // SSXAudioSystem_FindViewInRange
                                                // `true`: ends at the shared exit with its extra cycle.
        let extra_cycle = 'play: {
            if (c.g[V0][0] as i64) < 0 {
                break 'play false;
            }
            c.cycles += 11;
            prcs_rider_body_call(c, 0x28, 0x0029_8508)?;
            c.g[A1][0] = c.g[V0][0];
            c.mov_s(12, 20);
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x0028_8c08, 0x0029_8518)?; // SSXAudioSystem_IsAnyViewInRange
            if c.g[V0][0] == 0 {
                break 'play false;
            }
            // A local human rider among the two?
            let mut human = false;
            for rider_reg in [S6, S0] {
                let rider = r32(c, rider_reg);
                c.cycles += 3;
                c.g[V0][0] = Addr(rider.wrapping_add(PRCS_RIDER_HUMAN)).lw(c);
                c.g[V1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(rider.wrapping_add(PRCS_RIDER_LOCAL)).lw(c);
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S5][0];
                if c.g[V1][0] != 0 {
                    human = true;
                    break;
                }
            }
            if !human {
                break 'play true;
            }
            // Already playing?
            c.cycles += 5;
            c.g[A1][0] = PRCS_TRACK_KEY;
            c.g[A2][0] = 0;
            PRCS_IS_TRACKED_SOUND_PLAYING.call(c, 0x0029_8564)?;
            if c.g[V0][0] != 0 {
                break 'play false;
            }
            // Volume: |vel(a) + vel(b)| through curve table 1.
            c.cycles += 193;
            prcs_play(c)?;
            true
        };
        if extra_cycle {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 10;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// The play path of [`ssx_audio_system_play_rider_collision_sound`]
/// (0x29856c..0x298978; its first block's 193 cycles already charged).
fn prcs_play(c: &mut Cpu) -> crate::call::Flow {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.g[S2][0] = 0x18; // queue record size
    c.g[S4][0] = 0x36; // the sample
                       // The two velocities (slot 0x10 of each rider's physics object).
    c.g[V1][0] = Addr(r32(c, S6).wrapping_add(PRCS_RIDER_BODY)).lw(c);
    c.g[A1][0] = sx(r32(c, S6).wrapping_add(PRCS_RIDER_BODY));
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(0x10)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x14)).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0029_858c)?;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(PRCS_RIDER_BODY)).lw(c);
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(PRCS_RIDER_BODY));
    c.g[S0][0] = c.g[V0][0];
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(0x10)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x14)).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0029_85a8)?;
    // sum = va + vb, spilled through the stack; its length in f12.
    let spill_hi = sp(c).wrapping_add(0x10);
    let spill = sp(c);
    c.lqc2(3, r32(c, S0));
    c.lqc2(4, r32(c, V0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spill_hi);
    c.g[A0][0] = 0x1; // curve table 1
    c.lq(V0, spill_hi);
    c.sq(V0, spill);
    c.lqc2(3, spill);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.mtc1(V0, 12);
    c.cvt_s_w(12, 12);
    call_ptr(c, 0x0029_0c10, 0x0029_8604)?; // Audio_EvalCurveTable
                                            // New push-queue entry: count += 1, copy the 24-byte template.
    c.g[S0][0] = Addr(r32(c, S5)).lw(c); // monitor
    c.cvt_w_s(1, 0);
    c.mfc1(S3, 1); // the volume
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    let monitor = r32(c, S0);
    c.g[V0][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(PRCS_MONITOR_QUEUE));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x20)).lw(c);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.ldl(V0, monitor.wrapping_add(0x1e3));
    c.ldr(V0, monitor.wrapping_add(0x1dc));
    c.ldl(A0, monitor.wrapping_add(0x1eb));
    c.ldr(A0, monitor.wrapping_add(0x1e4));
    c.ldl(A3, monitor.wrapping_add(0x1f3));
    c.ldr(A3, monitor.wrapping_add(0x1ec));
    let record = r32(c, V1);
    c.sdl(V0, record.wrapping_add(0x7));
    c.sdr(V0, record);
    c.sdl(A0, record.wrapping_add(0xf));
    c.sdr(A0, record.wrapping_add(0x8));
    c.sdl(A3, record.wrapping_add(0x17));
    c.sdr(A3, record.wrapping_add(0x10));
    // monitor->pushSource[count] = 0 (inline at +0x1fc).
    c.g[V0][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(r32(c, V0)));
    Addr(r32(c, V0).wrapping_add(0x1fc)).sw(c, 0);
    prcs_queue_store(c, (V1, V0), 0x34, 2, 0, 0);
    // vals[count] = the record's first byte (sign-extended).
    c.g[A3][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[A0][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[T0][0] = Addr(r32(c, S1).wrapping_add(0x20)).lw(c);
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
    Addr(r32(c, A3)).sw(c, c.g[V1][0]);
    prcs_queue_store(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (+0x40), 0, 8)
    c.g[A0][0] = Addr(monitor.wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0029_86dc)?;
    // The entry's parameters (volume 100, 90, 50, 127, 1).
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    prcs_queue_store(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T0][0] = 0x1;
    prcs_queue_store(c, (V0, V1), 0x64, 2, 0, 0);
    prcs_queue_store(c, (V1, V0), 0x68, 2, 0, 0);
    prcs_queue_store(c, (V0, V1), 0x6c, 2, 0, 0);
    prcs_queue_store(c, (V1, V0), 0x70, 4, 0x4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    prcs_queue_store(c, (V0, V1), 0x70, 4, 0x0, a0);
    prcs_queue_store(c, (V1, V0), 0x70, 4, 0x8, a1);
    prcs_queue_store(c, (V0, V1), 0x70, 4, 0xc, a2);
    prcs_queue_store(c, (V1, V0), 0x74, 2, 0, 0);
    prcs_queue_store(c, (V0, V1), 0x80, 2, 0, 0);
    prcs_queue_store(c, (V1, V0), 0x84, 2, 0, 0);
    prcs_queue_store(c, (V0, V1), 0x78, 2, 0, a3);
    prcs_queue_store(c, (V1, V0), 0x7c, 2, 0, t0);
    // sys->pushBank[count] = 0; sys->pushSample[count] = 0x36.
    prcs_sys_store(c, A0, V1, PRCS_SYS_PUSH_BANK, 0);
    let sample = c.g[S4][0];
    prcs_sys_store(c, V1, A0, PRCS_SYS_PUSH_SAMPLE, sample);
    // vals[cur] = volume; the record's volume byte = clamp(volume, 0, 127).
    c.g[V1][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    c.g[A1][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[A2][0] = sx(r32(c, A1).wrapping_add(PRCS_MONITOR_QUEUE));
    c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x38)).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    Addr(r32(c, V0)).sw(c, c.g[S3][0]);
    c.g[A0][0] = Addr(r32(c, A1).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x1c)).lw(c);
    c.g[A3][0] = Addr(r32(c, A2).wrapping_add(0x38)).lw(c);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A1][0] = Addr(r32(c, A2).wrapping_add(0x20)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    // The value at vals[cur] (re-read each time, as the original does).
    let val_at = |c: &mut Cpu| {
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
    };
    let mut negative = false;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        val_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        val_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            val_at(c);
            c.g[V0][0] = Addr(r32(c, V0)).lbu(c);
        }
    }
    c.cycles += 49;
    Addr(r32(c, A0)).sb(c, c.g[V0][0]);
    // gain[cur] = SSXAudioSystem_GetChannelGainPtr(sys, 5, 0).
    c.g[A2][0] = 0;
    c.g[A1][0] = 0x5;
    c.g[A0][0] = c.g[S5][0];
    c.g[S0][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    call_ptr(c, 0x0028_7968, 0x0029_88cc)?;
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 13); // -1.0
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x214)).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 12); // 100.0
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // SndPlayParams_SetRadiusVanish(&monitor->queue, 100, -1)
    c.g[V0][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(PRCS_MONITOR_QUEUE));
    PRCS_SET_RADIUS_VANISH.call(c, 0x0029_8904)?;
    // listener[cur] = SSXAudioSystem_GetRiderListenerIndex(sys, a).
    c.g[A1][0] = c.g[S6][0];
    c.g[S0][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    c.g[A0][0] = c.g[S5][0];
    call_ptr(c, 0x0028_8a20, 0x0029_8914)?;
    c.g[A2][0] = Addr(r32(c, S0)).lw(c);
    c.g[A3][0] = sx(r32(c, S6).wrapping_add(PRCS_RIDER_SOURCE));
    c.g[A0][0] = c.g[S5][0];
    c.g[V1][0] = Addr(r32(c, A2).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[A1][0] = Addr(r32(c, A2).wrapping_add(0x23c)).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // monitor->pushSource[cur] = &a->source; Audio_PlayPushedSound(sys).
    c.g[A1][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(PRCS_MONITOR_COUNT)).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    Addr(r32(c, V1).wrapping_add(0x1fc)).sw(c, c.g[A3][0]);
    call_ptr(c, 0x0029_06b8, 0x0029_8950)?;
    // SSXAudioSystem_TrackSound(sys, 7, handle, clock + 50, 0, 0).
    c.g[V1][0] = Addr(r32(c, S5).wrapping_add(PRCS_SYS_SND_ROOT)).lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = PRCS_TRACK_KEY;
    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[A3][0] = Addr(r32(c, V0).wrapping_add(PRCS_MONITOR_CLOCK)).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x32));
    PRCS_TRACK_SOUND.call(c, 0x0029_8978)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `SSXAudioSystem_PlayRiderCollisionSound` against the generated body:
    /// views near or far from rider a (the real `FindViewInRange` /
    /// `IsAnyViewInRange`, positions and velocities from fake virtuals),
    /// either rider a local human or neither, a collision sound still
    /// tracked or not (the real ring), then the real push queue, curve
    /// table, gain lookup, `SetRadiusVanish`, `GetRiderListenerIndex`,
    /// `Audio_PlayPushedSound` (no bank: `SND_Play` fails) and `TrackSound`.
    /// Some seeds unwind at one of the fake virtual calls.
    #[test]
    fn ssx_audio_system_play_rider_collision_sound_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS, LOG_UNWIND_AT};
        use crate::difftest::{check_lohi, DATA};
        use crate::game::sound::soundsys::tests::audio;
        const OTHER: u32 = DATA + 0x24_1000;
        const POS: u32 = DATA + 0x27_0000;
        const VEL_A: u32 = DATA + 0x27_0040;
        const VEL_B: u32 = DATA + 0x27_0080;
        const RING: u32 = DATA + 0x27_1000;
        const VIEWMAN: u32 = DATA + 0x28_0000;
        const VIEWS: u32 = DATA + 0x28_0400;
        check_lohi(
            ee::s08::f_00298488__generated,
            ssx_audio_system_play_rider_collision_sound,
            150,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_8488, 0x0029_8488);
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Views: module + 0x84 -> view manager (count, views).
                w(c, audio::MODULE + 0x84, VIEWMAN);
                w(c, VIEWMAN + 0x10, 1 + r.next_u32() % 2);
                let far = seed % 6 == 4;
                for i in 0..2 {
                    let view = VIEWS + 0x40 * i;
                    w(c, VIEWMAN + 4 + 4 * i, view);
                    for k in 0..4 {
                        let v = if far {
                            2.0e5f32.to_bits()
                        } else {
                            r.f32(3000.0)
                        };
                        w(c, view + 0x20 + 4 * k, v);
                    }
                }
                // Both riders share a vtable whose position (0x28) and
                // velocity (0x10) virtuals are fakes; their results forced:
                // pos, pos, vel a, vel b, pos (GetRiderListenerIndex).
                let vt = vtable(c, &mut r, &[0x28, 0x10]);
                w(c, audio::RIDER + PRCS_RIDER_BODY, vt);
                w(c, OTHER + PRCS_RIDER_BODY, vt);
                for k in 0..4 {
                    let p = r.f32(3000.0);
                    w(c, POS + 4 * k, p);
                    let va = r.f32(60.0);
                    w(c, VEL_A + 4 * k, va);
                    let vb = r.f32(60.0);
                    w(c, VEL_B + 4 * k, vb);
                }
                for (n, v) in [POS, POS, VEL_A, VEL_B, POS].into_iter().enumerate() {
                    w(c, LOG_RESULTS + 4 * n as u32, v);
                }
                let unwind = if seed % 5 == 3 {
                    r.next_u32() % 5
                } else {
                    u32::MAX
                };
                w(c, LOG_UNWIND_AT, unwind);
                // Rider b's flags; rider a's fixed listener index.
                w(c, OTHER + PRCS_RIDER_HUMAN, (r.next_u32() % 3 != 0) as u32);
                w(c, OTHER + PRCS_RIDER_LOCAL, (r.next_u32() % 3 != 0) as u32);
                w(c, audio::RIDER + 0x870, r.next_u32() % 2);
                // Tracked-sound ring (capacity 8): empty, or a live record
                // with key 7, subkey 0 (handle 0).
                let read = r.next_u32() % 8;
                let tracked = seed % 5 == 2;
                w(c, audio::SYS + 0x597c, 8);
                w(c, audio::SYS + 0x5980, RING);
                w(c, audio::SYS + 0x5988, read);
                w(
                    c,
                    audio::SYS + 0x5984,
                    if tracked { (read + 5) % 8 } else { read },
                );
                let record = [7, 0, r.next_u32(), r.next_u32(), 0];
                for (k, v) in record.into_iter().enumerate() {
                    w(c, RING + 4 * ((read + k as u32) % 8), v);
                }
                // The monitor's clock.
                w(c, audio::MONITOR + PRCS_MONITOR_CLOCK, r.next_u32() % 10000);
                c.g[A2][0] = sx(OTHER);
            },
        );
    }
}
