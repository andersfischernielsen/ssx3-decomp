//! Staging module oc1547 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1547`.

#![allow(unused_imports)]

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
        0x0028_f3c8,
        ssx_audio_on_music_set_event_state,
        "SSXAudioSystem_OnMusicSetEventState",
    ),
    (
        0x0028_fae0,
        ssx_audio_restart_race_music,
        "SSXAudioSystem_RestartRaceMusic",
    ),
    (
        0x002a_8b28,
        midibanks_play_timer_callback,
        "MIDIBanks_PlayTimerCallback",
    ),
    (0x002a_e048, path_plat_init, "PATH_PlatInit"),
    (0x002b_1b98, song_instance_create, "SongInstance_Create"),
];

/// `lw off(gp)`: a word of the game's global block.
fn oc1547_gp_word(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_add(off);
    Addr(a).lw(c)
}

/// `int SSXAudioSystem_OnMusicSetEventState(cSSXAudio* self, int code)`
/// (0x0028f3c8). Forwards a music event to the monitor at `self + 0x118` and
/// returns its changed flag. When the monitor has no song tagged, the song-end
/// codes (10, 11, 18) also silence the monitor's voice (a virtual call through
/// the descriptor at `self + 0x5558`) while the global flag at gp+0x530 is set,
/// and clear `self->mSongPlaying` (+0x530).
pub fn ssx_audio_on_music_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        on_music_set_event_state_body(c)?;
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

fn on_music_set_event_state_body(c: &mut Cpu) -> Flow {
    call_ptr(c, 0x002b_3bc0, 0x0028_f3f4)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x002b_49e0, 0x0028_f400)?;
    c.g[V1][0] = u64::MAX;
    let song_tagged = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = c.g[S3][0];
    if song_tagged {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
    let song_end_code = c.g[V0][0] != 0;
    c.g[V1][0] = oc1547_gp_word(c, 0x530);
    if !song_end_code {
        c.cycles += 3;
        c.g[V0][0] = 0x12;
        if c.g[S0][0] != c.g[V0][0] {
            c.g[V0][0] = c.g[S3][0];
            return Ok(());
        }
        c.g[V0][0] = c.g[S3][0];
        c.cycles += 1;
        c.g[V1][0] = oc1547_gp_word(c, 0x530);
    }
    // The stop: only when the global flag (V1) is set.
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    if c.g[V1][0] != c.g[V0][0] {
        let s2 = c.g[S2][0] as u32;
        Addr(s2.wrapping_add(0x530)).sw(c, 0);
    } else {
        c.cycles += 7;
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x5558)).lw(c);
        c.g[A1][0] = 0x8;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_f450)?;
        let s2 = c.g[S2][0] as u32;
        Addr(s2.wrapping_add(0x530)).sw(c, 0);
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S3][0];
    Ok(())
}

/// `void SSXAudioSystem_RestartRaceMusic(cSSXAudio* self)` (0x0028fae0).
/// Drops the race music's monitor entries, then, when no song is playing or
/// the song is a Peak transition, stops the music, selects and plays a new
/// song and starts the race music bank.
pub fn ssx_audio_restart_race_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        restart_race_music_body(c, frame)?;
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

fn restart_race_music_body(c: &mut Cpu, frame: Frame) -> Flow {
    call_ptr(c, 0x0028_d960, 0x0028_fb04)?;
    let music_enabled = c.g[V0][0] != 0;
    c.g[V0][0] = 0x004a_0000;
    let early_exit = if music_enabled {
        race_music_branch(c, frame)?
    } else {
        false
    };
    if !early_exit {
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
    }
    Ok(())
}

