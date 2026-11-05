//! Staging wave 203 (oc227): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_c8c8,
        ssx_audio_system_update_music_intensity,
        "SSXAudioSystem_UpdateMusicIntensity",
    ),
    (
        0x0016_2568,
        chase_camera_algorithm_gather_rider_state,
        "ChaseCameraAlgorithm_GatherRiderState",
    ),
    (0x003d_4f10, path_update_tracks, "PATH_UpdateTracks"),
];

/// An FPU register as a float.
#[inline]
fn fpr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `(int)(signed char)x`: the `sll 24; sra 24` the compiler emits for a
/// `char` argument or result.
#[inline]
fn sext8(x: u64) -> u64 {
    sx((((x as u32) << 24) as i32 >> 24) as u32)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateMusicIntensity (0x28c8c8).

/// Fields of the audio system (`SSXAudioSystem`) the music-intensity update
/// touches.
mod music {
    /// The music player (`MusicMonitor`, embedded).
    pub const MONITOR: u32 = 0x118;
    /// Its vtable pointer (event slot at +0x18).
    pub const MONITOR_VT: u32 = 0x5558;
    /// Ambience gain written for the mixer.
    pub const AMBIENCE_GAIN: u32 = 0x5830;
    /// Music gain (copy of the channel gain).
    pub const MUSIC_GAIN: u32 = 0x60a0;
    /// Speed latched when the rider left the ground.
    pub const AIR_SPEED: u32 = 0x6244;
    /// Lowest height seen during the jump.
    pub const AIR_LOW: u32 = 0x6248;
    /// Peak height expected for the jump.
    pub const AIR_HIGH: u32 = 0x624c;
    /// Frames the music stays held.
    pub const HOLD: u32 = 0x6250;
    /// Rider's air predictor (`rider + 0x788`): state, speed, heights.
    pub const RIDER_AIR: u32 = 0x788;
    pub const AIR_STATE: u32 = 0xac;
    pub const AIR_TAKEOFF_SPEED: u32 = 0x98;
    pub const AIR_PEAK: u32 = 0x9c;
    pub const AIR_HEIGHT: u32 = 0xa0;
    /// `gp` globals: "in a jump" this frame and last frame.
    pub const GP_IN_AIR: u32 = 0x52c;
    pub const GP_WAS_IN_AIR: u32 = 0x530;
}

const FIND_RIDER: Callee = Callee {
    addr: 0x0028_5d98,
    f: ee::s07::SSXAudioSystem_FindRider,
};
const GAIN_PTR: Callee = Callee {
    addr: 0x0028_7968,
    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
};
const MUSIC_TRIGGER: Callee = Callee {
    addr: 0x0028_d898,
    f: ee::s07::SSXAudioSystem_GetMusicTrigger,
};
const IS_REPLAY: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};
const START_AMBIENCE: Callee = Callee {
    addr: 0x0029_10e0,
    f: ee::s07::SSXAudioSystem_StartAmbienceLoop,
};
const STOP_AMBIENCE: Callee = Callee {
    addr: 0x0029_13d8,
    f: ee::s07::SSXAudioSystem_StopAmbienceLoop,
};
const MUSIC_ENABLED: Callee = Callee {
    addr: 0x0028_d960,
    f: ee::s07::SSXAudioSystem_IsGameMusicEnabled,
};

/// `void SSXAudioSystem_UpdateMusicIntensity(SSXAudioSystem* self)`
/// (0x28c8c8): while the local rider is in a jump (air state 1 or 3) fast
/// enough, duck the music by how high the rider is in the jump (music and
/// ambience gains, monitor volume/percent), start the ambience loop on
/// take-off and fire monitor event 7; on landing (or replay, or too slow)
/// stop the ambience loop, fire event 8 and restore full volume.
pub fn ssx_audio_system_update_music_intensity(c: &mut Cpu) {
    use music::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x68);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        let sys = r32(c, S1);
        let gp = r32(c, GP);

        // Hold counter.
        let hold = Addr(sys.wrapping_add(HOLD)).lw(c);
        c.g[V0][0] = sx((hold as u32).wrapping_sub(1));
        if (hold as i64) > 0 {
            c.cycles += 1;
            let v = c.g[V0][0];
            Addr(sys.wrapping_add(HOLD)).sw(c, v);
        }

        // The speed that drives the intensity: the take-off speed while in
        // a jump (latched on the first frame), 0 otherwise.
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = u64::MAX;
        FIND_RIDER.call(c, 0x0028_c910)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0011_fe98, 0x0028_c91c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        c.g[A1][0] = 0;
        let mut have_speed = false;
        if c.g[A0][0] == 1 {
            c.cycles += 4;
            let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
            c.g[V0][0] = air;
            let state = Addr((air as u32).wrapping_add(AIR_STATE)).lw(c);
            c.g[V1][0] = state;
            c.g[A2][0] = air;
            let jumping = if state == 1 {
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = 3;
                state == 3
            };
            if jumping {
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            c.cycles += 2;
            let in_air = Addr(gp.wrapping_add(GP_IN_AIR)).lw(c);
            c.g[V1][0] = in_air;
            c.cycles += 3;
            c.g[V0][0] = 1;
            if jumping {
                if in_air == 1 {
                    c.f[20] = Addr(sys.wrapping_add(AIR_SPEED)).read_u32(c);
                } else {
                    c.cycles += 4;
                    let speed = Addr((air as u32).wrapping_add(AIR_TAKEOFF_SPEED)).read_fpu(c);
                    c.f[20] = speed.0;
                    Addr(sys.wrapping_add(AIR_LOW)).sw(c, 0);
                    Addr(sys.wrapping_add(AIR_SPEED)).write_fpu(c, speed);
                }
                have_speed = true;
            } else if in_air == 1 {
                c.cycles += 2;
                c.f[20] = Addr(sys.wrapping_add(AIR_SPEED)).read_u32(c);
                have_speed = true;
            }
        }
        if !have_speed {
            c.cycles += 1;
            c.f[20] = 0;
        }
        let speed = fpr(c, 20);

        // Whether the monitor may be driven this frame (s3).
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[S3][0] = 0;
        GAIN_PTR.call(c, 0x0028_c99c)?;
        let threshold = Addr(gp.wrapping_sub(0x45d4)).read_fpu(c);
        c.f[0] = threshold.0;
        let gain = Addr(r32(c, V0)).read_fpu(c);
        c.f[1] = gain.0;
        let in_air = Addr(gp.wrapping_add(GP_IN_AIR)).lw(c);
        c.g[V0][0] = in_air;
        let fast = abi::c_lt_s(c, threshold, speed);
        Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, gain);
        Addr(gp.wrapping_add(GP_WAS_IN_AIR)).sw(c, in_air);
        Addr(gp.wrapping_add(GP_IN_AIR)).sw(c, 0);
        if fast {
            c.cycles += 6;
            c.g[A0][0] = sx(sys.wrapping_add(MONITOR));
            call_ptr(c, 0x002b_4878, 0x0028_c9c4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            if c.g[S0][0] == 1 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x002a_4168, 0x0028_c9dc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002a_41d8, 0x0028_c9ec)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        MUSIC_TRIGGER.call(c, 0x0028_c9fc)?;
                        if c.g[V0][0] == 0 {
                            c.g[S3][0] = c.g[S0][0];
                        }
                    }
                }
            }
        }

        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        IS_REPLAY.call(c, 0x0028_ca08)?;
        let replay = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
        if !replay {
            c.cycles += 6;
            c.g[AT][0] = 0x4000_0000;
            c.f[0] = 0x4000_0000;
            let moving = abi::c_lt_s(c, FpuF32(0x4000_0000), speed);
            c.g[V1][0] = 1;
            if moving {
                let extra = music_in_air(c, sys, gp)?;
                return music_finish(c, frame, extra);
            }
            c.cycles += 1;
            c.g[V1][0] = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
        }
        music_landed(c, sys)?;
        music_finish(c, frame, true)
    })
}

