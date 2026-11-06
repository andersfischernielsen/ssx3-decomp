//! Staging wave 210 (oc284): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_74a0,
        ssx_audio_system_play_object_sound,
        "SSXAudioSystem_PlayObjectSound",
    ),
    (0x0023_1348, game_module_save_state, "GameModule_SaveState"),
    (
        0x0028_8a20,
        ssx_audio_system_get_rider_listener_index,
        "SSXAudioSystem_GetRiderListenerIndex",
    ),
    (0x0031_0ea8, rider_model_save_state, "RiderModel_SaveState"),
    (
        0x0013_c5e0,
        rider_railslide_state_save_state,
        "RiderRailslideState_SaveState",
    ),
    (
        0x0013_1598,
        rider_board_press_sub_save_state,
        "RiderBoardPressSub_SaveState",
    ),
    (0x0035_2ae8, modifier_block_dtor, "ModifierBlock_dtor"),
    (
        0x003a_8cd0,
        world_cache_resolve_resource,
        "cWorldCache_ResolveResource",
    ),
    (0x003c_72a0, snd_random_range, "SND_RandomRange"),
    (0x0039_1480, vu0_copy_output_mem, "Vu0_CopyOutputMem"),
];

// ---------------------------------------------------------------------------
// 0x002974a0 SSXAudioSystem_PlayObjectSound

/// Offset of the object-sound "suppressed" word in the audio system (a
/// non-negative value means object sounds are off).
const POS_SUPPRESS: u32 = 0x62a8;

/// `void SSXAudioSystem_PlayObjectSound(SSXAudioSystem* self, WorldObject*
/// obj, int sound)` (0x2974a0): play a world object's one-shot sound at the
/// object's position (`obj+0x40`). Sound 0 is nothing; sound 0x66 only plays
/// when the current track's flag word is set; nothing plays while
/// `self+0x62a8` is non-negative or the object already has a tracked sound.
/// The sound number picks the bank: <100 bank 2, <150 bank 8 (minus 100),
/// <200 bank 9 (minus 150), else bank 4 (minus 200). The play record is
/// pushed inline (the `PushPlayParams` sequence), then the 3D position,
/// radius 300 / no vanish, listener and priority 0x50 are set, the sound is
/// started and tracked against the object.
pub fn ssx_audio_system_play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let played = c.g[S0][0] == 0 || play_object_sound_body(c)?;
        if played {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x80);
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`ssx_audio_system_play_object_sound`] for a non-zero sound
/// (in s0; self in s3, object in s6). `Ok(true)` when it ran to the end (the
/// sound was played), `Ok(false)` on an early exit.
fn play_object_sound_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 0x66;
    if c.g[S0][0] == 0x66 {
        // Sound 0x66: only when the current track's flag is set.
        c.cycles += 10;
        call_ptr(c, 0x0014_dc80, 0x0029_74e8)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_74f4)?; // BELibrary_GetInterface(be, 0)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0029_74fc)?; // the current track
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] == 0 {
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(POS_SUPPRESS));
    c.cycles += 2;
    if (c.g[V0][0] as i64) >= 0 {
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = 0;
    Callee {
        addr: 0x0029_5028,
        f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
    }
    .call(c, 0x0029_7524)?;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    // The object's position, copied to the frame (the 3D source).
    c.cycles += 5;
    c.lq(2, (c.g[S6][0] as u32).wrapping_add(0x40));
    c.g[S5][0] = 0x2;
    let n = c.g[S0][0] as i64;
    c.g[V1][0] = (n < 100) as u64;
    c.sq(2, c.g[SP][0] as u32);
    // Bank (s5) and sound within the bank (s4).
    if n < 100 {
        c.cycles += 2;
        c.g[S4][0] = c.g[S0][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = (n < 150) as u64;
        c.g[S4][0] = sx((n as u32).wrapping_sub(100));
        if n < 150 {
            c.cycles += 2;
            c.g[S5][0] = 0x8;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (n < 200) as u64;
            c.g[S4][0] = sx((n as u32).wrapping_sub(150));
            if n < 200 {
                c.cycles += 2;
                c.g[S5][0] = 0x9;
            } else {
                c.cycles += 2;
                c.g[S4][0] = sx((n as u32).wrapping_sub(200));
                c.g[S5][0] = 0x4;
            }
        }
    }
    // Push a play record (the inlined PushPlayParams), bank s5, sound s4;
    // then the channel volume clamped to 0..=127 into the record's first byte.
    c.cycles += 154;
    c.g[S0][0] = c.lw((c.g[S3][0] as u32));
    c.g[S2][0] = 0x18;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x8;
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
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x00416210, 0x0029764c)?;
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
    c.g[A0][0] = c.lw((c.g[S3][0] as u32));
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S5][0]);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S4][0]);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
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
    let at_least_max = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    let mut negative = false;
    if !at_least_max {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
        }
    }
    // Channel gain, 3D source (position on the frame), radius 300 / no
    // vanish, listener, priority 0x50; play and track it against the object.
    c.cycles += 64;
    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[A2][0] = 0u64;
    c.g[A1][0] = 0x5;
    c.g[A0][0] = c.g[S3][0];
    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    call_ptr(c, 0x00287968, 0x0029783c)?;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
    c.g[A2][0] = 0x1;
    c.g[AT][0] = 0x43960000;
    c.mtc1(1, 12);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 13);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[SP][0]);
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    Callee {
        addr: 0x002a9988,
        f: ee::s08::SndPlayParams_SetRadiusVanish,
    }
    .call(c, 0x002978ac)?;
    c.g[A1][0] = c.g[SP][0];
    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x00288940, 0x002978bc)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = c.g[S3][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x23c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x250));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = 0x50;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[V1][0]);
    call_ptr(c, 0x002906b8, 0x002978fc)?;
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x260));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x32));
    Callee {
        addr: 0x00295628,
        f: ee::s08::SSXAudioSystem_TrackSound,
    }
    .call(c, 0x00297924)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// 0x00231348 GameModule_SaveState

