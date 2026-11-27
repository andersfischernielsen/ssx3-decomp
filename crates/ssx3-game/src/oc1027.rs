//! Staging module oc1027 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1027`.

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
        0x0028_cdf8,
        ssx_audio_system_play_character_tune,
        "SSXAudioSystem_PlayCharacterTune",
    ),
    (
        0x0028_e548,
        ssx_audio_system_on_music_cue,
        "SSXAudioSystem_OnMusicCue",
    ),
    (0x0029_0018, audio_on_delayed_sound, "Audio_OnDelayedSound"),
    (
        0x0029_d6e0,
        ssx_audio_system_on_mission_end,
        "SSXAudioSystem_OnMissionEnd",
    ),
];

/// Starts the character's victory tune in the final heat: unless a replay
/// plays, the radio mode is the mountain or the heat ranking is 3 or worse, it maps the
/// ranked character (jump table of ten characters) to a track and loads, tags and starts
/// the music with the `CharTune` bank. `void (mgr)`.
/// `SSXAudioSystem_PlayCharacterTune` (0x0028cdf8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_play_character_tune(c: &mut Cpu) {
    if c.pc != 0x0028cdf8 {
        return ee::s07::f_0028cdf8__generated(c);
    }
    guest(c, |c| {
        'b_28cf78: {
            'b_28cf74: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x0028ce20);
                c.g[S5][0] = c.g[A0][0];
                Callee {
                    addr: 0x00288ae0,
                    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
                }
                .call(c, 0x0028ce20)?;
                let t = c.g[V0][0] != 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t {
                    break 'b_28cf74;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x608c));
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_28cf74;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = 0x3;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                        let t = c.g[V1][0] != c.g[A0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_28cf78;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = 0x530000;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6730));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_28cf78;
                            } else {
                                'b_28cee4: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6708));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S4][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_28cee4;
                                    } else {
                                        'b_28cee0: {
                                            'b_28ced8: {
                                                'b_28ced0: {
                                                    'b_28cec8: {
                                                        'b_28cec0: {
                                                            'b_28ceb8: {
                                                                'b_28ceb0: {
                                                                    'b_28cea8: {
                                                                        'b_28cea0: {
                                                                            c.cycles += 7;
                                                                            c.g[V0][0] = 0x480000;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2a10,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[V1][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[A0][0] as u32;
                                                                            match t {
                                                                                0x0028ce98 => {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.g[S4][0] =
                                                                                        0xa;
                                                                                    if t {
                                                                                        break 'b_28cee4;
                                                                                    } else {
                                                                                        break 'b_28cea0;
                                                                                    }
                                                                                }
                                                                                0x0028cea0 => {
                                                                                    break 'b_28cea0;
                                                                                }
                                                                                0x0028cea8 => {
                                                                                    break 'b_28cea8;
                                                                                }
                                                                                0x0028ceb0 => {
                                                                                    break 'b_28ceb0;
                                                                                }
                                                                                0x0028ceb8 => {
                                                                                    break 'b_28ceb8;
                                                                                }
                                                                                0x0028cec0 => {
                                                                                    break 'b_28cec0;
                                                                                }
                                                                                0x0028cec8 => {
                                                                                    break 'b_28cec8;
                                                                                }
                                                                                0x0028ced0 => {
                                                                                    break 'b_28ced0;
                                                                                }
                                                                                0x0028ced8 => {
                                                                                    break 'b_28ced8;
                                                                                }
                                                                                0x0028cee0 => {
                                                                                    break 'b_28cee0;
                                                                                }
                                                                                _ => {
                                                                                    c.jump_out(
                                                                                        t,
                                                                                        0x0028cdf8,
                                                                                        0x0028cf94,
                                                                                    );
                                                                                    return Ok(());
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.g[S4][0] = 0x2;
                                                                        if t {
                                                                            break 'b_28cee4;
                                                                        } else {
                                                                            break 'b_28cea8;
                                                                        }
                                                                    }
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[S4][0] = 0x8;
                                                                    if t {
                                                                        break 'b_28cee4;
                                                                    } else {
                                                                        break 'b_28ceb0;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[S4][0] = 0x1;
                                                                if t {
                                                                    break 'b_28cee4;
                                                                } else {
                                                                    break 'b_28ceb8;
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[S4][0] = 0x5;
                                                            if t {
                                                                break 'b_28cee4;
                                                            } else {
                                                                break 'b_28cec0;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[S4][0] = 0x9;
                                                        if t {
                                                            break 'b_28cee4;
                                                        } else {
                                                            break 'b_28cec8;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[S4][0] = 0x3;
                                                    if t {
                                                        break 'b_28cee4;
                                                    } else {
                                                        break 'b_28ced0;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S4][0] = 0x7;
                                                if t {
                                                    break 'b_28cee4;
                                                } else {
                                                    break 'b_28ced8;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S4][0] = 0x4;
                                            if t {
                                                break 'b_28cee4;
                                            } else {
                                                break 'b_28cee0;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S4][0] = 0x6;
                                        break 'b_28cee4;
                                    }
                                }
                                'b_28cf70: {
                                    c.cycles += 2;
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x118));
                                    if t {
                                        break 'b_28cf70;
                                    } else {
                                        c.cycles += 33;
                                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x508));
                                        c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5558));
                                        c.g[S1][0] = 0x480000;
                                        c.sw((c.g[S5][0] as u32).wrapping_add(0x62ac), c.g[V0][0]);
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a00));
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = 0x1;
                                        c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x0028cf18);
                                        c.g[S0][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[S0][0] as u32)));
                                        Callee {
                                            addr: 0x00287968,
                                            f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
                                        }
                                        .call(c, 0x0028cf18)?;
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[T0][0] = 0u64;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                        c.g[A2][0] = 0xc;
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x0028cf34);
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, t, 0x0028cf34)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x0028cf40);
                                        c.g[A1][0] = 0xc9;
                                        call_ptr(c, 0x002b49c0, 0x0028cf40)?;
                                        c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5558));
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x0028cf58);
                                        c.g[A0][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x0028cf58)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x0028cf64);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x002b4a08, 0x0028cf64)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x0028cf70);
                                        c.g[A1][0] = 0x7f;
                                        call_ptr(c, 0x002b3c28, 0x0028cf70)?;
                                        break 'b_28cf70;
                                    }
                                }
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                break 'b_28cf74;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_28cf78;
        }
        c.cycles += 7;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// Music cue handler: by the cue state in `event+0x1c` (0, 1/2, 3/4/5, 7) it
