//! Staging module oc806 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc806`.

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
    (
        0x0028_7fc8,
        ssx_audio_system_load_volume_config,
        "SSXAudioSystem_LoadVolumeConfig",
    ),
    (
        0x0028_8f60,
        ssx_audio_system_set_debug_mode,
        "SSXAudioSystem_SetDebugMode",
    ),
    (
        0x0028_9470,
        ssx_audio_system_read_playlist_names,
        "SSXAudioSystem_ReadPlaylistNames",
    ),
    (
        0x0028_d488,
        ssx_audio_system_select_random_song_hand,
        "SSXAudioSystem_SelectRandomSong",
    ),
];

/// `SSXAudioSystem_LoadVolumeConfig(self)` (0x00287fc8): read the audio mix
/// config (an INI file of sections with MUSIC, DJ, PA, CHARACTER, BOARD,
/// COLLISION, AMBIENT, ARCADESFX, ARCADESPEECH levels and a TIME in ms) into
/// up to 25 mix records at `self + 0x64a0` (0x28 bytes each; `+0x6478` is the
/// record count). Every record starts at level 100, time 0.
///
/// The normal entry only, transcribed from the generated body; a resume
/// (`pc` inside the function) runs the generated body.
pub fn ssx_audio_system_load_volume_config(c: &mut Cpu) {
    if c.pc != 0x00287fc8 {
        return ee::s07::f_00287fc8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00287fc8;
        loop {
            match pc {
                0x00287fc8 => {
                    c.cycles += 38;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8d0));
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x470));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x647c));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6480));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x814), c.g[V0][0]);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x64a0));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6484));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x818), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x81c), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6488));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x648c));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x649c));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6498));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x840));
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6494));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x820), c.g[A0][0]);
                    c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6490));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.g[A0][0] = c.g[SP][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x830), c.g[RA][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x824), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00288048);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6478), 0u64);
                    call_ptr(c, 0x002a77f8, 0x00288048)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x64;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[A1][0] = 0x18;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x00288060;
                    continue;
                }
                0x00288060 => {
                    c.cycles += 13;
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x24), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x20), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x1c), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x18), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x10), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0xc), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x8), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x4), c.g[A0][0]);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    if t {
                        if back_edge_unwinds(c, 0x00288060) {
                            return Ok(());
                        }
                        pc = 0x00288060;
                        continue;
                    }
                    pc = 0x00288094;
                    continue;
                }
                0x00288094 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    if t {
                        pc = 0x00288318;
                        continue;
                    }
                    pc = 0x0028809c;
                    continue;
                }
                0x0028809c => {
                    c.cycles += 1;
                    pc = 0x002880a0;
                    continue;
                }
                0x002880a0 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002880a8);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7da0, 0x002880a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35d8));
                    c.g[RA][0] = sx(0x002880c0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002880c0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002880e0;
                        continue;
                    }
                    pc = 0x002880c8;
                    continue;
                }
                0x002880c8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002880d8);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002880d8)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x814));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x002880e0;
                    continue;
                }
                0x002880e0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35e0));
                    c.g[RA][0] = sx(0x002880f0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002880f0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00288110;
                        continue;
                    }
                    pc = 0x002880f8;
                    continue;
                }
                0x002880f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288108);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288108)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x818));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288110;
                    continue;
                }
                0x00288110 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35e8));
                    c.g[RA][0] = sx(0x00288120);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288120)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288140;
                        continue;
                    }
                    pc = 0x00288128;
                    continue;
                }
                0x00288128 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288138);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288138)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x81c));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288140;
                    continue;
                }
                0x00288140 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2930));
                    c.g[RA][0] = sx(0x00288150);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288150)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00288170;
                        continue;
                    }
                    pc = 0x00288158;
                    continue;
                }
                0x00288158 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288168);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288168)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x820));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288170;
                    continue;
                }
                0x00288170 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                    c.g[RA][0] = sx(0x00288180);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288180)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002881b4;
                        continue;
                    }
                    pc = 0x00288188;
                    continue;
                }
                0x00288188 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288198);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288198)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x824));
                    pc = 0x0028819c;
                    continue;
                }
                0x0028819c => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.lo[0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.madd(0, 3, 3, 19);
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002881b4;
                    continue;
                }
                0x002881b4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2940));
                    c.g[RA][0] = sx(0x002881c4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002881c4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002881f4;
                        continue;
                    }
                    pc = 0x002881cc;
                    continue;
                }
                0x002881cc => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002881dc);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002881dc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002881f4;
                    continue;
                }
                0x002881f4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35f0));
                    c.g[RA][0] = sx(0x00288204);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288204)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288234;
                        continue;
                    }
                    pc = 0x0028820c;
                    continue;
                }
                0x0028820c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028821c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028821c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288234;
                    continue;
                }
                0x00288234 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2950));
                    c.g[RA][0] = sx(0x00288244);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288244)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288274;
                        continue;
                    }
                    pc = 0x0028824c;
                    continue;
                }
                0x0028824c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028825c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028825c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288274;
                    continue;
                }
                0x00288274 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2960));
                    c.g[RA][0] = sx(0x00288284);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288284)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002882b4;
                        continue;
                    }
                    pc = 0x0028828c;
                    continue;
                }
                0x0028828c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028829c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028829c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002882b4;
                    continue;
                }
                0x002882b4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35f8));
                    c.g[RA][0] = sx(0x002882c4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002882c4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002882cc;
                    continue;
                }
                0x002882cc => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002882d8);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002882d8)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x810));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x461c));
                    c.mult(0, 2, 3, 19);
                    c.mul_s(0, 0, 1);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.swc1(0, c.g[V1][0] as u32);
                    pc = 0x002882f8;
                    continue;
                }
                0x002882f8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00288300);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7f90, 0x00288300)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002880a0) {
                            return Ok(());
                        }
                        pc = 0x002880a0;
                        continue;
                    }
                    pc = 0x00288308;
                    continue;
                }
                0x00288308 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6478), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    pc = 0x00288318;
                    continue;
                }
                0x00288318 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 25i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x00288338;
                        continue;
                    }
                    pc = 0x00288324;
                    continue;
                }
                0x00288324 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0028832c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7b08, 0x0028832c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = 0x28;
                    if t {
                        if back_edge_unwinds(c, 0x002882f8) {
                            return Ok(());
                        }
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288334;
                    continue;
                }
                0x00288334 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00288338;
                    continue;
                }
                0x00288338 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00288340);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002a7890, 0x00288340)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x840));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x830));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00287fc8, 0x00288370);
                    return Ok(());
                }
            }
        }
    });
}

