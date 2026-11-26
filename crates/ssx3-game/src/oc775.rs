//! Staging module oc775 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc775`.

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
    (0x003d_5330, path_count_tracks, "PATH_CountTracks"),
    (
        0x002a_a910,
        stream_array_create_stream_default_params,
        "StreamArray_CreateStreamDefaultParams",
    ),
    (
        0x002a_b200,
        ssx_audio_system_base_ctor,
        "SSXAudioSystem_BaseCtor",
    ),
    // Font_ctor (0x3915e8) is already hand-written in
    // `game::render::lightman::font_ctor` but was never registered: it gets
    // its entry (and its test) here; the reorg moves the entry to lightman.
    (
        0x0039_15e8,
        crate::game::render::lightman::font_ctor,
        "Font_ctor",
    ),
];

/// `int PATH_CountTracks(unsigned mask)` (0x3d5330): counts the non-zero words
/// among the 24 slots of the (up to four) path tables selected by bits 24..27 of
/// `mask`. Table `i` lives at `0x517644 + i * 0x928`: word 0 is the "in use"
/// flag, words 9.. (offset 0x24) are the 24 slots.
pub fn path_count_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x510000;
        c.g[A3][0] = 0u64;
        c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
        c.g[A1][0] = 0u64;
        c.g[T1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x24));
        c.g[T4][0] = 0x1000000;
        c.g[T3][0] = 0x928;
        c.g[T2][0] = 0x928;
        c.g[V0][0] = sx((c.g[T4][0] as u32) << ((c.g[A1][0] as u32) & 31));
        'l_3d5358: loop {
            'b_3d539c: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                let t = c.g[V0][0] == 0u64;
                c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if t {
                    break 'b_3d539c;
                } else {
                    c.cycles += 5;
                    c.mult(0, 3, 5, 11);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.mult(0, 2, 5, 10);
                    if t {
                        break 'b_3d539c;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = 0x17;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                        'l_3d5380: loop {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[A2][0] as i64) >= 0;
                            if c.g[V1][0] != 0 {
                                c.g[A3][0] = c.g[V0][0];
                            }
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003d5380);
                                }
                                continue 'l_3d5380;
                            } else {
                                break 'b_3d539c;
                            }
                        }
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[T0][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx((c.g[T4][0] as u32) << ((c.g[A1][0] as u32) & 31));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003d5358);
                }
                continue 'l_3d5358;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[A3][0];
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void StreamArray_CreateStreamDefaultParams(StreamSys** self, a1, a2, a3, t0)`
/// (0x2aa910): pushes a fresh bank-monitor instance (`mInst` at +0x1f4 of the
/// sys object, parameter arrays at the `+0x1d8` block), copies the default
/// descriptor from `sys+0x1dc` (24 bytes), clears its per-instance fields,
/// sets the delay to 100/90/50, the volume map to 127 and close-on-end to 1,
/// stores `a2` as the volume-control pointer, calls `StreamArray_CreateStream`
/// (0x2aa7f0) with `(self, a1, a3, t0)` and pops the instance again.
pub fn stream_array_create_stream_default_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 162;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[T1][0] = 0x18;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[T0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S6][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A1][0] = 0u64;
        c.g[S0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
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
        c.g[RA][0] = sx(0x002aaa1c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002aaa1c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T1][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T2][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T3][0] = 0x7f;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[T4][0] = 0x1;
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A3][0] = c.g[S3][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[A0][0] = c.g[S2][0];
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
        c.sw(c.g[V1][0] as u32, c.g[T0][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T2][0]);
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
        c.sw(c.g[V1][0] as u32, c.g[T3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[T4][0]);
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[RA][0] = sx(0x002aab60);
        c.sw(c.g[V0][0] as u32, c.g[S6][0]);
        call_ptr(c, 0x002aa7f0, 0x002aab60)?;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_BaseCtor(this, embedded, maxBanks, maxStreams, heap)`