/// In a jump: scale music and ambience by the height ratio. Returns whether
/// the exit takes the extra cycle (`s0` reloaded at the common tail).
fn music_in_air(c: &mut Cpu, sys: u32, gp: u32) -> Result<bool, crate::call::Unwound> {
    use music::*;
    c.cycles += 7;
    let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
    c.g[V0][0] = air;
    let low_seen = Addr(sys.wrapping_add(AIR_LOW)).read_fpu(c);
    c.f[0] = low_seen.0;
    let height = Addr((air as u32).wrapping_add(AIR_HEIGHT)).read_fpu(c);
    let lower = abi::c_lt_s(c, height, low_seen);
    let v1 = c.g[V1][0];
    Addr(gp.wrapping_add(GP_IN_AIR)).sw(c, v1);
    let mut low = height;
    if lower {
        c.cycles += 1;
        low = low_seen;
    }
    c.f[4] = low.0;
    c.cycles += 3;
    let was_in_air = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
    c.g[V0][0] = was_in_air;
    Addr(sys.wrapping_add(AIR_LOW)).write_fpu(c, low);
    let high = if was_in_air == 0 {
        // First frame of the jump: expect half the remaining rise above
        // the lowest point (at least 0.5).
        c.cycles += 11;
        let air = Addr(r32(c, S2).wrapping_add(RIDER_AIR)).lw(c);
        c.g[V0][0] = air;
        c.g[AT][0] = 0x3f00_0000;
        let half = FpuF32(0x3f00_0000);
        c.f[2] = half.0;
        let h = Addr((air as u32).wrapping_add(AIR_HEIGHT)).read_fpu(c);
        c.f[1] = h.0;
        let peak = Addr((air as u32).wrapping_add(AIR_PEAK)).read_fpu(c);
        let rise = peak - h;
        c.f[0] = rise.0;
        let mut high = rise * half;
        if abi::c_lt_s(c, high, half) {
            high = half;
        }
        c.cycles += 3;
        let high = high + low;
        c.f[3] = high.0;
        Addr(sys.wrapping_add(AIR_HIGH)).write_fpu(c, high);
        high
    } else {
        c.cycles += 1;
        let high = Addr(sys.wrapping_add(AIR_HIGH)).read_fpu(c);
        c.f[3] = high.0;
        high
    };

    // Volume floor from speed: clamp((1 - speed * k * 0.5) * 127, 20, 127).
    c.cycles += 18;
    let k = Addr(gp.wrapping_sub(0x45d0)).read_fpu(c);
    let below = abi::c_lt_s(c, low, high);
    let half = FpuF32(0x3f00_0000);
    let one = FpuF32(0x3f80_0000);
    let vmax = FpuF32(0x42fe_0000);
    let vmin = FpuF32(0x41a0_0000);
    let speed = fpr(c, 20);
    let scaled = (speed * k) * half;
    c.f[0] = scaled.0;
    c.f[2] = half.0;
    let inv = Addr(gp.wrapping_sub(0x45cc)).read_fpu(c);
    c.f[23] = inv.0;
    let rest = one - scaled;
    c.f[1] = rest.0;
    c.g[AT][0] = 0x41a0_0000;
    c.f[21] = vmax.0;
    c.f[5] = vmin.0;
    let floor = (rest * vmax).min(vmax).max(vmin);
    c.f[20] = floor.0;
    let play = c.g[S3][0] != 0;
    let mon = sx(sys.wrapping_add(MONITOR));

    if below {
        c.cycles += 13;
        let ratio = low.div(high);
        c.f[22] = ratio.0;
        c.g[AT][0] = 0x42d6_0000;
        c.g[S2][0] = 0x7f;
        let t = vmax - ratio * FpuF32(0x42d6_0000);
        c.f[0] = t.0;
        let v = t.min(vmax).max(vmin);
        c.f[1] = v.0;
        c.f[0] = v.to_i32() as u32;
        c.g[V0][0] = sx((c.f[0]) << 24);
        c.g[S0][0] = sext8(c.f[0] as u64);
        if play {
            c.cycles += 21;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            GAIN_PTR.call(c, 0x0028_cb1c)?;
            let k = Addr(r32(c, V0)).read_fpu(c);
            c.g[A0][0] = mon;
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0x14));
            c.g[AT][0] = 0x42c8_0000;
            c.g[V0][0] = sx(0x7fu32.wrapping_sub(r32(c, V0)));
            let k = k * inv;
            c.f[1] = k.0;
            let g = FpuF32::from_i32(r32(c, V0) as i32) * k;
            c.f[0] = g.0;
            let pct = g * FpuF32(0x42c8_0000);
            c.f[2] = pct.0;
            Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, g);
            c.f[0] = pct.to_i32() as u32;
            c.g[A1][0] = sext8(c.f[0] as u64);
            call_ptr(c, 0x002b_3d10, 0x0028_cb60)?;
        }
        c.cycles += 15;
        let floor = fpr(c, 20);
        let ratio = fpr(c, 22);
        let d = (vmax - floor) * ratio;
        c.f[1] = d.0;
        let v = (vmax - d).min(vmax).max(floor);
        c.f[0] = v.0;
        c.f[1] = v.to_i32() as u32;
        c.g[V0][0] = sx(c.f[1] << 24);
        c.g[S0][0] = sext8(c.f[1] as u64);
        c.g[V1][0] = sx(0x7fu32.wrapping_sub(r32(c, S0)));
        let amb = FpuF32::from_i32(r32(c, V1) as i32) * inv;
        c.f[0] = amb.0;
        Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
        if play {
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = mon;
            call_ptr(c, 0x002b_3c98, 0x0028_cba8)?;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        }
    } else {
        c.cycles += 4;
        let amb = (vmax - floor) * inv;
        c.f[0] = amb.0;
        Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
        if play {
            c.cycles += 22;
            c.f[0] = floor.to_i32() as u32;
            c.g[S0][0] = mon;
            c.g[A0][0] = mon;
            c.g[A1][0] = sext8(c.f[0] as u64);
            call_ptr(c, 0x002b_3c98, 0x0028_cbdc)?;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            GAIN_PTR.call(c, 0x0028_cbec)?;
            let g = Addr(r32(c, V0)).read_fpu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[AT][0] = 0x42c8_0000;
            c.f[1] = 0x42c8_0000;
            Addr(sys.wrapping_add(MUSIC_GAIN)).write_fpu(c, g);
            let pct = g * FpuF32(0x42c8_0000);
            c.f[0] = pct.0;
            c.f[1] = pct.to_i32() as u32;
            c.g[A1][0] = sext8(c.f[1] as u64);
            call_ptr(c, 0x002b_3d10, 0x0028_cc18)?;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S1][0];
    }

    // Ambience gain times the master ambience channel.
    c.cycles += 9;
    c.g[A1][0] = 5;
    c.g[A2][0] = 0;
    GAIN_PTR.call(c, 0x0028_cc28)?;
    let master = Addr(r32(c, V0)).read_fpu(c);
    c.f[0] = master.0;
    let amb = Addr(sys.wrapping_add(AMBIENCE_GAIN)).read_fpu(c) * master;
    let was_in_air = Addr(gp.wrapping_add(GP_WAS_IN_AIR)).lw(c);
    c.g[V0][0] = was_in_air;
    c.f[1] = amb.0;
    Addr(sys.wrapping_add(AMBIENCE_GAIN)).write_fpu(c, amb);
    if was_in_air != 0 {
        return Ok(true);
    }
    // Take-off: start the ambience loop, fire event 7.
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    START_AMBIENCE.call(c, 0x0028_cc48)?;
    c.g[S0][0] = mon;
    if c.g[S3][0] == 0 {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002b_4620, 0x0028_cc58)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    MUSIC_ENABLED.call(c, 0x0028_cc68)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 8;
    monitor_event(c, sys, 7, 0x0028_cc88)?;
    Ok(false)
}

