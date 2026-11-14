//! Staging module oc365 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc365`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0029_48d0,
    ssx_audio_system_play_landing_sound,
    "SSXAudioSystem_PlayLandingSound",
)];

/// Rider fields `SSXAudioSystem_PlayLandingSound` reads.
pub(crate) mod landing_rider {
    /// Sound source position handed to the pushed sound (`&rider->mPosition`).
    pub const SOURCE: u32 = 0x110;
    /// The rider's controller object (embedded; vptr at +0).
    pub const CONTROLLER: u32 = 0x6c0;
    /// Landing speed (`float`), the input of the volume curve.
    pub const SPEED: u32 = 0x770;
    /// View / listener index, is-human, is-local.
    pub const VIEW: u32 = 0x870;
    pub const HUMAN: u32 = 0x874;
    pub const LOCAL: u32 = 0x87c;
    /// Controller vtable slots: is-computer and position.
    pub const VT_IS_COMPUTER: u32 = 0x48;
    pub const VT_POSITION: u32 = 0x28;
}

/// The landing volume curve (five keys, five values) at `0x482d30`.
#[cfg(test)]
pub(crate) const LANDING_VOLUME_CURVE: u32 = 0x0048_2d30;

/// The system's landing-sound play handle (`sys + 0x5fb8`).
pub(crate) const LANDING_HANDLE: u32 = 0x5fb8;

/// `lw base+off` on whole registers.
#[inline(always)]
fn pls_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `void SSXAudioSystem_PlayLandingSound(SSXAudioSystem* sys, Rider* rider,
/// float speed)` (0x2948d0): the board-landing sound. First the crowd
/// reacts when this is the rider the crowd follows (`FindRider` on the
/// rider's view, or view 0 in replays): a stop for speed <= 0, else a
/// cheer of size 0/1/2 (speed < 0.25 / < 0.65 / above). Then, unless the
/// controller is human-driven and the rider local, the rider must be within
/// 100 units of a view. The volume comes from a five-key curve over the
/// rider's landing speed (clamped to 0..127), the sample from the surface
/// (`GetSurfaceSoundType * 8 + 1` local, `+ 5` remote). The previous landing
/// sound is faded out unless its channel is gone; a new entry goes on the
/// monitor's push queue (bank 10, volume 100, pan 90, 50, 127; gain channel
/// 6; radius 30 / vanish -1; the rider as source and listener; for a human
/// local rider in mode 2 outside replays, the replay listener), and its
/// play handle (`Audio_PlayPushedSound`) is kept at `sys + 0x5fb8`.
pub fn ssx_audio_system_play_landing_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S4, 0x80);
        frame.save(c, S0, 0xc0);
        c.g[S4][0] = c.g[A0][0]; // sys
        frame.save(c, S5, 0x70);
        c.g[S0][0] = u64::MAX; // the crowd's view index
        frame.save_fpr(c, 20, 0xd0);
        c.g[S5][0] = c.g[A1][0]; // rider
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.mov_s(20, 12); // speed

        pls_crowd(c)?;
        if pls_in_range(c)? {
            pls_volume_and_priority(c)?;
            pls_fade_previous(c)?;
            pls_replay_check(c)?;
            pls_push_entry(c)?;
            pls_finish_entry(c)?;
            // sys->landingHandle = Audio_PlayPushedSound(sys)
            c.cycles += 3;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0029_06b8, 0x0029_4f10)?;
            Addr(r32(c, S4).wrapping_add(LANDING_HANDLE)).sw(c, c.g[V0][0]);
        }

        c.cycles += 13;
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
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// The crowd reaction: view index (s0) 0 in replays, else the rider's view
/// for a human local rider (else -1); when `FindRider(sys, view)` is this
/// rider, stop (speed <= 0) or cheer 0/1/2. Leaves v1 = the controller's
/// vtable.
fn pls_crowd(c: &mut Cpu) -> crate::call::Flow {
    use landing_rider::*;
    call_ptr(c, 0x0028_8ae0, 0x0029_4914)?; // SSXAudioSystem_IsReplayPlaying
    if c.g[V0][0] == 0 {
        c.g[V0][0] = pls_lw(c, c.g[S5][0], HUMAN);
        c.cycles += 2;
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = pls_lw(c, c.g[S5][0], LOCAL);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.g[S0][0] = pls_lw(c, c.g[S5][0], VIEW);
        }
    } else {
        c.cycles += 2;
        c.g[S0][0] = 0;
    }

    // SSXAudioSystem_FindRider(sys, view) == rider?
    c.cycles += 5;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x0028_5d98, 0x0029_4948)?;
    if c.g[V0][0] == c.g[S5][0] {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_le_s(20, 0);
        c.g[A1][0] = c.g[S0][0];
        if c.fcc() {
            // speed <= 0: CrowdAudio stop (sys, view)
            c.cycles += 4;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x002a_7438, 0x0029_496c)?;
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3e80_0000; // 0.25
            c.mtc1(AT, 0);
            c.c_lt_s(20, 0);
            c.g[A2][0] = c.g[S0][0];
            let (size, ret_at) = if c.fcc() {
                c.cycles += 5;
                (0, 0x0029_4998)
            } else {
                c.cycles += 5;
                let lit = (c.g[GP][0] as u32).wrapping_sub(0x45a4); // 0.65
                c.lwc1(0, lit);
                c.c_lt_s(20, 0);
                c.g[A2][0] = c.g[S0][0];
                if c.fcc() {
                    c.cycles += 5;
                    (1, 0x0029_49c0)
                } else {
                    c.cycles += 4;
                    (2, 0x0029_49d4)
                }
            };
            // CrowdAudio cheer (sys, size, view)
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = size;
            call_ptr(c, 0x002a_73a8, ret_at)?;
        }
    }
    c.g[V1][0] = pls_lw(c, c.g[S5][0], CONTROLLER);
    Ok(())
}

