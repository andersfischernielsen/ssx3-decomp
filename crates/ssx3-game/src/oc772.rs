//! Staging module oc772 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc772`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_bf78,
        ssx_audio_system_set_music_mode,
        "SSXAudioSystem_SetMusicMode",
    ),
    (
        0x0029_89a8,
        ssx_audio_system_play_ground_wall_hit_sound,
        "SSXAudioSystem_PlayGroundWallHitSound",
    ),
    (
        0x0019_f878,
        fe_module_rider_preview_find_texture,
        "FEModule_RiderPreviewFindTexture",
    ),
    (
        0x0018_6728,
        cfe_state_event_select_transition_in,
        "cFEStateEventSelect_TransitionIn",
    ),
    (
        0x0020_0388,
        event_select_get_event_record,
        "EventSelect_GetEventRecord",
    ),
    (
        0x0028_f768,
        ssx_audio_system_play_loading_screen_loop,
        "SSXAudioSystem_PlayLoadingScreenLoop",
    ),
];

/// `void cFEStateEventSelect_TransitionIn(cFEStateEventSelect* self)`
/// (0x186728): hands the event-select state (`self + 0x48`) to
/// `EventSelect_TransitionIn`. The argument is an add, not a load.
pub fn cfe_state_event_select_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
        call_ptr(c, 0x0020_0a70, 0x0018_6738)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int FEModule_RiderPreviewFindTexture(RiderPreview* self, char* name)`
/// (0x19f878): the id (`+0x1c + 0xc*i`) of the first of the `self+0x18`
/// entries whose name (`+0x20 + 0xc*i`) equals `name` (`strcmp`), else -1.
pub fn fe_module_rider_preview_find_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S4][0] = c.g[A1][0];
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[S1][0] = sx(r32(c, S3).wrapping_add(0x1c));
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(0x20));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_65a8, 0x0019_f8bc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    found = true;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_f8b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_SetMusicMode(SSXAudioSystem* self, int mode)` (0x28bf78): switches the radio mode (0 default map, 1 and 3 radio playlist, 2 off). Stores the mode at +0x608c (no-op when unchanged), applies the music-player playlist mask, copies the BE option block to the stack to read the DJ-speech flag (bit 17) for `SetMusicChannelEnabled`, and, when in game with a song playing and the player state allows it, starts a new random song with `PlayMusic`.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn ssx_audio_system_set_music_mode(c: &mut Cpu) {
    'b_28c2bc: {
        'b_28c2b8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2d0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
            let t = c.g[V0][0] == c.g[A1][0];
            c.g[V0][0] = 0x1;
            if t {
                break 'b_28c2b8;
            } else {
                'b_28c264: {
                    'b_28c23c: {
                        'b_28c0fc: {
                            'b_28c0f8: {
                                c.cycles += 2;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x608c), c.g[A1][0]);
                                if t {
                                    break 'b_28c0f8;
                                } else {
                                    'b_28bfd8: {
                                        'b_28bfc0: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V0][0] = 0x2;
                                                break 'b_28bfc0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x118));
                                                if t {
                                                    break 'b_28bfd8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c0),
                                                    );
                                                    if t {
                                                        break 'b_28c2bc;
                                                    } else {
                                                        break 'b_28bfc0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x3;
                                        if t {
                                            break 'b_28c264;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A1][0] == c.g[V0][0];
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x118));
                                            if t {
                                                break 'b_28c0fc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                if t {
                                                    break 'b_28c2bc;
                                                } else {
                                                    break 'b_28bfd8;
                                                }
                                            }
                                        }
                                    }
                                    'b_28c098: {
                                        'b_28c068: {
                                            c.cycles += 14;
                                            c.g[A1][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x518));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x0028bfe8);
                                            c.g[S2][0] = c.g[V0][0];
                                            c.pc = 0x002b4070;
                                            call_indirect(c, 0x002b4070);
                                            if c.pc != 0x0028bfe8 {
                                                return;
                                            }
                                            c.g[RA][0] = sx(0x0028bff0);
                                            c.pc = 0x0014dc80;
                                            call_indirect(c, 0x0014dc80);
                                            if c.pc != 0x0028bff0 {
                                                return;
                                            }
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x0028bffc);
                                            c.g[A1][0] = 0x4;
                                            c.pc = 0x0014dd58;
                                            call_indirect(c, 0x0014dd58);
                                            if c.pc != 0x0028bffc {
                                                return;
                                            }
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                            c.g[V0][0] = c.g[V1][0] & 0x7;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = c.g[SP][0];
                                            if t {
                                                break 'b_28c068;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                                'l_28c014: loop {
                                                    c.cycles += 19;
                                                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(5, (c.g[V1][0] as u32));
                                                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.ldl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.ldl(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.ldr(
                                                        8,
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(5, (c.g[A0][0] as u32));
                                                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.sdl(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x17),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.sdl(
                                                        8,
                                                        (c.g[A0][0] as u32).wrapping_add(0x1f),
                                                    );
                                                    c.sdr(
                                                        8,
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if c.cycles >= c.next_event {
                                                            c.pc = 0x0028c014;
                                                            c.event();
                                                        }
                                                        continue 'l_28c014;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_28c098;
                                                        } else {
                                                            break 'b_28c068;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                        'l_28c06c: loop {
                                            c.cycles += 11;
                                            c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                                            c.g[A2][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.g[A3][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.g[T0][0] =
                                                c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x10),
                                                c.g[A3][0],
                                            );
                                            c.sd(
                                                (c.g[A0][0] as u32).wrapping_add(0x18),
                                                c.g[T0][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                            if t {
                                                if c.cycles >= c.next_event {
                                                    c.pc = 0x0028c06c;
                                                    c.event();
                                                }
                                                continue 'l_28c06c;
                                            } else {
                                                break 'b_28c098;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, (c.g[V1][0] as u32));
                                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.sdr(2, (c.g[A0][0] as u32));
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                                    c.g[RA][0] = sx(0x0028c0bc);
                                    c.g[A1][0] = c.g[A1][0] & 0x1;
                                    c.pc = 0x00287558;
                                    call_indirect(c, 0x00287558);
                                    if c.pc != 0x0028c0bc {
                                        return;
                                    }
                                    c.g[RA][0] = sx(0x0028c0c4);
                                    c.pc = 0x0028b1b0;
                                    call_indirect(c, 0x0028b1b0);
                                    if c.pc != 0x0028c0c4 {
                                        return;
                                    }
                                    if c.g[V0][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        break 'b_28c2bc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                            break 'b_28c2bc;
                                        } else {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x0028c0e0);
                                            c.g[A0][0] = c.g[S2][0];
                                            c.pc = 0x002b49e0;
                                            call_indirect(c, 0x002b49e0);
                                            if c.pc != 0x0028c0e0 {
                                                return;
                                            }
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65));
                                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                break 'b_28c2bc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_28c23c;
                                                } else {
                                                    break 'b_28c0f8;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                            break 'b_28c0fc;
                        }
                        'b_28c1bc: {
                            'b_28c18c: {
                                c.cycles += 14;
                                c.g[A1][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0028c10c);
                                c.g[S2][0] = c.g[V0][0];
                                c.pc = 0x002b4070;
                                call_indirect(c, 0x002b4070);
                                if c.pc != 0x0028c10c {
                                    return;
                                }
                                c.g[RA][0] = sx(0x0028c114);
                                c.pc = 0x0014dc80;
                                call_indirect(c, 0x0014dc80);
                                if c.pc != 0x0028c114 {
                                    return;
                                }
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0028c120);
                                c.g[A1][0] = 0x4;
                                c.pc = 0x0014dd58;
                                call_indirect(c, 0x0014dd58);
                                if c.pc != 0x0028c120 {
                                    return;
                                }
                                c.g[V1][0] = 0x530000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                                c.g[V0][0] = c.g[V1][0] & 0x7;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[SP][0];
                                if t {
                                    break 'b_28c18c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                                    'l_28c138: loop {
                                        c.cycles += 19;
                                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                                        c.ldr(8, (c.g[V1][0] as u32));
                                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
                                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x17));
                                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                        c.sdr(8, (c.g[A0][0] as u32));
                                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                        if t {
                                            if c.cycles >= c.next_event {
                                                c.pc = 0x0028c138;
                                                c.event();
                                            }
                                            continue 'l_28c138;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_28c1bc;
                                            } else {
                                                break 'b_28c18c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
                            'l_28c190: loop {
                                c.cycles += 11;
                                c.g[T0][0] = c.ld((c.g[V1][0] as u32));
                                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sd((c.g[A0][0] as u32), c.g[T0][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                if t {
                                    if c.cycles >= c.next_event {
                                        c.pc = 0x0028c190;
                                        c.event();
                                    }
                                    continue 'l_28c190;
                                } else {
                                    break 'b_28c1bc;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(8, (c.g[V1][0] as u32));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(8, (c.g[A0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                        c.g[RA][0] = sx(0x0028c1e0);
                        c.g[A1][0] = c.g[A1][0] & 0x1;
                        c.pc = 0x00287558;
                        call_indirect(c, 0x00287558);
                        if c.pc != 0x0028c1e0 {
                            return;
                        }
                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                            break 'b_28c2bc;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0028c1f4);
                            c.pc = 0x0028b1b0;
                            call_indirect(c, 0x0028b1b0);
                            if c.pc != 0x0028c1f4 {
                                return;
                            }
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                break 'b_28c2bc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                                if c.g[V0][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                    break 'b_28c2bc;
                                } else {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x0028c210);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.pc = 0x002b49e0;
                                    call_indirect(c, 0x002b49e0);
                                    if c.pc != 0x0028c210 {
                                        return;
                                    }
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                        break 'b_28c2bc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0028c22c);
                                        c.g[A0][0] = c.g[S2][0];
                                        c.pc = 0x002b4af0;
                                        call_indirect(c, 0x002b4af0);
                                        if c.pc != 0x0028c22c {
                                            return;
                                        }
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x12d;
                                        if t {
                                            break 'b_28c23c;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S1][0] != c.g[V0][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                                                break 'b_28c2bc;
                                            } else {
                                                break 'b_28c23c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0028c244);
                    c.g[A0][0] = c.g[S0][0];
                    c.pc = 0x0028d488;
                    call_indirect(c, 0x0028d488);
                    if c.pc != 0x0028c244 {
                        return;
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0028c25c);
                    c.g[T0][0] = 0u64;
                    c.pc = 0x0028cf98;
                    call_indirect(c, 0x0028cf98);
                    if c.pc != 0x0028c25c {
                        return;
                    }
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    if t {
                        break 'b_28c2bc;
                    } else {
                        break 'b_28c264;
                    }
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0028c270);
                c.g[A1][0] = 0u64;
                c.pc = 0x00287558;
                call_indirect(c, 0x00287558);
                if c.pc != 0x0028c270 {
                    return;
                }
                c.g[RA][0] = sx(0x0028c278);
                c.pc = 0x0028b1b0;
                call_indirect(c, 0x0028b1b0);
                if c.pc != 0x0028c278 {
                    return;
                }
                if c.g[V0][0] == 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                    break 'b_28c2bc;
                } else {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0028c288);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.pc = 0x002b49e0;
                    call_indirect(c, 0x002b49e0);
                    if c.pc != 0x0028c288 {
                        return;
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65));
                    c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                        break 'b_28c2bc;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0028c2b0);
                        c.g[T0][0] = 0u64;
                        c.pc = 0x0028cf98;
                        call_indirect(c, 0x0028cf98);
                        if c.pc != 0x0028c2b0 {
                            return;
                        }
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd0), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5fd4), 0u64);
                        break 'b_28c2b8;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        break 'b_28c2bc;
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2d0));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_PlayGroundWallHitSound(SSXAudioSystem* self, Rider* rider)` (0x2989a8): for a human, local rider (+0x874 set, +0x87c > 0) that has no wall-hit sound tracked yet (`IsTrackedSoundPlaying(self, 6, 0)`), pushes a wall-hit voice (id 0x36, type 5) onto the bank manager's tables, sets its volume channel, radius vanish and listener, plays it and tracks it (`TrackSound(self, 6, handle, counter + 0x32, 0, 0)`).
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn ssx_audio_system_play_ground_wall_hit_sound(c: &mut Cpu) {
    'b_2989e0: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x874));
        let t = c.g[V0][0] == 0u64;
        c.g[S3][0] = c.g[A0][0];
        if t {
            break 'b_2989e0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x87c));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            break 'b_2989e0;
        }
    }
    'b_298ce0: {
        'b_298cdc: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = c.g[S3][0];
            if t {
                break 'b_298cdc;
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0x6;
                c.g[RA][0] = sx(0x002989f4);
                c.g[A2][0] = 0u64;
                c.pc = 0x00295028;
                call_indirect(c, 0x00295028);
                if c.pc != 0x002989f4 {
                    return;
                }
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'b_298ce0;
                } else {
                    c.cycles += 184;
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32));
                    c.g[T1][0] = 0x18;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[S2][0] = 0x36;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 9);
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
                    c.mult(0, 4, 4, 9);
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
                    c.g[RA][0] = sx(0x00298ad4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.pc = 0x00416210;
                    call_indirect(c, 0x00416210);
                    if c.pc != 0x00298ad4 {
                        return;
                    }
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A3][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[T0][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[T1][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[T2][0] = 0x7f;
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[T3][0] = 0x1;
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = 0x5;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[T2][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[T3][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                    c.g[RA][0] = sx(0x00298c30);
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.pc = 0x00287968;
                    call_indirect(c, 0x00287968);
                    if c.pc != 0x00298c30 {
                        return;
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[AT][0] = 0x42c80000;
                    c.mtc1(1, 12);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 13);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x00298c68);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    c.pc = 0x002a9988;
                    call_indirect(c, 0x002a9988);
                    if c.pc != 0x00298c68 {
                        return;
                    }
                    c.g[A1][0] = c.g[S4][0];
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x00298c78);
                    c.g[A0][0] = c.g[S3][0];
                    c.pc = 0x00288a20;
                    call_indirect(c, 0x00288a20);
                    if c.pc != 0x00298c78 {
                        return;
                    }
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x110));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x23c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x00298cb4);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A3][0]);
                    c.pc = 0x002906b8;
                    call_indirect(c, 0x002906b8);
                    if c.pc != 0x00298cb4 {
                        return;
                    }
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0x6;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0u64;
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x00298cdc);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x32));
                    c.pc = 0x00295628;
                    call_indirect(c, 0x00295628);
                    if c.pc != 0x00298cdc {
                        return;
                    }
                    break 'b_298cdc;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        break 'b_298ce0;
    }
    c.cycles += 7;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

/// `void* EventSelect_GetEventRecord(void* dst, int peak, int goal, int kind)` (0x200388): copies the 0x6c-byte event record `[peak][goal]` of the race (kind 0, stride 0x1b0 per peak), freestyle (1, 0x21c) or freeride (2, 0x360) location table into `dst` (any other kind copies race record [0][0]) and returns `dst`. Word-copy loop when source and destination are 8-aligned, `ldl/ldr` copy otherwise.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn event_select_get_event_record(c: &mut Cpu) {
    'b_2006b0: {
        'b_2005fc: {
            'b_2005e0: {
                'b_200454: {
                    c.cycles += 3;
                    c.g[T1][0] = c.g[A0][0];
                    let t = c.g[A3][0] != 0u64;
                    c.g[V1][0] = c.g[A2][0];
                    if t {
                        break 'b_200454;
                    } else {
                        'b_20041c: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x1b0;
                            c.g[V0][0] = 0x6c;
                            c.mult(0, 2, 3, 2);
                            c.mult(0, 4, 5, 4);
                            c.g[V1][0] = 0x480000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A3][0] = c.g[T1][0];
                            if t {
                                break 'b_20041c;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                'l_2003c8: loop {
                                    c.cycles += 19;
                                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(3, (c.g[A0][0] as u32));
                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                    c.sdr(3, (c.g[A3][0] as u32));
                                    c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                    c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if c.cycles >= c.next_event {
                                            c.pc = 0x002003c8;
                                            c.event();
                                        }
                                        continue 'l_2003c8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_2005e0;
                                        } else {
                                            break 'b_20041c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                        'l_200420: loop {
                            c.cycles += 11;
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            if t {
                                if c.cycles >= c.next_event {
                                    c.pc = 0x00200420;
                                    c.event();
                                }
                                continue 'l_200420;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_2005e0;
                                } else {
                                    break 'b_200454;
                                }
                            }
                        }
                    }
                }
                'b_200520: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_200520;
                    } else {
                        'b_2004e8: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x21c;
                            c.g[V0][0] = 0x6c;
                            c.mult(0, 2, 3, 2);
                            c.mult(0, 4, 5, 4);
                            c.g[V1][0] = 0x480000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A3][0] = c.g[T1][0];
                            if t {
                                break 'b_2004e8;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                'l_200494: loop {
                                    c.cycles += 19;
                                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(3, (c.g[A0][0] as u32));
                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                    c.sdr(3, (c.g[A3][0] as u32));
                                    c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                    c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if c.cycles >= c.next_event {
                                            c.pc = 0x00200494;
                                            c.event();
                                        }
                                        continue 'l_200494;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_2005e0;
                                        } else {
                                            break 'b_2004e8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                        'l_2004ec: loop {
                            c.cycles += 11;
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            if t {
                                if c.cycles >= c.next_event {
                                    c.pc = 0x002004ec;
                                    c.event();
                                }
                                continue 'l_2004ec;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_2005e0;
                                } else {
                                    break 'b_200520;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != c.g[V0][0] {
                    c.g[V1][0] = 0x480000;
                    break 'b_2005fc;
                } else {
                    'b_2005b0: {
                        c.cycles += 12;
                        c.g[A0][0] = 0x360;
                        c.g[V0][0] = 0x6c;
                        c.mult(0, 2, 3, 2);
                        c.mult(0, 4, 5, 4);
                        c.g[V1][0] = 0x480000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72c8));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A3][0] = c.g[T1][0];
                        if t {
                            break 'b_2005b0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                            'l_20055c: loop {
                                c.cycles += 19;
                                c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[A0][0] as u32));
                                c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[A3][0] as u32));
                                c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                if t {
                                    if c.cycles >= c.next_event {
                                        c.pc = 0x0020055c;
                                        c.event();
                                    }
                                    continue 'l_20055c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_2005e0;
                                    } else {
                                        break 'b_2005b0;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    'l_2005b4: loop {
                        c.cycles += 11;
                        c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                        c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if c.cycles >= c.next_event {
                                c.pc = 0x002005b4;
                                c.event();
                            }
                            continue 'l_2005b4;
                        } else {
                            break 'b_2005e0;
                        }
                    }
                }
            }
            c.cycles += 7;
            c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[A3][0] as u32));
            let t = true;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            if t {
                break 'b_2006b0;
            } else {
                break 'b_2005fc;
            }
        }
        'b_200698: {
            'b_200668: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
                c.g[V0][0] = c.g[T1][0] | c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[T1][0];
                if t {
                    break 'b_200668;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    'l_200614: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(7, (c.g[V1][0] as u32));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[A0][0] as u32));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if c.cycles >= c.next_event {
                                c.pc = 0x00200614;
                                c.event();
                            }
                            continue 'l_200614;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_200698;
                            } else {
                                break 'b_200668;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            'l_20066c: loop {
                c.cycles += 11;
                c.g[A3][0] = c.ld((c.g[V1][0] as u32));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A0][0] as u32), c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0020066c;
                        c.event();
                    }
                    continue 'l_20066c;
                } else {
                    break 'b_200698;
                }
            }
        }
        c.cycles += 6;
        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[V1][0] as u32));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.sdr(7, (c.g[A0][0] as u32));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        break 'b_2006b0;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[T1][0];
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_PlayLoadingScreenLoop(SSXAudioSystem* self)` (0x28f768): pushes the loading-screen loop voice (id 0xd) onto the bank manager's tables, clamps its base volume to 127, sets the volume channel and stores the pushed-sound handle at +0x62a8.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn ssx_audio_system_play_loading_screen_loop(c: &mut Cpu) {
    'b_28fa38: {
        'b_28fa0c: {
            c.cycles += 163;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[A1][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = 0x8;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = 0x18;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0xd;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S4][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.lw((c.g[S4][0] as u32));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 18);
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
            c.mult(0, 4, 4, 18);
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
            c.g[RA][0] = sx(0x0028f860);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210;
            call_indirect(c, 0x00416210);
            if c.pc != 0x0028f860 {
                return;
            }
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
            c.g[A0][0] = c.lw((c.g[S4][0] as u32));
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S3][0]);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32));
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
            c.mult(0, 4, 4, 18);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if t {
                break 'b_28fa0c;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_28fa38;
                } else {
                    break 'b_28fa0c;
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
            break 'b_28fa38;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
            break 'b_28fa38;
        }
    }
    c.cycles += 23;
    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[A1][0] = 0x1;
    c.g[A0][0] = c.g[S4][0];
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x0028fa50);
    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
    c.pc = 0x00287968;
    call_indirect(c, 0x00287968);
    if c.pc != 0x0028fa50 {
        return;
    }
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = c.g[S4][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[RA][0] = sx(0x0028fa70);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.pc = 0x002906b8;
    call_indirect(c, 0x002906b8);
    if c.pc != 0x0028fa70 {
        return;
    }
    c.sw((c.g[S4][0] as u32).wrapping_add(0x62a8), c.g[V0][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::{check, check_lohi};
    use crate::game::sound::bankmanager::tests::update_peak_music_setup;
    use crate::game::sound::soundsys::tests::oc749_fake_body;
    use crate::game::sound::soundsys::tests::oc749_gate;
    use crate::game::sound::soundsys::tests::oc749_install;
    use crate::game::sound::soundsys::tests::oc749_mgr;
    use crate::game::sound::soundsys::tests::oc749_res;
    use crate::game::sound::soundsys::tests::oc749_unwind;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (the machine's own RAM, away
    /// from the pool and the fakes' call log at DATA+0x100000). Results and
    /// the unwind index come from the `oc749` control block.
    const OC772_CTRL: u32 = DATA + 0x22_0000;
    const OC772_MAGIC: u32 = 0x0772_0772;

    macro_rules! oc772_fakes {
        ($($fake:ident = $addr:expr, $shard:ident :: $slot:ident / $gen:ident;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC772_CTRL) != OC772_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    if oc749_unwind(c) {
                        return;
                    }
                    oc749_fake_body(c, $addr);
                    ret(c);
                }
            )*
            fn oc772_install() {
                oc749_install();
                $(
                    fakes::hook(
                        addr_of_mut!(ee::$shard::$slot),
                        $fake,
                        fakes::prev_cell($fake),
                    );
                )*
            }
        };
    }

    oc772_fakes! {
        oc772_fake_287558 = 0x287558, s07::OVERRIDE_00287558 / f_00287558__generated;
        oc772_fake_28b1b0 = 0x28b1b0, s07::OVERRIDE_0028B1B0 / f_0028b1b0__generated;
        oc772_fake_2b4070 = 0x2b4070, s08::OVERRIDE_002B4070 / f_002b4070__generated;
        oc772_fake_2b49e0 = 0x2b49e0, s08::OVERRIDE_002B49E0 / f_002b49e0__generated;
        oc772_fake_2b4af0 = 0x2b4af0, s08::OVERRIDE_002B4AF0 / f_002b4af0__generated;
        oc772_fake_295028 = 0x295028, s08::OVERRIDE_00295028 / f_00295028__generated;
        oc772_fake_295628 = 0x295628, s08::OVERRIDE_00295628 / f_00295628__generated;
    }

    /// Both gates on (and maybe one fake unwinds).
    fn oc772_gate(c: &mut Cpu, r: &mut Rng) {
        oc749_gate(c, r);
        c.bus.write32(OC772_CTRL, OC772_MAGIC);
    }

    /// `cFEStateEventSelect_TransitionIn`: forwards `self + 0x48` to
    /// `EventSelect_TransitionIn`. The callee is real and has no fake slot
    /// yet, so a1 is 0 (its short path: nothing to do, returns 0).
    #[test]
    fn cfe_state_event_select_transition_in_matches() {
        check(
            ee::s02::f_00186728__generated,
            cfe_state_event_select_transition_in,
            200,
            &|c, seed| {
                let mut r = Rng(0x0018_6728 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA + 0x100 * (1 + seed % 7));
                c.g[A1][0] = 0;
                c.pc = 0x0018_6728;
            },
        );
    }

    fn put_name(c: &mut Cpu, at: u32, name: &[u8]) {
        for i in 0..8u32 {
            let b = name.get(i as usize).copied().unwrap_or(0);
            c.bus.write8(at + i, b);
        }
    }

    /// `FEModule_RiderPreviewFindTexture`: 0 to 5 preview entries (also a
    /// negative count), with the wanted name at entry k, last, or absent.
    #[test]
    fn fe_module_rider_preview_find_texture_matches() {
        check(
            ee::s03::f_0019f878__generated,
            fe_module_rider_preview_find_texture,
            300,
            &|c, seed| {
                let mut r = Rng(0x0019_f878 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                let name = DATA + 0x1000;
                let count = [0u32, 1, 2, 3, 5, 0xffff_fffe, 4][seed as usize % 7];
                c.bus.write32(this + 0x18, count);
                let names: [&[u8]; 5] = [b"rider0", b"rider1", b"abc", b"abd", b"rider12"];
                for i in 0..5u32 {
                    c.bus.write32(this + 0x1c + 0xc * i, 0x100 + 7 * i);
                    put_name(c, this + 0x20 + 0xc * i, names[i as usize]);
                }
                let want: &[u8] = match (seed / 7) % 6 {
                    0 => b"rider0",
                    1 => b"rider1",
                    2 => b"abc",
                    3 => b"abd",
                    4 => b"rider12",
                    _ => b"nope",
                };
                put_name(c, name, want);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }

    /// `EventSelect_GetEventRecord`: all four kinds (race, freestyle,
    /// freeride, anything else), peaks and goals that make the source 8-byte
    /// aligned or not, destinations of every alignment.
    #[test]
    fn event_select_get_event_record_matches() {
        check_lohi(
            ee::s05::f_00200388__generated,
            event_select_get_event_record,
            400,
            &|c, seed| {
                let mut r = Rng(0x0020_0388 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                fill(c, &mut r, 0x0047_8000, 0x880);
                let dst = DATA + 0x100 + [0u32, 8, 4, 1, 3, 12, 16, 5][seed as usize / 6 % 8];
                let kind = [0u32, 1, 2, 3, 0xffff_ffff, 7][seed as usize % 6];
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = sx(kind);
            },
        );
    }

    /// A sound manager fixture shared by the voice-pushing functions: the
    /// audio system at `this` (`+0`/`+4`/`+8` the manager and its tables,
    /// `+0x118` the music player whose first word is a second manager).
    fn oc772_voice_system(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let this = DATA + 0x100;
        let p = DATA + 0x400;
        let mgr = DATA + 0x1000;
        let q = DATA + 0x2000;
        c.g[A0][0] = sx(this);
        c.bus.write32(this, mgr);
        c.bus.write32(this + 4, DATA + 0x3000);
        c.bus.write32(this + 8, DATA + 0x3100);
        c.bus.write32(this + 0x118, p);
        c.bus.write32(p, q);
        oc749_mgr(c, r, mgr, DATA + 0x10000, seed % 2);
        oc749_mgr(c, r, q, DATA + 0x14000, (seed / 2) % 3);
        this
    }

    /// `SSXAudioSystem_PlayGroundWallHitSound`: a human local rider or not
    /// (+0x874, +0x87c), the wall-hit sound already tracked or not, and
    /// fakes that unwind at some call.
    #[test]
    fn ssx_audio_system_play_ground_wall_hit_sound_matches() {
        oc772_install();
        check_lohi(
            ee::s08::f_002989a8__generated,
            ssx_audio_system_play_ground_wall_hit_sound,
            300,
            &|c, seed| {
                let mut r = Rng(0x0029_89a8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc772_gate(c, &mut r);
                oc772_voice_system(c, &mut r, seed);
                let rider = DATA + 0x500;
                c.g[A1][0] = sx(rider);
                c.bus.write32(rider + 0x110, r.next_u32());
                c.bus.write32(rider + 0x874, (seed % 5 != 0) as u32);
                c.bus.write32(
                    rider + 0x87c,
                    [0, 1, 5, 0xffff_fff0][(seed as usize / 3) % 4],
                );
                oc749_res(c, 0x295028, (seed % 3 == 0) as u32);
                oc749_res(c, 0x287968, DATA + 0x3200);
                oc749_res(c, 0x288a20, r.next_u32());
                oc749_res(c, 0x2906b8, r.next_u32());
                oc749_res(c, 0x295628, r.next_u32());
            },
        );
    }

    /// `SSXAudioSystem_PlayLoadingScreenLoop`: pushes the loop voice (the
    /// volume clamp always sees the 127 it just stored), with fakes that
    /// unwind at some call.
    #[test]
    fn ssx_audio_system_play_loading_screen_loop_matches() {
        oc772_install();
        check_lohi(
            ee::s07::f_0028f768__generated,
            ssx_audio_system_play_loading_screen_loop,
            300,
            &|c, seed| {
                let mut r = Rng(0x0028_f768 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc772_gate(c, &mut r);
                oc772_voice_system(c, &mut r, seed);
                oc749_res(c, 0x287968, DATA + 0x3200);
                oc749_res(c, 0x2906b8, r.next_u32());
            },
        );
    }

    /// The audio system for `SetMusicMode`: the cached mode (+0x608c), the
    /// radio playlist (+0x6098), the default map (+0x518), the song-playing
    /// flag (+0x530), and what the music-player fakes report.
    #[allow(clippy::too_many_arguments)]
    fn set_music_mode_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        this: u32,
        mode: u32,
        cached: u32,
        in_game: bool,
        playing: bool,
        playlist: bool,
        state: u32,
        busy: u32,
    ) {
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(mode);
        c.bus.write32(this + 0x608c, cached);
        c.bus.write64(
            this + 0x6098,
            if playlist { DATA as u64 + 0xe000 } else { 0 },
        );
        c.bus.write64(this + 0x518, DATA as u64 + 0xe100);
        c.bus.write32(this + 0x530, playing as u32);
        c.bus.write32(this + 0x5fd0, r.next_u32());
        c.bus.write32(this + 0x5fd4, r.next_u32());
        // The BE option block the function copies to its stack.
        fill(c, r, 0x0053_5610, 0xa2);
        oc749_res(c, 0x28b1b0, in_game as u32);
        oc749_res(c, 0x2b49e0, state);
        oc749_res(c, 0x2b4af0, busy);
        oc749_res(c, 0x14dc80, DATA + 0x500);
        oc749_res(c, 0x14dd58, DATA + 0x510);
    }

    /// Would the mode change reach `SelectRandomSong` / `PlayMusic`?
    fn set_music_mode_plays(
        mode: u32,
        in_game: bool,
        playing: bool,
        playlist: bool,
        state: u32,
        busy: u32,
    ) -> bool {
        match mode {
            0 => in_game && playing && state.wrapping_sub(101) < 3,
            1 | 3 => {
                playlist
                    && in_game
                    && playing
                    && state.wrapping_sub(1) >= 3
                    && (busy == 0 || state == 0x12d)
            }
            2 => in_game && state.wrapping_sub(101) >= 3,
            _ => false,
        }
    }

    const MUSIC_STATES: [u32; 13] = [0, 1, 2, 3, 4, 100, 101, 102, 103, 104, 300, 301, 302];

    /// `SSXAudioSystem_SetMusicMode`: every mode (also an unknown one and the
    /// unchanged-mode early exit), in game or not, song playing or not, and
    /// the music-player states the checks look at. The two real callees
    /// without an override slot (`SelectRandomSong`, `PlayMusic`) are kept
    /// off the path here (see the next test).
    #[test]
    fn ssx_audio_system_set_music_mode_matches() {
        oc772_install();
        check(
            ee::s07::f_0028bf78__generated,
            ssx_audio_system_set_music_mode,
            400,
            &|c, seed| {
                let mut r = Rng(0x0028_bf78 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc772_gate(c, &mut r);
                let mode = [0xffff_ffffu32, 0, 1, 2, 3, 4, 7][seed as usize % 7];
                let cached = if seed % 5 == 0 {
                    mode
                } else {
                    mode.wrapping_add(1 + r.next_u32() % 3)
                };
                let mut in_game = r.next_u32() % 4 != 0;
                let playing = r.next_u32() % 3 != 0;
                let playlist = r.next_u32() % 3 != 0;
                let state = MUSIC_STATES[r.next_u32() as usize % 13];
                let busy = r.next_u32() % 2;
                if cached != mode
                    && set_music_mode_plays(mode, in_game, playing, playlist, state, busy)
                {
                    in_game = false;
                }
                set_music_mode_fixture(
                    c,
                    &mut r,
                    DATA + 0x100,
                    mode,
                    cached,
                    in_game,
                    playing,
                    playlist,
                    state,
                    busy,
                );
            },
        );
    }

    /// `SSXAudioSystem_SetMusicMode` down to `SelectRandomSong` and
    /// `PlayMusic` (modes 0, 1, 2 and 3 with every condition met), on the
    /// music fixture of `SSXAudioSystem_UpdatePeakMusic` (a replay is
    /// playing, so `PlayMusic` returns at once).
    #[test]
    fn ssx_audio_system_set_music_mode_plays_matches() {
        oc772_install();
        check(
            ee::s07::f_0028bf78__generated,
            ssx_audio_system_set_music_mode,
            120,
            &|c, seed| {
                update_peak_music_setup(c, seed);
                let mut r = Rng(0x0028_bf78 ^ 0x5eed ^ (seed as u64) << 12);
                c.bus.write32(OC772_CTRL, OC772_MAGIC);
                let mode = [0u32, 1, 2, 3][seed as usize % 4];
                let (state, busy) = match mode {
                    0 => (101 + seed % 3, 0),
                    2 => (seed % 3, 0),
                    _ if seed % 8 < 4 => (5 + seed % 4, 0),
                    _ => (0x12d, 1),
                };
                set_music_mode_fixture(
                    c,
                    &mut r,
                    DATA,
                    mode,
                    mode.wrapping_add(2),
                    true,
                    true,
                    true,
                    state,
                    busy,
                );
                c.pc = 0x0028_bf78;
            },
        );
    }
}