/// mutes and restarts the speech system, updates the first-time-on-track locks and
/// queues the song announcement. Returns 1 (`bool (mgr, event)`).
/// `SSXAudioSystem_OnMusicCue` (0x0028e548): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_on_music_cue(c: &mut Cpu) {
    if c.pc != 0x0028e548 {
        return ee::s07::f_0028e548__generated(c);
    }
    guest(c, |c| {
        'b_28e868: {
            'b_28e864: {
                'b_28e600: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_28e600;
                    } else {
                        c.cycles += 15;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x0028e588);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002b11b0, 0x0028e588)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x0028e598);
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x002a26f0, 0x0028e598)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0028e5a4);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x002b1758, 0x0028e5a4)?;
                        c.g[RA][0] = sx(0x0028e5ac);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002a42c8, 0x0028e5ac)?;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_28e864;
                        } else {
                            c.cycles += 14;
                            c.g[S0][0] = 0x1;
                            c.g[RA][0] = sx(0x0028e5c0);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x576c), c.g[S0][0]);
                            Callee {
                                addr: 0x0028b1e8,
                                f: ee::s07::SSXAudioSystem_GetBELibrary,
                            }
                            .call(c, 0x0028e5c0)?;
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x0028e5cc);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dd58, 0x0028e5cc)?;
                            c.g[RA][0] = sx(0x0028e5d4);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x00144bc0, 0x0028e5d4)?;
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                            c.g[RA][0] = sx(0x0028e5e0);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x002a1e20, 0x0028e5e0)?;
                            c.g[V1][0] = c.g[V0][0];
                            let t = c.g[V1][0] != c.g[S0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_28e864;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = 0x2;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x577c), c.g[V1][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5780), c.g[V0][0]);
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x57d4), 0u64);
                                if t {
                                    break 'b_28e864;
                                } else {
                                    break 'b_28e600;
                                }
                            }
                        }
                    }
                }
                'b_28e678: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
                    if t {
                        break 'b_28e678;
                    } else {
                        'b_28e654: {
                            'b_28e644: {
                                c.cycles += 8;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0028e620);
                                c.g[S0][0] = c.g[V0][0];
                                call_ptr(c, 0x002b11b0, 0x0028e620)?;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_28e644;
                                } else {
                                    c.cycles += 5;
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x0028e63c);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x002a26f0, 0x0028e63c)?;
                                    let t = true;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_28e654;
                                    } else {
                                        break 'b_28e644;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A1][0] = 0x1;
                            c.g[RA][0] = sx(0x0028e650);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x002a26f0, 0x0028e650)?;
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_28e654;
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0028e65c);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x002b1758, 0x0028e65c)?;
                        c.g[RA][0] = sx(0x0028e664);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002a42c8, 0x0028e664)?;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_28e864;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x574c), c.g[V0][0]);
                            if t {
                                break 'b_28e868;
                            } else {
                                break 'b_28e678;
                            }
                        }
                    }
                }
                'b_28e860: {
                    'b_28e7e0: {
                        'b_28e694: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_28e694;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_28e694;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_28e7e0;
                                    } else {
                                        break 'b_28e694;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = 0xffffffffffffffff;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_28e864;
                        } else {
                            'b_28e704: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
                                if t {
                                    break 'b_28e704;
                                } else {
                                    'b_28e6fc: {
                                        'b_28e6ec: {
                                            c.cycles += 8;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x0028e6c8);
                                            c.g[S0][0] = c.g[V0][0];
                                            call_ptr(c, 0x002b11b0, 0x0028e6c8)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                            c.g[V0][0] = 0x4;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            if t {
                                                break 'b_28e6ec;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x0028e6e4);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x002a26f0, 0x0028e6e4)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_28e6fc;
                                                } else {
                                                    break 'b_28e6ec;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A1][0] = 0x1;
                                        c.g[RA][0] = sx(0x0028e6f8);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, 0x002a26f0, 0x0028e6f8)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        break 'b_28e6fc;
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x0028e704);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x002b1758, 0x0028e704)?;
                                    break 'b_28e704;
                                }
                            }
                            'b_28e7d4: {
                                'b_28e7d0: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0028e70c);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x002a42c8, 0x0028e70c)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_28e7d0;
                                    } else {
                                        c.cycles += 11;
                                        c.g[RA][0] = sx(0x0028e71c);
                                        call_ptr(c, 0x0014dc80, 0x0028e71c)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x0028e728);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x0028e728)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[RA][0] = sx(0x0028e738);
                                        c.g[A2][0] = 0x1;
                                        call_ptr(c, 0x00146008, 0x0028e738)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_28e7d0;
                                        } else {
                                            c.cycles += 15;
                                            c.g[RA][0] = sx(0x0028e748);
                                            Callee {
                                                addr: 0x0028b1e8,
                                                f: ee::s07::SSXAudioSystem_GetBELibrary,
                                            }
                                            .call(c, 0x0028e748)?;
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x0028e754);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0014dd58, 0x0028e754)?;
                                            c.g[RA][0] = sx(0x0028e75c);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x0028e75c)?;
                                            c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x0028e76c);
                                            c.g[A1][0] = c.g[S0][0];
                                            call_ptr(c, 0x002a1e20, 0x0028e76c)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if c.g[A0][0] != c.g[V0][0] {
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x5774),
                                                    c.g[V0][0],
                                                );
                                                break 'b_28e7d4;
                                            } else {
                                                'b_28e7cc: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        sx((c.g[S1][0] as u32)
                                                            .wrapping_add(0x579c));
                                                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                    c.g[S4][0] = c.g[V0][0];
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[S2][0] = c.lw((c.g[V0][0] as u32));
                                                    let t = c.g[S2][0] != c.g[A0][0];
                                                    c.g[S5][0] = c.g[V1][0];
                                                    if t {
                                                        break 'b_28e7cc;
                                                    } else {
                                                        'b_28e7c8: {
                                                            'b_28e7c4: {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[RA][0] = sx(0x0028e7a4);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x002a40e0, 0x0028e7a4,
                                                                )?;
                                                                if c.g[V0][0] != 0u64 {
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x5770),
                                                                        c.g[S2][0],
                                                                    );
                                                                    break 'b_28e7c4;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0028e7b8);
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    call_ptr(
                                                                        c, 0x002a4158, 0x0028e7b8,
                                                                    )?;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[V0][0] = sx((c.g[S4][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[S5][0] as u32),
                                                                        ));
                                                                    if t {
                                                                        break 'b_28e7c8;
                                                                    } else {
                                                                        c.cycles += 1;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x5770,
                                                                                ),
                                                                            c.g[S2][0],
                                                                        );
                                                                        break 'b_28e7c4;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 1;
                                                            c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                                .wrapping_add((c.g[S5][0] as u32)));
                                                            break 'b_28e7c8;
                                                        }
                                                        c.cycles += 1;
                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                        break 'b_28e7cc;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = 0x1;
                                                break 'b_28e7d0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5774), c.g[V0][0]);
                                break 'b_28e7d4;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x5778), c.g[V1][0]);
                            if t {
                                break 'b_28e860;
                            } else {
                                break 'b_28e7e0;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_28e864;
                    } else {
                        c.cycles += 29;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x0028e7f8);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002b11b0, 0x0028e7f8)?;
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0028e808);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x002a26f0, 0x0028e808)?;
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x0028e814);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002b1758, 0x0028e814)?;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5774), c.g[V0][0]);
                        c.g[RA][0] = sx(0x0028e828);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5784), c.g[V0][0]);
                        call_ptr(c, 0x002b4908, 0x0028e828)?;
                        c.g[RA][0] = sx(0x0028e830);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5778), c.g[V0][0]);
                        Callee {
                            addr: 0x0028b1e8,
                            f: ee::s07::SSXAudioSystem_GetBELibrary,
                        }
                        .call(c, 0x0028e830)?;
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x0028e83c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x0028e83c)?;
                        c.g[RA][0] = sx(0x0028e844);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00144bc0, 0x0028e844)?;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[RA][0] = sx(0x0028e850);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002a1e20, 0x0028e850)?;
                        c.g[V1][0] = 0x2;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x57dc), 0u64);
                            break 'b_28e860;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x57d8), 0u64);
                            break 'b_28e860;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_28e864;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_28e868;
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
        c.pc = t;
        return Ok(());
    });
}

