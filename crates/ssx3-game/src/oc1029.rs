//! Staging module oc1029 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1029`.

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
        0x0028_c430,
        ssx_audio_system_play_song_segment_callback,
        "SSXAudioSystem_PlaySongSegmentCallback",
    ),
    (
        0x0029_1438,
        ssx_audio_system_restart_handle_sound,
        "SSXAudioSystem_RestartHandleSound",
    ),
    (
        0x0029_5cc8,
        ssx_audio_system_stop_tracked_sounds,
        "SSXAudioSystem_StopTrackedSounds",
    ),
    (
        0x0029_97b8,
        ssx_audio_system_play_meter_full_sound,
        "SSXAudioSystem_PlayMeterFullSound",
    ),
    (
        0x0029_b0e0,
        ssx_audio_system_start_trick_meter_sound,
        "SSXAudioSystem_StartTrickMeterSound",
    ),
];

/// `int SSXAudioSystem_PlaySongSegmentCallback(SSXAudioSystem* audio, Cue* cue)`:
/// the music cue callback. Advances the cue's segment index (wrapping it into
/// `0..count`), pushes the segment's sound onto the bank-monitor ring (the
/// 0x18-byte record the other `Audio_*` helpers push too: kind, data pointer,
/// volume scaled by 0.01 and clamped to 0..127), points the bank monitor's
/// volume control at `audio+0x60a0`, calls the song-instance update and
/// `Audio_PlayPushedSound`. Returns 0 (also when the segment count is <= 0).
pub fn ssx_audio_system_play_song_segment_callback(c: &mut Cpu) {
    guest(c, |c| {
        'b_28c884: {
            'b_28c4a0: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S3][0] = c.g[A0][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S4][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x20));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[S5][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                if t {
                    break 'b_28c4a0;
                } else {
                    c.cycles += 2;
                    let t = (c.g[A0][0] as i64) <= 0;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x20), 0u64);
                    if t {
                        break 'b_28c884;
                    } else {
                        break 'b_28c4a0;
                    }
                }
            }
            'b_28c4dc: {
                c.cycles += 7;
                c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                c.mtc1(2, 20);
                c.cvt_s_w(20, 20);
                c.g[S2][0] = c.lw((c.g[S3][0] as u32));
                c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x60a0));
                let t = (c.g[S1][0] as i64) >= 0;
                c.g[FP][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
                if t {
                    break 'b_28c4dc;
                } else {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    'l_28c4c0: loop {
                        c.cycles += 7;
                        if (c.g[S1][0] as i64) < 0 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            if event_due(c) {
                                deliver_event_here(c, 0x0028c4c0);
                            }
                            continue 'l_28c4c0;
                        } else {
                            break 'b_28c4dc;
                        }
                    }
                }
            }
            'b_28c7e4: {
                'b_28c7e0: {
                    'b_28c790: {
                        c.cycles += 162;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[T1][0] = 0x18;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1d8));
                        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[A1][0] = 0u64;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                        c.g[A2][0] = 0x8;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        c.mult(0, 3, 3, 9);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.ldl(4, (c.g[S2][0] as u32).wrapping_add(0x1e3));
                        c.ldr(4, (c.g[S2][0] as u32).wrapping_add(0x1dc));
                        c.ldl(7, (c.g[S2][0] as u32).wrapping_add(0x1eb));
                        c.ldr(7, (c.g[S2][0] as u32).wrapping_add(0x1e4));
                        c.ldl(8, (c.g[S2][0] as u32).wrapping_add(0x1f3));
                        c.ldr(8, (c.g[S2][0] as u32).wrapping_add(0x1ec));
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[V1][0] as u32));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                        c.mult(0, 4, 4, 9);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[RA][0] = sx(0x0028c5b0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x00416210, 0x0028c5b0)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = 0x64;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                        c.g[A1][0] = 0x5a;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A2][0] = 0x32;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A3][0] = 0x7f;
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[T0][0] = 0x1;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x45d8));
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                        c.mul_s(2, 20, 0);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[AT][0] = 0x42fe0000;
                        c.mtc1(1, 3);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[S5][0]);
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[S6][0]);
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        if t {
                            break 'b_28c790;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                            c.mtc1(0, 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.lwc1(0, (c.g[V0][0] as u32));
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.c_le_s(1, 0);
                            let t = !c.fcc();
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_28c7e0;
                            } else {
                                break 'b_28c790;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.lwc1(0, (c.g[V0][0] as u32));
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x7f;
                    if t {
                        break 'b_28c7e0;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        let t = true;
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        if t {
                            break 'b_28c7e4;
                        } else {
                            break 'b_28c7e0;
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                break 'b_28c7e4;
            }
            'b_28c858: {
                'b_28c82c: {
                    c.cycles += 12;
                    c.g[V0][0] = 0x18;
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if t {
                        break 'b_28c82c;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        let t = (c.g[V1][0] as i64) < 0;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_28c858;
                        } else {
                            break 'b_28c82c;
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
                    break 'b_28c858;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                    break 'b_28c858;
                }
            }
            c.cycles += 11;
            c.sb((c.g[A0][0] as u32), c.g[V0][0]);
            c.g[A1][0] = c.g[FP][0];
            c.g[RA][0] = sx(0x0028c868);
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x520));
            call_ptr(c, 0x002b2850, 0x0028c868)?;
            c.g[RA][0] = sx(0x0028c870);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x002906b8,
                f: ee::s07::Audio_PlayPushedSound,
            }
            .call(c, 0x0028c870)?;
            c.g[V1][0] = c.lw((c.g[S7][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S7][0] as u32), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            break 'b_28c884;
        }
        c.cycles += 14;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_RestartHandleSound(SSXAudioSystem* audio)`: stops the
/// tracked voice at `audio+0x5fcc` unless it is busy (`0x3b8ab8` under the
/// `0x3b58a0`/`0x3b58d8` lock pair), then pushes sound 8 (pan 0x10, kind 5)
/// onto the ring and stores the handle `Audio_PlayPushedSound` returns.
pub fn ssx_audio_system_restart_handle_sound(c: &mut Cpu) {
    guest(c, |c| {
        'b_291494: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0029145c);
            c.g[S4][0] = c.g[A0][0];
            call_ptr(c, 0x003b58a0, 0x0029145c)?;
            c.g[RA][0] = sx(0x00291464);
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5fcc));
            call_ptr(c, 0x003b8ab8, 0x00291464)?;
            c.g[RA][0] = sx(0x0029146c);
            c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
            call_ptr(c, 0x003b58d8, 0x0029146c)?;
            if c.g[S0][0] != 0u64 {
                c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                break 'b_291494;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[A2][0] = 0x1;
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5fcc));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.mtc1(0, 12);
                c.g[RA][0] = sx(0x00291490);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                call_ptr(c, 0x002ad5f0, 0x00291490)?;
                c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                break 'b_291494;
            }
        }
        c.cycles += 158;
        c.g[T1][0] = 0x18;
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[S2][0] = 0x8;
        c.g[S3][0] = 0x10;
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
        c.g[RA][0] = sx(0x0029156c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x0029156c)?;
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
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x5;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = 0u64;
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
        c.g[V1][0] = c.lw((c.g[S4][0] as u32));
        c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S2][0]);
        c.g[V1][0] = c.lw((c.g[S4][0] as u32));
        c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S3][0]);
        c.g[RA][0] = sx(0x002916c8);
        c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
        Callee {
            addr: 0x00287968,
            f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
        }
        .call(c, 0x002916c8)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[RA][0] = sx(0x002916e8);
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        Callee {
            addr: 0x002906b8,
            f: ee::s07::Audio_PlayPushedSound,
        }
        .call(c, 0x002916e8)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x5fcc), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_StopTrackedSounds(SSXAudioSystem* audio, int event)`:
/// walks the five-word entries of the pending-sound ring (`audio+0x597c`:
/// capacity, buffer, head, tail) once. Entries of `event` are dropped (their
/// handle is stopped through `BankMonitor_StopHandle` when the fourth word is
/// set); the others are pushed back in order.
pub fn ssx_audio_system_stop_tracked_sounds(c: &mut Cpu) {
    guest(c, |c| {
        'b_295d04: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S3][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            if c.g[V0][0] == 0u64 {
                break 'b_295d04;
            } else {
                break 'b_295d04;
            }
        }
        'b_296068: {
            c.cycles += 4;
            c.div(0, 3, 2);
            c.g[V1][0] = c.hi[0];
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[S2][0] = c.g[V1][0];
            if t {
                break 'b_296068;
            } else {
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x597c));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                'l_295d20: loop {
                    'b_295d70: {
                        'b_295d38: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                            c.g[A0][0] = c.g[V1][0];
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_295d38;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A2][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_295d70;
                                } else {
                                    break 'b_295d38;
                                }
                            }
                        }
                        'b_295d64: {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), c.g[A1][0]);
                            if t {
                                break 'b_295d64;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), 0u64);
                                break 'b_295d64;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                        break 'b_295d70;
                    }
                    'b_295db4: {
                        'b_295d80: {
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_295d80;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A3][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_295db4;
                                } else {
                                    break 'b_295d80;
                                }
                            }
                        }
                        'b_295da8: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), c.g[A1][0]);
                            if t {
                                break 'b_295da8;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), 0u64);
                                break 'b_295da8;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                        break 'b_295db4;
                    }
                    'b_295df8: {
                        'b_295dc4: {
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_295dc4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[T1][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_295df8;
                                } else {
                                    break 'b_295dc4;
                                }
                            }
                        }
                        'b_295dec: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), c.g[A1][0]);
                            if t {
                                break 'b_295dec;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), 0u64);
                                break 'b_295dec;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                        c.g[T1][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                        break 'b_295df8;
                    }
                    'b_295e3c: {
                        'b_295e08: {
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_295e08;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[T0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_295e3c;
                                } else {
                                    break 'b_295e08;
                                }
                            }
                        }
                        'b_295e30: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), c.g[A1][0]);
                            if t {
                                break 'b_295e30;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), 0u64);
                                break 'b_295e30;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                        c.g[T0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                        break 'b_295e3c;
                    }
                    'b_295e78: {
                        'b_295e4c: {
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[A1][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_295e4c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[A1][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_295e78;
                                } else {
                                    break 'b_295e4c;
                                }
                            }
                        }
                        'b_295e74: {
                            c.cycles += 9;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), c.g[A1][0]);
                            if t {
                                break 'b_295e74;
                            } else {
                                c.cycles += 1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5988), 0u64);
                                break 'b_295e74;
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_295e78;
                    }
                    'b_296060: {
                        'b_29605c: {
                            'b_295ea8: {
                                c.cycles += 2;
                                if c.g[S3][0] != c.g[A2][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295ea8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T0][0] == 0u64;
                                    c.g[A1][0] = c.g[A3][0];
                                    if t {
                                        break 'b_29605c;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                        c.mtc1(0, 12);
                                        c.g[A2][0] = 0x1;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                        c.g[RA][0] = sx(0x00295ea0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                        Callee {
                                            addr: 0x002ad5f0,
                                            f: ee::s08::BankMonitor_StopHandle,
                                        }
                                        .call(c, 0x00295ea0)?;
                                        let t = true;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_296060;
                                        } else {
                                            break 'b_295ea8;
                                        }
                                    }
                                }
                            }
                            'b_295ebc: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_295ebc;
                                } else {
                                    break 'b_295ebc;
                                }
                            }
                            'b_295f00: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295f00;
                                } else {
                                    'b_295efc: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), c.g[V1][0]);
                                        if t {
                                            break 'b_295efc;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), 0u64);
                                            break 'b_295efc;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295f00;
                                }
                            }
                            'b_295f14: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_295f14;
                                } else {
                                    break 'b_295f14;
                                }
                            }
                            'b_295f58: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295f58;
                                } else {
                                    'b_295f54: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), c.g[V1][0]);
                                        if t {
                                            break 'b_295f54;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), 0u64);
                                            break 'b_295f54;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295f58;
                                }
                            }
                            'b_295f6c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_295f6c;
                                } else {
                                    break 'b_295f6c;
                                }
                            }
                            'b_295fb0: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295fb0;
                                } else {
                                    'b_295fac: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T1][0]);
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), c.g[V1][0]);
                                        if t {
                                            break 'b_295fac;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), 0u64);
                                            break 'b_295fac;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_295fb0;
                                }
                            }
                            'b_295fc4: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_295fc4;
                                } else {
                                    break 'b_295fc4;
                                }
                            }
                            'b_296008: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                                c.g[A0][0] = c.hi[0];
                                if c.g[A0][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_296008;
                                } else {
                                    'b_296004: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), c.g[V1][0]);
                                        if t {
                                            break 'b_296004;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), 0u64);
                                            break 'b_296004;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                    break 'b_296008;
                                }
                            }
                            'b_29601c: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.div(0, 2, 3);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_29601c;
                                } else {
                                    break 'b_29601c;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5988));
                            c.g[A0][0] = c.hi[0];
                            if c.g[A0][0] == c.g[V0][0] {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                break 'b_296060;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x597c));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), c.g[V1][0]);
                                if t {
                                    break 'b_29605c;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5984), 0u64);
                                    break 'b_29605c;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        break 'b_296060;
                    }
                    c.cycles += 2;
                    if c.g[S2][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5984));
                        if event_due(c) {
                            deliver_event_here(c, 0x00295d20);
                        }
                        continue 'l_295d20;
                    } else {
                        break 'b_296068;
                    }
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
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_PlayMeterFullSound(SSXAudioSystem* audio, Rider* rider)`:
/// the "trick meter full" sting. Does nothing while `0x288ae0` says the audio
/// is busy or unless the rider has both state words (+0x874, +0x87c > 0).
/// Then it plays only when the rider's float at +0x470 is negative, the
/// global flag at `gp-0x6a0` (bit 0) is clear, `0x270280` of the race
/// object's +0x28 is zero and its +0x214 is 4: it pushes sound 0x69 (kind 9)
/// onto the ring and plays it.
pub fn ssx_audio_system_play_meter_full_sound(c: &mut Cpu) {
    guest(c, |c| {
        'b_299b50: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x002997e0);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x00288ae0, 0x002997e0)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_299b50;
            } else {
                'b_2997fc: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x874));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_2997fc;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x87c));
                        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                        break 'b_2997fc;
                    }
                }
                'b_299b4c: {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = 0x1;
                    if t {
                        break 'b_299b4c;
                    } else {
                        'b_299820: {
                            c.cycles += 6;
                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x470));
                            c.mtc1(0, 0);
                            c.c_le_s(0, 1);
                            let t = c.fcc();
                            c.g[S0][0] = 0x1;
                            if t {
                                break 'b_299820;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0u64;
                                break 'b_299820;
                            }
                        }
                        'b_29984c: {
                            'b_299848: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x6a0));
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                let t = c.g[V0][0] != 0u64;
                                if c.g[V1][0] != 0 {
                                    c.g[S0][0] = 0u64;
                                }
                                if t {
                                    break 'b_299848;
                                } else {
                                    c.cycles += 6;
                                    c.g[RA][0] = sx(0x00299838);
                                    call_ptr(c, 0x0028b1c8, 0x00299838)?;
                                    c.g[RA][0] = sx(0x00299840);
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                    call_ptr(c, 0x00270280, 0x00299840)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_29984c;
                                    } else {
                                        break 'b_299848;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[S0][0] = 0u64;
                            break 'b_29984c;
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x00299854);
                        call_ptr(c, 0x0028b1c8, 0x00299854)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                        c.g[V1][0] = c.g[V1][0] ^ 0x4;
                        if c.g[V1][0] != 0 {
                            c.g[S0][0] = 0u64;
                        }
                        let t = c.g[S0][0] == 0u64;
                        c.g[S2][0] = 0x18;
                        if t {
                            break 'b_299b4c;
                        } else {
                            'b_299b14: {
                                'b_299ae8: {
                                    c.cycles += 154;
                                    c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                                    c.g[A1][0] = 0u64;
                                    c.g[A2][0] = 0x8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                    c.g[S3][0] = 0x69;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.mult(0, 3, 3, 18);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32), 0u64);
                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                    c.mult(0, 4, 4, 18);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[A3][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                                    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[RA][0] = sx(0x0029993c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                    call_ptr(c, 0x00416210, 0x0029993c)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[A0][0] = 0x64;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                    c.g[A1][0] = 0x5a;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[A2][0] = 0x32;
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A3][0] = 0x7f;
                                    c.sw((c.g[V1][0] as u32), 0u64);
                                    c.g[T0][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32), 0u64);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32), 0u64);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32), 0u64);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V1][0] as u32), c.g[T0][0]);
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
                                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                                    c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                    c.mult(0, 4, 4, 18);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    if t {
                                        break 'b_299ae8;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                        let t = (c.g[V1][0] as i64) < 0;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_299b14;
                                        } else {
                                            break 'b_299ae8;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                if c.g[V1][0] == 0u64 {
                                    c.g[V0][0] = 0x7f;
                                    break 'b_299b14;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                    break 'b_299b14;
                                }
                            }
                            c.cycles += 14;
                            c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                            c.g[A1][0] = 0x9;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x00299b2c);
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                            call_ptr(c, 0x00287968, 0x00299b2c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A0][0] = c.g[S4][0];
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[RA][0] = sx(0x00299b4c);
                            c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                            call_ptr(c, 0x002906b8, 0x00299b4c)?;
                            break 'b_299b4c;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_299b50;
            }
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
        return Ok(());
    });
}

/// `void SSXAudioSystem_StartTrickMeterSound(SSXAudioSystem* audio, Rider* rider)`:
/// when the rider has both state words (+0x874, +0x87c > 0) and the audio is
/// not busy (`0x288ae0`), stops the previous meter voice (`audio+0x5fdc`, if
/// not -1) and pushes and plays sound 0x66 (kind 9), keeping its handle.
pub fn ssx_audio_system_start_trick_meter_sound(c: &mut Cpu) {
    guest(c, |c| {
        'b_29b3a4: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x0029b104);
            c.g[S0][0] = c.g[A1][0];
            call_ptr(c, 0x00288ae0, 0x0029b104)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_29b3a4;
            } else {
                'b_29b120: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x874));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_29b120;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x87c));
                        c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
                        break 'b_29b120;
                    }
                }
                'b_29b3a0: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_29b3a0;
                    } else {
                        'b_29b148: {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5fdc));
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.g[A2][0] = 0x1;
                            if t {
                                break 'b_29b148;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                                c.mtc1(0, 12);
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x0029b148);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                Callee {
                                    addr: 0x002ad5f0,
                                    f: ee::s08::BankMonitor_StopHandle,
                                }
                                .call(c, 0x0029b148)?;
                                break 'b_29b148;
                            }
                        }
                        c.cycles += 150;
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32));
                        c.g[T1][0] = 0x18;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x8;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                        c.g[S2][0] = 0x66;
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
                        c.g[RA][0] = sx(0x0029b220);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x00416210, 0x0029b220)?;
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
                        c.g[A1][0] = 0x9;
                        c.g[A2][0] = 0u64;
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
                        c.g[RA][0] = sx(0x0029b37c);
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                        call_ptr(c, 0x00287968, 0x0029b37c)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[RA][0] = sx(0x0029b39c);
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        call_ptr(c, 0x002906b8, 0x0029b39c)?;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x5fdc), c.g[V0][0]);
                        break 'b_29b3a0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_29b3a4;
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
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (the fakes sit in global override
    /// slots and only act on machines that carry it).
    const OC1029_CTRL: u32 = DATA + 0xf_0000;
    const OC1029_MAGIC: u32 = 0x1029_c0de;
    /// The callees faked: a forced result lives at `OC1029_CTRL + 0x100 + 8*i`
    /// (flag word, value word).
    const OC1029_CALLEES: [u32; 11] = [
        0x0041_6210,
        0x002b_2850,
        0x0029_06b8,
        0x003b_58a0,
        0x003b_8ab8,
        0x003b_58d8,
        0x002a_d5f0,
        0x0028_7968,
        0x0028_8ae0,
        0x0028_b1c8,
        0x0027_0280,
    ];

    /// The sound object, its controller (`*(audio+0x118) -> controller`),
    /// the bank-monitor block at `controller + 0x1d8`, the field regions the
    /// block's pointers address, and the rider / race objects.
    const OC1029_AUDIO: u32 = DATA + 0x1000;
    const OC1029_CTL: u32 = DATA + 0x8000;
    const OC1029_HOLDER: u32 = DATA + 0x8800;
    const OC1029_REGION: u32 = DATA + 0xa000;
    const OC1029_OBJ: u32 = DATA + 0xc000;
    const OC1029_RACE: u32 = DATA + 0xd800;

    fn oc1029_unhooked(_c: &mut Cpu) {}

    /// The fake for the callee at `A`: logs the call like `fakes::fake_call`
    /// (an unwind included) and returns the forced result when the fixture set
    /// one; machines without the gate word run the body that was there before.
    fn oc1029_fake<const A: u32>(c: &mut Cpu) {
        if c.bus.read32(OC1029_CTRL) != OC1029_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1029_fake::<A>), oc1029_unhooked);
        }
        let i = OC1029_CALLEES.iter().position(|&a| a == A).unwrap() as u32;
        let slot = OC1029_CTRL + 0x100 + 8 * i;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, A);
        if c.pc == ra && c.bus.read32(slot) != 0 {
            c.g[V0][0] = sx(c.bus.read32(slot + 4));
        }
    }

    macro_rules! oc1029_hooks {
        ($($slot:path => $addr:literal),* $(,)?) => {
            fn oc1029_install_fakes() {
                $( fakes::hook(
                    addr_of_mut!($slot),
                    oc1029_fake::<$addr>,
                    fakes::prev_cell(oc1029_fake::<$addr>),
                ); )*
            }
        };
    }

    oc1029_hooks! {
        ee::s15::OVERRIDE_00416210 => 0x0041_6210,
        ee::s08::OVERRIDE_002B2850 => 0x002b_2850,
        ee::s07::OVERRIDE_002906B8 => 0x0029_06b8,
        ee::s13::OVERRIDE_003B58A0 => 0x003b_58a0,
        ee::s13::OVERRIDE_003B8AB8 => 0x003b_8ab8,
        ee::s13::OVERRIDE_003B58D8 => 0x003b_58d8,
        ee::s08::OVERRIDE_002AD5F0 => 0x002a_d5f0,
        ee::s07::OVERRIDE_00287968 => 0x0028_7968,
        ee::s07::OVERRIDE_00288AE0 => 0x0028_8ae0,
        ee::s07::OVERRIDE_0028B1C8 => 0x0028_b1c8,
        ee::s07::OVERRIDE_00270280 => 0x0027_0280,
    }

    /// Make the callee at `addr` return `value` (zero-extended or sign-extended
    /// from 32 bits) instead of its pointer-like default.
    fn oc1029_force(c: &mut Cpu, addr: u32, value: u32) {
        let i = OC1029_CALLEES.iter().position(|&a| a == addr).unwrap() as u32;
        c.bus.write32(OC1029_CTRL + 0x100 + 8 * i, 1);
        c.bus.write32(OC1029_CTRL + 0x104 + 8 * i, value);
    }

    /// Scrambled machine with the fakes armed, and the sound object built:
    /// `audio+0` and `holder+0` both point at the controller, whose bank-monitor
    /// block (`+0x1d8`) holds pointers to 0x100-byte random regions (the arrays
    /// the ring push writes through), instance index `seed % 3`, and a
    /// base-volume array of mixed small and large ints.
    fn oc1029_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1029_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC1029_CTRL, OC1029_MAGIC);
        fill(c, &mut r, OC1029_AUDIO, 0x1900);
        fill(c, &mut r, OC1029_CTL, 0x200);
        fill(c, &mut r, OC1029_REGION, 0x600);
        let monitor = OC1029_CTL + 0x1d8;
        c.bus.write32(OC1029_AUDIO, OC1029_CTL);
        c.bus.write32(OC1029_AUDIO + 4, OC1029_REGION + 0x100 * 20);
        c.bus.write32(OC1029_AUDIO + 8, OC1029_REGION + 0x100 * 21);
        c.bus.write32(OC1029_AUDIO + 0x118, OC1029_HOLDER);
        c.bus.write32(OC1029_HOLDER, OC1029_CTL);
        c.bus.write32(OC1029_CTL + 0x1f4, seed % 3);
        for (k, off) in [
            0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus
                .write32(monitor + off, OC1029_REGION + 0x100 * k as u32);
        }
        let base_vol = OC1029_REGION + 0x100 * 2;
        for i in 0..4 {
            let v = [-5i32, 0, 50, 100, 127, 130, 1000][(r.next_u32() % 7) as usize];
            c.bus.write32(base_vol + 4 * i, v as u32);
        }
        c.bus.write32(GAME_GP - 0x45d8, 0.01f32.to_bits());
        r
    }

    /// Writes the rider words the meter sounds read: +0x874 non-zero and
    /// +0x87c positive with probability `percent`; otherwise one of them fails.
    fn oc1029_rider(c: &mut Cpu, r: &mut Rng, percent: u32) {
        let (first, second) = if r.next_u32() % 100 < percent {
            (1 + r.next_u32() % 9, 1 + r.next_u32() % 9)
        } else {
            [(0, 5), (3, 0), (3, 0xffff_ffff)][(r.next_u32() % 3) as usize]
        };
        c.bus.write32(OC1029_OBJ + 0x874, first);
        c.bus.write32(OC1029_OBJ + 0x87c, second);
    }

    /// The `0x288ae0` busy test: zero (go on) with probability 75 %, else a
    /// non-zero pointer (return at once).
    fn oc1029_not_busy(c: &mut Cpu, r: &mut Rng) {
        if r.next_u32() % 100 < 75 {
            oc1029_force(c, 0x0028_8ae0, 0);
        }
    }

    /// `SSXAudioSystem_RestartHandleSound`: the voice is busy (1, 7), free (0) or
    /// the lock call result is negative; the ring push, `GetChannelGainPtr` and
    /// `Audio_PlayPushedSound` follow; unwinds at the call indices.
    #[test]
    fn ssx_audio_system_restart_handle_sound_matches() {
        check_reaching(
            ee::s07::f_00291438__generated,
            ssx_audio_system_restart_handle_sound,
            120,
            &[
                0x0029_145c,
                0x0029_1464,
                0x0029_146c,
                0x0029_1490,
                0x0029_156c,
                0x0029_16c8,
                0x0029_16e8,
            ],
            &|c, seed| {
                let _r = oc1029_base(c, seed, 0x0029_1438, 7);
                // The `0x3b8ab8` result is the "busy" test: signed `0 < v0`.
                let busy = [0u32, 1, 0xffff_ffff, 7][(seed % 4) as usize];
                oc1029_force(c, 0x003b_8ab8, busy);
                c.g[A0][0] = sx(OC1029_AUDIO);
            },
        );
    }

    /// `SSXAudioSystem_StartTrickMeterSound`: busy audio exits at once, the rider
    /// words fail, the previous voice is -1 or not, with unwinds.
    #[test]
    fn ssx_audio_system_start_trick_meter_sound_matches() {
        check_reaching(
            ee::s08::f_0029b0e0__generated,
            ssx_audio_system_start_trick_meter_sound,
            200,
            &[0x0029_b104, 0x0029_b148, 0x0029_b39c],
            &|c, seed| {
                let mut r = oc1029_base(c, seed, 0x0029_b0e0, 3);
                oc1029_not_busy(c, &mut r);
                oc1029_rider(c, &mut r, 80);
                let previous = if r.next_u32() % 100 < 40 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(OC1029_AUDIO + 0x5fdc, previous);
                c.g[A0][0] = sx(OC1029_AUDIO);
                c.g[A1][0] = sx(OC1029_OBJ);
            },
        );
    }

    /// The race object `0x28b1c8` returns for `PlayMeterFullSound`: +0x28 is
    /// passed to `0x270280`, +0x214 must be 4 to play.
    fn oc1029_race(c: &mut Cpu, r: &mut Rng) {
        fill(c, r, OC1029_RACE, 0x100);
        let state = if r.next_u32() % 100 < 70 {
            4
        } else {
            r.next_u32()
        };
        c.bus.write32(OC1029_RACE + 0x214, state);
        oc1029_force(c, 0x0028_b1c8, OC1029_RACE);
    }

    /// `SSXAudioSystem_PlayMeterFullSound`: every guard of the sting (busy,
    /// rider words, float sign, the global flag, `0x270280`, the race state).
    #[test]
    fn ssx_audio_system_play_meter_full_sound_matches() {
        check_reaching(
            ee::s08::f_002997b8__generated,
            ssx_audio_system_play_meter_full_sound,
            300,
            &[
                0x0029_97e0,
                0x0029_9838,
                0x0029_9840,
                0x0029_9854,
                0x0029_993c,
                0x0029_9b2c,
                0x0029_9b4c,
            ],
            &|c, seed| {
                let mut r = oc1029_base(c, seed, 0x0029_97b8, 7);
                oc1029_not_busy(c, &mut r);
                oc1029_rider(c, &mut r, 85);
                let mut sign = [-1.0f32, 1.0, 0.0, -0.0, -3.5][(r.next_u32() % 5) as usize];
                if r.next_u32() % 100 < 70 {
                    sign = -2.0;
                }
                c.bus.write32(OC1029_OBJ + 0x470, sign.to_bits());
                let flag = if r.next_u32() % 100 < 25 {
                    r.next_u32() | 1
                } else {
                    r.next_u32() & !1
                };
                c.bus.write32(GAME_GP - 0x6a0, flag);
                oc1029_race(c, &mut r);
                if r.next_u32() % 100 < 70 {
                    oc1029_force(c, 0x0027_0280, 0);
                }
                c.g[A0][0] = sx(OC1029_AUDIO);
                c.g[A1][0] = sx(OC1029_OBJ);
            },
        );
    }

    /// `SSXAudioSystem_StopTrackedSounds`: a ring of `cap` five-word entries
    /// (empty, partly full, wrapped) holding the event, zeros and random words;
    /// matching entries with and without the stop flag, the others pushed back.
    #[test]
    fn ssx_audio_system_stop_tracked_sounds_matches() {
        check_reaching(
            ee::s08::f_00295cc8__generated,
            ssx_audio_system_stop_tracked_sounds,
            300,
            &[0x0029_5ea0],
            &|c, seed| {
                let mut r = oc1029_base(c, seed, 0x0029_5cc8, 4);
                let event = r.next_u32() % 8;
                let cap = [1u32, 3, 5, 8, 16, 16][(r.next_u32() % 6) as usize];
                let (head, tail) = (r.next_u32() % cap, r.next_u32() % cap);
                let buf = OC1029_REGION + 0x100 * 22;
                for i in 0..16 {
                    let v = match r.next_u32() % 5 {
                        0 | 1 => event,
                        2 => 0,
                        3 => 1,
                        _ => r.next_u32(),
                    };
                    c.bus.write32(buf + 4 * i, v);
                }
                c.bus.write32(OC1029_AUDIO + 0x597c, cap);
                c.bus.write32(OC1029_AUDIO + 0x5980, buf);
                c.bus.write32(OC1029_AUDIO + 0x5984, head);
                c.bus.write32(OC1029_AUDIO + 0x5988, tail);
                c.g[A0][0] = sx(OC1029_AUDIO);
                c.g[A1][0] = event as u64;
            },
        );
    }

    /// `SSXAudioSystem_PlaySongSegmentCallback`: segment counts positive, zero and
    /// negative; the index below zero (wrapped by adding the count), in range
    /// and past the end (reset to 0); base volumes below, inside and above 0..127.
    #[test]
    fn ssx_audio_system_play_song_segment_callback_matches() {
        check_reaching(
            ee::s07::f_0028c430__generated,
            ssx_audio_system_play_song_segment_callback,
            300,
            &[0x0028_c5b0, 0x0028_c868, 0x0028_c870],
            &|c, seed| {
                let mut r = oc1029_base(c, seed, 0x0028_c430, 3);
                let cue = OC1029_OBJ;
                let obj = OC1029_OBJ + 0x100;
                let desc = OC1029_OBJ + 0x200;
                let count = [5i32, 1, 3, 8, 0, -2, 4, 6][(r.next_u32() % 8) as usize];
                // A negative index with a non-positive count never ends: keep the
                // index at or above the count there.
                let idx = if count > 0 {
                    (r.next_u32() % (4 * count as u32)) as i32 - 2 * count
                } else {
                    count + (r.next_u32() % 3) as i32
                };
                let volume = [0i32, 50, 100, 127, 200, -10, 80][(r.next_u32() % 7) as usize];
                c.bus.write32(cue + 0x18, obj);
                c.bus.write32(cue + 0x1c, OC1029_REGION + 0x100 * 23);
                c.bus.write32(cue + 0x20, idx as u32);
                c.bus.write32(cue + 0x24, r.next_u32());
                c.bus.write32(obj, desc);
                c.bus.write32(obj + 0x34, r.next_u32());
                c.bus.write32(desc + 0x70, count as u32);
                c.bus.write32(desc + 0x8c, volume as u32);
                c.g[A0][0] = sx(OC1029_AUDIO);
                c.g[A1][0] = sx(cue);
            },
        );
    }
}