/// `monitor->vt->event(monitor + delta, n)` (slot +0x18 of the vtable at
/// `sys + 0x5558`), with the monitor in s0.
fn monitor_event(c: &mut Cpu, sys: u32, event: u64, ret: u32) -> Flow {
    let vt = Addr(sys.wrapping_add(music::MONITOR_VT)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = event;
    let delta = Addr((vt as u32).wrapping_add(0x18)).lh(c);
    c.g[A0][0] = delta;
    let func = Addr((vt as u32).wrapping_add(0x1c)).lw(c);
    c.g[V1][0] = func;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// Landed (or replay / too slow): if the jump just ended stop the ambience
/// loop and fire event 8; then full volume and the stored music gain.
fn music_landed(c: &mut Cpu, sys: u32) -> Flow {
    use music::*;
    c.cycles += 3;
    c.g[V0][0] = 1;
    let mon = sx(sys.wrapping_add(MONITOR));
    c.g[S0][0] = mon;
    if c.g[V1][0] == 1 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        STOP_AMBIENCE.call(c, 0x0028_cca8)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b_46e8, 0x0028_ccb0)?;
        c.g[A1][0] = 0x7f;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MUSIC_ENABLED.call(c, 0x0028_ccc0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                monitor_event(c, sys, 8, 0x0028_cce0)?;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002b_4708, 0x0028_cce8)?;
            c.cycles += 1;
            c.g[A1][0] = 0x7f;
        }
    } else {
        c.cycles += 1;
        c.g[A1][0] = 0x7f;
    }
    c.cycles += 12;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002b_3c98, 0x0028_ccf4)?;
    c.g[AT][0] = 0x42c8_0000;
    c.f[1] = 0x42c8_0000;
    c.g[A0][0] = c.g[S0][0];
    let g = Addr(sys.wrapping_add(MUSIC_GAIN)).read_fpu(c);
    let pct = g * FpuF32(0x42c8_0000);
    c.f[0] = pct.0;
    c.f[1] = pct.to_i32() as u32;
    c.g[A1][0] = sext8(c.f[1] as u64);
    call_ptr(c, 0x002b_3d10, 0x0028_cd1c)
}

/// Common tail: `extra` is the path through the 1-cycle `lq s0` block.
fn music_finish(c: &mut Cpu, frame: Frame, extra: bool) -> Flow {
    if extra {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x40);
    c.cycles += 10;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 23, 0x68);
    frame.restore_fpr(c, 22, 0x60);
    frame.restore_fpr(c, 21, 0x58);
    frame.restore_fpr(c, 20, 0x50);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_GatherRiderState (0x162568).

