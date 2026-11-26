//! Staging module oc768 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc768`.

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
        0x0029_c7b0,
        ssx_audio_system_on_start_gate_open,
        "SSXAudioSystem_OnStartGateOpen",
    ),
    (
        0x0019_fd58,
        fe_module_rider_preview_poll_textures,
        "FEModule_RiderPreviewPollTextures",
    ),
    (
        0x0028_cf98,
        ssx_audio_system_play_music,
        "SSXAudioSystem_PlayMusic",
    ),
    (
        0x0028_8d18,
        ssx_audio_system_setup_channel_gains,
        "SSXAudioSystem_SetupChannelGains",
    ),
    (0x003d_1038, path_set_paused, "PATH_SetPaused"),
    (
        0x002a_dde0,
        bank_monitor_kill_timers_by_callback,
        "BankMonitor_KillTimersByCallback",
    ),
];

/// `void SSXAudioSystem_OnStartGateOpen(AudioSystem* self)` (0x29c7b0): the race
/// start. Resumes the music player, resets the music state, records the game
/// module's value, queues sound 0x62 and (unless a replay plays) the start-gate
/// one-shot (sound 0x5e plus its volume ramp, an inlined stream-create at
/// `self->snd`), sends the music event 0x25 once, picks the DJ intro or the PA
/// speech for the game mode, and stores the game mode.
pub fn ssx_audio_system_on_start_gate_open(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        'b_29cb0c: {
            c.cycles += 28;
            Frame::open(c, 0x60);
            frame.save(c, S4, 0x10);
            c.g[S4][0] = c.g[A0][0];
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            frame.save_ra(c, 0);
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
            call_ptr(c, 0x002b3a98, 0x0029c7d8)?;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0028df08, 0x0029c7e0)?;
            call_ptr(c, 0x0028b1c8, 0x0029c7e8)?;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = 0x62;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S4][0] as u32).wrapping_add(0x582c), c.g[V0][0]);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0029c810)?;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x00288ae0, 0x0029c818)?;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                break 'b_29cb0c;
            } else {
                'b_29cad0: {
                    'b_29caa4: {
                        c.cycles += 155;
                        c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[S2][0] = 0x18;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x8;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                        c.g[S3][0] = 0x5e;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.mult(0, 3, 3, 18);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.sdr(2, c.g[V1][0] as u32);
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                        c.mult(0, 4, 4, 18);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x00416210, 0x0029c8f8)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[A0][0] = 0x64;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                        c.g[A1][0] = 0x5a;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A2][0] = 0x32;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A3][0] = 0x7f;
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[T0][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                        c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                        c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 18);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if t {
                            break 'b_29caa4;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_29cad0;
                            } else {
                                break 'b_29caa4;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_29cad0;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        break 'b_29cad0;
                    }
                }
                c.cycles += 15;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[A1][0] = 0x9;
                c.g[A0][0] = c.g[S4][0];
                c.g[A2][0] = 0u64;
                c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x00287968, 0x0029cae8)?;
                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x002906b8, 0x0029cb08)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                break 'b_29cb0c;
            }
        }
        'b_29cb2c: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
            if t {
                break 'b_29cb2c;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0029cb2c)?;
                break 'b_29cb2c;
            }
        }
        'b_29cbf4: {
            c.cycles += 11;
            call_ptr(c, 0x0014dc80, 0x0029cb34)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x0029cb40)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x00295028, 0x0029cb50)?;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                break 'b_29cbf4;
            } else {
                'b_29cb84: {
                    'b_29cb74: {
                        c.cycles += 5;
                        c.g[V0][0] = 0x530000;
                        c.g[S0][0] = 0x1;
                        c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                        let t = c.g[V0][0] == c.g[S0][0];
                        c.g[A0][0] = 0x2;
                        if t {
                            break 'b_29cb74;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] != c.g[A0][0];
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_29cb84;
                            } else {
                                break 'b_29cb74;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x002a3170, 0x0029cb7c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                    if t {
                        break 'b_29cbf4;
                    } else {
                        break 'b_29cb84;
                    }
                }
                'b_29cbe4: {
                    'b_29cbc0: {
                        'b_29cbb0: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                            if c.g[V1][0] == 0u64 {
                                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                                break 'b_29cbb0;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x002a3400, 0x0029cb9c)?;
                                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5560));
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x002b1758, 0x0029cba8)?;
                                let t = true;
                                if t {
                                    break 'b_29cbc0;
                                } else {
                                    break 'b_29cbb0;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[S0][0] {
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x5774), c.g[S0][0]);
                            break 'b_29cbe4;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[A0][0] {
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                                break 'b_29cbf4;
                            } else {
                                break 'b_29cbc0;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x002a4078, 0x0029cbc8)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x5774), c.g[S0][0]);
                        break 'b_29cbe4;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x002a40e8, 0x0029cbd8)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                            break 'b_29cbf4;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x5774), c.g[S0][0]);
                            break 'b_29cbe4;
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x002b4908, 0x0029cbec)?;
                c.sw((c.g[S4][0] as u32).wrapping_add(0x5778), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x627c));
                break 'b_29cbf4;
            }
        }
        'b_29cc5c: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                break 'b_29cc5c;
            } else {
                'b_29cc38: {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00288ae0, 0x0029cc04)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6298));
                        break 'b_29cc38;
                    } else {
                        c.cycles += 11;
                        c.g[V0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x627c), c.g[V0][0]);
                        c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                        c.g[A1][0] = 0x25;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0029cc30)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                        if t {
                            break 'b_29cc5c;
                        } else {
                            break 'b_29cc38;
                        }
                    }
                }
                'b_29cc58: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x25;
                    if t {
                        break 'b_29cc58;
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                        c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0029cc58)?;
                        break 'b_29cc58;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                break 'b_29cc5c;
            }
        }
        c.cycles += 19;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x260));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x32));
        call_ptr(c, 0x00295950, 0x0029cc78)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.restore(c, S0, 0x50);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
        frame.restore(c, S1, 0x40);
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        frame.restore(c, S2, 0x30);
        c.sw((c.g[S4][0] as u32).wrapping_add(0x5824), c.g[A0][0]);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    });
}