/// The game module's state machine members saved by
/// [`game_module_save_state`]: (offset in the module, the state's
/// `SaveState`, return address of the call).
const GAME_MODULE_STATES: [(u32, Callee, u32); 15] = [
    (
        0xb0,
        Callee {
            addr: 0x0023_4990,
            f: ee::s06::GameStatePreRaceLoad_SaveState,
        },
        0x0023_1410,
    ),
    (
        0xd0,
        Callee {
            addr: 0x0023_6d60,
            f: ee::s06::GameStatePreRaceMenu_SaveState,
        },
        0x0023_141c,
    ),
    (
        0xe0,
        Callee {
            addr: 0x0023_4de0,
            f: ee::s06::GameStateCountdown_SaveState,
        },
        0x0023_1428,
    ),
    (
        0xf0,
        Callee {
            addr: 0x0023_4eb8,
            f: ee::s06::GameStateRace_SaveState,
        },
        0x0023_1434,
    ),
    (
        0x100,
        Callee {
            addr: 0x0023_40b8,
            f: ee::s06::GameStateFinish_SaveState,
        },
        0x0023_1440,
    ),
    (
        0x118,
        Callee {
            addr: 0x0023_3b90,
            f: ee::s06::GameStateQuit_SaveState,
        },
        0x0023_144c,
    ),
    (
        0x128,
        Callee {
            addr: 0x0023_6ec8,
            f: ee::s06::GameStateResults_SaveState,
        },
        0x0023_1458,
    ),
    (
        0x138,
        Callee {
            addr: 0x0023_3bd0,
            f: ee::s06::GameStateIdle_SaveState,
        },
        0x0023_1464,
    ),
    (
        0x148,
        Callee {
            addr: 0x0023_6b70,
            f: ee::s06::GameStateResetAI_SaveState,
        },
        0x0023_1470,
    ),
    (
        0x158,
        Callee {
            addr: 0x0023_5990,
            f: ee::s06::GameStateReset_SaveState,
        },
        0x0023_147c,
    ),
    (
        0x178,
        Callee {
            addr: 0x0023_6af8,
            f: ee::s06::GameStateEventSwitchLoad_SaveState,
        },
        0x0023_1488,
    ),
    (
        0x18c,
        Callee {
            addr: 0x0023_6fa8,
            f: ee::s06::GameStateFinishCinematic_SaveState,
        },
        0x0023_1494,
    ),
    (
        0x19c,
        Callee {
            addr: 0x0023_5f48,
            f: ee::s06::GameStateRestartRace_SaveState,
        },
        0x0023_14a0,
    ),
    (
        0x1b8,
        Callee {
            addr: 0x0023_6728,
            f: ee::s06::GameStateEventSelect_SaveState,
        },
        0x0023_14ac,
    ),
    (
        0x1dc,
        Callee {
            addr: 0x0023_61c8,
            f: ee::s06::GameStateWarpToEvent_SaveState,
        },
        0x0023_14b8,
    ),
];

