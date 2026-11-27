//! Staging module oc1034 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1034`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_1560,
        ssx_audio_system_rider_say_20b8,
        "SSXAudioSystem_RiderSay20B8",
    ),
    (
        0x002a_19d8,
        ssx_audio_system_trigger_speech_event,
        "SSXAudioSystem_TriggerSpeechEvent",
    ),
    (
        0x002a_1bd0,
        ssx_audio_system_return_true_1bd0,
        "SSXAudioSystem_ReturnTrue_1bd0",
    ),
    (
        0x002a_1e68,
        ssx_audio_system_get_finish_speech_mask,
        "SSXAudioSystem_GetFinishSpeechMask",
    ),
    (
        0x002a_2a10,
        ssx_audio_system_dj_say_210b,
        "SSXAudioSystem_DJSay210B",
    ),
    (
        0x002a_2c30,
        ssx_audio_system_dj_say_212d,
        "SSXAudioSystem_DJSay212D",
    ),
    (
        0x002a_3400,
        ssx_audio_system_pa_say_20c3,
        "SSXAudioSystem_PASay20C3",
    ),
    (
        0x002a_3860,
        ssx_audio_system_pa_say_20c7,
        "SSXAudioSystem_PASay20C7",
    ),
];

/// The 32-bit address `base + off` as the generated code computes it.
fn say_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_RiderSay20B8 (0x2a1560).

/// `void SSXAudioSystem_RiderSay20B8(SSXAudioSystem* self, Rider* rider)`
/// (0x2a1560): a rider's speech line. Nothing plays while the speech volume is
/// off or during a replay. A computer or remote rider speaks only as the focus
/// rider. The line is registered with the speech system (`self + 0x5560`) when
/// the rider is in the camera's range, then the rider's speaker mask goes to
/// the sound function at `gp + 0x173c`.
pub fn ssx_audio_system_rider_say_20b8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        rider_say_20b8_body(c)?;
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

/// The body between the prologue and the epilogue. Every early exit of the
/// generated code is a plain `return Ok(())` here (the caller restores S0).
fn rider_say_20b8_body(c: &mut Cpu) -> Flow {
    call_ptr(c, 0x0029_f0a0, 0x002a_1584)?; // SSXAudioSystem_IsSpeechVolumeOn(self)
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0028_8ae0, 0x002a_1594)?; // SSXAudioSystem_IsReplayPlaying(self)
    if c.g[V0][0] != 0 {
        return Ok(());
    }

    // The rider's sub-object at +0x6c0: `rider->isComputer()` (vtable +0x48).
    c.cycles += 10;
    c.g[V1][0] = Addr(say_ea(c.g[S0][0], 0x6c0)).lw(c);
    c.g[A1][0] = sx(say_ea(c.g[S0][0], 0x6c0));
    c.g[S3][0] = c.g[A1][0];
    c.g[S2][0] = 0;
    c.g[A0][0] = Addr(say_ea(c.g[V1][0], 0x48)).lh(c);
    c.g[V0][0] = Addr(say_ea(c.g[V1][0], 0x4c)).lw(c);
    let is_computer = c.g[V0][0] as u32;
    c.g[A0][0] = sx(say_ea(c.g[A1][0], c.g[A0][0] as u32));
    call_ptr(c, is_computer, 0x002a_15bc)?;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(say_ea(c.g[S0][0], 0x87c)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[S2][0] = 1;
        }
    }

    // Focus check: the rider must be the focus rider when `S2` (check) is set.
    c.cycles += 2;
    c.g[A0][0] = c.g[S1][0];
    if c.g[S2][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = u64::MAX;
        call_ptr(c, 0x0028_5d98, 0x002a_15e0)?; // SSXAudioSystem_FindRider(self, -1)
        if c.g[V0][0] != c.g[S0][0] {
            return Ok(());
        }
    }

    // The speech system must be ready.
    c.cycles += 6;
    c.g[S2][0] = sx(say_ea(c.g[S1][0], 0x5560));
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x002b_0e28, 0x002a_15f8)?; // SpeechSystem_IsInstanceReady(bm, 0)
    if c.g[V0][0] == 0 {
        return Ok(());
    }

    // The rider's position (vtable +0x28) and the camera range check.
    c.cycles += 11;
    c.g[V1][0] = Addr(say_ea(c.g[S0][0], 0x6c0)).lw(c);
    c.g[A0][0] = Addr(say_ea(c.g[V1][0], 0x28)).lh(c);
    c.g[V0][0] = Addr(say_ea(c.g[V1][0], 0x2c)).lw(c);
    let get_position = c.g[V0][0] as u32;
    c.g[A0][0] = sx(say_ea(c.g[S3][0], c.g[A0][0] as u32));
    call_ptr(c, get_position, 0x002a_1614)?;
    c.lwc1(12, say_ea(c.g[GP][0], 0x5e8));
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0028_8b40, 0x002a_1624)?; // SSXAudioSystem_CameraInRange(self, pos)
    if (c.g[V0][0] as i64) < 0 {
        return Ok(());
    }

    // Register the line with the speech system.
    c.cycles += 13;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0012_3128, 0x002a_1634)?; // cRider_getIdentity(rider)
    c.mtc1(0, 12);
    let identity = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[T0][0] = identity;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20b8;
    c.g[A3][0] = c.g[S0][0];
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    call_ptr(c, 0x002b_1458, 0x002a_1658)?; // SpeechSystem_SetEventContext(...)
    if c.g[V0][0] == 0 {
        return Ok(());
    }

    // Speaker mask, then the sound call.
    c.cycles += 13;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0029_f330, 0x002a_1668)?; // SSXAudioSystem_GetRiderSpeechMask(rider)
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20b8;
    call_ptr(c, 0x003d_8008, 0x002a_167c)?;
    c.g[V1][0] = Addr(say_ea(c.g[GP][0], 0x173c)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S0][0];
    let sound = c.g[V1][0] as u32;
    c.g[A1][0] = 1;
    call_ptr(c, sound, 0x002a_1690)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_TriggerSpeechEvent (0x2a19d8).

