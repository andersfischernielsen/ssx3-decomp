//! Staging module oc1050 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1050`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_3d80,
        music_player_fade_song_volume,
        "MusicPlayer_FadeSongVolume",
    ),
    (
        0x002b_4180,
        music_player_skip_songs,
        "MusicPlayer_SkipSongs",
    ),
    (
        0x002b_4470,
        music_player_start_loops,
        "MusicPlayer_StartLoops",
    ),
    (
        0x002b_44d8,
        music_player_stop_loops,
        "MusicPlayer_StopLoops",
    ),
    (
        0x002b_4540,
        music_player_timer_forward_slot1,
        "MusicPlayer_TimerForwardSlot1",
    ),
    (
        0x002b_4568,
        music_player_schedule_loop,
        "MusicPlayer_ScheduleLoop",
    ),
    (
        0x002b_4620,
        music_player_attach_loop_bank,
        "MusicPlayer_AttachLoopBank",
    ),
    (
        0x002b_4740,
        music_player_loop_timer_callback,
        "MusicPlayer_LoopTimerCallback",
    ),
];

/// Callees (the generated alias of each, which dispatches through its override slot).
const MPL_ADD_TIMER: Callee = Callee {
    addr: 0x002a_dca0,
    f: ee::s08::f_002adca0,
};
const MPL_KILL_TIMERS: Callee = Callee {
    addr: 0x002a_dde0,
    f: ee::s08::f_002adde0,
};
const MPL_SELECT_NEXT_SONG: Callee = Callee {
    addr: 0x002b_42b8,
    f: ee::s08::f_002b42b8,
};
const MPL_CLEAR_LOOPS_ACTIVE: Callee = Callee {
    addr: 0x002b_4708,
    f: ee::s08::f_002b4708,
};
const MPL_SCHEDULE_LOOP: Callee = Callee {
    addr: 0x002b_4568,
    f: ee::s08::f_002b4568,
};
const MPL_SONG_BEAT_MS: Callee = Callee {
    addr: 0x002b_2690,
    f: ee::s08::f_002b2690,
};
const MPL_SONG_MS_TO_NEXT: Callee = Callee {
    addr: 0x002b_2718,
    f: ee::s08::f_002b2718,
};
const MPL_SONG_BEAT_INDEX: Callee = Callee {
    addr: 0x002b_2778,
    f: ee::s08::f_002b2778,
};
const MPL_SONG_PICK_PHRASE: Callee = Callee {
    addr: 0x002b_2500,
    f: ee::s08::f_002b2500,
};

/// `cMusicSys` fields.
const MPL_STREAMS: u32 = 0x0;
const MPL_CURRENT_SONG_NUM: u32 = 0x3f0;
const MPL_SONG: u32 = 0x408;
/// Fade state: the per-tick step (float), the start volume (float), the target.
const MPL_FADE_STEP: u32 = 0x40c;
const MPL_FADE_START: u32 = 0x410;
const MPL_FADE_TARGET: u32 = 0x414;
/// The loop timer object: a halfword offset and a function pointer (v01).
const MPL_LOOP_OBJ: u32 = 0x5440;
const MPL_LOOP_OBJ_OFFSET: u32 = 0x8;
const MPL_LOOP_OBJ_FN: u32 = 0xc;
/// `cSongInstance` fields.
const MPL_SONG_BANK_INDEX: u32 = 0x34;
const MPL_SONG_STATE: u32 = 0x38;
const MPL_SONG_VOLUME: u32 = 0x40;
const MPL_SONG_LOOPING: u32 = 0x4c;
/// The stream system's bank manager (its first word), the BankMonitor timer
/// owner inside it, and the bank table pointer.
const MPL_BANK_MANAGER: u32 = 0x0;
const MPL_BANK_MONITOR: u32 = 0x1d8;
const MPL_BANK_TABLE: u32 = 0xacc;
/// `cBankInstance` (0x60 bytes each): its status word and its handle.
const MPL_BANK_SIZE: u32 = 0x60;
const MPL_BANK_STATUS: u32 = 0x0;
const MPL_BANK_HANDLE: u32 = 0x4;
/// The 8-byte monitor payloads in the game image (`D_004A38xx`, read with
/// `ldl`/`ldr` at base+7 and base).
const MPL_PAYLOAD_LOOP_START: u32 = 0x004a_3838;
const MPL_PAYLOAD_LOOP_STOP: u32 = 0x004a_3840;
const MPL_PAYLOAD_FADE: u32 = 0x004a_3850;

