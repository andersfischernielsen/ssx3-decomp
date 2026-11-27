//! Staging module oc1031 (wave wave263): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1031`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_dc48,
        ssx_audio_system_play_handplant_sfx,
        "SSXAudioSystem_PlayHandplantSfx",
    ),
    (
        0x0029_e590,
        ssx_audio_system_play_wipeout_impact_sfx,
        "SSXAudioSystem_PlayWipeoutImpactSfx",
    ),
    (
        0x0029_e970,
        ssx_audio_system_play_wipeout_slide_sfx,
        "SSXAudioSystem_PlayWipeoutSlideSfx",
    ),
    (
        0x002a_4770,
        ssx_audio_system_play_dj_result_chatter,
        "SSXAudioSystem_PlayDJResultChatter",
    ),
    (0x002a_86b8, midi_banks_play, "MIDIBanks_Play"),
];

// ---------------------------------------------------------------------------
// The inlined `sndPlayVoice` of the SSXAudioSystem sound-effect triggers.
//
// `**(audio + 0x118)` is the voice array: a `params` block at `+0x1d8` (a
// stack of per-voice parameter levels) and the current level index `top` at
// `+0x1f4`. Every trigger pushes a level (`top += 1`), copies the 24-byte
// default entry (`+0x1dc..+0x1f3`) into `params[0x20][top]`, clears and
// fills the per-level arrays and then asks the voice manager for a handle.
// ---------------------------------------------------------------------------

/// Index of the current level in the voice array (`StreamArray::top`).
pub(crate) const VOICE_TOP: u32 = 0x1f4;

/// `top << shift` (the address of level `top` of the per-level array at
/// `params + field`), the compiled code's `x = top; y = params[field];
/// x <<= shift; x += y`, with `x` and `y` left in those registers.
pub(crate) fn voice_slot(c: &mut Cpu, x: usize, y: usize, field: u32, shift: u32) -> u32 {
    c.g[x][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[y][0] = c.lw(r32(c, S1).wrapping_add(field));
    c.g[x][0] = sx(r32(c, x) << shift);
    c.g[x][0] = sx(r32(c, x).wrapping_add(r32(c, y)));
    r32(c, x)
}

/// From `top += 1` to the `memset` of the level's 8-byte delay record: stores
/// the new `top`, copies the default entry (24 bytes at `+0x1dc`) into
/// `params[0x20][top]` (`stride` holds 0x18), clears the target word and
/// `params[0x34][top]`, stores the entry's volume byte to `params[0x38][top]`,
/// clears `params[0x3c][top]` and calls `memset(&params[0x40 + 8 top], 0, 8)`
/// returning to `memset_ret`. Expects `s0` = voice array, `s1` = params, `v0` =
/// the old `top`.
pub(crate) fn voice_push_entry(c: &mut Cpu, stride: usize, memset_ret: u32) -> Flow {
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let top = c.g[V0][0];
    c.sw(r32(c, S0).wrapping_add(VOICE_TOP), top);
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.mult(0, V1, V1, stride);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let s0 = r32(c, S0);
    c.ldl(V0, s0.wrapping_add(0x1e3));
    c.ldr(V0, s0.wrapping_add(0x1dc));
    c.ldl(A0, s0.wrapping_add(0x1eb));
    c.ldr(A0, s0.wrapping_add(0x1e4));
    c.ldl(A3, s0.wrapping_add(0x1f3));
    c.ldr(A3, s0.wrapping_add(0x1ec));
    let e = r32(c, V1);
    c.sdl(V0, e.wrapping_add(7));
    c.sdr(V0, e);
    c.sdl(A0, e.wrapping_add(0xf));
    c.sdr(A0, e.wrapping_add(8));
    c.sdl(A3, e.wrapping_add(0x17));
    c.sdr(A3, e.wrapping_add(0x10));
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.sw(r32(c, V0).wrapping_add(0x1fc), 0);
    voice_clear_and_volume(c, stride, V1, V0);
    voice_memset_delay(c, memset_ret)
}

/// `params[0x34][top] = 0`, then `params[0x38][top] = (signed byte)
/// entry[top][0]` and `params[0x3c][top] = 0`. `x`/`y` are the registers the
/// first slot is built in (the second one swaps them).
pub(crate) fn voice_clear_and_volume(c: &mut Cpu, stride: usize, x: usize, y: usize) {
    let a = voice_slot(c, x, y, 0x34, 2);
    c.sw(a, 0);
    c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[T0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, stride);
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = c.lb(r32(c, A0));
    let vol = c.g[V1][0];
    c.sw(r32(c, A3), vol);
    let a = voice_slot(c, V0, V1, 0x3c, 2);
    c.sw(a, 0);
}

/// `memset(&params[0x40 + 8 top], 0, 8)` (the level's delay record).
pub(crate) fn voice_memset_delay(c: &mut Cpu, memset_ret: u32) -> Flow {
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, memset_ret)
}

/// The per-level defaults after the `memset`: `regs` are the registers the
/// compiler used for the constants 100, 90, 50, 127 and 1 (the delay-block
/// level/feedback/time/volume and the priority); the arrays at `params +
/// 0x60..0x84` are filled for level `top`.
pub(crate) fn voice_set_defaults(c: &mut Cpu, regs: [usize; 5]) {
    let [r100, r90, r50, r127, r1] = regs;
    c.g[r100][0] = 0x64;
    c.g[r90][0] = 0x5a;
    c.g[r50][0] = 0x32;
    c.g[r127][0] = 0x7f;
    c.g[r1][0] = 1;
    let a = voice_slot(c, V1, V0, 0x60, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V0, V1, 0x64, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V1, V0, 0x68, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V0, V1, 0x6c, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V1, V0, 0x70, 4);
    c.sw(a.wrapping_add(4), 0);
    let a = voice_slot(c, V0, V1, 0x70, 4);
    let v = c.g[r100][0];
    c.sw(a, v);
    let a = voice_slot(c, V1, V0, 0x70, 4);
    let v = c.g[r90][0];
    c.sw(a.wrapping_add(8), v);
    let a = voice_slot(c, V0, V1, 0x70, 4);
    let v = c.g[r50][0];
    c.sw(a.wrapping_add(0xc), v);
    let a = voice_slot(c, V0, V1, 0x74, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V1, V0, 0x80, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V0, V1, 0x84, 2);
    c.sw(a, 0);
    let a = voice_slot(c, V1, V0, 0x78, 2);
    let v = c.g[r127][0];
    c.sw(a, v);
    let a = voice_slot(c, V0, V1, 0x7c, 2);
    let v = c.g[r1][0];
    c.sw(a, v);
}