/// Pushes a delayed sound request (`req` in a1) into the voice arrays of the audio
/// object (`gp+0x574`), clamps the volume byte and plays the pushed sound. Returns 1.
/// `Audio_OnDelayedSound` (0x00290018): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn audio_on_delayed_sound(c: &mut Cpu) {
    if c.pc != 0x00290018 {
        return ee::s07::f_00290018__generated(c);
    }
    guest(c, |c| {
        'b_29031c: {
            'b_2902f0: {
                c.cycles += 176;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[A2][0] = 0x8;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S3][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S4][0] = 0x18;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                c.g[S5][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                c.g[S6][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.mult(0, 3, 3, 20);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[V1][0] as u32));
                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                c.mult(0, 4, 4, 20);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[RA][0] = sx(0x00290120);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                call_ptr(c, 0x00416210, 0x00290120)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[A0][0] = 0x64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = 0x5a;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A2][0] = 0x32;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A3][0] = 0x7f;
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[T0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[T0][0]);
                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[S5][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[S6][0]);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                c.mult(0, 4, 4, 20);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                if t {
                    break 'b_2902f0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_29031c;
                    } else {
                        break 'b_2902f0;
                    }
                }
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
            if c.g[V1][0] == 0u64 {
                c.g[V0][0] = 0x7f;
                break 'b_29031c;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                break 'b_29031c;
            }
        }
        c.cycles += 14;
        c.sb((c.g[A0][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x00290328);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x574));
        Callee {
            addr: 0x002906b8,
            f: ee::s07::Audio_PlayPushedSound,
        }
        .call(c, 0x00290328)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// Mission end: stops the wipeout and trick sounds, ends the loops, then picks the