/// `vf4.w = vf3 . vf(b)` (xyz), the compiler's VU0 dot-product idiom
/// (`vf6.w` as the 1.0 multiplier through ACC).
#[inline]
fn vu0_dot3(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `|v|` of the vector at `addr` into Q (`sqrt Q, vf4.w`), then `cfc2 rt, Q`
/// and `mtc1 rt, f`.
#[inline]
fn vu0_length(c: &mut Cpu, addr: u32, f: usize) {
    c.lqc2(3, addr);
    vu0_dot3(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, f);
}

/// Call virtual slot `slot` of the camera target (`self + 0x30`) with
/// `this` in `this_reg` (the other argument register keeps what the caller
/// set): `v0`/`v1` hold the vtable and function as the original loads them
/// (`vt_reg`, `fn_reg`).
#[inline]
fn target_vcall(
    c: &mut Cpu,
    self_: u32,
    obj_reg: usize,
    vt_reg: usize,
    this_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> Flow {
    c.g[obj_reg][0] = Addr(self_.wrapping_add(0x30)).lw(c);
    let obj = r32(c, obj_reg);
    c.g[vt_reg][0] = Addr(obj).lw(c);
    let vt = r32(c, vt_reg);
    c.g[this_reg][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[fn_reg][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let func = r32(c, fn_reg);
    c.g[this_reg][0] = sx(obj.wrapping_add(r32(c, this_reg)));
    call_ptr(c, func, ret)
}

/// `void ChaseCameraAlgorithm_GatherRiderState(ChaseCameraAlgorithm* self,
/// CamInfo* out, float a..h)` (0x162568): read the target's velocity (kept
/// from last frame below 27.8 m/s), split it into its horizontal part and
/// the vertical rest, blend the stored horizontal heading towards it by a
/// factor from the slope of the velocity (`|n.z|` mapped between two
/// thresholds onto [lo, hi]; the second set of parameters in trick/rail
/// states 2, 3 and 13), and fill the camera info: rider mode flags, two
/// target parameters, the frame (position, matrix, velocity), its speed and
/// a 0..1 boost amount; mirror three of them into the algorithm.
pub fn chase_camera_algorithm_gather_rider_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        // Keep the eight float parameters in callee-saved registers:
        // f25..f27 = a, b, c; f24 = d; f22, f23 = e, f; f21 = g; f20 = h.
        for (f, src, off) in [
            (27, 14, 0x108),
            (26, 13, 0x100),
            (25, 12, 0xf8),
            (24, 15, 0xf0),
            (23, 17, 0xe8),
            (22, 16, 0xe0),
            (21, 18, 0xd8),
        ] {
            frame.save_fpr(c, f, off);
            c.f[f] = c.f[src];
        }
        frame.save_fpr(c, 20, 0xd0);
        frame.save_ra(c, 0x90);
        c.f[20] = c.f[19];
        let me = r32(c, S1);
        let out = r32(c, S0);
        let sp0 = sp(c);

        // Velocity (returned in sp+0) and the rider's mode.
        c.g[A0][0] = Addr(me.wrapping_add(0x30)).lw(c);
        let obj = r32(c, A0);
        c.g[V0][0] = Addr(obj).lw(c);
        let vt = r32(c, V0);
        c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
        c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
        c.g[A0][0] = sx(sp0);
        let func = r32(c, V1);
        call_ptr(c, func, 0x0016_25e0)?;
        c.g[V0][0] = Addr(me.wrapping_add(0x30)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        call_ptr(c, 0x0011_fe98, 0x0016_25ec)?;
        c.g[S2][0] = c.g[V0][0];
        c.lwc1(1, r32(c, GP).wrapping_sub(0x6afc)); // 27.78 m/s
        vu0_length(c, sp0, 0);
        c.c_lt_s(0, 1);
        c.lq(2, sp0);
        if c.fcc() {
            // Too slow: keep last frame's velocity.
            c.cycles += 3;
            c.lq(2, me.wrapping_add(0x1b0));
            c.sq(2, sp0);
            c.lq(2, sp0);
        }

        // Horizontal part (sp+0x10), vertical rest (sp+0x20), direction
        // (sp+0x30).
        c.cycles += 23;
        c.mtc1(0, 1);
        c.sq(2, sp0 + 0x10);
        c.swc1(1, sp0 + 0x18);
        c.sq(2, me.wrapping_add(0x1b0));
        c.lqc2(3, sp0);
        c.lqc2(4, sp0 + 0x10);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x30);
        c.lq(2, sp0 + 0x30);
        c.sq(2, sp0 + 0x20);
        c.lqc2(3, sp0);
        c.lqc2(5, sp0);
        vu0_dot3(c, 5);
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.c_lt_s(1, 0);
        c.g[V0][0] = 0x50_0000;
        if c.fcc() {
            c.cycles += 13;
            c.lqc2(3, sp0);
            vu0_dot3(c, 3);
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, sp0 + 0x40);
            c.lq(2, sp0 + 0x40);
            c.sq(2, sp0 + 0x30);
        } else {
            // Zero velocity: the default direction (0x4ff120).
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            c.lq(3, r32(c, V0));
            c.sq(3, sp0 + 0x30);
        }

        // Pick the parameter set by the target's state.
        c.cycles += 11;
        target_vcall(c, me, A1, V1, A0, V0, 0xa0, 0x0016_26ec)?;
        c.g[V1][0] = c.g[V0][0];
        let state = c.g[V1][0];
        c.g[V0][0] = (sx((state as u32).wrapping_sub(2)) < 2) as u64;
        c.f[12] = c.f[22];
        let second = if c.g[V0][0] != 0 {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            c.f[12] = c.f[25];
            if state == 0xd {
                c.cycles += 1;
                c.f[12] = c.f[22];
                true
            } else {
                false
            }
        };
        // f12 = lo, f13 = hi, f14 = t0, f15 = t1.
        if second {
            c.cycles += 4;
            c.f[13] = c.f[23];
            c.f[14] = c.f[21];
            c.f[15] = c.f[20];
        } else {
            c.cycles += 3;
            c.f[13] = c.f[26];
            c.f[14] = c.f[27];
            c.f[15] = c.f[24];
        }

        // frac = clamp((|dir.z| - t0) / (t1 - t0), 0, 1).
        c.cycles += 6;
        let x = Addr(sp0 + 0x38).read_fpu(c).abs();
        c.f[0] = x.0;
        let (lo, hi, t0, t1) = (fpr(c, 12), fpr(c, 13), fpr(c, 14), fpr(c, 15));
        let frac = if !abi_le(c, t0, x) {
            c.cycles += 1;
            FpuF32::ZERO
        } else {
            c.cycles += 3;
            if abi_le(c, x, t1) {
                c.cycles += 5;
                let span = t1 - t0;
                let d = x - t0;
                c.f[0] = d.0;
                d.div(span)
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                FpuF32::ONE
            }
        };
        c.f[1] = frac.0;
        let range = hi - lo;

        // r = lo + (hi - lo) * frac; while racing (mode 1) a slow
        // horizontal velocity keeps the stored heading.
        c.cycles += 4;
        c.g[V0][0] = 1;
        let step = range * frac;
        c.f[0] = step.0;
        c.f[12] = (lo + step).0;
        if c.g[S2][0] == 1 {
            c.cycles += 14;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x6af8)); // 27.78 m/s
            vu0_length(c, sp0 + 0x10, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 2;
                c.lq(2, me.wrapping_add(0x100));
                c.sq(2, sp0 + 0x10);
            }
        }

        // heading = heading * r + horizontal * (1 - r); position = heading
        // + vertical rest. Mode flags.
        c.cycles += 89;
        c.mfc1(2, 12);
        c.lqc2(4, me.wrapping_add(0x100));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp0 + 0x70);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.lq(2, sp0 + 0x70);
        let mode = c.g[S2][0];
        c.g[A2][0] = mode ^ 4;
        c.sub_s(0, 20, 12);
        c.g[A1][0] = mode ^ 2;
        c.sq(2, sp0 + 0x60);
        c.g[A0][0] = mode ^ 5;
        c.mfc1(2, 0);
        c.lqc2(4, sp0 + 0x10);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp0 + 0x80);
        c.g[V1][0] = mode ^ 1;
        c.lq(2, sp0 + 0x80);
        c.g[A2][0] = (c.g[A2][0] == 0) as u64;
        c.g[A1][0] = (c.g[A1][0] == 0) as u64;
        c.g[A0][0] = (c.g[A0][0] == 0) as u64;
        c.sq(2, sp0 + 0x70);
        c.g[V1][0] = (c.g[V1][0] == 0) as u64;
        c.lqc2(3, sp0 + 0x60);
        c.lqc2(4, sp0 + 0x70);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x80);
        c.lq(2, sp0 + 0x80);
        c.sq(2, me.wrapping_add(0x100));
        c.sq(2, sp0 + 0x50);
        c.sq(2, sp0 + 0x40);
        c.lqc2(3, sp0 + 0x40);
        c.lqc2(4, sp0 + 0x20);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x60);
        let (is4, is2, is5, is1) = (c.g[A2][0], c.g[A1][0], c.g[A0][0], c.g[V1][0]);
        Addr(out + 0x64).sw(c, is4);
        Addr(out + 0x58).sw(c, is2);
        Addr(out + 0x5c).sw(c, is5);
        Addr(out + 0x60).sw(c, is1);
        Addr(out + 0x40).sw(c, mode);
        c.lq(2, sp0 + 0x60);
        c.sq(2, sp0 + 0x50);
        // Target parameters (slots 0x88, 0x90) and frame (slot 0x8).
        target_vcall(c, me, A1, V1, A0, V0, 0x88, 0x0016_2890)?;
        c.swc1(0, out + 0x4c);
        target_vcall(c, me, A1, V0, A0, V1, 0x90, 0x0016_28ac)?;
        c.swc1(0, out + 0x54);
        c.g[A0][0] = sx(sp0 + 0x80);
        c.lq(2, sp0 + 0x50);
        c.g[A2][0] = Addr(me.wrapping_add(0x30)).lw(c);
        c.sq(2, sp0 + 0x60);
        let obj = r32(c, A2);
        c.g[V1][0] = Addr(obj).lw(c);
        let vt = r32(c, V1);
        c.g[A1][0] = Addr(vt.wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0xc)).lw(c);
        c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
        let func = r32(c, V0);
        call_ptr(c, func, 0x0016_28d4)?;
        c.lq(3, me.wrapping_add(0x40));
        c.lq(4, sp0 + 0x80);
        c.lq(2, sp0 + 0x60);
        c.lwc1(1, out + 0x4c);
        c.mtc1(0, 2);
        c.sq(3, out);
        c.sq(4, out + 0x10);
        c.c_lt_s(2, 1);
        c.sq(2, out + 0x20);
        c.lqc2(3, out + 0x20);
        vu0_dot3(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 0);
        c.sq(4, sp0 + 0x70);
        c.swc1(0, out + 0x44);
        // Boost amount: (p + 1) / 2 for a positive parameter, else 0.
        if c.fcc() {
            c.cycles += 5;
            c.add_s(1, 1, 20);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 0);
            c.mul_s(1, 1, 0);
        } else {
            c.cycles += 1;
            c.f[1] = c.f[2];
        }

        c.cycles += 21;
        c.lwc1(0, out + 0x4c);
        c.swc1(1, out + 0x50);
        c.swc1(0, me.wrapping_add(0x2f4));
        c.lwc1(1, out + 0x50);
        c.swc1(1, me.wrapping_add(0x2f8));
        c.g[V0][0] = Addr(out + 0x5c).lw(c);
        let v0 = c.g[V0][0];
        Addr(me.wrapping_add(0x2fc)).sw(c, v0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore_ra(c, 0x90);
        for (f, off) in [
            (27, 0x108),
            (26, 0x100),
            (25, 0xf8),
            (24, 0xf0),
            (23, 0xe8),
            (22, 0xe0),
            (21, 0xd8),
            (20, 0xd0),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// `c.le.s a, b` (latched in FCC).
#[inline]
fn abi_le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

// ---------------------------------------------------------------------------
// PATH_UpdateTracks (0x3d4f10).

/// PATH streaming-music globals (`0x44f420..`).
mod path {
    /// `PathState*`: tick lengths at +0xc (normal) and +0x10 (streamed),
    /// the 24 track slots at +0x58.
    pub const STATE: u32 = 0x0044_f420;
    /// Mode byte: `'B'` = streamed (block) mode.
    pub const MODE: u32 = 0x0044_f434;
    /// Current PATH time.
    pub const TIME: u32 = 0x0044_f438;
    pub const TRACKS: u32 = 24;
    pub const SLOT0: u32 = 0x58;
}

/// Track (`PathTrack`) fields.
mod track {
    pub const BUSY: u32 = 0x3;
    pub const VOLUME_FADE: u32 = 0xd;
    pub const PLAY_STATE: u32 = 0xe;
    pub const AUX_FADE: u32 = 0x15;
    pub const FADE3: u32 = 0x1d;
    pub const LEAD: u32 = 0x24;
    pub const STREAMED: u32 = 0x2f;
    pub const ACTIVE: u32 = 0x30;
    pub const NODE: u32 = 0x36;
    pub const DEADLINE: u32 = 0x48;
    pub const STOP_AT: u32 = 0x4c;
    pub const HANDLE: u32 = 0x54;
}

const PATH_SERVICE_QUEUES: Callee = Callee {
    addr: 0x003d_1e80,
    f: ee::s14::PATH_ServiceQueues,
};
const PATH_VOLUME_FADE: Callee = Callee {
    addr: 0x003d_5a98,
    f: ee::s14::PATH_TrackUpdateVolumeFade,
};
const PATH_AUX_FADE: Callee = Callee {
    addr: 0x003d_5cc0,
    f: ee::s14::PATH_TrackUpdateAuxSendFade,
};
const PATH_FADE3: Callee = Callee {
    addr: 0x003d_5ee8,
    f: ee::s14::PATH_TrackUpdateFade3,
};
const PATH_QUEUED_COUNT: Callee = Callee {
    addr: 0x003d_4950,
    f: ee::s14::PATH_TrackGetQueuedCount,
};
const PATH_TRACK_ADVANCE: Callee = Callee {
    addr: 0x003d_3d20,
    f: ee::s14::PATH_TrackAdvance,
};

/// `void PATH_UpdateTracks(void)` (0x3d4f10): the per-tick service of the
/// PATH interactive-music player. Unless in streamed (`'B'`) mode, service
/// the event queues first. For each of the 24 track slots that is in use
/// and idle: run its volume / aux-send / third fades while it is active;
/// then, for the active tracks of the current mode, stop a track whose stop
/// time has come, or (if its platform stream is playing) advance it to the
/// next node once its queue runs low. Finally (normal mode) service the
/// queues until they are empty.
pub fn path_update_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = 0x45_0000;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = Addr(path::MODE).lbu(c);
        frame.save(c, S4, 0x50);
        c.g[V0][0] ^= 0x42;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S4][0] = (c.g[V0][0] == 0) as u64;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        // s6 = tick length, s7 = half of it, fp = 0x450000 (globals base).
        let streamed = c.g[S4][0] != 0;
        c.cycles += if streamed { 5 } else { 4 };
        c.g[V0][0] = 0x45_0000;
        c.g[V1][0] = Addr(path::STATE).lw(c);
        c.g[FP][0] = c.g[V0][0];
        let tick_off = if streamed { 0x10 } else { 0xc };
        c.g[S6][0] = Addr(r32(c, V1).wrapping_add(tick_off)).lw(c);

        c.cycles += 2;
        c.g[S3][0] = 0;
        if !streamed {
            c.cycles += 2;
            PATH_SERVICE_QUEUES.call(c, 0x003d_4f84)?;
        }
        c.cycles += 3;
        c.g[S7][0] = sx(r32(c, S6) >> 1);
        c.g[V0][0] = Addr(path::STATE).lw(c);
        loop {
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = Addr(r32(c, V0).wrapping_add(path::SLOT0)).lw(c);
            if c.g[S0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(track::BUSY)).lbu(c);
                if c.g[V0][0] == 0 {
                    path_update_track(c)?;
                }
            }
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            c.cycles += 3;
            let more = c.g[S3][0] < path::TRACKS as u64;
            c.g[V0][0] = Addr(path::STATE).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_4f90);
            }
        }

        // Drain the queues (s0 is already restored for the callee's frame).
        c.cycles += 1;
        loop {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            if c.g[S4][0] != 0 {
                break;
            }
            c.cycles += 4;
            PATH_SERVICE_QUEUES.call(c, 0x003d_50f0)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_50e0);
            }
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
        Ok(())
    })
}