/// `int SSXAudioSystem_TriggerSpeechEvent(SSXAudioSystem* self, int ev)`
/// (0x2a19d8): plays the speech event `ev`. Event 0 is the race intro (and the
/// music monitor once); the other events each start their own line. Returns 1
/// when an event played, 0 for an unknown event.
pub fn ssx_audio_system_trigger_speech_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        trigger_speech_event_body(c)?;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One `ev == k` arm of the switch. The compare is against V0, which the
/// previous arm left holding its own constant (`next` is what this arm leaves).
/// On a match the arm makes its call and returns 1 in V0.
fn trigger_speech_arm(
    c: &mut Cpu,
    compare_cycles: u64,
    next: u64,
    call_cycles: u64,
    target: u32,
    ret_addr: u32,
    a1: Option<u64>,
) -> Result<bool, Unwound> {
    c.cycles += compare_cycles;
    let miss = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = next;
    if miss {
        return Ok(false);
    }
    c.cycles += call_cycles;
    c.g[A0][0] = c.g[S0][0];
    if let Some(v) = a1 {
        c.g[A1][0] = v;
    }
    call_ptr(c, target, ret_addr)?;
    c.g[V0][0] = 1;
    Ok(true)
}

fn trigger_speech_event_body(c: &mut Cpu) -> Flow {
    if c.g[A1][0] == 0 {
        // Event 0: the race intro, then the music monitor the first time.
        c.cycles += 5;
        call_ptr(c, 0x002a_31c0, 0x002a_19f4)?; // SSXAudioSystem_PASayRaceIntro(self)
        c.g[V0][0] = Addr(say_ea(c.g[S0][0], 0x530)).lw(c);
        let playing = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if playing {
            return Ok(());
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d488, 0x002a_1a08)?;
        c.g[V0][0] = Addr(say_ea(c.g[S0][0], 0x118)).lw(c);
        c.g[V1][0] = 0x004a_0000;
        c.ldl(6, say_ea(c.g[V1][0], 0x364f));
        c.ldr(6, say_ea(c.g[V1][0], 0x3648));
        c.g[A1][0] = 0xa;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 3;
        c.g[T1][0] = 1;
        c.g[A0][0] = sx(say_ea(c.g[A0][0], 0x1d8));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x002a_dca0, 0x002a_1a3c)?; // cMonitor_MONITOR_Create (bank monitor timer)
        c.g[V0][0] = 1;
        return Ok(());
    }

    // Events 1..15: the arms compare A1 with the constant the previous arm
    // left in V0; the first arm starts from 1.
    c.g[V0][0] = 1;
    if trigger_speech_arm(c, 3, 2, 4, 0x002a_32b0, 0x002a_1a58, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 3, 4, 0x002a_3358, 0x002a_1a70, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 7, 4, 0x002a_3400, 0x002a_1a88, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 8, 4, 0x002a_3860, 0x002a_1aa0, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 0xa, 4, 0x002a_2018, 0x002a_1ab8, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 0xc, 4, 0x002a_2188, 0x002a_1ad0, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 0xe, 5, 0x002a_2568, 0x002a_1aec, Some(u64::MAX))? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 0xf, 4, 0x002a_2638, 0x002a_1b04, None)? {
        return Ok(());
    }
    if trigger_speech_arm(c, 2, 5, 4, 0x002a_39e0, 0x002a_1b1c, None)? {
        return Ok(());
    }

    // Event 5: the rider 0 line, with the audio interface's rider list.
    c.cycles += 2;
    let miss = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if miss {
        return Ok(());
    }
    c.cycles += 6;
    call_ptr(c, 0x0028_b1d8, 0x002a_1b34)?; // cAudioInterface_getAI()
    c.g[A1][0] = Addr(say_ea(c.g[V0][0], 0x28)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002a_3708, 0x002a_1b40)?;
    c.g[V0][0] = 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_ReturnTrue_1bd0 (0x2a1bd0).