/// `void GameModule_SaveState(GameModule* self, cStream* stream)`
/// (0x231348): serialize the module into a save stream: the two serializable
/// members at `+0x70`/`+0x74` (virtual `serialize`, vtable slot 0x60), the
/// four words at `+0x214`, `+0x218`, `+0x210`, `+0x21c` (`stream->write(p,
/// 4)`, slot 8), then every game state's `SaveState`.
pub fn game_module_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 97;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        // The serializable members: obj->serialize(stream) (a1 is the stream).
        for (member, ret) in [(0x70u32, 0x0023_1378u32), (0x74, 0x0023_1394)] {
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(member));
            if member == 0x74 {
                c.g[A1][0] = c.g[S0][0];
            }
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, ret)?;
        }
        // stream->write(&self->field, 4) for the four words.
        for (i, (field, ret)) in [
            (0x214u32, 0x0023_13b0u32),
            (0x218, 0x0023_13cc),
            (0x210, 0x0023_13e8),
            (0x21c, 0x0023_1404),
        ]
        .into_iter()
        .enumerate()
        {
            let vt = c.lw(c.g[S0][0] as u32);
            if i == 0 {
                c.g[A3][0] = vt;
            } else {
                c.g[V1][0] = vt;
            }
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(field));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((vt as u32).wrapping_add(0xc));
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, ret)?;
        }
        // Each game state: state->SaveState(stream).
        for (member, save, ret) in GAME_MODULE_STATES {
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(member));
            c.g[A1][0] = c.g[S0][0];
            save.call(c, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00288a20 SSXAudioSystem_GetRiderListenerIndex

/// Rider fields read by [`ssx_audio_system_get_rider_listener_index`].
const RLI_FIXED_FLAG: u32 = 0x874;
const RLI_FIXED_ON: u32 = 0x87c;
const RLI_FIXED_INDEX: u32 = 0x870;
/// The rider's embedded object whose vtable slot 5 (`+0x28`) gives its
/// world position.
const RLI_POS_OBJECT: u32 = 0x6c0;

/// `int SSXAudioSystem_GetRiderListenerIndex(SSXAudioSystem* self, Rider*
/// rider)` (0x288a20): which listener (view) hears this rider. 0 outside the
/// game; the rider's fixed index (`+0x870`) when it has one (`+0x874` and
/// `+0x87c` set); 0 with a single listener; else whether the rider's
/// position is closer to the second view.
pub fn ssx_audio_system_get_rider_listener_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Callee {
            addr: 0x0028_b1b0,
            f: ee::s07::SSXAudioSystem_IsInGame,
        }
        .call(c, 0x0028_8a3c)?;
        let in_game = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        // Ends at the shared "restore s0" block (+1) unless the second-view
        // test ran.
        let mut via_restore = true;
        if in_game {
            let rider = c.g[S0][0] as u32;
            c.cycles += 3;
            c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_FLAG));
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_ON));
                c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_INDEX));
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0028_8ce8,
                    f: ee::s07::SSXAudioSystem_GetListenerCount,
                }
                .call(c, 0x0028_8a70)?;
                c.g[V1][0] = 0x1;
                let rider = c.g[S0][0] as u32;
                c.g[A1][0] = sx(rider.wrapping_add(RLI_POS_OBJECT));
                if c.g[V0][0] == c.g[V1][0] {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(rider.wrapping_add(RLI_POS_OBJECT));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_8a90)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0028_8940,
                        f: ee::s07::SSXAudioSystem_IsCloserToSecondView,
                    }
                    .call(c, 0x0028_8a9c)?;
                    via_restore = false;
                }
            }
        }
        if via_restore {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00310ea8 RiderModel_SaveState

/// RiderModel fields: bone count and the bone array (32 bytes per bone).
const RM_NUM_BONES: u32 = 0x10;
const RM_BONES: u32 = 0x2c;

/// `void RiderModel_SaveState(RiderModel* self, cStream* stream)`
/// (0x310ea8): write the bone array (`numBones * 32` bytes) to the stream
/// (`stream->write`, vtable slot 1 at `+0x8`). The twin of
/// `RiderModel_LoadState`.
pub fn rider_model_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[T0][0] = stream;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(stream as u32);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RM_NUM_BONES));
        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(RM_BONES));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let write = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, write, 0x0031_0ed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0013c5e0 RiderRailslideState_SaveState, 0x00131598
// RiderBoardPressSub_SaveState

