//! Staging wave 206 (oc247): hand-written replacements for the hottest
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
use crate::abi;
use crate::guest::Addr;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_cb68, rider_wipeout_sub_handle_input, "RiderWipeoutSub_HandleInput"),
    (0x0029_47b0, ssx_audio_system_update_airborne_riders, "SSXAudioSystem_UpdateAirborneRiders"),
    (0x003b_a078, snd_is_voice_reserved, "SND_IsVoiceReserved"),
    (0x0022_c790, game_module_render_lens_flares, "GameModule_RenderLensFlares"),
    (0x002b_0290, speech_instance_is_stream_ready, "SpeechInstance_IsStreamReady"),
    (0x003c_6c28, sndstrm_get_free_blocks, "SNDSTRM_GetFreeBlocks"),
    (0x0031_7328, app_invoke_exec_callback, "App_InvokeExecCallback"),
    (0x002b_2160, song_instance_set_volume, "SongInstance_SetVolume"),
    (0x003c_6ab8, sndstrm_push_block, "SNDSTRM_PushBlock"),
];

// ---------------------------------------------------------------------------
// 0x0012cb68 RiderWipeoutSub_HandleInput

/// Callees of `RiderWipeoutSub_HandleInput`.
mod wipeout_input {
    use super::*;
    pub const RIDER_HANDLE_BOOST_INPUT: Callee = Callee {
        addr: 0x0011_4130,
        f: ee::s00::f_00114130,
    };
    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };
    pub const UPDATE_PHASE0: Callee = Callee {
        addr: 0x0012_cd20,
        f: ee::s00::f_0012cd20,
    };
    pub const UPDATE_PHASE1: Callee = Callee {
        addr: 0x0012_d4e8,
        f: ee::s00::f_0012d4e8,
    };
    pub const UPDATE_PHASE2: Callee = Callee {
        addr: 0x0012_d160,
        f: ee::s00::f_0012d160,
    };
    pub const UPDATE_RECOVER: Callee = Callee {
        addr: 0x0012_d848,
        f: ee::s00::f_0012d848,
    };
    pub const UPDATE_PHASE4: Callee = Callee {
        addr: 0x0012_d9d8,
        f: ee::s00::f_0012d9d8,
    };
    /// `SSXAudioSystem_GetInstance` (table call).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    /// `SSXAudioSystem_PlayWipeoutSlideSfx(audio, rider)` (table call).
    pub const AUDIO_SLIDE_SFX: u32 = 0x0029_e970;
    /// `SSXAudioSystem_PlayWipeoutImpactSfx(audio, rider, amount)` (table call).
    pub const AUDIO_IMPACT_SFX: u32 = 0x0029_e590;

    /// Wipeout sub-state: the phase (0..4; 3 = recovering).
    pub const PHASE: u32 = 0x0;
    /// Wipeout sub-state: the slide amount (decays toward 0, the slide
    /// button pushes it up).
    pub const SLIDE: u32 = 0x70;
    /// Wipeout sub-state: the rider.
    pub const RIDER: u32 = 0x80;
    /// Rider: the wipeout kind (0, 2 or other) that picks the slide rate.
    pub const RIDER_WIPEOUT_KIND: u32 = 0xb20;
    /// The pad's slide button.
    pub const SLIDE_BUTTON: u64 = 0x2000;
    /// The phase switch's jump table (5 entries).
    pub const PHASE_TABLE: u32 = 0x0045_7f80;
    /// `.lit4` literals (gp-relative): slide decay per tick, 0, the slide
    /// rates (default, kind 0, kind 2) and the tick length.
    pub const GP_DECAY: u32 = 0x77dc;
    pub const GP_ZERO: u32 = 0x77d8;
    pub const GP_RATE: u32 = 0x77d4;
    pub const GP_RATE_KIND0: u32 = 0x77d0;
    pub const GP_RATE_KIND2: u32 = 0x77cc;
    pub const GP_TICK: u32 = 0x77c8;
}

