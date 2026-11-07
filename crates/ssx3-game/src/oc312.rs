//! Staging wave 212 (oc312): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_7218, snd_init_patch_params, "SND_InitPatchParams"),
    (
        0x0010_c258,
        computer_actor_rider_roll_uber_trick,
        "ComputerActorRider_RollUberTrick",
    ),
    (0x003d_fd58, stream_file_enqueue_request, "StreamFile_EnqueueRequest"),
    (0x003d_9ed0, spch_check_event_masks, "SPCH_CheckEventMasks"),
    (
        0x0014_7410,
        be_new_player_int_get_alt_character,
        "BENewPlayerInt_GetAltCharacter",
    ),
    (
        0x0010_c320,
        computer_actor_rider_roll_fitted_trick,
        "ComputerActorRider_RollFittedTrick",
    ),
    (0x003c_3178, snd_channel_apply_lowpass, "SND_ChannelApplyLowpass"),
    (0x003c_8c30, snd_ee_voice_set_highpass, "SND_EeVoiceSetHighpass"),
    (0x0030_00a8, wscript_fn_start_object_loop, "WScriptFn_StartObjectLoop"),
];

// ---------------------------------------------------------------------------
// SND_InitPatchParams (0x3c7218)

/// Patch parameter block fields (`SND_InitPatchParams`).
pub(crate) const SND_PATCH_FLAGS: u32 = 0x0;
pub(crate) const SND_PATCH_B2: u32 = 0x2;
pub(crate) const SND_PATCH_VOLUME: u32 = 0x3;
pub(crate) const SND_PATCH_PAN: u32 = 0x4;
pub(crate) const SND_PATCH_RANGE: u32 = 0x8;
/// Six halfwords at +0x0c and six words at +0x18.
pub(crate) const SND_PATCH_HALVES: u32 = 0xc;
pub(crate) const SND_PATCH_WORDS: u32 = 0x18;
/// Four words at +0x48 and four at +0x58.
pub(crate) const SND_PATCH_A48: u32 = 0x48;
pub(crate) const SND_PATCH_A58: u32 = 0x58;