/// `SSXAudioSystem_SetDebugMode(self, mode)` (0x00288f60): store the debug
/// preset (`+0x6470`) and set the ten channel volumes (0.909 or 0 per preset)
/// plus the arcade SFX sub-channels flag; presets 0..=5 through a jump table at
/// `0x482970`, other modes only store the value.
///
/// The normal entry only, transcribed from the generated body.
pub fn ssx_audio_system_set_debug_mode(c: &mut Cpu) {
    if c.pc != 0x00288f60 {
        return ee::s07::f_00288f60__generated(c);
    }
    guest(c, |c| {
        'b_289380: {
            'b_28937c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6470), c.g[A1][0]);
                if t {
                    break 'b_28937c;
                } else {
                    'b_2892cc: {
                        'b_289214: {
                            'b_2891f8: {
                                'b_2891d4: {
                                    'b_289150: {
                                        'b_2890cc: {
                                            'b_289024: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x480000;
                                                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x2970));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x00288fa0 => {
                                                        c.cycles += 33;
                                                        c.lwc1(
                                                            20,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x45f4),
                                                        );
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x1;
                                                        c.g[RA][0] = sx(0x00288fb4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fb4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x5;
                                                        c.g[RA][0] = sx(0x00288fc4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fc4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x2;
                                                        c.g[RA][0] = sx(0x00288fd4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fd4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x3;
                                                        c.g[RA][0] = sx(0x00288fe4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fe4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x4;
                                                        c.g[RA][0] = sx(0x00288ff4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288ff4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x6;
                                                        c.g[RA][0] = sx(0x00289004);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00289004)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x7;
                                                        c.g[RA][0] = sx(0x00289014);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00289014)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x8;
                                                        let t = true;
                                                        c.mov_s(12, 20);
                                                        if t {
                                                            break 'b_2891d4;
                                                        } else {
                                                            break 'b_289024;
                                                        }
                                                    }
                                                    0x00289024 => {
                                                        break 'b_289024;
                                                    }
                                                    0x002890cc => {
                                                        break 'b_2890cc;
                                                    }
                                                    0x00289150 => {
                                                        break 'b_289150;
                                                    }
                                                    0x00289214 => {
                                                        break 'b_289214;
                                                    }
                                                    0x002892cc => {
                                                        break 'b_2892cc;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x00288f60, 0x00289394);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 42;
                                            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x45f0));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x1;
                                            c.mtc1(0, 21);
                                            c.g[RA][0] = sx(0x0028903c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028903c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x5;
                                            c.g[RA][0] = sx(0x0028904c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028904c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x2;
                                            c.g[RA][0] = sx(0x0028905c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028905c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x3;
                                            c.g[RA][0] = sx(0x0028906c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028906c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x4;
                                            c.g[RA][0] = sx(0x0028907c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028907c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x6;
                                            c.g[RA][0] = sx(0x0028908c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028908c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x7;
                                            c.g[RA][0] = sx(0x0028909c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028909c)?;
                                            c.mov_s(12, 20);
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x002890ac);
                                            c.g[A1][0] = 0x8;
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x002890ac)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x9;
                                            c.g[RA][0] = sx(0x002890bc);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x002890bc)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0xa;
                                            let t = true;
                                            c.mov_s(12, 21);
                                            if t {
                                                break 'b_2891f8;
                                            } else {
                                                break 'b_2890cc;
                                            }
                                        }
                                        c.cycles += 33;
                                        c.mtc1(0, 12);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x45ec));
                                        c.g[RA][0] = sx(0x002890e0);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x002890e0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x5;
                                        c.g[RA][0] = sx(0x002890f0);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x002890f0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[RA][0] = sx(0x00289100);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289100)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x3;
                                        c.g[RA][0] = sx(0x00289110);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289110)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x4;
                                        c.g[RA][0] = sx(0x00289120);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289120)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x6;
                                        c.g[RA][0] = sx(0x00289130);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289130)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x7;
                                        c.g[RA][0] = sx(0x00289140);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289140)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x8;
                                        let t = true;
                                        c.mov_s(12, 20);
                                        if t {
                                            break 'b_2891d4;
                                        } else {
                                            break 'b_289150;
                                        }
                                    }
                                    c.cycles += 33;
                                    c.mtc1(0, 20);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x45e8));
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x00289168);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289168)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x5;
                                    c.g[RA][0] = sx(0x00289178);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289178)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x2;
                                    c.g[RA][0] = sx(0x00289188);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289188)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x3;
                                    c.g[RA][0] = sx(0x00289198);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289198)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x4;
                                    c.g[RA][0] = sx(0x002891a8);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891a8)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x6;
                                    c.g[RA][0] = sx(0x002891b8);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891b8)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x7;
                                    c.g[RA][0] = sx(0x002891c8);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891c8)?;
                                    c.mov_s(12, 21);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x8;
                                    break 'b_2891d4;
                                }
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x002891dc);
                                Callee {
                                    addr: 0x00287700,
                                    f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                }
                                .call(c, 0x002891dc)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0x9;
                                c.g[RA][0] = sx(0x002891ec);
                                c.mov_s(12, 20);
                                Callee {
                                    addr: 0x00287700,
                                    f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                }
                                .call(c, 0x002891ec)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0xa;
                                c.mov_s(12, 20);
                                break 'b_2891f8;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00289200);
                            Callee {
                                addr: 0x00287700,
                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                            }
                            .call(c, 0x00289200)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0028920c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x002875d0,
                                f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                            }
                            .call(c, 0x0028920c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_289380;
                            } else {
                                break 'b_289214;
                            }
                        }
                        c.cycles += 46;
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x45e4));
                        c.g[A0][0] = c.g[S0][0];
                        c.mtc1(0, 20);
                        c.g[RA][0] = sx(0x00289228);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289228)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x5;
                        c.g[RA][0] = sx(0x00289238);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289238)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x2;
                        c.g[RA][0] = sx(0x00289248);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289248)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x3;
                        c.g[RA][0] = sx(0x00289258);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289258)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x4;
                        c.g[RA][0] = sx(0x00289268);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289268)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x6;
                        c.g[RA][0] = sx(0x00289278);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289278)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x7;
                        c.g[RA][0] = sx(0x00289288);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289288)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x8;
                        c.g[RA][0] = sx(0x00289298);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289298)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x9;
                        c.g[RA][0] = sx(0x002892a8);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x002892a8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xa;
                        c.g[RA][0] = sx(0x002892b8);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x002892b8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002892c4);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x002875d0,
                            f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                        }
                        .call(c, 0x002892c4)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_289380;
                        } else {
                            break 'b_2892cc;
                        }
                    }
                    c.cycles += 44;
                    c.mtc1(0, 20);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x002892e0);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x002892e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x5;
                    c.g[RA][0] = sx(0x002892f0);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x002892f0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x00289300);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289300)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x3;
                    c.g[RA][0] = sx(0x00289310);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289310)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x00289320);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289320)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x6;
                    c.g[RA][0] = sx(0x00289330);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289330)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x00289340);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289340)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x8;
                    c.g[RA][0] = sx(0x00289350);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289350)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x9;
                    c.g[RA][0] = sx(0x00289360);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289360)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00289370);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289370)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028937c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x002875d0,
                        f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                    }
                    .call(c, 0x0028937c)?;
                    break 'b_28937c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_289380;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_ReadPlaylistNames(self)` (0x00289470): when the playlist
