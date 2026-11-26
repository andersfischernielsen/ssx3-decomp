//! Staging module oc773 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc773`.

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
        0x0029_a220,
        ssx_audio_system_on_rider_reset,
        "SSXAudioSystem_OnRiderReset",
    ),
    (0x002a_7678, crowd_audio_add_source, "CrowdAudio_AddSource"),
    (
        0x001a_38c0,
        fe_async_manager_cancel,
        "FEAsyncManager_Cancel",
    ),
    (0x002b_07f8, speech_system_ctor, "SpeechSystem_ctor"),
    (
        0x0019_1938,
        cfe_state_rule_select_init,
        "cFEStateRuleSelect_Init",
    ),
];

/// `void SSXAudioSystem_OnRiderReset(SSXAudioSystem* sys, Rider* rider)` (0x29a220).
///
/// A rider was reset (`cSSXAudio_PlayerResetEvent`): when the rider is human
/// and local (`+0x874`, `+0x87c > 0`) and not in replay mode, adds the voice
/// 123 entry to the world's voice tables (at `world+0x1d8`, the heavy block);
/// ends the wipeout slide and tumble sounds, turns the boost sound off when
/// `sys+0x59e4 > 0`, and when `rider` is the focus rider calls
/// `SSXAudioSystem_ResetMusicFadeIn` (0x28f108).
pub fn ssx_audio_system_on_rider_reset(c: &mut Cpu) {
    guest(c, |c| {
        'b_29a258: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V1][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x874));
            let t = c.g[V0][0] == 0u64;
            c.g[S3][0] = c.g[A0][0];
            if t {
                break 'b_29a258;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x87c));
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                break 'b_29a258;
            }
        }
        'b_29a4c8: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = c.g[S3][0];
            if t {
                break 'b_29a4c8;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x00288ae0, 0x0029a268)?;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.g[S3][0];
                    break 'b_29a4c8;
                } else {
                    c.cycles += 150;
                    c.g[S0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[T1][0] = 0x18;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[S2][0] = 0x7b;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, 3, 3, 9);
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
                    c.mult(0, 4, 4, 9);
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
                    call_ptr(c, 0x00416210, 0x0029a348)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A3][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[T0][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[T1][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T2][0] = 0x7f;
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[T3][0] = 0x1;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0x5;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[A3][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[T2][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[T3][0]);
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x00287968, 0x0029a4a4)?;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x002906b8, 0x0029a4c4)?;
                    c.g[A0][0] = c.g[S3][0];
                    break 'b_29a4c8;
                }
            }
        }
        'b_29a4f0: {
            c.cycles += 8;
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x00296e20, 0x0029a4d0)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x00297438, 0x0029a4dc)?;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59e4));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = c.g[S3][0];
            if t {
                break 'b_29a4f0;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x002992d8, 0x0029a4f0)?;
                break 'b_29a4f0;
            }
        }
        'b_29a514: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0xffffffffffffffff;
            call_ptr(c, 0x00285d98, 0x0029a4fc)?;
            let t = c.g[S4][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_29a514;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x0028f108, 0x0029a510)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_29a514;
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