/// The enabled path. Returns true when it already restored `s0` and leaves
/// through the epilogue early (the callers skip their own tail).
fn race_music_branch(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 30;
    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x364f));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3648));
    c.g[S0][0] = 0x004a_0000;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
    c.g[S4][0] = c.g[S0][0];
    c.g[S3][0] = c.g[S1][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    call_ptr(c, 0x002a_dde0, 0x0028_fb34)?;
    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
    c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x3657));
    c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x3650));
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    call_ptr(c, 0x002a_dde0, 0x0028_fb4c)?;
    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
    c.g[V0][0] = 0x004a_0000;
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x365f));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3658));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    call_ptr(c, 0x002a_dde0, 0x0028_fb68)?;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x002b_49e0, 0x0028_fb70)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    let in_transition = c.g[V0][0] != 0;
    c.g[A1][0] = 0x12;
    if !in_transition {
        c.cycles += 3;
        c.g[V0][0] = 0x191;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x530)).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                return Ok(true);
            }
            c.cycles += 1;
            c.g[A1][0] = 0x12;
        }
    }
    // After the monitor checks: stop, select, play, and start the race bank.
    c.cycles += 30;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0028_def0, 0x0028_fba8)?;
    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3650));
    c.g[V0][0] = 0x1;
    c.g[A0][0] = c.g[S3][0];
    let one = c.g[V0][0];
    Addr((c.g[S2][0] as u32).wrapping_add(0x5828)).sw(c, one);
    call_ptr(c, 0x002b_3ac0, 0x0028_fbbc)?;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0028_d488, 0x0028_fbc4)?;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    c.g[A3][0] = u64::MAX;
    c.g[T0][0] = 0;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0028_cf98, 0x0028_fbdc)?;
    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x118)).lw(c);
    c.g[A0][0] = c.g[S3][0];
    c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    call_ptr(c, 0x002b_4908, 0x0028_fbf0)?;
    c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x7));
    c.ldr(6, c.g[S1][0] as u32);
    c.g[A0][0] = c.g[S0][0];
    c.g[T1][0] = c.g[V0][0];
    c.g[A1][0] = 0xbb8;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0x4;
    c.g[T2][0] = 0;
    c.g[T3][0] = 0;
    call_ptr(c, 0x002a_dca0, 0x0028_fc18)?;
    Ok(false)
}