/// `void RiderWipeoutSub_HandleInput(RiderWipeoutSub* this, Pad* pad)`
/// (0x12cb68): per-tick input of a wiped-out rider. Handles the boost input
/// (no boost), lets the reset button reset the rider onto the path (then
/// returns), otherwise (unless recovering, phase 3) decays the slide amount
/// toward 0 and, while the slide button is held, pushes it up at the wipeout
/// kind's rate and plays the slide sound (once it reaches 1) or the impact
/// sound; finally runs the current phase's update.
pub fn rider_wipeout_sub_handle_input(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use wipeout_input::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        // Rider_HandleBoostInput(rider, 0, 0).
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
        RIDER_HANDLE_BOOST_INPUT.call(c, 0x0012_cb90)?;
        // Rider_CheckResetPath(rider, pad bit 12, 0).
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32) & 0x1;
        RIDER_CHECK_RESET_PATH.call(c, 0x0012_cba8)?;
        'epilogue: {
            if c.g[V0][0] != 0 {
                // Reset onto the path: nothing else this tick.
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 4;
            let gp = r32(c, GP);
            let lit = |off: u32| Addr(gp.wrapping_sub(off));
            let this = r32(c, S0);
            c.g[V0][0] = Addr(this.wrapping_add(PHASE)).lw(c) ^ 0x3;
            if c.g[V0][0] != 0 {
                // Decay the slide amount toward 0 by one step.
                c.cycles += 5;
                let slide = Addr(this.wrapping_add(SLIDE)).read_fpu(c);
                let step = lit(GP_DECAY).read_fpu(c);
                abi::set_fpr(c, 2, slide);
                abi::set_fpr(c, 3, step);
                if abi::c_lt_s(c, step, slide) {
                    c.cycles += 2;
                    abi::set_fpr(c, 1, slide - step);
                } else {
                    c.cycles += 6;
                    let zero = lit(GP_ZERO).read_fpu(c);
                    abi::set_fpr(c, 0, zero);
                    abi::set_fpr(c, 1, FpuF32::ZERO);
                    if abi::c_lt_s(c, slide, zero) {
                        abi::set_fpr(c, 1, slide + step);
                    }
                }
                c.cycles += 5;
                let decayed = FpuF32(c.f[1]);
                Addr(this.wrapping_add(SLIDE)).write_fpu(c, decayed);
                c.g[V0][0] = Addr(r32(c, S1)).lw(c) & SLIDE_BUTTON;
                if c.g[V0][0] != 0 {
                    // The slide rate by wipeout kind.
                    c.cycles += 5;
                    c.g[V0][0] = Addr(this.wrapping_add(RIDER)).lw(c);
                    let mut rate = lit(GP_RATE).read_fpu(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RIDER_WIPEOUT_KIND)).lw(c);
                    c.g[V0][0] = 0x2;
                    if c.g[V1][0] == 0 {
                        c.cycles += 3;
                        rate = lit(GP_RATE_KIND0).read_fpu(c);
                    } else if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        rate = lit(GP_RATE_KIND2).read_fpu(c);
                    } else {
                        c.cycles += 4;
                    }
                    abi::set_fpr(c, 2, rate);
                    // slide += rate * tick.
                    c.cycles += 10;
                    let tick = lit(GP_TICK).read_fpu(c);
                    let slide = Addr(this.wrapping_add(SLIDE)).read_fpu(c);
                    let push = rate * tick;
                    abi::set_fpr(c, 0, push);
                    c.g[AT][0] = 0x3f80_0000;
                    abi::set_fpr(c, 2, FpuF32::ONE);
                    let pushed = slide + push;
                    abi::set_fpr(c, 1, pushed);
                    let full = FpuF32::ONE.le(pushed);
                    abi::set_fcc(c, full);
                    Addr(this.wrapping_add(SLIDE)).write_fpu(c, pushed);
                    c.cycles += 7;
                    if full {
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0012_cc6c)?;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, AUDIO_SLIDE_SFX, 0x0012_cc78)?;
                    } else {
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0012_cc88)?;
                        let this = r32(c, S0);
                        c.g[A1][0] = Addr(this.wrapping_add(RIDER)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        c.lwc1(12, this.wrapping_add(SLIDE));
                        call_ptr(c, AUDIO_IMPACT_SFX, 0x0012_cc98)?;
                    }
                }
            }
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(PHASE)).lw(c);
            // switch (phase).
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0x5) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if !in_range {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(PHASE_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(PHASE_TABLE));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (update, ret) = match target {
                0x0012_ccc0 => (UPDATE_PHASE0, 0x0012_ccc8),
                0x0012_ccd0 => (UPDATE_PHASE1, 0x0012_ccd8),
                0x0012_cce0 => (UPDATE_PHASE2, 0x0012_cce8),
                0x0012_ccf0 => (UPDATE_RECOVER, 0x0012_ccf8),
                0x0012_cd00 => (UPDATE_PHASE4, 0x0012_cd08),
                _ => {
                    c.jump_out(target, 0x0012_cb68, 0x0012_cd1c);
                    return Ok(());
                }
            };
            c.cycles += if target == 0x0012_cd00 { 2 } else { 4 };
            c.g[A0][0] = c.g[S0][0];
            update.call(c, ret)?;
            if target == 0x0012_cd00 {
                c.cycles += 1;
            }
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
// 0x002947b0 SSXAudioSystem_UpdateAirborneRiders

/// Callees of `SSXAudioSystem_UpdateAirborneRiders`.
mod airborne_riders {
    use super::*;
    pub const GET_RACE_MAN: Callee = Callee {
        addr: 0x0028_b1d8,
        f: ee::s07::f_0028b1d8,
    };
    pub const UPDATE_RIDER_AIR_STATE: Callee = Callee {
        addr: 0x0029_4678,
        f: ee::s07::f_00294678,
    };
    /// Audio system: one "airborne" flag word per rider slot (6).
    pub const AIR_FLAGS: u32 = 0x59e8;
    /// Race manager: the rider count and the rider pointers.
    pub const RACE_RIDER_COUNT: u32 = 0x78;
    pub const RACE_RIDERS: u32 = 0x28;
    /// The loop head (event checks).
    pub const LOOP_HEAD: u32 = 0x0029_47d8;
}

/// `void SSXAudioSystem_UpdateAirborneRiders(SSXAudioSystem* this)`
/// (0x2947b0): for each of the 6 rider slots whose airborne flag is set,
/// update that rider's air sound state if the race has such a rider;
/// otherwise clear the flag.
pub fn ssx_audio_system_update_airborne_riders(c: &mut Cpu) {
    use airborne_riders::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        // s0: the slot's flag, s1: the slot, s3: the slot * 4.
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(AIR_FLAGS));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        loop {
            'next: {
                'clear: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[V0][0] == 0 {
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    c.cycles += 6;
                    GET_RACE_MAN.call(c, 0x0029_47ec)?;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDER_COUNT)).lw(c);
                    c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    c.cycles += 7;
                    GET_RACE_MAN.call(c, 0x0029_4804)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S3)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDERS)).lw(c);
                    c.g[A1][0] = c.g[V1][0];
                    if c.g[V1][0] == 0 {
                        c.cycles += 1;
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    // UpdateRiderAirState(this, rider).
                    c.cycles += 4;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    UPDATE_RIDER_AIR_STATE.call(c, 0x0029_4820)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    break 'next;
                }
                c.cycles += 2;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LOOP_HEAD);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003ba078 SND_IsVoiceReserved

/// The reserved-voice table (`short[]`) `SND_IsVoiceReserved` scans.
const SND_RESERVED_VOICES: u32 = 0x0050_ac20;

/// `int SND_IsVoiceReserved(int voice, int count)` (0x3ba078): 1 if `voice`
/// is among the first `count` entries of the reserved-voice table, else 0.
pub fn snd_is_voice_reserved(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    if (c.g[A1][0] as i64) > 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x0051_0000;
        c.g[A2][0] = sx(SND_RESERVED_VOICES);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A2)).lh(c);
            let differs = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !differs {
                c.cycles += 2;
                c.g[V0][0] = 1;
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a088);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x0022c790 GameModule_RenderLensFlares

