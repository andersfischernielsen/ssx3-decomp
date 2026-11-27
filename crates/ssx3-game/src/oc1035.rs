//! Staging module oc1035 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1035`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_3aa0,
        ssx_audio_system_crowd_stream_done_callback,
        "SSXAudioSystem_CrowdStreamDoneCallback",
    ),
    (
        0x002a_3ce8,
        ssx_audio_system_arcade_say_20a9,
        "SSXAudioSystem_ArcadeSay20A9",
    ),
    (
        0x002a_3de0,
        ssx_audio_system_arcade_say_2133,
        "SSXAudioSystem_ArcadeSay2133",
    ),
    (
        0x002a_4a38,
        ssx_audio_system_is_section_speech_pending,
        "SSXAudioSystem_IsSectionSpeechPending",
    ),
    (
        0x002a_64a0,
        crowd_audio_instance_play_trick,
        "CrowdAudio_InstancePlayTrick",
    ),
    (
        0x002a_6648,
        crowd_audio_instance_play_fall,
        "CrowdAudio_InstancePlayFall",
    ),
];

// ---------------------------------------------------------------------------
// Callees and globals.

/// `SSXAudioSystem_IsArcadeSpeechOn(self)` (0x29f160).
const IS_ARCADE_SPEECH_ON: Callee = Callee {
    addr: 0x0029_f160,
    f: ee::s08::SSXAudioSystem_IsArcadeSpeechOn,
};

/// `SpeechSystem_SetEventContext(...)` (0x2b1458).
const SET_EVENT_CONTEXT: Callee = Callee {
    addr: 0x002b_1458,
    f: ee::s08::SpeechSystem_SetEventContext,
};

/// `SSXAudioSystem_IsCharacterSpeaking(self, n)` (0x2a10c0).
const IS_CHARACTER_SPEAKING: Callee = Callee {
    addr: 0x002a_10c0,
    f: ee::s08::SSXAudioSystem_IsCharacterSpeaking,
};

/// `CrowdAudio_PlayTrick(self, arg, voice)` (0x2a73a8).
const CROWD_PLAY_TRICK: Callee = Callee {
    addr: 0x002a_73a8,
    f: ee::s08::CrowdAudio_PlayTrick,
};

/// `CrowdAudio_InstanceStopSound(instance, voice)` (0x2a6848).
const INSTANCE_STOP_SOUND: Callee = Callee {
    addr: 0x002a_6848,
    f: ee::s08::CrowdAudio_InstanceStopSound,
};

/// `CrowdAudio_PickRandomSample(list)` (0x2a4ca0).
const PICK_RANDOM_SAMPLE: Callee = Callee {
    addr: 0x002a_4ca0,
    f: ee::s08::CrowdAudio_PickRandomSample,
};

/// `MIDIBanks_PushPlayParams(bank, params)` (0x2a8450).
const MIDI_PUSH_PLAY_PARAMS: Callee = Callee {
    addr: 0x002a_8450,
    f: ee::s08::MIDIBanks_PushPlayParams,
};

/// `MIDIBanks_Play(bank)` (0x2a86b8).
const MIDI_PLAY: Callee = Callee {
    addr: 0x002a_86b8,
    f: ee::s08::MIDIBanks_Play,
};

/// Offset of the MIDI bank pointer slot from `gp` (`*(gp + 0x608)` is the
/// bank; `*bank` is its sample table).
const GP_MIDI_BANK: u32 = 0x608;
/// Offset of the speech function pointer from `gp` (`D_004A482C`).
const GP_SPEECH_FN: u32 = 0x173c;

/// `int SSX...::GetFocusRider` / `ReplayMode` are reached through the table
/// (`call_indirect`), as the generated body does.
const CUR_FOCUS_RIDER: u32 = 0x0028_5d98;
const REPLAY_MODE: u32 = 0x0028_8ae0;
/// `func_003D8008(1, 0, id)`: the speech event lookup.
const SPEECH_EVENT_LOOKUP: u32 = 0x003d_8008;