/// `int SSXAudioSystem_ReturnTrue_1bd0(void)` (0x2a1bd0): always 1. A stub the
/// speech tables point at.
pub fn ssx_audio_system_return_true_1bd0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetFinishSpeechMask (0x2a1e68).

/// `int SSXAudioSystem_GetFinishSpeechMask(SSXAudioSystem* self, Rider* obj)`
/// (0x2a1e68): the speech "finish" bit for a rider's race place. In a
/// multiplayer race (`cGamePath == 2`) the bit follows `obj->RaceRanking`
/// (with the track-group-0 variant); otherwise the single-player ranking table
/// at 0x483070 picks it, indexed by the ranking at `0x536730`.
pub fn ssx_audio_system_get_finish_speech_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        if let Some(a0) = finish_speech_mask_body(c)? {
            c.cycles += 7;
            c.g[A0][0] = a0;
            c.g[V0][0] = a0;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
        }
        Ok(())
    })
}

/// The body: `Some(A0)` for the epilogue, `None` when the single-player table
/// holds an unknown label (the generated code jumps out and leaves the frame).
fn finish_speech_mask_body(c: &mut Cpu) -> Result<Option<u64>, Unwound> {
    call_ptr(c, 0x0014_dc80, 0x002a_1e88)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x002a_1e94)?; // cBE_getInterface(be, 0)
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = 2;
    c.g[S0][0] = Addr(say_ea(c.g[V1][0], 0x5c11)).lb(c); // cGamePath
    let not_multiplayer = c.g[S0][0] != c.g[V0][0];
    c.g[V0][0] = 0x0053_0000;
    if not_multiplayer {
        finish_single_player(c)
    } else {
        finish_multiplayer(c)
    }
}

/// The single-player arm: the ranking at 0x536730 indexes a jump table in the
/// image (0x483070); each label sets its own mask.
fn finish_single_player(c: &mut Cpu) -> Result<Option<u64>, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = Addr(0x0053_6730).lw(c);
    c.g[V1][0] = (c.g[A0][0] < 0xau64) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x0048_0000;
    if !in_table {
        c.cycles += 1;
        c.g[A0][0] = 0;
        return Ok(Some(0));
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3070));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    let value = match target {
        // The multiplayer arm's labels also sit in the table (indices 0..2).
        0x002a_1ec0 => {
            c.cycles += 2;
            1
        }
        0x002a_1ed0 => {
            c.cycles += 2;
            2
        }
        0x002a_1ee0 => {
            c.cycles += 2;
            4
        }
        0x002a_1f38 => {
            c.cycles += 2;
            8
        }
        0x002a_1f40 => {
            c.cycles += 2;
            0x10
        }
        0x002a_1f48 => {
            c.cycles += 5;
            c.g[V1][0] = 0x200;
            c.g[V0][0] = 1;
            0x200
        }
        0x002a_1f5c => {
            c.cycles += 2;
            0x40
        }
        0x002a_1f64 => {
            c.cycles += 2;
            0x80
        }
        0x002a_1f6c => {
            c.cycles += 2;
            0x100
        }
        0x002a_1f74 => {
            c.cycles += 2;
            0x200
        }
        0x002a_1f7c => {
            c.cycles += 1;
            0
        }
        _ => {
            c.jump_out(target, 0x002a_1e68, 0x002a_1f9c);
            return Ok(None);
        }
    };
    c.g[A0][0] = value;
    Ok(Some(value))
}

