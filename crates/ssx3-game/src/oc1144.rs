//! Staging module oc1144 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1144`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_8cc8, snd_apply_channel_params, "SND_ApplyChannelParams"),
    (0x003b_8d30, snd_remove_dsp, "SND_RemoveDsp"),
    (0x003b_8de8, sndmidi_update_fade, "SNDMIDI_UpdateFade"),
    (0x003b_8ee8, sndmidi_notes_finished, "SNDMIDI_NotesFinished"),
    (0x003b_8f78, sndmidi_note_on, "SNDMIDI_NoteOn"),
    (0x003b_9140, sndmidi_note_off, "SNDMIDI_NoteOff"),
    (0x003b_91e0, sndmidi_control_change, "SNDMIDI_ControlChange"),
    (0x003b_93c8, sndmidi_pitch_bend, "SNDMIDI_PitchBend"),
];

/// `SND_ApplyChannelParams` (0x003b8cc8): SNDCTRL_filteradd: the handle `a0` lookup (`SND_HandleToChannel`), then each handle
/// `SND_NextHandleChannel` yields gets `SND_ChannelAddDsp` with `a1`. Returns the lookup.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn snd_apply_channel_params(c: &mut Cpu) {
    if c.pc != 0x003b8cc8 {
        return ee::s13::f_003b8cc8__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003b8cc8;
    loop {
        match pc {
            0x003b8cc8 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003b8ce0);
                c.g[S1][0] = c.g[A1][0];
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b8ce0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S0][0] as i64) < 0; 
                c.g[V0][0] = 0xffffffffffffffff;
                if t { pc = 0x003b8d14; continue; }
                pc = 0x003b8cec; continue;
            }
            0x003b8cec => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                if t { pc = 0x003b8d00; continue; }
                pc = 0x003b8cf4; continue;
            }
            0x003b8cf4 => {
                c.cycles += 1;
                pc = 0x003b8cf8; continue;
            }
            0x003b8cf8 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003b8d00);
                c.g[A1][0] = c.g[S1][0];
                Callee { addr: 0x003c5068, f: ee::s13::SND_ChannelAddDsp }.call(c, 0x003b8d00)?;
                pc = 0x003b8d00; continue;
            }
            0x003b8d00 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003b8d0c);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003b8d0c)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                if t { if back_edge_unwinds(c, 0x003b8cf8) { return Ok(()); } pc = 0x003b8cf8; continue; }
                pc = 0x003b8d14; continue;
            }
            0x003b8d14 => {
                c.cycles += 6;
                c.g[V0][0] = c.g[S0][0];
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b8cc8, 0x003b8d2c); return Ok(()); }
        }
    }
    });
}

/// `SND_RemoveDsp` (0x003b8d30): SNDCTRL_filterremove: as `SND_ApplyChannelParams`, with `SND_ChannelRemoveDsp`.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn snd_remove_dsp(c: &mut Cpu) {
    if c.pc != 0x003b8d30 {
        return ee::s13::f_003b8d30__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003b8d30;
    loop {
        match pc {
            0x003b8d30 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003b8d48);
                c.g[S1][0] = c.g[A1][0];
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b8d48)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S0][0] as i64) < 0; 
                c.g[V0][0] = 0xffffffffffffffff;
                if t { pc = 0x003b8d7c; continue; }
                pc = 0x003b8d54; continue;
            }
            0x003b8d54 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                if t { pc = 0x003b8d68; continue; }
                pc = 0x003b8d5c; continue;
            }
            0x003b8d5c => {
                c.cycles += 1;
                pc = 0x003b8d60; continue;
            }
            0x003b8d60 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003b8d68);
                c.g[A1][0] = c.g[S1][0];
                Callee { addr: 0x003c5128, f: ee::s13::SND_ChannelRemoveDsp }.call(c, 0x003b8d68)?;
                pc = 0x003b8d68; continue;
            }
            0x003b8d68 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003b8d74);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003b8d74)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                if t { if back_edge_unwinds(c, 0x003b8d60) { return Ok(()); } pc = 0x003b8d60; continue; }
                pc = 0x003b8d7c; continue;
            }
            0x003b8d7c => {
                c.cycles += 6;
                c.g[V0][0] = c.g[S0][0];
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b8d30, 0x003b8d94); return Ok(()); }
        }
    }
    });
}