/// `int CrowdAudio_AddSource(CrowdAudio* self, int ambient_patch)` (0x2a7678).
///
/// Adds a crowd source to the crowd instance of the current viewport
/// (`CrowdAudio_InstanceAddSource`). Returns -1 without crowd instances
/// (`self+0x18 == 0`); otherwise the packed id `(id << 8) | viewport` when the
/// instance accepted it (id >= 0), else the negative result. The bank monitor's
/// instance counter (`+0x1f4` of the sound system) is decremented on exit.
pub fn crowd_audio_add_source(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a7704: {
            'b_2a769c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                if c.g[T0][0] != 0u64 {
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    break 'b_2a769c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_2a7704;
                    } else {
                        break 'b_2a769c;
                    }
                }
            }
            'b_2a76f4: {
                c.cycles += 14;
                c.g[A3][0] = 0x190;
                c.g[A2][0] = c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[A1][0] = c.g[V1][0];
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.mult(0, 2, 4, 7);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                call_ptr(c, 0x002a5d08, 0x002a76cc)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[A1][0] = sx((c.g[V0][0] as u32) << 8);
                if t {
                    break 'b_2a76f4;
                } else {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
                    break 'b_2a76f4;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1f4), c.g[V1][0]);
            break 'b_2a7704;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `void FEAsyncManager_Cancel(FEAsyncManager* self, int idx)` (0x1a38c0).
///
/// Cancels the async file `idx` by its state: 1 (queued) just resets the state
/// to 0; 2 (loading) releases the async handle, frees the data buffer and resets
/// the state; 3 (loaded) records the frame count + 14 for the entry (`+0x118`)
/// and resets the state. Other states do nothing.
pub fn fe_async_manager_cancel(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a39a8: {
            'b_1a3954: {
                'b_1a393c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S2][0] = c.g[A1][0];
                    call_ptr(c, 0x001a39c0, 0x001a38e0)?;
                    c.g[V1][0] = 0x1;
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_1a393c;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x001a39c0, 0x001a38f4)?;
                        c.g[V1][0] = 0x2;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_1a3954;
                        } else {
                            c.cycles += 12;
                            c.g[S0][0] = 0x11c;
                            c.g[A1][0] = 0u64;
                            c.mult(0, 16, 18, 16);
                            c.g[A2][0] = 0u64;
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x104));
                            call_ptr(c, 0x003df9d8, 0x001a391c)?;
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            if c.g[A0][0] == 0u64 {
                                c.sw(c.g[S0][0] as u32, 0u64);
                                break 'b_1a393c;
                            } else {
                                c.cycles += 3;
                                call_ptr(c, 0x00317e98, 0x001a3938)?;
                                c.sw(c.g[S0][0] as u32, 0u64);
                                break 'b_1a393c;
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x001a39d8, 0x001a394c)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1a39a8;
                } else {
                    break 'b_1a3954;
                }
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x001a39c0, 0x001a395c)?;
            c.g[V1][0] = 0x3;
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1a39a8;
            } else {
                c.cycles += 16;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001a3980)?;
                c.g[V1][0] = 0x11c;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe));
                c.mult(0, 4, 18, 3);
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0u64;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A0][0] = c.g[S1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                call_ptr(c, 0x001a39d8, 0x001a39a4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1a39a8;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `SpeechSystem* SpeechSystem_ctor(SpeechSystem* self, int in_charge, ...)` (0x2b07f8).
///
/// Constructs the speech system: with `in_charge` builds the embedded sound
/// system, bank manager and stream system (`SoundManagerRef_ctor`,
/// `StreamArray_ctor`); installs the vtables (for a derived object it first
/// copies the vtables to the stack with adjusted deltas); clears the callback
/// fields, registers itself in the global at `gp+0x6c4`, allocates the speech
/// instance array, clears the event contexts and copies the five big-file
/// names.
pub fn speech_system_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b087c: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = c.g[T0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = c.g[A3][0];
            c.g[S4][0] = c.g[A1][0];
            let t = c.g[S4][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            if t {
                break 'b_2b087c;
            } else {
                c.cycles += 20;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc98));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
                c.g[A0][0] = c.g[V0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc98), c.g[V1][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw(c.g[S0][0] as u32, c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3a0), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x002b0850)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc98));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0028b9b8, 0x002b0864)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x002aa490, 0x002b087c)?;
                break 'b_2b087c;
            }
        }
        'b_2b0970: {
            c.cycles += 17;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x38a0));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x38c0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[S4][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2b0970;
            } else {
                c.cycles += 44;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x38a0));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x38c0));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c0));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
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
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1c8));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                break 'b_2b0970;
            }
        }
        c.cycles += 16;
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3870));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3308));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.g[A0][0] = 0x8;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x6c4), c.g[S0][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002b09a8)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        'l_2b09b0: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
            let t = c.g[A0][0] != 0u64;
            c.sw(c.g[V1][0] as u32, 0u64);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002b09b0);
                }
                continue 'l_2b09b0;
            } else {
                c.cycles += 71;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002b1428, 0x002b09d4)?;
                c.g[V0][0] = 0x480000;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3318));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[A2][0] as u32);
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x15f));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x158));
                c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x167));
                c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x160));
                c.sb((c.g[S0][0] as u32).wrapping_add(0x168), c.g[A1][0]);
                c.g[V1][0] = 0x480000;
                c.g[V0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3330));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A2][0] as u32);
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x173));
                c.sdr(7, (c.g[S0][0] as u32).wrapping_add(0x16c));
                c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x17b));
                c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x174));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x17c), c.g[A1][0]);
                c.g[A0][0] = 0x480000;
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3348));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A2][0] as u32);
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x187));
                c.sdr(7, (c.g[S0][0] as u32).wrapping_add(0x180));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x18f));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x188));
                c.sb((c.g[S0][0] as u32).wrapping_add(0x190), c.g[A1][0]);
                c.g[V1][0] = 0x480000;
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3360));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A2][0] as u32);
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x19b));
                c.sdr(7, (c.g[S0][0] as u32).wrapping_add(0x194));
                c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x1a3));
                c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x19c));
                c.sb((c.g[S0][0] as u32).wrapping_add(0x1a4), c.g[A1][0]);
                c.g[A0][0] = 0x480000;
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3378));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[A2][0] as u32);
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x1af));
                c.sdr(7, (c.g[S0][0] as u32).wrapping_add(0x1a8));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1b7));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x1b0));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x1b8), c.g[A1][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void cFEStateRuleSelect_Init(cFEStateRuleSelect* self)` (0x191938).
///
/// Opens the "my rules" screen, picks the rule table and rule count of the
/// current event type, fills the rule list and highlights/enables the rows
/// (`FillRuleList`, per-row screen item flags and texts).
pub fn cfe_state_rule_select_init(c: &mut Cpu) {
    if c.pc != 0x00191938 {
        return ee::s02::f_00191938__entries(c);
    }
    guest(c, |c| 'l_191938: loop {
        'b_1919a0: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = 0x460000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00191974)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00191988)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1919a0;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001919a0)?;
                break 'b_1919a0;
            }
        }
        'b_191a38: {
            c.cycles += 11;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
            c.g[S4][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x001919ac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001919b8)?;
            c.g[V1][0] = 0x530000;
            c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c12));
            c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            if t {
                break 'b_191a38;
            } else {
                'b_191a34: {
                    'b_191a2c: {
                        'b_191a1c: {
                            'b_191a10: {
                                'b_191a04: {
                                    'b_1919f4: {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00191938 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00191938);
                                                }
                                                continue 'l_191938;
                                            }
                                            0x001919e4 => {
                                                c.cycles += 4;
                                                c.g[V0][0] = 0x440000;
                                                c.g[S4][0] = 0xb;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1098));
                                                if t {
                                                    break 'b_191a34;
                                                } else {
                                                    break 'b_1919f4;
                                                }
                                            }
                                            0x001919f4 => {
                                                break 'b_1919f4;
                                            }
                                            0x00191a04 => {
                                                break 'b_191a04;
                                            }
                                            0x00191a10 => {
                                                break 'b_191a10;
                                            }
                                            0x00191a1c => {
                                                break 'b_191a1c;
                                            }
                                            0x00191a2c => {
                                                break 'b_191a2c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00191938, 0x00191c44);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x440000;
                                    c.g[S4][0] = 0xa;
                                    let t = true;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10a8));
                                    if t {
                                        break 'b_191a34;
                                    } else {
                                        break 'b_191a04;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a60));
                                let t = true;
                                c.g[S4][0] = 0x8;
                                if t {
                                    break 'b_191a34;
                                } else {
                                    break 'b_191a10;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a58));
                            let t = true;
                            c.g[S4][0] = 0x8;
                            if t {
                                break 'b_191a34;
                            } else {
                                break 'b_191a1c;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = 0x440000;
                        c.g[S4][0] = 0xb;
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10b8));
                        if t {
                            break 'b_191a34;
                        } else {
                            break 'b_191a2c;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a50));
                    c.g[S4][0] = 0x8;
                    break 'b_191a34;
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                break 'b_191a38;
            }
        }
        c.cycles += 4;
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0xe;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5f));
        'l_191a48: loop {
            c.cycles += 7;
            c.sb(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00191a48);
                }
                continue 'l_191a48;
            } else {
                'b_191b18: {
                    c.cycles += 18;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                    c.g[A0][0] = 0x530000;
                    c.g[A2][0] = 0x20;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x003e6448, 0x00191a7c)?;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001913b8, 0x00191a88)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                    call_ptr(c, 0x00317670, 0x00191a94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191aa0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_191b18;
                    } else {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = c.g[S4][0];
                        c.g[V0][0] = c.g[V0][0] | 0x80;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        call_ptr(c, 0x0039b760, 0x00191ac0)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0039b960, 0x00191acc)?;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e0));
                        call_ptr(c, 0x00317670, 0x00191ad8)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191ae4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        let t = c.g[S0][0] == 0u64;
                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                        if t {
                            break 'b_191b18;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0x440000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1058));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            call_ptr(c, 0x00317670, 0x00191b0c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00191b18)?;
                            break 'b_191b18;
                        }
                    }
                }
                c.cycles += 18;
                c.g[V0][0] = 0x4a0000;
                c.g[S2][0] = 0u64;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
                c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x2));
                c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                c.g[S6][0] = 0xfffffffffffffffe;
                c.g[V0][0] = 0x4a0000;
                c.g[S3][0] = 0x1000000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
                c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                c.g[S5][0] = 0x1000000;
                'l_191b60: loop {
                    'b_191bd8: {
                        c.cycles += 17;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00191b68)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191b74)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = c.g[S7][0];
                        c.g[V1][0] = c.g[V1][0] & c.g[S6][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                        call_ptr(c, 0x00317670, 0x00191b88)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191b94)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        if t {
                            break 'b_191bd8;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00191bbc)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00191bd4)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                            break 'b_191bd8;
                        }
                    }
                    'b_191c04: {
                        'b_191bf8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_191bf8;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lbu(c.g[SP][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                let t = true;
                                c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                                if t {
                                    break 'b_191c04;
                                } else {
                                    break 'b_191bf8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        break 'b_191c04;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[S2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 11i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00191b60);
                        }
                        continue 'l_191b60;
                    } else {
                        c.cycles += 11;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, put_words, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc773's callee fakes in a machine's RAM.
    pub(crate) const OC773_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC773_MAGIC: u32 = 0x0773_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc773_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc773_arm(c: &mut Cpu) {
        c.bus.write32(OC773_CTRL, OC773_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc773_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc773_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc773_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc773_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc773_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc773_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => sx(m - 3),
            };
        }
    }

    macro_rules! oc773_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC773_CTRL) != OC773_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc773_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc773_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc773_fakes!(
        (
            oc773_fk_288ae0,
            OC773_PREV_288AE0,
            ee::s07::OVERRIDE_00288AE0,
            ee::s07::f_00288ae0__generated,
            0x0028_8ae0
        ),
        (
            oc773_fk_285d98,
            OC773_PREV_285D98,
            ee::s07::OVERRIDE_00285D98,
            ee::s07::f_00285d98__generated,
            0x0028_5d98
        ),
        (
            oc773_fk_287968,
            OC773_PREV_287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc773_fk_2906b8,
            OC773_PREV_2906B8,
            ee::s07::OVERRIDE_002906B8,
            ee::s07::f_002906b8__generated,
            0x0029_06b8
        ),
        (
            oc773_fk_289c18,
            OC773_PREV_289C18,
            ee::s07::OVERRIDE_00289C18,
            ee::s07::f_00289c18__generated,
            0x0028_9c18
        ),
        (
            oc773_fk_28a058,
            OC773_PREV_28A058,
            ee::s07::OVERRIDE_0028A058,
            ee::s07::f_0028a058__generated,
            0x0028_a058
        ),
        (
            oc773_fk_28b248,
            OC773_PREV_28B248,
            ee::s07::OVERRIDE_0028B248,
            ee::s07::f_0028b248__generated,
            0x0028_b248
        ),
        (
            oc773_fk_296e20,
            OC773_PREV_296E20,
            ee::s08::OVERRIDE_00296E20,
            ee::s08::f_00296e20__generated,
            0x0029_6e20
        ),
        (
            oc773_fk_297438,
            OC773_PREV_297438,
            ee::s08::OVERRIDE_00297438,
            ee::s08::f_00297438__generated,
            0x0029_7438
        ),
        (
            oc773_fk_2992d8,
            OC773_PREV_2992D8,
            ee::s08::OVERRIDE_002992D8,
            ee::s08::f_002992d8__generated,
            0x0029_92d8
        ),
        (
            oc773_fk_2ade88,
            OC773_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc773_fk_416210,
            OC773_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc773_fk_1a39c0,
            OC773_PREV_1A39C0,
            ee::s03::OVERRIDE_001A39C0,
            ee::s03::f_001a39c0__generated,
            0x001a_39c0
        ),
        (
            oc773_fk_1a39d8,
            OC773_PREV_1A39D8,
            ee::s03::OVERRIDE_001A39D8,
            ee::s03::f_001a39d8__generated,
            0x001a_39d8
        ),
        (
            oc773_fk_3df9d8,
            OC773_PREV_3DF9D8,
            ee::s14::OVERRIDE_003DF9D8,
            ee::s14::f_003df9d8__generated,
            0x003d_f9d8
        ),
        (
            oc773_fk_3e6448,
            OC773_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc773_fk_317e98,
            OC773_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc773_fk_317e30,
            OC773_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc773_fk_317670,
            OC773_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc773_fk_397cc0,
            OC773_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc773_fk_39c870,
            OC773_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc773_fk_39d860,
            OC773_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc773_fk_39b760,
            OC773_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc773_fk_39b960,
            OC773_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc773_fk_3a1000,
            OC773_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc773_fk_39a670,
            OC773_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039_a670
        ),
        (
            oc773_fk_39a568,
            OC773_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc773_fk_14dc80,
            OC773_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc773_fk_14dd58,
            OC773_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
    );

    /// `FEAsyncManager_Cancel`: all four states (queued, loading with and
    /// without a data buffer, loaded, other), the frame-count method call, and
    /// the unwinds of its calls.
    #[test]
    fn fe_async_manager_cancel_matches() {
        oc773_install_fakes();
        fakes::covered(
            ee::s03::f_001a38c0__generated,
            fe_async_manager_cancel,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x773a, 5);
                oc773_arm(c);
                c.pc = 0x001a_38c0;
                let this = DATA + 0x1000;
                let idx = seed % 3;
                let state = [1u32, 2, 3, 9][(seed as usize / 3) % 4];
                for n in 0..4 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * n, state);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(idx);
                // The file's async handle (+0x104) and data buffer (+0x100).
                let rec = this + 0x11c * idx;
                c.bus.write32(rec + 0x104, 0x0012_3400 + idx);
                c.bus
                    .write32(rec + 0x100, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                // GrMan (gp-0x854) and its frame-count method.
                let grman = DATA + 0x3000;
                let inner = DATA + 0x4000;
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0x10d8, inner);
                c.bus.write16(inner + 0x390, (seed * 4) as u16);
                c.bus.write32(inner + 0x394, fakes::FAKE);
            },
        );
    }

    /// `CrowdAudio_AddSource`: no crowd instances, an instance that refuses
    /// (15 or more sources), a bank slot that is empty, and a free slot that
    /// takes the source (a match on the patch id, instance count 0 or 3).
    #[test]
    fn crowd_audio_add_source_matches() {
        check(
            ee::s08::f_002a7678__generated,
            crowd_audio_add_source,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x773b, 0);
                c.pc = 0x002a_7678;
                let this = DATA + 0x1000;
                let sys = DATA + 0x2000;
                let bm = sys + 0x1d8;
                let vports = DATA + 0x3000;
                let slots = DATA + 0x4000;
                let instances = DATA + 0x8000;
                let patch = r.next_u32() | 1;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(patch);
                c.bus.write32(this, sys);
                c.bus
                    .write32(this + 0x18, if seed % 5 == 0 { 0 } else { instances });
                let minst = seed % 3;
                let viewport = (seed / 3) % 2;
                c.bus.write32(bm + 0x1c, minst);
                c.bus.write32(bm + 0x64, vports);
                c.bus.write32(vports + 4 * minst, viewport);
                // The bank monitor's per-instance slot-table pointers.
                c.bus
                    .write32(bm + 0x24 + 4 * minst, if seed % 7 == 0 { 0 } else { slots });
                c.bus.write32(bm + 0x38, DATA + 0x5000);
                c.bus.write32(bm + 0x3c, DATA + 0x5400);
                let inst = instances + 0x190 * viewport;
                let count = match seed % 4 {
                    0 => 20,
                    1 => 0,
                    _ => 3,
                };
                // The first few of the instance's 15 source slots are busy.
                for i in 0..(seed % 5) {
                    c.bus.write32(inst + 0x20 + 0xc * i, 1);
                }
                c.bus.write32(inst + 0xd4, count);
                c.bus.write32(inst + 0xe0, patch);
            },
        );
    }

    /// `SSXAudioSystem_OnRiderReset`: human/local or not, replay mode or not
    /// (the voice is added, with its voice-table writes), turbo on or off,
    /// focus rider or not, and the unwinds of its calls.
    #[test]
    fn ssx_audio_system_on_rider_reset_matches() {
        oc773_install_fakes();
        fakes::covered(
            ee::s08::f_0029a220__generated,
            ssx_audio_system_on_rider_reset,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x773c, 9);
                oc773_arm(c);
                c.pc = 0x0029_a220;
                let sys = DATA + 0x1000;
                let rider = DATA + 0x30000;
                let world = DATA + 0x10000;
                let s1 = world + 0x1d8;
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(rider);
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(rider + 0x87c, if seed % 5 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(sys, world);
                c.bus.write32(sys + 4, DATA + 0x20000);
                c.bus.write32(sys + 8, DATA + 0x20400);
                c.bus.write32(sys + 0x118, DATA + 0x20800);
                c.bus.write32(DATA + 0x20800, world);
                c.bus.write32(world + 0x1f4, seed % 4);
                // The voice tables the heavy path fills (one array per field).
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(s1 + off, DATA + 0x40000 + 0x400 * k as u32);
                }
                c.bus.write32(s1 + 0x20, DATA + 0x50000);
                c.bus.write32(world + 0x214, DATA + 0x21000);
                for i in 0..6 {
                    c.bus.write32(s1 + 4 + 4 * i, r.next_u32());
                }
                c.bus
                    .write32(sys + 0x59e4, if seed % 2 == 0 { 0 } else { 2 });
                if seed % 4 != 0 {
                    oc773_force_zero(c, 0x0028_8ae0);
                }
                if seed % 3 == 1 {
                    oc773_force(c, 0x0028_5d98, rider);
                }
                if seed % 6 == 1 {
                    oc773_force_zero(c, 0x0028_9c18);
                }
            },
        );
    }

    /// `SpeechSystem_ctor`: in charge (builds the sound, bank and stream
    /// systems, with zero banks and streams) and not in charge (copies the
    /// vtables to the stack with adjusted deltas), and the unwinds.
    #[test]
    fn speech_system_ctor_matches() {
        oc773_install_fakes();
        fakes::covered(
            ee::s08::f_002b07f8__generated,
            speech_system_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x773d, 8);
                oc773_arm(c);
                c.pc = 0x002b_07f8;
                let this = DATA + 0x10000;
                let in_charge = seed % 2;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(in_charge);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = sx(DATA + 0x2000);
                if in_charge == 0 {
                    // `this->streams -> bank manager -> bank monitor`, and the
                    // vtable images the constructor copies.
                    let streams = DATA + 0x20000;
                    let banks = DATA + 0x21000;
                    c.bus.write32(this, streams);
                    c.bus.write32(streams, banks);
                    c.bus.write32(banks + 0x1d8, DATA + 0x23000);
                    for i in 0..0x10 {
                        c.bus.write32(0x0048_38a0 + 4 * i, r.next_u32());
                        c.bus.write32(0x0048_38c0 + 4 * i, r.next_u32());
                    }
                }
                for i in 0..0x20 {
                    c.bus.write32(0x0048_3318 + 4 * i, r.next_u32());
                }
                c.bus.write32(GAME_GP + 0x6c4, r.next_u32());
            },
        );
    }

    /// `cFEStateRuleSelect_Init`: every event type (the jump table: 6 cases,
    /// out-of-range types and a corrupt table entry), a missing screen or
    /// rule row, and the unwinds of its many calls.
    #[test]
    fn cfe_state_rule_select_init_matches() {
        oc773_install_fakes();
        let table: [u32; 6] = [
            0x0019_19e4,
            0x0019_19f4,
            0x0019_1a10,
            0x0019_1a04,
            0x0019_1a1c,
            0x0019_1a2c,
        ];
        fakes::covered(
            ee::s02::f_00191938__generated,
            cfe_state_rule_select_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x773e, 40);
                oc773_arm(c);
                c.pc = 0x0019_1938;
                let this = DATA + 0x10000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, DATA + 0x1000);
                put_words(c, 0x0045_fb30, &table);
                let event = seed % 8;
                c.bus.write8(0x0053_5c12, event as u8);
                if seed % 29 == 0 && event < 6 {
                    c.bus.write32(0x0045_fb30 + 4 * event, 0x0019_1900);
                }
                if seed % 5 == 1 {
                    oc773_force_zero(c, 0x0039_7cc0);
                }
                if seed % 7 == 2 {
                    oc773_force_zero(c, 0x0039_d860);
                }
                // Rule strings / selection bytes the row loops read.
                for i in 0..0x40 {
                    c.bus.write8(0x004a_16a8 + i, (r.next_u32() % 12) as u8);
                    c.bus.write8(0x004a_13a0 + i, (r.next_u32() % 12) as u8);
                }
            },
        );
    }
}