/// `int FEModule_RiderPreviewPollTextures(FEModule* self)` (0x19fd58): the rider
/// preview screen's texture loading poll. Returns 1 at once when the textures
/// are already loaded (`+0xcb8`). Otherwise asks the BE for the rider's profile
/// bolt list (`+0x28c` count, entries from `+0x290`, 4 bytes each: id halfword
/// and flags halfword), checks every bolt's texture file is loaded (`BoltFile_IsLoaded`);
/// when all are, hands each file's shape data to
/// `FEModule_RiderPreviewLoadShapeTextures` once (a 256-entry "used" table on the
/// stack, bit 4 of the flags selects the variant), releases the file and
/// clears the bolt's texture slot. Returns whether all were loaded.
pub fn fe_module_rider_preview_poll_textures(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x490 };
        'b_19fed0: {
            c.cycles += 14;
            Frame::open(c, 0x490);
            frame.save(c, S1, 0x470);
            frame.save(c, S0, 0x480);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S2, 0x460);
            frame.save(c, S3, 0x450);
            frame.save(c, S4, 0x440);
            frame.save(c, S5, 0x430);
            frame.save(c, S6, 0x420);
            frame.save(c, S7, 0x410);
            frame.save_ra(c, 0x400);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xcb8));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_19fed0;
            } else {
                'b_19fe0c: {
                    c.cycles += 15;
                    c.g[S6][0] = 0x1;
                    call_ptr(c, 0x0014dc80, 0x0019fd98)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019fda4)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    call_ptr(c, 0x0014ad28, 0x0019fdb4)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S5][0] = c.g[V0][0];
                    c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28c));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                    if t {
                        break 'b_19fe0c;
                    } else {
                        c.cycles += 3;
                        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                        c.g[S0][0] = c.g[S3][0];
                        'l_19fdd8: loop {
                            'b_19fe00: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                call_ptr(c, 0x0019cce8, 0x0019fde8)?;
                                let t = (c.g[V0][0] as i64) < 0;
                                c.g[A2][0] = c.g[V0][0];
                                if t {
                                    break 'b_19fe00;
                                } else {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x0019e238, 0x0019fdfc)?;
                                    if c.g[V0][0] == 0 {
                                        c.g[S6][0] = 0u64;
                                    }
                                    break 'b_19fe00;
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = c.g[S0][0] != 0u64;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019fdd8);
                                }
                                continue 'l_19fdd8;
                            } else {
                                break 'b_19fe0c;
                            }
                        }
                    }
                }
                'b_19fecc: {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        break 'b_19fecc;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x400;
                        call_ptr(c, 0x003e6448, 0x0019fe20)?;
                        let t = (c.g[S3][0] as i64) <= 0;
                        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                        if t {
                            break 'b_19fecc;
                        } else {
                            c.cycles += 2;
                            c.g[S7][0] = 0x1;
                            c.g[S5][0] = c.g[S3][0];
                            'l_19fe30: loop {
                                'b_19fec0: {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                    call_ptr(c, 0x0019cce8, 0x0019fe40)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let t = (c.g[S0][0] as i64) < 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_19fec0;
                                    } else {
                                        'b_19fe9c: {
                                            'b_19fe94: {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[A2][0] = c.g[S0][0];
                                                c.g[S3][0] = sx((c.g[S0][0] as u32) << 2);
                                                call_ptr(c, 0x0019e2b0, 0x0019fe5c)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = sx((c.g[SP][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                                if t {
                                                    break 'b_19fe94;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                    if t {
                                                        break 'b_19fe9c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[A2][0] = c.g[S0][0];
                                                        c.g[A0][0] = c.g[S4][0];
                                                        call_ptr(c, 0x0019e2b0, 0x0019fe7c)?;
                                                        c.g[A2][0] = c.lhu(
                                                            (c.g[S2][0] as u32).wrapping_add(0x2),
                                                        );
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.g[A2][0] & 0x10;
                                                        c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
                                                        call_ptr(c, 0x0019f908, 0x0019fe94)?;
                                                        break 'b_19fe94;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[V0][0] =
                                                sx((c.g[SP][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                            break 'b_19fe9c;
                                        }
                                        c.cycles += 9;
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[A0][0] = c.g[S4][0];
                                        c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                                        call_ptr(c, 0x0019dc20, 0x0019feac)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[A3][0] = 0xffffffffffffffff;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[A2][0] = c.lh(c.g[S2][0] as u32);
                                        call_ptr(c, 0x0019cd38, 0x0019fec0)?;
                                        break 'b_19fec0;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S5][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019fe30);
                                    }
                                    continue 'l_19fe30;
                                } else {
                                    break 'b_19fecc;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S6][0];
                break 'b_19fed0;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x480);
        frame.restore(c, S1, 0x470);
        frame.restore(c, S2, 0x460);
        frame.restore(c, S3, 0x450);
        frame.restore(c, S4, 0x440);
        frame.restore(c, S5, 0x430);
        frame.restore(c, S6, 0x420);
        frame.restore(c, S7, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        return Ok(());
    });
}

/// `void SSXAudioSystem_PlayMusic(AudioSystem* self, int startEvent, int
/// requestLine, int type, int skipPeakTheme)` (0x28cf98): start the music for
/// the current location: the peak's ambience bank in radio mode (`self+0x608c`
/// == 2), the peak theme (for locations from 17 up, unless skipped, requested
/// or a race type 6 to 11), else the normal music bank, then the fade-in
/// reset. Does nothing when a replay plays and `self+0x6294` is clear.
pub fn ssx_audio_system_play_music(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        'b_28d464: {
            'b_28cfe4: {
                c.cycles += 16;
                Frame::open(c, 0xe0);
                frame.save(c, S4, 0x90);
                frame.save(c, S0, 0xd0);
                c.g[S4][0] = c.g[A0][0];
                frame.save(c, S1, 0xc0);
                c.g[S0][0] = c.g[A3][0];
                frame.save(c, S2, 0xb0);
                c.g[S1][0] = c.g[A2][0];
                frame.save(c, S3, 0xa0);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S5, 0x80);
                frame.save_ra(c, 0x70);
                c.g[S3][0] = c.g[T0][0];
                call_ptr(c, 0x00288ae0, 0x0028cfd0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_28cfe4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6294));
                    if c.g[V0][0] == 0u64 {
                        frame.restore(c, S0, 0xd0);
                        break 'b_28d464;
                    } else {
                        break 'b_28cfe4;
                    }
                }
            }
            'b_28d3c8: {
                'b_28d3b8: {
                    'b_28d170: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x608c));
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x6280), 0u64);
                        c.g[V0][0] = 0x2;
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x62a0), c.g[A0][0]);
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x623c), 0u64);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.sw((c.g[S4][0] as u32).wrapping_add(0x6240), 0u64);
                        if t {
                            break 'b_28d170;
                        } else {
                            'b_28d034: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[A0][0];
                                c.g[A3][0] = c.g[S0][0];
                                if t {
                                    break 'b_28d034;
                                } else {
                                    c.cycles += 11;
                                    call_ptr(c, 0x0028b1e8, 0x0028d010)?;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dd58, 0x0028d01c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00144bc0, 0x0028d024)?;
                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x002a1e20, 0x0028d030)?;
                                    c.g[A3][0] = c.g[V0][0];
                                    break 'b_28d034;
                                }
                            }
                            'b_28d14c: {
                                'b_28d09c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[A3][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x2;
                                    if t {
                                        break 'b_28d09c;
                                    } else {
                                        c.cycles += 23;
                                        c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                        c.g[S1][0] = 0x480000;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a38));
                                        c.g[A1][0] = 0x8;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[S5][0] = c.g[S2][0];
                                        call_ptr(c, 0x00287968, 0x0028d06c)?;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d088)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = 0x65;
                                        call_ptr(c, 0x002b49c0, 0x0028d094)?;
                                        let t = true;
                                        if t {
                                            break 'b_28d14c;
                                        } else {
                                            break 'b_28d09c;
                                        }
                                    }
                                }
                                'b_28d0fc: {
                                    c.cycles += 2;
                                    let t = c.g[A3][0] != c.g[V0][0];
                                    c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    if t {
                                        break 'b_28d0fc;
                                    } else {
                                        c.cycles += 22;
                                        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                        c.g[S1][0] = 0x480000;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a48));
                                        c.g[A1][0] = 0x8;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[S5][0] = c.g[S2][0];
                                        call_ptr(c, 0x00287968, 0x0028d0cc)?;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d0e8)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = 0x66;
                                        call_ptr(c, 0x002b49c0, 0x0028d0f4)?;
                                        let t = true;
                                        if t {
                                            break 'b_28d14c;
                                        } else {
                                            break 'b_28d0fc;
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                c.g[S1][0] = 0x480000;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[S0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x10));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2a58));
                                c.g[A1][0] = 0x8;
                                c.g[A2][0] = 0u64;
                                c.g[S0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                call_ptr(c, 0x00287968, 0x0028d120)?;
                                c.g[S5][0] = c.g[S2][0];
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A3][0] = c.g[V0][0];
                                c.g[A2][0] = 0xc;
                                let t = c.g[V1][0] as u32;
                                c.g[T0][0] = 0u64;
                                call_ptr(c, t, 0x0028d140)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0x67;
                                call_ptr(c, 0x002b49c0, 0x0028d14c)?;
                                break 'b_28d14c;
                            }
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x002b46d0, 0x0028d154)?;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = 0u64;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x6280), c.g[V0][0]);
                            call_ptr(c, 0x002b3cc0, 0x0028d168)?;
                            let t = true;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_28d3b8;
                            } else {
                                break 'b_28d170;
                            }
                        }
                    }
                    'b_28d22c: {
                        'b_28d1f8: {
                            c.cycles += 11;
                            call_ptr(c, 0x0014dc80, 0x0028d178)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014dd58, 0x0028d184)?;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x00144bc0, 0x0028d18c)?;
                            c.g[V1][0] = c.g[V0][0];
                            c.g[V0][0] = c.g[V1][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[SP][0];
                            if t {
                                break 'b_28d1f8;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                                'l_28d1a4: loop {
                                    c.cycles += 19;
                                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                                    c.ldr(5, c.g[V1][0] as u32);
                                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.sdr(5, c.g[A0][0] as u32);
                                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0028d1a4);
                                        }
                                        continue 'l_28d1a4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        if t {
                                            break 'b_28d22c;
                                        } else {
                                            break 'b_28d1f8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                        'l_28d200: loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0028d200);
                                }
                                continue 'l_28d200;
                            } else {
                                break 'b_28d22c;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                        break 'b_28d3c8;
                    } else {
                        c.cycles += 2;
                        if c.g[S1][0] != 0u64 {
                            c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                            break 'b_28d3c8;
                        } else {
                            c.cycles += 2;
                            if c.g[S3][0] != 0u64 {
                                c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                break 'b_28d3c8;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S4][0];
                                call_ptr(c, 0x002a42c8, 0x0028d25c)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    break 'b_28d3c8;
                                } else {
                                    'b_28d394: {
                                        'b_28d2f0: {
                                            c.cycles += 14;
                                            call_ptr(c, 0x0028b1e8, 0x0028d26c)?;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0014dd58, 0x0028d278)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x0028d280)?;
                                            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A0][0] = c.g[S4][0];
                                            call_ptr(c, 0x002a1e20, 0x0028d28c)?;
                                            c.g[V1][0] = c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_28d2f0;
                                            } else {
                                                c.cycles += 21;
                                                c.g[S2][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                                c.g[S1][0] = 0x4a0000;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x3630));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[S0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[S0][0] = sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00287968, 0x0028d2c0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A3][0] = c.g[V0][0];
                                                c.g[A2][0] = 0xc;
                                                let t = c.g[V1][0] as u32;
                                                c.g[T0][0] = 0u64;
                                                call_ptr(c, t, 0x0028d2dc)?;
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x002b49c0, 0x0028d2e8)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S5][0];
                                                if t {
                                                    break 'b_28d394;
                                                } else {
                                                    break 'b_28d2f0;
                                                }
                                            }
                                        }
                                        'b_28d348: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[S2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                            if t {
                                                break 'b_28d348;
                                            } else {
                                                c.cycles += 20;
                                                c.g[S1][0] = 0x4a0000;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x3638));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[S0][0] =
                                                    c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = 0x1;
                                                c.g[A2][0] = 0u64;
                                                c.g[S0][0] = sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00287968, 0x0028d318)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A3][0] = c.g[V0][0];
                                                c.g[A2][0] = 0xc;
                                                let t = c.g[V1][0] as u32;
                                                c.g[T0][0] = 0u64;
                                                call_ptr(c, t, 0x0028d334)?;
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[A1][0] = 0x2;
                                                call_ptr(c, 0x002b49c0, 0x0028d340)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S5][0];
                                                if t {
                                                    break 'b_28d394;
                                                } else {
                                                    break 'b_28d348;
                                                }
                                            }
                                        }
                                        c.cycles += 19;
                                        c.g[S1][0] = 0x4a0000;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3640));
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[S0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x1;
                                        c.g[A2][0] = 0u64;
                                        c.g[S0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        call_ptr(c, 0x00287968, 0x0028d368)?;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A3][0] = c.g[V0][0];
                                        c.g[A2][0] = 0xc;
                                        let t = c.g[V1][0] as u32;
                                        c.g[T0][0] = 0u64;
                                        call_ptr(c, t, 0x0028d384)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = 0x3;
                                        call_ptr(c, 0x002b49c0, 0x0028d390)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        break 'b_28d394;
                                    }
                                    c.cycles += 9;
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x002b3cc0, 0x0028d39c)?;
                                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
                                    c.g[A1][0] = 0xc;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0028d3b4)?;
                                    c.g[A0][0] = c.g[S5][0];
                                    break 'b_28d3b8;
                                }
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A1][0] = 0x7f;
                call_ptr(c, 0x002b3c28, 0x0028d3c0)?;
                let t = true;
                frame.restore(c, S0, 0xd0);
                if t {
                    break 'b_28d464;
                } else {
                    break 'b_28d3c8;
                }
            }
            'b_28d430: {
                c.cycles += 23;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0u64;
                c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x00287968, 0x0028d3e0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[A3][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0xc;
                let t = c.g[V1][0] as u32;
                c.g[T0][0] = 0u64;
                call_ptr(c, t, 0x0028d3fc)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x508));
                c.g[A0][0] = c.g[S5][0];
                c.sw((c.g[S4][0] as u32).wrapping_add(0x62a4), c.g[V0][0]);
                call_ptr(c, 0x002b4908, 0x0028d40c)?;
                c.sw((c.g[S4][0] as u32).wrapping_add(0x5778), c.g[V0][0]);
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x002b3cc0, 0x0028d41c)?;
                let t = c.g[S3][0] == 0u64;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                if t {
                    break 'b_28d430;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = 0x24;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = c.g[V1][0];
                    }
                    break 'b_28d430;
                }
            }
            c.cycles += 13;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5558));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0028d448)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0xffffffffffffffff;
            call_ptr(c, 0x00285d98, 0x0028d454)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0028f108, 0x0028d460)?;
            frame.restore(c, S0, 0xd0);
            break 'b_28d464;
        }
        c.cycles += 8;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        return Ok(());
    });
}