/// The shared tail of the multiplayer arm (0x2a1f08): `A0 = V0` when V1 is 0.
fn finish_multiplayer_tail(c: &mut Cpu) -> u64 {
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
    c.g[A0][0]
}

/// The multiplayer arm: the rider's `RaceRanking` (`obj + 0xec`) picks the bit.
fn finish_multiplayer(c: &mut Cpu) -> Result<Option<u64>, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x002a_4078, 0x002a_1eb0)?; // SSXAudioSystem_IsTrackGroup0Event(self)
    let not_event = c.g[V0][0] == 0;
    c.g[V1][0] = Addr(say_ea(c.g[S1][0], 0xec)).lw(c);
    if not_event {
        c.cycles += 2;
        c.g[A0][0] = 2;
        c.g[V0][0] = 1;
        return Ok(Some(finish_multiplayer_tail(c)));
    }
    c.cycles += 2;
    let nonzero = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if !nonzero {
        c.cycles += 2;
        c.g[A0][0] = 1;
        return Ok(Some(1));
    }
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[A0][0] = 2;
        return Ok(Some(2));
    }
    c.cycles += 2;
    let not_two = c.g[V1][0] != c.g[S0][0];
    c.g[V0][0] = 3;
    if !not_two {
        c.cycles += 2;
        c.g[A0][0] = 4;
        return Ok(Some(4));
    }
    c.cycles += 2;
    let is_three = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = 8;
    if is_three {
        return Ok(Some(8));
    }
    c.cycles += 4;
    c.g[V1][0] ^= 4;
    c.g[A0][0] = 0x200;
    c.g[V0][0] = 0x10;
    Ok(Some(finish_multiplayer_tail(c)))
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_DJSay210B (0x2a2a10) and DJSay212D (0x2a2c30).

/// `void SSXAudioSystem_DJSay210B(SSXAudioSystem* self)` (0x2a2a10): the DJ's
/// line 0x210b. Only in a DJ speech state, and when the speech system accepts
/// the event, the line plays with the language and the rival's voice mask.
pub fn ssx_audio_system_dj_say_210b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let charge = dj_say_210b_body(c)?;
        if charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Returns whether the generated code charges its `+1` before the epilogue
/// (false only when the speech system refused the event).
fn dj_say_210b_body(c: &mut Cpu) -> Result<bool, Unwound> {
    call_ptr(c, 0x0029_f0d8, 0x002a_2a28)?; // SSXAudioSystem_IsDJSpeechOn(self)
    c.g[A0][0] = sx(say_ea(c.g[S0][0], 0x5560));
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 10;
    c.mtc1(0, 12);
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x210b;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xa;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    call_ptr(c, 0x002b_1458, 0x002a_2a50)?; // SpeechSystem_SetEventContext(bm, 0, 0x210b, ...)
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 24;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0029_f198, 0x002a_2a60)?; // SSXAudioSystem_GetLanguageMask(self)
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0028_b1e8, 0x002a_2a68)?; // SSXAudioSystem_GetBELibrary()
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x002a_2a74)?; // cBE_getInterface(be, 0)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_5750, 0x002a_2a7c)?; // getRival(interface)
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x002a_1da0, 0x002a_2a8c)?; // SSXAudioSystem_CharacterToVoiceMask(self, rival, 0)
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x210b;
    call_ptr(c, 0x003d_8008, 0x002a_2aa0)?;
    c.g[V1][0] = Addr(say_ea(c.g[GP][0], 0x173c)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S0][0];
    let sound = c.g[V1][0] as u32;
    c.g[A1][0] = 2;
    call_ptr(c, sound, 0x002a_2ab8)?;
    Ok(true)
}

/// `void SSXAudioSystem_DJSay212D(SSXAudioSystem* self)` (0x2a2c30): the DJ's
/// line 0x212d, as 210B but with the race location's speech context mask.
pub fn ssx_audio_system_dj_say_212d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let charge = dj_say_212d_body(c)?;
        if charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// As [`dj_say_210b_body`], for line 0x212d.