/// The common body of the small rider-state `SaveState`s: `stream->write(
/// self, size)` (a0 = self, a1 = stream; the write is vtable slot 1 at
/// `+0x8`) returning to `ret`.
fn rider_state_save_self_bytes(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = size;
        c.g[V1][0] = c.lw(c.g[A3][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let write = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, write, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderRailslideState_SaveState(RiderRailslideState* self, cStream*
/// stream)` (0x13c5e0): write the state's first 0x50 bytes to the stream.
pub fn rider_railslide_state_save_state(c: &mut Cpu) {
    rider_state_save_self_bytes(c, 0x50, 0x0013_c608);
}

/// `void RiderBoardPressSub_SaveState(RiderBoardPressSub* self, cStream*
/// stream)` (0x131598): write the sub-state's first 0x14 bytes to the
/// stream.
pub fn rider_board_press_sub_save_state(c: &mut Cpu) {
    rider_state_save_self_bytes(c, 0x14, 0x0013_15c0);
}

// ---------------------------------------------------------------------------
// 0x00352ae8 ModifierBlock_dtor

/// ModifierBlock fields written by [`modifier_block_dtor`]: the vtables of
/// the embedded rail-modifier list (`+0x1c`, vtable at `+0x24`) and
/// modifier list node (`+0x10`, vtable at `+0x18`).
const MB_MOD_LIST: u32 = 0x10;
const MB_MOD_LIST_VTABLE: u32 = 0x18;
const MB_RAIL_LIST: u32 = 0x1c;
const MB_RAIL_LIST_VTABLE: u32 = 0x24;

/// `ModifierBlock* ModifierBlock_dtor(ModifierBlock* self, int flags)`
/// (0x352ae8): free the transform, UV-scroll, texture-flip, boost and
/// particle modifiers and every rail modifier (owner -1), restore the two
/// embedded lists' base vtables and unlink them; `operator delete` when
/// `flags & 1`.
pub fn modifier_block_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        let frees = [
            Callee {
                addr: 0x0035_30d0,
                f: ee::s11::ModifierBlock_FreeTransform,
            },
            Callee {
                addr: 0x0035_3118,
                f: ee::s11::ModifierBlock_FreeUVScroll,
            },
            Callee {
                addr: 0x0035_3150,
                f: ee::s11::ModifierBlock_FreeTexFlip,
            },
            Callee {
                addr: 0x0035_3188,
                f: ee::s11::ModifierBlock_FreeBoost,
            },
            Callee {
                addr: 0x0035_31d0,
                f: ee::s11::ModifierBlock_FreeParticles,
            },
        ];
        for (i, free) in frees.into_iter().enumerate() {
            if i > 0 {
                c.g[A0][0] = c.g[S1][0];
            }
            free.call(c, 0x0035_2b04 + 8 * i as u32)?;
        }
        // Every rail modifier, whatever its owner (-1).
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Callee {
            addr: 0x0035_3300,
            f: ee::s11::ModifierBlock_FreeRailModifiers,
        }
        .call(c, 0x0035_2b34)?;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = 0x0049_1200;
        c.g[A0][0] = sx(this.wrapping_add(MB_RAIL_LIST));
        c.sw(this.wrapping_add(MB_RAIL_LIST_VTABLE), 0x0049_1200);
        Callee {
            addr: 0x0035_b6d0,
            f: ee::s11::RailModifier_ListUnlink,
        }
        .call(c, 0x0035_2b48)?;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = 0x0049_1340;
        c.g[A0][0] = sx(this.wrapping_add(MB_MOD_LIST));
        c.sw(this.wrapping_add(MB_MOD_LIST_VTABLE), 0x0049_1340);
        Callee {
            addr: 0x0034_5760,
            f: ee::s11::ModifierList_Unlink,
        }
        .call(c, 0x0035_2b5c)?;
        c.g[S0][0] &= 0x1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0035_2b70)?; // operator delete
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003a8cd0 cWorldCache_ResolveResource

/// cWorldCache fields used by [`world_cache_resolve_resource`].
const WC_DEFAULT_TABLE: u32 = 0x4;
const WC_TABLES: u32 = 0x8;
const WC_LOADER: u32 = 0x3e8;
/// The per-group pending lists (24 bytes per group; the list pointer first).
const WC_GROUPS: u32 = 0x3f8;
/// The "no handle" sentinel object (`D_004a47d0`, gp+0x16e0).
const WC_NO_HANDLE_GP: u32 = 0x16e0;

/// `void cWorldCache_ResolveResource(cWorldCache* self, int group, int id,
/// u32 key, int handle)` (0x3a8cd0): record (id, key low byte, key >> 8) in
/// the group's pending list, ask the loader (`self+0x3e8`, vtable slot 3 at
/// `+0x18`) to resolve it (with handle 0 when `handle` is the no-handle
/// sentinel, then keeping the sentinel; otherwise the loader's result becomes
/// the handle), and merge `(handle >> 2) << 8` into the resource's state word
/// (`table[id].bits[key >> 8]`, table `key & 0xff` or the default for 0xff).
pub fn world_cache_resolve_resource(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[T1][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        c.g[T2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = c.g[T1][0] & 0xff;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx((c.g[T1][0] as u32) >> 8);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xff;
        c.g[S0][0] = c.g[T0][0];
        let this = c.g[T2][0] as u32;
        // The resource table: by the key's low byte, 0xff the default one.
        if c.g[A0][0] == 0xff {
            c.cycles += 2;
            c.g[S2][0] = c.lw(this.wrapping_add(WC_DEFAULT_TABLE));
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw(this.wrapping_add(WC_TABLES));
            c.g[V0][0] = sx(((c.g[A0][0] as u32) << 2).wrapping_add(c.g[V1][0] as u32));
            c.g[S2][0] = c.lw(c.g[V0][0] as u32);
        }
        // Append (id, key low byte, key >> 8) to the group's pending list.
        c.cycles += 25;
        c.g[V1][0] = 0x18;
        c.g[A2][0] = sx((c.g[T1][0] as u32) >> 8);
        c.mult(0, V0, A1, V1);
        c.g[A3][0] = sx((c.g[GP][0] as u32).wrapping_add(WC_NO_HANDLE_GP));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(WC_GROUPS));
        let list = c.g[A1][0] as u32;
        let entry = |c: &mut Cpu| -> u32 {
            let count = c.lw(list.wrapping_add(0x8)) as u32;
            let entries = c.lw(list.wrapping_add(0xc)) as u32;
            (count << 2).wrapping_add(entries)
        };
        c.g[V0][0] = sx(entry(c));
        c.sb(c.g[V0][0] as u32, c.g[S1][0]);
        c.g[V1][0] = sx(entry(c));
        c.g[V0][0] = c.lw(list.wrapping_add(0xc));
        c.sb((c.g[V1][0] as u32).wrapping_add(0x1), c.g[T1][0]);
        c.g[A0][0] = sx(entry(c));
        c.g[V0][0] = c.lw(list.wrapping_add(0xc));
        c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[A2][0]);
        c.g[V0][0] = sx((c.lw(list.wrapping_add(0x8)) as u32).wrapping_add(1));
        c.sw(list.wrapping_add(0x8), c.g[V0][0]);
        // loader->Resolve(id, key, handle or 0).
        let no_handle = c.g[S0][0] == c.g[A3][0];
        c.cycles += 11;
        let this = c.g[T2][0] as u32;
        c.g[T0][0] = c.lw(this.wrapping_add(WC_LOADER));
        c.g[A3][0] = if no_handle { 0 } else { c.g[S0][0] };
        c.g[A2][0] = c.g[T1][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let resolve = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if no_handle {
            call_ptr(c, resolve, 0x003a_8da8)?;
        } else {
            call_ptr(c, resolve, 0x003a_8dd4)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[V0][0] = sx((c.g[S1][0] as u32) << 3);
        // table[id].bits[key >> 8] |= (handle >> 2) << 8.
        c.cycles += 16;
        c.g[A0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 2) as u32);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
        frame.restore(c, S0, 0x40);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
        frame.restore(c, S2, 0x20);
        c.g[V0][0] |= c.g[V1][0];
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003c72a0 SND_RandomRange

/// `int SND_RandomRange(int range)` (0x3c72a0): a random value in about
/// `[-range, range)`: `range` is clamped to `0..=0x10000`, then
/// `((SND_Random() & 0x7fff) - 0x4000) * range >> 14`.
pub fn snd_random_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let range = c.g[S0][0] as i64;
        c.g[V0][0] = (0x10000 < range) as u64;
        if range > 0x10000 {
            c.cycles += 2;
            c.g[S0][0] = 0x10000;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (-1 < range) as u64;
            if range <= -1 {
                c.g[S0][0] = 0;
            }
        }
        c.cycles += 10;
        Callee {
            addr: 0x003b_c5c8,
            f: ee::s13::SND_Random,
        }
        .call(c, 0x003c_72d8)?;
        c.g[V0][0] &= 0x7fff;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4000));
        c.mult(0, V0, V0, S0);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 14) as u32);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00391480 Vu0_CopyOutputMem

/// VU0 data memory (EE address) where the microprograms leave their output.
const VU0_OUTPUT_MEM: u32 = 0x1100_4200;

/// `void Vu0_CopyOutputMem(int unused, u128* dst)` (0x391480): copy the 100
/// quadwords of VU0 output memory (`0x11004200`) to `dst`, ten per loop
/// pass (a0 = source, a1 = destination, a2 = passes left - 1; an event check
/// at each back edge).
pub fn vu0_copy_output_mem(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = VU0_OUTPUT_MEM as u64;
    c.g[A2][0] = 0x9;
    loop {
        c.cycles += 24;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        for k in 0..10u32 {
            // Alternating v1 / v0, as the original's unrolled body.
            let reg = if k % 2 == 0 { V1 } else { V0 };
            c.lq(reg, (c.g[A0][0] as u32).wrapping_add(0x10 * k));
            if k == 9 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
                c.sq(reg, (c.g[A1][0] as u32).wrapping_add(0x90));
            } else {
                c.sq(reg, (c.g[A1][0] as u32).wrapping_add(0x10 * k));
            }
        }
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_1490);
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;

    /// SSXAudioSystem_PlayObjectSound: sound 0, sound 0x66 (the back end's
    /// interface getter is a fake that may unwind; the current track's flag
    /// word 0 or set), suppressed or not, the object already tracked (one
    /// queued record with its key and handle 0) or not, and every bank range.
    /// The voice records are the MIDI-bank fixture of the PlaySound tests
    /// (real GetChannelGainPtr / Audio_PlayPushedSound); one listener (the
    /// real IsCloserToSecondView returns at once); TrackSound finds no
    /// playing instance.
    #[test]
    fn ssx_audio_system_play_object_sound_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        const THIS: u32 = DATA + 0x100;
        const OBJ: u32 = DATA + 0x4_8000;
        const WORLD: u32 = DATA + 0x4_4000;
        const INSTANCES: u32 = DATA + 0x4_0000;
        const RING_DATA: u32 = DATA + 0x5_0000;
        const BE_LIBRARY: u32 = DATA + 0x5_8000;
        const BE_OBJ: u32 = DATA + 0x5_8100;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        const TRACK_FLAG: u32 = 0x0043_d950;
        const RING: u32 = 0x597c;
        check(
            ee::s08::f_002974a0__generated,
            ssx_audio_system_play_object_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_74a0, 1);
                c.pc = 0x0029_74a0;
                midi_bank(c, &mut r, seed);
                let sound = [
                    0u32, 0x66, 0x66, 5, 99, 100, 120, 149, 150, 170, 199, 200, 230,
                ][(seed % 13) as usize];
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = sound as u64;
                fill(c, &mut r, OBJ + 0x40, 4);
                // Object sounds on (negative) or suppressed.
                let suppress = match (seed / 13) % 5 {
                    0 => 0,
                    1 => r.next_u32() & 0x7fff_ffff,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                c.bus.write32(THIS + POS_SUPPRESS, suppress);
                // Sound 0x66: the back end and the current track's flag.
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(BE_OBJ + 4, vt);
                c.bus
                    .write32(TRACK_FLAG, if seed % 26 == 1 { 0 } else { 1 });
                // One listener: IsCloserToSecondView returns 0 at once.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                // The tracked-sound ring (10 words): empty, or one record
                // for this object (key, handle 0, ..., subkey 0).
                c.bus.write32(THIS + RING, 10);
                c.bus.write32(THIS + RING + 4, RING_DATA);
                let read = r.next_u32() % 10;
                let tracked = seed % 7 == 3;
                let write = if tracked { (read + 5) % 10 } else { read };
                c.bus.write32(THIS + RING + 8, write);
                c.bus.write32(THIS + RING + 0xc, read);
                for k in 0..10 {
                    c.bus.write32(RING_DATA + 4 * k, r.next_u32());
                }
                if tracked {
                    for k in 0..5 {
                        let w = if k == 0 { OBJ } else { 0 };
                        c.bus.write32(RING_DATA + 4 * ((read + k) % 10), w);
                    }
                }
                // The bank monitor's instances (all free).
                c.bus.write32(DATA + 0x700 + 0x1d8 + 0x8ec, INSTANCES);
            },
        );
    }

    /// GameModule_SaveState: the two members' `serialize` and every stream
    /// write (all through the state SaveStates) are fakes (some unwind); the
    /// state objects are random words.
    #[test]
    fn game_module_save_state_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA};
        const THIS: u32 = DATA + 0x2_0000;
        const STREAM: u32 = DATA + 0x2_1000;
        const MEMBERS: u32 = DATA + 0x2_2000;
        check(
            ee::s06::f_00231348__generated,
            game_module_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0023_1348, 40);
                c.pc = 0x0023_1348;
                fill(c, &mut r, THIS, 0x90);
                let vt = vtable(c, &mut r, &[0x60]);
                for k in 0..2 {
                    let m = MEMBERS + 0x100 * k;
                    fill(c, &mut r, m, 8);
                    c.bus.write32(m + 0xc, vt);
                    c.bus.write32(THIS + 0x70 + 4 * k, m);
                }
                let svt = vtable(c, &mut r, &[8]);
                c.bus.write32(STREAM, svt);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// SSXAudioSystem_GetRiderListenerIndex: outside / inside the game, the
    /// rider's fixed index on or off (flag, on word), one or two listeners;
    /// with two the rider's position getter is a fake (may unwind) and the
    /// real IsCloserToSecondView compares against two random views.
    #[test]
    fn ssx_audio_system_get_rider_listener_index_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const RIDER: u32 = DATA + 0x2_0000;
        const WORLD: u32 = DATA + 0x3_0000;
        const APP: u32 = DATA + 0x3_1000;
        const VIEWS: u32 = DATA + 0x3_2000;
        const VIEW: u32 = DATA + 0x3_3000;
        let f = |x: f32| x.to_bits();
        check(
            ee::s07::f_00288a20__generated,
            ssx_audio_system_get_rider_listener_index,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_8a20, 1);
                c.pc = 0x0028_8a20;
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(RIDER);
                // In the game (the world's app word) or not.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus
                    .write32(WORLD + 0x84, if seed % 7 == 0 { 0 } else { APP });
                c.bus.write32(APP + 0x84, VIEWS);
                c.bus.write32(VIEWS + 0x10, 1 + (seed / 2) % 2);
                for i in 0..2u32 {
                    let v = VIEW + 0x100 * i;
                    c.bus.write32(VIEWS + 4 + 4 * i, v);
                    for k in 0..0x40u32 {
                        c.bus
                            .write32(v + 4 * k, f(r.next_u32() as f32 / 1e7 - 200.0));
                    }
                }
                // The fixed listener index.
                let flag = [0u32, 1, 0x8000_0000][(seed % 3) as usize];
                let on = [0u32, 2, 0xffff_ffff][((seed / 3) % 3) as usize];
                c.bus.write32(RIDER + RLI_FIXED_FLAG, flag);
                c.bus.write32(RIDER + RLI_FIXED_ON, on);
                c.bus.write32(RIDER + RLI_FIXED_INDEX, r.next_u32() % 2);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(RIDER + RLI_POS_OBJECT, vt);
                fill(c, &mut r, DATA, 0x10);
            },
        );
    }

    /// RiderModel_SaveState: the stream's write is a fake (some unwind);
    /// random bone counts and array pointers.
    #[test]
    fn rider_model_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, FAKE};
        use crate::difftest::DATA;
        covered(
            ee::s10::f_00310ea8__generated,
            rider_model_save_state,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_0ea8, 1);
                c.pc = 0x0031_0ea8;
                let this = DATA;
                let stream = DATA + 0x800;
                let vt = DATA + 0x900;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, vt);
                c.bus.write16(vt + 0x8, (r.next_u32() & 7) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus.write32(this + RM_NUM_BONES, r.next_u32() % 8);
                c.bus.write32(this + RM_BONES, ptr(&mut r));
            },
        );
    }

    /// The stream setup of the rider-state SaveState tests: `this` at DATA,
    /// the stream's write is a fake (some unwind).
    fn rider_state_save_setup(c: &mut Cpu, seed: u32, entry: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::DATA;
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        let vt = DATA + 0x900;
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(stream);
        c.bus.write32(stream, vt);
        c.bus.write16(vt + 0x8, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + 0xc, FAKE);
    }

    /// RiderRailslideState_SaveState: write(self, 0x50) through a fake.
    #[test]
    fn rider_railslide_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_0013c5e0__generated,
            rider_railslide_state_save_state,
            64,
            &|c, seed| rider_state_save_setup(c, seed, 0x0013_c5e0),
        );
    }

    /// RiderBoardPressSub_SaveState: write(self, 0x14) through a fake.
    #[test]
    fn rider_board_press_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_00131598__generated,
            rider_board_press_sub_save_state,
            64,
            &|c, seed| rider_state_save_setup(c, seed, 0x0013_1598),
        );
    }

    /// ModifierBlock_dtor: transform / boost / particle modifiers present
    /// or not (their release methods are fakes; some unwind), no UV-scroll,
    /// texture-flip or rail modifiers, the modifier list linked to a node or
    /// empty; delete flag on (real operator delete over the test heap) or off.
    #[test]
    fn modifier_block_dtor_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::DATA;
        use crate::game::be::belibrary::tests::alloc_ready;
        const THIS: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x800;
        const TRANSFORM: u32 = DATA + 0x900;
        const BOOST: u32 = DATA + 0xa00;
        const NODE: u32 = DATA + 0xb00;
        check(ee::s11::f_00352ae8__generated, modifier_block_dtor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0035_2ae8, 3);
            alloc_ready(c);
            c.pc = 0x0035_2ae8;
            for k in 0..0x40u32 {
                c.bus.write32(THIS - 0x10 + 4 * k, 0);
            }
            c.bus.write16(VT + 0x8, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(VT + 0xc, FAKE);
            c.bus.write32(TRANSFORM, VT);
            c.bus.write32(BOOST + 4, VT);
            c.bus.write32(NODE, 0);
            c.bus.write32(NODE + 4, 0);
            c.bus.write32(NODE + 8, VT);
            let present = |bit: u32, p: u32| if (seed >> bit) & 1 != 0 { p } else { 0 };
            c.bus.write32(THIS, present(1, TRANSFORM));
            c.bus.write32(THIS + 0xc, present(2, BOOST));
            c.bus.write32(THIS + MB_MOD_LIST, present(3, NODE));
            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = ((seed & 1) | (r.next_u32() & 6)) as u64;
        });
    }

    /// cWorldCache_ResolveResource: default table (key low byte 0xff) or an
    /// indexed one, groups 0..3 with small pending counts, the no-handle
    /// sentinel or a handle; the loader's Resolve is a fake (some unwind; its
    /// result is the new handle).
    #[test]
    fn world_cache_resolve_resource_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1000;
        const TABLES: u32 = DATA + 0x2000;
        const TABLE: u32 = DATA + 0x3000;
        const BITS: u32 = DATA + 0x4_0000;
        const LISTS: u32 = DATA + 0x6000;
        const ENTRIES: u32 = DATA + 0x8000;
        const LOADER: u32 = DATA + 0x9000;
        const VT: u32 = DATA + 0x9100;
        check(
            ee::s13::f_003a8cd0__generated,
            world_cache_resolve_resource,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_8cd0, 1);
                c.pc = 0x003a_8cd0;
                c.bus.write32(THIS + WC_DEFAULT_TABLE, TABLE);
                c.bus.write32(THIS + WC_TABLES, TABLES);
                for k in 0..0x100u32 {
                    c.bus.write32(TABLES + 4 * k, TABLE + 0x100 * (k % 4));
                }
                for k in 0..0x80u32 {
                    c.bus.write32(TABLE + 8 * k, r.next_u32());
                    c.bus.write32(TABLE + 8 * k + 4, BITS + 0x200 * (k % 0x20));
                }
                for g in 0..4u32 {
                    let list = LISTS + 0x100 * g;
                    c.bus.write32(THIS + WC_GROUPS + 0x18 * g, list);
                    c.bus.write32(list + 8, r.next_u32() % 16);
                    c.bus.write32(list + 0xc, ENTRIES + 0x100 * g);
                }
                crate::difftest::fill(c, &mut r, BITS, 0x1000);
                c.bus.write32(THIS + WC_LOADER, LOADER);
                c.bus.write32(LOADER + 8, VT);
                c.bus.write16(VT + 0x18, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(VT + 0x1c, FAKE);
                let lo = if seed % 3 == 0 {
                    0xff
                } else {
                    r.next_u32() & 0xff
                };
                let hi = r.next_u32() % 0x40;
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (r.next_u32() % 0x10) as u64;
                c.g[A3][0] = sx(hi << 8 | lo);
                c.g[T0][0] = if seed % 2 == 0 {
                    sx(GAME_GP + WC_NO_HANDLE_GP)
                } else {
                    sx(r.next_u32())
                };
            },
        );
    }

    /// SND_RandomRange: ranges around the clamps (negative, 0, inside,
    /// 0x10000, above, 64-bit values); the real SND_Random over a random
    /// generator state.
    #[test]
    fn snd_random_range_matches() {
        use crate::difftest::{fill, scramble, Rng};
        check(ee::s13::f_003c72a0__generated, snd_random_range, 300, &|c, seed| {
            let mut r = Rng(0x003c_72a0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003c_72a0;
            // The generator state (0x50ac50).
            fill(c, &mut r, 0x0050_ac50, 0x10);
            let range: u64 = match seed % 8 {
                0 => u64::MAX,
                1 => 0,
                2 => 0x10000,
                3 => 0x10001,
                4 => sx(r.next_u32()),
                5 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                _ => (r.next_u32() % 0x10000) as u64,
            };
            c.g[A0][0] = range;
        });
    }

    /// Vu0_CopyOutputMem: random VU0 output memory copied to a random
    /// destination; events off, delivered, or unwind-requesting (ignored at
    /// the back edges, as in the original) at the loop's back edges.
    #[test]
    fn vu0_copy_output_mem_matches() {
        use crate::difftest::events::schedule;
        use crate::difftest::{fill, scramble, Rng, DATA};
        check(ee::s12::f_00391480__generated, vu0_copy_output_mem, 120, &|c, seed| {
            let mut r = Rng(0x0039_1480 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            for k in 0..0x190u32 {
                c.bus.write32(VU0_OUTPUT_MEM + 4 * k, r.next_u32());
            }
            let dst = DATA + 0x1000 + 0x10 * (r.next_u32() % 0x100);
            fill(c, &mut r, dst, 0x190);
            c.g[A1][0] = sx(dst);
            schedule(c, &mut r, seed, 0x0039_1480);
        });
    }
}