/// Callees and layout of `GameModule_RenderLensFlares`.
mod render_lens_flares {
    /// `LensFX_ClearSources(fx, view)` (table call).
    pub const CLEAR_SOURCES: u32 = 0x002e_2f98;
    /// `LensFX_AddSources(fx, sources, count, kind, view)` (table call).
    pub const ADD_SOURCES: u32 = 0x002e_2ff8;
    /// `LensFX_Render(fx, view)` (table call).
    pub const RENDER: u32 = 0x002e_30d0;
    /// Game module: its lens FX.
    pub const LENS_FX: u32 = 0x1c;
    /// Lens FX: "rendering" flag (set around the render).
    pub const RENDERING: u32 = 0x6494;
    /// World: two flare source lists `{count, sources[]}` (kinds 0 and 2).
    pub const SOURCES0_COUNT: u32 = 0x7bc0;
    pub const SOURCES0: u32 = 0x7bc4;
    pub const SOURCES2_COUNT: u32 = 0x7fc4;
    pub const SOURCES2: u32 = 0x7fc8;
}

/// `void GameModule_RenderLensFlares(GameModule* this, World* world, int
/// view)` (0x22c790): rebuild the lens FX's flare sources for `view` from
/// the world's two source lists (kinds 0 and 2) and render the flares, with
/// the lens FX's "rendering" flag set meanwhile.
pub fn game_module_render_lens_flares(c: &mut Cpu) {
    use render_lens_flares::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // world
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0]; // view
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        Addr(r32(c, V0).wrapping_add(RENDERING)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        call_ptr(c, CLEAR_SOURCES, 0x0022_c7c8)?;
        // The kind-0 sources.
        let world = r32(c, S0);
        c.g[A2][0] = Addr(world.wrapping_add(SOURCES0_COUNT)).lw(c);
        c.g[A1][0] = sx(world.wrapping_add(SOURCES0));
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
            c.g[A3][0] = 0;
            c.g[T0][0] = c.g[S2][0];
            call_ptr(c, ADD_SOURCES, 0x0022_c7e4)?;
        }
        // The kind-2 sources.
        c.cycles += 3;
        let world = r32(c, S0);
        c.g[A2][0] = Addr(world.wrapping_add(SOURCES2_COUNT)).lw(c);
        c.g[A1][0] = sx(world.wrapping_add(SOURCES2));
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
            c.g[A3][0] = 0x2;
            c.g[T0][0] = c.g[S2][0];
            call_ptr(c, ADD_SOURCES, 0x0022_c800)?;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, RENDER, 0x0022_c80c)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LENS_FX)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(RENDERING)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002b0290 SpeechInstance_IsStreamReady

/// Callees and layout of `SpeechInstance_IsStreamReady`.
mod speech_stream_ready {
    use super::*;
    pub const STREAM_ARRAY_GET_STATE: Callee = Callee {
        addr: 0x002a_ae08,
        f: ee::s08::f_002aae08,
    };
    pub const STREAM_ARRAY_IS_PLAYING: Callee = Callee {
        addr: 0x002a_b028,
        f: ee::s08::f_002ab028,
    };
    pub const STREAM_ARRAY_GET_TIME_REMAINING: Callee = Callee {
        addr: 0x002a_ae78,
        f: ee::s08::f_002aae78,
    };
    /// `gp+0x6c4`: the speech system (its first word is the stream array).
    pub const SPEECH_GP: u32 = 0x6c4;
    /// Speech instance: its stream slot.
    pub const STREAM_SLOT: u32 = 0x4;
    /// A playing stream counts as ready when less than this much is left.
    pub const READY_REMAINING: i64 = 500;
}

/// `int SpeechInstance_IsStreamReady(SpeechInstance* this)` (0x2b0290): 1
/// when the instance's stream slot is idle (state 0); for a slot in state 1,
/// 1 when it is playing with less than 500 left; 0 otherwise.
pub fn speech_instance_is_stream_ready(c: &mut Cpu) {
    use speech_stream_ready::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let speech = Addr(r32(c, GP).wrapping_add(SPEECH_GP));
        c.g[V0][0] = speech.lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        STREAM_ARRAY_GET_STATE.call(c, 0x002b_02b4)?;
        c.g[S1][0] = c.g[V0][0]; // the state
        'done: {
            c.g[V0][0] = 1;
            if c.g[S1][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_one = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if not_one {
                break 'done;
            }
            c.cycles += 6;
            c.g[V0][0] = speech.lw(c);
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            STREAM_ARRAY_IS_PLAYING.call(c, 0x002b_02dc)?;
            let playing = c.g[V0][0] != 0;
            c.g[V0][0] = speech.lw(c);
            if !playing {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 9;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            STREAM_ARRAY_GET_TIME_REMAINING.call(c, 0x002b_02f0)?;
            // (int)remaining < 500 ? state : 0.
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1);
            c.g[V0][0] = 0;
            c.g[V1][0] = ((c.g[V1][0] as i64) < READY_REMAINING) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003c6c28 SNDSTRM_GetFreeBlocks

/// SNDSTRM's per-slot state pointers (`+0x304 + 4 * slot`).
const SNDSTRM_SLOT_BASE: u32 = 0x0051_57b8;
const SNDSTRM_SLOT_STATES: u32 = 0x304;

/// `int SNDSTRM_GetFreeBlocks(int slot)` (0x3c6c28): the slot's block count
/// (`+0x40`) minus one minus the highest block index in use by its channels
/// (`+0x62` channels, a signed halfword each at `+0x34`, at least 0).
pub fn sndstrm_get_free_blocks(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(SNDSTRM_SLOT_BASE);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    let state = Addr(r32(c, V0).wrapping_add(SNDSTRM_SLOT_STATES)).lw(c);
    c.g[A2][0] = state;
    let state = state as u32;
    c.g[V0][0] = Addr(state.wrapping_add(0x62)).lbu(c);
    c.g[A1][0] = 0; // the highest index so far
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = c.g[V0][0]; // channels left
        c.g[V1][0] = sx(state.wrapping_add(0x34));
        loop {
            c.cycles += 6;
            let index = Addr(r32(c, V1)).lhu(c) as u32 as i16 as i64;
            c.g[V0][0] = (index < c.g[A1][0] as i64) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V0][0] = sx((index as u32) << 16);
                c.g[A1][0] = index as u64;
            }
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            c.cycles += 2;
            let more = c.g[A0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_6c50);
            }
        }
    }
    c.cycles += 6;
    let blocks = Addr(state.wrapping_add(0x40)).lhu(c) as u32 as i16 as i32 as u32;
    c.g[V0][0] = sx(blocks.wrapping_sub(r32(c, A1)).wrapping_sub(1));
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x00317328 App_InvokeExecCallback