/// `void SSXAudioSystem_SetupChannelGains(AudioSystem* self, int mode)` (0x288d18):
/// fill the per-kind volume scaling globals (gp+0x414..0x460: SFX, music,
/// speech, board, ambient, per-character...) for the mix `mode` (0: all 1.0
/// except the character speech 0.6; 1: the tuned in-race mix; others leave
/// them as they are), then re-apply every one of the 11 channel volumes with
/// `SetChannelVolume(self, ch, GetChannelVolume(self, ch))`.
pub fn ssx_audio_system_setup_channel_gains(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        'b_288e1c: {
            'b_288e18: {
                'b_288d8c: {
                    c.cycles += 5;
                    Frame::open(c, 0x20);
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A1][0] != 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_288d8c;
                    } else {
                        c.cycles += 24;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4610));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x460));
                        c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x41c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x418));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x420));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x424));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x428));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x430));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x434));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x438));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x43c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x440));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x444));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x448));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x44c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x450));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x454));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x458));
                        let t = true;
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x45c));
                        if t {
                            break 'b_288e18;
                        } else {
                            break 'b_288d8c;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_288e1c;
                } else {
                    c.cycles += 32;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x460c));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4608));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 6);
                    c.g[AT][0] = 0x3f400000;
                    c.mtc1(1, 2);
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4604));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4600));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 5);
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x45fc));
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x45f8));
                    c.swc1(3, (c.g[GP][0] as u32).wrapping_add(0x420));
                    c.swc1(4, (c.g[GP][0] as u32).wrapping_add(0x424));
                    c.swc1(5, (c.g[GP][0] as u32).wrapping_add(0x430));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x438));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x44c));
                    c.swc1(7, (c.g[GP][0] as u32).wrapping_add(0x450));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x454));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x45c));
                    c.swc1(8, (c.g[GP][0] as u32).wrapping_add(0x460));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x418));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x41c));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x428));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x434));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x43c));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x440));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x444));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x448));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x458));
                    break 'b_288e18;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            break 'b_288e1c;
        }
        c.cycles += 80;
        // Re-apply every channel's set volume: SetChannelVolume(self, ch,
        // GetChannelVolume(self, ch)) for the eleven channels, so the new
        // scaling factors reach the derived per-channel gains.
        for channel in 0..=10u32 {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel as u64;
            call_ptr(c, 0x0028_7920, 0x0028_8e24 + 0x1c * channel)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel as u64;
            call_ptr(c, 0x0028_7700, 0x0028_8e34 + 0x1c * channel)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    });
}