/// `SNDMIDI_UpdateFade` (0x003b8de8): SNDI_midiupdatefade: steps the volume fade of MIDI slot `a0`: for each channel record
/// of the slot whose key matches, the fade value is advanced and `SND_SetVolume` is called.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_update_fade(c: &mut Cpu) {
    if c.pc != 0x003b8de8 {
        return ee::s13::f_003b8de8__generated(c);
    }
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
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
Callee { addr: 0x003b9920, f: ee::s13::SNDMIDI_Stop }.call(c, 0x003b8e64)?;
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
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
if (c.g[A0][0] as i64) < 0 {
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b8ebc;
} else {
c.cycles += 11;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x16));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 10);
c.g[A1][0] = c.g[A1][0] & 0x7f;
c.mult(0, 5, 5, 3);
c.g[RA][0] = sx(0x003b8eb8);
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
Callee { addr: 0x003b85f0, f: ee::s13::SND_SetVolume }.call(c, 0x003b8eb8)?;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b8ebc;
}
}
}
c.cycles += 2;
let t = (c.g[S2][0] as i64) >= 0;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003b8e78); } continue 'l_3b8e78;
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
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}

/// `SNDMIDI_NotesFinished` (0x003b8ee8): SNDI_midiseqover: scans the 24 note records for channel `a0`; returns 1 when none of
/// them is still sounding (`SND_HandleToChannel` finds it), else 0.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_notes_finished(c: &mut Cpu) {
    if c.pc != 0x003b8ee8 {
        return ee::s13::f_003b8ee8__generated(c);
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
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = true; 
                c.g[S3][0] = c.g[A0][0];
                if t { pc = 0x003b8f1c; continue; }
                pc = 0x003b8f18; continue;
            }
            0x003b8f18 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x003b8f1c; continue;
            }
            0x003b8f1c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.g[S2][0];
                if t { pc = 0x003b8f58; continue; }
                pc = 0x003b8f28; continue;
            }
            0x003b8f28 => {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                
                if c.g[V0][0] != c.g[S3][0] { c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8)); if back_edge_unwinds(c, 0x003b8f18) { return Ok(()); } pc = 0x003b8f18; continue; }
                pc = 0x003b8f34; continue;
            }
            0x003b8f34 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                let t = (c.g[A0][0] as i64) < 0; 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                if t { if back_edge_unwinds(c, 0x003b8f18) { return Ok(()); } pc = 0x003b8f18; continue; }
                pc = 0x003b8f40; continue;
            }
            0x003b8f40 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b8f48);
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b8f48)?;
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x003b8f1c) { return Ok(()); } pc = 0x003b8f1c; continue; }
                pc = 0x003b8f50; continue;
            }
            0x003b8f50 => {
                c.cycles += 2;
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.g[S2][0];
                pc = 0x003b8f58; continue;
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
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b8ee8, 0x003b8f74); return Ok(()); }
        }
    }
    });
}

/// `SNDMIDI_NoteOn` (0x003b8f78): SNDI_midiparsenoteon: note-on for MIDI channel `a0` (note `a2`, velocity `a3`, song
/// record `a1`): takes a free note record, plays the patch (`SND_Play`) and stores the
/// handle.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_note_on(c: &mut Cpu) {
    if c.pc != 0x003b8f78 {
        return ee::s13::f_003b8f78__generated(c);
    }
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
c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[RA][0] = sx(0x003b9028);
c.g[A0][0] = c.lw((c.g[S2][0] as u32));
Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b9028)?;
if (c.g[V0][0] as i64) >= 0 {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x003b8ff0); } continue 'l_3b8ff0;
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
Callee { addr: 0x003ba020, f: ee::s13::SND_InitPlayParams }.call(c, 0x003b9044)?;
c.g[A0][0] = c.lw((c.g[S7][0] as u32));
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
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32) >> 17);
c.g[A0][0] = c.g[A0][0] & 0x7f;
c.g[V0][0] = c.g[V0][0] & 0xffff;
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 7);
c.sb((c.g[SP][0] as u32), c.g[V1][0]);
c.g[A1][0] = c.lbu((c.g[A3][0] as u32));
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
Callee { addr: 0x003b6820, f: ee::s13::SND_Play }.call(c, 0x003b90cc)?;
c.sw((c.g[S2][0] as u32), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.sb((c.g[S2][0] as u32).wrapping_add(0x7), c.g[V1][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
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
c.pc = t; return Ok(());
}
    });
}

