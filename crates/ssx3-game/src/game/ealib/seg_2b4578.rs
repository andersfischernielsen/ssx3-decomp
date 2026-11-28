//! `src/ealib/seg_2b4578.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::lib::snd::SND_HI;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_47f8, eac_unpack, "EAC_Unpack"),
    (0x003b_8de8, sndmidi_update_fade, "SNDMIDI_UpdateFade"),
    (0x003b_8ee8, sndmidi_notes_finished, "SNDMIDI_NotesFinished"),
    (0x003b_8f78, sndmidi_note_on, "SNDMIDI_NoteOn"),
    (0x003b_9140, sndmidi_note_off, "SNDMIDI_NoteOff"),
    (0x003b_91e0, sndmidi_control_change, "SNDMIDI_ControlChange"),
    (0x003b_93c8, sndmidi_pitch_bend, "SNDMIDI_PitchBend"),
    (0x003b_97d8, snd_midi_init, "SNDMIDI_Init"),
    (0x003b_98c0, snd_midi_stop_all, "SNDMIDI_StopAll"),
    (0x003b_9920, snd_midi_stop, "SNDMIDI_Stop"),
    (0x003b_9b68, snd_midi_set_pitch, "SNDMIDI_SetPitch"),
    (0x003b_9c20, snd_midi_set_pan, "SNDMIDI_SetPan"),
    (0x003b_9dc8, snd_midi_resume, "SNDMIDI_Resume"),
    (0x003b_9e48, snd_midi_set_volume, "SNDMIDI_SetVolume"),
    (0x003b_9f38, snd_midi_fade_volume, "SNDMIDI_FadeVolume"),
    (0x003b_9fc8, snd_midi_get_status, "SNDMIDI_GetStatus"),
];

/// `RefPack_Unpack` (0x3b4740).
pub(crate) const OC327_REFPACK_UNPACK: Callee = Callee {
    addr: 0x003b_4740,
    f: ee::s13::RefPack_Unpack,
};

// ---------------------------------------------------------------------------
// Streaming / sound DSP.

/// `void* EAC_Unpack(...)` (0x3b47f8): a one-call wrapper over
/// `RefPack_Unpack` (the EA "refpack" decompressor).
pub fn eac_unpack(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(OC327_REFPACK_UNPACK),
        0x003b_4808,
    );
}

/// `SNDMIDI_UpdateFade` (0x003b8de8): SNDI_midiupdatefade: steps the volume fade of MIDI slot `a0`: for each channel record
/// of the slot whose key matches, the fade value is advanced and `SND_SetVolume` is called.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_update_fade(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_3b8e40: {
            'b_3b8e38: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V1][0] = 0x70;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x510000;
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.mult(0, 3, 19, 3);
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let t = (c.g[V0][0] as i64) >= 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                if t {
                    break 'b_3b8e38;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    let t = true;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if t {
                        break 'b_3b8e40;
                    } else {
                        break 'b_3b8e38;
                    }
                }
            }
            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            break 'b_3b8e40;
        }
        'b_3b8e54: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_3b8e54;
            } else {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_3b8e54;
            }
        }
        'b_3b8ec8: {
            'b_3b8e6c: {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 0x510000;
                if t {
                    break 'b_3b8e6c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003b8e64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    Callee {
                        addr: 0x003b9920,
                        f: ee::s13::SNDMIDI_Stop,
                    }
                    .call(c, 0x003b8e64)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_3b8ec8;
                    } else {
                        break 'b_3b8e6c;
                    }
                }
            }
            c.cycles += 3;
            c.g[S2][0] = 0x17;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57e0));
            'l_3b8e78: loop {
                'b_3b8ebc: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        break 'b_3b8ebc;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                        if (c.g[A0][0] as i64) < 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_3b8ebc;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x16));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 10);
                            c.g[A1][0] = c.g[A1][0] & 0x7f;
                            c.mult(0, 5, 5, 3);
                            c.g[RA][0] = sx(0x003b8eb8);
                            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                            Callee {
                                addr: 0x003b85f0,
                                f: ee::s13::SND_SetVolume,
                            }
                            .call(c, 0x003b8eb8)?;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_3b8ebc;
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) >= 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b8e78);
                    }
                    continue 'l_3b8e78;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_3b8ec8;
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `SNDMIDI_NotesFinished` (0x003b8ee8): SNDI_midiseqover: scans the 24 note records for channel `a0`; returns 1 when none of
/// them is still sounding (`SND_HandleToChannel` finds it), else 0.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_notes_finished(c: &mut Cpu) {
    if c.pc != 0x003b8ee8 {
        return sndmidi_notes_finished_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003b8ee8;
        loop {
            match pc {
                0x003b8ee8 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = 0x510000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57e0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = 0x1;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    let t = true;
                    c.g[S3][0] = c.g[A0][0];
                    if t {
                        pc = 0x003b8f1c;
                        continue;
                    }
                    pc = 0x003b8f18;
                    continue;
                }
                0x003b8f18 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x003b8f1c;
                    continue;
                }
                0x003b8f1c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        pc = 0x003b8f58;
                        continue;
                    }
                    pc = 0x003b8f28;
                    continue;
                }
                0x003b8f28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                        if back_edge_unwinds(c, 0x003b8f18) {
                            return Ok(());
                        }
                        pc = 0x003b8f18;
                        continue;
                    }
                    pc = 0x003b8f34;
                    continue;
                }
                0x003b8f34 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    let t = (c.g[A0][0] as i64) < 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x003b8f18) {
                            return Ok(());
                        }
                        pc = 0x003b8f18;
                        continue;
                    }
                    pc = 0x003b8f40;
                    continue;
                }
                0x003b8f40 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003b8f48);
                    Callee {
                        addr: 0x003ba6e8,
                        f: ee::s13::SND_HandleToChannel,
                    }
                    .call(c, 0x003b8f48)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003b8f1c) {
                            return Ok(());
                        }
                        pc = 0x003b8f1c;
                        continue;
                    }
                    pc = 0x003b8f50;
                    continue;
                }
                0x003b8f50 => {
                    c.cycles += 2;
                    c.g[S2][0] = 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003b8f58;
                    continue;
                }
                0x003b8f58 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b8ee8, 0x003b8f74);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `sndmidi_notes_finished` (converted from the `generated` body): any entry