/// `PS2ExecMan_InvokeCallback(man, notify)`.
const PS2_EXEC_MAN_INVOKE_CALLBACK: Callee = Callee {
    addr: 0x0031_74a8,
    f: ee::s10::f_003174a8,
};

/// `void App_InvokeExecCallback(App* this)` (0x317328): run the exec
/// manager's (`this+8`) pending callback, with notification
/// (`PS2ExecMan_InvokeCallback(man, 1)`).
pub fn app_invoke_exec_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
        PS2_EXEC_MAN_INVOKE_CALLBACK.call(c, 0x0031_733c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002b2160 SongInstance_SetVolume

/// Callees and layout of `SongInstance_SetVolume`.
mod song_set_volume {
    use super::*;
    /// `SND_Lock()` / `SND_Unlock()` (table calls).
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const GET_STREAM_HANDLE: Callee = Callee {
        addr: 0x002b_2488,
        f: ee::s08::f_002b2488,
    };
    /// `SNDSTRM_SetLowpass(handle, value)` (table call): the song's volume
    /// goes through the stream's low-pass parameter.
    pub const SNDSTRM_SET_LOWPASS: u32 = 0x003b_80a0;
    /// Song instance: the current volume.
    pub const VOLUME: u32 = 0x44;
    pub const MAX_VOLUME: u64 = 0xffff;
}

/// `void SongInstance_SetVolume(SongInstance* this, int volume)` (0x2b2160):
/// clamp the volume to 0..0xffff; if it changed, under the SND lock, store
/// it and pass it to the song's stream (when the song has one).
pub fn song_instance_set_volume(c: &mut Cpu) {
    use song_set_volume::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = MAX_VOLUME;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S1, 0x10);
        c.g[V0][0] = ((MAX_VOLUME as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.g[S0][0] = c.g[A1][0];
        }
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = (-1 < (c.g[S0][0] as i64)) as u64;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(VOLUME)).lw(c);
        if c.g[V1][0] == 0 {
            c.g[S0][0] = 0;
        }
        // s0: the clamped volume.
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 6;
            call_ptr(c, SND_LOCK, 0x002b_21a4)?;
            c.g[A0][0] = c.g[S1][0];
            GET_STREAM_HANDLE.call(c, 0x002b_21ac)?;
            let no_stream = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_stream {
                c.cycles += 3;
                Addr(r32(c, S1).wrapping_add(VOLUME)).sw(c, c.g[S0][0]);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, SNDSTRM_SET_LOWPASS, 0x002b_21c0)?;
            }
            c.cycles += 3;
            call_ptr(c, SND_UNLOCK, 0x002b_21c8)?;
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
// 0x003c6ab8 SNDSTRM_PushBlock

