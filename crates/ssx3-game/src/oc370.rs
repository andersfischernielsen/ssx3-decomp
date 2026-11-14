//! Staging module oc370 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc370`.

#![allow(unused_imports)]

use crate::abi::{addu, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0029_f660,
    ssx_audio_system_play_rider_grunt,
    "SSXAudioSystem_PlayRiderGrunt",
)];
// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayRiderGrunt (0x29f660)

/// Rider fields read by `SSXAudioSystem_PlayRiderGrunt`.
pub(crate) const PGR_RIDER_FINISH_TIME: u32 = 0x470;
pub(crate) const PGR_RIDER_VIEW: u32 = 0x6c0;
pub(crate) const PGR_RIDER_VIEW_INDEX: u32 = 0x870;
pub(crate) const PGR_RIDER_IS_HUMAN: u32 = 0x874;
pub(crate) const PGR_RIDER_IS_LOCAL: u32 = 0x87c;
/// The audio system's round-robin grunt counter (0..5).
pub(crate) const PGR_GRUNT_COUNTER: u32 = 0x5734;
/// `SSXAudioSystem::p118`: pointer to the pointer to the sound manager.
pub(crate) const PGR_MGR_HOLDER: u32 = 0x118;
/// Sound manager: the play-parameter block (`sub`) and its current slot.
pub(crate) const PGR_MGR_SUB: u32 = 0x1d8;
pub(crate) const PGR_MGR_CUR: u32 = 0x1f4;

const PGR_IS_SPEECH_VOLUME_ON: Callee = Callee {
    addr: 0x0029_f0a0,
    f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
};
const PGR_IS_CHARACTER_SPEAKING: Callee = Callee {
    addr: 0x002a_10c0,
    f: ee::s08::SSXAudioSystem_IsCharacterSpeaking,
};
const PGR_IS_TRACKED_SOUND_PLAYING: Callee = Callee {
    addr: 0x0029_5028,
    f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
};
const PGR_STOP_CHARACTER_SPEECH: Callee = Callee {
    addr: 0x002a_1028,
    f: ee::s08::SSXAudioSystem_StopCharacterSpeech,
};
const PGR_GET_GRUNT_BANK_ID: Callee = Callee {
    addr: 0x0029_f620,
    f: ee::s08::SSXAudioSystem_GetGruntBankId,
};
const PGR_AUDIO_RAND: Callee = Callee {
    addr: 0x002a_df60,
    f: ee::s08::Audio_Rand,
};
const PGR_GET_GRUNT_VOICE: Callee = Callee {
    addr: 0x0029_f378,
    f: ee::s08::SSXAudioSystem_GetGruntVoice,
};
const PGR_SET_RADIUS_VANISH: Callee = Callee {
    addr: 0x002a_9988,
    f: ee::s08::SndPlayParams_SetRadiusVanish,
};
const PGR_TRACK_SOUND: Callee = Callee {
    addr: 0x0029_5628,
    f: ee::s08::SSXAudioSystem_TrackSound,
};
const PGR_IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;
const PGR_RIDER_GET_CHARACTER: u32 = 0x0012_3128;
const PGR_CBE_GET_BE: u32 = 0x0014_dc80;
const PGR_BE_GET_INTERFACE: u32 = 0x0014_dd58;
const PGR_GET_RIDER_SPEED_VOLUME: u32 = 0x0029_0c40;
const PGR_MEMSET: u32 = 0x0041_6210;
const PGR_GET_RIDER_LISTENER_INDEX: u32 = 0x0028_8a20;
const PGR_GET_CHANNEL_GAIN_PTR: u32 = 0x0028_7968;
const PGR_START_SLOT: u32 = 0x0029_06b8;
/// The game-state byte the grunt checks (2: split-screen replay view?).
const PGR_GAME_STATE_BYTE: u32 = 0x0053_5c11;

/// `dest = rider->isHuman && rider->isLocal` the way the compiled code
/// computes it (v0 holds the last word loaded).
fn pgr_is_local_human(c: &mut Cpu, dest: usize) {
    let rider = r32(c, S5);
    c.g[V0][0] = c.lw(rider.wrapping_add(PGR_RIDER_IS_HUMAN));
    c.g[dest][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = c.lw(rider.wrapping_add(PGR_RIDER_IS_LOCAL));
        c.g[dest][0] = (c.g[V0][0] != 0) as u64;
    }
}