/// (0x2ab200, frame 0x100): with `embedded` set, builds the sound system, bank
/// manager and stream system inside `this` (vtable/sub-object pointers at
/// +0x0, +0x118, +0x5560, +0x5908, +0x6200) and runs their ctors; always runs
/// the crowd, music and speech system ctors, rewrites the sub-objects'
/// vtables (0x4837d8, 0x483760, 0x4837a0), when not embedded copies them to
/// the stack rebased to `this`, and installs the crowd/music/speech vtables.
/// Returns `this`.
pub fn ssx_audio_system_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ab28c: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S1][0] = c.g[T0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S3][0] = c.g[A3][0];
            c.g[S4][0] = c.g[A1][0];
            let t = c.g[S4][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            if t {
                break 'b_2ab28c;
            } else {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5730));
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5728));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6200));
                c.g[A0][0] = c.g[A2][0];
                c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6200), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5908), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                c.g[RA][0] = sx(0x002ab260);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5560), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x002ab260)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6200));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002ab274);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0028b9b8, 0x002ab274)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5560));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x002ab28c);
                c.g[T0][0] = c.g[S1][0];
                call_ptr(c, 0x002aa490, 0x002ab28c)?;
                break 'b_2ab28c;
            }
        }
        'b_2ab42c: {
            c.cycles += 35;
            c.g[A2][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002ab2a4);
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, 0x002a69a0, 0x002ab2a4)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002ab2bc);
            c.g[A3][0] = c.g[S3][0];
            call_ptr(c, 0x002b28c0, 0x002ab2bc)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.g[RA][0] = sx(0x002ab2d4);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002b07f8, 0x002ab2d4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x37d8));
            c.g[A3][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[T7][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3760));
            c.g[S2][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x37a0));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[T7][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[S4][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[S1][0]);
            if t {
                break 'b_2ab42c;
            } else {
                c.cycles += 69;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x37d8));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3760));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T4][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[T1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A2][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5728));
                c.g[A1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.ld((c.g[T7][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T5][0]);
                c.g[T4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw(c.g[T4][0] as u32);
                c.g[V0][0] = c.lhu((c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5730));
                c.g[T3][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x37a0));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T0][0]);
                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw(c.g[T4][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[T6][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_2ab42c;
            }
        }
        c.cycles += 19;
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5558), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3870));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x37f8));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x571c), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5720), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        return Ok(());
    });
}