/// The volume clamp of the wipeout triggers: with `A2 = params`, `A3 =
/// params[0x38]` the volume of level `top` is clamped into `0..=127` (a
/// negative stored volume gives 0, 127 or more gives 127, anything else the
/// unsigned byte of the entry), leaving `V0` = the value and `A0` = the
/// address `params[0x20][top]` it is stored to (by the caller: `sb`).
/// Starts at the `A0 = lw(arr + 0x1f4)` after `A2 = arr + 0x1d8`.
pub(crate) fn voice_clamp_volume(c: &mut Cpu, stride: usize) {
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(VOICE_TOP));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
    c.g[A3][0] = c.lw(r32(c, A2).wrapping_add(0x38));
    c.mult(0, A0, A0, stride);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x20));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[V0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
    let below = c.g[V0][0] != 0;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    let mut done = false;
    if below {
        c.cycles += 6;
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
        c.g[V1][0] = c.lw(r32(c, V0));
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        done = negative;
    }
    if !done {
        c.cycles += 7;
        c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x1c));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V0][0] = c.lbu(r32(c, V0));
        }
    }
}

/// `SSXAudioSystem_PlayHandplantSfx(audio, rider)` (0x29dc48): when the rider's
/// `+0x874` and `+0x87c` are both set, queues sound id 0x61 as a new voice
/// level (an inlined `sndPlayVoice(audio, 0, 0x61, 9)`): pushes the level,
/// fills the defaults (volume 100/90/50, pan 127, priority 1), sets the two
/// per-level tables of the audio object (`audio[1][top] = 0`, `audio[2][top]
/// = 0x61`), asks the voice manager for the handle (`0x287968`) and stores it
/// in the handle array (`arr[0x214][top]`) before starting the voice
/// (`0x2906b8`).
pub fn ssx_audio_system_play_handplant_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x874));
        let skip = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x87c));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let skip = c.g[V1][0] == 0;
        c.g[T1][0] = 0x18;
        if !skip {
            c.cycles += 148;
            c.g[S0][0] = c.lw(r32(c, S3));
            c.g[A1][0] = 0;
            c.g[A2][0] = 8;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x1d8));
            c.g[S2][0] = 0x61;
            voice_push_entry(c, T1, 0x0029_dd54)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 9;
            c.g[A2][0] = 0;
            voice_set_defaults(c, [A3, T0, T1, T2, T3]);
            // audio[1][top] = 0; audio[2][top] = 0x61.
            c.g[V1][0] = c.lw(r32(c, S3));
            c.g[A3][0] = c.lw(r32(c, S3).wrapping_add(4));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(VOICE_TOP));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.sw(r32(c, V0), 0);
            c.g[V1][0] = c.lw(r32(c, S3));
            c.g[A3][0] = c.lw(r32(c, S3).wrapping_add(8));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(VOICE_TOP));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            let id = c.g[S2][0];
            c.sw(r32(c, V0), id);
            c.g[S0][0] = c.lw(r32(c, S3).wrapping_add(0x118));
            call_ptr(c, 0x0028_7968, 0x0029_deb0)?;
            // arr[0x214][top] = handle; start the voice.
            c.g[A1][0] = c.lw(r32(c, S0));
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(VOICE_TOP));
            c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(0x214));
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
            let handle = c.g[V0][0];
            c.sw(r32(c, V1), handle);
            call_ptr(c, 0x0029_06b8, 0x0029_ded0)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The gate shared by the two wipeout triggers: `0x288ae0` (replay playing)
/// must return 0, and the rider's `+0x874`/`+0x87c` must both be set. Leaves
/// `V1` and `V0` as the generated code does (`V0` = `audio + 0x6080`, the
/// per-rider "last played" float table) and returns whether to play.
fn wipeout_rider_ready(c: &mut Cpu) -> bool {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x874));
    let skip = c.g[V0][0] == 0;
    c.g[V1][0] = 0;
    if !skip {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x87c));
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    let skip = c.g[V1][0] == 0;
    c.g[V0][0] = sx(r32(c, S4).wrapping_add(0x6080));
    !skip
}

/// The two stores and calls both wipeout triggers end with: the handle of the
/// new voice goes into `arr[0x214][top]` (`0x287968` allocates it, `0x2906b8`
/// starts it), after the audio object's tables get `[top] = 0` and `[top] =
/// voice` (`s3`). Shared from `A0 = lw(audio)` up to the clamp: the caller adds
/// the cycles and the `sb`.
fn wipeout_tables_and_clamp(c: &mut Cpu) {
    c.g[A0][0] = c.lw(r32(c, S4));
    c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(4));
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(VOICE_TOP));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.sw(r32(c, V0), 0);
    c.g[V1][0] = c.lw(r32(c, S4));
    c.g[A0][0] = c.lw(r32(c, S4).wrapping_add(8));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(VOICE_TOP));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    let id = c.g[S3][0];
    c.sw(r32(c, V0), id);
    c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(0x118));
    c.g[A1][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(VOICE_TOP));
    c.g[A2][0] = sx(r32(c, A1).wrapping_add(0x1d8));
    c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x38));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    let v127 = c.g[A3][0];
    c.sw(r32(c, V0), v127);
    voice_clamp_volume(c, S2);
}