/// `(Audio_Rand(mgr->sub.rng) & 0x7fff) * 3 / 0x7fff` into v1.
fn pgr_rand_third(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 9;
    c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(PGR_MGR_SUB));
    PGR_AUDIO_RAND.call(c, ret)?;
    c.g[V0][0] &= 0x7fff;
    c.g[A0][0] = 0x7fff;
    c.g[V1][0] = sx(r32(c, V0) << 1);
    c.cycles += 5;
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.divu(0, V1, A0);
    c.g[V1][0] = c.lo[0];
    Ok(())
}

/// The address of slot `cur` of the array `*(sub + field)` (elements of
/// `1 << shift` bytes), computed in `ri` with the base loaded into `rb`.
fn pgr_slot(c: &mut Cpu, ri: usize, rb: usize, field: u32, shift: u32) -> u32 {
    let mgr = r32(c, S0);
    let sub = r32(c, S1);
    c.g[ri][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[rb][0] = c.lw(sub.wrapping_add(field));
    c.g[ri][0] = sx(r32(c, ri) << shift);
    c.g[ri][0] = addu(c.g[ri][0], c.g[rb][0]);
    r32(c, ri)
}

/// `void SSXAudioSystem_PlayRiderGrunt(SSXAudioSystem* self, cRider* rider,
/// int kind, int force)` (0x29f660): play an effort grunt for `rider`.
/// Nothing happens with speech volume off, during a replay, for an AI rider
/// whose character is already speaking (unless `force`), for a rider that
/// finished, or while a grunt (tracked sound 3) still plays. Otherwise the
/// character's speech stops and a new sound slot is pushed on the manager's
/// play-parameter stack: the grunt bank and a voice (`kind` 1 or 0: hard or
/// soft; bank 0xe picks one of three random variants per kind, any other
/// `kind` cycles the round-robin counter), the volume from the rider's speed
/// (clamped to 0..127), listener, channel gain, a short radius for local
/// humans (100 for others) and the rider's position; a local human in the
/// split view also gets the view index. The slot is started and tracked as
/// sound 3.
pub fn ssx_audio_system_play_rider_grunt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        for (reg, off) in [
            (S4, 0x50),
            (S0, 0x90),
            (S1, 0x80),
            (S5, 0x40),
            (S2, 0x70),
            (S3, 0x60),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0]; // self
        c.g[S0][0] = c.g[A2][0]; // kind
        c.g[S1][0] = c.g[A3][0]; // force
        c.g[S5][0] = c.g[A1][0]; // rider
        pgr_body(c)?;
        frame.restore(c, S0, 0x90);
        c.cycles += 11;
        for (reg, off) in [
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `SSXAudioSystem_PlayRiderGrunt` between prologue and
/// epilogue (an early exit returns `Ok`).
fn pgr_body(c: &mut Cpu) -> crate::call::Flow {
    PGR_IS_SPEECH_VOLUME_ON.call(c, 0x0029_f6a0)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, PGR_IS_REPLAY_PLAYING, 0x0029_f6b0)?;
    if c.g[V0][0] != 0 {
        return Ok(());
    }

    // An AI rider (or a remote one) waits for its character to finish
    // speaking, unless forced.
    c.cycles += 3;
    pgr_is_local_human(c, V1);
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        if c.g[S1][0] == 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, PGR_RIDER_GET_CHARACTER, 0x0029_f6e4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            PGR_IS_CHARACTER_SPEAKING.call(c, 0x0029_f6f0)?;
            if c.g[V0][0] != 0 {
                return Ok(());
            }
            c.cycles += 1;
        }
    }
    let finish = r32(c, S5).wrapping_add(PGR_RIDER_FINISH_TIME);
    c.lwc1(1, finish);
    // A rider with a finish time (>= 0) has finished.
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_le_s(0, 1);
    let finished = c.fcc();
    c.g[V0][0] = 1;
    if !finished {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 3;
    c.g[A2][0] = 0;
    PGR_IS_TRACKED_SOUND_PLAYING.call(c, 0x0029_f72c)?;
    if c.g[V0][0] != 0 {
        return Ok(());
    }

    // Stop the character's speech, pick the bank (s7) and the voice (s6).
    c.cycles += 12;
    c.g[A0][0] = c.g[S5][0];
    call_ptr(c, PGR_RIDER_GET_CHARACTER, 0x0029_f73c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    PGR_STOP_CHARACTER_SPEECH.call(c, 0x0029_f748)?;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S5][0];
    PGR_GET_GRUNT_BANK_ID.call(c, 0x0029_f754)?;
    c.g[S7][0] = c.g[V0][0];
    let bank_is_e = c.g[S7][0] == 0xe;
    c.g[V0][0] = 1;
    if bank_is_e {
        c.cycles += 2;
        if c.g[S0][0] == 1 {
            pgr_rand_third(c, 0x0029_f77c)?;
            c.g[S6][0] = c.g[V1][0];
        } else {
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                let counter = r32(c, S4).wrapping_add(PGR_GRUNT_COUNTER);
                c.g[S6][0] = c.lw(counter);
                c.cycles += 4;
                c.g[V0][0] = addu(c.g[S6][0], 1);
                c.g[V1][0] = ((c.g[V0][0] as i64) < 6) as u64;
                c.sw(counter, c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    c.sw(counter, 0);
                }
            } else {
                pgr_rand_third(c, 0x0029_f7bc)?;
                c.g[S6][0] = addu(c.g[V1][0], 3);
            }
        }
    } else {
        c.cycles += 2;
        c.g[A0][0] = c.g[S4][0];
        if c.g[S0][0] == 1 {
            c.cycles += 5;
            c.g[A1][0] = c.g[S5][0];
            PGR_GET_GRUNT_VOICE.call(c, 0x0029_f80c)?;
            c.g[V0][0] = sx(r32(c, V0) << 1);
            c.g[S6][0] = addu(c.g[V0][0], 1);
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[S5][0];
            PGR_GET_GRUNT_VOICE.call(c, 0x0029_f820)?;
            c.g[S6][0] = sx(r32(c, V0) << 1);
        }
    }

    // fp: a local human in the split view (game-state byte 2, no replay).
    c.cycles += 4;
    c.g[FP][0] = 0;
    pgr_is_local_human(c, V1);
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    if c.g[V1][0] != 0 {
        c.cycles += 10;
        call_ptr(c, PGR_CBE_GET_BE, 0x0029_f84c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, PGR_BE_GET_INTERFACE, 0x0029_f858)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = c.lb(PGR_GAME_STATE_BYTE);
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, PGR_IS_REPLAY_PLAYING, 0x0029_f874)?;
            c.g[FP][0] = (c.g[V0][0] < 1) as u64;
        }
        c.g[A0][0] = c.g[S4][0];
    }

    // Push a new slot on the manager's play-parameter stack (s0 = mgr,
    // s1 = sub, s3 = volume).
    c.cycles += 157;
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, PGR_GET_RIDER_SPEED_VOLUME, 0x0029_f884)?;
    c.g[S2][0] = 0x18;
    c.g[S0][0] = c.lw(r32(c, S4));
    c.g[S3][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    let mgr = r32(c, S0);
    c.g[V0][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[S1][0] = sx(mgr.wrapping_add(PGR_MGR_SUB));
    let sub = r32(c, S1);
    c.g[V0][0] = addu(c.g[V0][0], 1);
    c.sw(mgr.wrapping_add(PGR_MGR_CUR), c.g[V0][0]);
    // ch[cur] = def (24 bytes, unaligned copy through v0/a0/a3).
    c.g[V1][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[V0][0] = c.lw(sub.wrapping_add(0x20));
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.ldl(V0, mgr.wrapping_add(0x1e3));
    c.ldr(V0, mgr.wrapping_add(0x1dc));
    c.ldl(A0, mgr.wrapping_add(0x1eb));
    c.ldr(A0, mgr.wrapping_add(0x1e4));
    c.ldl(A3, mgr.wrapping_add(0x1f3));
    c.ldr(A3, mgr.wrapping_add(0x1ec));
    let ch = r32(c, V1);
    c.sdl(V0, ch.wrapping_add(0x7));
    c.sdr(V0, ch);
    c.sdl(A0, ch.wrapping_add(0xf));
    c.sdr(A0, ch.wrapping_add(0x8));
    c.sdl(A3, ch.wrapping_add(0x17));
    c.sdr(A3, ch.wrapping_add(0x10));
    // tgt[cur] (inline at mgr+0x1fc) = 0, p34[cur] = 0.
    c.g[V0][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[S0][0], c.g[V0][0]);
    c.sw(r32(c, V0).wrapping_add(0x1fc), 0);
    let a = pgr_slot(c, V1, V0, 0x34, 2);
    c.sw(a, 0);
    // vals[cur] = ch[cur].vol (signed byte).
    c.g[A3][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[A0][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[T0][0] = c.lw(sub.wrapping_add(0x20));
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = c.lw(sub.wrapping_add(0x38));
    c.g[A3][0] = addu(c.g[A3][0], c.g[V0][0]);
    c.g[A0][0] = addu(c.g[A0][0], c.g[T0][0]);
    c.g[V1][0] = c.lb(r32(c, A0));
    c.sw(r32(c, A3), c.g[V1][0]);
    let a = pgr_slot(c, V0, V1, 0x3c, 2);
    c.sw(a, 0);
    c.g[A0][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = addu(c.g[S1][0], c.g[A0][0]);
    c.g[A0][0] = addu(c.g[A0][0], 0x40);
    call_ptr(c, PGR_MEMSET, 0x0029_f95c)?;
    let mgr = r32(c, S0);
    let sub = r32(c, S1);
    c.g[V1][0] = c.lw(mgr.wrapping_add(PGR_MGR_CUR));
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.lw(sub.wrapping_add(0x60));
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
    c.g[A3][0] = 0x7f;
    c.sw(r32(c, V1), 0);
    c.g[T0][0] = 1;
    let a = pgr_slot(c, V0, V1, 0x64, 2);
    c.sw(a, 0);
    let a = pgr_slot(c, V1, V0, 0x68, 2);
    c.sw(a, 0);
    let a = pgr_slot(c, V0, V1, 0x6c, 2);
    c.sw(a, 0);
    // p70[cur] = {100, 0, 90, 50}.
    let a = pgr_slot(c, V1, V0, 0x70, 4);
    c.sw(a.wrapping_add(0x4), 0);
    let a = pgr_slot(c, V0, V1, 0x70, 4);
    c.sw(a, c.g[A0][0]);
    let a = pgr_slot(c, V1, V0, 0x70, 4);
    c.sw(a.wrapping_add(0x8), c.g[A1][0]);
    let a = pgr_slot(c, V0, V1, 0x70, 4);
    c.sw(a.wrapping_add(0xc), c.g[A2][0]);
    let a = pgr_slot(c, V1, V0, 0x74, 2);
    c.sw(a, 0);
    let a = pgr_slot(c, V0, V1, 0x80, 2);
    c.sw(a, 0);
    let a = pgr_slot(c, V1, V0, 0x84, 2);
    c.sw(a, 0);
    let a = pgr_slot(c, V0, V1, 0x78, 2);
    c.sw(a, c.g[A3][0]);
    let a = pgr_slot(c, V1, V0, 0x7c, 2);
    c.sw(a, c.g[T0][0]);

    // self->bank[cur] = s7, self->voice[cur] = s6.
    let this = r32(c, S4);
    c.g[A0][0] = c.lw(this);
    c.g[V1][0] = c.lw(this.wrapping_add(0x4));
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(PGR_MGR_CUR));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.sw(r32(c, V0), c.g[S7][0]);
    c.g[V1][0] = c.lw(this);
    c.g[A0][0] = c.lw(this.wrapping_add(0x8));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PGR_MGR_CUR));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
    c.sw(r32(c, V0), c.g[S6][0]);

    // vals[cur] = volume; ch[cur].vol = clamp(vals[cur], 0, 127).
    c.g[V1][0] = c.lw(this.wrapping_add(PGR_MGR_HOLDER));
    c.g[A1][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(PGR_MGR_CUR));
    c.g[A2][0] = addu(c.g[A1][0], PGR_MGR_SUB as u64);
    let sub2 = r32(c, A2);
    c.g[V1][0] = c.lw(sub2.wrapping_add(0x38));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.sw(r32(c, V0), c.g[S3][0]);
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(PGR_MGR_CUR));
    c.g[V1][0] = c.lw(sub2.wrapping_add(0x1c));
    c.g[A3][0] = c.lw(sub2.wrapping_add(0x38));
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A1][0] = c.lw(sub2.wrapping_add(0x20));
    c.g[V1][0] = addu(c.g[V1][0], c.g[A3][0]);
    c.g[V0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let below_max = c.g[V0][0] != 0;
    c.g[A0][0] = addu(c.g[A0][0], c.g[A1][0]);
    let mut clamp = true;
    if below_max {
        c.cycles += 6;
        c.g[V0][0] = c.lw(sub2.wrapping_add(0x1c));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A3][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        clamp = !negative;
    }
    if clamp {
        c.cycles += 7;
        c.g[V0][0] = c.lw(sub2.wrapping_add(0x1c));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A3][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(sub2.wrapping_add(0x1c));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A3][0]);
            c.g[V0][0] = c.lbu(r32(c, V0));
        }
    }

    // Listener and channel gain of the new slot.
    c.cycles += 28;
    c.sb(r32(c, A0), c.g[V0][0]);
    c.g[A1][0] = c.g[S5][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    call_ptr(c, PGR_GET_RIDER_LISTENER_INDEX, 0x0029_fb48)?;
    c.g[A1][0] = c.lw(r32(c, S0));
    c.g[A0][0] = c.g[S5][0];
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(PGR_MGR_CUR));
    c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(0x23c));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = addu(c.g[V1][0], c.g[A2][0]);
    c.sw(r32(c, V1), c.g[V0][0]);
    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    call_ptr(c, PGR_RIDER_GET_CHARACTER, 0x0029_fb6c)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 4;
    call_ptr(c, PGR_GET_CHANNEL_GAIN_PTR, 0x0029_fb7c)?;
    c.g[A0][0] = c.lw(r32(c, S0));
    c.g[A2][0] = 0;
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(PGR_MGR_CUR));
    c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x214));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = addu(c.g[V1][0], c.g[A1][0]);
    c.sw(r32(c, V1), c.g[V0][0]);
    pgr_is_local_human(c, A2);

    // Radius: ~30 for a local human, 100 for everyone else.
    c.cycles += 2;
    c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    if c.g[A2][0] != 0 {
        c.cycles += 8;
        let lit = r32(c, GP).wrapping_sub(0x4574);
        c.lwc1(12, lit);
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = addu(c.g[A0][0], PGR_MGR_SUB as u64);
        PGR_SET_RADIUS_VANISH.call(c, 0x0029_fbcc)?;
    } else {
        c.cycles += 8;
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 12);
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = addu(c.g[A0][0], PGR_MGR_SUB as u64);
        PGR_SET_RADIUS_VANISH.call(c, 0x0029_fbf0)?;
    }
    let rider = r32(c, S5);
    c.g[V1][0] = c.lw(rider.wrapping_add(PGR_RIDER_VIEW));

    // tgt[cur] = rider position (the view's virtual getter at slot 0x28).
    c.cycles += 12;
    c.g[A1][0] = sx(rider.wrapping_add(PGR_RIDER_VIEW));
    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    let vt = r32(c, V1);
    c.g[A0][0] = c.lh(vt.wrapping_add(0x28));
    c.g[V0][0] = c.lw(vt.wrapping_add(0x2c));
    let getter = r32(c, V0);
    c.g[A0][0] = addu(c.g[A1][0], c.g[A0][0]);
    call_ptr(c, getter, 0x0029_fc0c)?;
    c.g[A0][0] = c.lw(r32(c, S0));
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(PGR_MGR_CUR));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A0][0] = addu(c.g[A0][0], c.g[V1][0]);
    c.sw(r32(c, A0).wrapping_add(0x1fc), c.g[V0][0]);
    if c.g[FP][0] != 0 {
        // p80[cur] = 1, p84[cur] = rider's view index.
        c.cycles += 16;
        let this = r32(c, S4);
        c.g[A0][0] = c.lw(this.wrapping_add(PGR_MGR_HOLDER));
        c.g[A1][0] = 1;
        c.g[V1][0] = c.lw(r32(c, A0));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PGR_MGR_CUR));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x258));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
        c.sw(r32(c, V0), c.g[A1][0]);
        c.g[A0][0] = c.lw(this.wrapping_add(PGR_MGR_HOLDER));
        c.g[A1][0] = c.lw(r32(c, S5).wrapping_add(PGR_RIDER_VIEW_INDEX));
        c.g[V1][0] = c.lw(r32(c, A0));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PGR_MGR_CUR));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x25c));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
        c.sw(r32(c, V0), c.g[A1][0]);
    }

    // Start the slot and track it as sound 3 for 50 + mgr->duration.
    c.cycles += 13;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, PGR_START_SLOT, 0x0029_fc6c)?;
    c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(PGR_MGR_HOLDER));
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 3;
    c.g[V0][0] = c.lw(r32(c, V1));
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[A3][0] = c.lw(r32(c, V0).wrapping_add(0x260));
    c.g[A3][0] = addu(c.g[A3][0], 0x32);
    PGR_TRACK_SOUND.call(c, 0x0029_fc94)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SSXAudioSystem_PlayRiderGrunt -----------------------------------

    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the oc370 fakes (in the test machine's RAM).
    pub(crate) const OC370_CTRL: u32 = DATA + 0x0e_3700;
    pub(crate) const OC370_MAGIC: u32 = 0x4f43_3370;
    /// A result word meaning "keep the fake's pointer result".
    const OC370_KEEP: u32 = 0x5a5a_5a5a;

    /// Result of fake `k`: `CTRL + 4 + 8k` on its first call, `CTRL + 8 + 8k`
    /// later (call counter at `CTRL + 0x200 + 4k`).
    fn oc370_force(c: &mut Cpu, k: u32) {
        let counter = OC370_CTRL + 0x200 + 4 * k;
        let n = c.bus.read32(counter);
        c.bus.write32(counter, n + 1);
        let w = c
            .bus
            .read32(OC370_CTRL + 4 + 8 * k + if n > 0 { 4 } else { 0 });
        if w != OC370_KEEP {
            c.g[V0][0] = sx(w);
        }
    }

    fn oc370_set(c: &mut Cpu, k: u32, first: u32, later: u32) {
        c.bus.write32(OC370_CTRL + 4 + 8 * k, first);
        c.bus.write32(OC370_CTRL + 8 + 8 * k, later);
    }

    /// A gated logging fake for an override slot whose result the fixture
    /// can force (`oc370_set`).
    macro_rules! oc370_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $k:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC370_CTRL) != OC370_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
                if c.pc == ra {
                    oc370_force(c, $k);
                }
            }
        };
    }

    const PGR_K_SPEECH_ON: u32 = 0;
    const PGR_K_REPLAY: u32 = 1;
    const PGR_K_SPEAKING: u32 = 3;
    const PGR_K_TRACKED: u32 = 4;
    const PGR_K_BANK: u32 = 6;
    const PGR_K_RAND: u32 = 7;
    const PGR_K_VOICE: u32 = 8;
    const PGR_K_SPEED_VOLUME: u32 = 11;
    const PGR_K_SND_PLAY: u32 = 17;
    const PGR_K_ADD_INSTANCE: u32 = 19;

    oc370_fake!(
        oc370_fake_speech_on,
        OC370_PREV_SPEECH_ON,
        0x0029_f0a0,
        ee::s08::f_0029f0a0__generated,
        0
    );
    oc370_fake!(
        oc370_fake_replay,
        OC370_PREV_REPLAY,
        0x0028_8ae0,
        ee::s07::f_00288ae0__generated,
        1
    );
    oc370_fake!(
        oc370_fake_get_character,
        OC370_PREV_GET_CHARACTER,
        0x0012_3128,
        ee::s00::f_00123128__generated,
        2
    );
    oc370_fake!(
        oc370_fake_speaking,
        OC370_PREV_SPEAKING,
        0x002a_10c0,
        ee::s08::f_002a10c0__generated,
        3
    );
    oc370_fake!(
        oc370_fake_tracked,
        OC370_PREV_TRACKED,
        0x0029_5028,
        ee::s08::f_00295028__generated,
        4
    );
    oc370_fake!(
        oc370_fake_stop_speech,
        OC370_PREV_STOP_SPEECH,
        0x002a_1028,
        ee::s08::f_002a1028__generated,
        5
    );
    oc370_fake!(
        oc370_fake_bank,
        OC370_PREV_BANK,
        0x0029_f620,
        ee::s08::f_0029f620__generated,
        6
    );
    oc370_fake!(
        oc370_fake_rand,
        OC370_PREV_RAND,
        0x002a_df60,
        ee::s08::f_002adf60__generated,
        7
    );
    oc370_fake!(
        oc370_fake_voice,
        OC370_PREV_VOICE,
        0x0029_f378,
        ee::s08::f_0029f378__generated,
        8
    );
    oc370_fake!(
        oc370_fake_get_be,
        OC370_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated,
        9
    );
    oc370_fake!(
        oc370_fake_get_interface,
        OC370_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated,
        10
    );
    oc370_fake!(
        oc370_fake_speed_volume,
        OC370_PREV_SPEED_VOLUME,
        0x0029_0c40,
        ee::s07::f_00290c40__generated,
        11
    );
    oc370_fake!(
        oc370_fake_listener,
        OC370_PREV_LISTENER,
        0x0028_8a20,
        ee::s07::f_00288a20__generated,
        12
    );
    oc370_fake!(
        oc370_fake_gain_ptr,
        OC370_PREV_GAIN_PTR,
        0x0028_7968,
        ee::s07::f_00287968__generated,
        13
    );
    oc370_fake!(
        oc370_fake_radius,
        OC370_PREV_RADIUS,
        0x002a_9988,
        ee::s08::f_002a9988__generated,
        14
    );
    oc370_fake!(
        oc370_fake_track_sound,
        OC370_PREV_TRACK_SOUND,
        0x0029_5628,
        ee::s08::f_00295628__generated,
        15
    );
    oc370_fake!(
        oc370_fake_snd_lock,
        OC370_PREV_SND_LOCK,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated,
        16
    );
    oc370_fake!(
        oc370_fake_snd_play,
        OC370_PREV_SND_PLAY,
        0x003b_6820,
        ee::s13::f_003b6820__generated,
        17
    );
    oc370_fake!(
        oc370_fake_snd_unlock,
        OC370_PREV_SND_UNLOCK,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated,
        18
    );
    oc370_fake!(
        oc370_fake_add_instance,
        OC370_PREV_ADD_INSTANCE,
        0x002a_cfe0,
        ee::s08::f_002acfe0__generated,
        19
    );
    oc370_fake!(
        oc370_fake_set_bank_sample,
        OC370_PREV_SET_BANK_SAMPLE,
        0x002a_bce8,
        ee::s08::f_002abce8__generated,
        20
    );
    oc370_fake!(
        oc370_fake_add_timer,
        OC370_PREV_ADD_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated,
        21
    );

    fn oc370_hook_pgr() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F0A0),
            oc370_fake_speech_on,
            &OC370_PREV_SPEECH_ON,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
            oc370_fake_replay,
            &OC370_PREV_REPLAY,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123128),
            oc370_fake_get_character,
            &OC370_PREV_GET_CHARACTER,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A10C0),
            oc370_fake_speaking,
            &OC370_PREV_SPEAKING,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_00295028),
            oc370_fake_tracked,
            &OC370_PREV_TRACKED,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A1028),
            oc370_fake_stop_speech,
            &OC370_PREV_STOP_SPEECH,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F620),
            oc370_fake_bank,
            &OC370_PREV_BANK,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADF60),
            oc370_fake_rand,
            &OC370_PREV_RAND,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F378),
            oc370_fake_voice,
            &OC370_PREV_VOICE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc370_fake_get_be,
            &OC370_PREV_GET_BE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc370_fake_get_interface,
            &OC370_PREV_GET_INTERFACE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00290C40),
            oc370_fake_speed_volume,
            &OC370_PREV_SPEED_VOLUME,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288A20),
            oc370_fake_listener,
            &OC370_PREV_LISTENER,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00287968),
            oc370_fake_gain_ptr,
            &OC370_PREV_GAIN_PTR,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A9988),
            oc370_fake_radius,
            &OC370_PREV_RADIUS,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_00295628),
            oc370_fake_track_sound,
            &OC370_PREV_TRACK_SOUND,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc370_fake_snd_lock,
            &OC370_PREV_SND_LOCK,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B6820),
            oc370_fake_snd_play,
            &OC370_PREV_SND_PLAY,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc370_fake_snd_unlock,
            &OC370_PREV_SND_UNLOCK,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ACFE0),
            oc370_fake_add_instance,
            &OC370_PREV_ADD_INSTANCE,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ABCE8),
            oc370_fake_set_bank_sample,
            &OC370_PREV_SET_BANK_SAMPLE,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            oc370_fake_add_timer,
            &OC370_PREV_ADD_TIMER,
        );
    }

    /// `SSXAudioSystem_PlayRiderGrunt`: every early exit (speech off, replay,
    /// AI character speaking, finished rider, grunt still playing), bank 0xe
    /// with kinds 1/0/other (round-robin counter wrapping at 6), other banks,
    /// the split-view flag, volumes below 0, inside 0..126 and from 127 up,
    /// both radii. Callees are gated logging fakes with forced results; the
    /// slot start (`0x2906b8`, no override slot) runs for real on a
    /// consistent manager (its own callees faked, its timer loop empty).
    #[test]
    fn ssx_audio_system_play_rider_grunt_matches() {
        oc370_hook_pgr();
        covered(
            ee::s08::f_0029f660__generated,
            ssx_audio_system_play_rider_grunt,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_f660, 24);
                c.pc = 0x0029_f660;
                c.bus.write32(OC370_CTRL, OC370_MAGIC);
                for k in 0..22 {
                    oc370_set(c, k, OC370_KEEP, OC370_KEEP);
                }
                let this = DATA + 0x2_0000;
                let rider = DATA + 0x3_0000;
                let vtable = DATA + 0x4_0000;
                let holder = DATA + 0x5_0000;
                let mgr = DATA + 0x1_0000;
                let arrays = DATA + 0x7_0000;
                let table = DATA + 0x8_0000;

                // The sound manager and its play-parameter block.
                fill(c, &mut r, mgr + 0x1dc, 6);
                c.bus.write32(mgr + PGR_MGR_CUR, r.next_u32() % 3);
                c.bus.write32(mgr + 0x1f8, arrays); // ch (0x18 each)
                fill(c, &mut r, arrays, 0x18);
                for (i, off) in [0x20c, 0x210, 0x214].into_iter().enumerate() {
                    c.bus.write32(mgr + off, arrays + 0x1000 * (i as u32 + 1));
                }
                for i in 0..10u32 {
                    c.bus
                        .write32(mgr + 0x238 + 4 * i, arrays + 0x4000 + 0x100 * i);
                }
                c.bus.write32(mgr + 0x260, r.next_u32());
                c.bus.write32(mgr + 0xacc, table);
                for i in 0..16 {
                    fill(c, &mut r, table + 0x60 * i, 2);
                    c.bus.write32(table + 0x60 * i, r.next_u32() % 2);
                }

                // The audio system.
                c.bus.write32(this, mgr);
                c.bus.write32(this + 4, arrays + 0x6000);
                c.bus.write32(this + 8, arrays + 0x7000);
                c.bus.write32(this + PGR_MGR_HOLDER, holder);
                c.bus.write32(holder, mgr);
                c.bus.write32(this + PGR_GRUNT_COUNTER, r.next_u32() % 7);

                // The rider and its view's vtable (position getter at 0x28).
                c.bus.write32(rider + PGR_RIDER_VIEW, vtable);
                c.bus
                    .write16(vtable + 0x28, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vtable + 0x2c, FAKE);
                c.bus
                    .write32(rider + PGR_RIDER_IS_HUMAN, (r.next_u32() % 3 != 0) as u32);
                c.bus
                    .write32(rider + PGR_RIDER_IS_LOCAL, (r.next_u32() % 3 != 0) as u32);
                c.bus
                    .write32(rider + PGR_RIDER_VIEW_INDEX, r.next_u32() % 4);
                let finish = match r.next_u32() % 12 {
                    0 => 0,
                    1 => 0x40a0_0000,
                    2 => r.edge_f32(),
                    _ => 0xbf80_0000,
                };
                c.bus.write32(rider + PGR_RIDER_FINISH_TIME, finish);
                c.bus.write8(
                    PGR_GAME_STATE_BYTE,
                    if r.next_u32() % 2 == 0 { 2 } else { 1 },
                );

                // Callee results.
                let pick = |r: &mut crate::difftest::Rng, n: u32| r.next_u32() % n == 0;
                let on = !pick(&mut r, 10) as u32;
                oc370_set(c, PGR_K_SPEECH_ON, on, on);
                let replay1 = pick(&mut r, 10) as u32;
                let replay2 = r.next_u32() % 2;
                oc370_set(c, PGR_K_REPLAY, replay1, replay2);
                let speaking = r.next_u32() % 2;
                oc370_set(c, PGR_K_SPEAKING, speaking, speaking);
                let tracked = pick(&mut r, 10) as u32;
                oc370_set(c, PGR_K_TRACKED, tracked, tracked);
                let bank = if r.next_u32() % 2 == 0 {
                    0xe
                } else {
                    r.next_u32() % 16
                };
                oc370_set(c, PGR_K_BANK, bank, bank);
                let rand = r.next_u32();
                oc370_set(c, PGR_K_RAND, rand, rand);
                let voice = r.next_u32() % 8;
                oc370_set(c, PGR_K_VOICE, voice, voice);
                let volume = match r.next_u32() % 6 {
                    0 => (-(1 + (r.next_u32() % 100) as i32)) as u32,
                    1 => 127,
                    2 => 128 + r.next_u32() % 1000,
                    3 => r.next_u32(),
                    _ => r.next_u32() % 127,
                };
                oc370_set(c, PGR_K_SPEED_VOLUME, volume, volume);
                if pick(&mut r, 4) {
                    oc370_set(c, PGR_K_SND_PLAY, 0xffff_fff0, 0xffff_fff0);
                }
                if pick(&mut r, 3) {
                    oc370_set(c, PGR_K_ADD_INSTANCE, 0, 0);
                }

                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = match r.next_u32() % 4 {
                    0 => 0,
                    1 | 2 => 1,
                    _ => sx(r.next_u32() % 5 + 2),
                };
                c.g[A3][0] = (r.next_u32() % 3 == 0) as u64;
            },
        );
    }
}