/// `void PATH_SetPaused(uint mask, uchar paused)` (0x3d1038): the path player
/// pause command. Does nothing before init (`0x44f42c` byte), while a map is
/// selected (`0x44f430`), or when the `PATH_SelectMap(-1)` query fails. A mask of
/// -1 also stores the value as the master pause byte (`0x44f42d`). Then, for each
/// of the four path maps whose id nibbles match the mask (and that `PATH_SelectMap`
/// accepts), tells every loaded track whose mask bit is set to pause
/// (`PATH_PlatSetPaused(track->voice, paused)`) and remembers it in the track
/// (+3). Finally clears the selection.
pub fn path_set_paused(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xa0 };
        'b_3d118c: {
            'b_3d1188: {
                c.cycles += 16;
                Frame::open(c, 0xa0);
                c.g[V0][0] = 0x450000;
                frame.save(c, S2, 0x70);
                frame.save(c, S3, 0x60);
                c.g[S2][0] = c.g[A0][0];
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
                c.g[S3][0] = c.g[A1][0] & 0xff;
                frame.save(c, S0, 0x90);
                frame.save(c, S1, 0x80);
                frame.save(c, S4, 0x50);
                frame.save(c, S5, 0x40);
                frame.save(c, S6, 0x30);
                frame.save(c, S7, 0x20);
                frame.save(c, FP, 0x10);
                let t = c.g[V1][0] == 0u64;
                frame.save_ra(c, 0);
                if t {
                    break 'b_3d1188;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                    let t = c.g[V0][0] != 0u64;
                    frame.restore(c, S0, 0x90);
                    if t {
                        break 'b_3d118c;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 0xffffffffffffffff;
                        call_ptr(c, 0x003d5450, 0x003d1090)?;
                        let t = c.g[V0][0] == 0u64;
                        frame.restore(c, S0, 0x90);
                        if t {
                            break 'b_3d118c;
                        } else {
                            'b_3d10b4: {
                                c.cycles += 4;
                                c.g[V0][0] = 0xffffffffffff0000;
                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[V0][0] = 0x510000;
                                if t {
                                    break 'b_3d10b4;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x450000;
                                    c.sb((c.g[V0][0] as u32).wrapping_sub(0xbd3), c.g[S3][0]);
                                    c.g[V0][0] = 0x510000;
                                    break 'b_3d10b4;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = 0u64;
                            c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
                            c.g[S5][0] = 0x450000;
                            c.g[FP][0] = 0xfffffffff0000000;
                            c.g[S7][0] = 0xf000000;
                            c.g[V0][0] = 0x928;
                            'l_3d10d0: loop {
                                'b_3d1170: {
                                    'b_3d116c: {
                                        c.cycles += 5;
                                        c.mult(0, 5, 4, 2);
                                        c.g[V0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_3d116c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[V0][0] = 0x510000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x7640));
                                            c.g[V1][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                                            c.g[V0][0] = c.g[V1][0] & c.g[FP][0];
                                            c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = c.g[V1][0] & c.g[S7][0];
                                            if t {
                                                break 'b_3d116c;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[A0][0] = c.g[S4][0];
                                                    break 'b_3d1170;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[A0][0] & 0xff;
                                                    call_ptr(c, 0x003d5450, 0x003d1118)?;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A0][0] = c.g[S4][0];
                                                        break 'b_3d1170;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[S1][0] = 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S5][0] as u32)
                                                                .wrapping_sub(0xbe0));
                                                        'l_3d1128: loop {
                                                            'b_3d1160: {
                                                                'b_3d115c: {
                                                                    c.cycles += 5;
                                                                    c.g[V1][0] = sx((c.g[S1][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V1][0] as u32,
                                                                        ));
                                                                    c.g[S0][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x58));
                                                                    let t = c.g[S0][0] == 0u64;
                                                                    c.g[V0][0] = sx((c.g[S2][0]
                                                                        as u32)
                                                                        >> ((c.g[S1][0] as u32)
                                                                            & 31));
                                                                    if t {
                                                                        break 'b_3d115c;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] ^ 0x1;
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] & 0x1;
                                                                        if c.g[V0][0] != 0u64 {
                                                                            c.g[S1][0] =
                                                                                sx((c.g[S1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            break 'b_3d1160;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x54,
                                                                                ));
                                                                            c.g[A1][0] = c.g[S3][0];
                                                                            call_ptr(
                                                                                c, 0x002af7a0,
                                                                                0x003d1158,
                                                                            )?;
                                                                            c.sb(
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x3,
                                                                                    ),
                                                                                c.g[S3][0],
                                                                            );
                                                                            break 'b_3d115c;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[S1][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_3d1160;
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = ((c.g[S1][0] as i64)
                                                                < 24i64)
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[S5][0] as u32)
                                                                .wrapping_sub(0xbe0));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x003d1128,
                                                                    );
                                                                }
                                                                continue 'l_3d1128;
                                                            } else {
                                                                break 'b_3d116c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[S4][0];
                                    break 'b_3d1170;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x928;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d10d0);
                                    }
                                    continue 'l_3d10d0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x450000;
                                    c.sw((c.g[S5][0] as u32).wrapping_sub(0xbe0), 0u64);
                                    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0u64);
                                    break 'b_3d1188;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
            break 'b_3d118c;
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    });
}

