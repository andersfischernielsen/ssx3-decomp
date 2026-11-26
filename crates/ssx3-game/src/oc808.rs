//! Staging module oc808 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc808`.

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
    (0x002a_7da0, ini_parser_read_token, "IniParser_ReadToken"),
    (
        0x002a_8d90,
        ps2_sound_manager_set_output_mode,
        "PS2SoundManager_SetOutputMode",
    ),
    (0x002a_fa68, speech_instance_ctor, "SpeechInstance_ctor"),
    (0x002b_28c0, music_player_ctor, "MusicPlayer_ctor"),
];

/// `char* IniParser_ReadToken(IniParser* self)` (0x002a7da0, `IniParser_ReadToken`): skips blanks/comments, then copies the next token (up to NUL, space, tab, CR or LF) from the cursor at `self+8` into the buffer at `self+0xc`, NUL-terminates it and returns the buffer.
///
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ini_parser_read_token(c: &mut Cpu) {
    if c.pc != 0x002a7da0 {
        return ee::s08::f_002a7da0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a7da0;
        loop {
            match pc {
                0x002a7da0 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = 0u64;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S3][0] = 0xa;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S4][0] = 0x9;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S5][0] = 0x20;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x002a7de0);
                    c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    Callee {
                        addr: 0x002a78f8,
                        f: ee::s08::IniParser_SkipWhitespace,
                    }
                    .call(c, 0x002a7de0)?;
                    let t = true;
                    c.g[S2][0] = 0xd;
                    if t {
                        pc = 0x002a7dfc;
                        continue;
                    }
                    pc = 0x002a7de8;
                    continue;
                }
                0x002a7de8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a7df0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002a78e0,
                        f: ee::s08::IniParser_GetChar,
                    }
                    .call(c, 0x002a7df0)?;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x002a7dfc;
                    continue;
                }
                0x002a7dfc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lb(c.g[V0][0] as u32);

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        pc = 0x002a7e2c;
                        continue;
                    }
                    pc = 0x002a7e0c;
                    continue;
                }
                0x002a7e0c => {
                    c.cycles += 2;

                    if c.g[V0][0] == c.g[S5][0] {
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        pc = 0x002a7e2c;
                        continue;
                    }
                    pc = 0x002a7e14;
                    continue;
                }
                0x002a7e14 => {
                    c.cycles += 2;

                    if c.g[V0][0] == c.g[S4][0] {
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        pc = 0x002a7e2c;
                        continue;
                    }
                    pc = 0x002a7e1c;
                    continue;
                }
                0x002a7e1c => {
                    c.cycles += 2;

                    if c.g[V0][0] == c.g[S3][0] {
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        pc = 0x002a7e2c;
                        continue;
                    }
                    pc = 0x002a7e24;
                    continue;
                }
                0x002a7e24 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        if back_edge_unwinds(c, 0x002a7de8) {
                            return Ok(());
                        }
                        pc = 0x002a7de8;
                        continue;
                    }
                    pc = 0x002a7e2c;
                    continue;
                }
                0x002a7e2c => {
                    c.cycles += 12;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    c.sb(c.g[V1][0] as u32, 0u64);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a7da0, 0x002a7e5c);
                    return Ok(());
                }
            }
        }
    });
}