/// `int SNDSTRM_PushBlock(int slot, Block* block)` (0x3c6ab8): queue a
/// stream block on the slot's ring. Fails with -13 when some channel's
/// position (`+0x34`, one signed halfword per channel) has reached the last
/// ring entry (`+0x40` entries). Otherwise writes the ring entry at the
/// write index (`+0x46`, 0x20 bytes each from `+0x7c`): the sequence number
/// (`+0x4`), the block's size word (low 31 bits) with its top bit kept as
/// the block's flag, and one word per channel from the block (`+0xc`),
/// advancing every channel's position; adds the size to the queued bytes
/// (`+0x48`), bumps the sequence number and the write index (wrapping at the
/// ring size). Returns the sequence number the block got.
pub fn sndstrm_push_block(c: &mut Cpu) {
    const ERR_FULL: u64 = 0xffff_ffff_ffff_fff3;
    const SIZE_MASK: u64 = 0x7fff_ffff;
    let lh16 = |c: &mut Cpu, a: u32| Addr(a).lhu(c) as u32 as i16 as i64 as u64;
    c.cycles += 9;
    c.g[V0][0] = 0x0051_0000;
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(SNDSTRM_SLOT_BASE);
    c.g[T2][0] = c.g[A1][0]; // the block
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[A3][0] = Addr(r32(c, V0).wrapping_add(SNDSTRM_SLOT_STATES)).lw(c);
    let st = r32(c, A3);
    c.g[V0][0] = Addr(st.wrapping_add(0x62)).lbu(c);
    c.g[T0][0] = 0;
    'done: {
        // Room check: every channel's position below the last entry.
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[V0][0]; // channels
            c.g[A0][0] = sx(st.wrapping_add(0x34));
            loop {
                c.cycles += 10;
                let pos_at = r32(c, A0);
                c.g[V1][0] = lh16(c, pos_at);
                c.g[V0][0] = lh16(c, st.wrapping_add(0x40));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                if c.g[V1][0] == 0 {
                    c.cycles += 2;
                    c.g[A1][0] = ERR_FULL;
                    break 'done;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6ae8);
                }
            }
        }
        // The ring entry at the write index.
        c.cycles += 26;
        c.g[V0][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[A1][0] = SIZE_MASK;
        let block = r32(c, T2);
        c.g[A0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A2][0] = 0xffff_ffff_8000_0000;
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 11) as u32);
        c.g[A0][0] &= c.g[A1][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7c));
        c.g[T0][0] = 0;
        let entry = st.wrapping_add(r32(c, V0));
        c.g[T1][0] = sx(entry);
        c.g[V1][0] = Addr(entry.wrapping_add(4)).lw(c);
        c.g[V1][0] &= c.g[A2][0];
        c.g[V1][0] |= c.g[A0][0];
        Addr(entry.wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V1][0] &= c.g[A1][0];
        c.g[V0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) >> 31);
        c.g[V0][0] = sx(r32(c, V0) << 31);
        c.g[V1][0] |= c.g[V0][0];
        Addr(entry.wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c);
        Addr(entry).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(st.wrapping_add(0x62)).lbu(c);
        c.g[A1][0] = sx(entry.wrapping_add(8));
        // One word per channel; each channel's position advances.
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = sx(st.wrapping_add(0x34));
            c.g[A0][0] = sx(block.wrapping_add(0xc));
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(r32(c, V1)).sh(c, c.g[V0][0]);
                c.g[V0][0] = Addr(st.wrapping_add(0x62)).lbu(c);
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_6b98);
                }
            }
        }
        // Queued bytes, sequence number, write index.
        c.cycles += 21;
        c.g[A0][0] = Addr(block.wrapping_add(4)).lw(c);
        c.g[V0][0] = SIZE_MASK;
        c.g[V1][0] = Addr(st.wrapping_add(0x48)).lw(c);
        c.g[A0][0] &= c.g[V0][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        Addr(st.wrapping_add(0x48)).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(st.wrapping_add(4)).lw(c);
        c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        Addr(st.wrapping_add(0x46)).sh(c, c.g[V1][0]);
        c.g[A0][0] = Addr(st.wrapping_add(0x46)).lhu(c);
        c.g[V0][0] = Addr(st.wrapping_add(0x40)).lhu(c);
        c.g[A0][0] = sx(r32(c, A0) << 16);
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[A0][0] == 0 {
            Addr(st.wrapping_add(0x46)).sh(c, 0);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    // -- 0x0012cb68 RiderWipeoutSub_HandleInput --------------------------------

    /// `RiderWipeoutSub_HandleInput` against the generated body: every phase
    /// (recovering phase 3 skips the slide update; 5+ and negative ones fall
    /// out of the switch), slide amounts around the decay step and around 1,
    /// the slide button on and off, the three wipeout kinds. The real boost
    /// input, reset-path check (reset button never pressed) and audio callees
    /// run on their short paths (no boost, not in game, rider without a sound
    /// slot). The phase jump table is empty RAM (each phase update is a fake
    /// through `jump_out`, some unwinding), except that some phase-4 seeds put
    /// the real entry there and run the real `UpdatePhase4` (timer not
    /// expired, animation not finished).
    #[test]
    fn rider_wipeout_sub_handle_input_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        use super::wipeout_input::*;
        const THIS: u32 = DATA + 0x6_0000;
        const PAD: u32 = DATA + 0x6_0100;
        const RIDER_OBJ: u32 = DATA + 0x6_1000;
        const CTRL: u32 = DATA + 0x6_3000;
        const SEQS: u32 = DATA + 0x6_3100;
        const SEQ_NODE: u32 = DATA + 0x6_3200;
        const WORLD: u32 = DATA + 0x6_4000;
        const AUDIO: u32 = DATA + 0x6_5000;
        const BE_LIBRARY: u32 = DATA + 0x6_6000;
        covered(
            ee::s00::f_0012cb68__generated,
            rider_wipeout_sub_handle_input,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_cb68, 1);
                c.pc = 0x0012_cb68;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let f = |x: f32| x.to_bits();
                // Globals: the behaviour library, the world (not in game:
                // the audio calls stop early), the audio system.
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, 0);
                w(c, GAME_GP + 0x410, AUDIO);
                // The literals.
                w(c, GAME_GP - GP_DECAY, f(0.004_169_862_7));
                w(c, GAME_GP - GP_ZERO, 0);
                w(c, GAME_GP - GP_RATE, f(17.002_617));
                w(c, GAME_GP - GP_RATE_KIND0, f(8.001_495));
                w(c, GAME_GP - GP_RATE_KIND2, f(34.002_766));
                w(c, GAME_GP - GP_TICK, f(1.0 / 60.0));
                // The phase.
                let phase = match seed % 8 {
                    7 => 0xffff_fff0 | r.next_u32() % 16,
                    p => p,
                };
                let real_phase4 = phase == 4 && r.next_u32() % 2 == 0;
                if real_phase4 {
                    w(c, PHASE_TABLE + 4 * 4, 0x0012_cd00);
                }
                // The slide amount: around the decay step, around 1, or
                // small enough to stay below 1 (the real phase 4).
                let slide = if real_phase4 {
                    r.f32(0.4)
                } else {
                    match r.next_u32() % 4 {
                        0 => r.f32(0.01),
                        1 => f(1.0 - (r.next_u32() % 64) as f32 * 0.01),
                        2 => 0,
                        _ => r.f32(1.5),
                    }
                };
                w(c, THIS + PHASE, phase);
                w(c, THIS + SLIDE, slide);
                w(c, THIS + RIDER, RIDER_OBJ);
                // The pad: the reset button (bit 12) never pressed, the
                // slide button at random.
                w(c, PAD, r.next_u32() & !0x1000);
                // The rider: no boost (+0x2fc), a wipeout kind, no sound slot
                // (+0x874); for phase 4 the timer (+0x470) not expired and an
                // animation controller whose layer 2 is not finished.
                w(c, RIDER_OBJ + 0x2fc, 0);
                w(c, RIDER_OBJ + RIDER_WIPEOUT_KIND, [0, 2, 1, 5][(r.next_u32() % 4) as usize]);
                w(c, RIDER_OBJ + 0x874, 0);
                w(c, RIDER_OBJ + 0x470, f(-1.0));
                w(c, RIDER_OBJ + 0x784, CTRL);
                w(c, CTRL + 0x50, SEQS);
                w(c, SEQS + 0x14, SEQ_NODE);
                w(c, SEQ_NODE + 0xc0, 0);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(PAD);
            },
        );
    }
    // -- 0x002947b0 SSXAudioSystem_UpdateAirborneRiders ------------------------

    /// `SSXAudioSystem_UpdateAirborneRiders` against the generated body: the
    /// six airborne flags set or clear, rider counts from none to more than
    /// six (and negative), empty rider slots. The real `UpdateRiderAirState`
    /// runs its "landed" path (animation state neither 1 nor 3): its rider
    /// virtual is a fake (some unwind, some forced to a slot index so
    /// `SetRiderAirFlag` sets a later flag).
    #[test]
    fn ssx_audio_system_update_airborne_riders_matches() {
        use super::airborne_riders::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const GAME: u32 = DATA + 0x2_0100;
        const RACE: u32 = DATA + 0x2_0200;
        const RIDERS: u32 = DATA + 0x3_0000;
        covered(
            ee::s07::f_002947b0__generated,
            ssx_audio_system_update_airborne_riders,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_47b0, 6);
                c.pc = 0x0029_47b0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, GAME);
                w(c, GAME + 0xc, RACE);
                let count = match r.next_u32() % 9 {
                    8 => 0xffff_ffff,
                    n => n,
                };
                w(c, RACE + RACE_RIDER_COUNT, count);
                for i in 0..6u32 {
                    w(c, THIS + AIR_FLAGS + 4 * i, if r.next_u32() % 3 == 0 { 0 } else { r.next_u32() | 1 });
                    let rider = RIDERS + 0x1000 * i;
                    let anim = rider + 0x800;
                    w(c, RACE + RACE_RIDERS + 4 * i, if r.next_u32() % 5 == 0 { 0 } else { rider });
                    w(c, rider + 0x788, anim);
                    w(c, anim + 0xac, [0, 2, 4][(r.next_u32() % 3) as usize]);
                    // The rider's air-state object's vtable (slot 0x38).
                    let vt = rider + 0xc00;
                    c.bus.write16(vt + 0x38, (r.next_u32() as i16 & !3) as u16);
                    w(c, vt + 0x3c, FAKE + 0x10 * i);
                    w(c, rider + 0x6c0, vt);
                }
                // Fake results: a pointer (no flag) or a slot index 1..5.
                for n in 0..8u32 {
                    let v = if r.next_u32() % 2 == 0 { 0 } else { 1 + r.next_u32() % 5 };
                    w(c, LOG_RESULTS + 4 * n, v);
                }
                c.g[A0][0] = sx(THIS);
            },
        );
    }
    // -- 0x003ba078 SND_IsVoiceReserved ----------------------------------------

    /// `SND_IsVoiceReserved` against the generated body: counts from
    /// negative to 24, voices that are in the scanned part of the table, past
    /// it, or nowhere (and a voice with junk upper bits that never matches).
    #[test]
    fn snd_is_voice_reserved_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s13::f_003ba078__generated, snd_is_voice_reserved, 300, &|c, seed| {
            let mut r = Rng(0x003b_a078 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x003b_a078;
            // A table of distinct signed halfwords.
            for i in 0..32u32 {
                let v = (i as i16 * 37 - 300) as u16;
                c.bus.write16(SND_RESERVED_VOICES + 2 * i, v);
            }
            let count: i64 = match seed % 6 {
                0 => -((r.next_u32() % 4) as i64),
                _ => (r.next_u32() % 25) as i64,
            };
            c.g[A1][0] = count as u64;
            let k = r.next_u32() % 32;
            c.g[A0][0] = match seed % 4 {
                0 => r.next_u32() as u64 | 1 << 40,
                _ => (k as i16 * 37 - 300) as i64 as u64,
            };
        });
    }
    // -- 0x0022c790 GameModule_RenderLensFlares --------------------------------

    /// `GameModule_RenderLensFlares` against the generated body, views 0 and
    /// 1: each world source list empty, negative, or with 1..3 sources (the
    /// real `LensFX_ClearSources` / `AddSources` rebuild the flares), then
    /// the real `LensFX_Render` with the `LensFX_PrepareView` fixture (camera
    /// interface slots and the projector's occlusion slot are fakes, the sun
    /// texture is -1 so the sun is hidden) and the `LensFX_DrawFlares` render
    /// state push/pop fakes (the added flares are off, nothing is drawn);
    /// unwinds in the first six calls.
    #[test]
    fn game_module_render_lens_flares_matches() {
        use super::render_lens_flares::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{fill, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::lens_prep::{
            CAMERA_GP, CAM_VT, VT_FOV, VT_VIEW_MATRIX, VT_VIEW_RECT,
        };
        use crate::game::visualfx::lensfx::tests::painter_sun;
        const THIS: u32 = DATA + 0x1000;
        const WORLD: u32 = DATA + 0x1_0000;
        const FX: u32 = DATA + 0x4_0000;
        const CAM: u32 = DATA + 0x5_0000;
        const CAM_VTABLE: u32 = DATA + 0x5_1000;
        const NO_SUN: u32 = DATA + 0x5_4000;
        const SRC: u32 = DATA + 0x5_5000;
        const MATRIX: u32 = DATA + 0x5_6000;
        covered(
            ee::s05::f_0022c790__generated,
            game_module_render_lens_flares,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_c790, 6);
                c.pc = 0x0022_c790;
                let view = seed % 2;
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(WORLD);
                c.g[A2][0] = view as u64;
                c.bus.write32(THIS + LENS_FX, FX);
                // The world's source lists.
                for (count_at, list) in [(SOURCES0_COUNT, SOURCES0), (SOURCES2_COUNT, SOURCES2)] {
                    let n = match r.next_u32() % 5 {
                        0 => 0,
                        1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                        _ => 1 + r.next_u32() % 3,
                    };
                    c.bus.write32(WORLD + count_at, n);
                    for k in 0..3 {
                        c.bus.write32(WORLD + list + 4 * k, SRC + 0x40 * k);
                    }
                }
                fill(c, &mut r, SRC, 0x30);
                // The camera and its interface (as in the PrepareView test).
                c.bus.write32(GAME_GP + CAMERA_GP, CAM);
                c.bus.write32(CAM + CAM_VT, CAM_VTABLE);
                for (i, off) in [VT_VIEW_RECT, VT_VIEW_MATRIX, VT_FOV, 0x2d8]
                    .into_iter()
                    .enumerate()
                {
                    let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                    c.bus.write16(CAM_VTABLE + off, delta as u16);
                    c.bus.write32(CAM_VTABLE + off + 4, FAKE + 0x10 * i as u32);
                }
                fill(c, &mut r, MATRIX, 16);
                c.bus.write32(LOG_RESULTS + 4, MATRIX);
                for v in [6, 7] {
                    painter_sun(c, &mut r, v, DATA + 0x5_2000, DATA + 0x5_3000, &[0x108]);
                }
                c.bus.write32(NO_SUN, u32::MAX);
                c.bus.write32(LOG_RESULTS + 12, NO_SUN);
                fill(c, &mut r, FX, 0x18);
                // DrawFlares: the sun flag and the render-state methods.
                c.bus.write32(GAME_GP + 0xa28, if seed % 3 == 0 { 0 } else { 1 });
                // (At a fixed address: `vtable` picks a random one that could
                // land on the lens FX.)
                const FX_VT: u32 = DATA + 0x5_7000;
                for (i, off) in [0x10u32, 0x18].into_iter().enumerate() {
                    c.bus.write16(FX_VT + off, (r.next_u32() as i16 & !3) as u16);
                    c.bus.write32(FX_VT + off + 4, FAKE + 0x40 + 0x10 * i as u32);
                }
                c.bus.write32(FX + 0x649c, FX_VT);
            },
        );
    }
    // -- 0x002b0290 SpeechInstance_IsStreamReady -------------------------------

    /// `SpeechInstance_IsStreamReady` against the generated body, through the
    /// real stream array / stream buffer / SNDSTRM status calls (the SND lock
    /// hooks are fakes, some unwinding): an empty array slot (state 0), a
    /// stream in state 0, 1 or 2; in state 1 its request playing (status 2)
    /// or not, with 0..1000 ms left (around the 500 threshold).
    #[test]
    fn speech_instance_is_stream_ready_matches() {
        use super::speech_stream_ready::*;
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x100;
        const SPEECH: u32 = DATA + 0x200;
        const ARRAY: u32 = DATA + 0x300;
        const SLOTS: u32 = DATA + 0x400;
        const BUFFER: u32 = DATA + 0x500;
        const STREAM: u32 = DATA + 0x1000;
        const REQUEST: u32 = DATA + 0x1400;
        const SND_SLOT: u32 = DATA + 0x1800;
        /// SNDSTRM: the stream count (byte) and the stream table.
        const SNDSTRM_COUNT: u32 = 0x0051_0000 - 0x56d2;
        const SNDSTRM_STREAMS: u32 = 0x0051_0000 - 0x5f60;
        /// SNDSTRM: the per-slot table the buffered-bytes query reads.
        const SNDSTRM_SLOTS: u32 = 0x0051_57b8 + 0x304;
        covered(
            ee::s08::f_002b0290__generated,
            speech_instance_is_stream_ready,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_0290, 4);
                c.pc = 0x002b_0290;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let slot = r.next_u32() % 4;
                c.g[A0][0] = sx(THIS);
                w(c, THIS + STREAM_SLOT, slot);
                w(c, GAME_GP + SPEECH_GP, SPEECH);
                w(c, SPEECH, ARRAY);
                w(c, ARRAY + 8, SLOTS);
                w(c, SLOTS + 4 * slot, if seed % 7 == 0 { 0 } else { BUFFER });
                // The buffer's SND stream handle 0.
                w(c, BUFFER + 8, 0);
                c.bus.write8(SNDSTRM_COUNT, 4);
                w(c, SNDSTRM_STREAMS, STREAM);
                // The stream: state, current request, rate (1000: the
                // remaining time is the request's length in ms), SND slot 0
                // with some bytes buffered.
                let state = [0, 1, 1, 1, 2][(r.next_u32() % 5) as usize];
                w(c, STREAM + 0x124, state);
                w(c, STREAM + 0x11c, REQUEST);
                c.bus.write16(STREAM + 0x14, 1000);
                w(c, STREAM + 8, 0);
                w(c, SNDSTRM_SLOTS, SND_SLOT);
                w(c, SND_SLOT + 0x48, 0x800 + r.next_u32() % 0x800);
                w(c, SND_SLOT + 0x4c, r.next_u32() % 0x800);
                // The request: id 0 (stream 0), playing or not, its length.
                w(c, REQUEST, 0);
                w(c, REQUEST + 0xc, 0);
                w(c, REQUEST + 0x10, 0);
                c.bus.write8(REQUEST + 0x24, (r.next_u32() % 3 != 0) as u8);
                w(c, REQUEST + 0x14, 0);
                w(c, REQUEST + 0x18, r.next_u32() % 1000);
                w(c, REQUEST + 0x1c, r.next_u32() % 1000);
            },
        );
    }
    // -- 0x003c6c28 SNDSTRM_GetFreeBlocks --------------------------------------

    /// `SNDSTRM_GetFreeBlocks` against the generated body: slots 0..7 with
    /// 0..8 channels whose block indices are positive, negative (ignored:
    /// the maximum starts at 0) or equal, and signed block counts.
    #[test]
    fn sndstrm_get_free_blocks_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(ee::s13::f_003c6c28__generated, sndstrm_get_free_blocks, 300, &|c, seed| {
            let mut r = Rng(0x003c_6c28 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x003c_6c28;
            let slot = r.next_u32() % 8;
            let state = DATA + 0x100 * slot;
            c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
            c.bus
                .write32(SNDSTRM_SLOT_BASE + SNDSTRM_SLOT_STATES + 4 * slot, state);
            c.bus.write8(state + 0x62, (r.next_u32() % 9) as u8);
            for k in 0..8 {
                let v = match r.next_u32() % 4 {
                    0 => r.next_u32() as u16,
                    1 => 7,
                    _ => (r.next_u32() % 40) as u16,
                };
                c.bus.write16(state + 0x34 + 2 * k, v);
            }
            let blocks = if seed % 5 == 0 { r.next_u32() as u16 } else { (r.next_u32() % 64) as u16 };
            c.bus.write16(state + 0x40, blocks);
        });
    }
    // -- 0x00317328 App_InvokeExecCallback -------------------------------------

    /// `App_InvokeExecCallback` against the generated body: the real
    /// `PS2ExecMan_InvokeCallback` with no callback, or a callback and the
    /// notification virtual (both fakes, some unwinding).
    #[test]
    fn app_invoke_exec_callback_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::DATA;
        const APP: u32 = DATA + 0x100;
        const MAN: u32 = DATA + 0x200;
        const MAN_VT: u32 = DATA + 0x300;
        covered(ee::s10::f_00317328__generated, app_invoke_exec_callback, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_7328, 2);
            c.pc = 0x0031_7328;
            c.g[A0][0] = sx(APP);
            c.bus.write32(APP + 8, MAN);
            c.bus.write32(MAN, if seed % 4 == 0 { 0 } else { FAKE });
            c.bus.write32(MAN + 4, r.next_u32());
            c.bus.write32(MAN + 8, MAN_VT);
            c.bus.write16(MAN_VT + 0x70, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(MAN_VT + 0x74, FAKE + 0x10);
        });
    }
    // -- 0x002b2160 SongInstance_SetVolume -------------------------------------

    /// `SongInstance_SetVolume` against the generated body: volumes above
    /// 0xffff, negative, in range, and equal to the current one (nothing to
    /// do); the real `SongInstance_GetStreamHandle` finding the song's track
    /// (stream index valid or negative, stream handle valid or negative) or
    /// not; the real `SNDSTRM_SetLowpass` finds no stream. The SND lock
    /// hooks are fakes (some unwinding).
    #[test]
    fn song_instance_set_volume_matches() {
        use super::song_set_volume::*;
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::DATA;
        const THIS: u32 = DATA + 0x100;
        const TRACK: u32 = DATA + 0x200;
        const TRACK_DATA: u32 = DATA + 0x300;
        /// PATH: the track groups (`+0x30` flags, `+0x34` used, `+0x58`
        /// the tracks) and the per-stream track data pointers.
        const PATH_GROUPS: u32 = 0x0051_7610;
        const PATH_TRACK_DATA: u32 = 0x004d_3e98;
        covered(ee::s08::f_002b2160__generated, song_instance_set_volume, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x002b_2160, 2);
            c.pc = 0x002b_2160;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            let current = r.next_u32() % 0x1_0000;
            w(c, THIS + VOLUME, current);
            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = match seed % 6 {
                0 => current as u64,
                1 => 0x1_0000 + (r.next_u32() % 0x10_0000) as u64,
                2 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
                3 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                _ => (r.next_u32() % 0x1_0000) as u64,
            };
            // The song's track (key 0x11000001: group flags 0x1x000000, bit 0).
            if seed % 5 != 0 {
                w(c, PATH_GROUPS + 0x30, 0x1100_0000 | (r.next_u32() & 0x00ff_ffff));
                w(c, PATH_GROUPS + 0x34, 1);
                w(c, PATH_GROUPS + 0x58, TRACK);
            }
            let index = if seed % 7 == 0 { u32::MAX } else { r.next_u32() % 4 };
            w(c, TRACK + 0x54, index);
            for k in 0..4 {
                w(c, PATH_TRACK_DATA + 4 * k, TRACK_DATA + 4 * k);
                let handle = if r.next_u32() % 4 == 0 { 0xffff_fff8 } else { r.next_u32() % 8 };
                w(c, TRACK_DATA + 4 * k, handle);
            }
        });
    }
    // -- 0x003c6ab8 SNDSTRM_PushBlock ------------------------------------------

    /// `SNDSTRM_PushBlock` against the generated body: slots 0..7 with 0..8
    /// channels, positions below the last ring entry or at/after it (full),
    /// write indices before the end, at the last entry (wrap) and past it,
    /// size words with the top bit set or clear.
    #[test]
    fn sndstrm_push_block_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(ee::s13::f_003c6ab8__generated, sndstrm_push_block, 300, &|c, seed| {
            let mut r = Rng(0x003c_6ab8 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x003c_6ab8;
            let slot = r.next_u32() % 8;
            let state = DATA + 0x1000 * slot;
            let block = DATA + 0x2_0000;
            c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
            c.g[A1][0] = sx(block);
            c.bus
                .write32(SNDSTRM_SLOT_BASE + SNDSTRM_SLOT_STATES + 4 * slot, state);
            // The ring: 1..16 entries (sometimes a signed junk count), the
            // write index inside, at the end, or past it.
            let entries = if seed % 11 == 0 { r.next_u32() as u16 } else { (1 + r.next_u32() % 16) as u16 };
            c.bus.write16(state + 0x40, entries);
            let write = match r.next_u32() % 4 {
                0 => entries.wrapping_sub(1),
                1 => entries.wrapping_add((r.next_u32() % 3) as u16),
                _ => (r.next_u32() % entries.max(1) as u32) as u16,
            };
            c.bus.write16(state + 0x46, write);
            let channels = (r.next_u32() % 9) as u8;
            c.bus.write8(state + 0x62, channels);
            for k in 0..8 {
                let pos = if r.next_u32() % 8 == 0 {
                    entries.wrapping_sub(1)
                } else {
                    (r.next_u32() % (entries.max(2) as u32 - 1)) as u16
                };
                c.bus.write16(state + 0x34 + 2 * k, pos);
            }
            c.bus.write32(state + 4, r.next_u32());
            c.bus.write32(state + 0x48, r.next_u32());
            fill(c, &mut r, block, 12);
        });
    }
}
