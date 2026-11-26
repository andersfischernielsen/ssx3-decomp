//! Staging module oc783 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc783`.

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
        0x0029_ed90,
        ssx_audio_system_set_language,
        "SSXAudioSystem_SetLanguage",
    ),
    (
        0x002d_b748,
        ccrowd_render2d_load_textures,
        "cCrowdRender2D_LoadTextures",
    ),
    (
        0x0019_4aa8,
        cfe_state_main_menu_init,
        "cFEStateMainMenu_Init",
    ),
];

/// `void SSXAudioSystem::OnSongChanged(SSXAudioSystem* this)` (0x28f478).
///
/// Re-applies the current music: calls the music system's start routine
/// (0x2b35a0, on `this + 0x118`), then, if the channel gain 1 of the audio
/// system is positive and the game world (`gp - 0x848`) has a current
/// frontend/UI state stack (`world + 0x84` -> `+ 0x48`), looks up the UI state
/// with the id `0x4a3660` (`cUIStateStack::findState`) and calls virtual slot
/// `+0xc0/+0xc4` of it with `(this, 5, 0)`. Nothing is returned.
///
/// Kept as a port only: no `OVERRIDES` entry and no test (its first callee,
/// 0x2b35a0, runs the music player (`MusicPlayer_StartSong`,
/// `SongInstance_Create` -> `SongInstance_ctor`, ...), real code with no
/// override slot, which cannot be faked and does not stay bounded on test data).
pub fn ssx_audio_system_on_song_changed(c: &mut Cpu) {
    guest(c, |c| {
        'b_28f514: {
            'b_28f510: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x002b35a0, 0x0028f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x00287930, 0x0028f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_28f510;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_28f514;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == 0u64;
                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        if t {
                            break 'b_28f510;
                        } else {
                            c.cycles += 2;
                            if c.g[S0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                break 'b_28f514;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = 0x4a0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3660));
                                call_ptr(c, 0x00317670, 0x0028f4e0)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039f9d8, 0x0028f4ec)?;
                                c.g[A3][0] = c.g[V0][0];
                                let t = c.g[A3][0] == 0u64;
                                c.g[A1][0] = 0x5;
                                if t {
                                    break 'b_28f510;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0028f510)?;
                                    break 'b_28f510;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_28f514;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `void SSXAudioSystem::SetLanguage(SSXAudioSystem* this, int initSpeech)` (0x29ed90).
///
/// Looks up the current language mask (`SSXAudioSystem_GetLanguageMask`). When it
/// differs from the one at `this + 0x572c`: shuts the speech system down
/// (`this + 0x5560`) when `initSpeech` is set, deletes the old speech stream
/// (`this + 0x5728`) if any, loads the speech file of the new language
/// (mask 2/4/8 pick 0x482df8/0x482e10/0x482e28, every other mask 0x482e40) with
/// `FILESYS_*` when the file exists, and records the mask. Finally, with
/// `initSpeech`, restarts the speech system (`SPCH_Init`, `SPCH_CreateInstance`).
pub fn ssx_audio_system_set_language(c: &mut Cpu) {
    guest(c, |c| {
        'b_29eec4: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S4][0] = c.g[A1][0];
            call_ptr(c, 0x0029f198, 0x0029edb8)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x572c));
            let t = c.g[S3][0] == c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_29eec4;
            } else {
                'b_29eddc: {
                    c.cycles += 2;
                    if c.g[S4][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5728));
                        break 'b_29eddc;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                        call_ptr(c, 0x002b0da0, 0x0029edd8)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5728));
                        break 'b_29eddc;
                    }
                }
                'b_29edec: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5728));
                    if t {
                        break 'b_29edec;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = 0x64;
                        call_ptr(c, 0x003dedc0, 0x0029edec)?;
                        break 'b_29edec;
                    }
                }
                'b_29ee78: {
                    'b_29ee74: {
                        'b_29ee50: {
                            'b_29ee28: {
                                'b_29ee04: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[S3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_29ee04;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x480000;
                                        let t = true;
                                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2df8));
                                        if t {
                                            break 'b_29ee28;
                                        } else {
                                            break 'b_29ee04;
                                        }
                                    }
                                }
                                'b_29ee18: {
                                    c.cycles += 2;
                                    let t = c.g[S3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x8;
                                    if t {
                                        break 'b_29ee18;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x480000;
                                        let t = true;
                                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e10));
                                        if t {
                                            break 'b_29ee28;
                                        } else {
                                            break 'b_29ee18;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S3][0] != c.g[V0][0];
                                c.g[V0][0] = 0x480000;
                                if t {
                                    break 'b_29ee50;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x480000;
                                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e28));
                                    break 'b_29ee28;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00317f38, 0x0029ee30)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_29ee74;
                            } else {
                                c.cycles += 6;
                                c.g[A3][0] = c.g[S1][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x64;
                                call_ptr(c, 0x003ded50, 0x0029ee48)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x572c), c.g[S3][0]);
                                if t {
                                    break 'b_29ee78;
                                } else {
                                    break 'b_29ee50;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e40));
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00317f38, 0x0029ee5c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_29ee74;
                        } else {
                            c.cycles += 4;
                            c.g[A3][0] = c.g[S1][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x64;
                            call_ptr(c, 0x003ded50, 0x0029ee74)?;
                            break 'b_29ee74;
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x572c), c.g[S3][0]);
                    break 'b_29ee78;
                }
                'b_29eec0: {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5560));
                    if t {
                        break 'b_29eec0;
                    } else {
                        c.cycles += 16;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002b0ce0, 0x0029ee88)?;
                        c.g[S0][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3568));
                        c.g[A1][0] = 0x4;
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x00287968, 0x0029eea0)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A3][0] = c.g[S0][0];
                        c.g[T0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        c.g[T1][0] = 0x1;
                        c.g[T2][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x002b0e60, 0x0029eec0)?;
                        break 'b_29eec0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_29eec4;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void cCrowdRender2D::LoadTextures(cCrowdRender2D* this)` (0x2db748).
///
/// Loads the crowd sprite sheet (`FILE_load` 0x3e18c8 of "crwd.ssh", stored at
/// `this + 0`), then, for the one animation entry of the table at 0x445d3c
/// (count in the first word, names 5 bytes apart, shape and handle arrays),
/// finds each shape by name (`SHAPE_locate`, 0x3b39b8) and registers it with
/// the graphics manager's virtual slot `+0x170` (handle stored in the table).
/// Finally copies the first shape's `short`/`short`/`byte` fields (`+4`, `+6`,
/// `+0`) into `this + 4`, `+8`, `+0xc` (sprite width, height, format).
pub fn ccrowd_render2d_load_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[A1][0] = 0x5000000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = 0x480000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x75c0));
        call_ptr(c, 0x003e18c8, 0x002db788)?;
        c.g[V1][0] = 0x440000;
        c.sw(c.g[S3][0] as u32, c.g[V0][0]);
        c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5d3c));
        c.g[V0][0] = 0u64;
        c.g[S6][0] = c.g[S7][0];
        c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add(0x94));
        c.g[V1][0] = 0xd8;
        'l_2db7a8: loop {
            'b_2db830: {
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t {
                    break 'b_2db830;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    'l_2db7d8: loop {
                        c.cycles += 22;
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x003b39b8, 0x002db7e8)?;
                        c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.g[A1][0] = c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_sub(0x40), c.g[V0][0]);
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0u64;
                        c.g[T0][0] = 0u64;
                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002db81c)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002db7d8);
                            }
                            continue 'l_2db7d8;
                        } else {
                            break 'b_2db830;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S5][0];
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V1][0] = 0xd8;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002db7a8);
                }
                continue 'l_2db7a8;
            } else {
                c.cycles += 21;
                c.g[V0][0] = 0x440000;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d38));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x6));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void cFEStateMainMenu::Init(cFEStateMainMenu* this)` (0x194aa8).
///
/// Front-end main menu initialisation: adds the "main menu" screen to the UI
/// engine (`this + 0x10`; 0x397cc0, screen stored at `this + 0x40`) and
/// plays its first frame (0x39c870), clears the `cBERules` words at 0x5308b8
/// (0x20 bytes) and the flags `+0x48`/`+0x54`, resets the network/game
/// systems (0x266cd8, 0x261008, 0x25b688, `cGameComm_destroy` 0x256c50,
/// `cCommSystem_destroy` 0x255898), submits back-end interface 7 of the world
/// (`gp - 0x848`), selects audio state 1 (0x28b180, 0x28f140), then, for the
/// controller ports 0 and 1, calls two virtual slots of the engine's input
/// object, counting in `this + 0x55` the ports whose second call succeeded.
pub fn cfe_state_main_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_194afc: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00194ad0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00194ae4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_194afc;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00194afc)?;
                break 'b_194afc;
            }
        }
        c.cycles += 37;
        c.g[A0][0] = 0x530000;
        c.g[A2][0] = 0x20;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x003e6448, 0x00194b10)?;
        c.g[S2][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
        call_ptr(c, 0x00266cd8, 0x00194b20)?;
        call_ptr(c, 0x00261008, 0x00194b28)?;
        call_ptr(c, 0x0025b688, 0x00194b30)?;
        call_ptr(c, 0x00256c50, 0x00194b38)?;
        call_ptr(c, 0x00255898, 0x00194b40)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014dd58, 0x00194b50)?;
        c.g[A0][0] = 0x530000;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4b30), 0u64);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00194b6c)?;
        call_ptr(c, 0x0028b180, 0x00194b74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0028f140, 0x00194b80)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sb((c.g[S1][0] as u32).wrapping_add(0x55), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        'l_194b90: loop {
            'b_194bd0: {
                c.cycles += 13;
                c.g[A1][0] = c.g[S2][0] & 0xff;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x164));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00194ba8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x168));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00194bbc)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                if t {
                    break 'b_194bd0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x55));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x55), c.g[V0][0]);
                    break 'b_194bd0;
                }
            }
            c.cycles += 2;
            if c.g[V1][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                if event_due(c) {
                    deliver_event_here(c, 0x00194b90);
                }
                continue 'l_194b90;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc783's callee fakes in a machine's RAM.
    pub(crate) const OC783_CTRL: u32 = DATA + 0x30_0010;
    pub(crate) const OC783_MAGIC: u32 = 0x0783_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc783_mail(addr: u32) -> u32 {
        DATA + 0x30_2000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc783_arm(c: &mut Cpu) {
        c.bus.write32(OC783_CTRL, OC783_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc783_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc783_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc783_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc783_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc783_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc783_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc783_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC783_CTRL) != OC783_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc783_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc783_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc783_fakes!(
        (
            oc783_fk_3e18c8,
            OC783_PREV_3E18C8,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc783_fk_3b39b8,
            OC783_PREV_3B39B8,
            ee::s13::OVERRIDE_003B39B8,
            ee::s13::f_003b39b8__generated,
            0x003b_39b8
        ),
        (
            oc783_fk_317670,
            OC783_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc783_fk_397cc0,
            OC783_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc783_fk_39c870,
            OC783_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc783_fk_3e6448,
            OC783_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc783_fk_266cd8,
            OC783_PREV_266CD8,
            ee::s07::OVERRIDE_00266CD8,
            ee::s07::f_00266cd8__generated,
            0x0026_6cd8
        ),
        (
            oc783_fk_261008,
            OC783_PREV_261008,
            ee::s06::OVERRIDE_00261008,
            ee::s06::f_00261008__generated,
            0x0026_1008
        ),
        (
            oc783_fk_25b688,
            OC783_PREV_25B688,
            ee::s06::OVERRIDE_0025B688,
            ee::s06::f_0025b688__generated,
            0x0025_b688
        ),
        (
            oc783_fk_14dd58,
            OC783_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc783_fk_28b180,
            OC783_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc783_fk_2b0da0,
            OC783_PREV_2B0DA0,
            ee::s08::OVERRIDE_002B0DA0,
            ee::s08::f_002b0da0__generated,
            0x002b_0da0
        ),
        (
            oc783_fk_3dedc0,
            OC783_PREV_3DEDC0,
            ee::s14::OVERRIDE_003DEDC0,
            ee::s14::f_003dedc0__generated,
            0x003d_edc0
        ),
        (
            oc783_fk_317f38,
            OC783_PREV_317F38,
            ee::s10::OVERRIDE_00317F38,
            ee::s10::f_00317f38__generated,
            0x0031_7f38
        ),
        (
            oc783_fk_3ded50,
            OC783_PREV_3DED50,
            ee::s14::OVERRIDE_003DED50,
            ee::s14::f_003ded50__generated,
            0x003d_ed50
        ),
        (
            oc783_fk_287968,
            OC783_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc783_fk_2b0e60,
            OC783_PREV_2B0E60,
            ee::s08::OVERRIDE_002B0E60,
            ee::s08::f_002b0e60__generated,
            0x002b_0e60
        ),
        (
            oc783_fk_14dc80,
            OC783_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc783_fk_2adf60,
            OC783_PREV_2ADF60,
            ee::s08::OVERRIDE_002ADF60,
            ee::s08::f_002adf60__generated,
            0x002a_df60
        ),
        (
            oc783_fk_2b9100,
            OC783_PREV_2B9100,
            ee::s08::OVERRIDE_002B9100,
            ee::s08::f_002b9100__generated,
            0x002b_9100
        ),
        (
            oc783_fk_252fa0,
            OC783_PREV_252FA0,
            ee::s06::OVERRIDE_00252FA0,
            ee::s06::f_00252fa0__generated,
            0x0025_2fa0
        ),
    );

    pub(crate) const OC783_SYS: u32 = DATA + 0x1_0000;
    pub(crate) const OC783_SPEECH_STREAM: u32 = DATA + 0x2_0000;
    /// The word `SSXAudioSystem_GetLanguageMask` reads its mask from (bits 22..24).
    pub(crate) const OC783_LANG_WORD: u32 = 0x0053_5610;

    /// Fixture of `SSXAudioSystem_SetLanguage`: the mask word (0..4 in bits 22..24
    /// gives mask 1, 2, 4, 8, 1), the recorded mask (equal on some seeds), the old
    /// speech stream (present on some), and the file-exists result of 0x317f38.
    fn oc783_set_language_fixture(c: &mut Cpu, seed: u32, init_speech: bool) {
        let _r = machine(c, seed, 0x783a, 8);
        oc783_arm(c);
        c.g[A0][0] = sx(OC783_SYS);
        c.g[A1][0] = init_speech as u64;
        let k = seed % 5;
        c.bus.write32(OC783_LANG_WORD, k << 22);
        let mask = [1u32, 2, 4, 8, 1][k as usize];
        let recorded = if (seed / 5) % 4 == 0 { mask } else { mask ^ 3 };
        c.bus.write32(OC783_SYS + 0x572c, recorded);
        let stream = if (seed / 3) % 2 == 0 {
            0
        } else {
            OC783_SPEECH_STREAM
        };
        c.bus.write32(OC783_SYS + 0x5728, stream);
        if (seed / 2) % 2 == 0 {
            oc783_force_zero(c, 0x0031_7f38);
        }
    }

    /// `SSXAudioSystem_SetLanguage` without the speech-system restart: each
    /// language mask (2/4/8 and the default), unchanged language, old stream
    /// present or not, language file present or not, and the unwinds of its
    /// calls (the real `GetLanguageMask` runs, its two callees are faked).
    #[test]
    fn ssx_audio_system_set_language_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s08::f_0029ed90__generated,
            ssx_audio_system_set_language,
            250,
            &|c, seed| oc783_set_language_fixture(c, seed, false),
        );
    }

    /// `SSXAudioSystem_SetLanguage` with `initSpeech` set: also shuts down the
    /// speech system first and restarts it (the real `SPCH_Init` runs).
    #[test]
    fn ssx_audio_system_set_language_with_speech_init_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s08::f_0029ed90__generated,
            ssx_audio_system_set_language,
            120,
            &|c, seed| oc783_set_language_fixture(c, seed, true),
        );
    }

    pub(crate) const OC783_CROWD: u32 = DATA + 0x1_0000;
    /// The animation table of `cCrowdRender2D::LoadTextures` (count in word 0,
    /// shape pointer array at `+0x54`).
    pub(crate) const OC783_CROWD_TABLE: u32 = 0x0044_5d3c;
    pub(crate) const OC783_RENDERER: u32 = DATA + 0x2_0000;
    pub(crate) const OC783_RENDERER_VT: u32 = DATA + 0x2_1000;
    pub(crate) const OC783_SHAPE: u32 = DATA + 0x3_0000;

    /// `cCrowdRender2D_LoadTextures`: no, one and three table entries (the
    /// shape lookups and the renderer's virtual `+0x170` calls are faked),
    /// the first shape's size fields (the table's own pointer when empty) and
    /// the unwinds of every call.
    #[test]
    fn ccrowd_render2d_load_textures_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s09::f_002db748__generated,
            ccrowd_render2d_load_textures,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x783b, 5);
                oc783_arm(c);
                c.g[A0][0] = sx(OC783_CROWD);
                let n = [0u32, 1, 3, 2][(seed % 4) as usize];
                c.bus.write32(OC783_CROWD_TABLE, n);
                // The shape of entry 0 (read back from the table at the end).
                c.bus.write32(OC783_CROWD_TABLE + 0x54, OC783_SHAPE);
                c.bus.write16(OC783_SHAPE, r.next_u32() as u16);
                c.bus.write16(OC783_SHAPE + 4, r.next_u32() as u16);
                c.bus.write16(OC783_SHAPE + 6, r.next_u32() as u16);
                // The graphics manager behind `gp + 0x2a90`.
                c.bus.write32(GAME_GP + 0x2a90, OC783_RENDERER);
                c.bus.write32(OC783_RENDERER + 0x10d8, OC783_RENDERER_VT);
                c.bus
                    .write16(OC783_RENDERER_VT + 0x170, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_RENDERER_VT + 0x174, FAKE);
            },
        );
    }

    pub(crate) const OC783_MENU: u32 = DATA + 0x1_0000;
    pub(crate) const OC783_ENGINE: u32 = DATA + 0x2_0000;
    pub(crate) const OC783_INPUT: u32 = DATA + 0x3_0000;
    pub(crate) const OC783_INPUT_VT: u32 = DATA + 0x3_1000;
    pub(crate) const OC783_WORLD: u32 = DATA + 0x4_0000;
    pub(crate) const OC783_BE_IFACE: u32 = DATA + 0x4_1000;
    pub(crate) const OC783_BE_IFACE_VT: u32 = DATA + 0x4_2000;
    pub(crate) const OC783_AUDIO: u32 = DATA + 0x8_0000;

    /// `cFEStateMainMenu_Init`: the screen lookup succeeding or not, the
    /// second input call's result (0 or not) for both controller ports, and
    /// the unwinds of every call. The real `cGameComm_destroy` and
    /// `cCommSystem_destroy` wrappers run on empty globals and
    /// `func_0028F140` returns at once (audio state already selected).
    #[test]
    fn cfe_state_main_menu_init_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s02::f_00194aa8__generated,
            cfe_state_main_menu_init,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x783c, 12);
                oc783_arm(c);
                c.g[A0][0] = sx(OC783_MENU);
                c.bus.write32(OC783_MENU + 0x10, OC783_ENGINE);
                // The engine's input object and its two virtual calls.
                c.bus.write32(OC783_ENGINE + 0xc, OC783_INPUT);
                c.bus.write32(OC783_INPUT + 8, OC783_INPUT_VT);
                c.bus
                    .write16(OC783_INPUT_VT + 0x160, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_INPUT_VT + 0x164, FAKE + 0x30);
                c.bus
                    .write16(OC783_INPUT_VT + 0x168, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_INPUT_VT + 0x16c, 0x0039_c870);
                // The world, its back-end interface 7 and the interface's
                // virtual submit.
                c.bus.write32(GAME_GP - 0x848, OC783_WORLD);
                c.bus.write32(OC783_WORLD + 0x78, r.next_u32());
                oc783_force(c, 0x0014_dd58, OC783_BE_IFACE);
                c.bus.write32(OC783_BE_IFACE + 0xc, OC783_BE_IFACE_VT);
                c.bus
                    .write16(OC783_BE_IFACE_VT + 8, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_BE_IFACE_VT + 0xc, FAKE + 0x20);
                // `func_0028F140` returns at once when the state is the same.
                oc783_force(c, 0x0028_b180, OC783_AUDIO);
                c.bus.write32(OC783_AUDIO + 0x6290, 1);
                // Screen found (non-zero) or not; the 0x39c870 result is also the
                // second input call's result.
                if seed % 3 == 0 {
                    oc783_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 3) % 2 == 0 {
                    oc783_force_zero(c, 0x0039_c870);
                }
            },
        );
    }
}