/// file exists, read up to 20 section names into `self + 0x60a4` (0x14 bytes
/// each; `+0x6234` is the count, 0 when the file is missing).
///
/// The normal entry only, transcribed from the generated body.
pub fn ssx_audio_system_read_playlist_names(c: &mut Cpu) {
    if c.pc != 0x00289470 {
        return ee::s07::f_00289470__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00289470;
        loop {
            match pc {
                0x00289470 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x840));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x810), c.g[RA][0]);
                    c.g[RA][0] = sx(0x00289490);
                    call_ptr(c, 0x00317f38, 0x00289490)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
                    if t {
                        pc = 0x00289504;
                        continue;
                    }
                    pc = 0x00289498;
                    continue;
                }
                0x00289498 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x002894a8);
                    c.g[S1][0] = 0x14;
                    call_ptr(c, 0x002a77f8, 0x002894a8)?;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), 0u64);
                    if t {
                        pc = 0x002894d0;
                        continue;
                    }
                    pc = 0x002894b0;
                    continue;
                }
                0x002894b0 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[V0][0];
                    c.mult(0, 4, 4, 17);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60a4));
                    c.g[RA][0] = sx(0x002894c4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, 0x004166f4, 0x002894c4)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), c.g[V1][0]);
                    pc = 0x002894d0;
                    continue;
                }
                0x002894d0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 20i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x002894f4;
                        continue;
                    }
                    pc = 0x002894e0;
                    continue;
                }
                0x002894e0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002894e8);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7b08, 0x002894e8)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                        if back_edge_unwinds(c, 0x002894b0) {
                            return Ok(());
                        }
                        pc = 0x002894b0;
                        continue;
                    }
                    pc = 0x002894f0;
                    continue;
                }
                0x002894f0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x002894f4;
                    continue;
                }
                0x002894f4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002894fc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002a7890, 0x002894fc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
                    if t {
                        pc = 0x0028950c;
                        continue;
                    }
                    pc = 0x00289504;
                    continue;
                }
                0x00289504 => {
                    c.cycles += 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), 0u64);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
                    pc = 0x0028950c;
                    continue;
                }
                0x0028950c => {
                    c.cycles += 4;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x810));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x840));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00289470, 0x0028951c);
                    return Ok(());
                }
            }
        }
    });
}

