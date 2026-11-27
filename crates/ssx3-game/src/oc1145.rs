//! Staging module oc1145 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1145`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_97d8, snd_midi_init, "SNDMIDI_Init"),
    (0x003b_98c0, snd_midi_stop_all, "SNDMIDI_StopAll"),
    (0x003b_9920, snd_midi_stop, "SNDMIDI_Stop"),
    (0x003b_9b68, snd_midi_set_pitch, "SNDMIDI_SetPitch"),
    (0x003b_9c20, snd_midi_set_pan, "SNDMIDI_SetPan"),
    (0x003b_9dc8, snd_midi_resume, "SNDMIDI_Resume"),
    (0x003b_9e48, snd_midi_set_volume, "SNDMIDI_SetVolume"),
    (0x003b_9f38, snd_midi_fade_volume, "SNDMIDI_FadeVolume"),
];

/// The MIDI slot table (`0x510000 - 0x5ee0`): 0x70-byte slot records, the
/// slot handle at +0x10 and the status byte at +0x28.
const MIDI_TABLE: u32 = 0x0050_a120;
/// The channel records (8 bytes each, 23 of them) at `MIDI_TABLE + 0x700`:
/// the voice word at +0, the slot index byte at +4.
const MIDI_CHANS: u32 = 0x0050_a820;
/// The sound block (`0x510000 - 0x5718`) holding the tick callback table.
const MIDI_BLOCK: u32 = 0x0050_a8e8;

/// `SND_Lock`, `SND_Unlock` and `SND_AddTickCallback` (the sound lock and
/// the tick callback registration).
const MIDI_SND_LOCK: u32 = 0x003b_58a0;
const MIDI_SND_UNLOCK: u32 = 0x003b_58d8;
const MIDI_TICK_ADD: u32 = 0x003b_5910;
/// `SNDMIDI_GetSlot(handle)`: the slot index of a handle, or -8.
const MIDI_GET_SLOT: u32 = 0x003b_8d98;
/// `SND_SetVolume`, `SND_Stop`, `SND_SetPitch`, `SND_SetPan`: the channel
/// operations the MIDI slot functions drive.
const MIDI_SND_VOLUME: u32 = 0x003b_85f0;
const MIDI_SND_STOP: u32 = 0x003b_89e0;
const MIDI_SND_PITCH: u32 = 0x003b_8790;
const MIDI_SND_PAN: u32 = 0x003b_8c00;
/// `SNDMIDI_Stop` (called per slot by `SNDMIDI_StopAll`) and
/// `SNDMIDI_GetStatus`.
const MIDI_STOP: u32 = 0x003b_9920;
const MIDI_GET_STATUS: u32 = 0x003b_9fc8;

/// `void SNDmidiinit(void)` (0x3b97d8): under the sound lock, mark the eight-
/// byte channel records free (handle -1), clear every MIDI slot's status,
/// register the tick callback and link the sound block to the callback table.
pub fn snd_midi_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, MIDI_SND_LOCK, 0x003b_97e8)?;
        // First loop: 23 eight-byte channel records from 0x50a820.
        c.g[V0][0] = 0x0051_0000;
        c.g[A1][0] = u64::MAX;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57e0));
        c.g[V1][0] = 0x17;
        loop {
            c.cycles += 7;
            c.sb((c.g[V0][0] as u32).wrapping_add(0x5), c.g[A1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sb((c.g[V0][0] as u32).wrapping_add(0x6), 0u64);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x7), 0u64);
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_9800);
            }
        }
        // Second loop: the 0x70-byte slot records of 0x50a120..0x50a820.
        c.cycles += 5;
        c.g[V0][0] = 0x0051_0000;
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x700));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        loop {
            c.cycles += 7;
            c.sb((c.g[V1][0] as u32).wrapping_add(0x28), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            if event_due(c) {
                deliver_event_here(c, 0x003b_9830);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = 0x003c_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6948));
        call_ptr(c, MIDI_TICK_ADD, 0x003b_9858)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[V0][0] = 0x003c_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6780));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1e8), c.g[V0][0]);
        call_ptr(c, MIDI_SND_UNLOCK, 0x003b_9870)?;
        c.g[V0][0] = 0u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDmidistopall(void)` (0x3b98c0): under the sound lock, stop the
/// fifteen slots 0..14 in turn (`SNDMIDI_Stop` on each slot's handle word),
/// then unlock.
pub fn snd_midi_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xf;
        call_ptr(c, MIDI_SND_LOCK, 0x003b_98d8)?;
        c.g[V0][0] = 0x0051_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        loop {
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            call_ptr(c, MIDI_STOP, 0x003b_98f4)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003b_98e8);
            }
        }
        c.cycles += 8;
        call_ptr(c, MIDI_SND_UNLOCK, 0x003b_9904)?;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = 0u64;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDseqstop(int handle)` (0x3b9920), `SNDMIDI_Stop`: stop the slot