/// The range gate: a computer-driven or remote rider must be within 100
/// units of a view (`FindViewInRange`). Leaves v0 = 0x480000.
fn pls_in_range(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use landing_rider::*;
    c.cycles += 7;
    c.g[S0][0] = sx(r32(c, S5).wrapping_add(CONTROLLER));
    let vt = c.g[V1][0];
    let m = VSlot::read(c, vt, VT_IS_COMPUTER);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(c.g[S0][0]);
    call_ptr(c, m.func as u32, 0x0029_49ec)?;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = pls_lw(c, c.g[S5][0], CONTROLLER);
    } else {
        c.cycles += 3;
        let local = pls_lw(c, c.g[S5][0], LOCAL) != 0;
        c.g[V0][0] = 0x48_0000;
        if local {
            return Ok(true);
        }
        c.cycles += 1;
        c.g[V1][0] = pls_lw(c, c.g[S5][0], CONTROLLER);
    }
    // FindViewInRange(sys, controller->position(), 100.0) >= 0
    c.cycles += 11;
    let vt = c.g[V1][0];
    let m = VSlot::read(c, vt, VT_POSITION);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(c.g[S0][0]);
    call_ptr(c, m.func as u32, 0x0029_4a14)?;
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 12);
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, 0x0028_8b40, 0x0029_4a28)?;
    let out = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = 0x48_0000;
    Ok(!out)
}

/// Volume (s6) from the landing-speed curve, clamped to 0..127; priority /
/// sample (s7) from the surface: `type * 8 + 1` for a human local rider,
/// `+ 5` otherwise.
fn pls_volume_and_priority(c: &mut Cpu) -> crate::call::Flow {
    use landing_rider::*;
    c.cycles += 35;
    let speed_at = r32(c, S5).wrapping_add(SPEED);
    c.lwc1(12, speed_at);
    // The curve, by value on the stack (five doublewords).
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d30));
    let src = c.g[A3][0] as u32;
    let dst = sp(c);
    for (i, r) in [3usize, 5, 6, 3, 5].into_iter().enumerate() {
        let off = 8 * i as u32;
        c.ldl(r, src.wrapping_add(off + 7));
        c.ldr(r, src.wrapping_add(off));
        c.sdl(r, dst.wrapping_add(off + 7));
        c.sdr(r, dst.wrapping_add(off));
    }
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0029_0b58, 0x0029_4a90)?; // Audio_EvalCurve5
    c.cvt_w_s(1, 0);
    c.mfc1(S6, 1);
    c.g[A0][0] = 0x7f;
    c.g[V1][0] = u64::MAX;
    c.g[A1][0] = pls_lw(c, c.g[S5][0], HUMAN);
    c.g[A2][0] = 0;
    c.g[V0][0] = ((c.g[S6][0] as i64) < 128) as u64;
    if c.g[V0][0] == 0 {
        c.g[S6][0] = c.g[A0][0];
    }
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S6][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        c.g[S6][0] = 0;
    }
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = pls_lw(c, c.g[S5][0], LOCAL);
        c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
    }
    // SSXAudioSystem_GetSurfaceSoundType(sys, rider)
    c.cycles += 2;
    let local = c.g[A2][0] != 0;
    c.g[A0][0] = c.g[S4][0];
    let (ret_at, add) = if local {
        c.cycles += 5;
        (0x0029_4ad4, 1)
    } else {
        c.cycles += 4;
        (0x0029_4ae8, 5)
    };
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, 0x0029_1710, ret_at)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(add));
    Ok(())
}