/// `SSXAudioSystem_SelectRandomSong(self)` (0x0028d488): pick a random song
/// index in `[0, tracks)`, store it and make the music player switch to it.
///
/// Same body as the unwired ports in `bankmanager.rs`; this one is installed.
pub fn ssx_audio_system_select_random_song_hand(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x7fff;
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let player = c.lw((this as u32).wrapping_add(0x118));
        c.g[S1][0] = c.lw((this as u32).wrapping_add(0x504));
        c.g[S3][0] = c.lw((this as u32).wrapping_add(0x62a4));
        c.g[V1][0] = player;
        c.g[V0][0] = c.lw((player as u32).wrapping_add(0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;

        // Random fraction of the (scaled) track count.
        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = c.lw((this as u32).wrapping_add(0x118));
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        c.sw((this as u32).wrapping_add(0x508), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4ec)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff;

        // Skip up to ten songs to find a playable one.
        c.cycles += 5;
        c.divu(0, V0, S0);
        c.g[V1][0] = c.lo[0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = this;
            loop {
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                Callee {
                    addr: 0x0028_d8a0,
                    f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
                }
                .call(c, 0x0028_d528)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = sx((this as u32).wrapping_add(0x118));
                c.g[A1][0] = 1;
                call_ptr(c, 0x002b_42b8, 0x0028_d538)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = this;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_d520);
                }
            }
        }

        // Restart the music player on the new selection.
        c.cycles += 3;
        c.g[V0][0] = c.lw((this as u32).wrapping_add(0x508));
        let same = c.g[S3][0] == c.g[V0][0];
        c.g[S0][0] = sx((this as u32).wrapping_add(0x118));
        if !same {
            c.cycles += 1;
        } else {
            c.cycles += 9;
            c.g[A0][0] = this;
            Callee {
                addr: 0x0028_d8a0,
                f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
            }
            .call(c, 0x0028_d554)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002b_42b8, 0x0028_d564)?;
            c.g[V0][0] = c.lw((this as u32).wrapping_add(0x508));
            if c.g[S3][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{elf_image, fakes, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the test machine's RAM).
    const OC806_CTRL: u32 = DATA + 0x20_0000;
    const OC806_MAGIC: u32 = 0x4f43_3830;
    /// Per-fake result policy words: `OC806_CTRL + 0x10 + 4 * index`.
    const OC806_POLICY: u32 = OC806_CTRL + 0x10;
    /// The shared salt of the policy hash.
    const OC806_SALT: u32 = OC806_CTRL + 4;

    // Fake indexes (policy slots).
    const FX_NEXT_SECTION: u32 = 2;
    const FX_AT_SECTION_END: u32 = 4;
    const FX_STRCASECMP: u32 = 6;
    const FX_PATH_EXISTS: u32 = 9;
    const FX_AUDIO_RAND: u32 = 11;

    /// The fake callee: logs the call like `fakes::fake_call` (arguments,
    /// registers, cycles), then, when the machine's policy word for this fake
    /// is `p` in 1..=9, returns 1 with probability `(p - 1) / 8` and 0
    /// otherwise (a pointer-like value when the policy is 0).
    fn oc806_fake_call(c: &mut Cpu, addr: u32, index: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        let policy = c.bus.read32(OC806_POLICY + 4 * index);
        let salt = c.bus.read32(OC806_SALT);
        fakes::fake_call(c, addr);
        if c.pc == ra && policy != 0 {
            let h = fakes::mix(u64::from(salt) ^ u64::from(n) << 20 ^ u64::from(addr));
            c.g[V0][0] = ((h & 7) < u64::from(policy - 1)) as u64;
        }
    }

    macro_rules! oc806_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $index:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC806_CTRL) != OC806_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc806_fake_call(c, $addr, $index);
                }
            )*
            fn oc806_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc806_fakes!(
        (
            oc806_fake_2a77f8,
            OC806_PREV_2A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8,
            0
        ),
        (
            oc806_fake_2a7890,
            OC806_PREV_2A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890,
            1
        ),
        (
            oc806_fake_2a7b08,
            OC806_PREV_2A7B08,
            ee::s08::OVERRIDE_002A7B08,
            ee::s08::f_002a7b08__generated,
            0x002a_7b08,
            2
        ),
        (
            oc806_fake_2a7da0,
            OC806_PREV_2A7DA0,
            ee::s08::OVERRIDE_002A7DA0,
            ee::s08::f_002a7da0__generated,
            0x002a_7da0,
            3
        ),
        (
            oc806_fake_2a7f90,
            OC806_PREV_2A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90,
            4
        ),
        (
            oc806_fake_2a80d8,
            OC806_PREV_2A80D8,
            ee::s08::OVERRIDE_002A80D8,
            ee::s08::f_002a80d8__generated,
            0x002a_80d8,
            5
        ),
        (
            oc806_fake_41aa88,
            OC806_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88,
            6
        ),
        (
            oc806_fake_2875d0,
            OC806_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0,
            7
        ),
        (
            oc806_fake_287700,
            OC806_PREV_287700,
            ee::s07::OVERRIDE_00287700,
            ee::s07::f_00287700__generated,
            0x0028_7700,
            8
        ),
        (
            oc806_fake_317f38,
            OC806_PREV_317F38,
            ee::s10::OVERRIDE_00317F38,
            ee::s10::f_00317f38__generated,
            0x0031_7f38,
            9
        ),
        (
            oc806_fake_4166f4,
            OC806_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4,
            10
        ),
        (
            oc806_fake_2adf60,
            OC806_PREV_2ADF60,
            ee::s08::OVERRIDE_002ADF60,
            ee::s08::f_002adf60__generated,
            0x002a_df60,
            11
        ),
        (
            oc806_fake_28d8a0,
            OC806_PREV_28D8A0,
            ee::s07::OVERRIDE_0028D8A0,
            ee::s07::f_0028d8a0__generated,
            0x0028_d8a0,
            12
        ),
        (
            oc806_fake_2b42b8,
            OC806_PREV_2B42B8,
            ee::s08::OVERRIDE_002B42B8,
            ee::s08::f_002b42b8__generated,
            0x002b_42b8,
            13
        ),
    );

    /// A scrambled machine with the fakes armed: the object at `DATA + 0x1000`
    /// in `a0`, the fake policies and `pc` at the function's entry.
    fn oc806_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        entry: u32,
        policies: &[(u32, u32)],
    ) {
        oc806_install_fakes();
        fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC806_CTRL, OC806_MAGIC);
        c.bus.write32(OC806_SALT, seed);
        for &(index, policy) in policies {
            c.bus.write32(OC806_POLICY + 4 * index, policy);
        }
        c.g[A0][0] = u64::from(DATA + 0x1000);
        c.pc = entry;
    }

    /// Mix-file parse: keys that match or not (policy of the `strcasecmp`
    /// fake), sections ending early or late, up to 25 mixes, unwinds.
    #[test]
    fn ssx_audio_system_load_volume_config_matches() {
        fakes::covered(
            ee::s07::f_00287fc8__generated,
            ssx_audio_system_load_volume_config,
            300,
            &|c, seed| {
                oc806_machine(
                    c,
                    seed,
                    0x806_0001,
                    150,
                    0x0028_7fc8,
                    &[
                        (FX_STRCASECMP, 1 + (seed / 3) % 9),
                        (FX_AT_SECTION_END, 4 + seed % 6),
                        (FX_NEXT_SECTION, 2 + (seed / 7) % 8),
                    ],
                );
            },
        );
    }

    /// Debug presets 0..=5 (jump table), larger values store only.
    #[test]
    fn ssx_audio_system_set_debug_mode_matches() {
        fakes::covered(
            ee::s07::f_00288f60__generated,
            ssx_audio_system_set_debug_mode,
            240,
            &|c, seed| {
                oc806_machine(c, seed, 0x806_0002, 12, 0x0028_8f60, &[]);
                elf_image(c);
                c.g[A1][0] = match seed % 8 {
                    7 => 100,
                    m => u64::from(m),
                };
            },
        );
    }

    /// Playlist file missing or present, 0..=20 sections.
    #[test]
    fn ssx_audio_system_read_playlist_names_matches() {
        fakes::covered(
            ee::s07::f_00289470__generated,
            ssx_audio_system_read_playlist_names,
            240,
            &|c, seed| {
                oc806_machine(
                    c,
                    seed,
                    0x806_0003,
                    10,
                    0x0028_9470,
                    &[
                        (FX_PATH_EXISTS, if seed % 4 == 0 { 1 } else { 9 }),
                        (FX_NEXT_SECTION, 2 + (seed / 4) % 8),
                    ],
                );
            },
        );
    }

    /// Random song: the first pick equals the previous one (second pick,
    /// then stop) or differs, 1..=7 songs, 1..=10 skips.
    #[test]
    fn ssx_audio_system_select_random_song_matches() {
        fakes::covered(
            ee::s07::f_0028d488__generated,
            ssx_audio_system_select_random_song_hand,
            240,
            &|c, seed| {
                let rand_policy = if seed % 3 == 1 { 0 } else { 9 };
                oc806_machine(
                    c,
                    seed,
                    0x806_0004,
                    8,
                    0x0028_d488,
                    &[(FX_AUDIO_RAND, rand_policy)],
                );
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x1d8, 0x1234_5678);
                c.bus.write32(this + 0x504, 1 + seed % 7);
                c.bus
                    .write32(this + 0x62a4, if seed % 3 == 2 { 1 } else { 0 });
                c.bus.write32(this + 0x508, 0xdead_beef);
            },
        );
    }
}