/// `void MIDIBanks_PlayTimerCallback(MIDIBanks* self, Obj* obj)` (0x002a8b28).
/// Pushes the play parameters, sets the volume-control slot and the base and
/// current volume of the instrument, then plays the bank. The current volume is
/// the base volume clamped to 0..127 (a negative base gives 0, the low byte
/// of a base in range).
pub fn midibanks_play_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = oc1547_gp_word(c, 0x654);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        let obj = c.g[S0][0] as u32;
        c.g[A1][0] = Addr(obj.wrapping_add(0x1c)).lw(c);
        c.g[A2][0] = Addr(obj.wrapping_add(0x20)).lw(c);
        call_ptr(c, 0x002a_8450, 0x002a_8b48)?;
        // mgr->BankMonitor.mVolControl[mInst] = obj->mPtr
        c.g[A0][0] = oc1547_gp_word(c, 0x654);
        c.g[A2][0] = 0x18;
        c.g[A1][0] = Addr(obj.wrapping_add(0x18)).lw(c);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let mgr = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(mgr.wrapping_add(0x1f4)).lw(c);
        c.g[A0][0] = Addr(mgr.wrapping_add(0x214)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let ptr = c.g[A1][0];
        Addr(c.g[V0][0] as u32).sw(c, ptr);
        // bm->mBaseVol[mInst] = obj->mData3
        c.g[V1][0] = oc1547_gp_word(c, 0x654);
        c.g[A0][0] = Addr(obj.wrapping_add(0x24)).lw(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        let mgr = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(mgr.wrapping_add(0x1f4)).lw(c);
        c.g[A3][0] = sx(mgr.wrapping_add(0x1d8));
        let bm = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(bm.wrapping_add(0x38)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let data3 = c.g[A0][0];
        Addr(c.g[V0][0] as u32).sw(c, data3);
        // The current volume of the instrument.
        c.g[A0][0] = Addr(mgr.wrapping_add(0x1f4)).lw(c);
        c.g[V1][0] = Addr(bm.wrapping_add(0x1c)).lw(c);
        c.g[T0][0] = Addr(bm.wrapping_add(0x38)).lw(c);
        c.mult(0, 4, 4, 6);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = Addr(bm.wrapping_add(0x20)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
        let at_or_above_127 = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let early_exit = if at_or_above_127 {
            false
        } else {
            c.cycles += 6;
            c.g[V0][0] = Addr(bm.wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let negative = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0;
            negative
        };
        if !early_exit {
            c.cycles += 7;
            c.g[V0][0] = Addr(bm.wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0x7f;
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(bm.wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
            }
        }
        c.cycles += 8;
        let value = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sb(c, value);
        c.g[A0][0] = oc1547_gp_word(c, 0x654);
        call_ptr(c, 0x002a_86b8, 0x002a_8c0c)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatInit(cb, id)` (0x002ae048): clears the 24 path slots (the
/// player, handle and bank index set to -1, the field at +0x18 to 0), starts
/// the 100 Hz client and the sync task, and stores the platform function
/// table (abort message, printf, allocator, resize, the sync task body).
pub fn path_plat_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x004d_0000;
        c.g[V0][0] = 0x004d_0000;
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ef8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3e98));
        c.g[A2][0] = u64::MAX;
        c.g[A0][0] = 0x17;
        loop {
            c.cycles += 9;
            Addr(c.g[V1][0] as u32).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let a2 = c.g[A2][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sw(c, a2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, a2);
            Addr(c.g[V0][0] as u32).sw(c, a2);
            Addr((c.g[V0][0] as u32).wrapping_add(0x18)).sw(c, 0);
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002a_e070);
                }
                continue;
            }
            break;
        }
        c.cycles += 27;
        c.g[A0][0] = c.g[A1][0];
        let cb = c.g[A3][0];
        Addr(oc1547_gp_word_addr(c, 0x6a8)).sw(c, cb);
        let id = c.g[A1][0];
        Addr(oc1547_gp_word_addr(c, 0x6ac)).sw(c, id);
        call_ptr(c, 0x003b_5910, 0x002a_e0a4)?;
        c.g[A0][0] = 0x002b_0000;
        c.g[A1][0] = 0x5;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fe0));
        c.g[A2][0] = 0x64;
        call_ptr(c, 0x003e_57f8, 0x002a_e0b8)?;
        c.g[A2][0] = 0x003e_0000;
        c.g[A1][0] = 0x0041_0000;
        c.g[A0][0] = 0x0025_0000;
        c.g[V1][0] = 0x0025_0000;
        c.g[V0][0] = 0x002b_0000;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6070));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75c8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2fa0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x26b8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2000));
        frame.restore_ra(c, 0);
        let (a2, a1, a0, v1, v0) = (
            c.g[A2][0],
            c.g[A1][0],
            c.g[A0][0],
            c.g[V1][0],
            c.g[V0][0],
        );
        Addr(oc1547_gp_word_addr(c, 0x690)).sw(c, a2);
        Addr(oc1547_gp_word_addr(c, 0x694)).sw(c, a1);
        Addr(oc1547_gp_word_addr(c, 0x69c)).sw(c, a0);
        Addr(oc1547_gp_word_addr(c, 0x6a0)).sw(c, v1);
        Addr(oc1547_gp_word_addr(c, 0x6a4)).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}

/// The address `gp + off` (for stores into the game's global block).
fn oc1547_gp_word_addr(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// `SongInstance* SongInstance_Create(...)` (0x002b1b98): allocates the 196-byte
/// instance and runs its constructor with the arguments passed through. The
/// byte in `t0` is sign-extended, and the ninth argument (`t3`) goes on the
/// stack.
pub fn song_instance_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, S1, 0x80);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[T0][0] as u32) << 24);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A3][0];
        c.g[S6][0] = c.g[T1][0];
        c.g[S7][0] = c.g[T2][0];
        c.g[S1][0] = c.g[T3][0];
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x33e0));
        c.g[A0][0] = 0xc4;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x002b_1bfc)?;
        // The ninth argument goes on the stack.
        let syncload = c.g[S1][0];
        Addr(frame.at(c, 0)).sw(c, syncload);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[T0][0] = c.g[S5][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S6][0];
        c.g[T3][0] = c.g[S7][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b_1c50, 0x002b_1c24)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    #[test]
    fn ssx_audio_on_music_set_event_state_matches() {
        check(
            ee::s07::f_0028f3c8__generated,
            ssx_audio_on_music_set_event_state,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0028_f3c8, 0);
                let this = DATA + 0x1000;
                let desc = DATA + 0x1200;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx([10u32, 11, 18, 12, 0x12, 3][(seed % 6) as usize]);
                c.bus.write32(GAME_GP + 0x530, (seed % 2 == 0) as u32);
                c.bus.write32(this + 0x5558, desc);
                c.bus.write16(desc + 0x18, (r.next_u32() % 0x80) as u16);
                c.bus.write32(desc + 0x1c, 0x0014_06c0);
                c.bus.write32(this + 0x530, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_restart_race_music_matches() {
        check(
            ee::s07::f_0028fae0__generated,
            ssx_audio_restart_race_music,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0028_fae0, 0);
                let this = DATA + 0x1000;
                let ptr = DATA + 0x1400;
                let obj = DATA + 0x1800;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x118, ptr);
                c.bus.write32(ptr, obj);
                c.bus.write32(this + 0x530, (seed % 3) as u32);
                c.bus.write32(this + 0x118 + 0x7, r.next_u32());
                // The music player's song picker (`MusicPlayer_SelectNextSong`)
                // scans a bit mask of `count` songs for a set bit: without a
                // set bit below `count` it never ends. Always give it one.
                let player = this + 0x118;
                let count = 1 + seed % 8;
                c.bus.write32(player + 0x3e4, count);
                c.bus.write32(player + 0x3f0, seed % count);
                c.bus.write64(player + 0x3f8, 1u64 << (r.next_u32() % count));
                c.bus.write32(GAME_GP + 0x530, (seed % 2) as u32);
                c.bus.write32(this + 0x5558, DATA + 0x1a00);
            },
        );
    }

    #[test]
    fn midibanks_play_timer_callback_matches() {
        check(
            ee::s08::f_002a8b28__generated,
            midibanks_play_timer_callback,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002a_8b28, 0);
                let ctl = DATA + 0x2000;
                let mgr = DATA + 0x2100;
                let obj = DATA + 0x2900;
                c.bus.write32(GAME_GP + 0x654, ctl);
                c.bus.write32(ctl, mgr);
                let m_inst = seed % 4;
                c.bus.write32(mgr + 0x1f4, m_inst);
                c.bus.write32(mgr + 0x214, DATA + 0x2400);
                c.bus.write32(mgr + 0x210, DATA + 0x2600);
                c.bus.write32(mgr + 0x1f8, DATA + 0x2800);
                let snd_x = [0u32, 50, 126, 127, 300, 0xffff_fff0][(seed % 6) as usize];
                c.bus.write32(DATA + 0x2600 + 4 * m_inst, snd_x);
                c.bus.write32(obj + 0x18, DATA + 0x2a00);
                c.bus.write32(obj + 0x1c, r.next_u32());
                c.bus.write32(obj + 0x20, r.next_u32());
                c.bus.write32(obj + 0x24, r.next_u32());
                c.g[A1][0] = sx(obj);
            },
        );
    }

    #[test]
    fn path_plat_init_matches() {
        check(
            ee::s08::f_002ae048__generated,
            path_plat_init,
            100,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x002a_e048, 0);
                c.g[A0][0] = sx(DATA + 0x3000 + seed * 0x10);
                c.g[A1][0] = sx(DATA + 0x3100 + (seed % 4) * 0x100);
            },
        );
    }

    #[test]
    fn song_instance_create_matches() {
        check(
            ee::s08::f_002b1b98__generated,
            song_instance_create,
            100,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x002b_1b98, 0);
                elf_image(c);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = sx(DATA + 0x4100);
                c.g[A2][0] = sx(DATA + 0x4200);
                c.g[A3][0] = seed as u64;
                c.g[T0][0] = (seed as u64).wrapping_mul(37);
                c.g[T1][0] = sx(DATA + 0x4300);
                c.g[T2][0] = (seed % 4) as u64;
                c.g[T3][0] = (seed % 2) as u64;
            },
        );
    }
}