/// Fade out the previous landing sound when its channel is still there
/// (`SND_HandleToChannel` under the SND lock). Leaves v0 = rider->human.
fn pls_fade_previous(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    call_ptr(c, 0x003b_58a0, 0x0029_4af8)?; // SND_Lock
    c.g[A0][0] = pls_lw(c, c.g[S4][0], LANDING_HANDLE);
    call_ptr(c, 0x003b_8ab8, 0x0029_4b00)?; // handle gone?
    c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
    call_ptr(c, 0x003b_58d8, 0x0029_4b08)?; // SND_Unlock
    if c.g[S0][0] == 0 {
        // BankMonitor fade (root monitor's queue, handle, 1, 0.0)
        c.cycles += 8;
        c.g[V0][0] = pls_lw(c, c.g[S4][0], 0x118);
        c.g[A2][0] = 1;
        c.g[A1][0] = pls_lw(c, c.g[S4][0], LANDING_HANDLE);
        c.g[A0][0] = pls_lw(c, c.g[V0][0], 0);
        c.mtc1(0, 12);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_d5f0, 0x0029_4b2c)?;
    }
    c.g[V0][0] = pls_lw(c, c.g[S5][0], landing_rider::HUMAN);
    Ok(())
}

/// fp = human local rider in mode 2 and no replay playing; s0 = monitor.
fn pls_replay_check(c: &mut Cpu) -> crate::call::Flow {
    use landing_rider::*;
    c.cycles += 3;
    c.g[FP][0] = 0;
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = pls_lw(c, c.g[S5][0], LOCAL);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 10;
        call_ptr(c, 0x0014_dc80, 0x0029_4b54)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_4b60)?; // GetInterface(be, 0)
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = Addr(0x0053_5c11).lb(c);
        if c.g[V0][0] == 2 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0028_8ae0, 0x0029_4b7c)?; // IsReplayPlaying
            c.g[FP][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.g[S0][0] = pls_lw(c, c.g[S4][0], 0);
    Ok(())
}