/// `void BankMonitor_KillTimersByCallback(BankMonitor* mon, long key)`
/// (0x2adde0, reference `cMonitor_MONITOR_DestroyAll1`): walk the monitor's 48
/// instances (0x2c bytes each, from `mon + 0x98`) and reset (`BankMonitor_TimerKill`)
/// every one whose key matches `key`. A key matches when the halfword at +0xe equals
/// key bytes 2..4 and either that halfword is -1 (wildcard: compare the whole words
/// at +0x10 / key bytes 4..8) or the halfword at +0x10 equals key bytes 4..6;
/// otherwise when the words at +0x10 / key bytes 4..8 are equal.
pub fn bank_monitor_kill_timers_by_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0u64;
        frame.save_ra(c, 0x20);
        c.g[S2][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[A1][0]);
        'l_2ade08: loop {
            'b_2ade4c: {
                c.cycles += 8;
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(2, c.g[SP][0] as u32);
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x12));
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = 0u64;
                if t {
                    break 'b_2ade4c;
                } else {
                    'b_2ade3c: {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[S2][0];
                        c.g[V1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_2ade3c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x10));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2ade4c;
                            } else {
                                break 'b_2ade3c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x1;
                        break 'b_2ade4c;
                    } else {
                        break 'b_2ade4c;
                    }
                }
            }
            'b_2ade60: {
                c.cycles += 2;
                if c.g[A0][0] == 0u64 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_2ade60;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002ad940, 0x002ade5c)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_2ade60;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 48i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002ade08);
                }
                continue 'l_2ade08;
            } else {
                c.cycles += 6;
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP, SP as TEST_SP};
    use std::sync::atomic::AtomicUsize;

    /// Gated fakes (the suite runs tests in parallel): a fake runs only while
    /// this magic is in the machine's RAM, else the body its slot held.
    pub(crate) const OC768_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC768_MAGIC: u32 = 0x0768_c001;

    /// Per-callee forced results, one word per fake at `OC768_MAIL + (addr & 0xffffc)`.
    pub(crate) const OC768_MAIL: u32 = DATA + 0x30_0000;

    /// Result tables of the texture-slot fakes (8 words each: 0x19cce8 at +0,
    /// 0x19e238 at +0x20, 0x19e2b0 at +0x40), indexed by the id/slot argument & 7.
    pub(crate) const OC768_TABLES: u32 = OC768_CTRL + 0x100;

    pub(crate) fn oc768_mail(addr: u32) -> u32 {
        OC768_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (sign-extended).
    pub(crate) fn oc768_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc768_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (else the fake's pointer) unless it unwound.
    pub(crate) fn oc768_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let mail = c.bus.read32(oc768_mail(addr));
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let table = |c: &mut Cpu, base: u32| sx(c.bus.read32(OC768_TABLES + base + 4 * (a2 & 7)));
        match addr {
            0x0019_cce8 => c.g[V0][0] = table(c, 0),
            0x0019_e238 => c.g[V0][0] = table(c, 0x20),
            0x0019_e2b0 => c.g[V0][0] = table(c, 0x40),
            _ if mail != 0 => c.g[V0][0] = sx(mail.wrapping_sub(3)),
            _ => {}
        }
    }

    macro_rules! oc768_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC768_CTRL) != OC768_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc768_fake_body(c, $addr);
                }
            )*
            pub(crate) fn oc768_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc768_fakes!(
        (
            oc768_fake_14dc80,
            OC768_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc768_fake_14dd58,
            OC768_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc768_fake_14ad28,
            OC768_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc768_fake_19cce8,
            OC768_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8
        ),
        (
            oc768_fake_19e238,
            OC768_PREV_19E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238
        ),
        (
            oc768_fake_19e2b0,
            OC768_PREV_19E2B0,
            ee::s03::OVERRIDE_0019E2B0,
            ee::s03::f_0019e2b0__generated,
            0x0019_e2b0
        ),
        (
            oc768_fake_19dc20,
            OC768_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc768_fake_19cd38,
            OC768_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38
        ),
        (
            oc768_fake_19f908,
            OC768_PREV_19F908,
            ee::s03::OVERRIDE_0019F908,
            ee::s03::f_0019f908__generated,
            0x0019_f908
        ),
        (
            oc768_fake_3e6448,
            OC768_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc768_fake_2af7a0,
            OC768_PREV_2AF7A0,
            ee::s08::OVERRIDE_002AF7A0,
            ee::s08::f_002af7a0__generated,
            0x002a_f7a0
        ),
        (
            oc768_fake_2b4388,
            OC768_PREV_2B4388,
            ee::s08::OVERRIDE_002B4388,
            ee::s08::f_002b4388__generated,
            0x002b_4388
        ),
        (
            oc768_fake_3b8160,
            OC768_PREV_3B8160,
            ee::s13::OVERRIDE_003B8160,
            ee::s13::f_003b8160__generated,
            0x003b_8160
        ),
        (
            oc768_fake_288ae0,
            OC768_PREV_288AE0,
            ee::s07::OVERRIDE_00288AE0,
            ee::s07::f_00288ae0__generated,
            0x0028_8ae0
        ),
        (
            oc768_fake_28b1e8,
            OC768_PREV_28B1E8,
            ee::s07::OVERRIDE_0028B1E8,
            ee::s07::f_0028b1e8__generated,
            0x0028_b1e8
        ),
        (
            oc768_fake_144bc0,
            OC768_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x0014_4bc0
        ),
        (
            oc768_fake_285d98,
            OC768_PREV_285D98,
            ee::s07::OVERRIDE_00285D98,
            ee::s07::f_00285d98__generated,
            0x0028_5d98
        ),
        (
            oc768_fake_287968,
            OC768_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc768_fake_2a42c8,
            OC768_PREV_2A42C8,
            ee::s08::OVERRIDE_002A42C8,
            ee::s08::f_002a42c8__generated,
            0x002a_42c8
        ),
        (
            oc768_fake_2b3c28,
            OC768_PREV_2B3C28,
            ee::s08::OVERRIDE_002B3C28,
            ee::s08::f_002b3c28__generated,
            0x002b_3c28
        ),
        (
            oc768_fake_2b3cc0,
            OC768_PREV_2B3CC0,
            ee::s08::OVERRIDE_002B3CC0,
            ee::s08::f_002b3cc0__generated,
            0x002b_3cc0
        ),
        (
            oc768_fake_2b46d0,
            OC768_PREV_2B46D0,
            ee::s08::OVERRIDE_002B46D0,
            ee::s08::f_002b46d0__generated,
            0x002b_46d0
        ),
        (
            oc768_fake_2b4908,
            OC768_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908
        ),
        (
            oc768_fake_2b49c0,
            OC768_PREV_2B49C0,
            ee::s08::OVERRIDE_002B49C0,
            ee::s08::f_002b49c0__generated,
            0x002b_49c0
        ),
        (
            oc768_fake_289c18,
            OC768_PREV_289C18,
            ee::s07::OVERRIDE_00289C18,
            ee::s07::f_00289c18__generated,
            0x0028_9c18
        ),
        (
            oc768_fake_28b1c8,
            OC768_PREV_28B1C8,
            ee::s07::OVERRIDE_0028B1C8,
            ee::s07::f_0028b1c8__generated,
            0x0028_b1c8
        ),
        (
            oc768_fake_2906b8,
            OC768_PREV_2906B8,
            ee::s07::OVERRIDE_002906B8,
            ee::s07::f_002906b8__generated,
            0x0029_06b8
        ),
        (
            oc768_fake_295028,
            OC768_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc768_fake_2a3170,
            OC768_PREV_2A3170,
            ee::s08::OVERRIDE_002A3170,
            ee::s08::f_002a3170__generated,
            0x002a_3170
        ),
        (
            oc768_fake_2a4078,
            OC768_PREV_2A4078,
            ee::s08::OVERRIDE_002A4078,
            ee::s08::f_002a4078__generated,
            0x002a_4078
        ),
        (
            oc768_fake_2a40e8,
            OC768_PREV_2A40E8,
            ee::s08::OVERRIDE_002A40E8,
            ee::s08::f_002a40e8__generated,
            0x002a_40e8
        ),
        (
            oc768_fake_2b3a98,
            OC768_PREV_2B3A98,
            ee::s08::OVERRIDE_002B3A98,
            ee::s08::f_002b3a98__generated,
            0x002b_3a98
        ),
        (
            oc768_fake_416210,
            OC768_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc768_fake_3d7ec8,
            OC768_PREV_3D7EC8,
            ee::s14::OVERRIDE_003D7EC8,
            ee::s14::f_003d7ec8__generated,
            0x003d_7ec8
        ),
    );

    /// Gate on and the register state of a game-like machine; `calls` is the
    /// fake-call count unwinds are drawn from.
    fn oc768_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        oc768_install();
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC768_CTRL, OC768_MAGIC);
        r
    }

    // -- BankMonitor_KillTimersByCallback -----------------------------------

    /// 48 instances, each of one of five kinds against a key whose halfword is
    /// the wildcard -1 for one seed in four: both halfword and word matches,
    /// partial matches and misses. The real `BankMonitor_TimerKill` runs.
    #[test]
    fn bank_monitor_kill_timers_by_callback_matches() {
        check(
            ee::s08::f_002adde0__generated,
            bank_monitor_kill_timers_by_callback,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_dde0, 0);
                c.pc = 0x002a_dde0;
                let mon = DATA + 0x100;
                fill(c, &mut r, mon + 0x98, 48 * 0x2c / 4);
                let k2 = if seed % 4 == 0 {
                    0xffff
                } else {
                    r.next_u32() & 0xffff
                };
                let k4 = r.next_u32();
                for i in 0..48u32 {
                    let inst = mon + 0x98 + 0x2c * i;
                    let (h, w) = match (i + seed) % 5 {
                        0 => (k2, k4),
                        1 => (k2, (k4 & 0xffff) | (r.next_u32() << 16)),
                        2 => (k2, k4 ^ 1),
                        3 => (k2 ^ 0x100, k4),
                        _ => (k2, k4 ^ 0x8000_0000),
                    };
                    c.bus.write16(inst + 0xe, h as u16);
                    c.bus.write32(inst + 0x10, w);
                }
                c.g[A0][0] = sx(mon);
                c.g[A1][0] = (k4 as u64) << 32 | (k2 as u64) << 16 | (r.next_u32() & 0xffff) as u64;
            },
        );
    }

    // -- PATH_SetPaused ------------------------------------------------------

    pub(crate) const PSP_ENABLED: u32 = 0x0044_f42c;

    pub(crate) const PSP_CURRENT_MAP: u32 = 0x0044_f420;

    pub(crate) const PSP_CURRENT_ID: u32 = 0x0044_f430;

    pub(crate) const PSP_MAPS: u32 = 0x0051_7610;

    pub(crate) const PSP_MAP_SIZE: u32 = 0x928;

    /// Four path maps (id, loaded flag and 24 track pointers each, one in
    /// four empty), masks of all ones, 32-bit ones and random bits; the player
    /// off or a map already selected for a few seeds. The real
    /// `PATH_SelectMap` runs, `PATH_PlatSetPaused` is a gated fake.
    #[test]
    fn path_set_paused_matches() {
        covered(
            ee::s14::f_003d1038__generated,
            path_set_paused,
            120,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x003d_1038, 8);
                let enabled = if seed % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(PSP_ENABLED, enabled as u8);
                let id = if seed % 8 == 1 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(PSP_CURRENT_ID, id);
                c.bus.write32(PSP_CURRENT_MAP, r.next_u32());
                let mask = match seed % 3 {
                    0 => u64::MAX,
                    1 if seed % 7 == 1 => 0xffff_ffff,
                    _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                };
                c.g[A0][0] = mask;
                c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                for m in 0..4u32 {
                    let map = PSP_MAPS + m * PSP_MAP_SIZE;
                    let id = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0x00ff_ffff
                    } else {
                        r.next_u32() | 0x1100_0000
                    };
                    c.bus.write32(map + 0x30, id);
                    c.bus
                        .write32(map + 0x34, if r.next_u32() % 3 == 0 { 0 } else { 1 });
                    for j in 0..24u32 {
                        let track = DATA + 0x1000 + (m * 24 + j) * 0x80;
                        fill(c, &mut r, track, 0x20);
                        c.bus.write32(track + 0x54, r.next_u32() % 8);
                        let p = if r.next_u32() % 4 == 0 { 0 } else { track };
                        c.bus.write32(map + 0x58 + 4 * j, p);
                    }
                }
            },
        );
    }

    // -- SSXAudioSystem_SetupChannelGains --------------------------------

    /// Modes 0, 1 and others (one seed in three each), with the game image in
    /// RAM for the `.lit4` scaling constants and `SetChannelVolume`'s jump
    /// table; the real Get/SetChannelVolume run, the music player update and
    /// the master volume call they make are gated fakes (which unwind for
    /// some seeds).
    #[test]
    fn ssx_audio_system_setup_channel_gains_matches() {
        covered(
            ee::s07::f_00288d18__generated,
            ssx_audio_system_setup_channel_gains,
            45,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0028_8d18, 2);
                elf_image(c);
                let this = DATA + 0x100;
                for off in (0..0x6400u32).step_by(4) {
                    c.bus.write32(this + off, 0);
                }
                for k in 0..11u32 {
                    let v = r.edge_f32();
                    c.bus.write32(this + 0x62bc + 0x10 * k, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (r.next_u32() % 9) as u64,
                };
            },
        );
    }

    // -- FEModule_RiderPreviewPollTextures -------------------------------

    pub(crate) const POLL_THIS: u32 = DATA + 0x2000;

    pub(crate) const POLL_LIST: u32 = DATA + 0x3000;

    /// A rider with -1..5 bolts (ids that map to a slot, or -1, through the
    /// gated slot-table fake), all files loaded for two seeds in three, the
    /// "used" table on the stack pre-seeded (the memset fake does not clear
    /// it), a texture data pointer or none, and one seed in five already done.
    #[test]
    fn fe_module_rider_preview_poll_textures_matches() {
        covered(
            ee::s03::f_0019fd58__generated,
            fe_module_rider_preview_poll_textures,
            160,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0019_fd58, 14);
                let world = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, DATA + 0x1800);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                c.bus.write32(POLL_THIS, r.next_u32());
                c.bus.write32(POLL_THIS + 4, r.next_u32());
                let done = if seed % 5 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(POLL_THIS + 0xcb8, done);
                let n: i32 = [-1, 0, 1, 2, 3, 4, 5][(seed / 5 % 7) as usize];
                c.bus.write32(POLL_LIST + 0x28c, n as u32);
                for i in 0..5u32 {
                    let flags =
                        (r.next_u32() & 0xe0) | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus
                        .write16(POLL_LIST + 0x290 + 4 * i, r.next_u32() as u16);
                    c.bus.write16(POLL_LIST + 0x292 + 4 * i, flags as u16);
                }
                oc768_force(c, 0x0014_ad28, POLL_LIST);
                for k in 0..8u32 {
                    let slot = if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(OC768_TABLES + 4 * k, slot);
                    let loaded = if seed % 3 != 0 { 1 } else { r.next_u32() & 1 };
                    c.bus.write32(OC768_TABLES + 0x20 + 4 * k, loaded);
                    let data = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x4000 + 0x10 * k
                    };
                    c.bus.write32(OC768_TABLES + 0x40 + 4 * k, data);
                    let used = (r.next_u32() % 3 == 0) as u32;
                    c.bus.write32(TEST_SP - 0x490 + 4 * k, used);
                }
                c.g[A0][0] = sx(POLL_THIS);
            },
        );
    }

    // -- SSXAudioSystem_PlayMusic ---------------------------------------------

    pub(crate) const PLAY_THIS: u32 = DATA + 0x10000;

    pub(crate) const PLAY_VT: u32 = DATA + 0x20000;

    pub(crate) const PLAY_LOC: u32 = DATA + 0x30000;

    /// Replay playing or not, radio mode or not, locations 0..21 (peak theme
    /// from 17 up), request line / skip flag / race-type query combinations,
    /// the three peak categories (the real `TrackToCategory` with the game
    /// image) and both copy loops (aligned and unaligned location data). The
    /// sound queue of the real `SetTrackedSoundExpiry`-like callees stays zero.
    #[test]
    fn ssx_audio_system_play_music_matches() {
        covered(
            ee::s07::f_0028cf98__generated,
            ssx_audio_system_play_music,
            240,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0028_cf98, 30);
                elf_image(c);
                let bits = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b1);
                let this = PLAY_THIS;
                oc768_force(c, 0x0028_8ae0, (seed % 3 == 0) as u32);
                c.bus.write32(this + 0x6294, (bits >> 3) & 1);
                c.bus.write32(
                    this + 0x608c,
                    if (bits >> 4) % 3 == 0 {
                        2
                    } else {
                        (bits >> 8) & 1
                    },
                );
                c.bus.write32(this + 0x508, r.next_u32());
                c.bus.write32(this + 0x5558, PLAY_VT);
                c.bus
                    .write16(PLAY_VT + 0x10, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(PLAY_VT + 0x14, fakes::FAKE + 0x10);
                c.bus
                    .write16(PLAY_VT + 0x20, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(PLAY_VT + 0x24, fakes::FAKE + 0x20);
                let loc = PLAY_LOC + if (bits >> 12) & 1 == 1 { 4 } else { 0 };
                fill(c, &mut r, PLAY_LOC, 0x20);
                c.bus.write32(loc, r.next_u32() % 22);
                oc768_force(c, 0x0014_4bc0, loc);
                oc768_force(c, 0x002a_42c8, (bits >> 13) & 1);
                oc768_force(c, 0x0028_9c18, (bits >> 14) % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 0x40) as u64;
                c.g[A2][0] = ((bits >> 16) & 1) as u64;
                c.g[A3][0] = match (bits >> 17) & 3 {
                    0 => u64::MAX,
                    n => n as u64,
                };
                c.g[T0][0] = ((bits >> 19) & 1) as u64;
            },
        );
    }

    // -- SSXAudioSystem_OnStartGateOpen ---------------------------------------

    pub(crate) const GATE_THIS: u32 = DATA + 0x10000;

    pub(crate) const GATE_OBJ: u32 = DATA + 0x40000;

    /// The sound object (`self+0`) with its stream arrays (volume byte below,
    /// at or above 127 or negative), replay on or off, the music-started and
    /// PA flags, the game path byte (1, 2, other) and the game mode's heat and
    /// PA-speech flag. The real `PASay20C3` (PA speech off), `CancelPendingInstance`
    /// and `SetTrackedSoundExpiry` (empty queue) run; everything else is a gated fake.
    #[test]
    fn ssx_audio_system_on_start_gate_open_matches() {
        covered(
            ee::s08::f_0029c7b0__generated,
            ssx_audio_system_on_start_gate_open,
            240,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0029_c7b0, 25);
                let bits = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b1);
                let this = GATE_THIS;
                let obj = GATE_OBJ;
                fill(c, &mut r, obj, 0x80);
                let s1 = obj + 0x1d8;
                for off in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ] {
                    let area = DATA + 0x50000 + off * 0x100;
                    c.bus.write32(s1 + off, area);
                    fill(c, &mut r, area, 0x40);
                }
                // `obj+0x1f4` is both the stream counter (incremented by the
                // function) and, as `s1+0x1c`, the volume array index.
                let volume = [-5i32 as u32, 100, 127, 200][((bits >> 2) & 3) as usize];
                for k in 0..8u32 {
                    c.bus.write32(DATA + 0x50000 + 0x38 * 0x100 + 4 * k, volume);
                }
                c.bus.write32(obj + 0x1f4, (bits >> 4) % 3);
                c.bus.write32(obj + 0x260, r.next_u32());
                c.bus.write32(this, obj);
                c.bus.write32(this + 4, DATA + 0x61000);
                c.bus.write32(this + 8, DATA + 0x62000);
                c.bus.write32(this + 0x118, DATA + 0x63000);
                c.bus.write32(DATA + 0x63000, obj);
                // The game module (`28b1c8`) and the callbacks' vtables.
                let module = DATA + 0x70000;
                c.bus.write32(module + 0xc, module + 0x100);
                c.bus.write32(module + 0x100 + 8, r.next_u32());
                oc768_force(c, 0x0028_b1c8, module);
                let vt1 = DATA + 0x71000;
                c.bus.write32(this + 0xc, vt1);
                c.bus.write16(vt1 + 8, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(vt1 + 0xc, fakes::FAKE + 0x10);
                let vt2 = DATA + 0x72000;
                c.bus.write32(this + 0x5558, vt2);
                c.bus.write16(vt2 + 0x20, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(vt2 + 0x24, fakes::FAKE + 0x20);
                // The world's game mode manager.
                let world = DATA + 0x73000;
                let manager = DATA + 0x73100;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0xc0, manager);
                c.bus
                    .write32(manager + 0x98, if (bits >> 7) & 1 == 1 { 1 } else { 0 });
                c.bus.write32(manager, (bits >> 8) & 3);
                c.bus
                    .write8(0x0053_5c11, [1u8, 2, 0][((bits >> 10) % 3) as usize]);
                // The speech instance the PA branch cancels.
                c.bus.write32(this + 0x5564, DATA + 0x74000);
                c.bus.write32(DATA + 0x74000, DATA + 0x74100);
                c.bus.write32(this + 0x627c, (bits >> 12) & 1);
                c.bus.write32(this + 0x6298, (bits >> 13) & 1);
                oc768_force(c, 0x0028_8ae0, (bits >> 14) & 1);
                oc768_force(c, 0x0029_5028, (bits >> 15) & 1);
                oc768_force(c, 0x002a_4078, (bits >> 16) & 1);
                oc768_force(c, 0x002a_40e8, (bits >> 17) & 1);
                oc768_force(c, 0x003d_7ec8, (bits >> 18) & 1);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