/// `FEModule_Load(FEModule* self)` (0x1a1ec0, frame 0x70): the front-end load
/// sequence: load timer progress 3.5/20/50/75/99, locale and texture init,
/// the UI file, character animations, bolt file manager, rider manager, the
/// front-end camera pose (0,0,-195)/(0,1,0), then the input, custom and
/// sound objects of the UI (alloc via 0x317d70), the irradiance database
/// and `cSSXAudio_FrontEndLoad`.
#[cfg(test)] // not installed: its test runs away in unslotted callees
#[allow(dead_code)]
pub fn fe_module_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 154;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.g[A0][0];
        c.g[AT][0] = 0x40600000;
        c.mtc1(1, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a1ee8);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317530, 0x001a1ee8)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1efc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1efc)?;
        c.g[RA][0] = sx(0x001a1f04);
        call_ptr(c, 0x0014dc80, 0x001a1f04)?;
        c.g[A1][0] = 0x9;
        c.g[RA][0] = sx(0x001a1f10);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x001a1f10)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a1f1c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014bd78, 0x001a1f1c)?;
        c.g[RA][0] = sx(0x001a1f24);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014af10, 0x001a1f24)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[RA][0] = sx(0x001a1f30);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x00195be0, 0x001a1f30)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42480000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1f44);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1f44)?;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001a1f60);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a1f60)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x460000;
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1278));
        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x9));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x9), c.g[A3][0]);
        c.g[V1][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0xa));
        c.sb((c.g[SP][0] as u32).wrapping_add(0xa), c.g[V1][0]);
        c.g[RA][0] = sx(0x001a1f9c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1278));
        call_ptr(c, 0x00397c40, 0x001a1f9c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42960000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a1fb0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a1fb0)?;
        c.g[RA][0] = sx(0x001a1fb8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a2b00, 0x001a1fb8)?;
        c.g[RA][0] = sx(0x001a1fc0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019ce68, 0x001a1fc0)?;
        c.g[RA][0] = sx(0x001a1fc8);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0498, 0x001a1fc8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[AT][0] = 0xffffffffc3430000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), 0u64);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001a2034);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001a2034)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        c.g[AT][0] = 0x42c60000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x001a2048);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00317550, 0x001a2048)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1288));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a2060);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a2060)?;
        c.g[V1][0] = 0x470000;
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e30));
        c.sb((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x460000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A2][0] = 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1298));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
        c.g[A0][0] = 0x14;
        c.g[RA][0] = sx(0x001a20a0);
        c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x001a20a0)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[RA][0] = sx(0x001a20ac);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a0720, 0x001a20ac)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12a8));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a20cc);
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x00317d70, 0x001a20cc)?;
        c.g[RA][0] = sx(0x001a20d4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001a2e20, 0x001a20d4)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = 0x500000;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x88));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[RA][0] = sx(0x001a2104);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        call_ptr(c, 0x0038acd0, 0x001a2104)?;
        c.g[RA][0] = sx(0x001a210c);
        call_ptr(c, 0x0028b180, 0x001a210c)?;
        c.g[RA][0] = sx(0x001a2114);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00285fb0, 0x001a2114)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and the fixture objects, clear of the fakes' call
    /// log (`DATA + 0x10_0000`) and pointer range (`DATA .. DATA + 0x4_0000`).
    const OC775_CTRL: u32 = DATA + 0x2e_0000;
    const OC775_MAGIC: u32 = 0x4f43_3737 + 5;
    const OC775_OBJ: u32 = DATA + 0x30_0000;

    /// Gated fakes for the callees that have an override slot.
    macro_rules! oc775_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC775_CTRL) != OC775_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc775_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc775_fakes!(
        (
            oc775_fake_002aa7f0,
            OC775_PREV_002AA7F0,
            ee::s08::OVERRIDE_002AA7F0,
            ee::s08::f_002aa7f0__generated,
            0x002a_a7f0
        ),
        (
            oc775_fake_002ade88,
            OC775_PREV_002ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc775_fake_00317e30,
            OC775_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc775_fake_003b5a28,
            OC775_PREV_003B5A28,
            ee::s13::OVERRIDE_003B5A28,
            ee::s13::f_003b5a28__generated,
            0x003b_5a28
        ),
        (
            oc775_fake_002948a0,
            OC775_PREV_002948A0,
            ee::s07::OVERRIDE_002948A0,
            ee::s07::f_002948a0__generated,
            0x0029_48a0
        ),
        (
            oc775_fake_002a4590,
            OC775_PREV_002A4590,
            ee::s08::OVERRIDE_002A4590,
            ee::s08::f_002a4590__generated,
            0x002a_4590
        ),
        (
            oc775_fake_00287968,
            OC775_PREV_00287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc775_fake_002b0e60,
            OC775_PREV_002B0E60,
            ee::s08::OVERRIDE_002B0E60,
            ee::s08::f_002b0e60__generated,
            0x002b_0e60
        ),
        (
            oc775_fake_00317530,
            OC775_PREV_00317530,
            ee::s10::OVERRIDE_00317530,
            ee::s10::f_00317530__generated,
            0x0031_7530
        ),
        (
            oc775_fake_00317550,
            OC775_PREV_00317550,
            ee::s10::OVERRIDE_00317550,
            ee::s10::f_00317550__generated,
            0x0031_7550
        ),
        (
            oc775_fake_0014dc80,
            OC775_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc775_fake_0014dd58,
            OC775_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc775_fake_0014bd78,
            OC775_PREV_0014BD78,
            ee::s01::OVERRIDE_0014BD78,
            ee::s01::f_0014bd78__generated,
            0x0014_bd78
        ),
        (
            oc775_fake_0014af10,
            OC775_PREV_0014AF10,
            ee::s01::OVERRIDE_0014AF10,
            ee::s01::f_0014af10__generated,
            0x0014_af10
        ),
        (
            oc775_fake_00397c40,
            OC775_PREV_00397C40,
            ee::s13::OVERRIDE_00397C40,
            ee::s13::f_00397c40__generated,
            0x0039_7c40
        ),
        (
            oc775_fake_001a2b00,
            OC775_PREV_001A2B00,
            ee::s03::OVERRIDE_001A2B00,
            ee::s03::f_001a2b00__generated,
            0x001a_2b00
        ),
        (
            oc775_fake_0019ce68,
            OC775_PREV_0019CE68,
            ee::s03::OVERRIDE_0019CE68,
            ee::s03::f_0019ce68__generated,
            0x0019_ce68
        ),
        (
            oc775_fake_001a0498,
            OC775_PREV_001A0498,
            ee::s03::OVERRIDE_001A0498,
            ee::s03::f_001a0498__generated,
            0x001a_0498
        ),
        (
            oc775_fake_00317d70,
            OC775_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc775_fake_001a0720,
            OC775_PREV_001A0720,
            ee::s03::OVERRIDE_001A0720,
            ee::s03::f_001a0720__generated,
            0x001a_0720
        ),
        (
            oc775_fake_001a2e20,
            OC775_PREV_001A2E20,
            ee::s03::OVERRIDE_001A2E20,
            ee::s03::f_001a2e20__generated,
            0x001a_2e20
        ),
        (
            oc775_fake_0028b180,
            OC775_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc775_fake_003de670,
            OC775_PREV_003DE670,
            ee::s14::OVERRIDE_003DE670,
            ee::s14::f_003de670__generated,
            0x003d_e670
        ),
        (
            oc775_fake_00391708,
            OC775_PREV_00391708,
            ee::s12::OVERRIDE_00391708,
            ee::s12::f_00391708__generated,
            0x0039_1708
        ),
    );

    /// `PATH_CountTracks`: the four path tables at 0x517644 (stride 0x928,
    /// in-use flag at +0, 24 slots from +0x24) filled with random zero and
    /// non-zero words; the mask picks tables by bits 24..27.
    #[test]
    fn path_count_tracks_matches() {
        check(
            ee::s14::f_003d5330__generated,
            path_count_tracks,
            300,
            &|c, seed| {
                let mut r = Rng(0x7d5330 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                for i in 0..4u32 {
                    let t = 0x0051_7644 + i * 0x928;
                    let used = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(t, used);
                    for j in 0..24u32 {
                        let w = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            r.next_u32()
                        };
                        c.bus.write32(t + 0x24 + 4 * j, w);
                    }
                }
                c.g[A0][0] = if seed % 5 == 0 {
                    sx(0xffff_ffff)
                } else {
                    sx((r.next_u32() & 0x0f00_0000) | (r.next_u32() & 0xf0ff_ffff))
                };
            },
        );
    }

    /// `StreamArray_CreateStreamDefaultParams`: the sys object with its
    /// parameter-array pointers (`+0x1d8 ..`) pointing at distinct zero-filled
    /// areas, a random instance counter (so the arrays are indexed 1..3) and a
    /// random default descriptor; `StreamArray_CreateStream` is a gated fake
    /// (unwinding on some seeds), `memset` runs for real.
    #[test]
    fn stream_array_create_stream_default_params_matches() {
        oc775_install_fakes();
        check(
            ee::s08::f_002aa910__generated,
            stream_array_create_stream_default_params,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x775a, 1);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                let this = OC775_OBJ;
                let sys = OC775_OBJ + 0x1000;
                c.bus.write32(this, sys);
                c.bus.write32(sys + 0x1f4, r.next_u32() % 3);
                for w in 0..6u32 {
                    c.bus.write32(sys + 0x1dc + 4 * w, r.next_u32());
                }
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    let area = OC775_OBJ + 0x4000 + 0x100 * k as u32;
                    c.bus.write32(sys + 0x1d8 + off, area);
                    for w in 0..0x40u32 {
                        c.bus.write32(area + 4 * w, r.next_u32());
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
            },
        );
    }

    /// Gp words every audio-system fixture needs (BELibrary dummy and world).
    fn oc775_gp_words(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), OC775_OBJ + 0x8000);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), OC775_OBJ + 0x9000);
    }

    /// `SSXAudioSystem_BaseCtor` with `embedded` 0 and 1, a zeroed 0x7780-byte
    /// object, gated fakes on the callees with slots (the same set the
    /// `SSXAudioSystem_ctor` test uses). The sound manager, bank manager,
    /// stream system and the crowd/music/speech ctors run for real; if one of
    /// them runs away on the fixture the entry is dropped.
    #[test]
    fn ssx_audio_system_base_ctor_matches() {
        oc775_install_fakes();
        check(
            ee::s08::f_002ab200__generated,
            ssx_audio_system_base_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x775b, 7);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                oc775_gp_words(c);
                c.g[A0][0] = sx(OC775_OBJ);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = 0x1c;
                c.g[A3][0] = 3;
                c.g[T0][0] = (seed / 2 % 3) as u64;
            },
        );
    }

    /// `Font_ctor` (already hand-written in `lightman`): default UV transforms
    /// and colours into a random font object, then `Font_Load` (a gated
    /// fake, unwinding on some seeds).
    #[test]
    fn font_ctor_matches() {
        oc775_install_fakes();
        check(
            ee::s12::f_003915e8__generated,
            crate::game::render::lightman::font_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x775d, 1);
                c.bus.write32(OC775_CTRL, OC775_MAGIC);
                for w in 0..0x20u32 {
                    c.bus.write32(OC775_OBJ + 4 * w, r.next_u32());
                }
                c.g[A0][0] = sx(OC775_OBJ);
                c.g[A1][0] = sx(DATA + 0x20_0000);
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }
}