/// other than 0x003b8ee8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn sndmidi_notes_finished_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003b8ee8 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = 0x510000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57e0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = 0u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = 0x1;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    let t = true;
                    c.g[S3][0] = c.g[A0][0];
                    if t {
                        pc = 0x003b8f1c;
                        continue;
                    }
                    pc = 0x003b8f18;
                    continue;
                }
                0x003b8f18 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x003b8f1c;
                    continue;
                }
                0x003b8f1c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        pc = 0x003b8f58;
                        continue;
                    }
                    pc = 0x003b8f28;
                    continue;
                }
                0x003b8f28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                        if back_edge_unwinds(c, 0x003b8f18) {
                            return Ok(());
                        }
                        pc = 0x003b8f18;
                        continue;
                    }
                    pc = 0x003b8f34;
                    continue;
                }
                0x003b8f34 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    let t = (c.g[A0][0] as i64) < 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x003b8f18) {
                            return Ok(());
                        }
                        pc = 0x003b8f18;
                        continue;
                    }
                    pc = 0x003b8f40;
                    continue;
                }
                0x003b8f40 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003b8f48);
                    Callee {
                        addr: 0x003ba6e8,
                        f: ee::s13::SND_HandleToChannel,
                    }
                    .call(c, 0x003b8f48)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003b8f1c) {
                            return Ok(());
                        }
                        pc = 0x003b8f1c;
                        continue;
                    }
                    pc = 0x003b8f50;
                    continue;
                }
                0x003b8f50 => {
                    c.cycles += 2;
                    c.g[S2][0] = 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003b8f58;
                    continue;
                }
                0x003b8f58 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003b8ee8, 0x003b8f74);
                    return Ok(());
                }
            }
        }
    });
}