/// big-challenge music by the mission audio type. `void (mgr, mission)`.
/// `SSXAudioSystem_OnMissionEnd` (0x0029d6e0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ssx_audio_system_on_mission_end(c: &mut Cpu) {
    if c.pc != 0x0029d6e0 {
        return ee::s08::f_0029d6e0__generated(c);
    }
    guest(c, |c| {
        'b_29d778: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0029d70c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd8), 0u64);
            call_ptr(c, 0x0028b1d8, 0x0029d70c)?;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[RA][0] = sx(0x0029d718);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x00296e20,
                f: ee::s08::SSXAudioSystem_StopWipeoutLoop,
            }
            .call(c, 0x0029d718)?;
            c.g[RA][0] = sx(0x0029d720);
            call_ptr(c, 0x0028b1d8, 0x0029d720)?;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[RA][0] = sx(0x0029d72c);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x00297438,
                f: ee::s08::SSXAudioSystem_StopWipeoutLoop2,
            }
            .call(c, 0x0029d72c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0029d738);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x0029b3c0,
                f: ee::s08::SSXAudioSystem_StopTrickMeterSound,
            }
            .call(c, 0x0029d738)?;
            c.g[RA][0] = sx(0x0029d740);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x002b46e8,
                f: ee::s08::MusicPlayer_AreLoopsActive,
            }
            .call(c, 0x0029d740)?;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                break 'b_29d778;
            } else {
                'b_29d76c: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0029d750);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0028d960, 0x0029d750)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x8;
                    if t {
                        break 'b_29d76c;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5558));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0029d76c);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0029d76c)?;
                        break 'b_29d76c;
                    }
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0029d774);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x002b4708,
                    f: ee::s08::MusicPlayer_ClearLoopsActive,
                }
                .call(c, 0x0029d774)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                break 'b_29d778;
            }
        }
        'b_29d8c4: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] == c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_29d8c4;
            } else {
                'b_29d824: {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x0029d790);
                    c.g[S1][0] = c.g[A0][0];
                    Callee {
                        addr: 0x002b49e0,
                        f: ee::s08::MusicPlayer_GetSongTag,
                    }
                    .call(c, 0x0029d790)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        break 'b_29d824;
                    } else {
                        c.cycles += 33;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x364f));
                        c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3648));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[RA][0] = sx(0x0029d7b8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        Callee {
                            addr: 0x002adde0,
                            f: ee::s08::BankMonitor_KillTimersByCallback,
                        }
                        .call(c, 0x0029d7b8)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                        c.g[V0][0] = 0x4a0000;
                        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x3657));
                        c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3650));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[RA][0] = sx(0x0029d7d4);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        Callee {
                            addr: 0x002adde0,
                            f: ee::s08::BankMonitor_KillTimersByCallback,
                        }
                        .call(c, 0x0029d7d4)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                        c.g[V0][0] = 0x4a0000;
                        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x365f));
                        c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x3658));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[RA][0] = sx(0x0029d7f0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        Callee {
                            addr: 0x002adde0,
                            f: ee::s08::BankMonitor_KillTimersByCallback,
                        }
                        .call(c, 0x0029d7f0)?;
                        c.g[A1][0] = 0x12;
                        c.g[RA][0] = sx(0x0029d7fc);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0028def0, 0x0029d7fc)?;
                        c.g[RA][0] = sx(0x0029d804);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0028d488, 0x0029d804)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0029d81c);
                        c.g[T0][0] = 0u64;
                        call_ptr(c, 0x0028cf98, 0x0029d81c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        if t {
                            break 'b_29d8c4;
                        } else {
                            break 'b_29d824;
                        }
                    }
                }
                'b_29d88c: {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x0029d82c);
                    c.g[S2][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x0014dc80, 0x0029d82c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0029d838);
                    c.g[A1][0] = 0xa;
                    call_ptr(c, 0x0014dd58, 0x0029d838)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0029d844);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x001547a0, 0x0029d844)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd0), c.g[V1][0]);
                    if t {
                        break 'b_29d88c;
                    } else {
                        'b_29d888: {
                            'b_29d880: {
                                'b_29d878: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2de0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x0029d870 => {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S2][0] = 0x21;
                                            if t {
                                                break 'b_29d88c;
                                            } else {
                                                break 'b_29d878;
                                            }
                                        }
                                        0x0029d878 => {
                                            break 'b_29d878;
                                        }
                                        0x0029d880 => {
                                            break 'b_29d880;
                                        }
                                        0x0029d888 => {
                                            break 'b_29d888;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x0029d6e0, 0x0029d8dc);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[S2][0] = 0x22;
                                if t {
                                    break 'b_29d88c;
                                } else {
                                    break 'b_29d880;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[S2][0] = 0x26;
                            if t {
                                break 'b_29d88c;
                            } else {
                                break 'b_29d888;
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = 0xffffffffffffffff;
                        break 'b_29d88c;
                    }
                }
                'b_29d8b8: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0029d894);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x002b4620,
                        f: ee::s08::MusicPlayer_AttachLoopBank,
                    }
                    .call(c, 0x0029d894)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'b_29d8b8;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5558));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0029d8b0);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0029d8b0)?;
                        c.g[V1][0] = 0x1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd4), c.g[V1][0]);
                        break 'b_29d8b8;
                    }
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0029d8c0);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x002b4708,
                    f: ee::s08::MusicPlayer_ClearLoopsActive,
                }
                .call(c, 0x0029d8c0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_29d8c4;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, fresh, run_generated, DATA, GAME_GP};

    /// Gate of the `oc1027_fake_*` callees (other tests' machines fall back).
    pub(crate) const OC1027_GATE: u32 = DATA + 0xf_0000;
    pub(crate) const OC1027_MAGIC: u32 = 0x1027_0c0e;
    /// Mailbox: the value (v0) fake number `id` returns, at `RES + 4 * id`.
    pub(crate) const OC1027_RES: u32 = DATA + 0xf_0100;

    macro_rules! oc1027_fakes {
        ($(($name:ident, $id:expr, $slot:path, $gen:path)),* $(,)?) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1027_GATE) != OC1027_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    let (target, ra) = (c.pc, c.g[RA][0] as u32);
                    fakes::fake(c, target, ra);
                    let v = c.bus.read32(OC1027_RES + 4 * $id);
                    c.g[V0][0] = sx(v);
                }
            )*

            /// Hooks the fakes into the callees' override slots (once per
            /// process; with the gate clear they run what the slots held).
            pub(crate) fn oc1027_install_fakes() {
                let slots: &[(*mut fn(&mut Cpu), fn(&mut Cpu))] = &[
                    $((std::ptr::addr_of_mut!($slot), $name as fn(&mut Cpu)),)*
                ];
                for &(slot, fake) in slots {
                    fakes::hook(slot, fake, fakes::prev_cell(fake));
                }
            }
        };
    }

    pub(crate) const F_REPLAY: u32 = 0;
    pub(crate) const F_GAIN: u32 = 1;
    pub(crate) const F_B49C0: u32 = 2;
    pub(crate) const F_B4A08: u32 = 3;
    pub(crate) const F_B3C28: u32 = 4;
    pub(crate) const F_B11B0: u32 = 5;
    pub(crate) const F_A26F0: u32 = 6;
    pub(crate) const F_B1758: u32 = 7;
    pub(crate) const F_A42C8: u32 = 8;
    pub(crate) const F_GETBELIB: u32 = 9;
    pub(crate) const F_14DD58: u32 = 10;
    pub(crate) const F_GETLOC: u32 = 11;
    pub(crate) const F_A1E20: u32 = 12;
    pub(crate) const F_14DC80: u32 = 13;
    pub(crate) const F_LOCKED: u32 = 14;
    pub(crate) const F_A40E0: u32 = 15;
    pub(crate) const F_A4158: u32 = 16;
    pub(crate) const F_B4908: u32 = 17;
    pub(crate) const F_MEMSET: u32 = 18;
    pub(crate) const F_PUSHED: u32 = 19;
    pub(crate) const F_RACEMAN: u32 = 20;
    pub(crate) const F_STOPWIPE: u32 = 21;
    pub(crate) const F_STOPWIPE2: u32 = 22;
    pub(crate) const F_TRICKMETER: u32 = 23;
    pub(crate) const F_LOOPS: u32 = 24;
    pub(crate) const F_D960: u32 = 25;
    pub(crate) const F_CLEARLOOPS: u32 = 26;
    pub(crate) const F_SONGTAG: u32 = 27;
    pub(crate) const F_KILLTIMERS: u32 = 28;
    pub(crate) const F_DEF0: u32 = 29;
    pub(crate) const F_D488: u32 = 30;
    pub(crate) const F_CF98: u32 = 31;
    pub(crate) const F_MISSIONTYPE: u32 = 32;
    pub(crate) const F_ATTACH: u32 = 33;

    oc1027_fakes! {
        (oc1027_fake_replay, F_REPLAY, ee::s07::OVERRIDE_00288AE0, ee::s07::f_00288ae0__generated),
        (oc1027_fake_gain, F_GAIN, ee::s07::OVERRIDE_00287968, ee::s07::f_00287968__generated),
        (oc1027_fake_b49c0, F_B49C0, ee::s08::OVERRIDE_002B49C0, ee::s08::f_002b49c0__generated),
        (oc1027_fake_b4a08, F_B4A08, ee::s08::OVERRIDE_002B4A08, ee::s08::f_002b4a08__generated),
        (oc1027_fake_b3c28, F_B3C28, ee::s08::OVERRIDE_002B3C28, ee::s08::f_002b3c28__generated),
        (oc1027_fake_b11b0, F_B11B0, ee::s08::OVERRIDE_002B11B0, ee::s08::f_002b11b0__generated),
        (oc1027_fake_a26f0, F_A26F0, ee::s08::OVERRIDE_002A26F0, ee::s08::f_002a26f0__generated),
        (oc1027_fake_b1758, F_B1758, ee::s08::OVERRIDE_002B1758, ee::s08::f_002b1758__generated),
        (oc1027_fake_a42c8, F_A42C8, ee::s08::OVERRIDE_002A42C8, ee::s08::f_002a42c8__generated),
        (oc1027_fake_getbelib, F_GETBELIB, ee::s07::OVERRIDE_0028B1E8, ee::s07::f_0028b1e8__generated),
        (oc1027_fake_14dd58, F_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated),
        (oc1027_fake_getloc, F_GETLOC, ee::s01::OVERRIDE_00144BC0, ee::s01::f_00144bc0__generated),
        (oc1027_fake_a1e20, F_A1E20, ee::s08::OVERRIDE_002A1E20, ee::s08::f_002a1e20__generated),
        (oc1027_fake_14dc80, F_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated),
        (oc1027_fake_locked, F_LOCKED, ee::s01::OVERRIDE_00146008, ee::s01::f_00146008__generated),
        (oc1027_fake_a40e0, F_A40E0, ee::s08::OVERRIDE_002A40E0, ee::s08::f_002a40e0__generated),
        (oc1027_fake_a4158, F_A4158, ee::s08::OVERRIDE_002A4158, ee::s08::f_002a4158__generated),
        (oc1027_fake_b4908, F_B4908, ee::s08::OVERRIDE_002B4908, ee::s08::f_002b4908__generated),
        (oc1027_fake_memset, F_MEMSET, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated),
        (oc1027_fake_pushed, F_PUSHED, ee::s07::OVERRIDE_002906B8, ee::s07::f_002906b8__generated),
        (oc1027_fake_raceman, F_RACEMAN, ee::s07::OVERRIDE_0028B1D8, ee::s07::f_0028b1d8__generated),
        (oc1027_fake_stopwipe, F_STOPWIPE, ee::s08::OVERRIDE_00296E20, ee::s08::f_00296e20__generated),
        (oc1027_fake_stopwipe2, F_STOPWIPE2, ee::s08::OVERRIDE_00297438, ee::s08::f_00297438__generated),
        (oc1027_fake_trickmeter, F_TRICKMETER, ee::s08::OVERRIDE_0029B3C0, ee::s08::f_0029b3c0__generated),
        (oc1027_fake_loops, F_LOOPS, ee::s08::OVERRIDE_002B46E8, ee::s08::f_002b46e8__generated),
        (oc1027_fake_d960, F_D960, ee::s07::OVERRIDE_0028D960, ee::s07::f_0028d960__generated),
        (oc1027_fake_clearloops, F_CLEARLOOPS, ee::s08::OVERRIDE_002B4708, ee::s08::f_002b4708__generated),
        (oc1027_fake_songtag, F_SONGTAG, ee::s08::OVERRIDE_002B49E0, ee::s08::f_002b49e0__generated),
        (oc1027_fake_killtimers, F_KILLTIMERS, ee::s08::OVERRIDE_002ADDE0, ee::s08::f_002adde0__generated),
        (oc1027_fake_def0, F_DEF0, ee::s07::OVERRIDE_0028DEF0, ee::s07::f_0028def0__generated),
        (oc1027_fake_d488, F_D488, ee::s07::OVERRIDE_0028D488, ee::s07::f_0028d488__generated),
        (oc1027_fake_cf98, F_CF98, ee::s07::OVERRIDE_0028CF98, ee::s07::f_0028cf98__generated),
        (oc1027_fake_missiontype, F_MISSIONTYPE, ee::s01::OVERRIDE_001547A0, ee::s01::f_001547a0__generated),
        (oc1027_fake_attach, F_ATTACH, ee::s08::OVERRIDE_002B4620, ee::s08::f_002b4620__generated),
    }

    /// Open the gate, clear the mailbox.
    fn oc1027_arm(c: &mut Cpu) {
        oc1027_install_fakes();
        c.bus.write32(OC1027_GATE, OC1027_MAGIC);
        for i in 0..40 {
            c.bus.write32(OC1027_RES + 4 * i, 0);
        }
    }

    fn oc1027_ret(c: &mut Cpu, id: u32, v: u32) {
        c.bus.write32(OC1027_RES + 4 * id, v);
    }

    /// The (return address, a1) of every logged fake call: a path signature.
    fn oc1027_sig(c: &mut Cpu) -> Vec<(u32, u64)> {
        let n = c.bus.read32(fakes::LOG_COUNT);
        (0..n)
            .map(|i| {
                let at = fakes::LOG_START + 0x220 * i + 0x10;
                (
                    c.bus.read64(at + 16 * RA as u32) as u32,
                    c.bus.read64(at + 16 * A1 as u32),
                )
            })
            .collect()
    }

    /// Distinct call signatures of the generated body over the seeds (no unwinds).
    fn oc1027_paths(genf: fn(&mut Cpu), seeds: u32, fixture: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            fixture(&mut c, seed);
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
            run_generated(&mut c, genf);
            seen.insert(oc1027_sig(&mut c));
        }
        seen.len()
    }

    // ------------------------------------------------ PlayCharacterTune

    /// `PlayCharacterTune(mgr)`: replay off, not mountain radio mode, the
    /// final heat, ranking under 3 and a known character (the jump table of
    /// ten characters) start the character tune.
    fn oc1027_pct_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1027_0001, 8);
        elf_image(c);
        oc1027_arm(c);
        let mgr = DATA + 0x2_0000;
        let (s3, world, race) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
        oc1027_ret(c, F_REPLAY, (r.next_u32() % 7 == 0) as u32);
        c.bus.write32(
            mgr + 0x608c,
            if r.next_u32() % 9 == 0 {
                2
            } else {
                r.next_u32() & 1
            },
        );
        c.bus.write32(mgr + 0x508, r.next_u32());
        c.bus.write32(mgr + 0x5558, s3);
        c.bus.write16(s3 + 0x10, (r.next_u32() & 0xf0) as u16);
        c.bus.write32(s3 + 0x14, fakes::FAKE);
        c.bus.write16(s3 + 0x20, (r.next_u32() & 0xf0) as u16);
        c.bus.write32(s3 + 0x24, fakes::FAKE);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0xc0, race);
        c.bus
            .write32(race, if r.next_u32() % 11 == 0 { 1 } else { 3 });
        c.bus.write32(0x0053_6730, r.next_u32() % 4);
        let idx = r.next_u32() % 4;
        c.bus.write32(0x0053_6708, idx);
        c.bus.write32(race + 0x18 + 4 * idx, r.next_u32() % 12);
        c.g[A0][0] = sx(mgr);
        c.pc = 0x0028_cdf8;
    }

    /// `PlayCharacterTune`: every callee is a gated fake (results from the
    /// mailbox), jump table of the characters from the game image.
    #[test]
    fn ssx_audio_system_play_character_tune_matches() {
        fakes::covered(
            ee::s07::f_0028cdf8__generated,
            ssx_audio_system_play_character_tune,
            300,
            &oc1027_pct_fixture,
        );
    }

    /// The fixture reaches the early exits and several characters' tracks.
    #[test]
    fn ssx_audio_system_play_character_tune_paths() {
        let n = oc1027_paths(ee::s07::f_0028cdf8__generated, 300, &oc1027_pct_fixture);
        assert!(n >= 8, "only {n} paths");
    }

    // ------------------------------------------------ OnMusicCue

    /// `OnMusicCue(mgr, event)`: the cue state in `event+0x1c` (0, 1/2, 3/4/5,
    /// 7) with the announce number in `+0x20`.
    fn oc1027_omc_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1027_0002, 12);
        elf_image(c);
        oc1027_arm(c);
        let mgr = DATA + 0x2_0000;
        let (ev, loc) = (DATA + 0x1000, DATA + 0x5000);
        fill_words(c, &mut r, mgr + 0x5740, 0x28);
        for i in 0..12 {
            c.bus.write32(mgr + 0x579c + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(ev + 0x1c, r.next_u32() % 9);
        c.bus.write32(
            ev + 0x20,
            [0xffff_ffff, 5, 0x1234][(r.next_u32() % 3) as usize],
        );
        c.bus.write32(loc, r.next_u32() % 11);
        oc1027_ret(c, F_A42C8, r.next_u32() % 2);
        oc1027_ret(c, F_A1E20, r.next_u32() % 4);
        oc1027_ret(c, F_LOCKED, r.next_u32() % 2);
        oc1027_ret(c, F_A40E0, r.next_u32() % 2);
        oc1027_ret(c, F_A4158, r.next_u32() % 2);
        oc1027_ret(c, F_B4908, r.next_u32());
        oc1027_ret(c, F_GETBELIB, DATA + 0x6000);
        oc1027_ret(c, F_14DD58, DATA + 0x6100);
        oc1027_ret(c, F_GETLOC, loc);
        oc1027_ret(c, F_14DC80, DATA + 0x6200);
        c.g[A0][0] = sx(mgr);
        c.g[A1][0] = sx(ev);
        c.pc = 0x0028_e548;
    }

    fn fill_words(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32, n: u32) {
        for i in 0..n {
            c.bus.write32(at + 4 * i, r.next_u32());
        }
    }

    /// `OnMusicCue`: all callees are gated fakes with mailbox results.
    #[test]
    fn ssx_audio_system_on_music_cue_matches() {
        fakes::covered(
            ee::s07::f_0028e548__generated,
            ssx_audio_system_on_music_cue,
            400,
            &oc1027_omc_fixture,
        );
    }

    /// Every event state and the lock branches are reached.
    #[test]
    fn ssx_audio_system_on_music_cue_paths() {
        let n = oc1027_paths(ee::s07::f_0028e548__generated, 400, &oc1027_omc_fixture);
        assert!(n >= 12, "only {n} paths");
    }

    // ------------------------------------------------ OnDelayedSound

    /// `OnDelayedSound(_, req)`: pushes a delayed sound's fields into the
    /// audio object's voice arrays (pointer table at `obj+0x1d8`), clamps the
    /// volume byte, then plays the pushed sound.
    fn oc1027_ods_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1027_0003, 4);
        elf_image(c);
        oc1027_arm(c);
        let (req, cell, obj) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x4000);
        for off in [0x18, 0x1c, 0x20, 0x24] {
            c.bus.write32(req + off, r.next_u32());
        }
        c.bus.write32(GAME_GP + 0x574, cell);
        c.bus.write32(cell, obj);
        c.bus.write32(cell + 4, DATA + 0x3000);
        c.bus.write32(cell + 8, DATA + 0x3400);
        fill_words(c, &mut r, obj, 0xc0);
        // cnt (obj+0x1f4) becomes 1..=3, so the record/array writes stay clear of the pointers.
        c.bus.write32(obj + 0x1f4, r.next_u32() % 3);
        let s1 = obj + 0x1d8;
        for (k, off) in [
            0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(s1 + off, DATA + 0x5000 + 0x400 * k as u32);
        }
        // The byte of the first copied record word (obj+0x1dc): -5, 50, 127, -128.
        let b = [0xfbu32, 0x32, 0x7f, 0x80][(r.next_u32() % 4) as usize];
        c.bus.write32(obj + 0x1dc, b | (r.next_u32() << 8));
        c.g[A1][0] = sx(req);
        c.pc = 0x0029_0018;
    }

    /// `OnDelayedSound`: memset and `PlayPushedSound` are gated fakes; the
    /// volume byte takes the negative, in-range and saturated branches.
    #[test]
    fn audio_on_delayed_sound_matches() {
        fakes::covered(
            ee::s07::f_00290018__generated,
            audio_on_delayed_sound,
            240,
            &oc1027_ods_fixture,
        );
    }

    // ------------------------------------------------ OnMissionEnd

    /// `OnMissionEnd(mgr, mission)`: stops the wipeout and trick sounds, ends
    /// the loops, then picks the big-challenge music by mission audio type.
    fn oc1027_ome_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1027_0004, 14);
        elf_image(c);
        oc1027_arm(c);
        let mgr = DATA + 0x2_0000;
        let (sobj, pt, pt2, race) = (DATA + 0x1000, DATA + 0x3000, DATA + 0x3100, DATA + 0x4000);
        c.bus.write32(mgr + 0x5558, sobj);
        c.bus.write16(sobj + 0x18, (r.next_u32() & 0xf0) as u16);
        c.bus.write32(sobj + 0x1c, fakes::FAKE);
        c.bus.write16(sobj + 0x20, (r.next_u32() & 0xf0) as u16);
        c.bus.write32(sobj + 0x24, fakes::FAKE);
        c.bus.write32(mgr + 0x118, pt);
        c.bus.write32(pt, pt2);
        c.bus.write32(
            mgr + 0x608c,
            if r.next_u32() % 4 == 0 {
                2
            } else {
                r.next_u32() & 1
            },
        );
        fill_words(c, &mut r, mgr + 0x5fd0, 4);
        oc1027_ret(c, F_RACEMAN, race);
        oc1027_ret(c, F_LOOPS, r.next_u32() % 2);
        oc1027_ret(c, F_D960, r.next_u32() % 2);
        oc1027_ret(c, F_SONGTAG, r.next_u32() % 6);
        oc1027_ret(c, F_MISSIONTYPE, r.next_u32() % 8);
        oc1027_ret(c, F_ATTACH, r.next_u32() % 2);
        oc1027_ret(c, F_14DC80, DATA + 0x6200);
        oc1027_ret(c, F_14DD58, DATA + 0x6100);
        c.g[A0][0] = sx(mgr);
        c.g[A1][0] = sx(r.next_u32());
        c.pc = 0x0029_d6e0;
    }

    /// `OnMissionEnd`: all callees are gated fakes with mailbox results; the
    /// song tag, loop and mission-type results pick every arm.
    #[test]
    fn ssx_audio_system_on_mission_end_matches() {
        fakes::covered(
            ee::s08::f_0029d6e0__generated,
            ssx_audio_system_on_mission_end,
            400,
            &oc1027_ome_fixture,
        );
    }

    /// Several call sequences are taken over the seeds.
    #[test]
    fn ssx_audio_system_on_mission_end_paths() {
        let n = oc1027_paths(ee::s08::f_0029d6e0__generated, 400, &oc1027_ome_fixture);
        assert!(n >= 8, "only {n} paths");
    }
}