// ---------------------------------------------------------------------------
// SSXAudioSystem_CrowdStreamDoneCallback

/// `bool SSXAudioSystem_CrowdStreamDoneCallback(CrowdAudio *self, Monitor *m)`
/// (0x2a3aa0): the crowd stream's done callback. Enter the sound critical
/// section, ask the monitor's stream (`m->mData1`, `+0x1c`) for its request
/// status, leave the section; when the request reports done (`kSNDREQUEST_Done`
/// = 3), play the crowd trick sample of voice 0 and return 1. Otherwise return 0.
/// A negative status returns 0 without touching the sample.
pub fn ssx_audio_system_crowd_stream_done_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        // SNDSYS_entercritical()
        call_ptr(c, 0x003b_58a0, 0x002a_3abc)?;
        // SNDSTRM_requeststatus(m->mData1, st)
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003b_7c40, 0x002a_3ac8)?;
        c.g[S0][0] = c.g[V0][0];
        // SNDSYS_leavecritical()
        call_ptr(c, 0x003b_58d8, 0x002a_3ad0)?;
        let failed = (c.g[S0][0] as i64) < 0;
        c.g[V1][0] = Addr(sp(c)).lw(c); // st[0]
        if failed {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3; // kSNDREQUEST_Done
            let not_done = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !not_done {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                CROWD_PLAY_TRICK.call(c, 0x002a_3af4)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsSectionSpeechPending

/// `bool SSXAudioSystem_IsSectionSpeechPending(SSXAudioSystem *self)`
/// (0x2a4a38): true when the section-speech slot (`+0x5818`) is set and
/// `SSXAudioSystem_IsCharacterSpeaking(self, 10)` reports speech.
pub fn ssx_audio_system_is_section_speech_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x5818).lw(c);
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0xa;
            IS_CHARACTER_SPEAKING.call(c, 0x002a_4a54)?;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_ArcadeSay20A9 / ArcadeSay2133

/// The epilogue both arcade-say functions share (`s0` is restored by the
/// exit path that reaches it).
fn arcade_say_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}

/// The speech-timing test: `f20` is the constant 0.0 the generated body
/// makes with `mtc1 $zero, f20`, `f0` the rider's finish time. Charges the
/// block's six cycles, sets the FPU condition (`0.0 <= finish`, i.e. the
/// finish time is not negative) and returns it.
fn finish_time_done(c: &mut Cpu, finish_addr: u32) -> bool {
    c.cycles += 6;
    let finish = Addr(finish_addr).read_fpu(c);
    c.f[0] = finish.0;
    c.f[20] = 0;
    let done = FpuF32(0).le(finish);
    abi::set_fcc(c, done);
    done
}

/// `void SSXAudioSystem_ArcadeSay20A9(SSXAudioSystem *self, Rider *rider, int n)`
/// (0x2a3ce8): the arcade announcer's line for a rider event, sound 0x20a9.
/// Nothing plays while the arcade speech is off, the rider is null, the rider
/// has already finished (`mFinishTime >= 0`), replay mode is on, or `n == 0`.
/// Otherwise, unless `n == 1` or the rider is the focus rider, it asks the
/// speech system for an event and plays the line through the speech function
/// pointer.
pub fn ssx_audio_system_arcade_say_20a9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        IS_ARCADE_SPEECH_ON.call(c, 0x002a_3d10)?;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                arcade_say_20a9_line(c, frame)?;
                arcade_say_epilogue(c, frame);
                return Ok(());
            }
        }
        frame.restore(c, S0, 0x30);
        arcade_say_epilogue(c, frame);
        Ok(())
    })
}