/// `SNDMIDI_NoteOff` (0x003b9140): SNDI_midiparsenoteoff: note-off for key `a1` on MIDI channel `a0`, velocity `a2`:
/// every matching record is keyed off with `SND_ChannelKeyOff`.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_note_off(c: &mut Cpu) {
    if c.pc != 0x003b9140 {
        return ee::s13::f_003b9140__generated(c);
    }
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
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x700));
c.g[A3][0] = 0u64;
c.g[S1][0] = 0xffffffffffffffff;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
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
c.g[A0][0] = c.lw((c.g[S0][0] as u32));
Callee { addr: 0x003c7138, f: ee::s13::SND_ChannelKeyOff }.call(c, 0x003b91b8)?;
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
if event_due(c) { deliver_event_here(c, 0x003b9180); } continue 'l_3b9180;
} else {
break 'b_3b91cc;
}
}
c.cycles += 5;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
}
    });
}

/// `SNDMIDI_ControlChange` (0x003b91e0): SNDI_midiparsecontrol: MIDI control change `a3` on channel `a0` (value `a2`, song
/// record `a1`): volume (7) and pan (10) reach the voices through `SND_SetVolume` and
/// `SND_SetPan`; the other controllers update the channel record.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_control_change(c: &mut Cpu) {
    if c.pc != 0x003b91e0 {
        return ee::s13::f_003b91e0__generated(c);
    }
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
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
let t = c.g[A3][0] != c.g[V0][0];
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
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
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x26));
break 'b_3b9264;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
c.sb((c.g[S0][0] as u32).wrapping_add(0x26), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x26));
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
c.g[A0][0] = c.lw((c.g[S3][0] as u32));
c.g[V1][0] = c.g[V1][0] | 0x3ff;
c.g[V0][0] = c.lbu((c.g[A1][0] as u32));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x700));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.g[V0][0] & 0x7f;
c.g[S2][0] = 0x17;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[S3][0] as u32), c.g[A0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
'l_3b92c0: loop {
'b_3b92fc: {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
if c.g[V0][0] != c.g[S4][0] {
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b92fc;
} else {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
if c.g[V0][0] != c.g[S5][0] {
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b92fc;
} else {
c.cycles += 9;
c.g[A1][0] = c.lw((c.g[S3][0] as u32));
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x16));
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 10);
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.g[A1][0] & 0x7f;
c.mult(0, 5, 5, 2);
c.g[RA][0] = sx(0x003b92f8);
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 7);
Callee { addr: 0x003b85f0, f: ee::s13::SND_SetVolume }.call(c, 0x003b92f8)?;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b92fc;
}
}
}
c.cycles += 2;
let t = (c.g[S2][0] as i64) >= 0;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003b92c0); } continue 'l_3b92c0;
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
c.g[A0][0] = c.lbu((c.g[V0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[RA][0] = sx(0x003b932c);
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
Callee { addr: 0x003bc570, f: ee::s13::SND_MidiToVolume }.call(c, 0x003b932c)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
c.g[A0][0] = 0xfffffffffffffc00;
c.g[V0][0] = c.g[V0][0] & 0x3ff;
c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[S3][0] as u32), c.g[V1][0]);
'l_3b9348: loop {
'b_3b9388: {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
if c.g[V0][0] != c.g[S4][0] {
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b9388;
} else {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x5));
if c.g[V0][0] != c.g[S5][0] {
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b9388;
} else {
c.cycles += 10;
c.g[A1][0] = c.lw((c.g[S3][0] as u32));
c.g[A2][0] = 0u64;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[A1][0] = c.g[A1][0] & 0x3ff;
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x003b9384);
c.g[A1][0] = c.g[A1][0] & 0xffff;
Callee { addr: 0x003b8c00, f: ee::s13::SND_SetPan }.call(c, 0x003b9384)?;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
break 'b_3b9388;
}
}
}
c.cycles += 2;
let t = (c.g[S2][0] as i64) >= 0;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003b9348); } continue 'l_3b9348;
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
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t; return Ok(());
    });
}