/// `int SND_InitPatchParams(Patch* p)` (0x3c7218): reset a sound patch's
/// parameter block to defaults: volume 0x7f, pan 0x40 (centre), range 8,
/// everything else (flags, bytes 2/5/6/7, the six halfword/word modulation
/// slots at +0x0c/+0x18 and the two word arrays at +0x48/+0x58) zero.
/// Returns 0. Leaf; both clearing loops check events at their back edges.
pub fn snd_init_patch_params(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A1][0] = c.g[A0][0];
    let patch = Addr(c.g[A1][0] as u32);
    c.g[V0][0] = 0x7f;
    c.g[V1][0] = 0x40;
    c.g[A0][0] = 0x8;
    patch.off(SND_PATCH_VOLUME).sb(c, 0x7f);
    patch.off(SND_PATCH_PAN).sb(c, 0x40);
    patch.off(SND_PATCH_RANGE).sh(c, 0x8);
    patch.off(SND_PATCH_B2).sb(c, 0);
    patch.off(SND_PATCH_FLAGS).sh(c, 0);
    for b in 5..8 {
        patch.off(b).sb(c, 0);
    }

    // a48[i] = a58[i] = 0 for i in 0..4 (v0 walks a58, v1 counts down).
    c.g[V0][0] = sx(patch.off(SND_PATCH_A58).0);
    c.g[V1][0] = 0x3;
    loop {
        c.cycles += 7;
        let at = c.g[V0][0] as u32;
        Addr(at.wrapping_sub(SND_PATCH_A58 - SND_PATCH_A48)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        Addr(at).sw(c, 0);
        c.g[V0][0] = sx(at.wrapping_add(4));
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_7250);
        }
    }

    // sC[i] = 0 (halfwords), i18[i] = 0 (words) for i in 0..6.
    c.cycles += 3;
    let patch = Addr(c.g[A1][0] as u32);
    c.g[V0][0] = sx(patch.off(SND_PATCH_HALVES).0);
    c.g[A0][0] = sx(patch.off(SND_PATCH_WORDS).0);
    c.g[V1][0] = 0x5;
    loop {
        c.cycles += 7;
        let half = c.g[V0][0] as u32;
        let word = c.g[A0][0] as u32;
        Addr(half).sh(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        Addr(word).sw(c, 0);
        c.g[V0][0] = sx(half.wrapping_add(2));
        c.g[A0][0] = sx(word.wrapping_add(4));
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_7278);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// ComputerActorRider_RollUberTrick (0x10c258)

/// The rider (player state) a computer actor drives: `self + 0x18`.
pub(crate) const CAR_RIDER: u32 = 0x18;
/// Rider ability flags; bit 0: uber tricks unlocked.
pub(crate) const CAR_RIDER_ABILITY_FLAGS: u32 = 0xb2c;
/// The AI rider's trick skill (0..1 float) in the computer actor.
pub(crate) const CAR_TRICK_SKILL: u32 = 0xdfc;
/// `BENewRaceInt_GetTrack` result that forbids uber tricks.
pub(crate) const CAR_NO_UBER_TRACK: u64 = 0xe;
/// 100.0f.
const CAR_HUNDRED: u32 = 0x42c8_0000;

/// `bool ComputerActorRider_RollUberTrick(ComputerActorRider* self)`
/// (0x10c258): whether the AI rider tries an uber trick now. Only riders
/// with uber tricks unlocked (rider flags bit 0), never on track 0xe
/// (`BENewRaceInt_GetTrack` of the race interface); then a dice roll:
/// `Rand_NextGame() % 100 < chance * 100`.
pub fn computer_actor_rider_roll_uber_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(me.wrapping_add(CAR_RIDER)).lw(c);
        let flags = Addr((c.g[V1][0] as u32).wrapping_add(CAR_RIDER_ABILITY_FLAGS)).lw(c);
        c.g[V0][0] = 0;
        if flags & 1 != 0 {
            // BELibrary_GetInterface(BELibrary_GetInstance(), 0) -> GetTrack.
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x0010_c284)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0010_c290)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4be0, 0x0010_c298)?;
            c.g[V1][0] = CAR_NO_UBER_TRACK;
            let track = c.g[V0][0];
            c.g[V0][0] = 0;
            if track != CAR_NO_UBER_TRACK {
                car_roll_against_chance(c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The dice roll of `ComputerActorRider_RollUberTrick` (0x10c2a4..):
/// `v0 = (float)(Rand_NextGame() % 100u) < self->chance * 100.0f`.
fn car_roll_against_chance(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 9;
    call_ptr(c, 0x0031_7810, 0x0010_c2ac)?;
    c.g[V1][0] = 100;
    let me = c.g[S0][0] as u32;
    let chance = Addr(me.wrapping_add(CAR_TRICK_SKILL)).read_fpu(c);
    c.f[1] = chance.0;
    let roll_raw = c.g[V0][0] as u32;
    abi::divu(c, roll_raw, 100);
    c.g[AT][0] = CAR_HUNDRED as u64;
    let hundred = FpuF32(CAR_HUNDRED);
    c.f[0] = hundred.0;

    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    let threshold = chance * hundred;
    c.f[1] = threshold.0;
    let roll = car_u32_to_float(c);
    c.f[0] = roll.0;

    c.cycles += 4;
    let hit = abi::c_lt_s(c, roll, threshold);
    c.g[V0][0] = 1;
    if !hit {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// gcc's `(float)(unsigned)v0` (the trick rolls): `cvt.s.w` directly, or for
/// values >= 2^31 halve (keeping the low bit), convert and double. Leaves
/// v0/v1 as the original does.
fn car_u32_to_float(c: &mut Cpu) -> FpuF32 {
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[V0][0] = (c.g[V0][0] & 1) | c.g[V1][0];
        let half = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        half + half
    } else {
        c.cycles += 4;
        FpuF32::from_i32(c.g[V0][0] as u32 as i32)
    }
}

// ---------------------------------------------------------------------------
// ComputerActorRider_RollFittedTrick (0x10c320)

/// 70.0f and 30.0f.
const CAR_SEVENTY: u32 = 0x428c_0000;
const CAR_THIRTY: u32 = 0x41f0_0000;

/// `bool ComputerActorRider_RollFittedTrick(ComputerActorRider* self)`
/// (0x10c320): whether the AI rider picks a trick fitted to the jump: a
/// dice roll `Rand_NextGame() % 100 < chance * 70 + 30` (so at least 30%).
pub fn computer_actor_rider_roll_fitted_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_7810, 0x0010_c334)?;
        c.g[V1][0] = 100;
        let me = c.g[S0][0] as u32;
        let chance = Addr(me.wrapping_add(CAR_TRICK_SKILL)).read_fpu(c);
        c.f[0] = chance.0;
        let roll_raw = c.g[V0][0] as u32;
        abi::divu(c, roll_raw, 100);
        let (seventy, thirty) = (FpuF32(CAR_SEVENTY), FpuF32(CAR_THIRTY));
        c.f[1] = seventy.0;
        c.g[AT][0] = CAR_THIRTY as u64;
        c.f[2] = thirty.0;

        c.cycles += 4;
        let scaled = chance * seventy;
        c.g[V0][0] = c.hi[0];
        let threshold = scaled + thirty;
        c.f[0] = threshold.0;
        let roll = car_u32_to_float(c);
        c.f[1] = roll.0;

        c.cycles += 4;
        let hit = abi::c_lt_s(c, roll, threshold);
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_EnqueueRequest (0x3dfd58)

/// StreamFile: its mutex (+4) and the request list: head (+0x5c), current
/// (+0x60) and tail (+0x64).
pub(crate) const SFQ_MUTEX: u32 = 0x4;
pub(crate) const SFQ_HEAD: u32 = 0x5c;
pub(crate) const SFQ_CURRENT: u32 = 0x60;
pub(crate) const SFQ_TAIL: u32 = 0x64;
/// Request: state (+4), previous (+8), next (+0xc).
pub(crate) const SFQ_REQ_STATE: u32 = 0x4;
pub(crate) const SFQ_REQ_PREV: u32 = 0x8;
pub(crate) const SFQ_REQ_NEXT: u32 = 0xc;

const SFQ_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const SFQ_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `void StreamFile_EnqueueRequest(StreamFile* sf, Request* req)`
/// (0x3dfd58): mark the request queued (state 1, no next) and append it to
/// the stream file's doubly linked request list under the file's mutex. An
/// empty list makes it head, current and tail at once.
pub fn stream_file_enqueue_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let (sf, req) = (Addr(c.g[S1][0] as u32), Addr(c.g[S0][0] as u32));
        req.off(SFQ_REQ_NEXT).sw(c, 0);
        c.g[A0][0] = sx(sf.off(SFQ_MUTEX).0);
        req.off(SFQ_REQ_STATE).sw(c, 1);
        SFQ_MUTEX_LOCK.call(c, 0x003d_fd84)?;

        let (sf, req) = (Addr(c.g[S1][0] as u32), Addr(c.g[S0][0] as u32));
        c.g[V0][0] = sf.off(SFQ_TAIL).lw(c);
        if c.g[V0][0] != 0 {
            // Append after the tail.
            let tail = c.g[V0][0];
            req.off(SFQ_REQ_PREV).sw(c, tail);
            c.cycles += 3;
            c.g[V0][0] = sf.off(SFQ_TAIL).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(SFQ_REQ_NEXT)).sw(c, req.0 as u64);
            sf.off(SFQ_TAIL).sw(c, req.0 as u64);
        } else {
            c.cycles += 5;
            req.off(SFQ_REQ_PREV).sw(c, 0);
            sf.off(SFQ_TAIL).sw(c, req.0 as u64);
            sf.off(SFQ_HEAD).sw(c, req.0 as u64);
            sf.off(SFQ_CURRENT).sw(c, req.0 as u64);
        }

        c.cycles += 7;
        c.g[A0][0] = sx(sf.off(SFQ_MUTEX).0);
        SFQ_MUTEX_UNLOCK.call(c, 0x003d_fdb8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_CheckEventMasks (0x3d9ed0)

/// Speech sentence header: byte 6 (length of the leading word table in
/// halfwords), byte 7 (high nibble: signed count of event conditions; low
/// nibble: number of 3-byte entries before the conditions).
pub(crate) const SPCH_SENT_B6: u32 = 0x6;
pub(crate) const SPCH_SENT_B7: u32 = 0x7;
/// The event block's byte 4: size of its header in quarter words.
pub(crate) const SPCH_EVENT_B4: u32 = 0x4;

/// `bool SPCH_CheckEventMasks(Sentence* s, EventBlock* ev, u32* masks)`
/// (0x3d9ed0): whether every event condition of a speech sentence holds.
/// Condition `i` is a 3-byte record `{a, slot, b}` in the sentence (after the
/// word table and the entries counted by byte 7's low nibble) paired with
/// the mask word `i` of the event block (after its header). A zero mask
/// always holds; otherwise `masks[slot] & mask` must be non-zero. The first
/// failing condition stops the scan; the scan's last record (`a`, `slot`,
/// `b`) is spilled to the frame (dead stores the original makes).
pub fn spch_check_event_masks(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A3][0] = c.g[A0][0];
    let frame = Frame::open(c, 0x10);
    let sent = Addr(c.g[A3][0] as u32);
    let b7 = sent.off(SPCH_SENT_B7).lbu(c);
    c.g[A0][0] = b7;
    c.g[T0][0] = c.g[A1][0];
    c.g[T5][0] = c.g[A2][0];
    c.g[V0][0] = sx((b7 as u32) << 24);
    // The high nibble as a signed count.
    let count = (((b7 as u32) << 24) as i32 >> 28) as u32;
    c.g[T1][0] = sx(count);
    c.g[T4][0] = 1;
    if count != 0 {
        c.cycles += 22;
        let skip = b7 & 0xf;
        c.g[A2][0] = skip;
        let b6 = sent.off(SPCH_SENT_B6).lbu(c);
        let ev = c.g[T0][0] as u32;
        let b4 = Addr(ev.wrapping_add(SPCH_EVENT_B4)).lbu(c) as u32;
        // Word table, rounded to a word, after the 0xc-byte header.
        let words = (((b6 as u32) << 1) + 3) & !3;
        let table_end = sent.0.wrapping_add(words + 0xc);
        let masks_off = (((b4 >> 2) + 3) & !3) + 8;
        c.g[A1][0] = sx(ev.wrapping_add(masks_off));
        let records = table_end.wrapping_add((skip as u32 * 3 + 3) & 0x7c);
        c.g[V1][0] = sx(table_end);
        c.g[A0][0] = sx(masks_off);
        c.g[V0][0] = sx(records);
        c.g[A3][0] = 0;
        if (count as i32) > 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
            // a0: condition record, a1: its mask word, a3: conditions held.
            loop {
                c.cycles += 5;
                let rec = c.g[A0][0] as u32;
                let mask = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A2][0] = mask;
                c.g[T3][0] = Addr(rec).lbu(c);
                c.g[T0][0] = Addr(rec.wrapping_add(1)).lbu(c);
                c.g[T2][0] = Addr(rec.wrapping_add(2)).lbu(c);
                if mask == 0 {
                    c.cycles += 1;
                } else {
                    c.cycles += 6;
                    let at = ((c.g[T0][0] as u32) << 2).wrapping_add(c.g[T5][0] as u32);
                    c.g[V0][0] = sx(at);
                    c.g[V1][0] = Addr(at).lw(c) & mask;
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[T4][0] = 0;
                        break;
                    }
                }
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                let more = (c.g[A3][0] as i64) < (c.g[T1][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A0][0] = sx(rec.wrapping_add(3));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_9f50);
                }
            }
            c.cycles += 3;
            let (t2, t0, t3) = (c.g[T2][0], c.g[T0][0], c.g[T3][0]);
            Addr(frame.at(c, 8)).sw(c, t2);
            Addr(frame.at(c, 4)).sw(c, t0);
            Addr(frame.at(c, 0)).sw(c, t3);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T4][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BENewPlayerInt_GetAltCharacter (0x147410)

/// The player records (0x1c bytes each) and the alternate-character byte.
pub(crate) const BENP_PLAYERS: u32 = 0x0053_4fe0;
pub(crate) const BENP_PLAYER_SIZE: u32 = 0x1c;
pub(crate) const BENP_PLAYER_ALT_CHARACTER: u32 = 0x12;
/// The slot table `BENewPlayerInt_GetSlot` reads (`0x5305b0[index]`).
pub(crate) const BENP_SLOTS: u32 = 0x0053_05b0;

const BENP_GET_SLOT: Callee = Callee {
    addr: 0x0014_7798,
    f: ee::s01::BENewPlayerInt_GetSlot,
};

/// `s8 BENewPlayerInt_GetAltCharacter(BENewPlayerInt* self, int index)`
/// (0x147410): the alternate character (signed byte +0x12) of the player
/// record the player in slot `index` uses.
pub fn be_new_player_int_get_alt_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        BENP_GET_SLOT.call(c, 0x0014_7420)?;
        c.g[A0][0] = BENP_PLAYER_SIZE as u64;
        c.g[V1][0] = sx(BENP_PLAYERS);
        let slot = c.g[V0][0] as u32;
        let off = abi::mult(c, slot, BENP_PLAYER_SIZE);
        frame.restore_ra(c, 0);
        let rec = off.wrapping_add(BENP_PLAYERS);
        c.g[V0][0] = Addr(rec.wrapping_add(BENP_PLAYER_ALT_CHARACTER)).lb(c);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyLowpass (0x3c3178)

/// The sound system state (`0x50a8e8`): the channel array (+0x1f0), the
/// output rate (+0x2a, u16) and the two voice index bases (+0x2f, +0x30).
pub(crate) const SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const SND_STATE_CHANNELS: u32 = 0x1f0;
pub(crate) const SND_STATE_RATE: u32 = 0x2a;
pub(crate) const SND_STATE_VOICE_BASE_A: u32 = 0x2f;
pub(crate) const SND_STATE_VOICE_BASE_B: u32 = 0x30;
/// Channel (0x8c bytes): its voices (+4, s16 each), voice count (+0x23),
/// flags (+0x24; bit 2: filtered).
pub(crate) const SND_CHANNEL_SIZE: u32 = 0x8c;
pub(crate) const SND_CHANNEL_VOICES: u32 = 0x4;
pub(crate) const SND_CHANNEL_VOICE_COUNT: u32 = 0x23;
pub(crate) const SND_CHANNEL_FLAGS: u32 = 0x24;

/// `void SND_ChannelApplyLowpass(int channel, int cutoff)` (0x3c3178): for
/// a filtered channel (flags bit 2), set every voice's lowpass to
/// `cutoff / (rate * 0.5)` (`SND_EeVoiceSetLowpass(voice - bases, f)`).
pub fn snd_channel_apply_lowpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = SND_CHANNEL_SIZE as u64;
        frame.save(c, S2, 0x20);
        let channel = c.g[A0][0] as u32;
        let off = abi::mult(c, channel, SND_CHANNEL_SIZE);
        c.g[A0][0] = sx(off);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x50);
        let snd = Addr(SND_STATE);
        c.g[V1][0] = sx(snd.0);
        c.g[A2][0] = snd.off(SND_STATE_CHANNELS).lw(c);
        let chan = Addr((c.g[A2][0] as u32).wrapping_add(off));
        c.g[S2][0] = sx(chan.0);
        let flags = chan.off(SND_CHANNEL_FLAGS).lhu(c);
        c.g[V0][0] = flags & 4;
        c.g[S1][0] = 0;
        if flags & 4 != 0 {
            c.cycles += 14;
            let rate = snd.off(SND_STATE_RATE).lhu(c);
            c.g[V0][0] = rate;
            let amount = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
            c.f[3] = amount.0;
            let half = FpuF32(0x3f00_0000);
            c.f[2] = half.0;
            let rate_f = FpuF32::from_i32(rate as u32 as i32);
            c.g[AT][0] = FpuF32::ONE.0 as u64;
            c.f[1] = FpuF32::ONE.0;
            let count = chan.off(SND_CHANNEL_VOICE_COUNT).lbu(c);
            c.g[V0][0] = count;
            let nyquist = rate_f * half;
            c.f[0] = nyquist.0;
            let inv = FpuF32::ONE.div(nyquist);
            c.f[1] = inv.0;
            c.f[20] = (amount * inv).0;
            if count != 0 {
                c.cycles += 3;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(chan.off(SND_CHANNEL_VOICES).0);
                c.g[V0][0] = snd.off(SND_STATE_VOICE_BASE_A).lbu(c);
                // s0: next voice, s1: voices done; v0 holds base A.
                loop {
                    c.cycles += 12;
                    c.f[12] = c.f[20];
                    let state = c.g[S3][0] as u32;
                    let base_b = Addr(state.wrapping_add(SND_STATE_VOICE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    let at = c.g[S0][0] as u32;
                    c.g[V1][0] = Addr(at).lh(c);
                    let bases = (base_b as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[S0][0] = sx(at.wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(bases));
                    call_ptr(c, 0x003c_8d00, 0x003c_3220)?;
                    let chan = c.g[S2][0] as u32;
                    let count = Addr(chan.wrapping_add(SND_CHANNEL_VOICE_COUNT)).lbu(c);
                    let more = (c.g[S1][0] as i64) < (count as i64);
                    c.g[V0][0] = more as u64;
                    if !more {
                        break;
                    }
                    let state = c.g[S3][0] as u32;
                    c.g[V0][0] = Addr(state.wrapping_add(SND_STATE_VOICE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3200);
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetHighpass (0x3c8c30)

/// The EE voice table pointer; a voice entry is 0x60 bytes: DSP chain head
/// at +0x40, its highpass (FIR) DSP node at +0x5c.
pub(crate) const HIGHPASS_VOICES: u32 = 0x0051_5d1c;
pub(crate) const HIGHPASS_VOICE_SIZE: u32 = 0x60;
pub(crate) const HIGHPASS_VOICE_DSP_CHAIN: u32 = 0x40;
pub(crate) const HIGHPASS_VOICE_FILTER: u32 = 0x5c;
/// The DSP node: its size (0x120), priority (+0x18, 0x50).
const HIGHPASS_NODE_SIZE: u64 = 0x120;
const HIGHPASS_PRIORITY: u64 = 0x50;

const HIGHPASS_INIT: Callee = Callee {
    addr: 0x003c_a990,
    f: ee::s14::SND_DspFirHighpass_Init,
};
const HIGHPASS_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};
const HIGHPASS_SET_CUTOFF: Callee = Callee {
    addr: 0x003c_a9d8,
    f: ee::s14::SND_DspFirHighpass_SetCutoff,
};
const HIGHPASS_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `void SND_EeVoiceSetHighpass(int voice, int cutoff)` (0x3c8c30): a
/// positive cutoff (Hz) gives the voice a FIR highpass DSP (created on
/// first use: `SND_MemAlloc(0x120)`, priority 0x50,
/// `SND_DspFirHighpass_Init`, inserted into the voice's DSP chain) and sets
/// it to `{cutoff << 8, rate << 8}`; zero or negative removes and frees the
/// voice's highpass DSP, if any. Frame 0x40 (s0 = voice entry, s1 =
/// cutoff); the entry offset uses `mult`.
pub fn snd_ee_voice_set_highpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = HIGHPASS_VOICE_SIZE as u64;
        c.g[V1][0] = 0x51_0000;
        let voice_index = c.g[A0][0] as u32;
        let off = abi::mult(c, voice_index, HIGHPASS_VOICE_SIZE);
        c.g[A0][0] = sx(off);
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = Addr(HIGHPASS_VOICES).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(off));
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[V0][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            let present = c.g[V0][0] != 0;
            c.g[V0][0] = 0x51_0000;
            if !present {
                // Create the highpass DSP and insert it into the chain.
                c.cycles += 15;
                c.g[A0][0] = HIGHPASS_NODE_SIZE;
                call_ptr(c, 0x003b_5c60, 0x003c_8c70)?; // SND_MemAlloc
                let filter = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER));
                let dsp = c.g[V0][0];
                filter.sw(c, dsp);
                c.g[A0][0] = HIGHPASS_PRIORITY;
                Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
                c.g[V0][0] = filter.lw(c);
                Addr(r32(c, V0).wrapping_add(0x18)).sh(c, HIGHPASS_PRIORITY);
                c.g[V1][0] = filter.lw(c);
                Addr(r32(c, V1).wrapping_add(0x1a)).sb(c, 0);
                c.g[A0][0] = filter.lw(c);
                HIGHPASS_INIT.call(c, 0x003c_8c94)?;
                let voice = r32(c, S0);
                c.g[A1][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
                c.g[A0][0] = sx(voice.wrapping_add(HIGHPASS_VOICE_DSP_CHAIN));
                HIGHPASS_INSERT_SORTED.call(c, 0x003c_8ca0)?;
                c.g[V0][0] = 0x51_0000;
            }
            // params = { cutoff << 8, rate << 8 } on the stack.
            c.cycles += 10;
            let cutoff = sx(r32(c, S1) << 8);
            c.g[A0][0] = cutoff;
            let rate = Addr(SND_STATE + SND_STATE_RATE).lhu(c);
            let params = sp(c);
            c.g[A1][0] = c.g[SP][0];
            Addr(params).sw(c, cutoff);
            c.g[V1][0] = sx((rate as u32) << 8);
            let rate_word = c.g[V1][0];
            Addr(params + 4).sw(c, rate_word);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            HIGHPASS_SET_CUTOFF.call(c, 0x003c_8cc4)?;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[A1][0] = Addr(voice.wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
            if c.g[A1][0] != 0 {
                // Remove the highpass DSP from the chain and free it.
                c.cycles += 6;
                c.g[A0][0] = sx(voice.wrapping_add(HIGHPASS_VOICE_DSP_CHAIN));
                HIGHPASS_DSP_REMOVE.call(c, 0x003c_8ce0)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).lw(c);
                call_ptr(c, 0x003b_5e98, 0x003c_8ce8)?; // SND_MemFree
                Addr(r32(c, S0).wrapping_add(HIGHPASS_VOICE_FILTER)).sw(c, 0);
            }
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_StartObjectLoop (0x3000a8)

/// `gp` word: the static parameter defaults are initialised.
pub(crate) const SOL_DEFAULTS_READY_GP: u32 = 0x2578;
/// The static parameter defaults (object id, sound, volume?, tag byte at
/// +0xc; 0x4fbb18), copied as a 16-byte block.
pub(crate) const SOL_DEFAULTS: u32 = 0x004f_bb18;
/// The kind of each parameter slot (2 = float).
pub(crate) const SOL_PARAM_KINDS: u32 = 0x0044_6550;
pub(crate) const SOL_KIND_FLOAT: u64 = 2;
/// `gp` words: the object directory, the world (its default object at
/// `+0x290`).
pub(crate) const SOL_DIRECTORY_GP: u32 = 0x16c8;
pub(crate) const SOL_WORLD_GP: u32 = 0xce8;
pub(crate) const SOL_WORLD_DEFAULT_OBJECT: u32 = 0x290;
/// The script functions' shared result value (12 bytes at 0x4c9098).
pub(crate) const SOL_RESULT: u32 = 0x004c_9098;

/// `Result WScriptFn_StartObjectLoop(Param* params, int count)` (0x3000a8;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 4-word
/// parameter block (object id, sound, arg) from the defaults and the
/// `{slot, value, -, kind}` parameters (an int given for a float slot is
/// converted), looks the object up (id -1: the world's default object) and,
/// if there is one, starts the looping sound on it
/// (`SSXAudioSystem_StartObjectLoop(GetInstance(), obj, sound, arg)`).
/// Returns the shared script result.
pub fn wscript_fn_start_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        let ready = Addr(gp).off(SOL_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0x50_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let defaults = Addr(SOL_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 12;
            c.g[V1][0] = sx(SOL_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(0xc).lbu(c);
            c.g[A2][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = u64::MAX;
            defaults.off(0xc).sb(c, tag);
            c.g[A1][0] = 1;
            defaults.off(4).sw(c, u64::MAX);
            defaults.off(8).sw(c, u64::MAX);
            Addr(gp).off(SOL_DEFAULTS_READY_GP).sw(c, 1);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S0][0] = sx(block);
        c.g[A2][0] = 0x10;
        c.g[A1][0] = sx(SOL_DEFAULTS);
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        call_ptr(c, 0x003e_6574, 0x0030_0128)?; // memcpy
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(SOL_PARAM_KINDS);
            c.g[A3][0] = SOL_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S0)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_0140);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(r32(c, GP)).off(SOL_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(r32(c, GP)).off(SOL_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S2][0] = Addr(world as u32).off(SOL_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            let sections = Addr(directory as u32).lw(c);
            c.g[V1][0] = sections;
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A1][0] = object;
            c.cycles += 1;
            c.g[S2][0] = object;
        }
        c.cycles += 2;
        c.g[V1][0] = 0x4d_0000;
        if c.g[S2][0] != 0 {
            // SSXAudioSystem_StartObjectLoop(GetInstance(), obj, sound, arg).
            c.cycles += 9;
            c.g[S1][0] = Addr(r32(c, S0)).off(8).lw(c);
            c.g[S0][0] = Addr(r32(c, S0)).off(4).lw(c);
            call_ptr(c, 0x0028_b180, 0x0030_0208)?; // SSXAudioSystem_GetInstance
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, 0x0029_7950, 0x0030_021c)?; // SSXAudioSystem_StartObjectLoop
            c.g[V1][0] = 0x4d_0000;
        }
        // *result = the shared script result (12 bytes).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(SOL_RESULT);
        let out = r32(c, S3);
        c.ldl(4, SOL_RESULT.wrapping_add(0x7));
        c.ldr(4, SOL_RESULT);
        c.g[A1][0] = Addr(SOL_RESULT).off(8).lw(c);
        c.sdl(4, out.wrapping_add(0x7));
        c.sdr(4, out);
        let tail = c.g[A1][0];
        Addr(out).off(8).sw(c, tail);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// SND_InitPatchParams on a block of random bytes; event seeds deliver
    /// (or unwind at) the back edges of both clearing loops.
    #[test]
    fn snd_init_patch_params_matches() {
        check_events(ee::s13::f_003c7218__generated, snd_init_patch_params, 120, &|c, seed| {
            let mut r = Rng(0x3c72_1800 ^ seed as u64);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x20);
            let hi = if seed % 4 == 0 { (r.next_u32() as u64) << 32 } else { 0 };
            c.g[A0][0] = hi | DATA as u64;
            let mode = schedule(c, &mut r, seed, 0x003c_7218);
            if mode != Events::Off && seed % 2 == 0 {
                // Due late enough to land in the second loop.
                c.next_event = c.cycles + 14 + 28 + (r.next_u32() % 40) as u64;
            }
            mode
        });
    }

    /// ComputerActorRider_RollUberTrick: uber tricks locked or unlocked,
    /// track 0xe or another, chances from 0 to 1 (and edge floats); real
    /// BELibrary_GetInstance/GetInterface/GetTrack/Rand_NextGame, the race
    /// interface's method a fake (sometimes unwinding).
    #[test]
    fn computer_actor_rider_roll_uber_trick_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::GAME_GP;
        covered(
            ee::s00::f_0010c258__generated,
            computer_actor_rider_roll_uber_trick,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10c258, 1);
                c.pc = 0x0010_c258;
                let (me, rider, lib, iface, vt) =
                    (DATA, DATA + 0x2000, DATA + 0x4000, DATA + 0x5000, DATA + 0x5100);
                c.g[A0][0] = me as u64;
                c.bus.write32(me + CAR_RIDER, rider);
                let flags = r.next_u32() & !1 | (seed % 4 != 0) as u32;
                c.bus.write32(rider + CAR_RIDER_ABILITY_FLAGS, flags);
                let chance = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    (r.next_u32() as f32 / u32::MAX as f32).to_bits()
                };
                c.bus.write32(me + CAR_TRICK_SKILL, chance);
                // BELibrary built; interface 0's GetTrack-provider method is a fake.
                c.bus.write32(GAME_GP - 0x1f04, lib);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                // BENewRaceInt's track.
                let track = if seed % 3 == 0 { 0xe } else { r.next_u32() % 0x20 };
                c.bus.write32(0x0053_5c08, track);
                // Rand_NextGame's generator state.
                fill(c, &mut r, 0x0050_0000 - 0xfd0, 8);
            },
        );
    }

    /// StreamFile_EnqueueRequest into an empty list or after a tail; the
    /// real EAMutex lock / unlock (syscalls faked, some unwinding), the
    /// mutex owned by the caller or not.
    #[test]
    fn stream_file_enqueue_request_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::put_words;
        covered(
            ee::s14::f_003dfd58__generated,
            stream_file_enqueue_request,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_fd58, 3);
                c.pc = 0x003d_fd58;
                let (sf, req, tail) = (DATA, DATA + 0x1000, DATA + 0x2000);
                fill(c, &mut r, sf, 0x20);
                fill(c, &mut r, req, 4);
                fill(c, &mut r, tail, 4);
                c.bus.write32(sf + SFQ_TAIL, if seed % 3 == 0 { 0 } else { tail });
                // The mutex: owner thread id (+4), lock count (+8), semaphore (+0xc).
                put_words(
                    c,
                    sf + SFQ_MUTEX + 4,
                    &[r.next_u32() % 4, r.next_u32() % 3, r.next_u32() % 16],
                );
                c.g[A0][0] = sx(sf);
                c.g[A1][0] = sx(req);
            },
        );
    }

    /// SPCH_CheckEventMasks: condition counts -8..7 (none, negative, some),
    /// random table/entry sizes, zero masks, all-matching or sparse slot
    /// masks (an early failure); event seeds deliver at the loop's back edge.
    #[test]
    fn spch_check_event_masks_matches() {
        check_events(ee::s14::f_003d9ed0__generated, spch_check_event_masks, 200, &|c, seed| {
            let mut r = Rng(0x3d9e_d000 ^ seed as u64);
            scramble(c, &mut r);
            let (sent, ev, slots) = (DATA, DATA + 0x1000, DATA + 0x2000);
            fill(c, &mut r, sent, 0x100);
            fill(c, &mut r, ev, 0x40);
            // Event seeds (seed % 3 != 0, see `schedule`) mostly get 4..7
            // conditions that all hold, so the loop runs its back edges.
            let evented = seed % 3 != 0 && seed % 5 != 0;
            let count = if evented {
                4 + r.next_u32() % 4
            } else {
                match seed % 4 {
                    0 => 0,
                    1 => 8 + r.next_u32() % 8,
                    _ => 1 + r.next_u32() % 7,
                }
            };
            c.bus.write8(sent + SPCH_SENT_B7, (count << 4 | r.next_u32() & 0xf) as u8);
            let b4 = c.bus.read8(ev + SPCH_EVENT_B4) as u32;
            let masks = ev + ((((b4 >> 2) + 3) & !3) + 8);
            for i in 0..8 {
                let m = match r.next_u32() % 3 {
                    0 => 0,
                    _ => 1 << (r.next_u32() % 32),
                };
                c.bus.write32(masks + 4 * i, m);
            }
            let all = seed % 3 == 0 || evented;
            for i in 0..0x100 {
                let w = if all { u32::MAX } else { r.next_u32() | r.next_u32() };
                c.bus.write32(slots + 4 * i, w);
            }
            c.g[A0][0] = sx(sent);
            c.g[A1][0] = sx(ev);
            c.g[A2][0] = sx(slots);
            let mode = schedule(c, &mut r, seed, 0x003d_9ed0);
            if mode != Events::Off {
                // Due by the first or second back edge (entry 9 + 22 + 1,
                // then 5..13 + 4 cycles per condition).
                c.next_event = c.cycles + 40 + (r.next_u32() % 12) as u64;
            }
            mode
        });
    }

    /// BENewPlayerInt_GetAltCharacter: slot indices 0..7 whose players use
    /// records 0..15 (alt characters of either sign), through the real
    /// BENewPlayerInt_GetSlot.
    #[test]
    fn be_new_player_int_get_alt_character_matches() {
        crate::difftest::check_lohi(ee::s01::f_00147410__generated, be_new_player_int_get_alt_character, 200, &|c, seed| {
            let mut r = Rng(0x0014_7410 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0014_7410;
            for i in 0..8 {
                c.bus.write32(BENP_SLOTS + 4 * i, r.next_u32() % 16);
            }
            fill(c, &mut r, BENP_PLAYERS, 16 * BENP_PLAYER_SIZE / 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (r.next_u32() % 8) as u64 | (r.next_u32() as u64) << 32;
        });
    }

    /// ComputerActorRider_RollFittedTrick: chances from 0 to 1 (and edge
    /// floats) against the real Rand_NextGame on random generator states.
    #[test]
    fn computer_actor_rider_roll_fitted_trick_matches() {
        crate::difftest::check_lohi(ee::s00::f_0010c320__generated, computer_actor_rider_roll_fitted_trick, 300, &|c, seed| {
            let mut r = Rng(0x0010_c320 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0010_c320;
            let chance = if seed % 4 == 0 {
                r.edge_f32()
            } else {
                (r.next_u32() as f32 / u32::MAX as f32).to_bits()
            };
            c.bus.write32(DATA + CAR_TRICK_SKILL, chance);
            fill(c, &mut r, 0x0050_0000 - 0xfd0, 8);
            c.g[A0][0] = sx(DATA);
        });
    }

    /// SND_ChannelApplyLowpass: channels filtered or not, 0..12 voices,
    /// random voice numbers and index bases; cutoffs at or above half the
    /// rate, so the real SND_EeVoiceSetLowpass takes its "filter off, none
    /// installed" short path (the voice array is empty RAM). Event seeds
    /// deliver at the loop's back edge.
    #[test]
    fn snd_channel_apply_lowpass_matches() {
        check_events(ee::s13::f_003c3178__generated, snd_channel_apply_lowpass, 150, &|c, seed| {
            let mut r = Rng(0x003c_3178 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let (chans, voices) = (DATA, DATA + 0x4_0000);
            c.bus.write32(SND_STATE + SND_STATE_CHANNELS, chans);
            c.bus.write32(0x0051_5d1c, voices);
            let rate = r.next_u32() % 0xc000;
            c.bus.write16(SND_STATE + SND_STATE_RATE, rate as u16);
            c.bus.write8(SND_STATE + SND_STATE_VOICE_BASE_A, (r.next_u32() % 16) as u8);
            c.bus.write8(SND_STATE + SND_STATE_VOICE_BASE_B, (r.next_u32() % 16) as u8);
            let index = r.next_u32() % 4;
            let chan = chans + index * SND_CHANNEL_SIZE;
            fill(c, &mut r, chan, SND_CHANNEL_SIZE / 4);
            let flags = r.next_u32() as u16 & !4 | if seed % 5 == 0 { 0 } else { 4 };
            c.bus.write16(chan + SND_CHANNEL_FLAGS, flags);
            let count = if seed % 7 == 0 { 0 } else { 1 + r.next_u32() % 12 };
            c.bus.write8(chan + SND_CHANNEL_VOICE_COUNT, count as u8);
            for i in 0..12 {
                c.bus.write16(chan + SND_CHANNEL_VOICES + 2 * i, (r.next_u32() % 48) as u16);
            }
            c.g[A0][0] = index as u64 | (r.next_u32() as u64) << 32;
            c.g[A1][0] = (rate / 2 + 1 + r.next_u32() % 0x10000) as u64;
            let mode = schedule(c, &mut r, seed, 0x003c_3178);
            if mode != Events::Off {
                c.next_event = c.cycles + 40 + (r.next_u32() % 200) as u64;
            }
            mode
        });
    }

    /// `SND_EeVoiceSetHighpass`: positive cutoffs with the voice's highpass
    /// DSP present or created by the real `SND_MemAlloc` from an empty heap
    /// and inserted into a 0-3 node chain, then the real FIR design; zero or
    /// negative cutoffs with a DSP to remove (the real `SND_DspRemove`,
    /// `SND_MemFree` on an empty heap) or none.
    #[test]
    fn snd_ee_voice_set_highpass_matches() {
        use crate::difftest::fakes::machine;
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        crate::difftest::check_lohi(
            ee::s14::f_003c8c30__generated,
            snd_ee_voice_set_highpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8c30, 0);
                c.pc = 0x003c_8c30;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.bus.write32(HIGHPASS_VOICES, VOICES);
                fill(c, &mut r, VOICES, HIGHPASS_VOICE_SIZE * 8 / 4);
                let voice = VOICES + HIGHPASS_VOICE_SIZE * index;
                // The DSP chain: 0-3 nodes {next +8, priority +0x18} (room
                // for a whole FIR node each); the highpass DSP (when
                // present) is one of them or not.
                let n = seed / 8 % 4;
                for i in 0..n {
                    let node = NODES + 0x200 * i;
                    fill(c, &mut r, node, 0x48);
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x200 });
                    c.bus
                        .write16(node + 0x18, (0x10 + 0x40 * i + r.next_u32() % 0x40) as u16);
                }
                c.bus.write32(
                    voice + HIGHPASS_VOICE_DSP_CHAIN,
                    if n == 0 { 0 } else { NODES },
                );
                let filter = match seed % 3 {
                    0 => 0,
                    1 if n > 0 => NODES + 0x200 * (r.next_u32() % n),
                    _ => NODES + 0x800,
                };
                if filter == NODES + 0x800 {
                    fill(c, &mut r, filter, 0x48);
                }
                c.bus.write32(voice + HIGHPASS_VOICE_FILTER, filter);
                // The driver heap: empty, room for the block.
                for state in [0x0051_5b40u32, SND_STATE] {
                    c.bus.write32(state + 0x234, HEAP);
                }
                c.bus.write32(HEAP, DATA + 0x20_4000);
                c.bus.write32(HEAP + 0x4, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0xc, 0x8_0000);
                c.bus.write32(HEAP + 0x10, 0x100);
                c.bus.write32(HEAP + 0x14, 0);
                c.bus.write16(
                    SND_STATE + SND_STATE_RATE,
                    [48000u32, 44100, 24000, r.next_u32() % 0x1_0000][(seed / 3 % 4) as usize] as u16,
                );
                c.g[A1][0] = match seed / 5 % 5 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    2 => (r.next_u32() as u64) << 32 | (r.next_u32() % 20000) as u64,
                    _ => 20 + (r.next_u32() % 20000) as u64,
                };
            },
        );
    }

    /// `WScriptFn_StartObjectLoop`: defaults initialised or not (default id
    /// -1 or random), 0..3 parameters (or a negative count) of matching,
    /// mismatching and int-for-float kinds, object ids -1 (the world's
    /// default object, present or not), sections present or empty, entries
    /// with and without an object; when an object is found, the real
    /// `GetInstance` and `StartObjectLoop` (sound 0, or object sounds
    /// suppressed: its early exits).
    #[test]
    fn wscript_fn_start_object_loop_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const AUDIO: u32 = DATA + 0x8000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const OBJS: u32 = DIR + 0x2000;
        check(ee::s10::f_003000a8__generated, wscript_fn_start_object_loop, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x30_00a8, 0);
            c.pc = 0x0030_00a8;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Static defaults.
            let ready = seed % 4 != 0;
            w(c, GAME_GP + SOL_DEFAULTS_READY_GP, ready as u32);
            fill(c, &mut r, SOL_DEFAULTS, 4);
            let id = match seed % 5 {
                0 => u32::MAX,
                k => (r.next_u32() % 4) << 8 | (k - 1),
            };
            w(c, SOL_DEFAULTS, id);
            // Sound 0 or a non-0x66 one (suppressed below).
            let sound = if seed % 3 == 0 { 0 } else { 1 + r.next_u32() % 0x60 };
            w(c, SOL_DEFAULTS + 4, sound);
            for i in 0..4 {
                w(c, SOL_PARAM_KINDS + 4 * i, r.next_u32() % 4);
            }
            // Parameters: never slot 0 (the id) or a 0x66 sound.
            let count = match seed % 7 {
                6 => (r.next_u32() % 3).wrapping_neg(),
                k => k % 4,
            };
            for i in 0..4u32 {
                let p = PARAMS + 0x10 * i;
                w(c, p, 1 + r.next_u32() % 3);
                w(c, p + 4, 1 + r.next_u32() % 0x60);
                w(c, p + 8, r.next_u32());
                w(c, p + 0xc, r.next_u32() % 4);
            }
            // Directory: sections 0..2 present, 3 empty; entries 0..2
            // objects, 3 empty.
            w(c, GAME_GP + SOL_DIRECTORY_GP, DIR);
            w(c, DIR, SECTIONS);
            w(c, SECTIONS + 8, TABLE);
            for i in 0..4 {
                w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                let obj = OBJS + 0x100 * i;
                w(
                    c,
                    ENTRIES + 4 * i,
                    if i == 3 {
                        r.next_u32() & 0xff
                    } else {
                        (obj >> 2) << 8 | (r.next_u32() & 0xff)
                    },
                );
                fill(c, &mut r, obj, 0x20);
            }
            w(c, SECTION + 0x1c, ENTRIES);
            w(c, GAME_GP + SOL_WORLD_GP, WORLD);
            w(
                c,
                WORLD + SOL_WORLD_DEFAULT_OBJECT,
                if seed % 10 == 5 { 0 } else { OBJS + 0x100 },
            );
            // The audio system: object sounds suppressed (non-negative).
            w(c, GAME_GP + 0x410, AUDIO);
            w(c, AUDIO + 0x62a8, r.next_u32() & 0x7fff_ffff);
            fill(c, &mut r, SOL_RESULT, 3);
            c.g[A0][0] = sx(RESULT_SLOT);
            c.g[A1][0] = count as i32 as i64 as u64;
            c.g[A2][0] = sx(PARAMS);
        });
    }
}