/// `int PS2SoundManager_SetOutputMode(PS2SoundManager* self, int mode)` (0x002a8d90): reads the sound config (`SND_GetConfig`), fills in the mono/stereo/Dolby/DTS option bytes for `mode` (0..3), installs DTS and writes it back (`SND_SetConfig`); stores the mode at `self+8` and returns 1 or 2.
///
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_sound_manager_set_output_mode(c: &mut Cpu) {
    if c.pc != 0x002a8d90 {
        return ee::s08::f_002a8d90__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a8d90;
        loop {
            match pc {
                0x002a8d90 => {
                    c.cycles += 21;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    c.g[RA][0] = sx(0x002a8db4);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x003b4dd8, 0x002a8db4)?;
                    c.g[S2][0] = 0x2;
                    c.g[A1][0] = 0x32;
                    c.g[V0][0] = 0x8ca0;
                    c.g[V1][0] = 0x5dc0;
                    c.g[A0][0] = 0x14;
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2a), c.g[V0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x41), c.g[A1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x3f), 0u64);
                    let t = c.g[S0][0] == c.g[S2][0];
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A1][0]);
                    if t {
                        pc = 0x002a8e5c;
                        continue;
                    }
                    pc = 0x002a8de4;
                    continue;
                }
                0x002a8de4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x002a8e00;
                        continue;
                    }
                    pc = 0x002a8df0;
                    continue;
                }
                0x002a8df0 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) < 0;

                    if t {
                        pc = 0x002a8f28;
                        continue;
                    }
                    pc = 0x002a8df8;
                    continue;
                }
                0x002a8df8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x002a8ed4;
                        continue;
                    }
                    pc = 0x002a8e00;
                    continue;
                }
                0x002a8e00 => {
                    c.cycles += 2;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002a8f28;
                        continue;
                    }
                    pc = 0x002a8e08;
                    continue;
                }
                0x002a8e08 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V1][0] = 0x8;
                    if t {
                        pc = 0x002a8f34;
                        continue;
                    }
                    pc = 0x002a8e14;
                    continue;
                }
                0x002a8e14 => {
                    c.cycles += 18;
                    c.g[V0][0] = 0x10;
                    c.g[A0][0] = 0x18;
                    c.g[A1][0] = 0x4;
                    c.g[A2][0] = 0x28;
                    c.g[A3][0] = 0x120;
                    c.g[T0][0] = 0x24;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2e), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2f), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x47), c.g[A1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4e), c.g[A2][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x52), c.g[T0][0]);
                    c.g[RA][0] = sx(0x002a8e54);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x49), c.g[S2][0]);
                    call_ptr(c, 0x003bc930, 0x002a8e54)?;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                    if t {
                        pc = 0x002a8f28;
                        continue;
                    }
                    pc = 0x002a8e5c;
                    continue;
                }
                0x002a8e5c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002a8e70;
                        continue;
                    }
                    pc = 0x002a8e68;
                    continue;
                }
                0x002a8e68 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002a8f34;
                        continue;
                    }
                    pc = 0x002a8e70;
                    continue;
                }
                0x002a8e70 => {
                    c.cycles += 25;
                    c.g[V1][0] = 0x10;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = 0x30;
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = 0x6;
                    c.g[T0][0] = 0x48;
                    c.g[T1][0] = 0x140;
                    c.g[T2][0] = 0x44;
                    c.g[T3][0] = 0x28;
                    c.g[T4][0] = 0x120;
                    c.g[T5][0] = 0x24;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2e), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2f), c.g[A0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x49), c.g[A2][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x47), c.g[S0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4e), c.g[T0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[T1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x52), c.g[T2][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x54), c.g[T3][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x56), c.g[T4][0]);
                    let t = true;
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[T5][0]);
                    if t {
                        pc = 0x002a8f28;
                        continue;
                    }
                    pc = 0x002a8ed4;
                    continue;
                }
                0x002a8ed4 => {
                    c.cycles += 3;
                    c.g[T3][0] = 0x2;
                    let t = c.g[V0][0] == c.g[T3][0];
                    c.g[V1][0] = 0x10;
                    if t {
                        if back_edge_unwinds(c, 0x002a8e68) {
                            return Ok(());
                        }
                        pc = 0x002a8e68;
                        continue;
                    }
                    pc = 0x002a8ee0;
                    continue;
                }
                0x002a8ee0 => {
                    c.cycles += 18;
                    c.g[V0][0] = 0x1;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2e), c.g[V1][0]);
                    c.g[A0][0] = 0x8;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = 0x30;
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = 0x3;
                    c.g[T0][0] = 0x28;
                    c.g[T1][0] = 0x120;
                    c.g[T2][0] = 0x24;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2f), c.g[A0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x49), c.g[A2][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x47), c.g[T3][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4e), c.g[T0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[T1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x52), c.g[T2][0]);
                    pc = 0x002a8f28;
                    continue;
                }
                0x002a8f28 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002a8f30);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x003b4fc0, 0x002a8f30)?;
                    c.g[V0][0] = 0x2;
                    pc = 0x002a8f34;
                    continue;
                }
                0x002a8f34 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a8d90, 0x002a8f4c);
                    return Ok(());
                }
            }
        }
    });
}