/// Loads the 8-byte payload at `base` into `rt` (`ldl` at base+7, `ldr` at base).
fn mpl_payload(c: &mut Cpu, rt: usize, base: u32) {
    c.ldl(rt, base.wrapping_add(7));
    c.ldr(rt, base);
}

/// `void MusicPlayer_FadeSongVolume(cMusicSys* self, signed char target, float duration)`
/// (0x2b3d80): when the current song's volume (`unk_40`) is not `target`, starts a
/// fade: the target goes to `unk_414`, the volume to `unk_410` (as a float), and the
/// per-tick step `(target - volume) / 25` over `duration` seconds to `unk_40C`. A
/// 40-tick monitor timer then steps the volume.
pub fn music_player_fade_song_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[T4][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        c.g[V0][0] = Addr((c.g[T4][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        let has_song = c.g[V0][0] != 0;
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        if has_song {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MPL_SONG_VOLUME)).lb(c);
            let same = c.g[A1][0] == c.g[A0][0];
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            if !same {
                mpl_fade_start(c)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The fade's start: the step division, the stores, and the timer call.
fn mpl_fade_start(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[V0][0] = 0x19;
    c.div(0, 3, 2);
    c.mtc1(4, 0);
    c.cvt_s_w(0, 0);
    c.cycles += 20;
    c.g[A0][0] = 0x4a_0000;
    let target = c.g[A1][0];
    Addr((c.g[T4][0] as u32).wrapping_add(MPL_FADE_TARGET)).sw(c, target);
    c.g[A3][0] = 0;
    c.swc1(0, (c.g[T4][0] as u32).wrapping_add(MPL_FADE_START));
    c.g[A1][0] = 0x28;
    c.g[V0][0] = Addr(c.g[T4][0] as u32).lw(c);
    c.g[T0][0] = 0;
    mpl_payload(c, 6, MPL_PAYLOAD_FADE);
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    c.g[T3][0] = 0;
    c.g[V1][0] = c.lo[0];
    c.mtc1(3, 0);
    c.cvt_s_w(0, 0);
    c.div_s(0, 0, 12);
    c.swc1(0, (c.g[T4][0] as u32).wrapping_add(MPL_FADE_STEP));
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MPL_BANK_MONITOR));
    MPL_ADD_TIMER.call(c, 0x002b_3e10)
}

/// `void MusicPlayer_SkipSongs(cMusicSys* self, int n)` (0x2b4180): clears the
/// current song number, then selects the next song `n + 1` times (`n` times
/// after the first selection, each in its own tick).
pub fn music_player_skip_songs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let unset = c.g[V0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(MPL_CURRENT_SONG_NUM)).sw(c, unset);
        c.g[A1][0] = 0;
        MPL_SELECT_NEXT_SONG.call(c, 0x002b_41ac)?;
        if (c.g[S0][0] as i64) <= 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                MPL_SELECT_NEXT_SONG.call(c, 0x002b_41c8)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_41b8);
                    }
                } else {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_StartLoops(cMusicSys* self)` (0x2b4470): when a song plays
/// and its loop flag (`unk_4C`) is clear, sets it and adds the loop-start monitor
/// timer.
pub fn music_player_start_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        let no_song = c.g[A1][0] == 0;
        frame.restore_ra(c, 0);
        if !no_song {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(MPL_SONG_LOOPING)).lw(c);
            let looping = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !looping {
                c.cycles += 15;
                c.g[V1][0] = 0x4a_0000;
                let flag = c.g[V0][0];
                Addr((c.g[A1][0] as u32).wrapping_add(MPL_SONG_LOOPING)).sw(c, flag);
                c.g[A3][0] = 0;
                mpl_payload(c, 6, MPL_PAYLOAD_LOOP_START);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[T3][0] = 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MPL_BANK_MONITOR));
                MPL_ADD_TIMER.call(c, 0x002b_44c8)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_StopLoops(cMusicSys* self)` (0x2b44d8): when a song plays,
/// kills the two loop monitor timers and clears its loop flag.
pub fn music_player_stop_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        let has_song = c.g[V0][0] != 0;
        c.g[V0][0] = 0x4a_0000;
        if has_song {
            c.cycles += 15;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            mpl_payload(c, 5, MPL_PAYLOAD_LOOP_START);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MPL_BANK_MONITOR));
            MPL_KILL_TIMERS.call(c, 0x002b_450c)?;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = 0x4a_0000;
            mpl_payload(c, 5, MPL_PAYLOAD_LOOP_STOP);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MPL_BANK_MONITOR));
            MPL_KILL_TIMERS.call(c, 0x002b_4528)?;
            c.g[A0][0] = c.g[S0][0];
            MPL_CLEAR_LOOPS_ACTIVE.call(c, 0x002b_4530)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_TimerForwardSlot1(cMusicSys* self)` (0x2b4540): calls the
/// loop timer object's function (`v01`) with `self` plus the object's halfword
/// offset as `this`.
pub fn music_player_timer_forward_slot1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(MPL_LOOP_OBJ)).lw(c);
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(MPL_LOOP_OBJ_OFFSET)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(MPL_LOOP_OBJ_FN)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x002b_455c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_ScheduleLoop(cMusicSys* self, cSongInstance* ent)`
/// (0x2b4568): for a playing song, reads the beat length, the time to the next
/// beat, the beat index and a random phrase offset, then adds the loop monitor
/// timer for the next beat.
pub fn music_player_schedule_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        let has_song = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_song {
            c.cycles += 26;
            c.g[A0][0] = c.g[S1][0];
            MPL_SONG_BEAT_MS.call(c, 0x002b_459c)?;
            c.mov_s(21, 0);
            c.g[A0][0] = c.g[S1][0];
            MPL_SONG_MS_TO_NEXT.call(c, 0x002b_45a8)?;
            c.mov_s(20, 0);
            c.g[A0][0] = c.g[S1][0];
            MPL_SONG_BEAT_INDEX.call(c, 0x002b_45b4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            MPL_SONG_PICK_PHRASE.call(c, 0x002b_45c0)?;
            c.g[A3][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V1][0] = 0x4a_0000;
            c.cvt_w_s(0, 20);
            c.mfc1(5, 0);
            mpl_payload(c, 6, MPL_PAYLOAD_LOOP_STOP);
            c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.cvt_w_s(0, 21);
            c.mfc1(10, 0);
            c.g[A3][0] = Addr((c.g[S2][0] as u32).wrapping_add(MPL_SONG)).lw(c);
            c.g[T0][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MPL_BANK_MONITOR));
            c.g[T1][0] = c.g[S0][0];
            c.g[T3][0] = 0;
            MPL_ADD_TIMER.call(c, 0x002b_45fc)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `int MusicPlayer_AttachLoopBank(cMusicSys* self)` (0x2b4620): marks the
/// song's loop flag and, while the song is loading (state 1), waits for its
/// bank (status 1), then starts the path track with the bank's handle and moves
/// the song to state 2. Returns 1 when the song is ready (state 2 or just started).
pub fn music_player_attach_loop_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        let has_song = c.g[V0][0] != 0;
        c.g[A0][0] = 1;
        if has_song {
            mpl_attach_song(c)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The song part of `MusicPlayer_AttachLoopBank`: leaves the return value in
/// `v0`. A busy bank returns early (no further charge), as the generated body's
/// jump to its exit does.
fn mpl_attach_song(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let flag = c.g[A0][0];
    Addr((c.g[V0][0] as u32).wrapping_add(MPL_SONG_LOOPING)).sw(c, flag);
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(MPL_SONG)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(MPL_SONG_STATE)).lw(c);
    let not_loading = c.g[V0][0] != c.g[A0][0];
    c.g[A2][0] = 0;
    if not_loading {
        c.cycles += 3;
        c.g[V0][0] ^= 2;
        c.g[A2][0] = c.g[A0][0];
        if c.g[V0][0] != 0 {
            c.g[A2][0] = 0;
        }
    } else {
        c.cycles += 11;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(MPL_SONG_BANK_INDEX)).lw(c);
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(MPL_STREAMS)).lw(c);
        c.g[V1][0] = 0x60;
        c.mult(0, 4, 4, 3);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(MPL_BANK_TABLE)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] ^= 1;
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[A2][0];
        if busy {
            return Ok(());
        }
        c.cycles += 10;
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(MPL_BANK_HANDLE)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = 0x1100_0000;
        c.g[A0][0] |= 0x2;
        call_ptr(c, 0x003d_2df8, 0x002b_4698)?;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        c.g[V0][0] = 2;
        c.g[A2][0] = 1;
        let state = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(MPL_SONG_STATE)).sw(c, state);
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[A2][0];
    Ok(())
}