/// of `handle`. Its channel records that use the slot are stopped (a voice
/// word >= 0 runs `SND_Stop`), the slot is marked stopped (status 2, handle
/// -1). Returns -1 if the slot is idle, 0 if it was stopped, the handle's
/// negative slot otherwise.
pub fn snd_midi_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 18, 0x40);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        frame.save(c, 19, 0x30);
        frame.save(c, 20, 0x20);
        frame.save(c, 21, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9948)?;
        c.g[S2][0] = c.g[V0][0];
        let slot_ok = (c.g[S2][0] as i64) >= 0;
        c.g[V1][0] = 0x0051_0000;
        if slot_ok {
            c.cycles += 7;
            c.g[V0][0] = 0x70;
            c.mult(0, 2, 18, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x28));
            let idle = c.g[V1][0] == 0;
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x700));
            if idle {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[S5][0] = u64::MAX;
                c.g[S4][0] = u64::MAX;
                c.g[S1][0] = 0x17;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != c.g[S2][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        if (c.g[A0][0] as i64) >= 0 {
                            c.cycles += 3;
                            call_ptr(c, MIDI_SND_STOP, 0x003b_99a0)?;
                        }
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x5), c.g[S5][0]);
                        c.cycles += 4;
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x6), 0u64);
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x7), 0u64);
                        c.sw((c.g[S0][0] as u32), c.g[S4][0]);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    }
                    c.cycles += 2;
                    let more = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_9980);
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = 0x2;
                c.g[V1][0] = u64::MAX;
                c.sb((c.g[S3][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.g[V0][0] = 0u64;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDMIDI_pitchmult(int handle, int pitch)` (0x3b9b68), `SNDMIDI_SetPitch`:
/// for the slot of `handle` whose pitch field (+0x22) differs from `pitch`,
/// store the pitch and apply it to each of its channels that have a voice.
pub fn snd_midi_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 19, 0x10);
        frame.save(c, 16, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        frame.save(c, 17, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = u64::MAX;
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9b8c)?;
        c.g[S2][0] = c.g[V0][0];
        let slot_ok = (c.g[S2][0] as i64) >= 0;
        c.g[V1][0] = 0x0051_0000;
        if slot_ok {
            c.cycles += 7;
            c.g[V0][0] = 0x70;
            c.mult(0, 2, 18, 2);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x22));
            let same = c.g[S3][0] == c.g[V1][0];
            c.g[V0][0] = 0x3;
            let mut tail = same;
            if !same {
                c.cycles += 3;
                c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x28));
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.g[S0][0];
                } else {
                    c.cycles += 4;
                    c.sh((c.g[A0][0] as u32).wrapping_add(0x22), c.g[S3][0]);
                    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                    c.g[S1][0] = 0x17;
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != c.g[S2][0] {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            if (c.g[A0][0] as i64) >= 0 {
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S3][0];
                                call_ptr(c, MIDI_SND_PITCH, 0x003b_9bf0)?;
                            }
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        }
                        c.cycles += 2;
                        let more = (c.g[S1][0] as i64) >= 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_9bd0);
                        }
                    }
                    tail = true;
                }
            }
            if tail {
                c.cycles += 2;
                c.g[S0][0] = 0u64;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDMIDI_Pan(int handle, int pan)` (0x3b9c20), `SNDMIDI_SetPan`: the
/// pan twin of `SNDMIDI_SetPitch`: the slot's pan field (+0x20) and each of
/// its voiced channels get `pan` (the channel's own volume and the voice's
/// pan scale it).
pub fn snd_midi_set_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 18, 0x30);
        frame.save(c, 17, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 19, 0x20);
        frame.save(c, 16, 0x50);
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX;
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9c48)?;
        c.g[S3][0] = c.g[V0][0];
        let slot_ok = (c.g[S3][0] as i64) >= 0;
        c.g[V1][0] = 0x0051_0000;
        if slot_ok {
            c.cycles += 7;
            c.g[V0][0] = 0x70;
            c.mult(0, 2, 19, 2);
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x20));
            let same = c.g[S2][0] == c.g[V1][0];
            c.g[V0][0] = 0x3;
            let mut tail = same;
            if !same {
                c.cycles += 3;
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x28));
                let other_type = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
                if !other_type {
                    c.cycles += 3;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x700));
                    c.g[S4][0] = 0x17;
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != c.g[S3][0] {
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A2][0] = 0u64;
                            if (c.g[A0][0] as i64) >= 0 {
                                c.cycles += 10;
                                c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
                                c.g[A1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.g[V1][0] &= 0x3ff;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A1][0] = c.g[S2][0] & 0xffff;
                                call_ptr(c, MIDI_SND_PAN, 0x003b_9cc8)?;
                            }
                            c.cycles += 1;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        }
                        c.cycles += 2;
                        let more = (c.g[S4][0] as i64) >= 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_9c88);
                        }
                    }
                    tail = true;
                }
            }
            if tail {
                c.cycles += 2;
                c.g[S1][0] = 0u64;
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDMIDI_Resume(int handle)` (0x3b9dc8): under the sound lock, a slot
/// that is paused (status 5) goes back to playing (status 3); the slot's
/// status (`SNDMIDI_GetStatus`) is the result.
pub fn snd_midi_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, MIDI_SND_LOCK, 0x003b_9de0)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9de8)?;
        c.g[S0][0] = c.g[V0][0];
        let slot_ok = (c.g[S0][0] as i64) >= 0;
        c.g[A0][0] = 0x70;
        if slot_ok {
            c.cycles += 8;
            c.g[V0][0] = 0x0051_0000;
            c.mult(0, 4, 16, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
            c.g[A1][0] = 0x5;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x28));
            let paused = c.g[V1][0] == c.g[A1][0];
            c.g[V0][0] = 0x3;
            if paused {
                c.cycles += 1;
                c.sb((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, MIDI_GET_STATUS, 0x003b_9e20)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 8;
        call_ptr(c, MIDI_SND_UNLOCK, 0x003b_9e2c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDMIDI_SetVolume(int handle, int volume)` (0x3b9e48): the slot's
/// volume word (+0x14, the 16-bit volume in its high half) is set to
/// `volume << 16` when it changes, and each voiced channel gets its volume
/// scaled by the slot's volume (`SND_SetVolume`). Returns 0, -1 when the
/// slot is not playing, or the negative slot.
pub fn snd_midi_set_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 20, 0x20);
        frame.save(c, 19, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 21, 0x10);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save_ra(c, 0);
        c.g[S5][0] = 0u64;
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9e74)?;
        c.g[S3][0] = c.g[V0][0];
        let slot_ok = (c.g[S3][0] as i64) >= 0;
        c.g[V1][0] = 0x0051_0000;
        if slot_ok {
            c.cycles += 8;
            c.g[V0][0] = 0x70;
            c.mult(0, 2, 19, 2);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
            c.g[A0][0] = sx((c.g[S4][0] as u32) << 16);
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            let same = c.g[V1][0] == c.g[A0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
            let mut tail = true;
            if !same {
                c.cycles += 4;
                c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = 0x3;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 4;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                    c.g[S2][0] = 0x17;
                    tail = false;
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] == c.g[S3][0] {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            if (c.g[A0][0] as i64) >= 0 {
                                c.cycles += 10;
                                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x5));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 10);
                                c.g[A1][0] &= 0x7f;
                                c.mult(0, 5, 5, 20);
                                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                                call_ptr(c, MIDI_SND_VOLUME, 0x003b_9efc)?;
                            }
                        }
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        c.cycles += 2;
                        let more = (c.g[S2][0] as i64) >= 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_9ec0);
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S5][0];
                } else {
                    c.g[S5][0] = u64::MAX;
                }
            }
            if tail {
                c.cycles += 1;
                c.g[V0][0] = c.g[S5][0];
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SNDMIDI_FadeVolume(int handle, int frames, int volume)` (0x3b9f38):
/// the slot's volume word gets `volume << 16`, and its per-frame step
/// (`(target - current) / frames`, with `frames` at least 1) is stored at +0x18
/// as the final volume step (+0x1c holds the target). Returns 0, or the
/// negative slot.
pub fn snd_midi_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, MIDI_GET_SLOT, 0x003b_9f54)?;
        c.g[V1][0] = c.g[V0][0];
        let slot_ok = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = 0x70;
        if slot_ok {
            c.cycles += 8;
            c.g[V0][0] = 0x0051_0000;
            c.mult(0, 4, 3, 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
            c.g[A1][0] = 0x3;
            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x28));
            let playing = c.g[V1][0] == c.g[A1][0];
            c.g[V0][0] = 0u64;
            if playing {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = sx((c.g[S1][0] as u32) << 16);
                c.g[A0][0] = 0x1;
                c.g[V0][0] = ((0u64 as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A0][0];
                }
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.div(0, 3, 16);
                c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[A1][0]);
                c.cycles += 3;
                c.g[V1][0] = c.lo[0];
                c.sw((c.g[A3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[V0][0] = 0u64;
            }
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, Rng, DATA};

    /// The SND lock and unlock hooks (`0x450000 - 0x3b94`, `- 0x3b90`) point
    /// at fakes, so the real `SND_Lock`/`SND_Unlock` log their calls.
    fn midi_world(c: &mut Cpu, seed: u32) -> Rng {
        let r = fakes::machine(c, seed, 0x1145, 3);
        c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
        c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
        // Every channel record unused: voice -1, slot index 0xff.
        for i in 0..23u32 {
            let rec = MIDI_CHANS + 8 * i;
            c.bus.write32(rec, u32::MAX);
            c.bus.write32(rec + 4, 0xff);
        }
        r
    }

    /// Slots 0..16 hold their own index as handle (a few not, so GetSlot's
    /// mismatch path runs), with statuses 0, 2, 3 and 5 in turn.
    fn midi_slots(c: &mut Cpu, seed: u32) {
        for k in 0..16u32 {
            let rec = MIDI_TABLE + 0x70 * k;
            let handle = if (seed + k) % 11 == 0 { 0xffff_fffe } else { k };
            c.bus.write32(rec + 0x10, handle);
            let status = [0u32, 2, 3, 5][((seed / 3 + k) % 4) as usize];
            c.bus.write32(rec + 0x28, status);
            c.bus.write32(rec + 0x14, (0x40 + (seed + k) % 0x40) << 16);
            // Pan field (+0x20) and pitch field (+0x22) match the test arguments
            // for some seeds (0x1234 / 0x40 / 0x4321 are the argument values).
            let pan = [0x1234u32, 0x40, 0][((seed + k) % 3) as usize];
            let pitch = [0x1234u32, 0x4321, 0][((seed / 2 + k) % 3) as usize];
            c.bus.write32(rec + 0x20, pan | (pitch << 16));
        }
    }

    /// The handle argument for a seed: mostly a valid slot, sometimes the
    /// out-of-range or negative ones.
    fn midi_handle(seed: u32) -> u32 {
        match seed % 9 {
            6 => 0x20,
            7 => u32::MAX,
            8 => 16 + seed % 7,
            _ => seed % 16,
        }
    }

    /// World, slots, and one channel record that uses the argument's slot.
    fn midi_call(c: &mut Cpu, seed: u32) -> u32 {
        midi_world(c, seed);
        midi_slots(c, seed);
        let handle = midi_handle(seed);
        let i = seed % 23;
        let word = if seed % 5 == 0 { 0x1000 } else { u32::MAX };
        c.bus.write32(MIDI_CHANS + 8 * i, word);
        c.bus.write32(MIDI_CHANS + 8 * i + 4, handle & 0x1f);
        handle
    }

    #[test]
    fn snd_midi_init_matches() {
        check(ee::s13::f_003b97d8__generated, snd_midi_init, 200, &|c, seed| {
            midi_world(c, seed);
        });
    }

    #[test]
    fn snd_midi_stop_all_matches() {
        check(ee::s13::f_003b98c0__generated, snd_midi_stop_all, 200, &|c, seed| {
            midi_world(c, seed);
            midi_slots(c, seed);
            let i = seed % 23;
            c.bus.write32(MIDI_CHANS + 8 * i, if seed % 4 == 0 { 0x1000 } else { u32::MAX });
            c.bus.write32(MIDI_CHANS + 8 * i + 4, seed % 16);
        });
    }

    #[test]
    fn snd_midi_stop_matches() {
        check(ee::s13::f_003b9920__generated, snd_midi_stop, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
        });
    }

    #[test]
    fn snd_midi_set_pitch_matches() {
        check(ee::s13::f_003b9b68__generated, snd_midi_set_pitch, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
            c.g[A1][0] = sx(if seed % 3 == 0 { 0x1234 } else { 0x4321 });
        });
    }

    #[test]
    fn snd_midi_set_pan_matches() {
        check(ee::s13::f_003b9c20__generated, snd_midi_set_pan, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
            c.g[A1][0] = sx(if seed % 3 == 0 { 0x1234 } else { 0x40 });
        });
    }

    #[test]
    fn snd_midi_resume_matches() {
        check(ee::s13::f_003b9dc8__generated, snd_midi_resume, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
        });
    }

    #[test]
    fn snd_midi_set_volume_matches() {
        check(ee::s13::f_003b9e48__generated, snd_midi_set_volume, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
            c.g[A1][0] = sx(0x40 + seed % 0x40);
            let k = handle & 0x1f;
            if k < 16 && seed % 2 == 0 {
                c.bus.write32(MIDI_TABLE + 0x70 * k + 0x14, (0x40 + seed % 0x40) << 16);
            }
        });
    }

    #[test]
    fn snd_midi_fade_volume_matches() {
        check(ee::s13::f_003b9f38__generated, snd_midi_fade_volume, 300, &|c, seed| {
            let handle = midi_call(c, seed);
            c.g[A0][0] = sx(handle);
            c.g[A1][0] = sx(seed % 4);
            c.g[A2][0] = sx(0x40 + seed % 0x40);
        });
    }
}