/// `SpeechInstance* SpeechInstance_ctor(...)` (0x002afa68): clears the instance fields and the 4-entry slot table, creates the speech stream (`StreamArray_CreateStreamDefaultParams`) and loads the speech info (`SpeechInstance_LoadSpeechInf`). Returns `this`.
///
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn speech_instance_ctor(c: &mut Cpu) {
    if c.pc != 0x002afa68 {
        return ee::s08::f_002afa68__generated(c);
    }
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = 0x3;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[T1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x900), c.g[V0][0]);
        c.g[A2][0] = c.g[T0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xec), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xf0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8cc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8d0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8f4), 0u64);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        'l_2afad0: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x4), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, 0u64);
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002afad0);
                }
                continue 'l_2afad0;
            } else {
                c.cycles += 19;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
                c.g[T0][0] = 0x20000;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x002afb04);
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                Callee {
                    addr: 0x002aa910,
                    f: ee::s08::StreamArray_CreateStreamDefaultParams,
                }
                .call(c, 0x002afb04)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002afb18);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x002b0088,
                    f: ee::s08::SpeechInstance_LoadSpeechInf,
                }
                .call(c, 0x002afb18)?;
                c.g[V0][0] = c.g[S0][0];
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `MusicPlayer* MusicPlayer_ctor(this, inchrg, heap, count, heap2)` (0x002b28c0, frame 0x940): builds the sound-manager reference, bank manager and stream array when not in charge, clears the song tables, then parses the music INI sections (song name, path, stream/bank settings) into up to 64 `cSongConfiguration`s. Returns `this`.
///
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn music_player_ctor(c: &mut Cpu) {
    if c.pc != 0x002b28c0 {
        return ee::s08::f_002b28c0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002b28c0;
        loop {
            match pc {
                0x002b28c0 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x940));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x920));
                    c.g[S0][0] = c.g[T0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x910));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x900));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x8f0));
                    c.g[S3][0] = c.g[A3][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x8e0));
                    c.g[S4][0] = c.g[A1][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    let t = c.g[S4][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x8a0), c.g[RA][0]);
                    if t {
                        pc = 0x002b2954;
                        continue;
                    }
                    pc = 0x002b2904;
                    continue;
                }
                0x002b2904 => {
                    c.cycles += 20;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5448));
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5f20));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5450));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5f20), c.g[V1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.sw(c.g[S2][0] as u32, c.g[A2][0]);
                    c.g[RA][0] = sx(0x002b2928);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5628), c.g[V0][0]);
                    Callee {
                        addr: 0x002ade88,
                        f: ee::s08::SoundManagerRef_ctor,
                    }
                    .call(c, 0x002b2928)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5f20));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b293c);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x0028b9b8, 0x002b293c)?;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[T0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002b2954);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x002aa490,
                        f: ee::s08::StreamArray_ctor,
                    }
                    .call(c, 0x002b2954)?;
                    pc = 0x002b2954;
                    continue;
                }
                0x002b2954 => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3928));
                    c.g[A0][0] = 0x480000;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
                    c.g[A3][0] = 0x480000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                    c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3948));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[S4][0] != 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
                    if t {
                        pc = 0x002b2a48;
                        continue;
                    }
                    pc = 0x002b2998;
                    continue;
                }
                0x002b2998 => {
                    c.cycles += 44;
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3928));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3948));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                    c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                    c.g[T3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.lw(c.g[S2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                    c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5448));
                    c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T2][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
                    c.g[A0][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5450));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                    pc = 0x002b2a48;
                    continue;
                }
                0x002b2a48 => {
                    c.cycles += 28;
                    c.g[V0][0] = 0x480000;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3e4), 0u64);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3e8), 0u64);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5440), c.g[V0][0]);
                    c.g[A1][0] = 0x480000;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3ec), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3448));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3f0), 0u64);
                    c.g[A2][0] = 0u64;
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x3f8), 0u64);
                    c.g[A3][0] = 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x408), 0u64);
                    c.g[A0][0] = 0x6400;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x418), 0u64);
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x420));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5438), 0u64);
                    c.g[FP][0] = 0x4a0000;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x543c), 0u64);
                    c.g[RA][0] = sx(0x002b2a9c);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x6fc), c.g[S2][0]);
                    call_ptr(c, 0x00317e30, 0x002b2a9c)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5434), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x920));
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A3][0] = c.g[S7][0];
                    c.g[A2][0] = 0x14;
                    c.g[V1][0] = 0u64;
                    c.mult(0, 4, 3, 6);
                    pc = 0x002b2ab8;
                    continue;
                }
                0x002b2ab8 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0x4;
                    pc = 0x002b2ac8;
                    continue;
                }
                0x002b2ac8 => {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x002b2ac8) {
                            return Ok(());
                        }
                        pc = 0x002b2ac8;
                        continue;
                    }
                    pc = 0x002b2ae4;
                    continue;
                }
                0x002b2ae4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[A0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 64i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 4, 3, 6);
                    if t {
                        if back_edge_unwinds(c, 0x002b2ab8) {
                            return Ok(());
                        }
                        pc = 0x002b2ab8;
                        continue;
                    }
                    pc = 0x002b2af4;
                    continue;
                }
                0x002b2af4 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0x4;
                    pc = 0x002b2b00;
                    continue;
                }
                0x002b2b00 => {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x002b2b00) {
                            return Ok(());
                        }
                        pc = 0x002b2b00;
                        continue;
                    }
                    pc = 0x002b2b1c;
                    continue;
                }
                0x002b2b1c => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1900));
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x35a8));
                    pc = 0x002b2b30;
                    continue;
                }
                0x002b2b30 => {
                    c.cycles += 9;
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x934), c.g[V0][0]);
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x35a8));
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x2234), c.g[V1][0]);
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x35a8));
                    c.sb((c.g[A0][0] as u32).wrapping_add(0x3b34), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x35a8));
                    if t {
                        if back_edge_unwinds(c, 0x002b2b30) {
                            return Ok(());
                        }
                        pc = 0x002b2b30;
                        continue;
                    }
                    pc = 0x002b2b54;
                    continue;
                }
                0x002b2b54 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6f8));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002b2b64);
                    c.g[A2][0] = 0x100;
                    Callee {
                        addr: 0x002a77f8,
                        f: ee::s08::IniParser_ctor,
                    }
                    .call(c, 0x002b2b64)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3570));
                    c.g[RA][0] = sx(0x002b2b78);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                    Callee {
                        addr: 0x002a7bf8,
                        f: ee::s08::IniParser_FindSection,
                    }
                    .call(c, 0x002b2b78)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0x480000;
                    if t {
                        pc = 0x002b2bb0;
                        continue;
                    }
                    pc = 0x002b2b80;
                    continue;
                }
                0x002b2b80 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x002b2ba0;
                        continue;
                    }
                    pc = 0x002b2b88;
                    continue;
                }
                0x002b2b88 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002b2b90);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7da0,
                        f: ee::s08::IniParser_ReadToken,
                    }
                    .call(c, 0x002b2b90)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28c0));
                    c.g[RA][0] = sx(0x002b2ba0);
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x104));
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002b2ba0)?;
                    pc = 0x002b2ba0;
                    continue;
                }
                0x002b2ba0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002b2ba8);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7f90,
                        f: ee::s08::IniParser_AtSectionEnd,
                    }
                    .call(c, 0x002b2ba8)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002b2b88) {
                            return Ok(());
                        }
                        pc = 0x002b2b88;
                        continue;
                    }
                    pc = 0x002b2bb0;
                    continue;
                }
                0x002b2bb0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x870));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3e4), 0u64);
                    c.g[S6][0] = 0x64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3ec), 0u64);
                    if t {
                        pc = 0x002b3330;
                        continue;
                    }
                    pc = 0x002b2bcc;
                    continue;
                }
                0x002b2bcc => {
                    c.cycles += 1;
                    pc = 0x002b2bd0;
                    continue;
                }
                0x002b2bd0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002b2bd8);
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3570));
                    call_ptr(c, 0x0041aa88, 0x002b2bd8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0xb;
                    if t {
                        pc = 0x002b3330;
                        continue;
                    }
                    pc = 0x002b2be0;
                    continue;
                }
                0x002b2be0 => {
                    c.cycles += 26;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[V1][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x3f8));
                    c.g[V0][0] = 0x1;
                    c.mult(0, 4, 5, 4);
                    c.g[V0][0] = c.g[V0][0] << (c.g[A1][0] & 63);
                    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.sd((c.g[S2][0] as u32).wrapping_add(0x3f8), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x124));
                    c.g[RA][0] = sx(0x002b2c0c);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, 0x004166f4, 0x002b2c0c)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3458));
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002b2c24);
                    c.g[A0][0] = 0xac;
                    call_ptr(c, 0x00317d70, 0x002b2c24)?;
                    c.g[RA][0] = sx(0x002b2c2c);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x002b1a98,
                        f: ee::s08::cSongConfiguration_ctor,
                    }
                    .call(c, 0x002b2c2c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    let t = true;
                    c.g[S4][0] = c.g[A0][0];
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2c48;
                    continue;
                }
                0x002b2c48 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002b2c50);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7da0,
                        f: ee::s08::IniParser_ReadToken,
                    }
                    .call(c, 0x002b2c50)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x37f0));
                    c.g[RA][0] = sx(0x002b2c68);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2c68)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b2c88;
                        continue;
                    }
                    pc = 0x002b2c70;
                    continue;
                }
                0x002b2c70 => {
                    c.cycles += 6;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.mult(0, 6, 6, 22);
                    let t = true;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x934));
                    if t {
                        pc = 0x002b2ce4;
                        continue;
                    }
                    pc = 0x002b2c88;
                    continue;
                }
                0x002b2c88 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x37f8));
                    c.g[RA][0] = sx(0x002b2c98);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2c98)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b2cb8;
                        continue;
                    }
                    pc = 0x002b2ca0;
                    continue;
                }
                0x002b2ca0 => {
                    c.cycles += 6;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.mult(0, 6, 6, 22);
                    let t = true;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2234));
                    if t {
                        pc = 0x002b2ce4;
                        continue;
                    }
                    pc = 0x002b2cb8;
                    continue;
                }
                0x002b2cb8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3800));
                    c.g[RA][0] = sx(0x002b2cc8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2cc8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b2cf4;
                        continue;
                    }
                    pc = 0x002b2cd0;
                    continue;
                }
                0x002b2cd0 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.mult(0, 6, 6, 22);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3b34));
                    pc = 0x002b2ce4;
                    continue;
                }
                0x002b2ce4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002b2cec);
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002b2cec)?;
                    let t = true;

                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2cf4;
                    continue;
                }
                0x002b2cf4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3808));
                    c.g[RA][0] = sx(0x002b2d04);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2d04)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2d3c;
                        continue;
                    }
                    pc = 0x002b2d0c;
                    continue;
                }
                0x002b2d0c => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002b2d1c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x890));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2d1c)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x890));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2d2c;
                    continue;
                }
                0x002b2d2c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3ec), c.g[V0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2d3c;
                    continue;
                }
                0x002b2d3c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3470));
                    c.g[RA][0] = sx(0x002b2d4c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2d4c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2d94;
                        continue;
                    }
                    pc = 0x002b2d54;
                    continue;
                }
                0x002b2d54 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002b2d64);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x894));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2d64)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x894));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x14;
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2d74;
                    continue;
                }
                0x002b2d74 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                    c.mult(0, 3, 3, 4);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2d94;
                    continue;
                }
                0x002b2d94 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3480));
                    c.g[RA][0] = sx(0x002b2da4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2da4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b2ddc;
                        continue;
                    }
                    pc = 0x002b2dac;
                    continue;
                }
                0x002b2dac => {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002b2dbc);
                    c.g[A2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002b2dbc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x002b2dd4);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x004166f4, 0x002b2dd4)?;
                    let t = true;

                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2ddc;
                    continue;
                }
                0x002b2ddc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3810));
                    c.g[RA][0] = sx(0x002b2dec);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2dec)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2e28;
                        continue;
                    }
                    pc = 0x002b2df4;
                    continue;
                }
                0x002b2df4 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2e04);
                    c.g[A2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002b2e04)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[RA][0] = sx(0x002b2e20);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x004166f4, 0x002b2e20)?;
                    let t = true;

                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2e28;
                    continue;
                }
                0x002b2e28 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3490));
                    c.g[RA][0] = sx(0x002b2e38);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2e38)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2e98;
                        continue;
                    }
                    pc = 0x002b2e40;
                    continue;
                }
                0x002b2e40 => {
                    c.cycles += 22;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2e50);
                    c.g[A2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x002a7ff8,
                        f: ee::s08::IniParser_GetString,
                    }
                    .call(c, 0x002b2e50)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x002b2e78);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, 0x004166f4, 0x002b2e78)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2e98;
                    continue;
                }
                0x002b2e98 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34a0));
                    c.g[RA][0] = sx(0x002b2ea8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2ea8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2edc;
                        continue;
                    }
                    pc = 0x002b2eb0;
                    continue;
                }
                0x002b2eb0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2ec0);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2ec0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2edc;
                    continue;
                }
                0x002b2edc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34b0));
                    c.g[RA][0] = sx(0x002b2eec);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2eec)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2f20;
                        continue;
                    }
                    pc = 0x002b2ef4;
                    continue;
                }
                0x002b2ef4 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2f04);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2f04)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2f20;
                    continue;
                }
                0x002b2f20 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34c0));
                    c.g[RA][0] = sx(0x002b2f30);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2f30)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2f64;
                        continue;
                    }
                    pc = 0x002b2f38;
                    continue;
                }
                0x002b2f38 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2f48);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2f48)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2f64;
                    continue;
                }
                0x002b2f64 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34d0));
                    c.g[RA][0] = sx(0x002b2f74);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2f74)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2fa8;
                        continue;
                    }
                    pc = 0x002b2f7c;
                    continue;
                }
                0x002b2f7c => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2f8c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2f8c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2fa8;
                    continue;
                }
                0x002b2fa8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34e0));
                    c.g[RA][0] = sx(0x002b2fb8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2fb8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b2fec;
                        continue;
                    }
                    pc = 0x002b2fc0;
                    continue;
                }
                0x002b2fc0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b2fd0);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b2fd0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b2fec;
                    continue;
                }
                0x002b2fec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34f0));
                    c.g[RA][0] = sx(0x002b2ffc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b2ffc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b3030;
                        continue;
                    }
                    pc = 0x002b3004;
                    continue;
                }
                0x002b3004 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3014);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3014)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3030;
                    continue;
                }
                0x002b3030 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3500));
                    c.g[RA][0] = sx(0x002b3040);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b3040)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b3074;
                        continue;
                    }
                    pc = 0x002b3048;
                    continue;
                }
                0x002b3048 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3058);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3058)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3074;
                    continue;
                }
                0x002b3074 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3510));
                    c.g[RA][0] = sx(0x002b3084);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b3084)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b30b8;
                        continue;
                    }
                    pc = 0x002b308c;
                    continue;
                }
                0x002b308c => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b309c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b309c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b30b8;
                    continue;
                }
                0x002b30b8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3520));
                    c.g[RA][0] = sx(0x002b30c8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b30c8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b30fc;
                        continue;
                    }
                    pc = 0x002b30d0;
                    continue;
                }
                0x002b30d0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b30e0);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b30e0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b30fc;
                    continue;
                }
                0x002b30fc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3530));
                    c.g[RA][0] = sx(0x002b310c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b310c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b3140;
                        continue;
                    }
                    pc = 0x002b3114;
                    continue;
                }
                0x002b3114 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3124);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3124)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3140;
                    continue;
                }
                0x002b3140 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3540));
                    c.g[RA][0] = sx(0x002b3150);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b3150)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b3184;
                        continue;
                    }
                    pc = 0x002b3158;
                    continue;
                }
                0x002b3158 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3168);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3168)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3184;
                    continue;
                }
                0x002b3184 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3818));
                    c.g[RA][0] = sx(0x002b3194);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b3194)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b31c8;
                        continue;
                    }
                    pc = 0x002b319c;
                    continue;
                }
                0x002b319c => {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002b31ac);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x89c));
                    Callee {
                        addr: 0x002a8070,
                        f: ee::s08::IniParser_GetFloat,
                    }
                    .call(c, 0x002b31ac)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x89c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b31c8;
                    continue;
                }
                0x002b31c8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3550));
                    c.g[RA][0] = sx(0x002b31d8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b31d8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b320c;
                        continue;
                    }
                    pc = 0x002b31e0;
                    continue;
                }
                0x002b31e0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b31f0);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b31f0)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b320c;
                    continue;
                }
                0x002b320c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3820));
                    c.g[RA][0] = sx(0x002b321c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b321c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002b3250;
                        continue;
                    }
                    pc = 0x002b3224;
                    continue;
                }
                0x002b3224 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3234);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3234)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3250;
                    continue;
                }
                0x002b3250 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3560));
                    c.g[RA][0] = sx(0x002b3260);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b3260)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b3294;
                        continue;
                    }
                    pc = 0x002b3268;
                    continue;
                }
                0x002b3268 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b3278);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b3278)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x94), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b3294;
                    continue;
                }
                0x002b3294 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3828));
                    c.g[RA][0] = sx(0x002b32a4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002b32a4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002b32d8;
                        continue;
                    }
                    pc = 0x002b32ac;
                    continue;
                }
                0x002b32ac => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b32bc);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b32bc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[A0][0]);
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b32d8;
                    continue;
                }
                0x002b32d8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3830));
                    c.g[RA][0] = sx(0x002b32e8);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002b32e8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x002b3314;
                        continue;
                    }
                    pc = 0x002b32f0;
                    continue;
                }
                0x002b32f0 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002b32fc);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x898));
                    Callee {
                        addr: 0x002a80d8,
                        f: ee::s08::IniParser_GetInt,
                    }
                    .call(c, 0x002b32fc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x898));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[A0][0]);
                    pc = 0x002b3314;
                    continue;
                }
                0x002b3314 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002b331c);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7f90,
                        f: ee::s08::IniParser_AtSectionEnd,
                    }
                    .call(c, 0x002b331c)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002b2c48) {
                            return Ok(());
                        }
                        pc = 0x002b2c48;
                        continue;
                    }
                    pc = 0x002b3324;
                    continue;
                }
                0x002b3324 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3e4), c.g[V0][0]);
                    pc = 0x002b3330;
                    continue;
                }
                0x002b3330 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 64i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x002b3358;
                        continue;
                    }
                    pc = 0x002b3340;
                    continue;
                }
                0x002b3340 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002b3348);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x002a7b08,
                        f: ee::s08::IniParser_NextSection,
                    }
                    .call(c, 0x002b3348)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if back_edge_unwinds(c, 0x002b2bd0) {
                            return Ok(());
                        }
                        pc = 0x002b2bd0;
                        continue;
                    }
                    pc = 0x002b3354;
                    continue;
                }
                0x002b3354 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    pc = 0x002b3358;
                    continue;
                }
                0x002b3358 => {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x002b3360);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x002a7890,
                        f: ee::s08::IniParser_dtor,
                    }
                    .call(c, 0x002b3360)?;
                    c.g[V0][0] = c.g[S2][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x920));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x910));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x900));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x8f0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x8e0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8a0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x940));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002b28c0, 0x002b3394);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (the fakes fall back to the previous
    /// body unless a fixture writes `OC808_MAGIC` here).
    const OC808_CTRL: u32 = DATA + 0x80_0000;
    const OC808_MAGIC: u32 = 0x0808_f4e5;
    /// Scripted results, 0x100 bytes per fake: word 0 is the call cursor,
    /// words 1..64 the results (`value + 1`, 0 = the fake's default).
    const OC808_QUEUE: u32 = DATA + 0x82_0000;
    /// The object under construction / the parser.
    const OC808_OBJ: u32 = DATA + 0x84_0000;
    /// Text the parser reads.
    const OC808_STR: u32 = DATA + 0x8e_0000;
    /// Pointer zone for the music player's embedded sub-objects.
    const OC808_ZONE: u32 = DATA + 0x90_0000;

    /// A callee fake: logs like `fakes::fake_call`, then returns the next
    /// scripted result of its queue (in `v0`, and `4 * (v & 3)` in `v1`).
    fn oc808_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let q = OC808_QUEUE + idx * 0x100;
        let cur = c.bus.read32(q);
        c.bus.write32(q, cur + 1);
        let scripted = if cur < 63 {
            c.bus.read32(q + 4 + 4 * cur)
        } else {
            0
        };
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && scripted != 0 {
            c.g[V0][0] = (scripted - 1) as u64;
            c.g[V1][0] = ((scripted - 1) & 3) as u64 * 4;
        }
    }

    macro_rules! oc808_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC808_CTRL) != OC808_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc808_fake_call(c, $addr, $idx);
                }
            )*
            fn oc808_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc808_fakes!(
        (
            oc808_fake_get_config,
            OC808_PREV_GET_CONFIG,
            ee::s13::OVERRIDE_003B4DD8,
            ee::s13::f_003b4dd8__generated,
            0x003b_4dd8,
            0
        ),
        (
            oc808_fake_install_dts,
            OC808_PREV_INSTALL_DTS,
            ee::s13::OVERRIDE_003BC930,
            ee::s13::f_003bc930__generated,
            0x003b_c930,
            1
        ),
        (
            oc808_fake_set_config,
            OC808_PREV_SET_CONFIG,
            ee::s13::OVERRIDE_003B4FC0,
            ee::s13::f_003b4fc0__generated,
            0x003b_4fc0,
            2
        ),
        (
            oc808_fake_stream_params,
            OC808_PREV_STREAM_PARAMS,
            ee::s08::OVERRIDE_002AA910,
            ee::s08::f_002aa910__generated,
            0x002a_a910,
            3
        ),
        (
            oc808_fake_load_speech_inf,
            OC808_PREV_LOAD_SPEECH_INF,
            ee::s08::OVERRIDE_002B0088,
            ee::s08::f_002b0088__generated,
            0x002b_0088,
            4
        ),
        (
            oc808_fake_sound_ref_ctor,
            OC808_PREV_SOUND_REF_CTOR,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88,
            5
        ),
        (
            oc808_fake_stream_array_ctor,
            OC808_PREV_STREAM_ARRAY_CTOR,
            ee::s08::OVERRIDE_002AA490,
            ee::s08::f_002aa490__generated,
            0x002a_a490,
            6
        ),
        (
            oc808_fake_ini_ctor,
            OC808_PREV_INI_CTOR,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8,
            7
        ),
        (
            oc808_fake_find_section,
            OC808_PREV_FIND_SECTION,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8,
            8
        ),
        (
            oc808_fake_get_string,
            OC808_PREV_GET_STRING,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8,
            9
        ),
        (
            oc808_fake_at_section_end,
            OC808_PREV_AT_SECTION_END,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90,
            10
        ),
        (
            oc808_fake_get_int,
            OC808_PREV_GET_INT,
            ee::s08::OVERRIDE_002A80D8,
            ee::s08::f_002a80d8__generated,
            0x002a_80d8,
            11
        ),
        (
            oc808_fake_get_float,
            OC808_PREV_GET_FLOAT,
            ee::s08::OVERRIDE_002A8070,
            ee::s08::f_002a8070__generated,
            0x002a_8070,
            12
        ),
        (
            oc808_fake_song_config_ctor,
            OC808_PREV_SONG_CONFIG_CTOR,
            ee::s08::OVERRIDE_002B1A98,
            ee::s08::f_002b1a98__generated,
            0x002b_1a98,
            13
        ),
        (
            oc808_fake_next_section,
            OC808_PREV_NEXT_SECTION,
            ee::s08::OVERRIDE_002A7B08,
            ee::s08::f_002a7b08__generated,
            0x002a_7b08,
            14
        ),
        (
            oc808_fake_ini_dtor,
            OC808_PREV_INI_DTOR,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890,
            15
        ),
        (
            oc808_fake_read_token,
            OC808_PREV_READ_TOKEN,
            ee::s08::OVERRIDE_002A7DA0,
            ee::s08::f_002a7da0__generated,
            0x002a_7da0,
            16
        ),
        (
            oc808_fake_operator_new,
            OC808_PREV_OPERATOR_NEW,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30,
            17
        ),
        (
            oc808_fake_mem_alloc_named,
            OC808_PREV_MEM_ALLOC_NAMED,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            18
        ),
        (
            oc808_fake_strcasecmp,
            OC808_PREV_STRCASECMP,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88,
            19
        ),
        (
            oc808_fake_strcpy,
            OC808_PREV_STRCPY,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4,
            20
        ),
        (
            oc808_fake_snd_bank_man_ctor,
            OC808_PREV_SND_BANK_MAN_CTOR,
            ee::s07::OVERRIDE_0028B9B8,
            ee::s07::f_0028b9b8__generated,
            0x0028_b9b8,
            21
        ),
    );

    /// Script the results of fake `idx` (`value + 1` encoded, see `OC808_QUEUE`).
    fn oc808_script(c: &mut Cpu, idx: u32, values: &[u32]) {
        for (i, v) in values.iter().enumerate().take(63) {
            c.bus
                .write32(OC808_QUEUE + idx * 0x100 + 4 + 4 * i as u32, v + 1);
        }
    }

    /// Text for the parser: some blanks and comments, a token, a terminator.
    fn oc808_token_text(r: &mut Rng, seed: u32) -> Vec<u8> {
        let lead: &[u8] = [
            &b""[..],
            b"  ",
            b"\t\n ",
            b"# comment\n  ",
            b"; c\r\n",
            b"[",
            b"  # x\n; y\n ",
        ][(seed % 7) as usize];
        let mut t = lead.to_vec();
        let n = r.next_u32() % 12;
        for _ in 0..n {
            t.push(b"abcXYZ019_.-/"[(r.next_u32() % 13) as usize]);
        }
        t.push([0, b' ', b'\t', b'\n', b'\r'][((seed / 7) % 5) as usize]);
        t.extend_from_slice(b"tail\0");
        t
    }

    /// `IniParser_ReadToken`: skip blanks/comments (real `SkipWhitespace`,
    /// `SkipComment`, `GetChar`), then copy the token into the parser's
    /// buffer at +0xc. Leading blanks, comments, `[`, token lengths 0..11 and
    /// all five terminators.
    #[test]
    fn ini_parser_read_token_matches() {
        check(
            ee::s08::f_002a7da0__generated,
            ini_parser_read_token,
            140,
            &|c, seed| {
                let mut r = Rng(0x0808_a001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002a_7da0;
                let text = oc808_token_text(&mut r, seed);
                c.bus.write_bytes(OC808_STR, &text);
                c.bus.write32(OC808_OBJ + 8, OC808_STR);
                c.g[A0][0] = sx(OC808_OBJ);
            },
        );
    }

    /// `PS2SoundManager_SetOutputMode`: modes -2..5 against the stored mode
    /// 0..3 reach every branch; the three sound-library callees are gated
    /// fakes (one unwinds on some seeds).
    #[test]
    fn ps2_sound_manager_set_output_mode_matches() {
        oc808_install_fakes();
        check(
            ee::s08::f_002a8d90__generated,
            ps2_sound_manager_set_output_mode,
            96,
            &|c, seed| {
                machine(c, seed, 0x808b, 3);
                c.pc = 0x002a_8d90;
                c.bus.write32(OC808_CTRL, OC808_MAGIC);
                let mode = [-2i32, -1, 0, 1, 2, 3, 4, 5][(seed % 8) as usize];
                c.bus.write32(OC808_OBJ + 8, (seed / 8) % 4);
                c.g[A0][0] = sx(OC808_OBJ);
                c.g[A1][0] = sx(mode as u32);
            },
        );
    }

    /// `SpeechInstance_ctor`: field clears, the slot-table loop and the two
    /// gated callees (stream creation, speech info loading; unwinding on some
    /// seeds). `gp+0x6c4` points at the stream system pointer.
    #[test]
    fn speech_instance_ctor_matches() {
        oc808_install_fakes();
        check(
            ee::s08::f_002afa68__generated,
            speech_instance_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x808c, 2);
                c.pc = 0x002a_fa68;
                c.bus.write32(OC808_CTRL, OC808_MAGIC);
                c.bus.write32(GAME_GP + 0x6c4, OC808_ZONE);
                c.bus.write32(OC808_ZONE, OC808_ZONE + 0x100);
                let mut r = Rng(0x0808_c001 ^ seed as u64);
                for i in 0..0x240 {
                    c.bus.write32(OC808_OBJ + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(OC808_OBJ);
            },
        );
    }

    /// `MusicPlayer_ctor`: with `inchrg` 0 the sound manager reference, bank
    /// manager and stream array are built (gated fakes, pointer chain from
    /// the object laid out in `OC808_ZONE`), then the INI loop runs over
    /// scripted `strcasecmp` / `AtSectionEnd` / `NextSection` results so the
    /// section, property and song-count branches vary across seeds. Every INI
    /// and allocator callee is a gated fake, unwinding on some seeds.
    #[test]
    fn music_player_ctor_matches() {
        oc808_install_fakes();
        check(
            ee::s08::f_002b28c0__generated,
            music_player_ctor,
            80,
            &|c, seed| {
                machine(c, seed, 0x808d, 60);
                c.pc = 0x002b_28c0;
                c.bus.write32(OC808_CTRL, OC808_MAGIC);
                let mut r = Rng(0x0808_d001 ^ (seed as u64) << 4);
                let inchrg = (seed % 3 != 0) as u64;
                // stream system chain: obj+0 -> a -> b, b+0x1d8 -> c
                c.bus.write32(OC808_OBJ, OC808_ZONE);
                c.bus.write32(OC808_ZONE, OC808_ZONE + 0x1000);
                c.bus
                    .write32(OC808_ZONE + 0x1000 + 0x1d8, OC808_ZONE + 0x2000);
                // GetInt results the body reads from its frame (sp - 0xb0)
                let sp = c.g[SP][0] as u32;
                c.bus.write32(sp - 0xb0, seed % 2);
                c.bus.write32(sp - 0xac, seed % 3);
                // scripted results
                let mut cmp = [0u32; 63];
                for (i, v) in cmp.iter_mut().enumerate() {
                    // 1 = match (strcasecmp 0), 2 = no match
                    *v = if i == 0 || r.next_u32() % 4 == 0 {
                        1
                    } else {
                        2
                    };
                }
                oc808_script(c, 19, &cmp);
                let mut end = [0u32; 63];
                for v in end.iter_mut() {
                    *v = 1 + (r.next_u32() % 3 == 0) as u32; // 1 = more, 2 = end
                }
                oc808_script(c, 10, &end);
                let mut next = [0u32; 63];
                for (i, v) in next.iter_mut().enumerate() {
                    *v = if i == 62 || r.next_u32() % 3 == 0 {
                        1
                    } else {
                        2
                    };
                }
                oc808_script(c, 14, &next);
                let mut ints = [0u32; 63];
                for v in ints.iter_mut() {
                    *v = r.next_u32() % 3;
                }
                oc808_script(c, 11, &ints);
                c.g[A0][0] = sx(OC808_OBJ);
                c.g[A1][0] = inchrg;
                c.g[A2][0] = 0x1000;
                c.g[A3][0] = 8;
                c.g[T0][0] = 0x2000;
            },
        );
    }
}