/// `SSXAudioSystem_PlayWipeoutImpactSfx(audio, rider, float amount)` (0x29e590):
/// when no replay is playing and the rider's `+0x874`/`+0x87c` are set, plays
/// sound 0x5f on voice level `top + 1` with a pitch that rises with `amount`:
/// `amount` is clamped to the rider's last value (`audio[0x6080][rider[0x870]]`),
/// scaled by 5 and truncated; the pitch is `0x1000 + 0x50 * that`. The
/// amount is remembered in the table afterwards.
pub fn ssx_audio_system_play_wipeout_impact_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x40);
        frame.save(c, S0, 0x80);
        c.g[S4][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x90);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        call_ptr(c, 0x0028_8ae0, 0x0029_e5cc)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x80);
        } else {
            if wipeout_rider_ready(c) {
                c.cycles += 170;
                c.g[S5][0] = c.lw(r32(c, S0).wrapping_add(0x870));
                c.g[S0][0] = c.lw(r32(c, S4));
                c.g[S7][0] = c.g[V0][0];
                c.g[A0][0] = sx(r32(c, S5) << 2);
                c.g[AT][0] = 0x40a0_0000;
                c.mtc1(AT, 1);
                c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.lwc1(0, r32(c, V0));
                c.g[S2][0] = 0x18;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x1d8));
                let top = c.g[V1][0];
                c.sw(r32(c, S0).wrapping_add(VOICE_TOP), top);
                // pitch = 0x1000 + 0x50 * (int)(min(amount, last) * 5.0)
                c.min_s(0, 20, 0);
                c.mul_s(0, 0, 1);
                c.g[A3][0] = 0x50;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
                c.g[A1][0] = 0;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x20));
                c.g[A2][0] = 8;
                c.mult(0, A0, A0, S2);
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[S3][0] = 0x5f;
                c.mult(0, V0, V0, A3);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                let s0 = r32(c, S0);
                c.ldl(V1, s0.wrapping_add(0x1e3));
                c.ldr(V1, s0.wrapping_add(0x1dc));
                c.ldl(A3, s0.wrapping_add(0x1eb));
                c.ldr(A3, s0.wrapping_add(0x1e4));
                c.ldl(T0, s0.wrapping_add(0x1f3));
                c.ldr(T0, s0.wrapping_add(0x1ec));
                let e = r32(c, A0);
                c.sdl(V1, e.wrapping_add(7));
                c.sdr(V1, e);
                c.sdl(A3, e.wrapping_add(0xf));
                c.sdr(A3, e.wrapping_add(8));
                c.sdl(T0, e.wrapping_add(0x17));
                c.sdr(T0, e.wrapping_add(0x10));
                c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1000));
                c.g[S6][0] = c.g[V0][0] & 0xffff;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                c.sw(r32(c, V1).wrapping_add(0x1fc), 0);
                voice_clear_and_volume(c, S2, V0, V1);
                voice_memset_delay(c, 0x0029_e704)?;
                voice_set_defaults(c, [A0, A1, A2, A3, T0]);
                wipeout_tables_and_clamp(c);
                c.cycles += 25;
                let vol = c.g[V0][0];
                c.sb(r32(c, A0), vol);
                c.g[A1][0] = 9;
                c.g[A2][0] = 0;
                c.g[A0][0] = c.g[S4][0];
                c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(0x118));
                call_ptr(c, 0x0028_7968, 0x0029_e8f4)?;
                c.g[A2][0] = c.lw(r32(c, S0));
                c.g[A3][0] = 0x18;
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(VOICE_TOP));
                c.g[A1][0] = c.lw(r32(c, A2).wrapping_add(0x214));
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                let handle = c.g[V0][0];
                c.sw(r32(c, V1), handle);
                c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(0x118));
                c.g[V1][0] = c.lw(r32(c, A1));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(VOICE_TOP));
                c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0x1f8));
                c.mult(0, V0, V0, A3);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                let pitch = c.g[S6][0];
                c.sh(r32(c, V0).wrapping_add(0xc), pitch);
                call_ptr(c, 0x0029_06b8, 0x0029_e934)?;
                c.g[V1][0] = sx(r32(c, S5) << 2);
                c.g[V1][0] = sx(r32(c, S7).wrapping_add(r32(c, V1)));
                c.swc1(20, r32(c, V1));
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    });
}