fn dj_say_212d_body(c: &mut Cpu) -> Result<bool, Unwound> {
    call_ptr(c, 0x0029_f0d8, 0x002a_2c48)?; // SSXAudioSystem_IsDJSpeechOn(self)
    c.g[A0][0] = sx(say_ea(c.g[S0][0], 0x5560));
    if c.g[V0][0] == 0 {
        return Ok(true);
    }
    c.cycles += 10;
    c.mtc1(0, 12);
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x212d;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xa;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    call_ptr(c, 0x002b_1458, 0x002a_2c70)?; // SpeechSystem_SetEventContext(bm, 0, 0x212d, ...)
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 24;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0029_f198, 0x002a_2c80)?; // SSXAudioSystem_GetLanguageMask(self)
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0028_b1e8, 0x002a_2c88)?; // SSXAudioSystem_GetBELibrary()
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x002a_2c94)?; // cBE_getInterface(be, 0)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_4bc0, 0x002a_2c9c)?; // getLocation(interface)
    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002a_1bd8, 0x002a_2cac)?; // SSXAudioSystem_GetSpeechContextMask(self, loc, -1)
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x212d;
    call_ptr(c, 0x003d_8008, 0x002a_2cc0)?;
    c.g[V1][0] = Addr(say_ea(c.g[GP][0], 0x173c)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S0][0];
    let sound = c.g[V1][0] as u32;
    c.g[A1][0] = 2;
    call_ptr(c, sound, 0x002a_2cd8)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PASay20C3 (0x2a3400).

/// `void SSXAudioSystem_PASay20C3(SSXAudioSystem* self)` (0x2a3400): the PA
/// announcer's line 0x20c3, when PA speech is on and no replay is playing.
pub fn ssx_audio_system_pa_say_20c3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        pa_say_20c3_body(c)?;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

fn pa_say_20c3_body(c: &mut Cpu) -> Flow {
    call_ptr(c, 0x0029_f128, 0x002a_3418)?; // SSXAudioSystem_IsPASpeechOn(self)
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0028_8ae0, 0x002a_3428)?; // SSXAudioSystem_IsReplayPlaying(self)
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 11;
    c.mtc1(0, 12);
    c.g[A0][0] = sx(say_ea(c.g[S0][0], 0x5560));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20c3;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xb;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    call_ptr(c, 0x002b_1458, 0x002a_3454)?; // SpeechSystem_SetEventContext(bm, 0, 0x20c3, ...)
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 25;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0029_f198, 0x002a_3464)?; // SSXAudioSystem_GetLanguageMask(self)
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0028_b1e8, 0x002a_346c)?; // SSXAudioSystem_GetBELibrary()
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x002a_3478)?; // cBE_getInterface(be, 0)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_4bc0, 0x002a_3480)?; // getLocation(interface)
    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002a_1bd8, 0x002a_3490)?; // SSXAudioSystem_GetSpeechContextMask(self, loc, -1)
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20c3;
    call_ptr(c, 0x003d_8008, 0x002a_34a4)?;
    c.g[V1][0] = Addr(say_ea(c.g[GP][0], 0x173c)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S0][0];
    let sound = c.g[V1][0] as u32;
    c.g[A1][0] = 2;
    call_ptr(c, sound, 0x002a_34bc)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PASay20C7 (0x2a3860).