/// `void MusicPlayer_LoopTimerCallback(cMusicSys* self)` (0x2b4740): the loop
/// timer's tick. It looks at the song's bank: a used bank schedules the next
/// loop beat; an empty or loading bank is left alone (see the return values).
/// Returns 1 except while the bank is loading (status 2), which returns 0.
pub fn music_player_loop_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x60;
        c.g[A2][0] = 1;
        c.g[T0][0] = Addr((c.g[A3][0] as u32).wrapping_add(MPL_SONG)).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(MPL_SONG_BANK_INDEX)).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.mult(0, 2, 2, 5);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(MPL_BANK_TABLE)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let status = c.g[V1][0];
        // Paths that return through the shared exit tick (`+1`), except the
        // default state (3+), which returns at once.
        let shared_exit = if status == c.g[A2][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[A3][0];
            c.g[A1][0] = c.g[T0][0];
            MPL_SCHEDULE_LOOP.call(c, 0x002b_47a4)?;
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 2;
            if (status as i64) < 2 {
                c.g[V0][0] = 1;
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = 1;
                if status == 2 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    true
                } else {
                    c.cycles += 2;
                    false
                }
            }
        };
        if shared_exit {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the
    /// machine's RAM: other tests' machines run the real bodies).
    const CTRL: u32 = DATA + 0x21_0000;
    const MAGIC: u32 = 0x1050_f00d;
    /// Call log: the next free record, and the records (target, a0..t3).
    const LOG_NEXT: u32 = DATA + 0x21_0004;
    const LOG: u32 = DATA + 0x21_0100;
    /// Mailboxes of the leaf fakes: beat ms (f0), ms to next (f0), beat index,
    /// phrase offset.
    const MB_BEAT_MS: u32 = DATA + 0x21_0800;
    const MB_MS_TO_NEXT: u32 = DATA + 0x21_0804;
    const MB_BEAT_INDEX: u32 = DATA + 0x21_0808;
    const MB_PHRASE: u32 = DATA + 0x21_080c;
    /// Fixture objects, each in its own region.
    const SELF: u32 = DATA + 0x22_0000;
    const STREAMS: u32 = DATA + 0x23_0000;
    const BANK_MGR: u32 = DATA + 0x23_1000;
    const BANKS: u32 = DATA + 0x23_2000;
    const SONG: u32 = DATA + 0x23_3000;
    const OBJ: u32 = DATA + 0x23_4000;
    /// `MusicPlayer_ClearLoopsActive` (real, cheap): the loop object's function.
    const CLEAR_LOOPS_ACTIVE: u32 = 0x002b_4708;

    /// Appends a call record (target and the argument registers) to the log.
    fn mpl_log(c: &mut Cpu, target: u32) {
        let mut p = c.bus.read32(LOG_NEXT);
        if p == 0 {
            p = LOG;
        }
        c.bus.write32(p, target);
        for (i, r) in [A0, A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
            c.bus.write32(p + 4 + 4 * i as u32, c.g[r][0] as u32);
        }
        c.bus.write32(LOG_NEXT, p + 0x40);
    }

    /// Leaf stand-in for `BankMonitor_AddTimer` (`0x2adca0`): logs the call.
    fn mpl_fake_add_timer(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_add_timer),
                ee::s08::f_002adca0__generated,
            );
        }
        mpl_log(c, 0x002a_dca0);
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `BankMonitor_KillTimersByCallback` (`0x2adde0`).
    fn mpl_fake_kill_timers(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_kill_timers),
                ee::s08::f_002adde0__generated,
            );
        }
        mpl_log(c, 0x002a_dde0);
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `MusicPlayer_SelectNextSong` (`0x2b42b8`).
    fn mpl_fake_select_next(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_select_next),
                ee::s08::f_002b42b8__generated,
            );
        }
        mpl_log(c, 0x002b_42b8);
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `SongInstance_GetBeatMs` (`0x2b2690`): f0 from its mailbox.
    fn mpl_fake_beat_ms(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_beat_ms),
                ee::s08::f_002b2690__generated,
            );
        }
        mpl_log(c, 0x002b_2690);
        c.f[0] = c.bus.read32(MB_BEAT_MS);
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `SongInstance_GetMsToNextBeat` (`0x2b2718`).
    fn mpl_fake_ms_to_next(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_ms_to_next),
                ee::s08::f_002b2718__generated,
            );
        }
        mpl_log(c, 0x002b_2718);
        c.f[0] = c.bus.read32(MB_MS_TO_NEXT);
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `SongInstance_GetBeatIndex` (`0x2b2778`): v0 from its mailbox.
    fn mpl_fake_beat_index(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_beat_index),
                ee::s08::f_002b2778__generated,
            );
        }
        mpl_log(c, 0x002b_2778);
        c.g[V0][0] = sx(c.bus.read32(MB_BEAT_INDEX));
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for `SongInstance_PickRandomPhraseOffset` (`0x2b2500`).
    fn mpl_fake_pick_phrase(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_pick_phrase),
                ee::s08::f_002b2500__generated,
            );
        }
        mpl_log(c, 0x002b_2500);
        c.g[V0][0] = sx(c.bus.read32(MB_PHRASE));
        c.pc = c.g[RA][0] as u32;
    }

    /// Leaf stand-in for the sentence path's `PATH_TrackPrepare` (`0x3d2df8`).
    fn mpl_fake_track_prepare(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(mpl_fake_track_prepare),
                ee::s14::f_003d2df8__generated,
            );
        }
        mpl_log(c, 0x003d_2df8);
        c.pc = c.g[RA][0] as u32;
    }

    /// Puts the gated fakes in their override slots (once per process).
    fn mpl_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            mpl_fake_add_timer,
            fakes::prev_cell(mpl_fake_add_timer),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADDE0),
            mpl_fake_kill_timers,
            fakes::prev_cell(mpl_fake_kill_timers),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B42B8),
            mpl_fake_select_next,
            fakes::prev_cell(mpl_fake_select_next),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2690),
            mpl_fake_beat_ms,
            fakes::prev_cell(mpl_fake_beat_ms),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2718),
            mpl_fake_ms_to_next,
            fakes::prev_cell(mpl_fake_ms_to_next),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2778),
            mpl_fake_beat_index,
            fakes::prev_cell(mpl_fake_beat_index),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B2500),
            mpl_fake_pick_phrase,
            fakes::prev_cell(mpl_fake_pick_phrase),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D2DF8),
            mpl_fake_track_prepare,
            fakes::prev_cell(mpl_fake_track_prepare),
        );
    }

    /// Hooked fakes, scrambled registers, the gate on, the mailboxes random.
    fn mpl_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        mpl_hook();
        let mut r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(CTRL, MAGIC);
        c.bus
            .write32(MB_BEAT_MS, ((r.next_u32() % 4000) as f32).to_bits());
        c.bus
            .write32(MB_MS_TO_NEXT, ((r.next_u32() % 4000) as f32).to_bits());
        c.bus.write32(MB_BEAT_INDEX, r.next_u32() % 3);
        c.bus.write32(MB_PHRASE, r.next_u32() % 16);
        r
    }

    /// A `cMusicSys` with its stream system, bank table (statuses rotate with
    /// the seed) and one song whose state, bank index and loop flag vary.
    /// Seeds with `seed % 5 == 0` have no song.
    fn mpl_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(SELF + MPL_STREAMS, STREAMS);
        c.bus.write32(STREAMS + MPL_BANK_MANAGER, BANK_MGR);
        c.bus.write32(BANK_MGR + MPL_BANK_TABLE, BANKS);
        for i in 0..4u32 {
            let status = [1, 0, 2, 3, u32::MAX][((seed + i) % 5) as usize];
            c.bus
                .write32(BANKS + MPL_BANK_SIZE * i + MPL_BANK_STATUS, status);
            c.bus
                .write32(BANKS + MPL_BANK_SIZE * i + MPL_BANK_HANDLE, r.next_u32());
        }
        c.bus
            .write32(SELF + MPL_SONG, if seed % 5 == 0 { 0 } else { SONG });
        c.bus.write32(SONG + MPL_SONG_BANK_INDEX, (seed / 3) % 4);
        c.bus
            .write32(SONG + MPL_SONG_STATE, [1, 2, 0, 3][(seed % 4) as usize]);
        c.bus.write8(SONG + MPL_SONG_VOLUME, r.next_u32() as u8);
        c.bus.write32(SONG + MPL_SONG_LOOPING, seed % 2);
        c.bus.write32(SELF + MPL_CURRENT_SONG_NUM, r.next_u32());
        c.bus.write32(SELF + MPL_LOOP_OBJ, OBJ);
    }

    #[test]
    fn music_player_fade_song_volume_matches() {
        check(
            ee::s08::f_002b3d80__generated,
            music_player_fade_song_volume,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x3d80);
                mpl_world(c, &mut r, seed);
                let cur = r.next_u32() as u8;
                c.bus.write8(SONG + MPL_SONG_VOLUME, cur);
                let target = if seed % 3 == 0 {
                    cur as u32
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(target);
                c.f[12] = if seed % 2 == 0 {
                    0x3fc0_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn music_player_skip_songs_matches() {
        check(
            ee::s08::f_002b4180__generated,
            music_player_skip_songs,
            250,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4180);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx((seed % 5).wrapping_sub(1));
            },
        );
    }

    #[test]
    fn music_player_start_loops_matches() {
        check(
            ee::s08::f_002b4470__generated,
            music_player_start_loops,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4470);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    #[test]
    fn music_player_stop_loops_matches() {
        check(
            ee::s08::f_002b44d8__generated,
            music_player_stop_loops,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x44d8);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    #[test]
    fn music_player_timer_forward_slot1_matches() {
        check(
            ee::s08::f_002b4540__generated,
            music_player_timer_forward_slot1,
            200,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4540);
                mpl_world(c, &mut r, seed);
                // The loop object: a this-offset of 0, 0x10 or 0x20 and the
                // real ClearLoopsActive as its function.
                c.bus
                    .write16(OBJ + MPL_LOOP_OBJ_OFFSET, ((seed % 3) * 0x10) as u16);
                c.bus.write32(OBJ + MPL_LOOP_OBJ_FN, CLEAR_LOOPS_ACTIVE);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    #[test]
    fn music_player_schedule_loop_matches() {
        check(
            ee::s08::f_002b4568__generated,
            music_player_schedule_loop,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4568);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(SONG);
            },
        );
    }

    #[test]
    fn music_player_attach_loop_bank_matches() {
        check(
            ee::s08::f_002b4620__generated,
            music_player_attach_loop_bank,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4620);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    #[test]
    fn music_player_loop_timer_callback_matches() {
        check(
            ee::s08::f_002b4740__generated,
            music_player_loop_timer_callback,
            300,
            &|c, seed| {
                let mut r = mpl_machine(c, seed, 0x4740);
                mpl_world(c, &mut r, seed);
                c.g[A0][0] = sx(SELF);
            },
        );
    }
}