/// The rest of `ArcadeSay20A9` once the arcade speech is on and the rider is
/// non-null. Restores `s0` on every exit.
fn arcade_say_20a9_line(c: &mut Cpu, frame: Frame) -> Flow {
    let done = finish_time_done(c, (c.g[S0][0] as u32).wrapping_add(0x470));
    c.g[V0][0] = 1;
    if !done {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    // Unless n == 1, the rider must be the focus rider.
    c.cycles += 3;
    c.g[V0][0] = 1;
    let is_one = c.g[S2][0] == c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    if !is_one {
        c.cycles += 4;
        c.g[A1][0] = u64::MAX;
        call_ptr(c, CUR_FOCUS_RIDER, 0x002a_3d58)?;
        let not_focus = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x30);
        if not_focus {
            return Ok(());
        }
    }
    // Replay mode silences the line.
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, REPLAY_MODE, 0x002a_3d68)?;
    frame.restore(c, S0, 0x30);
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 2;
    let n_is_zero = c.g[S2][0] == 0;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
    if n_is_zero {
        return Ok(());
    }
    c.cycles += 10;
    c.f[12] = c.f[20];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x20a9;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xc;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    SET_EVENT_CONTEXT.call(c, 0x002a_3d98)?;
    let no_event = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if !no_event {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20a9;
        call_ptr(c, SPEECH_EVENT_LOOKUP, 0x002a_3dac)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_SPEECH_FN)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = 1;
        call_ptr(c, target, 0x002a_3dc0)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `void SSXAudioSystem_ArcadeSay2133(SSXAudioSystem *self, Obj *obj, int ev)`
/// (0x2a3de0): the arcade line for event 0x2133, the same shape as
/// `ArcadeSay20A9` without the focus-rider test.
pub fn ssx_audio_system_arcade_say_2133(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        IS_ARCADE_SPEECH_ON.call(c, 0x002a_3e08)?;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                arcade_say_2133_line(c, frame)?;
                arcade_say_epilogue(c, frame);
                return Ok(());
            }
        }
        frame.restore(c, S0, 0x30);
        arcade_say_epilogue(c, frame);
        Ok(())
    })
}

/// The rest of `ArcadeSay2133` once the speech is on and `obj` is non-null.
/// `s0` is the speech system, `s1` the object. Restores `s0` on every exit.
fn arcade_say_2133_line(c: &mut Cpu, frame: Frame) -> Flow {
    let done = finish_time_done(c, (c.g[S1][0] as u32).wrapping_add(0x470));
    c.g[V0][0] = 1;
    if !done {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    // Replay mode silences the line.
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, REPLAY_MODE, 0x002a_3e44)?;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    c.cycles += 11;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
    c.f[12] = c.f[20];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x2133;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0xc;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    SET_EVENT_CONTEXT.call(c, 0x002a_3e70)?;
    let no_event = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if !no_event {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2133;
        call_ptr(c, SPEECH_EVENT_LOOKUP, 0x002a_3e84)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_SPEECH_FN)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = 1;
        call_ptr(c, target, 0x002a_3e98)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    Ok(())
}

// ---------------------------------------------------------------------------
// CrowdAudio_InstancePlayTrick / InstancePlayFall

/// `sw reg, 0(addr)`: the register is read before the store (the store
/// borrows `c` mutably).
fn sw_reg(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sw(c, v);
}

/// `sb reg, 0(addr)`, read the same way as [`sw_reg`].
fn sb_reg(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sb(c, v);
}

/// The return addresses and the few differences between the two instance
/// players (the trick and the fall share one body in the generated code).
#[derive(Clone, Copy)]
struct InstancePlayRets {
    stop: u32,
    pick: u32,
    push: u32,
    play: u32,
    /// Fall passes the picked record plus `0x18`, Trick the record itself.
    fall: bool,
}