/// `void SSXAudioSystem_PASay20C7(SSXAudioSystem* self)` (0x2a3860): the PA
/// announcer's line 0x20c7. Its voice index comes from the race interface
/// (the newRace state byte, the rider list, and the rider's ranking).
pub fn ssx_audio_system_pa_say_20c7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let charge = pa_say_20c7_body(c, frame)?;
        if charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Returns whether the generated code charges its `+1` (false for the two
/// early exits: no PA speech, or a replay playing).
fn pa_say_20c7_body(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    call_ptr(c, 0x0029_f128, 0x002a_3884)?; // SSXAudioSystem_IsPASpeechOn(self)
    let speech_off = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x50);
    if speech_off {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0028_8ae0, 0x002a_3894)?; // SSXAudioSystem_IsReplayPlaying(self)
    let replay = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x50);
    if replay {
        return Ok(false);
    }

    // 0x2a38d8: the race interface and its newRace state.
    c.cycles += 11;
    c.g[S0][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x002a_38a4)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x002a_38b0)?; // cBE_getInterface(be, 0)
    c.g[S4][0] = 2;
    c.g[V0][0] = 0x0053_0000;
    c.g[S3][0] = sx(say_ea(c.g[V0][0], 0x5bc8));
    c.g[V1][0] = Addr(say_ea(c.g[S3][0], 0x49)).lb(c);
    let not_two = c.g[V1][0] != c.g[S4][0];
    c.g[V1][0] = 0x0053_0000;
    if !not_two {
        c.cycles += 4;
        c.g[A0][0] = sx(say_ea(c.g[V1][0], 0x6730));
        c.g[A1][0] = Addr(say_ea(c.g[V1][0], 0x6730)).lw(c);
        c.g[V0][0] = Addr(say_ea(c.g[A0][0], 0x4)).lw(c);
        c.g[S0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    }

    // 0x2a39c0: the speech context of the PA line.
    c.cycles += 11;
    c.mtc1(0, 12);
    c.g[A0][0] = sx(say_ea(c.g[S1][0], 0x5560));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20c7;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xb;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    call_ptr(c, 0x002b_1458, 0x002a_38fc)?; // SpeechSystem_SetEventContext(bm, 0, 0x20c7, ...)
    let refused = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S1][0];
    if !refused {
        pa_say_20c7_voice_index(c)?;
        c.cycles += 11;
        c.g[A0][0] = 1;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20c7;
        call_ptr(c, 0x003d_8008, 0x002a_39a4)?;
        c.g[V1][0] = Addr(say_ea(c.g[GP][0], 0x173c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        let sound = c.g[V1][0] as u32;
        c.g[A1][0] = 3;
        call_ptr(c, sound, 0x002a_39c0)?;
    }
    Ok(true)
}

/// The voice index block (0x2a3960..0x2a3994): S0 ends up as the voice
/// choice the sound call takes in T0.
fn pa_say_20c7_voice_index(c: &mut Cpu) -> Flow {
    if !pa_say_20c7_rider_voice(c)? {
        return Ok(());
    }
    // 0x2a397c: the ranking-table word for the rider.
    c.cycles += 5;
    c.g[V0][0] = sx(say_ea(c.g[V0][0], 0x6730));
    c.g[V0][0] = sx(say_ea(c.g[S0][0], c.g[V0][0] as u32));
    c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let has_entry = c.g[S0][0] != 0;
    c.g[V0][0] = 1;
    if !has_entry {
        c.cycles += 2;
        c.g[S0][0] = 1;
        return Ok(());
    }
    // 0x2a398c.
    c.cycles += 2;
    let differs = c.g[S0][0] != c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] ^ 2;
    if !differs {
        c.cycles += 2;
        c.g[S0][0] = 2;
        return Ok(());
    }
    // The tail.
    c.cycles += 2;
    c.g[S0][0] = 4;
    if c.g[V0][0] != 0 {
        c.g[S0][0] = 0;
    }
    Ok(())
}