/// One idle track (s0, slot s3) of `PATH_UpdateTracks`.
fn path_update_track(c: &mut Cpu) -> Flow {
    use track::*;
    let t = r32(c, S0);
    c.cycles += 8;
    c.g[V0][0] = 0x45_0000;
    c.g[A1][0] = Addr(t + LEAD).lw(c);
    c.g[V1][0] = Addr(path::TIME).lw(c);
    c.g[S5][0] = c.g[V0][0];
    c.g[A0][0] = Addr(t + NODE).lh(c);
    c.g[S2][0] = 0;
    // s1 = time + lead: the end of what is queued.
    c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    if (c.g[A0][0] as i64) >= 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(t + ACTIVE).lw(c);
        c.g[S2][0] = (c.g[V0][0] != 0) as u64;
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        c.cycles += 3;
        let fades = [
            (VOLUME_FADE, PATH_VOLUME_FADE, 0x003d_4ff4),
            (AUX_FADE, PATH_AUX_FADE, 0x003d_5008),
            (FADE3, PATH_FADE3, 0x003d_501c),
        ];
        for (i, (field, fade, ret)) in fades.into_iter().enumerate() {
            if i > 0 {
                c.cycles += 2;
            }
            c.g[V0][0] = Addr(t + field).lb(c);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                fade.call(c, ret)?;
            }
        }
    }
    c.g[V0][0] = Addr(t + STREAMED).lbu(c);
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    if c.g[V0][0] != c.g[S4][0] {
        return Ok(());
    }
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        return Ok(());
    }

    c.cycles += 3;
    c.g[V1][0] = Addr(t + STOP_AT).lw(c);
    c.g[V0][0] = Addr(path::TIME).lw(c);
    if c.g[V1][0] != 0 {
        // Stop pending: stop the stream once time + tick/2 reaches it.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S7)));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x002a_e9f8, 0x003d_505c)?;
        Addr(t + STOP_AT).sw(c, 0);
        c.cycles += 1;
        return Ok(());
    }
    // Stream playing?
    c.cycles += 4;
    c.g[A0][0] = Addr(t + HANDLE).lw(c);
    call_ptr(c, 0x002a_ebd0, 0x003d_506c)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    PATH_QUEUED_COUNT.call(c, 0x003d_507c)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = Addr(t + DEADLINE).lw(c);
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, S6)));
    c.cycles += 2;
    // Not yet: the queue reaches past the next tick (deadline), or holds
    // more than the lead.
    if c.g[V0][0] != 0 {
        c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
    } else {
        c.g[V0][0] = Addr(t + LEAD).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(t + PLAY_STATE).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = Addr(t + HANDLE).lw(c);
        call_ptr(c, 0x002a_f428, 0x003d_50b8)?;
        let started = c.g[V0][0] != 0;
        c.g[V0][0] = u64::MAX;
        if !started {
            c.cycles += 1;
            Addr(t + PLAY_STATE).sb(c, u64::MAX);
        }
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    PATH_TRACK_ADVANCE.call(c, 0x003d_50cc)?;
    c.cycles += 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_UpdateChaseHeading (0x162c78).

/// `void ChaseCameraAlgorithm_UpdateChaseHeading(ChaseCameraAlgorithm*
/// self, CamInfo* info)` (0x162c78): above 27.8 m/s, measure the angle
/// between the stored chase heading (`self + 0x80`) and the target's
/// velocity (kept from last frame when shorter than 1), pass it (less a
/// 10.9 degree dead zone) through a 5-tap filter history at `self + 0x340`
/// (input, output and error rings, gains at +0x40..+0x48), and turn the
/// filtered angle into a blend rate (square-rooted twice in rider states
/// 11..13); blend the heading towards `info->velocity / info->speed`.
/// Below that speed keep the heading. Store the normalised heading in
/// `info + 0x30`.
pub fn chase_camera_algorithm_update_chase_heading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(0x6ae0)); // 27.78 m/s
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A1][0];
        let info = r32(c, S1);
        c.lwc1(0, info.wrapping_add(0x44));
        c.c_lt_s(1, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = r32(c, S0);
        let sp0 = sp(c);
        if c.fcc() {
            chase_heading_follow(c, me, info, sp0, gp)?;
        } else {
            c.cycles += 1;
            c.lq(2, me.wrapping_add(0x80));
        }

        // info->heading = normalize(heading).
        c.cycles += 25;
        c.sq(2, sp0);
        vu0_length(c, sp0, 0);
        frame.restore(c, S0, 0xa0);
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, sp0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, sp0);
        frame.restore_ra(c, 0x80);
        c.lq(2, sp0);
        c.sq(2, info.wrapping_add(0x30));
        frame.restore(c, S1, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The fast branch of `UpdateChaseHeading`: filter the heading error and
/// blend; leaves the new heading in `$2` (and at `self + 0x80`).
fn chase_heading_follow(c: &mut Cpu, me: u32, info: u32, sp0: u32, gp: u32) -> Flow {
    // Target velocity (virtual slot 0x18 of the target, into sp+0x10);
    // keep last frame's when |v|^2 < 1.
    c.cycles += 22;
    c.g[A2][0] = Addr(me.wrapping_add(0x30)).lw(c);
    let obj = r32(c, A2);
    c.g[A0][0] = sx(sp0 + 0x10);
    c.g[V0][0] = Addr(obj).lw(c);
    let vt = r32(c, V0);
    c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
    let func = r32(c, V1);
    call_ptr(c, func, 0x0016_2cc0)?;
    c.g[AT][0] = 0x3f80_0000;
    c.f[4] = 0x3f80_0000;
    c.lqc2(3, sp0 + 0x10);
    c.lqc2(5, sp0 + 0x10);
    vu0_dot3(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 4);
    c.lq(2, sp0 + 0x10);
    if c.fcc() {
        c.cycles += 3;
        c.lq(2, me.wrapping_add(0x1c0));
        c.sq(2, sp0 + 0x10);
        c.lq(2, sp0 + 0x10);
    }

    // len = sqrt(|h|^2 |v|^2); angle = 90 deg - acos(clamp(h.v / len)).
    c.cycles += 25;
    c.lwc1(2, gp.wrapping_sub(0x6adc)); // 0.001
    c.sq(2, me.wrapping_add(0x1c0));
    c.lqc2(3, me.wrapping_add(0x80));
    c.lqc2(5, me.wrapping_add(0x80));
    vu0_dot3(c, 5);
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    c.lqc2(3, me.wrapping_add(0x1c0));
    c.lqc2(5, me.wrapping_add(0x1c0));
    vu0_dot3(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.mul_s(0, 0, 1);
    c.sqrt_s(3, 0);
    c.c_lt_s(3, 2);
    if c.fcc() {
        c.cycles += 3;
        c.f[7] = 0;
    } else {
        c.cycles += 16;
        c.lqc2(3, me.wrapping_add(0x80));
        c.lqc2(5, me.wrapping_add(0x1c0));
        vu0_dot3(c, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[1] = 0xbf80_0000;
        c.div_s(0, 0, 3);
        c.c_le_s(1, 0);
        c.f[12] = c.f[1];
        if c.fcc() {
            c.cycles += 1;
            c.min_s(12, 0, 4);
        }
        c.cycles += 5;
        call_ptr(c, 0x0031_c128, 0x0016_2dc4)?; // acosf
        c.lwc1(1, gp.wrapping_sub(0x6ad8)); // pi/2
        c.sub_s(7, 1, 0);
    }
    c.g[V0][0] = Addr(me.wrapping_add(0x37c)).lw(c);

    // f7 = max(angle - dead zone, 0); f3 = f7 - previous output; f2 = sum
    // of the error ring.
    c.cycles += 4;
    c.g[V1][0] = 5;
    c.lwc1(1, gp.wrapping_sub(0x6ad4)); // dead zone
    c.cycles += 16;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
    c.f[0] = 0;
    c.div(0, 2, 3);
    c.sub_s(7, 7, 1);
    c.g[V0][0] = 5;
    c.g[T0][0] = sx(me.wrapping_add(0x368));
    c.f[2] = c.f[0];
    c.max_s(7, 7, 0);
    c.g[A3][0] = sx(me.wrapping_add(0x340));
    c.g[V0][0] = c.g[T0][0];
    c.g[V1][0] = 4;
    c.g[A0][0] = c.hi[0];
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[A0][0] = sx(me.wrapping_add(r32(c, A0)));
    c.lwc1(4, r32(c, A0).wrapping_add(0x354));
    c.sub_s(3, 7, 4);
    loop {
        c.cycles += 7;
        c.lwc1(0, r32(c, V0));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.add_s(2, 2, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_2e20);
        }
    }

    // Filter step: out = prev + d*k0 + sum*k1 + (in[prev] - f7)*k2; store
    // in/out/error at idx, advance idx; rate from the output.
    c.cycles += 4;
    let hist = r32(c, A3);
    c.g[A0][0] = Addr(hist + 0x3c).lw(c);
    c.g[A1][0] = 5;
    c.cycles += 52;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(4));
    c.lwc1(0, hist + 0x40);
    c.div(0, 2, 5);
    c.lwc1(1, hist + 0x44);
    c.mul_s(0, 3, 0);
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.mul_s(1, 2, 1);
    c.lwc1(3, hist + 0x48);
    c.g[A0][0] = sx(hist.wrapping_add(r32(c, A0)));
    c.lwc1(6, gp.wrapping_sub(0x6ad0));
    c.add_s(0, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.f[8] = 0x3f80_0000;
    c.f[2] = 0;
    c.lwc1(4, gp.wrapping_sub(0x6acc));
    c.add_s(0, 0, 1);
    c.lwc1(5, gp.wrapping_sub(0x6ac8));
    c.g[V1][0] = c.hi[0];
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(hist.wrapping_add(r32(c, V1)));
    c.lwc1(1, r32(c, V1));
    c.swc1(7, r32(c, A0));
    c.sub_s(1, 1, 7);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.mul_s(1, 1, 3);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(hist.wrapping_add(r32(c, V0)));
    c.add_s(0, 0, 1);
    c.swc1(0, r32(c, V0).wrapping_add(0x14));
    c.sub_s(1, 7, 0);
    c.mul_s(0, 0, 6);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.min_s(0, 0, 8);
    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.max_s(0, 0, 2);
    c.swc1(1, r32(c, V0));
    c.sub_s(0, 8, 0);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.mul_s(0, 0, 4);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.div(0, 2, 5);
    c.g[A0][0] = c.hi[0];
    let idx = c.g[A0][0];
    Addr(hist + 0x3c).sw(c, idx);
    c.g[V1][0] = Addr(me.wrapping_add(0x30)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x434)).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    c.add_s(2, 0, 5);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.sqrt_s(0, 2);
        c.sqrt_s(2, 0);
    }

    // heading = heading * rate + (velocity / speed) * (1 - rate).
    c.cycles += 32;
    c.lwc1(0, info.wrapping_add(0x44));
    c.sub_s(1, 8, 2);
    c.div_s(0, 8, 0);
    c.mfc1(3, 0);
    c.lqc2(4, info.wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x30);
    c.lq(2, sp0 + 0x30);
    c.sq(2, sp0 + 0x20);
    c.mfc1(3, 2);
    c.lqc2(4, me.wrapping_add(0x80));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x60);
    c.lq(2, sp0 + 0x60);
    c.sq(2, sp0 + 0x50);
    c.mfc1(3, 1);
    c.lqc2(4, sp0 + 0x20);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x70);
    c.lq(2, sp0 + 0x70);
    c.sq(2, sp0 + 0x60);
    c.lqc2(3, sp0 + 0x50);
    c.lqc2(4, sp0 + 0x60);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp0 + 0x70);
    c.lq(2, sp0 + 0x70);
    c.sq(2, me.wrapping_add(0x80));
    c.sq(2, sp0 + 0x40);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};

    /// A float in [0, 2 * half).
    fn pos(r: &mut Rng, half: f32) -> u32 {
        (f32::from_bits(r.f32(half)) + half).to_bits()
    }

    // --- SSXAudioSystem_UpdateMusicIntensity -------------------------------

    const SYS: u32 = DATA;
    const RIDER: u32 = DATA + 0x8000;
    const RIDER_STATE: u32 = DATA + 0x9000;
    const AIR: u32 = DATA + 0x9800;
    const WORLD: u32 = DATA + 0xa000;
    const GAME: u32 = DATA + 0xa100;
    const REPLAY: u32 = DATA + 0xa200;
    const RACE: u32 = DATA + 0xa300;
    const RACE_REPLAY: u32 = DATA + 0xa400;
    const SONG: u32 = DATA + 0xb000;
    const SONG_VT: u32 = DATA + 0xb100;
    const AMB_OWNER: u32 = DATA + 0xc000;
    const AMB_VT: u32 = DATA + 0xc100;
    const MON_VT: u32 = DATA + 0xc200;
    const BELIB: u32 = DATA + 0xd000;
    const BE_STATE: u32 = DATA + 0xd100;
    const BE_VT: u32 = DATA + 0xd200;

    /// The audio system, the local rider in or out of a jump (FindRider
    /// through the race manager, or the replay rider), the air predictor's
    /// heights, the music monitor (song present or not, ducking, playing
    /// state), the track id the race-music checks read (through the
    /// BELibrary state vtable, a fake), and the monitor/ambience vtables
    /// (fakes).
    fn music_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x28c8c8, 4);
        let gp = GAME_GP;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        // World -> game module -> race manager -> riders.
        w(c, gp - 0x848, WORLD);
        w(c, WORLD + 0x84, GAME);
        let replay = seed % 7 == 0;
        w(c, GAME + 0x28, if replay { REPLAY } else { 0 });
        w(c, REPLAY, 1 + seed % 9);
        w(c, GAME + 0xc, RACE);
        w(c, RACE + 0x7c, 1);
        w(c, RACE + 0x28, RIDER);
        w(c, RACE + 0x40, RACE_REPLAY);
        w(c, RACE_REPLAY + 0x18, RIDER);
        w(c, RIDER + 0x874, 0);
        // Rider: playing state (1 = racing) and air predictor.
        w(c, RIDER + 0x77c, RIDER_STATE);
        w(c, RIDER_STATE + 0xde0, if seed % 11 == 0 { 0 } else { 1 });
        w(c, RIDER + 0x788, AIR);
        w(c, AIR + 0xac, [1, 3, 0, 2, 1][(seed / 2 % 5) as usize]);
        let speed = pos(&mut r, 3.0);
        w(c, AIR + 0x98, speed);
        w(c, AIR + 0x9c, pos(&mut r, 8.0));
        w(c, AIR + 0xa0, pos(&mut r, 4.0));
        // Audio system state.
        w(c, SYS + 0x6250, (r.next_u32() % 3).wrapping_sub(1));
        w(c, SYS + 0x6244, pos(&mut r, 3.0));
        w(c, SYS + 0x6248, pos(&mut r, 4.0));
        w(c, SYS + 0x624c, pos(&mut r, 6.0));
        w(c, SYS + 0x62d8, pos(&mut r, 0.5)); // music channel gain
        w(c, SYS + 0x6318, pos(&mut r, 0.5)); // ambience channel gain
        w(c, SYS + 0x5830, r.next_u32());
        w(c, SYS + 0x5834, 1); // ambience loop already running: short path
        w(c, SYS + 0x623c, (seed / 3 % 4 == 0) as u32); // music trigger
        w(c, SYS + 0x608c, seed / 5 % 5); // game-music state
        w(c, gp + 0x52c, (seed % 3 != 0) as u32);
        w(c, gp + 0x530, r.next_u32() % 2);
        w(c, gp - 0x45d4, 3.999_498_6_f32.to_bits());
        w(c, gp - 0x45d0, 0.842_519_7_f32.to_bits());
        w(c, gp - 0x45cc, (1.0f32 / 127.0).to_bits());
        // Music monitor: song (or none), ducking flag.
        let mon = SYS + 0x118;
        let song = if seed % 13 == 0 { 0 } else { SONG };
        w(c, mon, AMB_OWNER);
        w(c, mon + 0x408, song);
        w(c, mon + 0x418, (seed % 4 == 1) as u32);
        w(c, SONG, SONG_VT);
        w(c, SONG_VT + 0x94, (seed % 8 == 1) as u32);
        w(c, SONG + 0x38, [2, 2, 0][(seed / 4 % 3) as usize]);
        w(c, SONG + 0x4c, (seed / 6 % 2) as u32);
        w(c, SONG + 0x54, 0);
        // Vtables of the stop-ambience owner and the monitor: fakes.
        w(c, AMB_OWNER + 0xab0, AMB_VT);
        c.bus.write16(AMB_VT + 0x18, 0x10);
        w(c, AMB_VT + 0x1c, FAKE);
        w(c, SYS + 0x5558, MON_VT);
        c.bus.write16(MON_VT + 0x18, 0x20);
        w(c, MON_VT + 0x1c, FAKE + 0x10);
        // BELibrary: current state's track (race-music checks).
        w(c, gp - 0x1f04, BELIB);
        w(c, gp - 0x1efc, 0);
        w(c, 0x0053_08a8, BE_STATE);
        w(c, BE_STATE + 4, BE_VT);
        c.bus.write16(BE_VT + 8, 0);
        w(c, BE_VT + 0xc, FAKE + 0x20);
        w(c, 0x0053_5c08, 0);
        w(c, 0x0043_d950, [0, 9, 12, 3][(seed / 7 % 4) as usize]);
        c.bus
            .write8(0x0053_5c10, [0, 2, 3, 0, 0][(seed / 9 % 5) as usize]);
        c.g[A0][0] = SYS as u64;
        c.pc = 0x0028_c8c8;
    }

    #[test]
    fn update_music_intensity_matches() {
        covered(
            ee::s07::f_0028c8c8__generated,
            ssx_audio_system_update_music_intensity,
            300,
            &|c, seed| music_fixture(c, seed),
        );
    }

    // --- ChaseCameraAlgorithm_GatherRiderState ----------------------------

    const CAM: u32 = DATA;
    const CAM_OUT: u32 = DATA + 0x1000;
    const CAM_TARGET: u32 = DATA + 0x2000;
    const CAM_TARGET_VT: u32 = DATA + 0x2100;
    const CAM_RIDER: u32 = DATA + 0x3000;
    const CAM_RIDER_STATE: u32 = DATA + 0x4000;

    /// A chase camera whose target (a fake vtable: velocity, state,
    /// parameters, frame) is a rider in one of the modes; velocities around
    /// the 27.8 m/s threshold, thresholds that put |dir.z| below, between
    /// and above them.
    fn gather_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::SP;
        let mut r = machine(c, seed, 0x162568, 4);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let vec = |c: &mut Cpu, r: &mut Rng, a: u32, len: f32| {
            for i in 0..4 {
                c.bus.write32(a + 4 * i, r.f32(len));
            }
        };
        w(c, CAM + 0x30, CAM_TARGET);
        w(c, CAM_TARGET, CAM_TARGET_VT);
        for (i, slot) in [0x18u32, 0xa0, 0x88, 0x90, 0x8].into_iter().enumerate() {
            c.bus.write16(CAM_TARGET_VT + slot, (i as u16) * 0x10);
            w(c, CAM_TARGET_VT + slot + 4, FAKE + 0x10 * i as u32);
        }
        // The velocity the first call "returns" in its result slot (sp+0).
        let len = if seed % 3 == 0 {
            0.0
        } else if seed % 2 == 0 {
            20.0
        } else {
            60.0
        };
        vec(c, &mut r, SP - 0x110, len);
        vec(c, &mut r, CAM + 0x1b0, 40.0);
        vec(c, &mut r, CAM + 0x100, 40.0);
        vec(c, &mut r, CAM + 0x40, 40.0);
        crate::difftest::put_words(c, 0x004f_f120, &[0, 0, 0x3f80_0000, 0]);
        w(c, GAME_GP - 0x6afc, 27.777_779_f32.to_bits());
        w(c, GAME_GP - 0x6af8, 27.777_779_f32.to_bits());
        // Rider mode (0x11fe98) and target state (fake call 1).
        w(c, CAM_TARGET + 4, CAM_RIDER);
        w(c, CAM_RIDER + 0x77c, CAM_RIDER_STATE);
        w(
            c,
            CAM_RIDER_STATE + 0xde0,
            [1, 2, 4, 5, 0, 1][(seed / 2 % 6) as usize],
        );
        w(
            c,
            LOG_RESULTS + 4,
            [2, 3, 13, 5, 1][(seed / 5 % 5) as usize],
        );
        // a..h: lo/hi pairs and |z| thresholds.
        for (i, (base, span)) in [
            (-5.0f32, 10.0f32),
            (-5.0, 10.0),
            (0.0, 0.6),
            (0.5, 0.6),
            (-5.0, 10.0),
            (-5.0, 10.0),
            (0.0, 0.6),
            (0.5, 0.6),
        ]
        .into_iter()
        .enumerate()
        {
            let u = (r.next_u32() as f32 / u32::MAX as f32) * span + base;
            c.f[12 + i] = u.to_bits();
        }
        c.g[A0][0] = CAM as u64;
        c.g[A1][0] = CAM_OUT as u64;
        c.pc = 0x0016_2568;
    }

    #[test]
    fn gather_rider_state_matches() {
        covered(
            ee::s01::f_00162568__generated,
            chase_camera_algorithm_gather_rider_state,
            300,
            &|c, seed| gather_fixture(c, seed),
        );
    }

    // --- PATH_UpdateTracks -------------------------------------------------

    /// The PATH player in normal or streamed mode with a mix of empty, busy,
    /// inactive, fading, stopping and playing tracks. Platform handles: 0 has
    /// no stream (the fades' platform calls take their short path), 1..3 are
    /// streams that are stopped / playing. Every playing track still has
    /// enough queued (deadline or lead), so PATH_TrackAdvance (a large tree
    /// of its own) is not entered.
    fn path_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x003d_4f10 ^ (seed as u64) << 12);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let state = DATA + 0x100;
        let streamed = seed % 2 == 0;
        c.bus
            .write8(0x0044_f434, if streamed { b'B' } else { b'A' });
        w(c, 0x0044_f420, state);
        let time = 0x1000 + r.next_u32() % 0x1_0000;
        w(c, 0x0044_f438, time);
        let ticks = [r.next_u32() % 0x1000, r.next_u32() % 0x1000];
        w(c, state + 0xc, ticks[0]);
        w(c, state + 0x10, ticks[1]);
        let step = ticks[streamed as usize];
        w(c, state + 0x14, r.next_u32());
        w(c, state + 0x18, r.next_u32());
        w(c, state + 0x91c, 0); // no queues to service
        w(c, GAME_GP + 0x410, DATA + 0xf000); // audio system: no PATH streams
                                              // Platform stream table: 0 none, 1 stopped, 2/3 playing.
        let table = 0x004d_3e98;
        w(c, table, 0);
        for h in 1..4u32 {
            let p = DATA + 0xe000 + 0x100 * h;
            w(c, table + 4 * h, p);
            c.bus.write8(p + 0x64, 1);
            c.bus.write8(p + 0x65, 0);
            w(c, p + 0x58, [0, 0, 1, 7][h as usize]);
            w(c, p + 0xc, 1);
        }
        for i in 0..24u32 {
            let t = DATA + 0x2000 + 0x100 * i;
            let used = r.next_u32() % 5 != 0;
            w(c, state + 0x58 + 4 * i, if used { t } else { 0 });
            crate::difftest::fill(c, &mut r, t, 0x40);
            c.bus.write8(t + 0x3, (r.next_u32() % 6 == 0) as u8);
            c.bus
                .write16(t + 0x36, if r.next_u32() % 4 == 0 { 0xffff } else { 3 });
            w(c, t + 0x30, (r.next_u32() % 5 != 0) as u32);
            let mut fading = false;
            for f in [0xd, 0x15, 0x1d] {
                let on = r.next_u32() % 4 == 0;
                fading |= on;
                c.bus.write8(t + f, if on { 0 } else { 0xff });
            }
            let mine = (r.next_u32() % 4 != 0) == streamed;
            c.bus.write8(t + 0x2f, mine as u8);
            let handle = if fading { 0 } else { 1 + r.next_u32() % 3 };
            w(c, t + 0x54, handle);
            let stop = if handle != 0 && r.next_u32() % 3 == 0 {
                time + r.next_u32() % (step + 2)
            } else {
                0
            };
            w(c, t + 0x4c, stop);
            w(c, t + 0x20, 0); // nothing queued
            let lead = (r.next_u32() % 0x200) as i32 - 0x100;
            let deadline = if lead < 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                (time as i32 + lead) as u32 + step + 1 + r.next_u32() % 0x100
            };
            w(c, t + 0x24, lead as u32);
            w(c, t + 0x48, deadline);
        }
        c.pc = 0x003d_4f10;
    }

    #[test]
    fn path_update_tracks_matches() {
        crate::difftest::check(ee::s14::f_003d4f10__generated, path_update_tracks, 200, &|c, seed| {
            path_fixture(c, seed)
        });
    }
}