/// The shared body of `CrowdAudio_InstancePlayTrick` (0x2a64a0) and
/// `CrowdAudio_InstancePlayFall` (0x2a6648): stop the instance's sound (its
/// `+0xd4` handle), then, when a handle was set, pick a sample for voice
/// `a1` in the MIDI bank (`*(gp+0x608)`), push the play parameters, store
/// the instance's volume and play the bank. `s0` is the instance, `s1` the
/// voice index.
fn crowd_instance_play(c: &mut Cpu, r: InstancePlayRets) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    INSTANCE_STOP_SOUND.call(c, r.stop)?;
    let inst = c.g[S0][0] as u32;
    c.g[V0][0] = Addr(inst).off(0xd4).lw(c);
    let idle = c.g[V0][0] == 0;
    c.g[V0][0] = 0x54;
    if !idle {
        c.cycles += 13;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
        c.mult(0, V0, S1, V0); // v0 = voice * 0x54
        let voice = c.g[S1][0];
        Addr(inst.wrapping_add(0x184)).sw(c, voice);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let rec = c.g[V1][0] as u32;
        c.lwc1(0, rec.wrapping_add(0x48));
        c.g[A0][0] = if r.fall {
            sx(rec.wrapping_add(0x18))
        } else {
            c.g[V1][0]
        };
        c.lwc1(1, rec.wrapping_add(0x4c));
        let level = FpuF32(c.f[0]) + FpuF32(c.f[1]);
        c.f[0] = level.0;
        c.swc1(0, inst.wrapping_add(0x188));
        PICK_RANDOM_SAMPLE.call(c, r.pick)?;
        c.g[A2][0] = c.g[V0][0];
        if c.g[V0][0] != 0 {
            play_picked_sample(c, r, inst)?;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The part of the instance player after a non-zero sample pick: push the
/// play parameters, check the bank's sample level, and play the bank.
fn play_picked_sample(c: &mut Cpu, r: InstancePlayRets, inst: u32) -> Flow {
    // `skip_fallback`: the level check decided on the early store path.
    let mut skip_fallback = false;
    'level: {
        c.cycles += 30;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
        c.g[A1][0] = Addr(inst.wrapping_add(0xd8)).lw(c);
        MIDI_PUSH_PLAY_PARAMS.call(c, r.push)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
        c.g[A2][0] = sx(inst.wrapping_add(0x10));
        c.g[A3][0] = 0x18;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1f4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let slot = (c.g[V1][0] as u32).wrapping_add(0x1fc);
        sw_reg(c, slot, A2);
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A2][0] = Addr(inst.wrapping_add(0x178)).lw(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1f4)).lw(c);
        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
        c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x38)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let slot = c.g[V0][0] as u32;
        sw_reg(c, slot, A2);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1f4)).lw(c);
        c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[A2][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x38)).lw(c);
        c.mult(0, A0, A0, A3);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x20)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
        let below = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        if below {
            break 'level;
        }
        c.cycles += 6;
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        if negative {
            skip_fallback = true;
        }
    }
    if !skip_fallback {
        c.cycles += 7;
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
        }
    }
    // The store of the chosen level, then the play.
    c.cycles += 30;
    let byte_at = c.g[A0][0] as u32;
    sb_reg(c, byte_at, V0);
    c.g[A2][0] = 1;
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
    c.g[A1][0] = Addr(inst.wrapping_add(0x17c)).lw(c);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1f4)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x214)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let slot = c.g[V0][0] as u32;
    sw_reg(c, slot, A1);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1f4)).lw(c);
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let slot = c.g[V0][0] as u32;
    sw_reg(c, slot, A2);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
    c.g[A1][0] = Addr(inst).lw(c);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1f4)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x23c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let slot = c.g[V0][0] as u32;
    sw_reg(c, slot, A1);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(GP_MIDI_BANK)).lw(c);
    MIDI_PLAY.call(c, r.play)?;
    c.g[V1][0] = if r.fall {
        !c.g[S1][0]
    } else {
        sx((c.g[S1][0] as u32).wrapping_add(1))
    };
    sw_reg(c, inst.wrapping_add(0x174), V0);
    sw_reg(c, inst.wrapping_add(0x18c), V1);
    Ok(())
}

