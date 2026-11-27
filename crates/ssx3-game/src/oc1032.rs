//! Staging module oc1032 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1032`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::sound::bankmanager::{
    HMT_ADD_TIMER, HMT_FIND_RIDER, HMT_MUSIC, HMT_MUSIC_VT, HMT_PAUSED, HMT_START_NEXT_SONG,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_d630,
        ssxaudio_start_music_trigger,
        "SSXAudioSystem_StartMusicTrigger",
    ),
    (
        0x0028_d7d8,
        ssxaudio_clear_music_trigger,
        "SSXAudioSystem_ClearMusicTrigger",
    ),
    (
        0x0028_d898,
        ssxaudio_get_music_trigger,
        "SSXAudioSystem_GetMusicTrigger",
    ),
    (
        0x0028_df18,
        ssxaudio_start_next_song,
        "SSXAudioSystem_StartNextSong",
    ),
    (
        0x0028_e068,
        ssxaudio_next_song_timer_callback,
        "SSXAudioSystem_NextSongTimerCallback",
    ),
    (
        0x0028_e088,
        ssxaudio_song_end_callback,
        "SSXAudioSystem_SongEndCallback",
    ),
    (
        0x0028_e510,
        ssxaudio_music_event_timer_callback,
        "SSXAudioSystem_MusicEventTimerCallback",
    ),
    (0x0028_fc38, ssxaudio_stop_music, "SSXAudioSystem_StopMusic"),
];

// SSXAudioSystem fields used by the music-trigger functions (names from the
// C++ reference, bankmanager.cpp).

/// `mInBigChallenge`: a big challenge runs, no music trigger starts.
pub(crate) const MT_IN_BIG_CHALLENGE: u32 = 0x5fd4;
/// `mBigChallengeStarting`.
pub(crate) const MT_BIG_CHALLENGE_STARTING: u32 = 0x5fd8;
/// `mSongInstance`: the song playing now (0 when none).
pub(crate) const MT_SONG_INSTANCE: u32 = 0x520;
/// `unk_623C`: the cue id the music trigger last started.
pub(crate) const MT_CUE_ID: u32 = 0x623c;
/// `unk_6240`: cleared together with the cue.
pub(crate) const MT_CUE_EXTRA: u32 = 0x6240;
/// `mMusicEventOwner`: AI index of the rider that owns the music event (-1: none).
pub(crate) const MT_MUSIC_OWNER: u32 = 0x62a0;
/// `mMusicRequestPending`: a music request waits for the song to end.
pub(crate) const MT_REQUEST_PENDING: u32 = 0x6274;
/// `mMusicRequest`: the cue the waiting request starts.
pub(crate) const MT_REQUEST: u32 = 0x6278;
/// Music event data: `mData1` (the cue) and `mData2` (the flag) of the event.
pub(crate) const MT_EVENT_DATA1: u32 = 0x1c;
pub(crate) const MT_EVENT_DATA2: u32 = 0x20;

/// `SSXAudioSystem_SelectRandomSong` (0x28d488).
pub(crate) const MT_SELECT_RANDOM_SONG: Callee = Callee {
    addr: 0x0028_d488,
    f: ee::s07::f_0028d488,
};

/// How a music-trigger body leaves: `Done` restores s0 after the extra
/// one-cycle block of the normal exit, `Skip` jumps straight to the restore.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TriggerExit {
    Done,
    Skip,
}