/// `SSXAudioSystem_PlayWipeoutSlideSfx(audio, rider)` (0x29e970): while a
/// rider slides after a wipeout (`+0x874` and `+0x87c` set, no replay playing)
/// starts sound 0x60 once: the rider's slot of the "last played" table
/// (`audio[0x6080][rider[0x870]]`) holds 1.0 while it plays; if it is 1.0
/// nothing happens, otherwise the voice is queued like the impact sound (but
/// without a pitch) and the slot is set to 1.0.
pub fn ssx_audio_system_play_wipeout_slide_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S4, 0x30);
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0028_8ae0, 0x0029_e9a0)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x70);
        } else {
            let mut playing = false;
            if wipeout_rider_ready(c) {
                c.cycles += 11;
                c.g[S5][0] = c.lw(r32(c, S0).wrapping_add(0x870));
                c.g[S6][0] = c.g[V0][0];
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.g[V1][0] = sx(r32(c, S5) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.lwc1(0, r32(c, V0));
                c.c_eq_s(0, 1);
                playing = c.fcc();
                frame.restore(c, S0, 0x70);
                if !playing {
                    c.cycles += 155;
                    c.g[S0][0] = c.lw(r32(c, S4));
                    c.g[S2][0] = 0x18;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 8;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(VOICE_TOP));
                    c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x1d8));
                    c.g[S3][0] = 0x60;
                    voice_push_entry(c, S2, 0x0029_eac8)?;
                    voice_set_defaults(c, [A0, A1, A2, A3, T0]);
                    wipeout_tables_and_clamp(c);
                    c.cycles += 19;
                    let vol = c.g[V0][0];
                    c.sb(r32(c, A0), vol);
                    c.g[A1][0] = 9;
                    c.g[A2][0] = 0;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(0x118));
                    call_ptr(c, 0x0028_7968, 0x0029_ecb8)?;
                    c.g[A1][0] = c.lw(r32(c, S0));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(VOICE_TOP));
                    c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(0x214));
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
                    let handle = c.g[V0][0];
                    c.sw(r32(c, V1), handle);
                    call_ptr(c, 0x0029_06b8, 0x0029_ecd8)?;
                    c.g[V1][0] = sx(r32(c, S5) << 2);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 0);
                    c.g[V1][0] = sx(r32(c, S6).wrapping_add(r32(c, V1)));
                    c.swc1(0, r32(c, V1));
                }
            }
            if !playing {
                c.cycles += 1;
                frame.restore(c, S0, 0x70);
            }
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SSXAudioSystem_PlayDJResultChatter(SSXAudioSystem* s)` (0x2a4770): after
/// a run, picks which DJ result remark to say. Does nothing (returns 0) unless
/// the "result pending" flag `+0x57f8` is set (it clears it). Of three
/// candidate remarks it keeps those that apply (`flags[0]`: `+0x5800 == 0`,
/// `flags[1]`: `+0x5804 >= 5`, `flags[2]`: `+0x5808` reaches the limit 24, or
/// 27 for track ids of groups 0/1/3; all only when `+0x57fc` is set). With
/// none, speaks the location remark for the current location `+0x5814` if
/// `+0x6254` says the DJ may (group 5: `PlayDJChatter`; group 4:
/// `DJSay20E9`; otherwise `DJSay20CC`). With several it drops the one said
/// last (`+0x580c`), picks one at random (`Audio_Rand`), calls
/// `DJSay2102`/`DJSay212E`/`DJSay212F` for pick 0/1/2 and records the pick.
/// Returns whether something was said.
pub fn ssx_audio_system_play_dj_result_chatter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a49cc: {
            'b_2a49c8: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x57f8));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0x2;
                if t {
                    break 'b_2a49c8;
                } else {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x57f8), 0u64);
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                    'l_2a47a0: loop {
                        c.cycles += 7;
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[V1][0] as i64) >= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002a47a0);
                            }
                            continue 'l_2a47a0;
                        } else {
                            'b_2a4854: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x57fc));
                                let t = c.g[V0][0] == 0u64;
                                c.g[S2][0] = 0u64;
                                if t {
                                    break 'b_2a4854;
                                } else {
                                    'b_2a47e4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5800));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5804));
                                            break 'b_2a47e4;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x1;
                                            c.g[S2][0] = 0x1;
                                            c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5804));
                                            break 'b_2a47e4;
                                        }
                                    }
                                    'b_2a4800: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 5i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5810));
                                            break 'b_2a4800;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x1;
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5810));
                                            break 'b_2a4800;
                                        }
                                    }
                                    'b_2a483c: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002a4808);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x002a40e0,
                                            f: ee::s08::SSXAudioSystem_IsTrackIdGroup0,
                                        }
                                        .call(c, 0x002a4808)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = 0x1b;
                                        if t {
                                            break 'b_2a483c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5810));
                                            c.g[RA][0] = sx(0x002a481c);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x002a4158,
                                                f: ee::s08::SSXAudioSystem_IsTrackIdGroup1,
                                            }
                                            .call(c, 0x002a481c)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = 0x1b;
                                            if t {
                                                break 'b_2a483c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5810));
                                                c.g[RA][0] = sx(0x002a4830);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x002a4238,
                                                    f: ee::s08::SSXAudioSystem_IsTrackIdGroup3,
                                                }
                                                .call(c, 0x002a4830)?;
                                                c.g[A0][0] = 0x18;
                                                c.g[V1][0] = 0x1b;
                                                if c.g[V0][0] != 0 {
                                                    c.g[A0][0] = c.g[V1][0];
                                                }
                                                break 'b_2a483c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5808));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_2a4854;
                                    } else {
                                        c.cycles += 2;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        break 'b_2a4854;
                                    }
                                }
                            }
                            'b_2a48c8: {
                                c.cycles += 2;
                                let t = c.g[S2][0] != 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x57fc), 0u64);
                                if t {
                                    break 'b_2a48c8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6254));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x17;
                                    if t {
                                        break 'b_2a49c8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5814));
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_2a49cc;
                                        } else {
                                            'b_2a4894: {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x002a487c);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x002a3fd8,
                                                    f: ee::s08::SSXAudioSystem_IsTrackIdGroup5,
                                                }
                                                .call(c, 0x002a487c)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_2a4894;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x002a488c);
                                                    c.g[A1][0] = 0u64;
                                                    Callee {
                                                        addr: 0x002a2e50,
                                                        f: ee::s08::SSXAudioSystem_PlayDJChatter,
                                                    }
                                                    .call(c, 0x002a488c)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_2a49cc;
                                                    } else {
                                                        break 'b_2a4894;
                                                    }
                                                }
                                            }
                                            'b_2a48b8: {
                                                c.cycles += 5;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5814));
                                                c.g[RA][0] = sx(0x002a48a0);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x002a4030,
                                                    f: ee::s08::SSXAudioSystem_IsTrackIdGroup4,
                                                }
                                                .call(c, 0x002a48a0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_2a48b8;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x002a48b0);
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5814));
                                                    Callee {
                                                        addr: 0x002a23d0,
                                                        f: ee::s08::SSXAudioSystem_DJSay20E9,
                                                    }
                                                    .call(c, 0x002a48b0)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_2a49cc;
                                                    } else {
                                                        break 'b_2a48b8;
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002a48c0);
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x5814));
                                            Callee {
                                                addr: 0x002a24b0,
                                                f: ee::s08::SSXAudioSystem_DJSay20CC,
                                            }
                                            .call(c, 0x002a48c0)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_2a49cc;
                                            } else {
                                                break 'b_2a48c8;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_2a4900: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                    break 'b_2a4900;
                                } else {
                                    'b_2a48fc: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x580c));
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        if t {
                                            break 'b_2a48fc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[SP][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                            if c.g[V1][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                                break 'b_2a4900;
                                            } else {
                                                c.cycles += 2;
                                                c.sw((c.g[V0][0] as u32), 0u64);
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                                break 'b_2a48fc;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                    break 'b_2a4900;
                                }
                            }
                            'b_2a4924: {
                                c.cycles += 9;
                                c.g[S1][0] = 0xffffffffffffffff;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                c.g[RA][0] = sx(0x002a4910);
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                                Callee {
                                    addr: 0x002adf60,
                                    f: ee::s08::Audio_Rand,
                                }
                                .call(c, 0x002a4910)?;
                                c.g[V0][0] = c.g[V0][0] & 0x7fff;
                                c.g[V1][0] = 0x7fff;
                                c.mult(0, 2, 2, 18);
                                if c.g[V1][0] == 0u64 {
                                    break 'b_2a4924;
                                } else {
                                    break 'b_2a4924;
                                }
                            }
                            'b_2a4960: {
                                c.cycles += 5;
                                c.divu(0, 2, 3);
                                c.g[V0][0] = c.lo[0];
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                let t = (c.g[A0][0] as i64) <= 0;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_2a4960;
                                } else {
                                    c.cycles += 2;
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                                    'l_2a4940: loop {
                                        c.cycles += 7;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                        if c.g[V0][0] != 0 {
                                            c.g[A0][0] = c.g[V1][0];
                                        }
                                        let t = (c.g[A0][0] as i64) > 0;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a4940);
                                            }
                                            continue 'l_2a4940;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = 0x1;
                                            break 'b_2a4960;
                                        }
                                    }
                                }
                            }
                            'b_2a49c0: {
                                'b_2a49b4: {
                                    'b_2a49a0: {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                        if t {
                                            break 'b_2a49a0;
                                        } else {
                                            'b_2a4990: {
                                                'b_2a4980: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_2a4980;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S1][0] == 0u64;

                                                        if t {
                                                            break 'b_2a4990;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x580c),
                                                                c.g[S1][0],
                                                            );
                                                            if t {
                                                                break 'b_2a49c0;
                                                            } else {
                                                                break 'b_2a4980;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                if c.g[S1][0] == c.g[V0][0] {
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32)
                                                            .wrapping_add(0x5810));
                                                    break 'b_2a49b4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x580c),
                                                        c.g[S1][0],
                                                    );
                                                    if t {
                                                        break 'b_2a49c0;
                                                    } else {
                                                        break 'b_2a4990;
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002a4998);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x002a2938,
                                                f: ee::s08::SSXAudioSystem_DJSay2102,
                                            }
                                            .call(c, 0x002a4998)?;
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x580c),
                                                c.g[S1][0],
                                            );
                                            if t {
                                                break 'b_2a49c0;
                                            } else {
                                                break 'b_2a49a0;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5810));
                                    c.g[RA][0] = sx(0x002a49ac);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x002a2cf0,
                                        f: ee::s08::SSXAudioSystem_DJSay212E,
                                    }
                                    .call(c, 0x002a49ac)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x580c), c.g[S1][0]);
                                    if t {
                                        break 'b_2a49c0;
                                    } else {
                                        break 'b_2a49b4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x002a49bc);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002a2da0,
                                    f: ee::s08::SSXAudioSystem_DJSay212F,
                                }
                                .call(c, 0x002a49bc)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x580c), c.g[S1][0]);
                                break 'b_2a49c0;
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2a49cc;
                            } else {
                                break 'b_2a49c8;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_2a49cc;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int MIDIBanks_Play(sSnd86B8Sys* sys)` (0x2a86b8): plays the MIDI sound of