/// `SNDMIDI_PitchBend` (0x003b93c8): SNDI_midiparsepitchbend: pitch bend `a3` on MIDI channel `a0` (bend value `a2`, song
/// record `a1`): the channel's voices get `SND_SetPitchBend`.
#[allow(unreachable_code, unused_assignments, unused_labels)]
pub fn sndmidi_pitch_bend(c: &mut Cpu) {
    if c.pc != 0x003b93c8 {
        return ee::s13::f_003b93c8__generated(c);
    }
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
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[T0][0] = 0xffffffffffffffff;
c.g[S1][0] = c.g[A1][0];
c.g[S4][0] = c.g[A2][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = 0xffffffffff010000;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0xffff;
c.g[S2][0] = 0x17;
c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[T0][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f));
if c.g[T0][0] != 0 { c.g[V0][0] = c.g[V1][0]; }
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
c.g[V0][0] = c.g[V0][0] & 0x7f;
c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 17);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[S1][0] as u32), c.g[A0][0]);
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
c.g[A1][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = c.lw((c.g[S0][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
c.g[RA][0] = sx(0x003b9484);
c.g[A1][0] = c.g[A1][0] & 0x7f;
Callee { addr: 0x003bc6b8, f: ee::s13::SND_SetPitchBend }.call(c, 0x003b9484)?;
break 'b_3b9484;
}
}
}
c.cycles += 2;
let t = (c.g[S2][0] as i64) >= 0;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003b9458); } continue 'l_3b9458;
} else {
c.cycles += 8;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
}
}
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (in each test machine's RAM), its magic.
    const CTRL: u32 = DATA + 0x21_1440;
    const MAGIC: u32 = 0x1144_c0de;
    /// Scripted results: 8 entries of `(target, v0, calls left, unused)`.
    const TAB: u32 = CTRL + 0x10;
    /// The sound channel and note tables get small random words, so the
    /// byte compares of the MIDI parsers take both arms.
    const STATE: u32 = 0x0050_a000;
    const STATE_WORDS: u32 = 0x800;
    /// Pointer argument for the MIDI parsers (a scratch word in the test RAM).
    const PTR: u32 = DATA + 0x200;

    const HANDLE_TO_CHANNEL: u32 = 0x003b_a6e8;
    const NEXT_HANDLE_CHANNEL: u32 = 0x003b_aa00;

    /// A logged fake callee (may unwind) whose `v0` comes from the script.
    fn oc1144_fake(c: &mut Cpu, target: u32) {
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
    fn oc1144_script(c: &mut Cpu, target: u32, value: u32, times: u32) {
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

    fn oc1144_install_fakes() {
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
    fn oc1144_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
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

    fn oc1144_args(c: &mut Cpu, a0: u32, a1: u32, a2: u32, a3: u32) {
        c.g[A0][0] = a0 as u64;
        c.g[A1][0] = a1 as u64;
        c.g[A2][0] = a2 as u64;
        c.g[A3][0] = a3 as u64;
    }

    #[test]
    fn snd_apply_channel_params_matches() {
        check(
            ee::s13::f_003b8cc8__generated,
            snd_apply_channel_params,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0001, 0x003b_8cc8);
                let handle = r.next_u32();
                oc1144_args(c, handle, PTR, 0, 0);
            },
        );
    }

    #[test]
    fn snd_remove_dsp_matches() {
        check(
            ee::s13::f_003b8d30__generated,
            snd_remove_dsp,
            200,
            &|c, seed| {
                let mut r = oc1144_machine(c, seed, 0x1144_0002, 0x003b_8d30);
                let handle = r.next_u32();
                oc1144_args(c, handle, PTR, 0, 0);
            },
        );
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
}