/// `void SSXAudioSystem_StartMusicTrigger(SSXAudioSystem* self, int id, Rider* obj)`
/// (0x28d630): starts music cue `id` for the rider `obj`, unless a big
/// challenge is running or the cue already plays. The rider must own the
/// music event, or claim it when nobody does and it is the focus rider. The
/// playing cue is stopped first, then the new one starts.
pub fn ssxaudio_start_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let exit = start_music_trigger_steps(c)?;
        if exit == TriggerExit::Done {
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

/// The body of [`ssxaudio_start_music_trigger`] after its prologue.
fn start_music_trigger_steps(c: &mut Cpu) -> Result<TriggerExit, crate::call::Unwound> {
    let this = Addr(c.g[S0][0] as u32);
    c.g[V0][0] = this.off(MT_IN_BIG_CHALLENGE).lw(c);
    c.g[S3][0] = c.g[A1][0];
    if c.g[V0][0] != 0 {
        return Ok(TriggerExit::Done);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(MT_BIG_CHALLENGE_STARTING).lw(c);
    if c.g[V0][0] != 0 {
        return Ok(TriggerExit::Skip);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(MT_CUE_ID).lw(c);
    let same_cue = c.g[V0][0] == c.g[S3][0];
    c.g[V0][0] = u64::MAX;
    if same_cue {
        return Ok(TriggerExit::Done);
    }
    c.g[S2][0] = 0;
    c.cycles += 3;
    c.g[V1][0] = this.off(MT_MUSIC_OWNER).lw(c);
    let obj = c.g[S1][0];
    if c.g[V1][0] == c.g[V0][0] {
        // Nobody owns the music event: claim it when `obj` is the focus rider.
        c.cycles += 4;
        c.g[A1][0] = u64::MAX;
        HMT_FIND_RIDER.call(c, 0x0028_d688)?;
        c.g[A1][0] = sx((obj as u32).wrapping_add(0x6c0));
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 8;
            c.g[V1][0] = Addr(obj as u32).off(0x6c0).lw(c);
            c.g[S2][0] = 1;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x38).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x3c).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_d6a8)?;
            let owner = c.g[V0][0];
            this.off(MT_MUSIC_OWNER).sw(c, owner);
        }
    } else {
        // Someone owns it: ok when `obj` has the owner's AI index.
        c.cycles += 9;
        c.g[V1][0] = Addr(obj as u32).off(0x6c0).lw(c);
        c.g[A1][0] = sx((obj as u32).wrapping_add(0x6c0));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x38).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x3c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_d6c8)?;
        c.g[V1][0] = this.off(MT_MUSIC_OWNER).lw(c);
        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
        c.g[S2][0] = (c.g[V0][0] < 1) as u64;
    }
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        return Ok(TriggerExit::Skip);
    }
    // Stop the playing cue (if any), then start the new one.
    c.cycles += 3;
    c.g[A1][0] = this.off(MT_CUE_ID).lw(c);
    let playing = c.g[A1][0] != 0;
    c.g[S1][0] = sx(this.0.wrapping_add(HMT_MUSIC));
    if playing {
        c.cycles += 6;
        c.g[V1][0] = this.off(HMT_MUSIC_VT).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_d700)?;
    }
    c.cycles += 8;
    c.g[V1][0] = this.off(HMT_MUSIC_VT).lw(c);
    c.g[A1][0] = c.g[S3][0];
    let cue = c.g[S3][0];
    this.off(MT_CUE_ID).sw(c, cue);
    this.off(MT_CUE_EXTRA).sw(c, 0);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0028_d720)?;
    Ok(TriggerExit::Done)
}

/// `int SSXAudioSystem_ClearMusicTrigger(SSXAudioSystem* self, Rider* obj)`
/// (0x28d7d8): with no rider, releases the music event. Otherwise the owner
/// releases it (its AI index matches `obj`'s), and a playing cue stops. Returns
/// whether the event was released.
pub fn ssxaudio_clear_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        let this = Addr(c.g[S0][0] as u32);
        if c.g[A1][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = u64::MAX;
            c.g[S1][0] = 1;
            let none = c.g[V0][0];
            this.off(MT_MUSIC_OWNER).sw(c, none);
        } else {
            clear_music_owner_if_matching(c, this)?;
        }
        // The cue stops when the event was released.
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = this.off(MT_CUE_ID).lw(c);
            c.g[A2][0] = sx(this.0.wrapping_add(HMT_MUSIC));
            if c.g[A1][0] != 0 {
                c.cycles += 9;
                c.g[V1][0] = this.off(HMT_MUSIC_VT).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_d870)?;
                this.off(MT_CUE_EXTRA).sw(c, 0);
                this.off(MT_CUE_ID).sw(c, 0);
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The rider branch of [`ssxaudio_clear_music_trigger`]: when the owner's
/// AI index (from `obj`'s getter) matches the event owner, release it and
/// report it (s1 = 1).
fn clear_music_owner_if_matching(c: &mut Cpu, this: Addr) -> Result<(), crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = this.off(MT_MUSIC_OWNER).lw(c);
    c.g[S2][0] = u64::MAX;
    if c.g[V0][0] != c.g[S2][0] {
        c.cycles += 9;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x6c0).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x38).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x3c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_d830)?;
        c.g[V1][0] = this.off(MT_MUSIC_OWNER).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 2;
            let none = c.g[S2][0];
            this.off(MT_MUSIC_OWNER).sw(c, none);
            c.g[S1][0] = 1;
        }
    }
    Ok(())
}

