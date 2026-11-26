//! Staging module oc769 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc769`.

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
    (
        0x002a_4c28,
        crowd_audio_add_sample_file,
        "CrowdAudio_AddSampleFile",
    ),
    (
        0x0039_a3f8,
        ui_list_box_add_item_ascii,
        "UIListBox_AddItemAscii",
    ),
    (0x002c_6e98, loc_string_table_load, "LocStringTable_Load"),
    (0x002a_9350, snd_play_params_ctor, "SndPlayParams_ctor"),
    (
        0x002b_3838,
        music_player_start_song,
        "MusicPlayer_StartSong",
    ),
];

/// `bool CrowdAudio_AddSampleFile(SampleList* list, const char* name, int flags)`
/// (0x2a4c28): appends the sample file `name` to a crowd sample list
/// (`count` at +0, up to 5 `items` from +4). Fails (0) when the list is full
/// or the file does not load (`EAFile_LoadFile` returns 0, the slot written
/// anyway); otherwise bumps `count` and returns 1.
pub fn crowd_audio_add_sample_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a4c90: {
            'b_2a4c88: {
                'b_2a4c80: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[A2][0];
                    if t {
                        break 'b_2a4c80;
                    } else {
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x002a4c54);
                        call_ptr(c, 0x003e1908, 0x002a4c54)?;
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                        let t = c.g[V1][0] != 0u64;
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_2a4c88;
                        } else {
                            break 'b_2a4c80;
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_2a4c90;
                } else {
                    break 'b_2a4c88;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            break 'b_2a4c90;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `int UIListBox_AddItemAscii(UIListBox* list, const char* text, int data)`
/// (0x39a3f8, frame 0x50): converts the ASCII `text` to a freshly allocated
/// wide string (`strlen`, `operator_new_named`, `UI_AsciiToWide`) and stores
/// the entry `{wide text +0x9c, text +0xa0, data +0xa4}` (stride 0x14) at the
/// list's count byte (+0x318), then bumps the count. Returns the new entry's
/// index (the old count).
pub fn ui_list_box_add_item_ascii(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = c.g[A2][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039a424);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416810, 0x0039a424)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039a440);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3dd0));
        call_ptr(c, 0x00317e30, 0x0039a440)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0039a450);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x00397ad8,
            f: ee::s13::f_00397ad8,
        }
        .call(c, 0x0039a450)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[A0][0] = 0x14;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 3, 2, 4);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[S2][0]);
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sb((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xff;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void LocStringTable_Load(LocStringTable* t, const char* name, char lang)`
/// (0x2c6e98, frame 0x40): loads the locale file (`EAFile_LoadFileEx`), stores
/// the language (+0), file (+4), string count (`LocStringTable_GetInfo` 2,
/// +8), header (+0xc: file + the offset at file+4, zeroed when its 4-byte
/// magic does not match) and the table (+0x10, header + the offset at
/// header+4), then resolves every table entry's id to its string pointer
/// in place (`LocStringTable_ResolveString`, 8-byte entries).
pub fn loc_string_table_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c6f5c: {
            'b_2c6f08: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002c6ec0);
                c.sb(c.g[S1][0] as u32, c.g[A2][0]);
                call_ptr(c, 0x003e18c8, 0x002c6ec0)?;
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002c6ed0);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                call_ptr(c, 0x003db9d8, 0x002c6ed0)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A1][0] = 0x4a0000;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3978));
                c.g[A2][0] = 0x4;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002c6ef8);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                call_ptr(c, 0x00416b18, 0x002c6ef8)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    break 'b_2c6f08;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                    if t {
                        break 'b_2c6f5c;
                    } else {
                        break 'b_2c6f08;
                    }
                }
            }
            c.cycles += 6;
            c.g[S2][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if t {
                break 'b_2c6f5c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                'l_2c6f28: loop {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x002c6f40);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x002c6dc0,
                        f: ee::s08::f_002c6dc0,
                    }
                    .call(c, 0x002c6f40)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x002c6f28);
                        }
                        continue 'l_2c6f28;
                    } else {
                        break 'b_2c6f5c;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cSndPlayOpts::cSndPlayOpts(this, inchrg, heap)` (`SndPlayParams_ctor`,
/// 0x2a9350, frame 0x50): constructs the sound-manager ref at +0x88 when
/// `inchrg` is set, points the object's vtable at its (stack copy of the)
/// 0x483be0 table when not, then allocates the play-option arrays (current
/// 0x60 bytes, nine 0x10-byte per-depth arrays, the 0x40 envelope array, five
/// more 0x10 arrays), pushes one level (`depth` +0x1c from -1 to 0), seeds its
/// fields (volume 127, envelope 100/90/50, ...), runs `SNDplaysetdef` on the
/// level's descriptor, copies the result back into the object's current
/// options (+4..+0x1c) and pops the level.
pub fn snd_play_params_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a9380: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = c.g[A2][0];
            if t {
                break 'b_2a9380;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x88));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002a9380);
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                Callee {
                    addr: 0x002ade88,
                    f: ee::s08::f_002ade88,
                }
                .call(c, 0x002a9380)?;
                break 'b_2a9380;
            }
        }
        'b_2a93d4: {
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V1][0] = 0x480000;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3be0));
            let t = c.g[S0][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_2a93d4;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3be0));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x88));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_2a93d4;
            }
        }
        c.cycles += 251;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x480000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3178));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = 0x60;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x002a93f8);
        c.g[S0][0] = 0x18;
        call_ptr(c, 0x00317e30, 0x002a93f8)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3188));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9414);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9414)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3198));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9430);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9430)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31a8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a944c);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a944c)?;
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3750));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9468);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9468)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x60), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31b8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9484);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9484)?;
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3758));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a94a0);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a94a0)?;
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3760));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a94bc);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a94bc)?;
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3768));
        c.g[A0][0] = 0x40;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a94d8);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a94d8)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31c8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a94f4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a94f4)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x74), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31d8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9510);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9510)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31e8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a952c);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a952c)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31f8));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002a9548);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9548)?;
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x10;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x80), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3208));
        c.g[RA][0] = sx(0x002a9564);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9564)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x84), c.g[V0][0]);
        c.g[A2][0] = 0x8;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.mult(0, 2, 2, 16);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0xb));
        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.ldl(4, (c.g[S1][0] as u32).wrapping_add(0x13));
        c.ldr(4, (c.g[S1][0] as u32).wrapping_add(0xc));
        c.ldl(7, (c.g[S1][0] as u32).wrapping_add(0x1b));
        c.ldr(7, (c.g[S1][0] as u32).wrapping_add(0x14));
        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[V0][0] as u32);
        c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x24), 0u64);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 16);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = sx(0x002a9630);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002a9630)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A3][0] = 0x7f;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[T0][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[A3][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[T0][0]);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 16);
        c.g[RA][0] = sx(0x002a975c);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003ba020, 0x002a975c)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.g[S1][0];
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 16);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A0][0] as u32);
        c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
        c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
        c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0xb));
        c.sdr(3, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x13));
        c.sdr(5, (c.g[S1][0] as u32).wrapping_add(0xc));
        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0x1b));
        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void MusicPlayer_StartSong(MusicPlayer* this, const char* name, int a2, int a3)`
/// (0x2b3838, frame 0xa0): stops the running song, rescales the bank
/// monitor's base volume by its volume control (clamped 0..127) when one is
/// set, then picks the song: with a `name` the index of the last song whose
/// name (`this+0x124`, 0xb bytes each) matches (stored at +0x3f0), else the
/// current one; creates its `SongInstance` (+0x408), loads the loop bank and
/// marks the player playing (+0x418 = 1, +0x41c = -1).
pub fn music_player_start_song(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b396c: {
            c.cycles += 30;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S5][0] = c.g[A1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[FP][0] = c.g[A3][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x002b3878);
            c.g[S7][0] = c.g[A2][0];
            Callee {
                addr: 0x002b3ac0,
                f: ee::s08::f_002b3ac0,
            }
            .call(c, 0x002b3878)?;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x38));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[S4][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[S4][0] == 0u64;
            c.g[S6][0] = c.lb(c.g[V0][0] as u32);
            if t {
                break 'b_2b396c;
            } else {
                'b_2b3968: {
                    'b_2b393c: {
                        c.cycles += 29;
                        c.mtc1(22, 0);
                        c.cvt_s_w(0, 0);
                        c.lwc1(1, c.g[S4][0] as u32);
                        c.g[AT][0] = 0x42fe0000;
                        c.mtc1(1, 2);
                        c.g[A2][0] = 0x18;
                        c.mtc1(0, 3);
                        c.mul_s(0, 0, 1);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.min_s(0, 0, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.max_s(0, 0, 3);
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                        c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 6);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if t {
                            break 'b_2b393c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_2b3968;
                            } else {
                                break 'b_2b393c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_2b3968;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        break 'b_2b3968;
                    }
                }
                c.cycles += 1;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                break 'b_2b396c;
            }
        }
        'b_2b3a1c: {
            'b_2b39dc: {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64;
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
                if t {
                    break 'b_2b39dc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3e4));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    'l_2b3980: loop {
                        'b_2b39a4: {
                            c.cycles += 2;
                            let t = (c.g[S0][0] as i64) < 0;
                            c.g[A1][0] = c.g[S5][0];
                            if t {
                                break 'b_2b39a4;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = 0xb;
                                c.mult(0, 4, 16, 4);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x124));
                                c.g[RA][0] = sx(0x002b399c);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, 0x004165a8, 0x002b399c)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002b3980);
                                    }
                                    continue 'l_2b3980;
                                } else {
                                    break 'b_2b39a4;
                                }
                            }
                        }
                        c.cycles += 14;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x3f0), c.g[S0][0]);
                        c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A3][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = c.g[S5][0];
                        c.g[T0][0] = c.g[S6][0];
                        c.g[T1][0] = c.g[S4][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1d8));
                        c.g[T2][0] = c.g[S7][0];
                        let t = true;
                        c.g[T3][0] = c.g[FP][0];
                        if t {
                            break 'b_2b3a1c;
                        } else {
                            break 'b_2b39dc;
                        }
                    }
                }
            }
            c.cycles += 16;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3f0));
            c.g[A1][0] = 0xb;
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.g[T0][0] = c.g[S6][0];
            c.mult(0, 5, 2, 5);
            c.g[T1][0] = c.g[S4][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A3][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T2][0] = c.g[S7][0];
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1d8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x124));
            c.g[T3][0] = c.g[FP][0];
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
            break 'b_2b3a1c;
        }
        c.cycles += 21;
        c.g[RA][0] = sx(0x002b3a24);
        Callee {
            addr: 0x002b1b98,
            f: ee::s08::f_002b1b98,
        }
        .call(c, 0x002b3a24)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x408), c.g[V0][0]);
        c.g[RA][0] = sx(0x002b3a30);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x002b43d8,
            f: ee::s08::f_002b43d8,
        }
        .call(c, 0x002b3a30)?;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x418), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x41c), c.g[V1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word (+4: make `EAFile_LoadFile` return 0; +8: `v1`
    /// after the `EAFile_LoadFileEx` fake) and the fixture objects, clear of
    /// the fakes' call log (`DATA + 0x10_0000`) and pointer range
    /// (`0x80_0000..0x84_0000`).
    const OC769_CTRL: u32 = DATA + 0x2f_0000;
    const OC769_MAGIC: u32 = 0x4f43_3736 + 9;
    const OC769_THIS: u32 = DATA + 0x20_0000;

    /// Gated fakes for the callees that have an override slot (allocators,
    /// file loaders, the sound and song callees). `strlen`, `strcmp`,
    /// `strncmp`, `memset`, `UI_AsciiToWide`, `LocStringTable_GetInfo`,
    /// `SongInstance_Create`/`_ctor` and the syscall wrapper run for real.
    macro_rules! oc769_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC769_CTRL) != OC769_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc769_install_plain_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc769_fakes!(
        (
            oc769_fake_2ade88,
            OC769_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc769_fake_317e30,
            OC769_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc769_fake_317d70,
            OC769_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc769_fake_3ba020,
            OC769_PREV_3BA020,
            ee::s13::OVERRIDE_003BA020,
            ee::s13::f_003ba020__generated,
            0x003b_a020
        ),
        (
            oc769_fake_2c6dc0,
            OC769_PREV_2C6DC0,
            ee::s08::OVERRIDE_002C6DC0,
            ee::s08::f_002c6dc0__generated,
            0x002c_6dc0
        ),
        (
            oc769_fake_2b3ac0,
            OC769_PREV_2B3AC0,
            ee::s08::OVERRIDE_002B3AC0,
            ee::s08::f_002b3ac0__generated,
            0x002b_3ac0
        ),
        (
            oc769_fake_417828,
            OC769_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc769_fake_2b4a20,
            OC769_PREV_2B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc769_fake_2b21e0,
            OC769_PREV_2B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc769_fake_2b43d8,
            OC769_PREV_2B43D8,
            ee::s08::OVERRIDE_002B43D8,
            ee::s08::f_002b43d8__generated,
            0x002b_43d8
        ),
        (
            oc769_fake_2ae478,
            OC769_PREV_2AE478,
            ee::s08::OVERRIDE_002AE478,
            ee::s08::f_002ae478__generated,
            0x002a_e478
        ),
    );

    /// `EAFile_LoadFile` (0x3e1908): the plain fake, but with the control word
    /// at +4 set the file does not load and returns 0.
    static OC769_PREV_3E1908: AtomicUsize = AtomicUsize::new(0);
    fn oc769_fake_3e1908(c: &mut Cpu) {
        if c.bus.read32(OC769_CTRL) != OC769_MAGIC {
            return fakes::fall_back(c, &OC769_PREV_3E1908, ee::s14::f_003e1908__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x003e_1908);
        if c.pc == ret && c.bus.read32(OC769_CTRL + 4) != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `EAFile_LoadFileEx` (0x3e18c8): the plain fake (its v0 is forced to
    /// the fixture file by the first `LOG_RESULTS` word), with `v1` set to
    /// the control word at +8: `LocStringTable_GetInfo` returns it as the
    /// string count (it reads v1 without setting it).
    static OC769_PREV_3E18C8: AtomicUsize = AtomicUsize::new(0);
    fn oc769_fake_3e18c8(c: &mut Cpu) {
        if c.bus.read32(OC769_CTRL) != OC769_MAGIC {
            return fakes::fall_back(c, &OC769_PREV_3E18C8, ee::s14::f_003e18c8__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x003e_18c8);
        if c.pc == ret {
            c.g[V1][0] = c.bus.read32(OC769_CTRL + 8) as u64;
        }
    }

    fn oc769_install_fakes() {
        use std::ptr::addr_of_mut;
        oc769_install_plain_fakes();
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1908),
            oc769_fake_3e1908,
            &OC769_PREV_3E1908,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E18C8),
            oc769_fake_3e18c8,
            &OC769_PREV_3E18C8,
        );
    }

    /// Common start of a fixture: scrambled registers, the fake hooks, the
    /// gate word and the game's gp.
    fn oc769_machine(c: &mut Cpu, seed: u32, calls: u32) -> crate::difftest::Rng {
        let r = machine(c, seed, 0x769, calls);
        c.bus.write32(OC769_CTRL, OC769_MAGIC);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `CrowdAudio_AddSampleFile` on lists holding 0..6 entries (5 and 6: full
    /// list, no load), loads that succeed and (every fourth seed) fail, and a
    /// fake unwinding in the load.
    #[test]
    fn crowd_audio_add_sample_file_matches() {
        oc769_install_fakes();
        check(
            ee::s08::f_002a4c28__generated,
            crowd_audio_add_sample_file,
            80,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 1);
                c.bus.write32(OC769_CTRL + 4, (seed % 4 == 1) as u32);
                c.bus.write32(OC769_THIS, seed % 7);
                for i in 0..5 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(OC769_THIS + 4 + 4 * i, v);
                }
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = sx(OC769_THIS + 0x1000);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x002a_4c28;
            },
        );
    }

    /// `UIListBox_AddItemAscii` with texts of 0..5 characters (real `strlen`
    /// and `UI_AsciiToWide`, faked allocator) appended at list positions
    /// 0..5, and a fake unwinding in the allocation.
    #[test]
    fn ui_list_box_add_item_ascii_matches() {
        oc769_install_fakes();
        check_lohi(
            ee::s13::f_0039a3f8__generated,
            ui_list_box_add_item_ascii,
            80,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 1);
                c.bus.write8(OC769_THIS + 0x318, (seed % 6) as u8);
                let text = OC769_THIS + 0x2000;
                let len = seed % 6;
                for i in 0..len {
                    c.bus.write8(text + i, 0x20 + (r.next_u32() % 0x5f) as u8);
                }
                c.bus.write8(text + len, 0);
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = sx(text);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0039_a3f8;
            },
        );
    }

    /// `LocStringTable_Load` on a fixture file with 0..3 strings, a header
    /// magic that matches (the table is walked and each id resolved by the
    /// fake) or not (every third seed: header cleared), and a fake unwinding
    /// in the file load or one of the resolves.
    #[test]
    fn loc_string_table_load_matches() {
        oc769_install_fakes();
        const FILE: u32 = DATA + 0x21_0000;
        const MAGIC_STR: u32 = 0x004a_3978;
        check(
            ee::s08::f_002c6e98__generated,
            loc_string_table_load,
            90,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 4);
                // The file loader returns FILE (forced result of fake call 0).
                c.bus.write32(fakes::LOG_RESULTS, FILE);
                c.bus.write32(OC769_CTRL + 8, seed % 4);
                // `GetInfo(file, 2)`: halfword at +0xe (0), or'ed with v1.
                c.bus.write16(FILE + 0xe, 0);
                // header at FILE + 0x40, table at header + 0x100.
                c.bus.write32(FILE + 4, 0x40);
                c.bus.write32(FILE + 0x40 + 4, 0x100);
                c.bus.write32(MAGIC_STR, u32::from_le_bytes(*b"LOCS"));
                let magic = if seed % 3 == 1 {
                    u32::from_le_bytes(*b"XOCS")
                } else {
                    u32::from_le_bytes(*b"LOCS")
                };
                c.bus.write32(FILE + 0x40, magic);
                for i in 0..4 {
                    c.bus.write32(FILE + 0x140 + 8 * i, r.next_u32());
                    c.bus.write32(FILE + 0x140 + 8 * i + 4, r.next_u32());
                }
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = sx(OC769_THIS + 0x1000);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x002c_6e98;
            },
        );
    }

    /// `SndPlayParams_ctor` with `inchrg` 1 (sound-manager ref constructed)
    /// and 0 (vtable copy on the stack, `this` pointing at a vtable slot
    /// area), a zeroed object, the fifteen allocations and the level setup
    /// (real `memset`, faked allocator and `SNDplaysetdef`), and a fake
    /// unwinding in one of the first calls.
    #[test]
    fn snd_play_params_ctor_matches() {
        oc769_install_fakes();
        check(
            ee::s08::f_002a9350__generated,
            snd_play_params_ctor,
            80,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 18);
                c.bus.write32(OC769_THIS, OC769_THIS + 0x4000);
                for i in 1..0x30 {
                    c.bus.write32(OC769_THIS + 4 * i, r.next_u32());
                }
                c.bus.write32(OC769_THIS, OC769_THIS + 0x4000);
                // The constant table the vtable copy reads.
                for i in 0..0x10 {
                    c.bus.write32(0x0048_3be0 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x002a_9350;
            },
        );
    }

    /// `MusicPlayer_StartSong` with 0..3 songs, a name that is absent, present
    /// or null (the current song is restarted), a volume control that is
    /// null or scales the base volume by a table of factors (clamped to
    /// 0..127), and a fake unwinding in one of the calls. `SongInstance_Create`
    /// and its ctor run for real on faked allocators.
    #[test]
    fn music_player_start_song_matches() {
        oc769_install_fakes();
        const VS: u32 = OC769_THIS + 0x4000;
        const BM: u32 = OC769_THIS + 0x5000;
        const MON: u32 = BM + 0x1d8;
        const BASE_VOL: u32 = OC769_THIS + 0x6000;
        const VOL_CTRL: u32 = OC769_THIS + 0x6100;
        const CUR: u32 = OC769_THIS + 0x6200;
        const FACTOR: u32 = OC769_THIS + 0x6400;
        const CFG: u32 = OC769_THIS + 0x7000;
        const NAME: u32 = OC769_THIS + 0x2000;
        check(
            ee::s08::f_002b3838__generated,
            music_player_start_song,
            120,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 12);
                let songs = seed % 4;
                c.bus.write32(OC769_THIS, VS);
                c.bus.write32(VS, BM);
                for i in 0..4 {
                    c.bus.write32(OC769_THIS + 4 + 4 * i, CFG + 0x200 * i);
                }
                for i in 0..0x200u32 {
                    c.bus.write32(CFG + 4 * i, r.next_u32());
                }
                c.bus.write32(OC769_THIS + 0x3e4, songs);
                c.bus.write32(OC769_THIS + 0x3f0, (seed / 4) % 4);
                for i in 0..4u32 {
                    let a = OC769_THIS + 0x124 + 0xb * i;
                    for (k, b) in format!("song{i}").bytes().enumerate() {
                        c.bus.write8(a + k as u32, b);
                    }
                    c.bus.write8(a + 5, 0);
                }
                let want = (seed / 3) % 4;
                for (k, b) in format!("song{want}").bytes().enumerate() {
                    c.bus.write8(NAME + k as u32, b);
                }
                c.bus.write8(NAME + 5, 0);
                // Bank monitor: instance, volume tables, current options.
                let inst = (seed / 5) % 3;
                c.bus.write32(MON + 0x1c, inst);
                c.bus.write32(MON + 0x20, CUR);
                c.bus.write32(MON + 0x38, BASE_VOL);
                c.bus.write32(MON + 0x3c, VOL_CTRL);
                c.bus.write32(BM + 0x1f4, (seed / 7) % 3);
                const VOLS: [u32; 8] = [
                    0,
                    5,
                    0x7f,
                    0x80,
                    0xff,
                    0x7fff_ffff,
                    0xffff_ff80,
                    0x1234_5680,
                ];
                for i in 0..3u32 {
                    let v = VOLS[r.next_u32() as usize % 8];
                    c.bus.write32(BASE_VOL + 4 * i, v);
                    let ptr = if (seed / 11 + i) % 3 == 0 {
                        0
                    } else {
                        FACTOR + 4 * i
                    };
                    c.bus.write32(VOL_CTRL + 4 * i, ptr);
                }
                const FACTORS: [f32; 6] = [0.0, 0.5, 1.0, 1.5, 2.0, -1.0];
                for i in 0..3u32 {
                    let f = FACTORS[r.next_u32() as usize % 6];
                    c.bus.write32(FACTOR + 4 * i, f.to_bits());
                }
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(NAME) };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.pc = 0x002b_3838;
            },
        );
    }
}