/// `SNDMIDI_NoteOn` (0x003b8f78): SNDI_midiparsenoteon: note-on for MIDI channel `a0` (note `a2`, velocity `a3`, song
/// record `a1`): takes a free note record, plays the patch (`SND_Play`) and stores the
/// handle.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_note_on(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
        c.g[T0][0] = 0x450000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
        c.g[A0][0] = 0x70;
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_sub(0x3b8c));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.mult(0, 4, 2, 4);
        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x19));
        c.g[V0][0] = 0x510000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[V1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = 0u64;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S6][0] = c.g[A3][0];
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S7][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[FP][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.sw((c.g[T0][0] as u32).wrapping_sub(0x3b8c), c.g[V1][0]);
        c.g[S3][0] = c.g[T0][0];
        'l_3b8ff0: loop {
            'b_3b9034: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x18;
                if t {
                    break 'b_3b9034;
                } else {
                    'b_3b9010: {
                        c.cycles += 5;
                        c.g[V1][0] = 0x510000;
                        c.div(0, 16, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x57e0));
                        if c.g[V0][0] == 0u64 {
                            break 'b_3b9010;
                        } else {
                            break 'b_3b9010;
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = c.hi[0];
                    c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                    c.sw((c.g[S3][0] as u32).wrapping_sub(0x3b8c), c.g[A1][0]);
                    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x003b9028);
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    Callee {
                        addr: 0x003ba6e8,
                        f: ee::s13::SND_HandleToChannel,
                    }
                    .call(c, 0x003b9028)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x003b8ff0);
                        }
                        continue 'l_3b8ff0;
                    } else {
                        c.cycles += 1;
                        c.g[S4][0] = 0x1;
                        break 'b_3b9034;
                    }
                }
            }
            'b_3b9110: {
                'b_3b9104: {
                    c.cycles += 2;
                    if c.g[S4][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_3b9104;
                    } else {
                        'b_3b90c4: {
                            'b_3b90b8: {
                                c.cycles += 25;
                                c.g[RA][0] = sx(0x003b9044);
                                c.g[A0][0] = c.g[SP][0];
                                Callee {
                                    addr: 0x003ba020,
                                    f: ee::s13::SND_InitPlayParams,
                                }
                                .call(c, 0x003b9044)?;
                                c.g[A0][0] = c.lw(c.g[S7][0] as u32);
                                c.g[T0][0] = 0x9;
                                c.g[A2][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x16));
                                c.g[V1][0] = sx((c.g[A0][0] as u32) >> 10);
                                c.g[A1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.g[V1][0] & 0x7f;
                                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.mult(0, 3, 3, 6);
                                c.g[V0][0] = c.g[A0][0] & 0x3ff;
                                c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x22));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 17);
                                c.g[A0][0] = c.g[A0][0] & 0x7f;
                                c.g[V0][0] = c.g[V0][0] & 0xffff;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 7);
                                c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.sb((c.g[SP][0] as u32).wrapping_add(0x3), c.g[A1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                let t = c.g[FP][0] != c.g[T0][0];
                                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                                if t {
                                    break 'b_3b90b8;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x2e));
                                    let t = (c.g[A0][0] as i64) < 0;
                                    c.g[V0][0] = 0x3c;
                                    if t {
                                        break 'b_3b90b8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A1][0] = c.g[S6][0];
                                        let t = true;
                                        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                        if t {
                                            break 'b_3b90c4;
                                        } else {
                                            break 'b_3b90b8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x3));
                            c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x2c));
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[S6][0]);
                            break 'b_3b90c4;
                        }
                        c.cycles += 11;
                        c.g[RA][0] = sx(0x003b90cc);
                        c.g[A2][0] = c.g[SP][0];
                        Callee {
                            addr: 0x003b6820,
                            f: ee::s13::SND_Play,
                        }
                        .call(c, 0x003b90cc)?;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sb((c.g[S2][0] as u32).wrapping_add(0x7), c.g[V1][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        let t = (c.g[V1][0] as i64) < 0;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        if t {
                            break 'b_3b9110;
                        } else {
                            c.cycles += 5;
                            c.sb((c.g[S2][0] as u32).wrapping_add(0x6), c.g[S6][0]);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sb((c.g[S2][0] as u32).wrapping_add(0x5), c.g[FP][0]);
                            let t = true;
                            c.sb((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            if t {
                                break 'b_3b9110;
                            } else {
                                break 'b_3b9104;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                break 'b_3b9110;
            }
            c.cycles += 11;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `SNDMIDI_NoteOff` (0x003b9140): SNDI_midiparsenoteoff: note-off for key `a1` on MIDI channel `a0`, velocity `a2`:
/// every matching record is keyed off with `SND_ChannelKeyOff`.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_note_off(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        c.cycles += 16;
        c.g[T0][0] = c.g[A0][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[A0][0] = 0x70;
        c.g[V1][0] = 0x510000;
        c.mult(0, 4, 8, 4);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x700));
        c.g[A3][0] = 0u64;
        c.g[S1][0] = 0xffffffffffffffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        'l_3b9180: loop {
            'b_3b91cc: {
                'b_3b91c4: {
                    'b_3b91c0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != c.g[T0][0] {
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            break 'b_3b91c0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x5));
                            let t = c.g[V0][0] != c.g[A1][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_3b91c0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x6));
                                let t = c.g[V0][0] != c.g[A2][0];

                                if t {
                                    break 'b_3b91c0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x7));
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                                    if t {
                                        break 'b_3b91c4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x003b91b8);
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        Callee {
                                            addr: 0x003c7138,
                                            f: ee::s13::SND_ChannelKeyOff,
                                        }
                                        .call(c, 0x003b91b8)?;
                                        let t = true;
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x7), c.g[S1][0]);
                                        if t {
                                            break 'b_3b91cc;
                                        } else {
                                            break 'b_3b91c0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 24i64) as u64;
                    break 'b_3b91c4;
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003b9180);
                    }
                    continue 'l_3b9180;
                } else {
                    break 'b_3b91cc;
                }
            }
            c.cycles += 5;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `SNDMIDI_ControlChange` (0x003b91e0): SNDI_midiparsecontrol: MIDI control change `a3` on channel `a0` (value `a2`, song
/// record `a1`): volume (7) and pan (10) reach the voices through `SND_SetVolume` and
/// `SND_SetPan`; the other controllers update the channel record.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_control_change(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_3b93a8: {
            'b_3b93a4: {
                'b_3b93a0: {
                    'b_3b9238: {
                        c.cycles += 18;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.g[V1][0] = 0x70;
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = 0x510000;
                        c.g[S4][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mult(0, 3, 20, 3);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[V0][0] = 0x14;
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S3][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S5][0] = c.g[A2][0];
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        if t {
                            break 'b_3b9238;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                            if t {
                                break 'b_3b93a0;
                            } else {
                                break 'b_3b9238;
                            }
                        }
                    }
                    'b_3b939c: {
                        'b_3b9398: {
                            'b_3b927c: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x1e;
                                let t = c.g[A3][0] != c.g[V0][0];
                                c.g[V0][0] = 0x7;
                                if t {
                                    break 'b_3b927c;
                                } else {
                                    'b_3b9264: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x26));
                                        c.g[V0][0] = (c.g[V0][0] < 0x7fu64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lbu((c.g[S0][0] as u32).wrapping_add(0x26));
                                            break 'b_3b9264;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                            c.sb(
                                                (c.g[S0][0] as u32).wrapping_add(0x26),
                                                c.g[V1][0],
                                            );
                                            c.g[V0][0] =
                                                c.lbu((c.g[S0][0] as u32).wrapping_add(0x26));
                                            break 'b_3b9264;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_3b9398;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        c.sb((c.g[S0][0] as u32).wrapping_add(0x26), c.g[V0][0]);
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        if t {
                                            break 'b_3b93a4;
                                        } else {
                                            break 'b_3b927c;
                                        }
                                    }
                                }
                            }
                            'b_3b930c: {
                                c.cycles += 2;
                                let t = c.g[A3][0] != c.g[V0][0];
                                c.g[V0][0] = 0xa;
                                if t {
                                    break 'b_3b930c;
                                } else {
                                    c.cycles += 15;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = 0xfffffffffffe0000;
                                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[V1][0] = c.g[V1][0] | 0x3ff;
                                    c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x700));
                                    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = c.g[V0][0] & 0x7f;
                                    c.g[S2][0] = 0x17;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
                                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                    c.sw(c.g[S3][0] as u32, c.g[A0][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                    'l_3b92c0: loop {
                                        'b_3b92fc: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
                                            if c.g[V0][0] != c.g[S4][0] {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_3b92fc;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
                                                if c.g[V0][0] != c.g[S5][0] {
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                    break 'b_3b92fc;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                                                    c.g[V0][0] = c
                                                        .lh((c.g[S0][0] as u32).wrapping_add(0x16));
                                                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 10);
                                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                    c.g[A1][0] = c.g[A1][0] & 0x7f;
                                                    c.mult(0, 5, 5, 2);
                                                    c.g[RA][0] = sx(0x003b92f8);
                                                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
                                                    Callee {
                                                        addr: 0x003b85f0,
                                                        f: ee::s13::SND_SetVolume,
                                                    }
                                                    .call(c, 0x003b92f8)?;
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                    break 'b_3b92fc;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = (c.g[S2][0] as i64) >= 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x003b92c0);
                                            }
                                            continue 'l_3b92c0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_3b93a8;
                                            } else {
                                                break 'b_3b930c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[A3][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_3b939c;
                            } else {
                                c.cycles += 13;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x700));
                                c.g[S2][0] = 0x17;
                                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[RA][0] = sx(0x003b932c);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                Callee {
                                    addr: 0x003bc570,
                                    f: ee::s13::SND_MidiToVolume,
                                }
                                .call(c, 0x003b932c)?;
                                c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                                c.g[A0][0] = 0xfffffffffffffc00;
                                c.g[V0][0] = c.g[V0][0] & 0x3ff;
                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.sw(c.g[S3][0] as u32, c.g[V1][0]);
                                'l_3b9348: loop {
                                    'b_3b9388: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if c.g[V0][0] != c.g[S4][0] {
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                            break 'b_3b9388;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
                                            if c.g[V0][0] != c.g[S5][0] {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_3b9388;
                                            } else {
                                                c.cycles += 10;
                                                c.g[A1][0] = c.lw(c.g[S3][0] as u32);
                                                c.g[A2][0] = 0u64;
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S0][0] as u32).wrapping_add(0x20));
                                                c.g[A1][0] = c.g[A1][0] & 0x3ff;
                                                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                                                c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[RA][0] = sx(0x003b9384);
                                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                                Callee {
                                                    addr: 0x003b8c00,
                                                    f: ee::s13::SND_SetPan,
                                                }
                                                .call(c, 0x003b9384)?;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_3b9388;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = (c.g[S2][0] as i64) >= 0;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x003b9348);
                                        }
                                        continue 'l_3b9348;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        if t {
                                            break 'b_3b93a8;
                                        } else {
                                            break 'b_3b9398;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_3b939c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    break 'b_3b93a0;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                break 'b_3b93a4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_3b93a8;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `SNDMIDI_PitchBend` (0x003b93c8): SNDI_midiparsepitchbend: pitch bend `a3` on MIDI channel `a0` (bend value `a2`, song
/// record `a1`): the channel's voices get `SND_SetPitchBend`.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_pitch_bend(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        c.cycles += 36;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V1][0] = 0x510000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = 0x70;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mult(0, 4, 19, 4);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x700));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T0][0] = 0xffffffffffffffff;
        c.g[S1][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = 0xffffffffff010000;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        c.g[S2][0] = 0x17;
        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[T0][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f));
        if c.g[T0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x7f;
        c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 17);
        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
        c.sw(c.g[S1][0] as u32, c.g[A0][0]);
        'l_3b9458: loop {
            'b_3b9484: {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] != c.g[S3][0];
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_3b9484;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x5));
                    let t = c.g[V0][0] != c.g[S4][0];

                    if t {
                        break 'b_3b9484;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
                        c.g[RA][0] = sx(0x003b9484);
                        c.g[A1][0] = c.g[A1][0] & 0x7f;
                        Callee {
                            addr: 0x003bc6b8,
                            f: ee::s13::SND_SetPitchBend,
                        }
                        .call(c, 0x003b9484)?;
                        break 'b_3b9484;
                    }
                }
            }
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003b9458);
                }
                continue 'l_3b9458;
            } else {
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// The MIDI slot table (`0x510000 - 0x5ee0`): 0x70-byte slot records, the
/// slot handle at +0x10 and the status byte at +0x28.
#[allow(dead_code)]
pub(crate) const MIDI_TABLE: u32 = 0x0050_a120;

/// The channel records (8 bytes each, 23 of them) at `MIDI_TABLE + 0x700`:
/// the voice word at +0, the slot index byte at +4.
#[allow(dead_code)]
pub(crate) const MIDI_CHANS: u32 = 0x0050_a820;

/// The sound block (`0x510000 - 0x5718`) holding the tick callback table.
#[allow(dead_code)]
pub(crate) const MIDI_BLOCK: u32 = 0x0050_a8e8;

/// `SND_Lock`, `SND_Unlock` and `SND_AddTickCallback` (the sound lock and
/// the tick callback registration).
pub(crate) const MIDI_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const MIDI_SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const MIDI_TICK_ADD: u32 = 0x003b_5910;

/// `SNDMIDI_GetSlot(handle)`: the slot index of a handle, or -8.
pub(crate) const MIDI_GET_SLOT: u32 = 0x003b_8d98;

/// `SND_SetVolume`, `SND_Stop`, `SND_SetPitch`, `SND_SetPan`: the channel
/// operations the MIDI slot functions drive.
pub(crate) const MIDI_SND_VOLUME: u32 = 0x003b_85f0;

pub(crate) const MIDI_SND_STOP: u32 = 0x003b_89e0;

pub(crate) const MIDI_SND_PITCH: u32 = 0x003b_8790;

pub(crate) const MIDI_SND_PAN: u32 = 0x003b_8c00;

/// `SNDMIDI_Stop` (called per slot by `SNDMIDI_StopAll`) and
/// `SNDMIDI_GetStatus`.
pub(crate) const MIDI_STOP: u32 = 0x003b_9920;

pub(crate) const MIDI_GET_STATUS: u32 = 0x003b_9fc8;

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
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
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
                        c.sw(c.g[S0][0] as u32, c.g[S4][0]);
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
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.g[V1][0] &= 0x3ff;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                c.g[S2][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                                c.g[V0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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

// Callees (generated bodies reached through their override slots).
pub(crate) const OC1146_GET_SLOT: Callee = Callee {
    addr: 0x003b_8d98,
    f: ee::s13::SNDMIDI_GetSlot,
};

/// `SNDMIDI_GetStatus` (0x3b9fc8): the status of one MIDI slot. Looks the slot
/// up with `SNDMIDI_GetSlot`; a negative slot is returned as is. Otherwise the
/// byte at +0x28 of the slot's 0x70-byte entry is returned, unless it is 3:
/// then the result is 6 when the word at +0x18 is set, else 3.
pub fn snd_midi_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC1146_GET_SLOT.call(c, 0x003b_9fd8)?;
        c.g[A1][0] = c.g[V0][0];
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[V1][0] = 0x70;
        // Whether the common tail (one cycle) runs: not when the entry's
        // byte differs from 3, which leaves through the outer exit.
        let mut tail = true;
        if !negative {
            c.cycles += 8;
            c.g[V0][0] = SND_HI as u64;
            c.mult(0, 3, 5, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
            c.g[A0][0] = 0x3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lb(c);
            if c.g[A1][0] != c.g[A0][0] {
                tail = false;
            } else {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
                c.g[V0][0] = 0x6;
                if c.g[V1][0] != 0 {
                    c.g[A1][0] = c.g[V0][0];
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0];
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use ssx3_ee_rt::cpu::sx;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------------------ fake callees

    /// Control word of this module's gated fakes (in the machine's RAM, so it
    /// never leaks into another test even when tests share a process).
    pub(crate) const OC327_CTRL: u32 = DATA + 0x3e_0000;

    pub(crate) const OC327_MAGIC: u32 = 0x0c32_7f00;

    /// One forced result word per fake.
    pub(crate) const OC327_RET: u32 = DATA + 0x3e_0010;

    pub(crate) const RET_REFPACK: u32 = 0;

    /// A logging fake when this module's control word is set, otherwise
    /// whatever was installed before (chained, as `oc318` does).
    pub(crate) fn oc327_fake_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC327_CTRL) == OC327_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                // Sign-extended, as `abi::set_ret` leaves it: `-1` is a
                // meaningful result for the script lookup.
                c.g[V0][0] = sx(c.bus.read32(OC327_RET + 4 * ret_idx));
            }
            return;
        }
        crate::difftest::fakes::fall_back(c, prev, fallback)
    }

    macro_rules! oc327_hook {
        ($fake:ident, $prev:ident, $addr:expr, $ret:expr, $fallback:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc327_fake_gated(c, $addr, $ret, &$prev, $fallback);
            }
        };
    }
    pub(crate) use oc327_hook;

    oc327_hook!(
        oc327_fake_refpack,
        OC327_PREV_REFPACK,
        0x003b_4740,
        RET_REFPACK,
        ee::s13::f_003b4740__generated
    );

    pub(crate) fn oc327_install_refpack() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003B4740),
            oc327_fake_refpack,
            &OC327_PREV_REFPACK,
        );
    }

    /// `EAC_Unpack`: a one-call wrapper (the decompressor is a fake).
    #[test]
    fn eac_unpack_matches() {
        oc327_install_refpack();
        covered(
            ee::s13::f_003b47f8__generated,
            eac_unpack,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003b_47f8, 1);
                c.pc = 0x003b_47f8;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_REFPACK, 0x1000 + seed);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }

    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::Rng;
    use std::ptr::addr_of_mut;

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::lib::snd::tests::OC1146_GATE;
    pub(crate) use crate::game::lib::snd::tests::OC1146_MAGIC;
    pub(crate) use crate::game::lib::snd::tests::OC1146_RESULT;

    pub(crate) use crate::game::lib::snd::tests::oc1146_hook;

    /// Gate word of this module's fakes (in each test machine's RAM), its magic.
    pub(crate) const CTRL: u32 = DATA + 0x21_1440;

    pub(crate) const MAGIC: u32 = 0x1144_c0de;

    /// Scripted results: 8 entries of `(target, v0, calls left, unused)`.
    pub(crate) const TAB: u32 = CTRL + 0x10;

    /// The sound channel and note tables get small random words, so the
    /// byte compares of the MIDI parsers take both arms.
    pub(crate) const STATE: u32 = 0x0050_a000;

    pub(crate) const STATE_WORDS: u32 = 0x800;

    /// Pointer argument for the MIDI parsers (a scratch word in the test RAM).
    pub(crate) const PTR: u32 = DATA + 0x200;

    pub(crate) const HANDLE_TO_CHANNEL: u32 = 0x003b_a6e8;

    pub(crate) const NEXT_HANDLE_CHANNEL: u32 = 0x003b_aa00;

    /// A logged fake callee (may unwind) whose `v0` comes from the script.
    pub(crate) fn oc1144_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for i in 0..8 {
            let at = TAB + 16 * i;
            if c.bus.read32(at) == target {
                let left = c.bus.read32(at + 8);
                let v = if left > 0 {
                    c.bus.write32(at + 8, left - 1);
                    c.bus.read32(at + 4)
                } else {
                    0
                };
                c.g[V0][0] = sx(v);
                return;
            }
        }
    }

    /// Script `v0` for calls to `target`: `value` for the next `times` calls,
    /// then 0.
    pub(crate) fn oc1144_script(c: &mut Cpu, target: u32, value: u32, times: u32) {
        for i in 0..8 {
            let at = TAB + 16 * i;
            let t = c.bus.read32(at);
            if t == 0 || t == target {
                c.bus.write32(at, target);
                c.bus.write32(at + 4, value);
                c.bus.write32(at + 8, times);
                return;
            }
        }
        panic!("oc1144 script full");
    }

    /// The gated fake of one callee: the gate on, it logs and scripts the
    /// call; off, it runs the body it replaced.
    macro_rules! oc1144_gated {
        ($name:ident, $prev:ident, $target:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CTRL) == MAGIC {
                    return oc1144_fake(c, $target);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }
    #[allow(unused_imports)]
    pub(crate) use oc1144_gated;

    oc1144_gated!(
        oc1144_fake_handle_to_channel,
        OC1144_PREV_HANDLE_TO_CHANNEL,
        0x003b_a6e8,
        ee::s13::f_003ba6e8__generated
    );

    oc1144_gated!(
        oc1144_fake_next_handle,
        OC1144_PREV_NEXT_HANDLE,
        0x003b_aa00,
        ee::s13::f_003baa00__generated
    );

    oc1144_gated!(
        oc1144_fake_add_dsp,
        OC1144_PREV_ADD_DSP,
        0x003c_5068,
        ee::s13::f_003c5068__generated
    );

    oc1144_gated!(
        oc1144_fake_remove_dsp,
        OC1144_PREV_REMOVE_DSP,
        0x003c_5128,
        ee::s13::f_003c5128__generated
    );

    oc1144_gated!(
        oc1144_fake_midi_stop,
        OC1144_PREV_MIDI_STOP,
        0x003b_9920,
        ee::s13::f_003b9920__generated
    );

    oc1144_gated!(
        oc1144_fake_set_volume,
        OC1144_PREV_SET_VOLUME,
        0x003b_85f0,
        ee::s13::f_003b85f0__generated
    );

    oc1144_gated!(
        oc1144_fake_set_pan,
        OC1144_PREV_SET_PAN,
        0x003b_8c00,
        ee::s13::f_003b8c00__generated
    );

    oc1144_gated!(
        oc1144_fake_play,
        OC1144_PREV_PLAY,
        0x003b_6820,
        ee::s13::f_003b6820__generated
    );

    oc1144_gated!(
        oc1144_fake_key_off,
        OC1144_PREV_KEY_OFF,
        0x003c_7138,
        ee::s13::f_003c7138__generated
    );

    oc1144_gated!(
        oc1144_fake_set_pitch_bend,
        OC1144_PREV_SET_PITCH_BEND,
        0x003b_c6b8,
        ee::s13::f_003bc6b8__generated
    );

    oc1144_gated!(
        oc1144_fake_midi_to_volume,
        OC1144_PREV_MIDI_TO_VOLUME,
        0x003b_c570,
        ee::s13::f_003bc570__generated
    );

    pub(crate) fn oc1144_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
            oc1144_fake_handle_to_channel,
            &OC1144_PREV_HANDLE_TO_CHANNEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BAA00),
            oc1144_fake_next_handle,
            &OC1144_PREV_NEXT_HANDLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C5068),
            oc1144_fake_add_dsp,
            &OC1144_PREV_ADD_DSP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C5128),
            oc1144_fake_remove_dsp,
            &OC1144_PREV_REMOVE_DSP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B9920),
            oc1144_fake_midi_stop,
            &OC1144_PREV_MIDI_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B85F0),
            oc1144_fake_set_volume,
            &OC1144_PREV_SET_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8C00),
            oc1144_fake_set_pan,
            &OC1144_PREV_SET_PAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B6820),
            oc1144_fake_play,
            &OC1144_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C7138),
            oc1144_fake_key_off,
            &OC1144_PREV_KEY_OFF,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BC6B8),
            oc1144_fake_set_pitch_bend,
            &OC1144_PREV_SET_PITCH_BEND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BC570),
            oc1144_fake_midi_to_volume,
            &OC1144_PREV_MIDI_TO_VOLUME,
        );
    }

    /// A machine for one seed: the callee fakes hooked and gated on, the
    /// sound tables randomised, the handle lookup scripted (`seed % 3`
    /// picks a missing handle, channel 0 or a random channel) and the next
    /// handle scripted for `seed % 3` calls. `pc` at the function's entry.
    pub(crate) fn oc1144_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        oc1144_install_fakes();
        let mut r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(CTRL, MAGIC);
        for i in 0..STATE_WORDS {
            c.bus.write32(STATE + 4 * i, r.next_u32() % 4);
        }
        let channel = match seed % 3 {
            0 => u32::MAX,
            1 => 0,
            _ => r.next_u32() % 16,
        };
        oc1144_script(c, HANDLE_TO_CHANNEL, channel, u32::MAX);
        oc1144_script(c, NEXT_HANDLE_CHANNEL, 1, seed % 3);
        c.pc = entry;
        r
    }

    pub(crate) fn oc1144_args(c: &mut Cpu, a0: u32, a1: u32, a2: u32, a3: u32) {
        c.g[A0][0] = a0 as u64;
        c.g[A1][0] = a1 as u64;
        c.g[A2][0] = a2 as u64;
        c.g[A3][0] = a3 as u64;
    }

    #[test]
    fn sndmidi_update_fade_matches() {
        check(
            ee::s13::f_003b8de8__generated,
            sndmidi_update_fade,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0003, 0x003b_8de8);
                let slot = r.next_u32() % 4;
                oc1144_args(c, slot, 0, 0, 0);
            },
        );
    }

    #[test]
    fn sndmidi_notes_finished_matches() {
        check(
            ee::s13::f_003b8ee8__generated,
            sndmidi_notes_finished,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0004, 0x003b_8ee8);
                let channel = r.next_u32() % 4;
                oc1144_args(c, channel, 0, 0, 0);
            },
        );
    }

    #[test]
    fn sndmidi_note_on_matches() {
        check(
            ee::s13::f_003b8f78__generated,
            sndmidi_note_on,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0005, 0x003b_8f78);
                let (a0, a2, a3) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                oc1144_args(c, a0, PTR, a2, a3);
            },
        );
    }

    #[test]
    fn sndmidi_note_off_matches() {
        check(
            ee::s13::f_003b9140__generated,
            sndmidi_note_off,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0006, 0x003b_9140);
                let (a0, a1, a2) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                oc1144_args(c, a0, a1, a2, 0);
            },
        );
    }

    #[test]
    fn sndmidi_control_change_matches() {
        check(
            ee::s13::f_003b91e0__generated,
            sndmidi_control_change,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0007, 0x003b_91e0);
                let (a0, a2) = (r.next_u32() % 4, r.next_u32() % 4);
                // The controllers the parser branches on (7, 10, 20, 30) and another.
                let controller = [0x07, 0x0a, 0x14, 0x1e, 0x03][(seed % 5) as usize];
                oc1144_args(c, a0, PTR, a2, controller);
            },
        );
    }

    #[test]
    fn sndmidi_pitch_bend_matches() {
        check(
            ee::s13::f_003b93c8__generated,
            sndmidi_pitch_bend,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0008, 0x003b_93c8);
                let (a0, a2, a3) = (r.next_u32() % 4, r.next_u32() % 4, r.next_u32());
                oc1144_args(c, a0, PTR, a2, a3);
            },
        );
    }

    /// The SND lock and unlock hooks (`0x450000 - 0x3b94`, `- 0x3b90`) point
    /// at fakes, so the real `SND_Lock`/`SND_Unlock` log their calls.
    pub(crate) fn midi_world(c: &mut Cpu, seed: u32) -> Rng {
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
    pub(crate) fn midi_slots(c: &mut Cpu, seed: u32) {
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
    pub(crate) fn midi_handle(seed: u32) -> u32 {
        match seed % 9 {
            6 => 0x20,
            7 => u32::MAX,
            8 => 16 + seed % 7,
            _ => seed % 16,
        }
    }

    /// World, slots, and one channel record that uses the argument's slot.
    pub(crate) fn midi_call(c: &mut Cpu, seed: u32) -> u32 {
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
        check(
            ee::s13::f_003b97d8__generated,
            snd_midi_init,
            200,
            &|c, seed| {
                midi_world(c, seed);
            },
        );
    }

    #[test]
    fn snd_midi_stop_all_matches() {
        check(
            ee::s13::f_003b98c0__generated,
            snd_midi_stop_all,
            200,
            &|c, seed| {
                midi_world(c, seed);
                midi_slots(c, seed);
                let i = seed % 23;
                c.bus.write32(
                    MIDI_CHANS + 8 * i,
                    if seed % 4 == 0 { 0x1000 } else { u32::MAX },
                );
                c.bus.write32(MIDI_CHANS + 8 * i + 4, seed % 16);
            },
        );
    }

    #[test]
    fn snd_midi_stop_matches() {
        check(
            ee::s13::f_003b9920__generated,
            snd_midi_stop,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
            },
        );
    }

    #[test]
    fn snd_midi_set_pitch_matches() {
        check(
            ee::s13::f_003b9b68__generated,
            snd_midi_set_pitch,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
                c.g[A1][0] = sx(if seed % 3 == 0 { 0x1234 } else { 0x4321 });
            },
        );
    }

    #[test]
    fn snd_midi_set_pan_matches() {
        check(
            ee::s13::f_003b9c20__generated,
            snd_midi_set_pan,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
                c.g[A1][0] = sx(if seed % 3 == 0 { 0x1234 } else { 0x40 });
            },
        );
    }

    #[test]
    fn snd_midi_resume_matches() {
        check(
            ee::s13::f_003b9dc8__generated,
            snd_midi_resume,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
            },
        );
    }

    #[test]
    fn snd_midi_set_volume_matches() {
        check(
            ee::s13::f_003b9e48__generated,
            snd_midi_set_volume,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
                c.g[A1][0] = sx(0x40 + seed % 0x40);
                let k = handle & 0x1f;
                if k < 16 && seed % 2 == 0 {
                    c.bus
                        .write32(MIDI_TABLE + 0x70 * k + 0x14, (0x40 + seed % 0x40) << 16);
                }
            },
        );
    }

    #[test]
    fn snd_midi_fade_volume_matches() {
        check(
            ee::s13::f_003b9f38__generated,
            snd_midi_fade_volume,
            300,
            &|c, seed| {
                let handle = midi_call(c, seed);
                c.g[A0][0] = sx(handle);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx(0x40 + seed % 0x40);
            },
        );
    }

    /// `SNDMIDI_GetStatus`: slot -1 (returned as is), and slots whose entry
    /// has the byte at +0x28 equal to 3 (word at +0x18 zero or not) or not.
    #[test]
    fn snd_midi_get_status_matches() {
        oc1146_hook();
        check(
            ee::s13::f_003b9fc8__generated,
            snd_midi_get_status,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x1146_9fc8, 0);
                c.bus.write32(OC1146_GATE, OC1146_MAGIC);
                let slot = [u32::MAX, 2, 5, 0][(seed % 4) as usize];
                c.bus.write32(OC1146_RESULT, slot);
                if slot != u32::MAX {
                    let entry = (SND_HI - 0x5ee0) + slot * 0x70;
                    c.bus
                        .write8(entry + 0x28, [3, 4][((seed / 4) % 2) as usize]);
                    c.bus
                        .write32(entry + 0x18, if seed % 3 == 0 { 0 } else { 7 });
                }
            },
        );
    }
}