/// `int SSXAudioSystem_GetMusicTrigger(SSXAudioSystem* self)` (0x28d898): the
/// cue id of the music trigger (self+0x623c).
pub fn ssxaudio_get_music_trigger(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(MT_CUE_ID).lw(c);
    ret(c);
}

/// `int SSXAudioSystem_StartNextSong(SSXAudioSystem* self, int quiet)`
/// (0x28df18): picks a random song and starts it with its timers. Returns 1
/// when it started. When the music player is busy (0x2a10c0), only the quiet
/// flag decides whether the busy notice plays (the notice plays when quiet
/// is 0).
pub fn ssxaudio_start_next_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = 0;
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002a_10c0, 0x0028_df44)?;
        if c.g[V0][0] != 0 {
            // Busy: only a non-quiet request plays the notice.
            c.cycles += 2;
            c.g[V0][0] = c.g[S3][0];
            if c.g[S1][0] == 0 {
                c.cycles += 14;
                c.g[V0][0] = Addr(this).off(HMT_MUSIC).lw(c);
                c.g[V1][0] = 0x4a_0000;
                c.ldl(A2, 0x004a_365f);
                c.ldr(A2, 0x004a_3658);
                c.g[A1][0] = 0x64;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = 0;
                c.g[T3][0] = 0;
                call_ptr(c, HMT_ADD_TIMER, 0x0028_e048)?;
                c.g[V0][0] = c.g[S3][0];
            }
        } else {
            // Not busy: the DJ speech (flag at 0x62b8) pauses the music player.
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(HMT_PAUSED).lw(c);
            c.g[S2][0] = sx(this.wrapping_add(HMT_MUSIC));
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x002b_3d48, 0x0028_df68)?;
            }
            c.cycles += 41;
            c.g[V1][0] = Addr(this).off(HMT_MUSIC_VT).lw(c);
            c.g[A1][0] = 0x12;
            c.g[S3][0] = 1;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_df84)?;
            c.g[A0][0] = c.g[S0][0];
            MT_SELECT_RANDOM_SONG.call(c, 0x0028_df8c)?;
            c.g[V1][0] = Addr(this).off(HMT_MUSIC).lw(c);
            c.g[V0][0] = 0x4a_0000;
            c.ldl(A2, 0x004a_364f);
            c.ldr(A2, 0x004a_3648);
            c.g[A1][0] = 0x7d0;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A3][0] = 0;
            c.g[T0][0] = 1;
            c.g[T1][0] = 1;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, HMT_ADD_TIMER, 0x0028_dfc0)?;
            c.g[V1][0] = Addr(this).off(HMT_MUSIC).lw(c);
            c.g[S1][0] = 0x4a_0000;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3650));
            c.g[A0][0] = c.g[S2][0];
            c.g[S0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002b_4908, 0x0028_dfdc)?;
            let s1 = c.g[S1][0] as u32;
            c.ldl(A2, s1.wrapping_add(0x7));
            c.ldr(A2, s1);
            c.g[A0][0] = c.g[S0][0];
            c.g[T1][0] = c.g[V0][0];
            c.g[A1][0] = 0x5dc;
            c.g[A3][0] = 0;
            c.g[T0][0] = 3;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            call_ptr(c, HMT_ADD_TIMER, 0x0028_e004)?;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_NextSongTimerCallback(void* self)` (0x28e068): the
/// timer that starts the next song (quiet), returning whether it did.
pub fn ssxaudio_next_song_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        HMT_START_NEXT_SONG.call(c, 0x0028_e078)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_SongEndCallback(SSXAudioSystem* self, MusicEvent* event)`
/// (0x28e088): when the song has ended (or none plays), the event's cue is
/// queued as the pending request. Otherwise a request with flag 1 is queued
/// too; any other flag is refused (returns 0).
pub fn ssxaudio_song_end_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = Addr(c.g[S0][0] as u32);
        c.g[A0][0] = this.off(MT_SONG_INSTANCE).lw(c);
        let no_song = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let song_over = if no_song {
            true
        } else {
            c.cycles += 5;
            call_ptr(c, 0x002b_2550, 0x0028_e0b0)?;
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] == c.g[V1][0]
        };
        let queued = if song_over {
            c.cycles += 4;
            c.g[A0][0] = 1;
            c.g[V0][0] = 1;
            let pending = c.g[A0][0];
            this.off(MT_REQUEST_PENDING).sw(c, pending);
            true
        } else {
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(MT_EVENT_DATA2).lw(c);
            c.cycles += 3;
            c.g[V0][0] = 0;
            if c.g[V1][0] == 1 {
                c.cycles += 2;
                let flag = c.g[V1][0];
                this.off(MT_REQUEST_PENDING).sw(c, flag);
                c.g[V0][0] = 1;
                true
            } else {
                false
            }
        };
        if queued {
            c.cycles += 2;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(MT_EVENT_DATA1).lw(c);
            let request = c.g[V1][0];
            this.off(MT_REQUEST).sw(c, request);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_MusicEventTimerCallback(SSXAudioSystem* self, MusicEvent* ev)`
/// (0x28e510): plays the event's cue through the music player (vtable slot 4),
/// returning 1.
pub fn ssxaudio_music_event_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(HMT_MUSIC_VT).lw(c);
        c.g[A1][0] = Addr(c.g[A1][0] as u32).off(MT_EVENT_DATA1).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(HMT_MUSIC));
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0028_e534)?;
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopMusic(SSXAudioSystem* self)` (0x28fc38): stops the
/// music player (vtable call through self+0x118).
pub fn ssxaudio_stop_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(HMT_MUSIC));
        call_ptr(c, 0x002b_3ac0, 0x0028_fc48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{scramble, Rng, DATA};
    use ssx3_ee as ee;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (one machine's RAM), its magic, and
    /// the forced results: one word per fake at `oc1032_result(k)`.
    const OC1032_CTRL: u32 = DATA + 0x0e_f000;
    const OC1032_MAGIC: u32 = 0x1032_5e7f;

    fn oc1032_result(k: u32) -> u32 {
        OC1032_CTRL + 4 + 4 * k
    }

    /// The rider getter (vtable slot 0x38 of `obj + 0x6c0`) and the music
    /// player's calls: fixture targets outside the function table.
    const OC1032_RIDER_GET: u32 = FAKE + 0x300;
    const OC1032_MUSIC_GET: u32 = FAKE + 0x310;

    /// A machine whose callees land in this module's fakes, with the rider
    /// getter returning its forced word (result 9).
    fn oc1032_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc1032_miss;
        c.bus.write32(OC1032_CTRL, OC1032_MAGIC);
        r
    }

    fn oc1032_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fake(c, target, ra);
        if c.pc == ra && target == OC1032_RIDER_GET {
            c.g[V0][0] = sx(c.bus.read32(oc1032_result(9)));
        }
    }

    /// `{delta, target}` at slot `off` of the vtable at `vt`.
    fn oc1032_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, target: u32) {
        c.bus.write16(vt + off, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(vt + off + 4, target);
    }

    /// The fakes of the callees these functions call through the table. Each
    /// gates on [`OC1032_CTRL`] and falls back to its prior body otherwise.
    macro_rules! oc1032_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $k:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1032_CTRL) != OC1032_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
                if c.pc == ra {
                    c.g[V0][0] = sx(c.bus.read32(oc1032_result($k)));
                }
            }
        };
    }

    oc1032_fake!(
        oc1032_find_rider,
        OC1032_PREV_FIND_RIDER,
        0x0028_5d98,
        ee::s07::f_00285d98__generated,
        0
    );
    oc1032_fake!(
        oc1032_2a10c0,
        OC1032_PREV_2A10C0,
        0x002a_10c0,
        ee::s08::f_002a10c0__generated,
        1
    );
    oc1032_fake!(
        oc1032_2b3d48,
        OC1032_PREV_2B3D48,
        0x002b_3d48,
        ee::s08::f_002b3d48__generated,
        2
    );
    oc1032_fake!(
        oc1032_2b4908,
        OC1032_PREV_2B4908,
        0x002b_4908,
        ee::s08::f_002b4908__generated,
        3
    );
    oc1032_fake!(
        oc1032_2adca0,
        OC1032_PREV_2ADCA0,
        0x002a_dca0,
        ee::s08::f_002adca0__generated,
        4
    );
    oc1032_fake!(
        oc1032_select_song,
        OC1032_PREV_SELECT,
        0x0028_d488,
        ee::s07::f_0028d488__generated,
        5
    );
    oc1032_fake!(
        oc1032_song_status,
        OC1032_PREV_STATUS,
        0x002b_2550,
        ee::s08::f_002b2550__generated,
        6
    );
    oc1032_fake!(
        oc1032_stop,
        OC1032_PREV_STOP,
        0x002b_3ac0,
        ee::s08::f_002b3ac0__generated,
        7
    );
    oc1032_fake!(
        oc1032_start_next,
        OC1032_PREV_START_NEXT,
        0x0028_df18,
        ee::s07::f_0028df18__generated,
        8
    );

    /// Hook the fakes into the callees' override slots (once per process).
    fn oc1032_install() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc1032_find_rider,
            &OC1032_PREV_FIND_RIDER,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A10C0),
            oc1032_2a10c0,
            &OC1032_PREV_2A10C0,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B3D48),
            oc1032_2b3d48,
            &OC1032_PREV_2B3D48,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B4908),
            oc1032_2b4908,
            &OC1032_PREV_2B4908,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            oc1032_2adca0,
            &OC1032_PREV_2ADCA0,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028D488),
            oc1032_select_song,
            &OC1032_PREV_SELECT,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2550),
            oc1032_song_status,
            &OC1032_PREV_STATUS,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B3AC0),
            oc1032_stop,
            &OC1032_PREV_STOP,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028DF18),
            oc1032_start_next,
            &OC1032_PREV_START_NEXT,
        );
    }

    /// The music player's vtable (`self + 0x5558`).
    const OC1032_AUDIO: u32 = DATA + 0x1_0000;
    const OC1032_RIDER: u32 = DATA + 0x2000;
    const OC1032_RIDER_VT: u32 = DATA + 0x2100;
    const OC1032_MUSIC_VT: u32 = DATA + 0x2200;

    #[test]
    fn ssxaudio_get_music_trigger_matches() {
        check(
            ee::s07::f_0028d898__generated,
            ssxaudio_get_music_trigger,
            200,
            &|c, seed| {
                let mut r = Rng(0x1032_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1032_AUDIO);
                let cue = if seed % 4 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(OC1032_AUDIO + MT_CUE_ID, cue);
            },
        );
    }

    #[test]
    fn ssxaudio_clear_music_trigger_matches() {
        check(
            ee::s07::f_0028d7d8__generated,
            ssxaudio_clear_music_trigger,
            300,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_d7d8, 4);
                let rider = if seed % 5 == 4 { 0 } else { OC1032_RIDER };
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.g[A1][0] = sx(rider);
                let owner = [u32::MAX, u32::MAX, 7, r.next_u32()][(r.next_u32() % 4) as usize];
                c.bus.write32(OC1032_AUDIO + MT_MUSIC_OWNER, owner);
                let cue = [0, 0, 3, r.next_u32() % 16 + 1][(r.next_u32() % 4) as usize];
                c.bus.write32(OC1032_AUDIO + MT_CUE_ID, cue);
                c.bus.write32(OC1032_AUDIO + MT_CUE_EXTRA, r.next_u32());
                c.bus.write32(OC1032_AUDIO + HMT_MUSIC_VT, OC1032_MUSIC_VT);
                oc1032_slot(c, &mut r, OC1032_MUSIC_VT, 0x20, OC1032_MUSIC_GET);
                if rider != 0 {
                    c.bus.write32(rider + 0x6c0, OC1032_RIDER_VT);
                    oc1032_slot(c, &mut r, OC1032_RIDER_VT, 0x38, OC1032_RIDER_GET);
                }
                let forced = if seed % 2 == 0 { owner } else { r.next_u32() };
                c.bus.write32(oc1032_result(9), forced);
            },
        );
    }

    #[test]
    fn ssxaudio_start_music_trigger_matches() {
        oc1032_install();
        check(
            ee::s07::f_0028d630__generated,
            ssxaudio_start_music_trigger,
            300,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_d630, 4);
                let id = [0, 3, 5, 9][(seed % 4) as usize];
                let obj = if seed % 7 == 6 {
                    DATA + 0x2400
                } else {
                    OC1032_RIDER
                };
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(OC1032_RIDER);
                c.bus.write32(
                    OC1032_AUDIO + MT_IN_BIG_CHALLENGE,
                    u32::from(seed % 13 == 12),
                );
                c.bus.write32(
                    OC1032_AUDIO + MT_BIG_CHALLENGE_STARTING,
                    u32::from(seed % 11 == 10),
                );
                let cue = [0, 3, id, 9][(r.next_u32() % 4) as usize];
                c.bus.write32(OC1032_AUDIO + MT_CUE_ID, cue);
                c.bus.write32(OC1032_AUDIO + MT_CUE_EXTRA, r.next_u32());
                let owner = [u32::MAX, u32::MAX, 9, r.next_u32()][(r.next_u32() % 4) as usize];
                c.bus.write32(OC1032_AUDIO + MT_MUSIC_OWNER, owner);
                c.bus.write32(OC1032_AUDIO + HMT_MUSIC_VT, OC1032_MUSIC_VT);
                oc1032_slot(c, &mut r, OC1032_MUSIC_VT, 0x20, OC1032_MUSIC_GET);
                c.bus.write32(OC1032_RIDER + 0x6c0, OC1032_RIDER_VT);
                oc1032_slot(c, &mut r, OC1032_RIDER_VT, 0x38, OC1032_RIDER_GET);
                // FindRider: the focus rider is `obj` or another rider.
                let focus = if seed % 2 == 0 { obj } else { DATA + 0x2400 };
                c.bus.write32(oc1032_result(0), focus);
                // The rider getter: the owner (ok) or another index.
                let index = if seed % 3 == 0 { owner } else { r.next_u32() };
                c.bus.write32(oc1032_result(9), index);
            },
        );
    }

    #[test]
    fn ssxaudio_start_next_song_matches() {
        oc1032_install();
        check(
            ee::s07::f_0028df18__generated,
            ssxaudio_start_next_song,
            300,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_df18, 6);
                let music = DATA + 0x3000;
                let music_obj = DATA + 0x3100;
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.g[A1][0] = sx(u32::from((seed / 2) % 2 == 1));
                // The music player busy (0x2a10c0 returns nonzero) or not.
                c.bus.write32(oc1032_result(1), seed % 2);
                c.bus.write32(oc1032_result(3), r.next_u32());
                c.bus.write32(oc1032_result(4), r.next_u32());
                c.bus.write32(oc1032_result(5), 0);
                c.bus
                    .write32(OC1032_AUDIO + HMT_PAUSED, u32::from(seed % 4 == 0));
                c.bus.write32(OC1032_AUDIO + HMT_MUSIC, music);
                c.bus.write32(music, music_obj);
                c.bus.write32(OC1032_AUDIO + HMT_MUSIC_VT, OC1032_MUSIC_VT);
                oc1032_slot(c, &mut r, OC1032_MUSIC_VT, 0x20, OC1032_MUSIC_GET);
            },
        );
    }

    #[test]
    fn ssxaudio_next_song_timer_callback_matches() {
        oc1032_install();
        check(
            ee::s07::f_0028e068__generated,
            ssxaudio_next_song_timer_callback,
            200,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_e068, 1);
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.bus
                    .write32(oc1032_result(8), [0, 1, 7][(seed % 3) as usize]);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ssxaudio_song_end_callback_matches() {
        oc1032_install();
        check(
            ee::s07::f_0028e088__generated,
            ssxaudio_song_end_callback,
            300,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_e088, 2);
                let event = DATA + 0x5000;
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.g[A1][0] = sx(event);
                let song = if seed % 4 == 0 { 0 } else { DATA + 0x4000 };
                c.bus.write32(OC1032_AUDIO + MT_SONG_INSTANCE, song);
                c.bus
                    .write32(oc1032_result(6), [u32::MAX, 0, 5][(seed % 3) as usize]);
                c.bus.write32(event + MT_EVENT_DATA1, r.next_u32());
                c.bus
                    .write32(event + MT_EVENT_DATA2, [1, 0, 2][((seed / 3) % 3) as usize]);
                c.bus
                    .write32(OC1032_AUDIO + MT_REQUEST_PENDING, r.next_u32());
                c.bus.write32(OC1032_AUDIO + MT_REQUEST, r.next_u32());
            },
        );
    }

    #[test]
    fn ssxaudio_music_event_timer_callback_matches() {
        check(
            ee::s07::f_0028e510__generated,
            ssxaudio_music_event_timer_callback,
            200,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_e510, 2);
                let event = DATA + 0x5000;
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.g[A1][0] = sx(event);
                c.bus.write32(event + MT_EVENT_DATA1, r.next_u32());
                c.bus.write32(OC1032_AUDIO + HMT_MUSIC_VT, OC1032_MUSIC_VT);
                oc1032_slot(c, &mut r, OC1032_MUSIC_VT, 0x20, OC1032_MUSIC_GET);
            },
        );
    }

    #[test]
    fn ssxaudio_stop_music_matches() {
        oc1032_install();
        check(
            ee::s07::f_0028fc38__generated,
            ssxaudio_stop_music,
            200,
            &|c, seed| {
                let mut r = oc1032_machine(c, seed, 0x0028_fc38, 1);
                c.g[A0][0] = sx(OC1032_AUDIO);
                c.bus.write32(oc1032_result(7), r.next_u32());
            },
        );
    }
}