/// 0x2a3960: the rider's voice mask and the race ranking. Returns `true` when
/// the block falls into 0x2a397c, `false` when it leaves the voice block
/// with S0 = 8.
fn pa_say_20c7_rider_voice(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 18;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    call_ptr(c, 0x0029_f198, 0x002a_390c)?; // SSXAudioSystem_GetLanguageMask(self)
    c.g[S2][0] = c.g[V0][0];
    call_ptr(c, 0x0028_b1d8, 0x002a_3914)?; // cAudioInterface_getAI()
    c.g[V0][0] = sx(say_ea(c.g[V0][0], 0x28));
    c.g[A0][0] = c.g[S1][0];
    c.g[V0][0] = sx(say_ea(c.g[V0][0], c.g[S0][0] as u32));
    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    call_ptr(c, 0x002a_1d48, 0x002a_3928)?; // SSXAudioSystem_GetRiderVoiceMask(self, entry)
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dc80, 0x002a_3930)?; // cBE_getBE()
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x002a_393c)?; // cBE_getInterface(be, 13)
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr(say_ea(c.g[S3][0], 0x49)).lb(c);
    let kind_two = c.g[V0][0] == c.g[S4][0];
    c.g[V0][0] = 0x0053_0000;
    if kind_two {
        return Ok(true);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(say_ea(c.g[V1][0], 0x14)).lw(c);
    let has_flag = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0053_0000;
    if has_flag {
        return Ok(true);
    }
    c.cycles += 2;
    c.g[S0][0] = 8;
    Ok(false)
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine_rt, FAKE};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};

    /// The game's cGamePath byte (newRace state) and the ranking table word.
    const GAME_PATH: u32 = 0x0053_5c11;
    const RANKING: u32 = 0x0053_6730;

    /// The shared fixture: the BELibrary dummy and the game image (machine_rt),
    /// the game path (multiplayer for a third of the seeds), a ranking, and the
    /// sound function pointer as a fake.
    fn speech_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _r = machine_rt(c, seed, salt, 4);
        elf_image(c);
        let path = if seed % 3 == 0 { 2 } else { (seed % 2) as u8 };
        c.bus.write8(GAME_PATH, path);
        c.bus.write32(RANKING, seed % 12);
        c.bus.write32(GAME_GP + 0x173c, FAKE);
    }

    #[test]
    fn ssx_audio_system_rider_say_20b8_matches() {
        check(
            ee::s08::f_002a1560__generated,
            ssx_audio_system_rider_say_20b8,
            40,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_1560);
                let this = DATA + 0x2000;
                let rider = DATA + 0x3000;
                let vt = DATA + 0x3800;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = rider as u64;
                c.pc = 0x002a_1560;
                // The rider's sub-object: isComputer() (+0x48) is a leaf that
                // returns 0; getPosition() (+0x28) is a fake returning a DATA
                // pointer.
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, 0x0014_06c0);
                c.bus.write16(vt + 0x28, 0);
                c.bus.write32(vt + 0x2c, FAKE);
                c.bus.write32(rider + 0x87c, seed % 2);
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_trigger_speech_event_matches() {
        check(
            ee::s08::f_002a19d8__generated,
            ssx_audio_system_trigger_speech_event,
            48,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_19d8);
                let this = DATA + 0x2000;
                // Every event the switch knows, plus an unknown one.
                let events = [0u32, 1, 2, 3, 5, 7, 8, 10, 12, 14, 15, 4];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = events[(seed % 12) as usize] as u64;
                c.pc = 0x002a_19d8;
                // A song is already playing: event 0 then skips the music
                // call (0x28d488 -> MusicPlayer_SelectNextSong), which spins on
                // zeroed music state. The monitor branch is not covered here.
                c.bus.write32(this + 0x530, 1);
                c.bus.write32(this + 0x118, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_return_true_1bd0_matches() {
        check(
            ee::s08::f_002a1bd0__generated,
            ssx_audio_system_return_true_1bd0,
            8,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x2a_1bd0, 0);
                c.pc = 0x002a_1bd0;
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_finish_speech_mask_matches() {
        check(
            ee::s08::f_002a1e68__generated,
            ssx_audio_system_get_finish_speech_mask,
            60,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_1e68);
                let this = DATA + 0x2000;
                let obj = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = obj as u64;
                c.pc = 0x002a_1e68;
                // RaceRanking 0..6 covers every multiplayer branch; the
                // ranking word covers the table labels and the default.
                c.bus.write32(obj + 0xec, seed % 7);
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_210b_matches() {
        check(
            ee::s08::f_002a2a10__generated,
            ssx_audio_system_dj_say_210b,
            40,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_2a10);
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.pc = 0x002a_2a10;
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_212d_matches() {
        check(
            ee::s08::f_002a2c30__generated,
            ssx_audio_system_dj_say_212d,
            40,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_2c30);
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.pc = 0x002a_2c30;
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c3_matches() {
        check(
            ee::s08::f_002a3400__generated,
            ssx_audio_system_pa_say_20c3,
            40,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_3400);
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.pc = 0x002a_3400;
                c.bus.write32(this + 0x5560, DATA + 0x6000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c7_matches() {
        check(
            ee::s08::f_002a3860__generated,
            ssx_audio_system_pa_say_20c7,
            40,
            &|c, seed| {
                speech_fixture(c, seed, 0x2a_3860);
                let this = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.pc = 0x002a_3860;
                c.bus.write32(this + 0x5560, DATA + 0x6000);
                c.bus.write32(this + 0x118, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
            },
        );
    }
}