/// the bank manager's current level `top` (`mgr = *sys`, level arrays in
/// `mgr + 0x1d8`) and returns its handle (-1 when none): first settles the
/// level's base volume (`baseVol[top]`: zeroed with the current volume
/// clamped when the level's position is set, otherwise scaled by its volume
/// control's gain, clamped to 0..127), looks the sound's bank up (`sys[4]`
/// entry `top`: `{id, sound}`, bank table `mgr[0xacc]` entries of 0x60
/// bytes), starts it under `SNDSYS_entercritical`/`SNDMIDI_play` when the
/// level has no radius, adds a bank-monitor instance for it (and attaches the
/// sample) and queues the level's echo timers (`mgr[0x248][top]`: count,
/// time, feedback, level) as `BankMonitor_AddTimer` delays whose volume decays
/// by the feedback until it reaches 0. Finally pops the level (`top -= 1`).
/// A resume entry (after an unwound callee or event) runs the generated body.
pub fn midi_banks_play(c: &mut Cpu) {
    if c.pc != 0x002a86b8 {
        return ee::s08::f_002a86b8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002a86b8;
        loop {
            match pc {
                0x002a86b8 => {
                    c.cycles += 24;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32));
                    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1fc));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S5][0] = c.lw((c.g[V0][0] as u32));
                    if t {
                        pc = 0x002a87a0;
                        continue;
                    }
                    pc = 0x002a8718;
                    continue;
                }
                0x002a8718 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = 0x18;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if t {
                        pc = 0x002a8770;
                        continue;
                    }
                    pc = 0x002a8758;
                    continue;
                }
                0x002a8758 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002a88fc;
                        continue;
                    }
                    pc = 0x002a8770;
                    continue;
                }
                0x002a8770 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        pc = 0x002a88fc;
                        continue;
                    }
                    pc = 0x002a878c;
                    continue;
                }
                0x002a878c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                    if t {
                        pc = 0x002a88fc;
                        continue;
                    }
                    pc = 0x002a87a0;
                    continue;
                }
                0x002a87a0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V1][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                        pc = 0x002a8904;
                        continue;
                    }
                    pc = 0x002a87bc;
                    continue;
                }
                0x002a87bc => {
                    c.cycles += 19;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[AT][0] = 0x42fe0000;
                    c.mtc1(1, 3);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.lwc1(0, (c.g[V1][0] as u32));
                    c.cvt_s_w(0, 0);
                    c.lwc1(2, (c.g[A1][0] as u32));
                    c.mul_s(0, 0, 2);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    if t {
                        pc = 0x002a8834;
                        continue;
                    }
                    pc = 0x002a8808;
                    continue;
                }
                0x002a8808 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.mtc1(0, 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.lwc1(0, (c.g[V0][0] as u32));
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x002a8884;
                        continue;
                    }
                    pc = 0x002a8834;
                    continue;
                }
                0x002a8834 => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.lwc1(0, (c.g[V0][0] as u32));
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.g[A1][0] = 0x7f;
                    if t {
                        pc = 0x002a8884;
                        continue;
                    }
                    pc = 0x002a885c;
                    continue;
                }
                0x002a885c => {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                    c.lwc1(0, (c.g[V0][0] as u32));
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.cvt_w_s(1, 0);
                    c.mfc1(5, 1);
                    let t = true;
                    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
                    if t {
                        pc = 0x002a8888;
                        continue;
                    }
                    pc = 0x002a8884;
                    continue;
                }
                0x002a8884 => {
                    c.cycles += 1;
                    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
                    pc = 0x002a8888;
                    continue;
                }
                0x002a8888 => {
                    c.cycles += 12;
                    c.g[V0][0] = 0x18;
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x38));
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if t {
                        pc = 0x002a88d0;
                        continue;
                    }
                    pc = 0x002a88b8;
                    continue;
                }
                0x002a88b8 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x002a88fc;
                        continue;
                    }
                    pc = 0x002a88d0;
                    continue;
                }
                0x002a88d0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        pc = 0x002a88fc;
                        continue;
                    }
                    pc = 0x002a88ec;
                    continue;
                }
                0x002a88ec => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                    pc = 0x002a88fc;
                    continue;
                }
                0x002a88fc => {
                    c.cycles += 2;
                    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                    pc = 0x002a8904;
                    continue;
                }
                0x002a8904 => {
                    c.cycles += 15;
                    c.g[A2][0] = 0x60;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[FP][0] = 0xffffffffffffffff;
                    c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xacc));
                    c.g[A3][0] = c.g[A0][0];
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.mult(0, 3, 3, 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x002a8944;
                        continue;
                    }
                    pc = 0x002a8940;
                    continue;
                }
                0x002a8940 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    pc = 0x002a8944;
                    continue;
                }
                0x002a8944 => {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                    c.mtc1(0, 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.lwc1(0, (c.g[V0][0] as u32));
                    c.c_eq_s(0, 1);

                    if !c.fcc() {
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                        pc = 0x002a89bc;
                        continue;
                    }
                    pc = 0x002a8974;
                    continue;
                }
                0x002a8974 => {
                    c.cycles += 2;
                    let t = (c.g[S1][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
                    if t {
                        pc = 0x002a89b8;
                        continue;
                    }
                    pc = 0x002a897c;
                    continue;
                }
                0x002a897c => {
                    c.cycles += 15;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[RA][0] = sx(0x002a8988);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x003b58a0, 0x002a8988)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                    c.g[T0][0] = 0x18;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f8));
                    c.mult(0, 4, 4, 8);
                    c.g[RA][0] = sx(0x002a89b0);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, 0x003b9a00, 0x002a89b0)?;
                    c.g[RA][0] = sx(0x002a89b8);
                    c.g[FP][0] = c.g[V0][0];
                    call_ptr(c, 0x003b58d8, 0x002a89b8)?;
                    pc = 0x002a89b8;
                    continue;
                }
                0x002a89b8 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                    pc = 0x002a89bc;
                    continue;
                }
                0x002a89bc => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A3][0] = 0x2;
                    c.g[RA][0] = sx(0x002a89d0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    Callee {
                        addr: 0x002acfe0,
                        f: ee::s08::BankMonitor_AddInstance,
                    }
                    .call(c, 0x002a89d0)?;
                    c.g[A0][0] = c.g[V0][0];
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
                    if t {
                        pc = 0x002a89f0;
                        continue;
                    }
                    pc = 0x002a89dc;
                    continue;
                }
                0x002a89dc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x002a89f0);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x002abd38,
                        f: ee::s08::SoundRequest_SetBankSample_2,
                    }
                    .call(c, 0x002a89f0)?;
                    pc = 0x002a89f0;
                    continue;
                }
                0x002a89f0 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x248));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0x64;
                    if t {
                        pc = 0x002a8ae0;
                        continue;
                    }
                    pc = 0x002a8a10;
                    continue;
                }
                0x002a8a10 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));

                    if c.g[A0][0] == 0u64 {
                        pc = 0x002a8a1c;
                        continue;
                    }
                    pc = 0x002a8a1c;
                    continue;
                }
                0x002a8a1c => {
                    c.cycles += 13;
                    c.g[S3][0] = 0x64;
                    c.mult(0, 2, 21, 2);
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[S6][0] = 0x7f;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[S5][0] = 0xffffffffffffffff;
                    c.g[S7][0] = sx((c.g[S2][0] as u32) << 3);
                    c.div(0, 2, 4);
                    c.g[S2][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = c.lo[0];
                    let t = true;
                    c.mult(0, 2, 2, 5);
                    if t {
                        pc = 0x002a8abc;
                        continue;
                    }
                    pc = 0x002a8a50;
                    continue;
                }
                0x002a8a50 => {
                    c.cycles += 20;
                    c.g[V0][0] = 0x4a0000;
                    c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x374f));
                    c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x3748));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.g[T2][0] = c.g[S0][0];
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3c));
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x002a8a94);
                    c.g[T3][0] = 0u64;
                    Callee {
                        addr: 0x002adca0,
                        f: ee::s08::BankMonitor_AddTimer,
                    }
                    .call(c, 0x002a8a94)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x002a8ae0;
                        continue;
                    }
                    pc = 0x002a8aa0;
                    continue;
                }
                0x002a8aa0 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.mult(0, 2, 16, 2);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));

                    if c.g[S3][0] == 0u64 {
                        pc = 0x002a8ab8;
                        continue;
                    }
                    pc = 0x002a8ab8;
                    continue;
                }
                0x002a8ab8 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    pc = 0x002a8abc;
                    continue;
                }
                0x002a8abc => {
                    c.cycles += 9;
                    c.div(0, 2, 19);
                    c.g[T2][0] = c.lo[0];
                    c.g[S0][0] = c.g[T2][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 128i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[S6][0];
                    }
                    c.g[V1][0] = ((c.g[S5][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = 0u64;
                    }

                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                        if back_edge_unwinds(c, 0x002a8a50) {
                            return Ok(());
                        }
                        pc = 0x002a8a50;
                        continue;
                    }
                    pc = 0x002a8ae0;
                    continue;
                }
                0x002a8ae0 => {
                    c.cycles += 17;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                    c.g[V0][0] = c.g[FP][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x1f4), c.g[V1][0]);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a86b8, 0x002a8b24);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, covered, machine};
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate of this module's fakes (in each test machine's RAM), with a
    /// script of `(target, v0)` pairs behind it.
    pub(crate) const OC1031_CTRL: u32 = DATA + 0x21_0000;
    pub(crate) const OC1031_MAGIC: u32 = 0x1031_a5e1;
    pub(crate) const OC1031_TAB: u32 = OC1031_CTRL + 0x10;

    /// Script `v0` for calls to `target`.
    pub(crate) fn oc1031_script(c: &mut Cpu, target: u32, v0: u32) {
        for i in 0..16 {
            let t = c.bus.read32(OC1031_TAB + 8 * i);
            if t == 0 || t == target {
                c.bus.write32(OC1031_TAB + 8 * i, target);
                c.bus.write32(OC1031_TAB + 8 * i + 4, v0);
                return;
            }
        }
        panic!("oc1031 script full");
    }

    /// A logged fake (may unwind) whose `v0` comes from the script.
    pub(crate) fn oc1031_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for i in 0..16 {
            if c.bus.read32(OC1031_TAB + 8 * i) == target {
                let v = c.bus.read32(OC1031_TAB + 8 * i + 4);
                c.g[V0][0] = sx(v);
            }
        }
    }

    macro_rules! oc1031_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1031_CTRL) == OC1031_MAGIC {
                    return oc1031_fake(c, $addr);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    oc1031_gated!(
        oc1031_fake_voice_alloc,
        OC1031_PREV_VOICE_ALLOC,
        0x0028_7968,
        ee::s07::f_00287968__generated
    );
    oc1031_gated!(
        oc1031_fake_voice_start,
        OC1031_PREV_VOICE_START,
        0x0029_06b8,
        ee::s07::f_002906b8__generated
    );
    oc1031_gated!(
        oc1031_fake_replay_playing,
        OC1031_PREV_REPLAY_PLAYING,
        0x0028_8ae0,
        ee::s07::f_00288ae0__generated
    );
    oc1031_gated!(
        oc1031_fake_group0,
        OC1031_PREV_GROUP0,
        0x002a_40e0,
        ee::s08::f_002a40e0__generated
    );
    oc1031_gated!(
        oc1031_fake_group1,
        OC1031_PREV_GROUP1,
        0x002a_4158,
        ee::s08::f_002a4158__generated
    );
    oc1031_gated!(
        oc1031_fake_group3,
        OC1031_PREV_GROUP3,
        0x002a_4238,
        ee::s08::f_002a4238__generated
    );
    oc1031_gated!(
        oc1031_fake_group5,
        OC1031_PREV_GROUP5,
        0x002a_3fd8,
        ee::s08::f_002a3fd8__generated
    );
    oc1031_gated!(
        oc1031_fake_group4,
        OC1031_PREV_GROUP4,
        0x002a_4030,
        ee::s08::f_002a4030__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_chatter,
        OC1031_PREV_DJ_CHATTER,
        0x002a_2e50,
        ee::s08::f_002a2e50__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_20e9,
        OC1031_PREV_DJ_20E9,
        0x002a_23d0,
        ee::s08::f_002a23d0__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_20cc,
        OC1031_PREV_DJ_20CC,
        0x002a_24b0,
        ee::s08::f_002a24b0__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_2102,
        OC1031_PREV_DJ_2102,
        0x002a_2938,
        ee::s08::f_002a2938__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_212e,
        OC1031_PREV_DJ_212E,
        0x002a_2cf0,
        ee::s08::f_002a2cf0__generated
    );
    oc1031_gated!(
        oc1031_fake_dj_212f,
        OC1031_PREV_DJ_212F,
        0x002a_2da0,
        ee::s08::f_002a2da0__generated
    );
    oc1031_gated!(
        oc1031_fake_rand,
        OC1031_PREV_RAND,
        0x002a_df60,
        ee::s08::f_002adf60__generated
    );
    oc1031_gated!(
        oc1031_fake_enter_critical,
        OC1031_PREV_ENTER_CRITICAL,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated
    );
    oc1031_gated!(
        oc1031_fake_midi_play,
        OC1031_PREV_MIDI_PLAY,
        0x003b_9a00,
        ee::s13::f_003b9a00__generated
    );
    oc1031_gated!(
        oc1031_fake_leave_critical,
        OC1031_PREV_LEAVE_CRITICAL,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc1031_gated!(
        oc1031_fake_bank_add,
        OC1031_PREV_BANK_ADD,
        0x002a_cfe0,
        ee::s08::f_002acfe0__generated
    );
    oc1031_gated!(
        oc1031_fake_bank_sample,
        OC1031_PREV_BANK_SAMPLE,
        0x002a_bd38,
        ee::s08::f_002abd38__generated
    );
    oc1031_gated!(
        oc1031_fake_bank_timer,
        OC1031_PREV_BANK_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated
    );

    pub(crate) fn oc1031_install_fakes() {
        macro_rules! hook {
            ($slot:expr, $fake:ident, $prev:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, &$prev);
            };
        }
        hook!(
            ee::s07::OVERRIDE_00287968,
            oc1031_fake_voice_alloc,
            OC1031_PREV_VOICE_ALLOC
        );
        hook!(
            ee::s07::OVERRIDE_002906B8,
            oc1031_fake_voice_start,
            OC1031_PREV_VOICE_START
        );
        hook!(
            ee::s07::OVERRIDE_00288AE0,
            oc1031_fake_replay_playing,
            OC1031_PREV_REPLAY_PLAYING
        );
        hook!(
            ee::s08::OVERRIDE_002A40E0,
            oc1031_fake_group0,
            OC1031_PREV_GROUP0
        );
        hook!(
            ee::s08::OVERRIDE_002A4158,
            oc1031_fake_group1,
            OC1031_PREV_GROUP1
        );
        hook!(
            ee::s08::OVERRIDE_002A4238,
            oc1031_fake_group3,
            OC1031_PREV_GROUP3
        );
        hook!(
            ee::s08::OVERRIDE_002A3FD8,
            oc1031_fake_group5,
            OC1031_PREV_GROUP5
        );
        hook!(
            ee::s08::OVERRIDE_002A4030,
            oc1031_fake_group4,
            OC1031_PREV_GROUP4
        );
        hook!(
            ee::s08::OVERRIDE_002A2E50,
            oc1031_fake_dj_chatter,
            OC1031_PREV_DJ_CHATTER
        );
        hook!(
            ee::s08::OVERRIDE_002A23D0,
            oc1031_fake_dj_20e9,
            OC1031_PREV_DJ_20E9
        );
        hook!(
            ee::s08::OVERRIDE_002A24B0,
            oc1031_fake_dj_20cc,
            OC1031_PREV_DJ_20CC
        );
        hook!(
            ee::s08::OVERRIDE_002A2938,
            oc1031_fake_dj_2102,
            OC1031_PREV_DJ_2102
        );
        hook!(
            ee::s08::OVERRIDE_002A2CF0,
            oc1031_fake_dj_212e,
            OC1031_PREV_DJ_212E
        );
        hook!(
            ee::s08::OVERRIDE_002A2DA0,
            oc1031_fake_dj_212f,
            OC1031_PREV_DJ_212F
        );
        hook!(
            ee::s08::OVERRIDE_002ADF60,
            oc1031_fake_rand,
            OC1031_PREV_RAND
        );
        hook!(
            ee::s13::OVERRIDE_003B58A0,
            oc1031_fake_enter_critical,
            OC1031_PREV_ENTER_CRITICAL
        );
        hook!(
            ee::s13::OVERRIDE_003B9A00,
            oc1031_fake_midi_play,
            OC1031_PREV_MIDI_PLAY
        );
        hook!(
            ee::s13::OVERRIDE_003B58D8,
            oc1031_fake_leave_critical,
            OC1031_PREV_LEAVE_CRITICAL
        );
        hook!(
            ee::s08::OVERRIDE_002ACFE0,
            oc1031_fake_bank_add,
            OC1031_PREV_BANK_ADD
        );
        hook!(
            ee::s08::OVERRIDE_002ABD38,
            oc1031_fake_bank_sample,
            OC1031_PREV_BANK_SAMPLE
        );
        hook!(
            ee::s08::OVERRIDE_002ADCA0,
            oc1031_fake_bank_timer,
            OC1031_PREV_BANK_TIMER
        );
    }

    /// Fake machine: the gate on, the fakes hooked.
    pub(crate) fn oc1031_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1031_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1031_CTRL, OC1031_MAGIC);
        r
    }

    pub(crate) const OC1031_AUDIO: u32 = DATA + 0x100;
    pub(crate) const OC1031_RIDER: u32 = DATA + 0x300;

    /// The audio object with its voice array, per-level arrays and a rider:
    /// `audio[0] = arr`, `audio[1]`/`audio[2]` level tables, `audio[0x118] ->
    /// &arr`, `arr[0x1f4]` the level index (0..2), `arr[0x1d8..]` the params
    /// whose arrays all point to their own 256-byte areas, the default entry's
    /// volume byte (`arr[0x1dc]`) cycling through 127, negative, small. The
    /// rider has index `+0x870` (0..3) and, by case, `+0x874`/`+0x87c` clear
    /// or set. `table_value` is the rider's "last played" float.
    pub(crate) fn oc1031_voice_world(c: &mut Cpu, r: &mut Rng, seed: u32, table_value: u32) {
        let audio = OC1031_AUDIO;
        let pp = DATA + 0x200;
        let arr = DATA + 0x1000;
        let params = arr + 0x1d8;
        c.bus.write32(audio, arr);
        c.bus.write32(audio + 4, DATA + 0x5100);
        c.bus.write32(audio + 8, DATA + 0x5200);
        c.bus.write32(audio + 0x118, pp);
        c.bus.write32(pp, arr);
        fill(c, r, params, 0x30);
        c.bus.write32(arr + VOICE_TOP, seed % 3);
        c.bus.write32(params + 0x20, DATA + 0x3000);
        for (i, off) in [
            0x34u32, 0x38, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus
                .write32(params + off, DATA + 0x4000 + 0x100 * i as u32);
        }
        c.bus.write32(params + 0x3c, DATA + 0x5000);
        c.bus.write8(
            arr + 0x1dc,
            [0x7f, 0x90, 0x05, 0x7e][(seed / 3 % 4) as usize],
        );
        let rider = OC1031_RIDER;
        let idx = seed % 4;
        c.bus.write32(rider + 0x870, idx);
        let case = seed / 4 % 4;
        c.bus
            .write32(rider + 0x874, if case == 0 { 0 } else { r.next_u32() | 1 });
        c.bus
            .write32(rider + 0x87c, if case == 1 { 0 } else { r.next_u32() | 1 });
        c.bus.write32(audio + 0x6080 + 4 * idx, table_value);
        c.g[A0][0] = sx(audio);
        c.g[A1][0] = sx(rider);
    }

    /// Handplant: the rider flags gate it; the voice level is queued and the
    /// handle stored (both fakes may unwind).
    #[test]
    fn ssx_audio_system_play_handplant_sfx_matches() {
        covered(
            ee::s08::f_0029dc48__generated,
            ssx_audio_system_play_handplant_sfx,
            300,
            &|c, seed| {
                let mut r = oc1031_machine(c, seed, 0x0029_dc48, 2);
                c.pc = 0x0029_dc48;
                oc1031_voice_world(c, &mut r, seed, 0);
            },
        );
    }

    /// Wipeout impact: replay-playing gate, rider flags, the amount below and
    /// above the rider's last value (both signs), the three volume clamp cases.
    #[test]
    fn ssx_audio_system_play_wipeout_impact_sfx_matches() {
        covered(
            ee::s08::f_0029e590__generated,
            ssx_audio_system_play_wipeout_impact_sfx,
            300,
            &|c, seed| {
                let mut r = oc1031_machine(c, seed, 0x0029_e590, 3);
                c.pc = 0x0029_e590;
                let last = r.f32(8.0);
                oc1031_voice_world(c, &mut r, seed, last);
                c.f[12] = r.f32(8.0);
                oc1031_script(c, 0x0028_8ae0, (seed % 7 == 0) as u32);
            },
        );
    }

    /// Wipeout slide: as the impact, plus the rider's slot already 1.0
    /// ("playing": nothing happens).
    #[test]
    fn ssx_audio_system_play_wipeout_slide_sfx_matches() {
        covered(
            ee::s08::f_0029e970__generated,
            ssx_audio_system_play_wipeout_slide_sfx,
            300,
            &|c, seed| {
                let mut r = oc1031_machine(c, seed, 0x0029_e970, 3);
                c.pc = 0x0029_e970;
                let last = if seed % 3 == 0 {
                    0x3f80_0000
                } else {
                    r.f32(2.0)
                };
                oc1031_voice_world(c, &mut r, seed, last);
                oc1031_script(c, 0x0028_8ae0, (seed % 7 == 0) as u32);
            },
        );
    }

    /// DJ result chatter: the pending flag, the three candidate remarks
    /// (`+0x5800`, `+0x5804 >= 5`, `+0x5808` against the limit 24 / 27 by
    /// track-id group), the no-candidate location remarks (group 5, 4, other),
    /// the "said last" removal and every pick.
    #[test]
    fn ssx_audio_system_play_dj_result_chatter_matches() {
        check_reaching(
            ee::s08::f_002a4770__generated,
            ssx_audio_system_play_dj_result_chatter,
            600,
            &[
                0x002a_4808,
                0x002a_481c,
                0x002a_4830,
                0x002a_487c,
                0x002a_488c,
                0x002a_48a0,
                0x002a_48b0,
                0x002a_48c0,
                0x002a_4910,
                0x002a_4998,
                0x002a_49ac,
                0x002a_49bc,
            ],
            &|c, seed| {
                let mut r = oc1031_machine(c, seed, 0x002a_4770, 4);
                c.pc = 0x002a_4770;
                let audio = OC1031_AUDIO;
                let pp = DATA + 0x200;
                let arr = DATA + 0x1000;
                c.bus.write32(audio + 0x118, pp);
                c.bus.write32(pp, arr);
                c.bus.write32(arr + 0x1d8, r.next_u32());
                c.bus.write32(audio + 0x57f8, (seed % 9 != 0) as u32);
                c.bus.write32(audio + 0x57fc, (seed % 7 != 0) as u32);
                c.bus.write32(audio + 0x5800, seed / 2 % 2);
                c.bus
                    .write32(audio + 0x5804, [0, 4, 5, 9][(seed / 3 % 4) as usize]);
                c.bus.write32(
                    audio + 0x5808,
                    [0x10, 0x17, 0x18, 0x1a, 0x1b, 0x1c][(seed / 5 % 6) as usize],
                );
                c.bus
                    .write32(audio + 0x580c, [u32::MAX, 0, 1, 2][(seed / 7 % 4) as usize]);
                c.bus.write32(audio + 0x5810, r.next_u32());
                c.bus
                    .write32(audio + 0x5814, [0x17, 3, 5, 0x20][(seed / 11 % 4) as usize]);
                c.bus.write32(audio + 0x6254, (seed % 3 != 0) as u32);
                for (i, target) in [
                    0x002a_40e0u32,
                    0x002a_4158,
                    0x002a_4238,
                    0x002a_3fd8,
                    0x002a_4030,
                ]
                .into_iter()
                .enumerate()
                {
                    oc1031_script(c, target, (seed >> (i + 1)) & 1);
                }
                oc1031_script(c, 0x002a_df60, r.next_u32());
                c.g[A0][0] = sx(audio);
            },
        );
    }

    /// MIDIBanks_Play: the level with and without a position, with and without
    /// a volume control, radius zero (with and without a bank) or set, the
    /// bank monitor adding an instance or not, echo timers of 0..3.
    #[test]
    fn midi_banks_play_matches() {
        check_reaching(
            ee::s08::f_002a86b8__generated,
            midi_banks_play,
            600,
            &[0x002a_8988, 0x002a_89d0, 0x002a_89f0, 0x002a_8a94],
            &|c, seed| {
                let mut r = oc1031_machine(c, seed, 0x002a_86b8, 8);
                c.pc = 0x002a_86b8;
                let sys = OC1031_AUDIO;
                let mgr = DATA + 0x1000;
                let params = mgr + 0x1d8;
                let top = seed % 3;
                c.bus.write32(sys, mgr);
                c.bus.write32(sys + 0x10, DATA + 0x2000);
                fill(c, &mut r, params, 0x30);
                c.bus.write32(mgr + VOICE_TOP, top);
                c.bus.write32(params + 0x20, DATA + 0x3000);
                c.bus.write32(params + 0x38, DATA + 0x3400);
                c.bus.write32(params + 0x3c, DATA + 0x3500);
                c.bus.write32(params + 0x6c, DATA + 0x3600);
                c.bus.write32(mgr + 0xacc, DATA + 0x2400);
                c.bus.write32(mgr + 0x248, DATA + 0x2800);
                // Level position (set or clear), base volume, volume control.
                c.bus.write32(
                    mgr + 0x1fc + 4 * top,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    DATA + 0x3400 + 4 * top,
                    [10u32, 100, 127, 200, 0xffff_fff0, 0][(seed / 2 % 6) as usize],
                );
                let ctl = if seed / 3 % 3 == 0 {
                    0
                } else {
                    DATA + 0x3700 + 4 * top
                };
                c.bus.write32(DATA + 0x3500 + 4 * top, ctl);
                let gain = [0u32, 0x3f80_0000, 0x4000_0000, 0xbf80_0000, 0x3f00_0000];
                c.bus
                    .write32(DATA + 0x3700 + 4 * top, gain[(seed / 5 % 5) as usize]);
                c.bus.write32(
                    DATA + 0x3600 + 4 * top,
                    if seed / 7 % 3 == 0 { 0x3f80_0000 } else { 0 },
                );
                // Sound table entry {id, sound}; bank table entry of that id.
                let id = seed / 11 % 4;
                c.bus.write32(DATA + 0x2000 + 8 * top, id);
                c.bus.write32(DATA + 0x2004 + 8 * top, r.next_u32());
                c.bus
                    .write32(DATA + 0x2400 + 0x60 * id, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x2404 + 0x60 * id, r.next_u32() % 40);
                // Echo timers: count, time, feedback, level.
                let d = DATA + 0x2800 + 16 * top;
                c.bus.write32(d, r.next_u32() % 500);
                c.bus.write32(d + 4, (seed / 13 % 4) as u32);
                c.bus.write32(d + 8, r.next_u32() % 150);
                c.bus.write32(d + 0xc, r.next_u32() % 150);
                oc1031_script(
                    c,
                    0x002a_cfe0,
                    if seed / 3 % 4 == 0 { 0 } else { DATA + 0x5000 },
                );
                c.g[A0][0] = sx(sys);
            },
        );
    }
}