/// `void CrowdAudio_InstancePlayTrick(CrowdAudio *self, int voice, int arg)`
/// (0x2a64a0): play a trick sound on one voice of a crowd instance.
pub fn crowd_audio_instance_play_trick(c: &mut Cpu) {
    guest(c, |c| {
        crowd_instance_play(
            c,
            InstancePlayRets {
                stop: 0x002a_64bc,
                pick: 0x002a_64f4,
                push: 0x002a_6508,
                play: 0x002a_6624,
                fall: false,
            },
        )
    })
}

/// `void CrowdAudio_InstancePlayFall(CrowdAudio *self, int voice, int arg)`
/// (0x2a6648): the same as the trick player, for the fall sound.
pub fn crowd_audio_instance_play_fall(c: &mut Cpu) {
    guest(c, |c| {
        crowd_instance_play(
            c,
            InstancePlayRets {
                stop: 0x002a_6664,
                pick: 0x002a_669c,
                push: 0x002a_66b0,
                play: 0x002a_67cc,
                fall: true,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, DATA, GAME_GP, SP as STACK};

    /// The crowd object (`self`) of the stream callback and the monitor.
    const CROWD: u32 = DATA + 0x1000;
    const MONITOR: u32 = DATA + 0x2000;
    /// The speech system and a rider for the arcade lines.
    const SPEECH: u32 = DATA + 0x1000;
    const RIDER: u32 = DATA + 0x2000;
    /// The crowd instance and its MIDI bank tables.
    const INST: u32 = DATA + 0x1000;
    const BANK: u32 = DATA + 0x4000;
    const TBL: u32 = DATA + 0x5000;
    const RECS: u32 = DATA + 0x6000;
    const ARR_A: u32 = DATA + 0x8000;
    const ARR_B: u32 = DATA + 0x9000;
    const ARR_C: u32 = DATA + 0x9400;
    const ARR_D: u32 = DATA + 0x9800;

    /// The stream callback: the status array is at the callee's `sp`
    /// (`SP - 0x40`). `requeststatus` is the fake's second call (index 1) and
    /// its forced result is a non-negative status (0 keeps the fake's hash).
    fn stream_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 4);
        c.g[A0][0] = sx(CROWD);
        c.g[A1][0] = sx(MONITOR);
        c.bus.write32(MONITOR + 0x1c, DATA + 0x3000);
        // No voices: the play call returns at once.
        c.bus.write32(CROWD + 0x18, 0);
        c.bus
            .write32(STACK - 0x40, [3u32, 2, 0, 3][seed as usize % 4]);
        c.bus
            .write32(LOG_RESULTS + 4, [0u32, 1, 0x7fff_0000][seed as usize % 3]);
    }

    /// The arcade lines: the speech system at `SPEECH` with its arcade channel
    /// volume, the rider at `RIDER` with its finish time, and the event `n`
    /// (`a2`). A null rider appears for seed 6 mod 7. `GET_FOCUS_RIDER`,
    /// `ReplayMode` and the speech lookup are real or fake callees; the
    /// speech function pointer is the fake method address.
    fn arcade_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 4);
        c.g[A0][0] = sx(SPEECH);
        c.g[A1][0] = if seed % 7 == 6 { 0 } else { sx(RIDER) };
        c.g[A2][0] = (seed % 3) as u64;
        c.bus.write32(
            SPEECH + 0x635c,
            [0u32, 0x3f80_0000, 1, 0x7f80_0000][(seed / 3 % 4) as usize],
        );
        c.bus.write32(
            RIDER + 0x470,
            [0x3f80_0000u32, 0xbf80_0000][(seed % 2) as usize],
        );
        c.bus.write32(GAME_GP + GP_SPEECH_FN, FAKE);
    }

    /// The speech system with its section-speech slot (`+0x5818`) either
    /// empty or set, so the callee's `IsCharacterSpeaking` branch runs.
    fn pending_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 2);
        c.g[A0][0] = sx(SPEECH);
        c.bus
            .write32(SPEECH + 0x5818, [0u32, 0x10][seed as usize % 2]);
    }

    /// A crowd instance with its MIDI bank: the bank pointer at
    /// `gp + 0x608`, the sample table at `TBL`, two records at `RECS` and the
    /// sample arrays. `+0xd4 == 0` (seed 0 mod 5) takes the idle path.
    fn instance_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 6);
        c.g[A0][0] = sx(INST);
        c.g[A1][0] = (seed % 2) as u64;
        c.bus
            .write32(INST + 0xd4, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(INST + 0xd8, DATA + 0x7000);
        c.bus.write32(INST + 0x178, r.next_u32());
        c.bus.write32(INST + 0x17c, r.next_u32());
        c.bus.write32(GAME_GP + GP_MIDI_BANK, BANK);
        c.bus.write32(BANK, TBL);
        c.bus.write32(TBL + 0x1f4, 2);
        c.bus.write32(TBL + 0x1d8, RECS);
        c.bus.write32(TBL + 0x214, ARR_B);
        c.bus.write32(TBL + 0x23c, ARR_C);
        c.bus.write32(TBL + 0x24c, ARR_D);
        c.bus.write32(RECS + 0x1c, 2);
        c.bus.write32(RECS + 0x20, 0x10);
        c.bus.write32(RECS + 0x38, ARR_A);
        fill(c, &mut r, ARR_A, 64);
        fill(c, &mut r, ARR_B, 64);
        fill(c, &mut r, ARR_C, 64);
        fill(c, &mut r, ARR_D, 64);
        fill(c, &mut r, DATA + 0x7000, 16);
        // The record's level and sample tables read as small values.
        for i in 0..16u32 {
            c.bus
                .write32(ARR_A + 4 * i, seed.wrapping_add(i).wrapping_mul(37) % 200);
        }
    }

    #[test]
    fn ssx_audio_system_crowd_stream_done_callback_matches() {
        check(
            ee::s08::f_002a3aa0__generated,
            ssx_audio_system_crowd_stream_done_callback,
            120,
            &|c, seed| stream_setup(c, seed, 0x2a3aa0),
        );
    }

    #[test]
    fn ssx_audio_system_is_section_speech_pending_matches() {
        check(
            ee::s08::f_002a4a38__generated,
            ssx_audio_system_is_section_speech_pending,
            120,
            &|c, seed| pending_setup(c, seed, 0x2a4a38),
        );
    }

    #[test]
    fn ssx_audio_system_arcade_say_20a9_matches() {
        check(
            ee::s08::f_002a3ce8__generated,
            ssx_audio_system_arcade_say_20a9,
            120,
            &|c, seed| arcade_setup(c, seed, 0x2a3ce8),
        );
    }

    #[test]
    fn ssx_audio_system_arcade_say_2133_matches() {
        check(
            ee::s08::f_002a3de0__generated,
            ssx_audio_system_arcade_say_2133,
            120,
            &|c, seed| arcade_setup(c, seed, 0x2a3de0),
        );
    }

    #[test]
    fn crowd_audio_instance_play_trick_matches() {
        check(
            ee::s08::f_002a64a0__generated,
            crowd_audio_instance_play_trick,
            60,
            &|c, seed| instance_setup(c, seed, 0x2a64a0),
        );
    }

    #[test]
    fn crowd_audio_instance_play_fall_matches() {
        check(
            ee::s08::f_002a6648__generated,
            crowd_audio_instance_play_fall,
            60,
            &|c, seed| instance_setup(c, seed, 0x2a6648),
        );
    }
}