/// A new push-queue entry on the monitor (s0; s1 = its queue): count += 1,
/// copy the record template, clear the entry, the default parameters, bank
/// 10 (s3) and the sample (s7) on the system's push tables, the volume (s6)
/// on the root monitor. Leaves a0 = the record's volume byte address and
/// v0 = the clamped volume.
fn pls_push_entry(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::sound::soundsys::boost_empty_store_entry as store;
    c.cycles += 154;
    let monitor = c.g[S0][0] as u32;
    c.g[S2][0] = 0x18; // record size
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[V0][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[S1][0] = sx(monitor.wrapping_add(0x1d8));
    let queue = c.g[S1][0];
    c.g[S3][0] = 0xa; // bank
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor.wrapping_add(0x1f4)).sw(c, c.g[V0][0]);
    c.g[V1][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[V0][0] = pls_lw(c, queue, 0x20);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // Record template (monitor + 0x1dc, 24 bytes, unaligned).
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
    // monitor->source[count] = 0 (inline array).
    c.g[V0][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x1fc).sw(c, 0);
    store(c, (V1, V0), 0x34, 2, 0, 0);
    // volume[count] = the record's first byte (sign-extended).
    c.g[A3][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[A0][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[T0][0] = pls_lw(c, queue, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = pls_lw(c, queue, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    store(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count], 0, 8)
    c.g[A0][0] = pls_lw(c, c.g[S0][0], 0x1f4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((queue as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0029_4c58)?;

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
    store(c, (V1, V0), 0x70, 4, 0x4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    store(c, (V0, V1), 0x70, 4, 0x0, a0);
    store(c, (V1, V0), 0x70, 4, 0x8, a1);
    store(c, (V0, V1), 0x70, 4, 0xc, a2);
    store(c, (V1, V0), 0x74, 2, 0, 0);
    store(c, (V0, V1), 0x80, 2, 0, 0);
    store(c, (V1, V0), 0x84, 2, 0, 0);
    store(c, (V0, V1), 0x78, 2, 0, a3);
    store(c, (V1, V0), 0x7c, 2, 0, t0);

    // sys->pushBank[count] = 10; sys->pushSample[count] = s7.
    let sys = c.g[S4][0];
    c.g[A0][0] = pls_lw(c, sys, 0);
    c.g[V1][0] = pls_lw(c, sys, 0x4);
    c.g[V0][0] = pls_lw(c, c.g[A0][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S3][0]);
    c.g[V1][0] = pls_lw(c, sys, 0);
    c.g[A0][0] = pls_lw(c, sys, 0x8);
    c.g[V0][0] = pls_lw(c, c.g[V1][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S7][0]);

    // Root monitor: volume[count] = s6, then the record's volume byte =
    // clamp(volume[count], 0, 127).
    c.g[V1][0] = pls_lw(c, sys, 0x118);
    c.g[A1][0] = pls_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = pls_lw(c, c.g[A1][0], 0x1f4);
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
    c.g[V1][0] = pls_lw(c, c.g[A2][0], 0x38);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S6][0]);
    c.g[A0][0] = pls_lw(c, c.g[A1][0], 0x1f4);
    c.g[V1][0] = pls_lw(c, c.g[A2][0], 0x1c);
    c.g[A3][0] = pls_lw(c, c.g[A2][0], 0x38);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = pls_lw(c, c.g[A2][0], 0x20);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = pls_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    // The current volume word: queue->volume[queue->count].
    let vol_at = |c: &mut Cpu| -> u64 {
        let n = pls_lw(c, c.g[A2][0], 0x1c);
        sx(((n as u32) << 2).wrapping_add(c.g[A3][0] as u32))
    };
    let mut clamp_top = true;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pls_lw(c, c.g[V0][0], 0);
        c.g[V0][0] = 0;
        clamp_top = (c.g[V1][0] as i64) >= 0;
    }
    if clamp_top {
        c.cycles += 7;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pls_lw(c, c.g[V0][0], 0);
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

/// Store the volume byte, then the entry's gain (channel 6), radius 30 /
/// vanish -1, source, listener and (fp) the replay flag and listener.
fn pls_finish_entry(c: &mut Cpu) -> crate::call::Flow {
    use landing_rider::*;
    c.cycles += 38;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
    // monitor->gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 6, 0)
    c.g[A1][0] = 0x6;
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S4][0];
    c.g[S0][0] = pls_lw(c, c.g[S4][0], 0x118);
    call_ptr(c, 0x0028_7968, 0x0029_4e48)?;
    c.g[A0][0] = pls_lw(c, c.g[S0][0], 0);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 13);
    c.g[V1][0] = pls_lw(c, c.g[A0][0], 0x1f4);
    c.g[A1][0] = pls_lw(c, c.g[A0][0], 0x214);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 12);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // SetRadiusVanish(&root->queue, 30.0, -1.0)
    c.g[V0][0] = pls_lw(c, c.g[S4][0], 0x118);
    c.g[A0][0] = pls_lw(c, c.g[V0][0], 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    call_ptr(c, 0x002a_9988, 0x0029_4e80)?;
    // monitor->source[count] = &rider->position (inline array).
    c.g[A2][0] = pls_lw(c, c.g[S4][0], 0x118);
    c.g[A3][0] = sx(r32(c, S5).wrapping_add(SOURCE));
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[V1][0] = pls_lw(c, c.g[A2][0], 0);
    c.g[V0][0] = pls_lw(c, c.g[V1][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V1][0] as u32).off(0x1fc).sw(c, c.g[A3][0]);
    // monitor->listener[count] = SSXAudioSystem_GetRiderListenerIndex(sys, rider)
    c.g[S0][0] = pls_lw(c, c.g[S4][0], 0x118);
    call_ptr(c, 0x0028_8a20, 0x0029_4eac)?;
    c.g[A0][0] = pls_lw(c, c.g[S0][0], 0);
    c.g[V1][0] = pls_lw(c, c.g[A0][0], 0x1f4);
    c.g[A1][0] = pls_lw(c, c.g[A0][0], 0x23c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    if c.g[FP][0] != 0 {
        // Live play of the local human: flag and listener for the replay.
        c.cycles += 16;
        for (table, listener) in [(0x258u32, false), (0x25c, true)] {
            c.g[A0][0] = pls_lw(c, c.g[S4][0], 0x118);
            c.g[A1][0] = if listener {
                pls_lw(c, c.g[S5][0], VIEW)
            } else {
                1
            };
            c.g[V1][0] = pls_lw(c, c.g[A0][0], 0);
            c.g[V0][0] = pls_lw(c, c.g[V1][0], 0x1f4);
            c.g[A0][0] = pls_lw(c, c.g[V1][0], table);
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

    /// `SSXAudioSystem_PlayLandingSound` on the shared audio world of the
    /// soundsys tests (real IsReplayPlaying, FindRider over a two-rider race
    /// manager, the crowd calls with no crowd, FindViewInRange, the curve,
    /// GetSurfaceSoundType, SND_HandleToChannel, the fade's instance search,
    /// memset, gain lookup, SetRadiusVanish, listener lookup and
    /// `Audio_PlayPushedSound` with no bank). Covers: in / out of replays,
    /// human / local or not, FindRider giving this rider or the other one,
    /// landing speeds <= 0, < 0.25, < 0.65 and above (and NaN-like edges);
    /// the controller flag from a fake (non-zero) or the real zero leaf, no
    /// view / a far view (early out) or a view at the rider; the game's
    /// volume curve or one spanning negative to >127 volumes; the previous
    /// handle negative, on a live channel or on a stale one; mode byte 2 or
    /// not. Some seeds unwind at a fake call.
    #[test]
    fn ssx_audio_system_play_landing_sound_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::{vtable, LOG_RESULTS, LOG_UNWIND_AT};
        use crate::difftest::{put_vec, DATA};
        use crate::game::sound::soundsys::tests::audio;
        use landing_rider::*;
        const OC365_PLS_VIEWS: u32 = DATA + 0x28_0000;
        const OC365_PLS_VIEW: u32 = DATA + 0x28_1000;
        const OC365_PLS_POS: u32 = DATA + 0x28_2000;
        const OC365_PLS_RACE_MAN: u32 = DATA + 0x28_3000;
        const OC365_PLS_OTHER: u32 = DATA + 0x28_4000;
        const OC365_PLS_MAIN_STATE: u32 = DATA + 0x28_6000;
        const OC365_PLS_REPLAY_RIDER: u32 = DATA + 0x28_7000;
        const OC365_PLS_CHANNELS: u32 = DATA + 0x28_8000;
        const OC365_PLS_INSTANCES: u32 = DATA + 0x2b_0000;
        /// The SND system block (`0x510000 - 0x5718`).
        const OC365_PLS_SND: u32 = 0x0050_a8e8;
        const OC365_PLS_RETURNS_ZERO: u32 = 0x0014_06c0;
        check_lohi(
            ee::s07::f_002948d0__generated,
            ssx_audio_system_play_landing_sound,
            200,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_48d0, 0x0029_48d0);
                let sys = audio::SYS;
                let rider = audio::RIDER;
                // Always in game (FindRider reads the race manager).
                c.bus.write32(audio::GAME + 0x84, audio::MODULE);
                // Race manager: two riders (this one and another), in either
                // order; the replay's followed rider.
                let (first, second) = if seed % 3 == 1 {
                    (OC365_PLS_OTHER, rider)
                } else {
                    (rider, OC365_PLS_OTHER)
                };
                c.bus.write32(audio::MODULE + 0xc, OC365_PLS_RACE_MAN);
                c.bus.write32(OC365_PLS_RACE_MAN + 0x28, first);
                c.bus.write32(OC365_PLS_RACE_MAN + 0x2c, second);
                c.bus.write32(OC365_PLS_RACE_MAN + 0x78, 2);
                c.bus.write32(OC365_PLS_RACE_MAN + 0x7c, 2);
                c.bus.write32(
                    OC365_PLS_RACE_MAN + 0x40,
                    if seed % 11 == 3 {
                        0
                    } else {
                        OC365_PLS_REPLAY_RIDER
                    },
                );
                c.bus.write32(
                    OC365_PLS_REPLAY_RIDER + 0x18,
                    if seed % 4 == 2 {
                        OC365_PLS_OTHER
                    } else {
                        rider
                    },
                );
                let view = r.next_u32() % 3;
                c.bus.write32(rider + VIEW, view);
                c.bus.write32(rider + 0xec, r.next_u32() % 0x9000);
                c.bus.write32(OC365_PLS_OTHER + VIEW, r.next_u32() % 3);
                c.bus.write32(OC365_PLS_OTHER + HUMAN, r.next_u32() % 2);
                c.bus.write32(OC365_PLS_OTHER + LOCAL, r.next_u32() % 2);
                c.bus.write32(OC365_PLS_OTHER + 0xec, r.next_u32() % 0x9000);
                // No crowd: the crowd calls return at once.
                c.bus.write32(sys + 0x18, 0);
                // Landing speed (f12).
                c.f[12] = match seed % 8 {
                    0 => (-1.0f32).to_bits(),
                    1 => 0.0f32.to_bits(),
                    2 => 0.1f32.to_bits(),
                    3 => 0.3f32.to_bits(),
                    4 => 0.7f32.to_bits(),
                    5 => 2.5f32.to_bits(),
                    6 => r.edge_f32(),
                    _ => r.f32(1.0),
                };
                // Controller vtable: position (0x28) and is-computer (0x48).
                let vt = vtable(c, &mut r, &[VT_POSITION, VT_IS_COMPUTER]);
                let flag_is_zero = seed % 4 == 3;
                if flag_is_zero {
                    c.bus
                        .write32(vt + VT_IS_COMPUTER + 4, OC365_PLS_RETURNS_ZERO);
                }
                c.bus.write32(rider + CONTROLLER, vt);
                c.bus.write32(LOG_RESULTS, OC365_PLS_POS);
                c.bus.write32(LOG_RESULTS + 4, OC365_PLS_POS);
                // Views: one, at the rider or far away; none sometimes.
                let count = if !flag_is_zero && seed % 9 == 4 { 0 } else { 1 };
                c.bus.write32(audio::MODULE + 0x84, OC365_PLS_VIEWS);
                c.bus.write32(OC365_PLS_VIEWS + 0x10, count);
                c.bus.write32(OC365_PLS_VIEWS + 4, OC365_PLS_VIEW);
                let pos = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 1.0f32.to_bits()];
                put_vec(c, OC365_PLS_POS, pos);
                let view_pos = if seed % 8 == 5 {
                    [1.0e4f32.to_bits(), pos[1], pos[2], pos[3]]
                } else {
                    pos
                };
                put_vec(c, OC365_PLS_VIEW + 0x20, view_pos);
                // Volume curve: the game's, or keys 0..40 with values -50..300.
                if seed % 3 != 0 {
                    for (i, v) in [
                        0.0f32, 10.0, 20.0, 30.0, 40.0, -50.0, 0.0, 100.0, 200.0, 300.0,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        c.bus
                            .write32(LANDING_VOLUME_CURVE + 4 * i as u32, v.to_bits());
                    }
                }
                let speed = match seed % 6 {
                    0 => r.edge_f32(),
                    _ => r.f32(45.0),
                };
                c.bus.write32(rider + SPEED, speed);
                // GetSurfaceSoundType: main state 4 (the system's surface)
                // or another (the rider's), surfaces -1..20.
                c.bus.write32(rider + 0x77c, OC365_PLS_MAIN_STATE);
                c.bus.write32(
                    OC365_PLS_MAIN_STATE + 0xde0,
                    if seed % 5 == 0 { 4 } else { r.next_u32() % 6 },
                );
                c.bus
                    .write32(rider + 0x438, (r.next_u32() % 22).wrapping_sub(1));
                c.bus
                    .write32(sys + 0x598c, (r.next_u32() % 22).wrapping_sub(1));
                // The previous handle: negative, on a live channel, stale.
                let handle = match seed % 5 {
                    0 => 0x8000_0000 | r.next_u32(),
                    1 | 3 => r.next_u32() % 4,
                    _ => (r.next_u32() & 0x7fff_ff00) | (r.next_u32() % 4),
                };
                c.bus.write32(sys + LANDING_HANDLE, handle);
                c.bus.write16(OC365_PLS_SND + 0x17e, 4);
                c.bus.write32(OC365_PLS_SND + 0x1f0, OC365_PLS_CHANNELS);
                for k in 0..4u32 {
                    let ch = OC365_PLS_CHANNELS + k * 0x8c;
                    c.bus.write32(ch, k);
                    c.bus.write8(ch + 0x69, (seed % 7 != 6) as u8);
                }
                // The fade's instance search: all instances free.
                c.bus
                    .write32(audio::MONITOR + 0x1d8 + 0x8ec, OC365_PLS_INSTANCES);
                c.bus.write8(0x0053_5c11, if seed % 5 < 3 { 2 } else { 1 });
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 4);
                }
            },
        );
    }
}
