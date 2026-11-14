//! Staging module oc371 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc371`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0012_f730,
        rider_natural_air_sub_handle_input,
        "RiderNaturalAirSub_HandleInput",
    ),
    (0x003d_dfa8, eafile_service_queue, "EAFile_ServiceQueue"),
    (0x003b_a0b0, snd_alloc_voices, "SND_AllocVoices"),
    (0x003b_b140, snd_play_patch_voices, "SND_PlayPatchVoices"),
    (
        0x0029_06b8,
        audio_play_pushed_sound,
        "Audio_PlayPushedSound",
    ),
    (0x003e_03d0, streamfile_issue_read, "StreamFile_IssueRead"),
    (0x003b_b588, snd_play_patch, "SND_PlayPatch"),
    (
        0x0029_4170,
        ssxaudio_play_takeoff_sound,
        "SSXAudioSystem_PlayTakeoffSound",
    ),
    (
        0x003a_7238,
        cstreamman_parse_chunks,
        "cStreamMan_ParseChunks",
    ),
    (
        0x0029_fcc8,
        ssxaudio_rider_say_2083,
        "SSXAudioSystem_RiderSay2083",
    ),
];

// ---------------------------------------------------------------------------
// RiderNaturalAirSub_HandleInput
// ---------------------------------------------------------------------------

/// `RiderNaturalAirSub` fields: `+0` steer heading, `+4` steer timer,
/// `+8` tweak flag, `+0xc` hold, `+0x10` held stick id, `+0x14` rider.
pub(crate) const NAIR_HEADING: u32 = 0x0;
pub(crate) const NAIR_TIMER: u32 = 0x4;
pub(crate) const NAIR_TWEAK: u32 = 0x8;
pub(crate) const NAIR_HOLD: u32 = 0xc;
pub(crate) const NAIR_ID: u32 = 0x10;
pub(crate) const NAIR_RIDER: u32 = 0x14;
/// Rider fields the sub state writes / reads (steer targets and rates).
pub(crate) const NAIR_RIDER_ANIM: u32 = 0x784;

const NAIR_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::f_00116120,
};
const NAIR_UPDATE_TWEAK_LAYER: Callee = Callee {
    addr: 0x0011_63b0,
    f: ee::s00::f_001163b0,
};
const NAIR_TRY_HANDPLANT: Callee = Callee {
    addr: 0x0010_7578,
    f: ee::s00::f_00107578,
};
const NAIR_TRY_RAILSLIDE: Callee = Callee {
    addr: 0x0010_6848,
    f: ee::s00::f_00106848,
};
const NAIR_HANDLE_BOOST: Callee = Callee {
    addr: 0x0011_4130,
    f: ee::s00::f_00114130,
};
const NAIR_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::f_0011fec8,
};
/// `Math_Atan(f12)` (through the table).
const NAIR_MATH_ATAN: u32 = 0x0031_c228;
/// `AnimController_GetLayerAnimId(ctl, layer)`.
const NAIR_ANIM_LAYER_ID: u32 = 0x0031_2aa0;
/// `AnimController_GetLayerAnimEntry(ctl, layer)`.
const NAIR_ANIM_LAYER_ENTRY: u32 = 0x0031_1ae8;
/// `AnimController_PlayAnim(ctl, anim, flags, f12 blend)`.
const NAIR_ANIM_PLAY: u32 = 0x0031_28e8;

/// `void RiderNaturalAirSub_HandleInput(RiderNaturalAirSub* self, const
/// Input* in)` (0x12f730): the natural-air rider sub state's per-tick
/// input. Resets / tweak layer / handplant / railslide / boost checks (any
/// that starts something ends the tick), then the stick (6-bit x, y) turns
/// into the rider's steer targets: a held stick steers until its timer or
/// direction runs out. Finally picks the idle air animation (0x11f, then
/// sub state 5) or the lean animations 9 / 10.
pub fn rider_natural_air_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let f = |c: &Cpu, n: usize| FpuF32(c.f[n]);
        let gp = c.g[GP][0] as u32;
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0x48);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        let me = Addr(c.g[S0][0] as u32);
        let input = Addr(c.g[S1][0] as u32);
        let word = input.lw(c);
        c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
        c.g[A1][0] = sx((word as u32 as i32 >> 12) as u32) & 1;
        NAIR_CHECK_RESET_PATH.call(c, 0x0012_f770)?;
        'out: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            // Tweak layer: with the flag, from the stick buttons (0x4000 /
            // 0x8000); without, released.
            c.cycles += 3;
            let tweak = me.off(NAIR_TWEAK).lw(c);
            c.g[V0][0] = tweak;
            c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
            if tweak == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                NAIR_UPDATE_TWEAK_LAYER.call(c, 0x0012_f7c8)?;
            } else {
                c.cycles += 12;
                let w = input.lw(c);
                c.g[V0][0] = (w & 0xc000 != 0) as u64;
                let flag = c.g[V0][0];
                me.off(NAIR_TWEAK).sw(c, flag);
                let w = input.lw(c) as u32 as i32;
                c.g[A2][0] = sx((w >> 15) as u32) & 1;
                c.g[A1][0] = sx((w >> 14) as u32) & 1;
                NAIR_UPDATE_TWEAK_LAYER.call(c, 0x0012_f7ac)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x20);
                    break 'out;
                }
                c.cycles += 2;
            }
            let w = input.lw(c);
            c.g[A0][0] = w;
            // The stick: x from bits 24-29 of the first word, y from bits
            // 0-5 of the second, each a signed 6-bit value times 1/31.
            c.cycles += 14;
            let w2 = input.off(4).lw(c) as u32;
            c.g[V1][0] = sx(((w as u32) << 2) as i32 as u32);
            let scale = Addr(gp.wrapping_sub(0x7704)).read_fpu(c);
            c.f[2] = scale.0;
            c.g[V0][0] = sx(((w2 << 26) as i32 >> 26) as u32);
            c.g[V1][0] = sx((((w as u32) << 2) as i32 >> 26) as u32);
            c.f[0] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
            c.f[1] = FpuF32::from_i32(c.g[V0][0] as u32 as i32).0;
            c.g[A0][0] = w & 0x2000;
            c.f[22] = (f(c, 0) * scale).0;
            c.f[21] = (f(c, 1) * scale).0;
            if w & 0x2000 != 0 {
                // Handplant button: up on the stick picks the variant.
                c.cycles += 6;
                c.g[AT][0] = 0x3f00_0000;
                c.f[0] = 0x3f00_0000;
                let up = f(c, 0).lt(f(c, 21));
                abi::set_fcc(c, up);
                c.g[A1][0] = 1;
                if !up {
                    c.cycles += 1;
                    c.g[A1][0] = 0;
                }
                c.cycles += 4;
                c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
                NAIR_TRY_HANDPLANT.call(c, 0x0012_f828)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x20);
                    break 'out;
                }
                c.cycles += 2;
                c.g[AT][0] = 0x3f80_0000;
                c.f[21] = 0x3f80_0000;
            }
            c.cycles += 4;
            c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
            NAIR_TRY_RAILSLIDE.call(c, 0x0012_f840)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            c.cycles += 7;
            c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            NAIR_HANDLE_BOOST.call(c, 0x0012_f858)?;
            // The held stick id: a new hold takes the input's id; another
            // id (or none) ends the hold.
            let hold = me.off(NAIR_HOLD).lw(c);
            c.g[V0][0] = u64::MAX;
            let mut take_id_load = true;
            if hold != 0 {
                c.cycles += 3;
                let id = me.off(NAIR_ID).lw(c);
                c.g[V1][0] = id;
                if id != u64::MAX {
                    c.g[A0][0] = input.off(2).lb(c);
                    take_id_load = false;
                } else {
                    c.cycles += 2;
                    let v = input.off(2).lb(c);
                    c.g[V0][0] = v;
                    me.off(NAIR_ID).sw(c, v);
                }
            }
            if take_id_load {
                c.cycles += 1;
                c.g[A0][0] = input.off(2).lb(c);
            }
            c.cycles += 3;
            let id = me.off(NAIR_ID).lw(c);
            c.g[V0][0] = id;
            let stick_id = c.g[A0][0];
            let clear = if stick_id != id {
                me.off(NAIR_HOLD).sw(c, 0);
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                if stick_id != u64::MAX {
                    false
                } else {
                    c.cycles += 1;
                    me.off(NAIR_HOLD).sw(c, 0);
                    true
                }
            };
            if clear {
                c.cycles += 1;
            }
            let hold = me.off(NAIR_HOLD).lw(c);
            c.g[V0][0] = hold;
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            if hold == 0 {
                c.cycles += 3;
                let id = me.off(NAIR_ID).lw(c);
                c.g[V1][0] = id;
                if id != u64::MAX {
                    c.cycles += 3;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.f[0] = 0xbf80_0000;
                    let v = FpuF32(c.f[0]);
                    me.off(NAIR_TIMER).write_fpu(c, v);
                }
            }
            // The steer limit: the larger of |x| and y.
            c.cycles += 5;
            c.f[0] = f(c, 22).abs().0;
            let le = f(c, 21).le(f(c, 0));
            abi::set_fcc(c, le);
            c.f[23] = c.f[21];
            if le {
                c.cycles += 1;
                c.f[23] = c.f[0];
            }
            // The stick angle: atan2(y, x) by quadrant.
            c.cycles += 4;
            c.f[20] = 0;
            let x0 = f(c, 22).eq_s(f(c, 20));
            abi::set_fcc(c, x0);
            if !x0 {
                c.cycles += 7;
                c.f[12] = f(c, 21).div(f(c, 22)).0;
                call_ptr(c, NAIR_MATH_ATAN, 0x0012_f91c)?;
                let neg = f(c, 22).lt(f(c, 20));
                abi::set_fcc(c, neg);
                c.f[1] = c.f[0];
                if neg {
                    c.cycles += 3;
                    let up = f(c, 20).lt(f(c, 21));
                    abi::set_fcc(c, up);
                    if up {
                        c.cycles += 3;
                        c.f[0] = Addr(gp.wrapping_sub(0x76f8)).lw(c) as u32;
                        c.f[1] = (f(c, 1) + f(c, 0)).0;
                    } else {
                        c.cycles += 2;
                        c.f[0] = Addr(gp.wrapping_sub(0x76f4)).lw(c) as u32;
                        c.f[1] = (f(c, 1) - f(c, 0)).0;
                    }
                }
                c.cycles += 1;
                c.f[2] = c.f[1];
                c.cycles += 1;
                c.f[0] = me.off(NAIR_TIMER).lw(c) as u32;
            } else {
                c.cycles += 4;
                let y0 = f(c, 21).eq_s(f(c, 20));
                abi::set_fcc(c, y0);
                c.f[2] = c.f[21];
                if y0 {
                    c.cycles += 1;
                    c.f[0] = me.off(NAIR_TIMER).lw(c) as u32;
                } else {
                    c.cycles += 4;
                    let up = f(c, 20).le(f(c, 21));
                    abi::set_fcc(c, up);
                    c.f[2] = Addr(gp.wrapping_sub(0x7700)).lw(c) as u32;
                    c.f[0] = me.off(NAIR_TIMER).lw(c) as u32;
                    if up {
                        c.cycles += 3;
                        c.f[2] = Addr(gp.wrapping_sub(0x76fc)).lw(c) as u32;
                    }
                }
            }
            // Keep steering while the timer runs, the stick is past the
            // timer's limit and points within 90 degrees of the heading.
            c.cycles += 4;
            c.f[1] = 0;
            let mut reset = f(c, 0).lt(f(c, 1));
            abi::set_fcc(c, reset);
            if !reset {
                c.cycles += 7;
                c.g[AT][0] = 0x3f00_0000;
                c.f[4] = 0x3f00_0000;
                c.f[0] = (f(c, 0) - f(c, 4)).max(f(c, 1)).0;
                reset = f(c, 23).le(f(c, 0));
                abi::set_fcc(c, reset);
                if !reset {
                    c.cycles += 10;
                    c.f[1] = me.off(NAIR_HEADING).lw(c) as u32;
                    c.f[0] = Addr(gp.wrapping_sub(0x76f0)).lw(c) as u32;
                    c.f[3] = (f(c, 2) - f(c, 1)).0;
                    c.f[0] = (f(c, 3) * f(c, 0) + f(c, 4)).0;
                    c.f[2] = f(c, 0).to_i32() as u32;
                    c.f[2] = FpuF32::from_i32(c.f[2] as i32).0;
                    let below = f(c, 0).lt(f(c, 2));
                    abi::set_fcc(c, below);
                    if below {
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f80_0000;
                        c.f[0] = 0x3f80_0000;
                        c.f[2] = (f(c, 2) - f(c, 0)).0;
                    }
                    c.cycles += 9;
                    c.f[0] = Addr(gp.wrapping_sub(0x76ec)).lw(c) as u32;
                    c.f[1] = Addr(gp.wrapping_sub(0x76e8)).lw(c) as u32;
                    c.f[0] = (f(c, 3) - f(c, 2) * f(c, 0)).abs().0;
                    reset = f(c, 1).lt(f(c, 0));
                    abi::set_fcc(c, reset);
                }
            }
            if reset {
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[0] = 0xbf80_0000;
                c.f[21] = 0;
                let v = FpuF32(c.f[0]);
                me.off(NAIR_TIMER).write_fpu(c, v);
                c.f[22] = c.f[21];
            }
            c.g[V1][0] = me.off(NAIR_RIDER).lw(c);
            // The rider's steer targets (x, y) and rates.
            c.cycles += 24;
            c.g[A1][0] = 2;
            let rate = Addr(gp.wrapping_sub(0x76e4)).read_fpu(c);
            c.f[1] = rate.0;
            let rider = Addr(c.g[V1][0] as u32);
            let v = FpuF32(c.f[22]);
            rider.off(0x1f8).write_fpu(c, v);
            rider.off(0x1f4).write_fpu(c, rate);
            let rider = Addr(me.off(NAIR_RIDER).lw(c) as u32);
            c.g[V0][0] = rider.0 as i32 as u64;
            c.f[0] = rider.off(0x1f0).lw(c) as u32;
            rider.off(0x200).write_fpu(c, rate);
            let v = FpuF32(c.f[0]);
            rider.off(0x204).write_fpu(c, v);
            let rider = Addr(me.off(NAIR_RIDER).lw(c) as u32);
            c.g[V1][0] = rider.0 as i32 as u64;
            rider.off(0x224).write_fpu(c, rate);
            let v = FpuF32(c.f[21]);
            rider.off(0x228).write_fpu(c, v);
            let rider = Addr(me.off(NAIR_RIDER).lw(c) as u32);
            c.g[A0][0] = rider.0 as i32 as u64;
            rider.off(0x218).write_fpu(c, rate);
            rider.off(0x21c).sw(c, 0);
            let rider = me.off(NAIR_RIDER).lw(c);
            c.g[V0][0] = rider;
            c.g[A0][0] = Addr(rider as u32).off(NAIR_RIDER_ANIM).lw(c);
            call_ptr(c, NAIR_ANIM_LAYER_ID, 0x0012_fa34)?;
            // Not steering at all: the idle air animation, then sub state 5.
            let rider = Addr(me.off(NAIR_RIDER).lw(c) as u32);
            c.g[V1][0] = rider.0 as i32 as u64;
            c.f[1] = 0;
            c.f[0] = rider.off(0x220).lw(c) as u32;
            let mut idle = f(c, 0).eq_s(f(c, 1));
            abi::set_fcc(c, idle);
            c.g[S1][0] = c.g[V0][0];
            if idle {
                c.cycles += 5;
                c.f[0] = rider.off(0x214).lw(c) as u32;
                idle = f(c, 0).eq_s(f(c, 1));
                abi::set_fcc(c, idle);
                if idle {
                    c.cycles += 5;
                    c.f[0] = rider.off(0x1f0).lw(c) as u32;
                    idle = f(c, 0).eq_s(f(c, 1));
                    abi::set_fcc(c, idle);
                    if idle {
                        c.cycles += 3;
                        let hold = me.off(NAIR_HOLD).lw(c);
                        c.g[V0][0] = hold;
                        idle = hold == 0;
                    }
                }
                if !idle {
                    c.g[V0][0] = me.off(NAIR_RIDER).lw(c);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = me.off(NAIR_RIDER).lw(c);
            }
            if idle {
                c.cycles += 3;
                c.g[V0][0] = 0x11f;
                if c.g[S1][0] != 0x11f {
                    c.cycles += 6;
                    c.g[A0][0] = rider.off(NAIR_RIDER_ANIM).lw(c);
                    c.g[A1][0] = 2;
                    call_ptr(c, NAIR_ANIM_LAYER_ENTRY, 0x0012_fa9c)?;
                    c.g[V1][0] = 9;
                    let landing = c.g[V0][0] == 9;
                    c.g[A1][0] = 0x11f;
                    if !landing {
                        c.cycles += 6;
                        let rider = me.off(NAIR_RIDER).lw(c);
                        c.g[V0][0] = rider;
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.f[12] = 0xbf80_0000;
                        c.g[A2][0] = 0;
                        c.g[A0][0] = Addr(rider as u32).off(NAIR_RIDER_ANIM).lw(c);
                        call_ptr(c, NAIR_ANIM_PLAY, 0x0012_fac0)?;
                    }
                    c.cycles += 1;
                }
                c.g[A0][0] = me.off(NAIR_RIDER).lw(c);
                c.cycles += 4;
                c.g[A1][0] = 5;
                NAIR_SET_SUB_STATE.call(c, 0x0012_facc)?;
                frame.restore(c, S0, 0x20);
                break 'out;
            }
            // Leaning: animation 9 when the y target is small for the
            // turn, else 10 (unless layer 2 is landing, entry 9).
            c.cycles += 6;
            c.g[A1][0] = 2;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(NAIR_RIDER_ANIM).lw(c);
            call_ptr(c, NAIR_ANIM_LAYER_ENTRY, 0x0012_fae4)?;
            c.g[V1][0] = 9;
            let landing = c.g[V0][0] == 9;
            c.g[A1][0] = 0xa;
            if !landing {
                c.cycles += 14;
                let rider = Addr(me.off(NAIR_RIDER).lw(c) as u32);
                c.g[A0][0] = rider.0 as i32 as u64;
                c.g[AT][0] = 0x3f80_0000;
                c.f[1] = 0x3f80_0000;
                c.f[0] = rider.off(0x1f0).read_fpu(c).abs().0;
                c.g[AT][0] = 0x3f00_0000;
                c.f[3] = 0x3f00_0000;
                c.f[1] = (f(c, 1) - f(c, 0)).0;
                c.f[2] = rider.off(0x220).lw(c) as u32;
                c.f[1] = (f(c, 1) * f(c, 3)).0;
                let small = f(c, 2).le(f(c, 1));
                abi::set_fcc(c, small);
                if small {
                    c.g[A1][0] = 9;
                }
                c.cycles += 2;
                let same = c.g[A1][0] == c.g[S1][0];
                frame.restore(c, S0, 0x20);
                if same {
                    break 'out;
                }
                c.cycles += 5;
                c.g[A0][0] = rider.off(NAIR_RIDER_ANIM).lw(c);
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, NAIR_ANIM_PLAY, 0x0012_fb44)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_ServiceQueue
// ---------------------------------------------------------------------------

/// The EA file system state (`0x519ab0`): `+0x8` highest priority served,
/// `+0xc` queued count, `+0x10` paused, `+0x14` mutex, `+0x24` the request
/// being served, `+0x30` the queue head, `+0x34` the open big files.
pub(crate) const EAFS_STATE: u32 = 0x0051_9ab0;
/// The command jump table (`.rodata`, commands 2..10).
pub(crate) const EAFS_JUMP_TABLE: u32 = 0x0049_5d80;
/// Request fields: `+0` command (bits 20-23), `+4` cancelled, `+0xc`
/// result, `+0x10` priority, `+0x18` flags / offset / result, `+0x1c`
/// size, `+0x20` buffer, `+0x24` file handle, `+0x2c` next.
pub(crate) const EAFS_REQ_PRIORITY: u32 = 0x10;
pub(crate) const EAFS_REQ_NEXT: u32 = 0x2c;
pub(crate) const EAFS_REQ_HANDLE: u32 = 0x24;

const EAFS_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::f_003e5700,
};
const EAFS_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::f_003e5760,
};
const EAFS_COMPLETE_REQUEST: Callee = Callee {
    addr: 0x003d_e420,
    f: ee::s14::f_003de420,
};
const EAFS_PLATFORM_OPEN: Callee = Callee {
    addr: 0x003e_3208,
    f: ee::s14::f_003e3208,
};
const EAFS_PLATFORM_CLOSE: Callee = Callee {
    addr: 0x003e_32f8,
    f: ee::s14::f_003e32f8,
};
const EAFS_PLATFORM_GET_SIZE: Callee = Callee {
    addr: 0x003e_34e8,
    f: ee::s14::f_003e34e8,
};
const EAFS_PLATFORM_GET_LSN: Callee = Callee {
    addr: 0x003e_3538,
    f: ee::s14::f_003e3538,
};
const EAFS_BIG_FIND_ENTRY: Callee = Callee {
    addr: 0x003e_2768,
    f: ee::s14::f_003e2768,
};
const EAFS_FREE_FILE_HANDLE: Callee = Callee {
    addr: 0x003d_e8c0,
    f: ee::s14::f_003de8c0,
};
const EAFS_PLATFORM_READ: Callee = Callee {
    addr: 0x003e_3350,
    f: ee::s14::f_003e3350,
};
const EAFS_PLATFORM_WRITE: Callee = Callee {
    addr: 0x003e_3478,
    f: ee::s14::f_003e3478,
};
/// libc through the table.
const EAFS_STRCHR: u32 = 0x0041_6404;
const EAFS_STRNCPY: u32 = 0x0041_6ce0;
const EAFS_STRCPY: u32 = 0x0041_66f4;
const EAFS_STRCMP: u32 = 0x0041_65a8;

/// `void EAFile_ServiceQueue(Request* req)` (0x3ddfa8, `iFILESYS_ExecCommand`):
/// queues `req` (if any) by priority, then, when nothing is being served
/// and the queue is not paused, takes the head (if its priority is allowed)
/// and executes it: open / exists (2, 8: a plain path, or `big|name` /
/// `|name` looked up in the open big files), close (3), read (4), write
/// (5), size (6), the others just complete.
pub fn eafile_service_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x270);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x260);
        frame.save(c, S1, 0x250);
        c.g[S0][0] = sx(0x0051_9ac4);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x220);
        c.g[S4][0] = 0x52_0000;
        frame.save(c, S2, 0x240);
        frame.save(c, S3, 0x230);
        frame.save_ra(c, 0x210);
        c.g[A0][0] = c.g[S0][0];
        EAFS_MUTEX_LOCK.call(c, 0x003d_dfdc)?;
        let st = Addr(EAFS_STATE);
        c.g[A0][0] = 0;
        if c.g[S1][0] != 0 {
            // Insert behind every request of the same or higher priority.
            let req = Addr(c.g[S1][0] as u32);
            c.cycles += 3;
            let head = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
            c.g[V0][0] = head;
            req.off(EAFS_REQ_NEXT).sw(c, head);
            if head != 0 {
                c.cycles += 5;
                let hp = Addr(head as u32).off(EAFS_REQ_PRIORITY).lw(c);
                let prio = req.off(EAFS_REQ_PRIORITY).lw(c);
                c.g[V1][0] = prio;
                c.g[V0][0] = ((prio as i64) < (hp as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = req.off(EAFS_REQ_NEXT).lw(c);
                    loop {
                        c.cycles += 3;
                        let next = Addr(c.g[A0][0] as u32).off(EAFS_REQ_NEXT).lw(c);
                        c.g[V0][0] = next;
                        req.off(EAFS_REQ_NEXT).sw(c, next);
                        if next == 0 {
                            break;
                        }
                        c.cycles += 4;
                        let np = Addr(next as u32).off(EAFS_REQ_PRIORITY).lw(c);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (np as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[A0][0] = req.off(EAFS_REQ_NEXT).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_e008);
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx(EAFS_STATE);
            let prev = c.g[A0][0];
            if prev == 0 {
                c.cycles += 1;
                let v = c.g[S1][0];
                st.off(0x30).sw(c, v);
            } else {
                c.cycles += 2;
                let s1 = c.g[S1][0];
                Addr(prev as u32).off(EAFS_REQ_NEXT).sw(c, s1);
            }
            c.cycles += 4;
            c.g[V1][0] = sx(EAFS_STATE);
            let n = st.off(0xc).lw(c);
            c.g[V0][0] = sx((n as u32).wrapping_add(1));
            let v = c.g[V0][0];
            st.off(0xc).sw(c, v);
        }
        'out: {
            // Serve the head unless one is being served or the queue is paused.
            c.cycles += 4;
            c.g[A0][0] = sx(EAFS_STATE);
            let busy = st.off(0x24).lw(c);
            c.g[V0][0] = busy;
            let mut idle = false;
            if busy == 0 {
                c.cycles += 3;
                let paused = st.off(0x10).lw(c);
                c.g[V0][0] = paused;
                if paused == 0 {
                    c.g[S1][0] = st.off(0x30).lw(c);
                    idle = true;
                }
            }
            if !idle {
                c.cycles += 4;
                c.g[A0][0] = sx(EAFS_STATE + 0x14);
                EAFS_MUTEX_UNLOCK.call(c, 0x003d_e06c)?;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.g[S1][0] = 0;
            } else {
                c.cycles += 5;
                let req = Addr(c.g[S1][0] as u32);
                c.g[V1][0] = req.off(EAFS_REQ_PRIORITY).lw(c);
                let max = st.off(0x8).lw(c);
                c.g[V0][0] = ((max as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = 0;
                } else {
                    c.cycles += 5;
                    let n = st.off(0xc).lw(c);
                    c.g[V1][0] = req.off(EAFS_REQ_NEXT).lw(c);
                    c.g[V0][0] = sx((n as u32).wrapping_sub(1));
                    let v = c.g[V0][0];
                    st.off(0xc).sw(c, v);
                    let v = c.g[V1][0];
                    st.off(0x30).sw(c, v);
                }
            }
            c.cycles += 6;
            c.g[V0][0] = sx(EAFS_STATE);
            c.g[A0][0] = sx(EAFS_STATE + 0x14);
            let v = c.g[S1][0];
            st.off(0x24).sw(c, v);
            EAFS_MUTEX_UNLOCK.call(c, 0x003d_e0b4)?;
            frame.restore(c, S0, 0x260);
            if c.g[S1][0] == 0 {
                break 'out;
            }
            let req = Addr(c.g[S1][0] as u32);
            // A cancelled request completes with 0.
            c.cycles += 3;
            c.g[V0][0] = req.off(4).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = 0;
                EAFS_COMPLETE_REQUEST.call(c, 0x003d_e0d0)?;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.g[V0][0] = req.lw(c);
            c.cycles += 6;
            let cmd = (c.g[V0][0] as u32 >> 20) & 0xf;
            c.g[V0][0] = cmd as u64;
            c.g[A0][0] = sx(cmd.wrapping_sub(2));
            c.g[V1][0] = (c.g[A0][0] < 9) as u64;
            c.g[V0][0] = 0x49_0000;
            if c.g[V1][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx(EAFS_JUMP_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(EAFS_JUMP_TABLE));
            let target = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = target;
            match target as u32 {
                0x003d_e108 => eafile_open_command(c, frame)?,
                0x003d_e308 => {
                    // Close: the platform file unless it is inside a big file.
                    c.cycles += 4;
                    c.g[V0][0] = 1;
                    let fh = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A0][0] = fh;
                    req.off(0xc).sw(c, 1);
                    if fh != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(fh as u32).off(8).lw(c);
                        if c.g[V0][0] != 0 {
                            req.off(0xc).sw(c, 0);
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = Addr(fh as u32).lw(c);
                            EAFS_PLATFORM_CLOSE.call(c, 0x003d_e32c)?;
                            let v0 = c.g[V0][0];
                            req.off(0xc).sw(c, v0);
                        }
                        c.cycles += 3;
                        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                        EAFS_FREE_FILE_HANDLE.call(c, 0x003d_e338)?;
                        req.off(EAFS_REQ_HANDLE).sw(c, 0);
                    }
                    c.cycles += 5;
                    let res = req.off(0xc).lw(c);
                    c.g[A0][0] = (res < 1) as u64;
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e348)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e350 => {
                    // Read: from the file, or at the entry's offset inside
                    // its big file.
                    c.cycles += 3;
                    c.g[A3][0] = req.off(0x1c).lw(c);
                    if (c.g[A3][0] as i64) <= 0 {
                        c.cycles += 4;
                        c.g[A0][0] = 1;
                        EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3ac)?;
                    } else {
                        c.cycles += 4;
                        let fh = req.off(EAFS_REQ_HANDLE).lw(c);
                        c.g[V0][0] = fh;
                        c.g[A0][0] = Addr(fh as u32).off(8).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[A0][0] = Addr(fh as u32).lw(c);
                            c.cycles += 5;
                            c.g[A2][0] = req.off(0x18).lw(c);
                            c.g[A1][0] = req.off(0x20).lw(c);
                            EAFS_PLATFORM_READ.call(c, 0x003d_e39c)?;
                        } else {
                            c.cycles += 9;
                            let big = Addr(fh as u32).lw(c);
                            c.g[V0][0] = big;
                            c.g[A2][0] = req.off(0x18).lw(c);
                            c.g[V1][0] = Addr(big as u32).off(0xc).lw(c);
                            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[A1][0] = req.off(0x20).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            EAFS_PLATFORM_READ.call(c, 0x003d_e388)?;
                        }
                    }
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3b4 => {
                    c.cycles += 8;
                    c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A3][0] = req.off(0x1c).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = req.off(0x20).lw(c);
                    c.g[A2][0] = req.off(0x18).lw(c);
                    EAFS_PLATFORM_WRITE.call(c, 0x003d_e3cc)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3d4 => {
                    // Size: the handle's size into the request.
                    c.cycles += 7;
                    c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A0][0] = 1;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                    let v0 = c.g[V0][0];
                    req.off(0x18).sw(c, v0);
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3e8)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3f0 => {
                    c.cycles += 3;
                    let res = req.off(0xc).lw(c);
                    c.g[A0][0] = (res < 1) as u64;
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3fc)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x260);
                }
                t => {
                    c.jump_out(t, 0x003d_dfa8, 0x003d_e41c);
                    return Err(crate::call::Unwound);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x250);
        frame.restore(c, S2, 0x240);
        frame.restore(c, S3, 0x230);
        frame.restore(c, S4, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    })
}

/// The open / exists commands of [`eafile_service_queue`] (`s1` the
/// request): parse `big|name`, `|name` or a plain path, open the plain path
/// (exists: close it again; open: size and LSN), else look the name up in
/// the open big files; free the handle of an exists, complete.
fn eafile_open_command(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let req = Addr(c.g[S1][0] as u32);
    let sp = |c: &Cpu| c.g[SP][0] as u32;
    c.cycles += 7;
    let name_buf = sp(c).wrapping_add(0x100);
    Addr(name_buf).sb(c, 0);
    c.g[A1][0] = 0x7c;
    let fh = req.off(EAFS_REQ_HANDLE).lw(c);
    c.g[A0][0] = sx((fh as u32).wrapping_add(0x10));
    call_ptr(c, EAFS_STRCHR, 0x003d_e11c)?;
    let has_bar = c.g[V0][0] != 0;
    c.g[V1][0] = 0x7c;
    if !has_bar {
        // A plain path: open it if the flags ask (bit 0).
        c.cycles += 4;
        c.g[V0][0] = req.off(0x18).lw(c) & 1;
        c.g[S3][0] = 1;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[S3][0] = 3;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x100));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRCPY, 0x003d_e1c0)?;
        }
        c.cycles += 1;
        c.g[S2][0] = 0;
    } else {
        c.cycles += 4;
        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x10).lb(c);
        if c.g[V0][0] != c.g[V1][0] {
            // `big|name`: the big file's name into the stack buffer.
            c.g[A1][0] = 0x7c;
            c.cycles += 15;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRCHR, 0x003d_e144)?;
            c.g[S3][0] = 4;
            c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
            Addr(sp(c)).sb(c, 0);
            c.g[A0][0] = c.g[SP][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[A1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRNCPY, 0x003d_e16c)?;
            c.g[V1][0] = sx(sp(c).wrapping_add(c.g[S0][0] as u32));
            Addr(c.g[V1][0] as u32).sb(c, 0);
            c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        } else {
            // `|name`: any big file.
            c.cycles += 2;
            c.g[S3][0] = 2;
        }
        // The entry name after the bar.
        c.cycles += 9;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x100));
        c.g[A1][0] = 0x7c;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, EAFS_STRCHR, 0x003d_e188)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        call_ptr(c, EAFS_STRCPY, 0x003d_e194)?;
        c.g[S2][0] = 0;
    }
    // A plain path: the platform file.
    c.cycles += 3;
    c.g[V0][0] = c.g[S3][0] & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let fh = req.off(EAFS_REQ_HANDLE).lw(c);
        c.g[A1][0] = req.off(0x18).lw(c);
        c.g[A2][0] = fh;
        c.g[A0][0] = sx((fh as u32).wrapping_add(0x10));
        EAFS_PLATFORM_OPEN.call(c, 0x003d_e1e4)?;
        let opened = c.g[V0][0] != 0;
        c.g[V1][0] = 8;
        if opened {
            c.cycles += 5;
            let cmd = (req.lw(c) as u32 >> 20) & 0xf;
            c.g[V0][0] = cmd as u64;
            c.g[S2][0] = 1;
            if cmd == 8 {
                c.cycles += 5;
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_CLOSE.call(c, 0x003d_e20c)?;
            } else {
                c.cycles += 10;
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_GET_SIZE.call(c, 0x003d_e220)?;
                c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).off(4).sw(c, v0);
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_GET_LSN.call(c, 0x003d_e234)?;
                c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).off(0xc).sw(c, v0);
            }
        }
    }
    // Not opened: search the open big files (bit 1: any, bit 2: the named one).
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 6;
        let search = c.g[V0][0] != 0;
        c.g[V0][0] = sx(EAFS_STATE);
        let mut found_none = true;
        if search {
            c.cycles += 3;
            c.g[S0][0] = Addr(EAFS_STATE).off(0x34).lw(c);
            c.g[S3][0] &= 4;
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                loop {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[SP][0];
                    let mut try_it = true;
                    if c.g[S3][0] != 0 {
                        c.cycles += 5;
                        let big = Addr(c.g[S0][0] as u32);
                        c.g[A0][0] = big.off(0xc).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        call_ptr(c, EAFS_STRCMP, 0x003d_e274)?;
                        try_it = c.g[V0][0] == 0;
                    }
                    if try_it {
                        c.cycles += 8;
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        c.g[A1][0] = sx(sp(c).wrapping_add(0x100));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = sx(sp(c).wrapping_add(0x200));
                        c.g[T0][0] = sx(sp(c).wrapping_add(0x204));
                        EAFS_BIG_FIND_ENTRY.call(c, 0x003d_e294)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 10;
                            c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            c.g[S2][0] = 1;
                            let s0 = c.g[S0][0];
                            Addr(c.g[V0][0] as u32).sw(c, s0);
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0x204)).lw(c);
                            c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            let v0 = c.g[V0][0];
                            Addr(c.g[V1][0] as u32).off(4).sw(c, v0);
                            c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0x200)).lw(c);
                            let v0 = c.g[V0][0];
                            Addr(c.g[A0][0] as u32).off(8).sw(c, v0);
                        }
                    }
                    c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        found_none = false;
                        break;
                    }
                    c.cycles += 2;
                    if c.g[S2][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_e260);
                    }
                }
            }
        }
        if found_none {
            c.cycles += 1;
        }
    }
    c.g[V0][0] = req.lw(c);
    // An exists request frees its handle and keeps the answer.
    c.cycles += 5;
    c.g[V1][0] = 8;
    let cmd = (c.g[V0][0] as u32 >> 20) & 0xf;
    c.g[V0][0] = cmd as u64;
    if cmd == 8 {
        c.cycles += 3;
        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        EAFS_FREE_FILE_HANDLE.call(c, 0x003d_e2f4)?;
        let s2 = c.g[S2][0];
        req.off(0x18).sw(c, s2);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e300)?;
    frame.restore(c, S0, 0x260);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_AllocVoices
// ---------------------------------------------------------------------------

/// The voices picked by [`snd_alloc_voices`] (`s16[]`, 0x50ac20).
pub(crate) const SNDV_SELECTED: u32 = 0x0050_ac20;
/// The sound system (0x50a8e8): `+0x3c` steal-equal-priority flag, `+0x180`
/// the tick, `+0x1f0` the voice array (0x8c bytes each: `+0` handle, `+4`
/// linked voices (s16), `+0x28` master voice, `+0x2a` priority (u8), `+0x2c`
/// start tick, `+0x69` in use).
pub(crate) const SNDV_SYSTEM: u32 = 0x0050_a8e8;
/// The handle serial (`+= 0x100` per allocation, 0x44c478).
pub(crate) const SNDV_SERIAL: u32 = 0x0044_c478;
pub(crate) const SNDV_STRIDE: u32 = 0x8c;

const SNDV_IS_VOICE_RESERVED: Callee = Callee {
    addr: 0x003b_a078,
    f: ee::s13::f_003ba078,
};
const SNDV_STOP: Callee = Callee {
    addr: 0x003b_89e0,
    f: ee::s13::f_003b89e0,
};

/// `int SND_AllocVoices(int n, int priority, int* handle, int lo, int hi)`
/// (0x3ba0b0): picks `n` voices in `[lo, hi)`: free ones first (the
/// oldest), then steals the lowest-priority (and oldest) ones below
/// `priority` (or at it, with the system flag). With all `n` found, sorts
/// them, stops what the stolen ones played, marks them in use and links
/// them to the first, whose handle (serial | voice) goes to `*handle`.
/// Returns the first voice, or -9.
pub fn snd_alloc_voices(c: &mut Cpu) {
    guest(c, |c| {
        let sp = |c: &Cpu| c.g[SP][0] as u32;
        let add =
            |c: &mut Cpu, r: usize, k: u32| c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(k));
        let lt = |c: &Cpu, a: usize, b: usize| ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64;
        c.cycles += 20;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = sx(-9i32 as u32);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        c.g[S7][0] = 0;
        let a3 = c.g[A3][0];
        Addr(sp(c) + 8).sw(c, a3);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        c.g[A3][0] = c.g[T0][0];
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x50);
        let (a1, a2, v0) = (c.g[A1][0], c.g[A2][0], c.g[V0][0]);
        Addr(sp(c)).sw(c, a1);
        Addr(sp(c) + 4).sw(c, a2);
        Addr(sp(c) + 0xc).sw(c, v0);
        // Nothing selected yet.
        if (c.g[FP][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(SNDV_SELECTED);
            c.g[V1][0] = u64::MAX;
            c.g[S3][0] = c.g[FP][0];
            loop {
                c.cycles += 7;
                let v1 = c.g[V1][0];
                Addr(c.g[V0][0] as u32).sh(c, v1);
                add(c, S3, u32::MAX);
                add(c, V0, 2);
                if c.g[S3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_a110);
                }
            }
        }
        c.cycles += 5;
        c.g[V1][0] = 0x45_0000;
        let serial = Addr(SNDV_SERIAL).lw(c);
        c.g[V0][0] = sx((serial as u32).wrapping_add(0x100));
        let v0 = c.g[V0][0];
        Addr(SNDV_SERIAL).sw(c, v0);
        if (v0 as i64) < 0 {
            c.cycles += 1;
            Addr(SNDV_SERIAL).sw(c, 0);
        }
        // Pass 1: for each of the n, the oldest free unreserved voice.
        c.cycles += 2;
        c.g[S3][0] = 0;
        if (c.g[FP][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = sx(SNDV_SELECTED);
            c.g[V1][0] = sx((c.g[S7][0] as u32) << 1);
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(SNDV_SELECTED));
            c.g[S1][0] = Addr(sp(c) + 8).lw(c);
            loop {
                c.cycles += 7;
                c.g[S2][0] = u64::MAX;
                add(c, S3, 1);
                c.g[S6][0] = u64::MAX;
                let s3 = c.g[S3][0];
                Addr(sp(c) + 0x14).sw(c, s3);
                c.g[V0][0] = lt(c, S1, A3);
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = SNDV_STRIDE as u64;
                    c.g[V1][0] = 0x51_0000;
                    c.mult(0, S3, S1, V0);
                    c.g[S5][0] = sx(SNDV_SYSTEM);
                    loop {
                        c.cycles += 5;
                        let voices = Addr(c.g[S5][0] as u32).off(0x1f0).lw(c);
                        c.g[V0][0] = voices;
                        c.g[S0][0] = sx((voices as u32).wrapping_add(c.g[S3][0] as u32));
                        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x69).lb(c);
                        if c.g[V1][0] == 0 {
                            c.cycles += 6;
                            frame.save(c, A3, 0x30);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S7][0];
                            SNDV_IS_VOICE_RESERVED.call(c, 0x003b_a1b4)?;
                            let reserved = c.g[V0][0] != 0;
                            frame.restore(c, A3, 0x30);
                            if reserved {
                                c.cycles += 1;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x2c).lw(c);
                                c.g[V0][0] = (c.g[V1][0] < c.g[S2][0]) as u64;
                                if c.g[V0][0] != 0 {
                                    c.cycles += 2;
                                    c.g[S2][0] = c.g[V1][0];
                                    c.g[S6][0] = c.g[S1][0];
                                    c.cycles += 1;
                                }
                            }
                        }
                        add(c, S1, 1);
                        c.cycles += 3;
                        c.g[V0][0] = lt(c, S1, A3);
                        add(c, S3, SNDV_STRIDE);
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_a190);
                        }
                    }
                }
                c.cycles += 2;
                let found = (c.g[S6][0] as i64) >= 0;
                c.g[S3][0] = Addr(sp(c) + 0x14).lw(c);
                if found {
                    c.cycles += 4;
                    let s6 = c.g[S6][0];
                    Addr(c.g[S4][0] as u32).sh(c, s6);
                    add(c, S7, 1);
                    add(c, S4, 2);
                    c.g[S3][0] = Addr(sp(c) + 0x14).lw(c);
                }
                c.cycles += 3;
                c.g[V0][0] = lt(c, S3, FP);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[S1][0] = Addr(sp(c) + 8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003b_a160);
                }
            }
        }
        // Pass 2: steal the lowest priority below ours, oldest first.
        c.cycles += 3;
        c.g[V0][0] = lt(c, S7, FP);
        c.g[S3][0] = c.g[S7][0];
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = sx(SNDV_SELECTED);
            c.g[V1][0] = sx((c.g[S7][0] as u32) << 1);
            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(SNDV_SELECTED));
            let tag = Addr(sp(c)).lw(c);
            c.g[V0][0] = sx((tag as u32).wrapping_sub(1));
            let v0 = c.g[V0][0];
            Addr(sp(c) + 0x10).sw(c, v0);
            c.g[V1][0] = 0x51_0000;
            'pass2: loop {
                c.cycles += 11;
                c.g[S4][0] = Addr(sp(c) + 0x10).lw(c);
                c.g[V1][0] = sx(SNDV_SYSTEM);
                c.g[S1][0] = Addr(sp(c) + 8).lw(c);
                c.g[V0][0] = Addr(SNDV_SYSTEM).off(0x3c).lbu(c);
                c.g[S2][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = Addr(sp(c)).lw(c);
                c.g[S6][0] = u64::MAX;
                if c.g[V0][0] != 0 {
                    c.g[S4][0] = c.g[V1][0];
                }
                c.g[V0][0] = lt(c, S1, A3);
                c.g[S2][0] |= 0xffff;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = SNDV_STRIDE as u64;
                    c.g[V1][0] = 0x51_0000;
                    c.mult(0, S5, S1, V0);
                    c.g[T0][0] = sx(SNDV_SYSTEM);
                    loop {
                        c.cycles += 10;
                        frame.save(c, A2, 0x20);
                        c.g[A0][0] = c.g[S1][0];
                        frame.save(c, A3, 0x30);
                        c.g[A1][0] = c.g[S7][0];
                        frame.save(c, T0, 0x40);
                        SNDV_IS_VOICE_RESERVED.call(c, 0x003b_a290)?;
                        frame.restore(c, A2, 0x20);
                        frame.restore(c, A3, 0x30);
                        let reserved = c.g[V0][0] != 0;
                        frame.restore(c, T0, 0x40);
                        if reserved {
                            c.cycles += 1;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = Addr(c.g[T0][0] as u32).off(0x1f0).lw(c);
                            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            let prio = Addr(c.g[S0][0] as u32).off(0x2a).lbu(c);
                            c.g[V0][0] = (prio < 0x65) as u64;
                            if c.g[V0][0] != 0 {
                                c.cycles += 4;
                                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x2a).lbu(c);
                                c.g[V0][0] = lt(c, V1, S4);
                                let mut pick = false;
                                if c.g[V0][0] != 0 {
                                    // A lower priority.
                                    c.cycles += 3;
                                    c.g[S4][0] = c.g[V1][0];
                                    c.g[S2][0] = Addr(c.g[S0][0] as u32).off(0x2c).lw(c);
                                    pick = true;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[S4][0] {
                                        // The same: the older one.
                                        c.cycles += 4;
                                        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x2c).lw(c);
                                        c.g[V0][0] = (c.g[V1][0] < c.g[S2][0]) as u64;
                                        if c.g[V0][0] != 0 {
                                            c.cycles += 1;
                                            c.g[S2][0] = c.g[V1][0];
                                            pick = true;
                                        }
                                    }
                                }
                                if pick {
                                    c.cycles += 1;
                                    c.g[S6][0] = c.g[S1][0];
                                    c.cycles += 1;
                                }
                            }
                        }
                        add(c, S1, 1);
                        c.cycles += 3;
                        c.g[V0][0] = lt(c, S1, A3);
                        add(c, S5, SNDV_STRIDE);
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_a278);
                        }
                    }
                }
                c.cycles += 2;
                if (c.g[S6][0] as i64) < 0 {
                    add(c, S3, 1);
                } else {
                    c.cycles += 5;
                    let s6 = c.g[S6][0];
                    Addr(c.g[A2][0] as u32).sh(c, s6);
                    add(c, S7, 1);
                    c.g[V0][0] = lt(c, S7, FP);
                    add(c, A2, 2);
                    if c.g[V0][0] == 0 {
                        break 'pass2;
                    }
                    c.cycles += 1;
                    add(c, S3, 1);
                }
                c.cycles += 3;
                c.g[V0][0] = lt(c, S3, FP);
                c.g[V1][0] = 0x51_0000;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_a238);
                }
            }
        }
        c.cycles += 2;
        let all = c.g[S7][0] == c.g[FP][0];
        c.g[V0][0] = Addr(sp(c) + 0xc).lw(c);
        if all {
            snd_alloc_voices_take(c)?;
        }
        c.cycles += 12;
        for (r, off) in [
            (S0, 0xe0),
            (S1, 0xd0),
            (S2, 0xc0),
            (S3, 0xb0),
            (S4, 0xa0),
            (S5, 0x90),
            (S6, 0x80),
            (S7, 0x70),
            (FP, 0x60),
        ] {
            frame.restore(c, r, off);
        }
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// [`snd_alloc_voices`] with all `s7` voices found: sort them, the handle,
/// stop what the stolen voices played, claim them, link them to the first.
/// Leaves the result in `v0`.
fn snd_alloc_voices_take(c: &mut Cpu) -> crate::call::Flow {
    let sp = |c: &Cpu| c.g[SP][0] as u32;
    let add = |c: &mut Cpu, r: usize, k: u32| c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(k));
    let add_reg = |c: &mut Cpu, d: usize, s: usize| {
        c.g[d][0] = sx((c.g[d][0] as u32).wrapping_add(c.g[s][0] as u32))
    };
    let lt = |c: &Cpu, a: usize, b: usize| ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64;
    c.cycles += 4;
    c.g[S5][0] = 0x51_0000;
    c.g[T2][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
    c.g[T4][0] = c.g[S5][0];
    c.g[T3][0] = 1;
    // Bubble sort.
    loop {
        c.cycles += 2;
        c.g[S3][0] = 0;
        if (c.g[T2][0] as i64) > 0 {
            c.cycles += 4;
            c.g[T0][0] = sx(SNDV_SELECTED);
            c.g[T1][0] = c.g[T2][0];
            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[A0][0] = Addr(c.g[A1][0] as u32).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
                c.g[V0][0] = lt(c, V0, A0);
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lhu(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    let (a2, a0) = (c.g[A2][0], c.g[A0][0]);
                    Addr(c.g[A1][0] as u32).sh(c, a2);
                    c.g[T3][0] = 0;
                    Addr(c.g[V1][0] as u32).sh(c, a0);
                }
                c.cycles += 4;
                c.g[S3][0] = c.g[A3][0];
                c.g[V0][0] = lt(c, S3, T1);
                c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_a360);
                }
            }
        }
        c.cycles += 2;
        if c.g[T3][0] != 0 {
            break;
        }
        c.g[T3][0] = 1;
        if event_due(c) {
            deliver_event_here(c, 0x003b_a348);
        }
    }
    // The handle: serial | first voice.
    c.cycles += 9;
    c.g[V1][0] = 0x45_0000;
    c.g[V0][0] = Addr(SNDV_SERIAL).lw(c);
    c.g[V1][0] = Addr(SNDV_SELECTED).lh(c);
    c.g[V0][0] |= c.g[V1][0];
    c.g[V1][0] = Addr(sp(c) + 4).lw(c);
    let v0 = c.g[V0][0];
    Addr(c.g[V1][0] as u32).sw(c, v0);
    c.g[V0][0] = Addr(SNDV_SELECTED).lh(c);
    let v0 = c.g[V0][0];
    Addr(sp(c) + 0xc).sw(c, v0);
    if (c.g[S7][0] as i64) > 0 {
        // Claim: stop what a stolen voice played (its master's handle for
        // a linked one), mark in use, start tick, priority.
        c.cycles += 7;
        c.g[V0][0] = 0x51_0000;
        c.g[S4][0] = SNDV_STRIDE as u64;
        c.g[S2][0] = sx(SNDV_SYSTEM);
        c.g[S6][0] = 1;
        c.g[S1][0] = sx(SNDV_SELECTED);
        c.g[S3][0] = c.g[S7][0];
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lh(c);
            c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0x1f0).lw(c);
            c.mult(0, V0, V0, S4);
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x69).lb(c);
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.cycles += 2;
                if (c.g[A0][0] as i64) < 0 {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x28).lh(c);
                    c.mult(0, V1, V0, S4);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                }
                c.cycles += 2;
                SNDV_STOP.call(c, 0x003b_a424)?;
            }
            c.cycles += 8;
            let voice = Addr(c.g[S0][0] as u32);
            let s6 = c.g[S6][0];
            voice.off(0x69).sb(c, s6);
            add(c, S1, 2);
            add(c, S3, u32::MAX);
            c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x180).lw(c);
            c.g[V1][0] = Addr(sp(c)).lw(c);
            let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
            voice.off(0x2c).sw(c, v0);
            voice.off(0x2a).sb(c, v1);
            if c.g[S3][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a3e8);
            }
        }
    }
    // The first voice holds the handle and the list; the others point to it.
    c.cycles += 25;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = Addr(SNDV_SELECTED).lh(c);
    c.g[T1][0] = sx(SNDV_SYSTEM);
    c.g[A1][0] = SNDV_STRIDE as u64;
    c.g[V0][0] = Addr(sp(c) + 4).lw(c);
    c.mult(0, A0, A0, A1);
    c.g[T0][0] = u64::MAX;
    c.g[S3][0] = 1;
    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A3][0] = lt(c, S3, S7);
    c.g[V0][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
    add_reg(c, A0, V0);
    let a2 = c.g[A2][0];
    Addr(c.g[A0][0] as u32).sw(c, a2);
    c.g[V1][0] = Addr(SNDV_SELECTED).lh(c);
    c.g[V0][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[A0][0] = Addr(SNDV_SELECTED).lhu(c);
    add_reg(c, V1, V0);
    let a0 = c.g[A0][0];
    Addr(c.g[V1][0] as u32).off(4).sh(c, a0);
    c.g[V0][0] = Addr(SNDV_SELECTED).lh(c);
    c.g[V1][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
    c.mult(0, V0, V0, A1);
    add_reg(c, V0, V1);
    let t0 = c.g[T0][0];
    Addr(c.g[V0][0] as u32).off(0x28).sh(c, t0);
    if c.g[A3][0] != 0 {
        c.cycles += 6;
        c.g[T2][0] = c.g[S5][0];
        c.g[T0][0] = SNDV_STRIDE as u64;
        c.g[V0][0] = sx(SNDV_SELECTED);
        c.g[T3][0] = u64::MAX;
        c.g[A3][0] = sx(SNDV_SELECTED + 2);
        loop {
            c.cycles += 23;
            c.g[V1][0] = Addr(SNDV_SELECTED).lh(c);
            c.g[A1][0] = sx((c.g[S3][0] as u32) << 1);
            c.g[V0][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
            add(c, S3, 1);
            c.mult(0, V1, V1, T0);
            c.g[A0][0] = Addr(c.g[A3][0] as u32).lhu(c);
            c.g[A2][0] = lt(c, S3, S7);
            add_reg(c, V1, V0);
            add_reg(c, V1, A1);
            let a0 = c.g[A0][0];
            Addr(c.g[V1][0] as u32).off(4).sh(c, a0);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lh(c);
            c.g[V1][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
            c.mult(0, V0, V0, T0);
            add_reg(c, V0, V1);
            let t3 = c.g[T3][0];
            Addr(c.g[V0][0] as u32).sw(c, t3);
            c.g[V1][0] = Addr(c.g[A3][0] as u32).lh(c);
            c.g[A0][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
            add(c, A3, 2);
            c.mult(0, V1, V1, T0);
            c.g[V0][0] = Addr(SNDV_SELECTED).lhu(c);
            add_reg(c, V1, A0);
            let v0 = c.g[V0][0];
            Addr(c.g[V1][0] as u32).off(0x28).sh(c, v0);
            if c.g[A2][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a4c0);
            }
        }
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(sp(c) + 0xc).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_PlayPatchVoices
// ---------------------------------------------------------------------------

const SNDP_NEXT_VOICE_GROUP: Callee = Callee {
    addr: 0x003c_6158,
    f: ee::s13::f_003c6158,
};
const SNDP_GET_GROUP_VOICE_RANGE: Callee = Callee {
    addr: 0x003c_0c18,
    f: ee::s13::f_003c0c18,
};
const SNDP_ALLOC_VOICES: Callee = Callee {
    addr: 0x003b_a0b0,
    f: ee::s13::f_003ba0b0,
};
const SNDP_RANDOM_RANGE: Callee = Callee {
    addr: 0x003c_72a0,
    f: ee::s13::f_003c72a0,
};
const SNDP_MIDI_TO_VOLUME: Callee = Callee {
    addr: 0x003b_c570,
    f: ee::s13::f_003bc570,
};
const SNDP_RANDOM: Callee = Callee {
    addr: 0x003b_c5c8,
    f: ee::s13::f_003bc5c8,
};
const SNDP_CALC_PITCH: Callee = Callee {
    addr: 0x003b_a938,
    f: ee::s13::f_003ba938,
};
const SNDP_CALC_SEND: Callee = Callee {
    addr: 0x003b_a740,
    f: ee::s13::f_003ba740,
};
const SNDP_CALC_VOLUME: Callee = Callee {
    addr: 0x003b_a7b0,
    f: ee::s13::f_003ba7b0,
};
const SNDP_START_VOICES: Callee = Callee {
    addr: 0x003c_3ee0,
    f: ee::s13::f_003c3ee0,
};
const SNDP_CHANNEL_RELEASE: Callee = Callee {
    addr: 0x003b_a550,
    f: ee::s13::f_003ba550,
};

/// Resume points of [`snd_play_patch_voices`] (its loop heads) and its
/// internal stages.
const SNDP_ENTRY: u32 = 0x003b_b140;
const SNDP_GROUP: u32 = 0x003b_b1b8;
const SNDP_COPY: u32 = 0x003b_b470;
const SNDP_START: u32 = 0x003b_b490;
const SNDP_RELEASE: u32 = 0x003b_b508;
const SNDP_FAIL: u32 = 0x003b_b530;
const SNDP_NEXT: u32 = 0x003b_b538;
const SNDP_EXIT: u32 = 0x003b_b554;

/// `int SND_PlayPatchVoices(int a0, int a1, int a2, const Note* note,
/// const Patch* patch, int t1, int t2, int detune)` (0x3bb140): for each
/// voice group of the patch, allocates its voices and sets up the channel
/// (volume, pitch from key and detune with random spread, envelope,
/// pan / send, LFO phases, the patch's voice list), then starts them; when
/// starting fails, releases the voices and tries the next group. Returns
/// the handle (`-9` if none started). Dispatcher-shaped: an event at a loop
/// back edge can unwind, and the function is re-entered at that head.
pub fn snd_play_patch_voices(c: &mut Cpu) {
    guest(c, |c| {
        let sp = |c: &Cpu| c.g[SP][0] as u32;
        let s1 = |c: &Cpu| Addr(c.g[S1][0] as u32);
        let s2 = |c: &Cpu| Addr(c.g[S2][0] as u32);
        let s4 = |c: &Cpu| Addr(c.g[S4][0] as u32);
        let mut pc = c.pc;
        loop {
            match pc {
                SNDP_ENTRY => {
                    c.cycles += 30;
                    let frame = Frame::open(c, 0xd0);
                    c.g[V1][0] = sx(-9i32 as u32);
                    frame.save(c, S2, 0xa0);
                    frame.save(c, S4, 0x80);
                    c.g[S2][0] = c.g[T0][0];
                    frame.save(c, S7, 0x50);
                    c.g[S4][0] = c.g[A3][0];
                    frame.save(c, FP, 0x40);
                    c.g[S7][0] = 0x7f;
                    frame.save(c, S0, 0xc0);
                    c.g[FP][0] = c.g[T3][0];
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S3, 0x90);
                    frame.save(c, S5, 0x70);
                    frame.save(c, S6, 0x60);
                    frame.save_ra(c, 0x30);
                    // The note's key, velocity and pan.
                    c.g[V0][0] = s4(c).lb(c);
                    let v0 = c.g[V0][0];
                    Addr(sp(c) + 0x24).sw(c, v0);
                    c.g[A3][0] = s4(c).off(1).lb(c);
                    let a3 = c.g[A3][0];
                    Addr(sp(c) + 0x28).sw(c, a3);
                    c.g[V0][0] = s4(c).off(5).lb(c);
                    for (r, off) in [
                        (A0, 0x10),
                        (A1, 0x14),
                        (A2, 0x18),
                        (T1, 0x1c),
                        (T2, 0x20),
                        (V0, 0x2c),
                        (V1, 8),
                    ] {
                        let v = c.g[r][0];
                        Addr(sp(c) + off).sw(c, v);
                    }
                    Addr(sp(c) + 0xc).sw(c, 0);
                    pc = SNDP_NEXT;
                }
                SNDP_NEXT => {
                    c.cycles += 6;
                    c.g[A0][0] = sx(sp(c) + 0xc);
                    c.g[A1][0] = c.g[S2][0];
                    SNDP_NEXT_VOICE_GROUP.call(c, 0x003b_b544)?;
                    c.g[S5][0] = c.g[V0][0];
                    if c.g[S5][0] != 0 {
                        c.g[A1][0] = c.g[SP][0];
                        if crate::events::back_edge_unwinds(c, SNDP_GROUP) {
                            return Ok(());
                        }
                        pc = SNDP_GROUP;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(sp(c) + 8).lw(c);
                        pc = SNDP_EXIT;
                    }
                }
                SNDP_GROUP => {
                    // The group's voices.
                    c.cycles += 12;
                    c.g[A2][0] = sx(sp(c) + 4);
                    c.g[A0][0] = c.g[S5][0];
                    SNDP_GET_GROUP_VOICE_RANGE.call(c, 0x003b_b1c4)?;
                    c.g[T0][0] = Addr(sp(c) + 4).lw(c);
                    c.g[A2][0] = sx(sp(c) + 8);
                    c.g[A0][0] = s2(c).off(3).lb(c);
                    c.g[A1][0] = s2(c).off(0xc).lb(c);
                    c.g[A3][0] = Addr(sp(c)).lw(c);
                    SNDP_ALLOC_VOICES.call(c, 0x003b_b1dc)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = SNDV_STRIDE as u64;
                    if (c.g[S3][0] as i64) < 0 {
                        pc = SNDP_NEXT;
                        continue;
                    }
                    snd_play_patch_setup_channel(c)?;
                    // The patch's voice list.
                    c.cycles += 32;
                    c.g[V1][0] = s4(c).off(0xe).lhu(c);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = 0;
                    let v1 = c.g[V1][0];
                    s1(c).off(0x6a).sh(c, v1);
                    c.g[V0][0] = s4(c).off(0xc).lhu(c);
                    s1(c).off(0x86).sh(c, 0);
                    let v0 = c.g[V0][0];
                    s1(c).off(0x88).sh(c, v0);
                    SNDP_CALC_PITCH.call(c, 0x003b_b40c)?;
                    let ch = s1(c);
                    let (p, n) = (s2(c), s4(c));
                    c.g[V0][0] = Addr(sp(c) + 0x10).lw(c);
                    let v0 = c.g[V0][0];
                    ch.off(0x12).sh(c, v0);
                    c.g[V0][0] = ch.off(0x4c).lhu(c);
                    c.g[V1][0] = Addr(sp(c) + 0x14).lw(c);
                    let v1 = c.g[V1][0];
                    ch.off(0x10).sh(c, v1);
                    c.g[V1][0] = n.off(8).lhu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let v0 = c.g[V0][0];
                    ch.off(0x1c).sh(c, v0);
                    c.g[V1][0] = n.off(0xa).lhu(c);
                    let v1 = c.g[V1][0];
                    ch.off(0x1e).sh(c, v1);
                    c.g[V0][0] = p.off(0x60).lw(c);
                    let v0 = c.g[V0][0];
                    ch.off(0x18).sw(c, v0);
                    c.g[V1][0] = p.off(0x58).lw(c);
                    let v1 = c.g[V1][0];
                    ch.off(0x14).sw(c, v1);
                    c.g[V0][0] = p.off(0x56).lhu(c);
                    let v0 = c.g[V0][0];
                    ch.off(0x20).sh(c, v0);
                    c.g[V1][0] = p.off(0x13).lbu(c);
                    let v1 = c.g[V1][0];
                    ch.off(0x22).sb(c, v1);
                    c.g[A0][0] = p.off(3).lbu(c);
                    let (s5, a0) = (c.g[S5][0], c.g[A0][0]);
                    ch.off(0x24).sh(c, s5);
                    ch.off(0x23).sb(c, a0);
                    c.g[V0][0] = p.off(3).lb(c);
                    c.g[V1][0] = sx(p.0.wrapping_add(0xbc));
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = SNDP_START;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = sx(ch.0.wrapping_add(0x50));
                        pc = SNDP_COPY;
                    }
                }
                SNDP_COPY => {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
                    let v0 = c.g[V0][0];
                    Addr(c.g[A0][0] as u32).sh(c, v0);
                    c.g[V0][0] = s2(c).off(3).lb(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(2));
                    if c.g[V0][0] != 0 {
                        if crate::events::back_edge_unwinds(c, SNDP_COPY) {
                            return Ok(());
                        }
                    } else {
                        pc = SNDP_START;
                    }
                }
                SNDP_START => {
                    c.cycles += 21;
                    c.g[V1][0] = s4(c).off(4).lbu(c);
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = 0;
                    let v1 = c.g[V1][0];
                    s1(c).off(0x5f).sb(c, v1);
                    c.g[A3][0] = Addr(sp(c) + 0x2c).lw(c);
                    c.g[V0][0] = s2(c).off(0x16).lbu(c);
                    let (a3, v0) = (c.g[A3][0], c.g[V0][0]);
                    s1(c).off(0x61).sb(c, a3);
                    s1(c).off(0x60).sb(c, v0);
                    SNDP_CALC_SEND.call(c, 0x003b_b4b4)?;
                    c.g[A0][0] = c.g[S3][0];
                    SNDP_CALC_VOLUME.call(c, 0x003b_b4bc)?;
                    c.g[A3][0] = s4(c).off(0xe).lhu(c);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[T0][0] = s4(c).off(0x12).lhu(c);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = Addr(sp(c) + 0x18).lw(c);
                    c.g[T1][0] = s4(c).off(0x14).lhu(c);
                    c.g[T2][0] = s4(c).off(0x16).lhu(c);
                    SNDP_START_VOICES.call(c, 0x003b_b4dc)?;
                    let started = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = Addr(sp(c) + 8).lw(c);
                    if started {
                        pc = SNDP_EXIT;
                        continue;
                    }
                    // Not started: release the voices.
                    c.cycles += 3;
                    c.g[V0][0] = s2(c).off(3).lb(c);
                    c.g[S0][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = SNDP_FAIL;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = SNDV_STRIDE as u64;
                        c.g[V1][0] = 0x51_0000;
                        c.mult(0, S1, S3, V0);
                        c.g[S3][0] = sx(SNDV_SYSTEM);
                        c.g[V0][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
                        pc = SNDP_RELEASE;
                    }
                }
                SNDP_RELEASE => {
                    c.cycles += 10;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 1);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lh(c);
                    SNDP_CHANNEL_RELEASE.call(c, 0x003b_b520)?;
                    c.g[V0][0] = s2(c).off(3).lb(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x1f0).lw(c);
                        if crate::events::back_edge_unwinds(c, SNDP_RELEASE) {
                            return Ok(());
                        }
                    } else {
                        pc = SNDP_FAIL;
                    }
                }
                SNDP_FAIL => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(-9i32 as u32);
                    let v0 = c.g[V0][0];
                    Addr(sp(c) + 8).sw(c, v0);
                    pc = SNDP_NEXT;
                }
                SNDP_EXIT => {
                    c.cycles += 12;
                    let frame = Frame { size: 0xd0 };
                    for (r, off) in [
                        (S0, 0xc0),
                        (S1, 0xb0),
                        (S2, 0xa0),
                        (S3, 0x90),
                        (S4, 0x80),
                        (S5, 0x70),
                        (S6, 0x60),
                        (S7, 0x50),
                        (FP, 0x40),
                    ] {
                        frame.restore(c, r, off);
                    }
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, SNDP_ENTRY, 0x003b_b584);
                    return Ok(());
                }
            }
        }
    })
}

/// [`snd_play_patch_voices`]: the channel (`s1`, voice `s3`) from the patch
/// (`s2`): volume, pitch, envelope, pan, sends and LFOs, up to the voice
/// list.
fn snd_play_patch_setup_channel(c: &mut Cpu) -> crate::call::Flow {
    let sp = |c: &Cpu| c.g[SP][0] as u32;
    let s1 = |c: &Cpu| Addr(c.g[S1][0] as u32);
    let s2 = |c: &Cpu| Addr(c.g[S2][0] as u32);
    c.cycles += 8;
    c.g[A0][0] = 0x51_0000;
    c.mult(0, V0, S3, V0);
    c.g[A0][0] = sx(SNDV_SYSTEM);
    c.g[V1][0] = Addr(SNDV_SYSTEM).off(0x1f0).lw(c);
    c.g[A0][0] = s2(c).off(0x12).lb(c);
    c.g[S0][0] = s2(c).off(0x11).lb(c);
    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // Volume: the patch's, with a random spread, clamped to 0..127.
    if c.g[A0][0] != 0 {
        c.cycles += 5;
        SNDP_RANDOM_RANGE.call(c, 0x003b_b210)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let ok = (c.g[S0][0] as i64) >= 0;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
        if !ok {
            c.cycles += 2;
            c.g[S0][0] = 0;
        } else {
            c.cycles += 1;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[S7][0];
            }
        }
    }
    c.cycles += 13;
    c.g[A0][0] = c.g[S0][0];
    c.g[S6][0] = 0x64;
    SNDP_MIDI_TO_VOLUME.call(c, 0x003b_b234)?;
    let v0 = c.g[V0][0];
    s1(c).off(0x4c).sh(c, v0);
    // Pitch: the patch's, minus 100 cents per key below the root, detune.
    c.g[V1][0] = s2(c).off(4).lhu(c);
    let v1 = c.g[V1][0];
    s1(c).off(0x84).sh(c, v1);
    c.g[V0][0] = s2(c).off(0xd).lb(c);
    c.g[A3][0] = Addr(sp(c) + 0x1c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
    c.mult(0, V0, V0, S6);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    let v0 = c.g[V0][0];
    s1(c).off(0x84).sh(c, v0);
    if c.g[FP][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
        let v0 = c.g[V0][0];
        s1(c).off(0x84).sh(c, v0);
    }
    c.cycles += 3;
    c.g[A0][0] = s2(c).off(6).lh(c);
    if c.g[A0][0] != 0 {
        c.cycles += 6;
        SNDP_RANDOM_RANGE.call(c, 0x003b_b278)?;
        c.g[V1][0] = s1(c).off(0x84).lhu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let v1 = c.g[V1][0];
        s1(c).off(0x84).sh(c, v1);
    }
    c.g[V0][0] = s2(c).off(0x20).lw(c);
    c.cycles += 10;
    let (ch, p) = (s1(c), s2(c));
    c.g[V1][0] = Addr(sp(c) + 0x24).lw(c);
    let v0 = c.g[V0][0];
    ch.off(0x70).sw(c, v0);
    c.g[A0][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V1][0] = p.off(0x24).lw(c);
    let (a0, v1) = (c.g[A0][0], c.g[V1][0]);
    ch.off(0x38).sw(c, a0);
    ch.off(0x74).sw(c, v1);
    ch.off(0x30).sw(c, 0);
    // Pan: the patch's with a random spread, clamped, scaled by the note's.
    c.g[A0][0] = p.off(0x15).lb(c);
    c.g[S0][0] = p.off(0x14).lb(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        SNDP_RANDOM_RANGE.call(c, 0x003b_b2b8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    }
    let (ch, p) = (s1(c), s2(c));
    c.cycles += 5;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
    c.g[V1][0] = u64::MAX;
    if c.g[V0][0] == 0 {
        c.g[S0][0] = c.g[S7][0];
    }
    c.cycles += 25;
    c.g[V1][0] = (-1 < (c.g[S0][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        c.g[S0][0] = 0;
    }
    c.g[V1][0] = Addr(sp(c) + 0x20).lw(c);
    c.mult(0, V0, S0, V1);
    c.div(0, V0, S7);
    c.g[V0][0] = c.lo[0];
    let v0 = c.g[V0][0];
    ch.off(0x2b).sb(c, v0);
    c.g[V0][0] = Addr(sp(c) + 0x28).lw(c);
    c.g[V1][0] = p.off(0x10).lb(c);
    let v0 = c.g[V0][0];
    ch.off(0x64).sb(c, v0);
    c.mult(0, V1, V1, S6);
    let v1 = c.g[V1][0];
    ch.off(0x82).sh(c, v1);
    // The envelope: its first segment's rate.
    c.g[A0][0] = p.off(0x28).lw(c);
    let a0 = c.g[A0][0];
    ch.off(0x6c).sw(c, a0);
    c.g[V0][0] = p.off(0x1a).lb(c);
    ch.off(0x5d).sb(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    let v0 = c.g[V0][0];
    ch.off(0x40).sw(c, v0);
    c.g[V1][0] = p.off(0xf).lbu(c);
    let v1 = c.g[V1][0];
    ch.off(0x5c).sb(c, v1);
    c.g[V0][0] = p.off(0xe).lbu(c);
    let v0 = c.g[V0][0];
    ch.off(0x5e).sb(c, v0);
    c.g[V1][0] = Addr(a0 as u32).lw(c);
    let v1 = c.g[V1][0];
    ch.off(0x44).sw(c, v1);
    if (v1 as i64) < 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x7fff_ffff;
        ch.off(0x44).sw(c, 0x7fff_ffff);
    }
    c.cycles += 8;
    c.g[A0][0] = ch.off(0x6c).lw(c);
    c.g[A1][0] = ch.off(0x40).lw(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
    c.g[V1][0] = ch.off(0x44).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.cycles += 16;
    c.div(0, V0, V1);
    c.g[V0][0] = c.lo[0];
    let v0 = c.g[V0][0];
    ch.off(0x3c).sw(c, v0);
    // Envelope rates, LFO depths, the bend range.
    for (from, to, width, r) in [
        (0x2c, 0x78, 4, V1),
        (0x30, 0x7c, 4, V0),
        (0x1b, 0x65, 1, V1),
        (0x1d, 0x66, 1, V0),
        (0x54, 0x80, 2, V1),
    ] {
        let v = match width {
            4 => p.off(from).lw(c),
            2 => p.off(from).lhu(c),
            _ => p.off(from).lbu(c),
        };
        c.g[r][0] = v;
        match width {
            4 => ch.off(to).sw(c, v),
            2 => ch.off(to).sh(c, v),
            _ => ch.off(to).sb(c, v),
        }
    }
    // LFO phases: random within the patch's ranges.
    c.g[V0][0] = p.off(0x1c).lb(c);
    if c.g[V0][0] == 0 {
        ch.off(0x67).sb(c, 0);
    } else {
        c.cycles += 6;
        SNDP_RANDOM.call(c, 0x003b_b3a8)?;
        c.g[V1][0] = s2(c).off(0x1c).lb(c);
        c.divu(0, V0, V1);
        c.cycles += 2;
        c.g[A0][0] = c.hi[0];
        let a0 = c.g[A0][0];
        s1(c).off(0x67).sb(c, a0);
    }
    c.cycles += 3;
    c.g[V0][0] = s2(c).off(0x1e).lbu(c);
    if c.g[V0][0] == 0 {
        s1(c).off(0x68).sb(c, 0);
    } else {
        c.cycles += 6;
        SNDP_RANDOM.call(c, 0x003b_b3d4)?;
        c.g[V1][0] = s2(c).off(0x1d).lbu(c);
        c.divu(0, V0, V1);
        c.cycles += 2;
        c.g[A0][0] = c.hi[0];
        let a0 = c.g[A0][0];
        s1(c).off(0x68).sb(c, a0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Audio_PlayPushedSound
// ---------------------------------------------------------------------------

/// `SND_Lock` / `SND_Play(bank, sample, params)` / `SND_Unlock`.
const PUSHED_SND_LOCK: u32 = 0x003b_58a0;
const PUSHED_SND_PLAY: u32 = 0x003b_6820;
const PUSHED_SND_UNLOCK: u32 = 0x003b_58d8;
/// `BankMonitor_AddInstance(monitor, handle, volume, 1)`.
const PUSHED_ADD_INSTANCE: u32 = 0x002a_cfe0;
/// `SoundRequest_SetBankSample(request, bank, sample)`.
const PUSHED_SET_BANK_SAMPLE: u32 = 0x002a_bce8;
/// `BankMonitor_AddTimer(monitor, delay, period, scale, bank, sample,
/// volume, 0)`.
const PUSHED_ADD_TIMER: u32 = 0x002a_dca0;
/// The echo period (a 64-bit value, `.data`).
pub(crate) const PUSHED_ECHO_PERIOD: u32 = 0x004a_3678;
/// The audio system's push stack (`sys + 0x1f4` depth; `+0x1fc` mute
/// flags, `+0x210` volumes, `+0x214` volume scales, `+0x1f8` play params
/// (0x18 each), `+0x248` echoes (0x10 each), `+0x254` keep flags) and the
/// banks (`sys + 0xacc`, 0x60 each).
pub(crate) const PUSHED_DEPTH: u32 = 0x1f4;

/// [`audio_play_pushed_sound`]: the clamp of the pushed volume (word at
/// `base + depth*4`) into the play params' byte at `a0`: 0 below 0, 127
/// from 127, else its low byte. `last` is the final block's cost.
fn pushed_clamp_volume(c: &mut Cpu, base: usize, last: u64) {
    let slot = |c: &mut Cpu| {
        let depth = Addr(c.g[A3][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = sx(((depth as u32) << 2).wrapping_add(c.g[base][0] as u32));
        Addr(c.g[V0][0] as u32)
    };
    'store: {
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            let s = slot(c);
            c.g[V1][0] = s.lw(c);
            c.g[V0][0] = 0;
            if (c.g[V1][0] as i64) < 0 {
                break 'store;
            }
        }
        c.cycles += 7;
        let s = slot(c);
        c.g[V1][0] = ((s.lw(c) as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
            break 'store;
        }
        c.cycles += last;
        let s = slot(c);
        c.g[V0][0] = s.lbu(c);
    }
    c.cycles += 2;
    let v0 = c.g[V0][0];
    Addr(c.g[A0][0] as u32).sb(c, v0);
}

/// `int Audio_PlayPushedSound(PushedSound* p)` (0x2906b8): plays the top of
/// the audio system's push stack (`p[0]` the system, `p[1]` the banks per
/// level, `p[2]` the samples): its volume (muted: 0; scaled: clamped to
/// 0..127) into the play params, `SND_Play` under the sound lock, the
/// instance registered with the bank monitor, and the echoes (each
/// quieter by the echo's percentage) as monitor timers; pops the level.
/// Returns the handle. Dispatcher-shaped (the echo loop can unwind).
pub fn audio_play_pushed_sound(c: &mut Cpu) {
    guest(c, |c| {
        let s3 = |c: &Cpu| Addr(c.g[S3][0] as u32);
        let frame = Frame { size: 0xa0 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x0029_06b8 => {
                    c.cycles += 24;
                    let frame = Frame::open(c, 0xa0);
                    frame.save(c, S3, 0x60);
                    frame.save(c, S0, 0x90);
                    c.g[S3][0] = c.g[A0][0];
                    for (r, off) in [
                        (S1, 0x80),
                        (S2, 0x70),
                        (S4, 0x50),
                        (S5, 0x40),
                        (S6, 0x30),
                        (S7, 0x20),
                        (FP, 0x10),
                    ] {
                        frame.save(c, r, off);
                    }
                    frame.save_ra(c, 0);
                    let sys = s3(c).lw(c);
                    c.g[A1][0] = sys;
                    c.g[A3][0] = sx((sys as u32).wrapping_add(0x1d8));
                    let mon = Addr(c.g[A3][0] as u32);
                    c.g[V0][0] = mon.off(0x1c).lw(c);
                    c.g[V1][0] = Addr(sys as u32).off(PUSHED_DEPTH).lw(c);
                    c.g[T0][0] = mon.off(0x38).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((sys as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1fc).lw(c);
                    c.g[S2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let muted = c.g[A0][0] != 0;
                    let depth4 = |c: &mut Cpu, base: usize| {
                        let d = mon.off(0x1c).lw(c);
                        sx(((d as u32) << 2).wrapping_add(c.g[base][0] as u32))
                    };
                    if muted {
                        // Muted: volume 0.
                        c.cycles += 16;
                        let d = Addr(sys as u32).off(PUSHED_DEPTH).lw(c);
                        c.g[V0][0] = sx(((d as u32) << 2).wrapping_add(c.g[T0][0] as u32));
                        Addr(c.g[V0][0] as u32).sw(c, 0);
                        c.g[A2][0] = 0x18;
                        c.g[A0][0] = Addr(sys as u32).off(PUSHED_DEPTH).lw(c);
                        c.g[V1][0] = mon.off(0x1c).lw(c);
                        c.g[T0][0] = mon.off(0x38).lw(c);
                        c.mult(0, A0, A0, A2);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[T0][0] as u32));
                        c.g[A1][0] = mon.off(0x20).lw(c);
                        c.g[V0][0] = ((Addr(c.g[V1][0] as u32).lw(c) as i64) < 127) as u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        pushed_clamp_volume(c, T0, 5);
                        c.g[V0][0] = s3(c).lw(c);
                    } else {
                        c.cycles += 7;
                        c.g[A2][0] = mon.off(0x3c).lw(c);
                        c.g[V0][0] = depth4(c, A2);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = s3(c).lw(c);
                        } else {
                            // Scaled: volume * scale, clamped to 0..127.
                            c.cycles += 19;
                            c.g[V0][0] = mon.off(0x1c).lw(c);
                            c.g[A0][0] = Addr(c.g[A1][0] as u32).off(PUSHED_DEPTH).lw(c);
                            c.g[V1][0] = mon.off(0x1c).lw(c);
                            c.g[V0][0] =
                                sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[A2][0] as u32));
                            c.g[AT][0] = 0x42fe_0000;
                            c.f[3] = 0x42fe_0000;
                            c.g[V1][0] =
                                sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[T0][0] as u32));
                            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                            let scale = Addr(c.g[A1][0] as u32).read_fpu(c);
                            c.f[2] = scale.0;
                            let scaled = |c: &mut Cpu, at: u64| {
                                let v =
                                    FpuF32::from_i32(Addr(at as u32).lw(c) as u32 as i32) * scale;
                                c.f[0] = v.0;
                                v
                            };
                            let at = c.g[V1][0];
                            let v = scaled(c, at);
                            let below = v.lt(FpuF32(c.f[3]));
                            abi::set_fcc(c, below);
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            let mut stored = false;
                            let mut check_high = !below;
                            if below {
                                c.cycles += 11;
                                let at = depth4(c, T0);
                                c.g[V0][0] = at;
                                c.f[1] = 0;
                                let v = scaled(c, at);
                                let pos = FpuF32::ZERO.le(v);
                                abi::set_fcc(c, pos);
                                c.g[A1][0] = 0;
                                check_high = pos;
                            }
                            if check_high {
                                c.cycles += 10;
                                let at = depth4(c, T0);
                                c.g[V0][0] = at;
                                let v = scaled(c, at);
                                let below = v.lt(FpuF32(c.f[3]));
                                abi::set_fcc(c, below);
                                c.g[A1][0] = 0x7f;
                                if below {
                                    c.cycles += 10;
                                    let at = depth4(c, T0);
                                    c.g[V0][0] = at;
                                    let v = scaled(c, at);
                                    c.f[1] = v.to_i32() as u32;
                                    c.g[A1][0] = sx(c.f[1]);
                                    let a1 = c.g[A1][0];
                                    Addr(c.g[V1][0] as u32).sw(c, a1);
                                    stored = true;
                                }
                            }
                            if !stored {
                                c.cycles += 1;
                                let a1 = c.g[A1][0];
                                Addr(c.g[V1][0] as u32).sw(c, a1);
                            }
                            c.cycles += 12;
                            c.g[V0][0] = 0x18;
                            c.g[A0][0] = mon.off(0x1c).lw(c);
                            c.g[V1][0] = mon.off(0x1c).lw(c);
                            c.g[A2][0] = mon.off(0x38).lw(c);
                            c.mult(0, A0, A0, V0);
                            c.g[V1][0] =
                                sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[A2][0] as u32));
                            c.g[A1][0] = mon.off(0x20).lw(c);
                            c.g[V0][0] = ((Addr(c.g[V1][0] as u32).lw(c) as i64) < 127) as u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            pushed_clamp_volume(c, A2, 4);
                            c.g[V0][0] = s3(c).lw(c);
                        }
                    }
                    // Play under the sound lock.
                    c.cycles += 16;
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).off(PUSHED_DEPTH).lw(c);
                    call_ptr(c, PUSHED_SND_LOCK, 0x0029_0910)?;
                    pushed_bank_of(c);
                    c.cycles += 17;
                    let sys = s3(c).lw(c);
                    c.g[A0][0] = sys;
                    c.g[A1][0] = 0x18;
                    c.g[V1][0] = s3(c).off(8).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[A2][0] = Addr(sys as u32).off(PUSHED_DEPTH).lw(c);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A3][0] = Addr(sys as u32).off(0x1f8).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.mult(0, A2, A2, A1);
                    c.g[A0][0] = c.g[T0][0];
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    call_ptr(c, PUSHED_SND_PLAY, 0x0029_097c)?;
                    c.g[FP][0] = c.g[V0][0];
                    call_ptr(c, PUSHED_SND_UNLOCK, 0x0029_0984)?;
                    if (c.g[FP][0] as i64) < 0 {
                        // Failed, and the level keeps no instance: just pop.
                        c.cycles += 9;
                        c.g[V0][0] = sx((s3(c).lw(c) as u32).wrapping_add(0x1d8));
                        let mon = Addr(c.g[V0][0] as u32);
                        c.g[V1][0] = mon.off(0x1c).lw(c);
                        c.g[A0][0] = mon.off(0x7c).lw(c);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        let keep = c.g[V0][0] != 0;
                        c.g[A0][0] = s3(c).lw(c);
                        if keep {
                            pc = 0x0029_0b14;
                            continue;
                        }
                    } else {
                        c.g[A0][0] = s3(c).lw(c);
                    }
                    // The instance.
                    c.cycles += 8;
                    c.g[A3][0] = 1;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, PUSHED_ADD_INSTANCE, 0x0029_09c4)?;
                    c.g[A3][0] = c.g[V0][0];
                    if c.g[A3][0] != 0 {
                        c.cycles += 12;
                        let banks = s3(c).off(4).lw(c);
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(banks as u32));
                        c.g[A2][0] = 0x60;
                        c.g[A1][0] = s3(c).lw(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0xacc).lw(c);
                        c.mult(0, V1, V1, A2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c) ^ 1;
                        c.g[A1][0] = u64::MAX;
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                        }
                        c.cycles += 6;
                        let samples = s3(c).off(8).lw(c);
                        c.g[A0][0] = c.g[A3][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(samples as u32));
                        c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                        call_ptr(c, PUSHED_SET_BANK_SAMPLE, 0x0029_0a18)?;
                    }
                    c.g[V0][0] = s3(c).lw(c);
                    // Echoes: count, delay step, gain percent, volume percent.
                    c.cycles += 7;
                    let sys = Addr(c.g[V0][0] as u32);
                    c.g[V1][0] = sys.off(PUSHED_DEPTH).lw(c);
                    c.g[A0][0] = sys.off(0x248).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let echo = Addr(c.g[S1][0] as u32);
                    c.g[V1][0] = echo.off(4).lw(c);
                    c.g[A0][0] = 0x64;
                    if (c.g[V1][0] as i64) <= 0 {
                        pc = 0x0029_0b10;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = echo.off(0xc).lw(c);
                    c.cycles += 13;
                    c.g[S4][0] = 0x64;
                    c.mult(0, V0, S2, V0);
                    c.g[A1][0] = echo.off(8).lw(c);
                    c.g[S5][0] = c.g[S0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    c.g[S7][0] = 0x7f;
                    let v1 = c.g[V1][0];
                    echo.off(4).sw(c, v1);
                    c.g[S6][0] = u64::MAX;
                    c.div(0, V0, A0);
                    c.g[S2][0] = echo.lw(c);
                    c.g[V0][0] = c.lo[0];
                    c.mult(0, V0, V0, A1);
                    pc = 0x0029_0aec;
                }
                0x0029_0a78 => {
                    // One echo timer.
                    c.cycles += 22;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = s3(c).off(4).lw(c);
                    c.g[T2][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = s3(c).off(8).lw(c);
                    let mon = Addr(c.g[A0][0] as u32);
                    c.g[V0][0] = mon.off(0x1c).lw(c);
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A3][0] = mon.off(0x3c).lw(c);
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[T0][0] = Addr(c.g[A2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V0][0] = 0x4a_0000;
                    c.ldl(A2, PUSHED_ECHO_PERIOD + 7);
                    c.ldr(A2, PUSHED_ECHO_PERIOD);
                    c.g[T3][0] = 0;
                    call_ptr(c, PUSHED_ADD_TIMER, 0x0029_0ac4)?;
                    let echo = Addr(c.g[S1][0] as u32);
                    let left = echo.off(4).lw(c);
                    c.g[V1][0] = sx((left as u32).wrapping_sub(1));
                    if (left as i64) <= 0 {
                        pc = 0x0029_0b10;
                        continue;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = echo.off(8).lw(c);
                    let v1 = c.g[V1][0];
                    echo.off(4).sw(c, v1);
                    c.mult(0, V0, S0, V0);
                    c.g[A0][0] = echo.lw(c);
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    pc = 0x0029_0aec;
                }
                0x0029_0aec => {
                    // The next echo's volume: / 100, clamped to 0..127;
                    // silent ends the echoes.
                    c.cycles += 9;
                    c.div(0, V0, S4);
                    c.g[T2][0] = c.lo[0];
                    c.g[S0][0] = c.g[T2][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[S7][0];
                    }
                    c.g[V1][0] = ((c.g[S6][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = 0;
                    }
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = s3(c).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x0029_0a78) {
                            return Ok(());
                        }
                        pc = 0x0029_0a78;
                    } else {
                        pc = 0x0029_0b10;
                    }
                }
                0x0029_0b10 => {
                    c.cycles += 1;
                    c.g[A0][0] = s3(c).lw(c);
                    pc = 0x0029_0b14;
                }
                0x0029_0b14 => {
                    // Pop the level.
                    c.cycles += 16;
                    c.g[V0][0] = c.g[FP][0];
                    let sys = Addr(c.g[A0][0] as u32);
                    for (r, off) in [
                        (S0, 0x90),
                        (S1, 0x80),
                        (S2, 0x70),
                        (S3, 0x60),
                        (S4, 0x50),
                        (S5, 0x40),
                        (S6, 0x30),
                        (S7, 0x20),
                        (FP, 0x10),
                    ] {
                        frame.restore(c, r, off);
                    }
                    let depth = sys.off(PUSHED_DEPTH).lw(c);
                    c.g[V1][0] = sx((depth as u32).wrapping_sub(1));
                    frame.restore_ra(c, 0);
                    let v1 = c.g[V1][0];
                    sys.off(PUSHED_DEPTH).sw(c, v1);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0029_06b8, 0x0029_0b54);
                    return Ok(());
                }
            }
        }
    })
}

/// [`audio_play_pushed_sound`]: `t0` = the bank of the level `s0` (`-1`
/// unless its slot is loaded, state 1), as the original computes it.
fn pushed_bank_of(c: &mut Cpu) {
    let s3 = Addr(c.g[S3][0] as u32);
    let banks = s3.off(4).lw(c);
    c.g[A0][0] = banks;
    c.g[V0][0] = sx(((c.g[S0][0] as u32) << 2).wrapping_add(banks as u32));
    c.g[A2][0] = 0x60;
    c.g[A1][0] = s3.lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0xacc).lw(c);
    c.mult(0, V1, V1, A2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c) ^ 1;
    c.g[T0][0] = u64::MAX;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[T0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
    }
}

// ---------------------------------------------------------------------------
// StreamFile_IssueRead
// ---------------------------------------------------------------------------

const SFIR_RING_IN_RANGE: Callee = Callee {
    addr: 0x003d_fc08,
    f: ee::s14::f_003dfc08,
};
const SFIR_FREE_REQUEST: Callee = Callee {
    addr: 0x003d_fe18,
    f: ee::s14::f_003dfe18,
};
const SFIR_MEM_COPY: Callee = Callee {
    addr: 0x003e_6574,
    f: ee::s14::f_003e6574,
};
const SFIR_ON_READ: Callee = Callee {
    addr: 0x003e_0170,
    f: ee::s14::f_003e0170,
};
const SFIR_QUEUE_READ: Callee = Callee {
    addr: 0x003d_d648,
    f: ee::s14::f_003dd648,
};
const SFIR_SET_REQUEST_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::f_003dcf70,
};
/// Ring markers: `-1` wrap to the start, `-2` skip `[1]` bytes.
const SFIR_WRAP: u64 = u64::MAX;
const SFIR_ENTRY: u32 = 0x003e_03d0;
const SFIR_SKIP: u32 = 0x003e_0400;
const SFIR_FREE: u32 = 0x003e_0450;
const SFIR_SCAN: u32 = 0x003e_0458;

/// `void StreamFile_IssueRead(StreamFile* sf, int priority)` (0x3e03d0):
/// moves the ring's read pointer (`+0x50`) past wrap / skip markers, frees
/// the finished requests that no longer overlap the free space (under the
/// file's mutex `+4`), then finds room for the next read at the write
/// pointer (`+0x58`): before the read pointer, at the end of the buffer
/// (`+0x34`), or by moving the pending data (`+0x54..+0x58`) back to the
/// start (`+0x2c`, 64-byte aligned unless memory-backed). Too little room
/// (`< +0x17c`): state 2 (`+0x38`). Otherwise copies from memory (source
/// `+0x60` kind 1) and completes, or queues a file read with this as its
/// callback. Dispatcher-shaped (both loops can unwind).
pub fn streamfile_issue_read(c: &mut Cpu) {
    guest(c, |c| {
        let s1 = |c: &Cpu| Addr(c.g[S1][0] as u32);
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                SFIR_ENTRY => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x50);
                    c.g[A2][0] = SFIR_WRAP;
                    frame.save(c, S1, 0x30);
                    frame.save(c, S3, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x40);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, S2, 0x20);
                    c.g[A1][0] = sx(-2i32 as u32);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = s1(c).off(0x54).lw(c);
                    pc = SFIR_SKIP;
                }
                SFIR_SKIP => {
                    // Skip the markers at the read pointer.
                    c.cycles += 3;
                    let rd = s1(c).off(0x50).lw(c);
                    c.g[V0][0] = rd;
                    c.g[A0][0] = rd;
                    if rd == c.g[V1][0] {
                        c.cycles += 1;
                        c.g[S0][0] = sx(s1(c).0.wrapping_add(4));
                        pc = 0x003e_0440;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(rd as u32).lw(c);
                    if c.g[V0][0] == c.g[A2][0] {
                        c.cycles += 3;
                        c.g[V0][0] = s1(c).off(0x30).lw(c);
                        let v0 = c.g[V0][0];
                        s1(c).off(0x50).sw(c, v0);
                    } else {
                        c.cycles += 2;
                        let skip = c.g[V0][0] == c.g[A1][0];
                        c.g[S0][0] = sx(s1(c).0.wrapping_add(4));
                        if !skip {
                            pc = 0x003e_0440;
                            continue;
                        }
                        c.cycles += 4;
                        let len = Addr(rd as u32).off(4).lw(c);
                        c.g[V0][0] = sx((rd as u32).wrapping_add(len as u32));
                        let v0 = c.g[V0][0];
                        s1(c).off(0x50).sw(c, v0);
                    }
                    if crate::events::back_edge_unwinds(c, SFIR_SKIP) {
                        return Ok(());
                    }
                }
                0x003e_0440 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    EAFS_MUTEX_LOCK.call(c, 0x003e_0448)?;
                    c.g[S2][0] = c.g[S0][0];
                    pc = SFIR_SCAN;
                }
                SFIR_FREE => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    SFIR_FREE_REQUEST.call(c, 0x003e_0458)?;
                    pc = SFIR_SCAN;
                }
                SFIR_SCAN => {
                    // Free the oldest request while it is done and outside
                    // the data still to be read.
                    c.cycles += 4;
                    c.g[S0][0] = s1(c).off(0x5c).lw(c);
                    let req = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
                    c.g[A0][0] = req;
                    c.g[V0][0] = 1;
                    let mut free = false;
                    if req != 0 {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(req as u32).off(4).lw(c);
                        if c.g[V1][0] != 1 {
                            c.cycles += 7;
                            c.g[A2][0] = Addr(req as u32).off(0x120).lw(c);
                            c.g[A0][0] = s1(c).off(0x50).lw(c);
                            c.g[A1][0] = s1(c).off(0x58).lw(c);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                            SFIR_RING_IN_RANGE.call(c, 0x003e_0488)?;
                            free = c.g[V0][0] == 0;
                            c.g[A1][0] = c.g[S0][0];
                        }
                    }
                    if free {
                        if crate::events::back_edge_unwinds(c, SFIR_FREE) {
                            return Ok(());
                        }
                        pc = SFIR_FREE;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    EAFS_MUTEX_UNLOCK.call(c, 0x003e_0498)?;
                    streamfile_issue_read_room(c)?;
                    pc = 0x003e_0698;
                }
                0x003e_0698 => {
                    c.cycles += 6;
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, SFIR_ENTRY, 0x003e_06b0);
                    return Ok(());
                }
            }
        }
    })
}

/// [`streamfile_issue_read`] after the unlock: find the room, then fill it
/// (or state 2). Ends with `s0` restored.
fn streamfile_issue_read_room(c: &mut Cpu) -> crate::call::Flow {
    let sf = Addr(c.g[S1][0] as u32);
    let frame = Frame { size: 0x50 };
    let lt = |c: &Cpu, a: usize, b: usize| ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64;
    c.g[T0][0] = sf.off(0x50).lw(c);
    c.g[A3][0] = sf.off(0x58).lw(c);
    let wrapped = c.g[A3][0] < c.g[T0][0];
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
    'room: {
        'fill: {
            if wrapped {
                // Room up to the read pointer.
                c.cycles += 2;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x41));
                break 'room;
            }
            // Room up to the end of the buffer.
            c.cycles += 8;
            c.g[V0][0] = sf.off(0x34).lw(c);
            c.g[V1][0] = sf.off(0x17c).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = lt(c, A2, V1);
            c.g[A1][0] = 1;
            if c.g[V1][0] == 0 {
                break 'fill;
            }
            // Not enough: move the pending data to the start, if it fits
            // before the read pointer.
            c.cycles += 6;
            c.g[V1][0] = sf.off(0x60).lw(c);
            c.g[A0][0] = sf.off(0x54).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
            c.g[A2][0] = c.g[V1][0];
            let memory = c.g[V0][0] == c.g[A1][0];
            c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V0][0] = sf.off(0x30).lw(c);
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if memory {
                c.cycles += 6;
                c.g[V0][0] = lt(c, V0, V1);
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if full {
                    c.cycles += 2;
                    return streamfile_issue_read_full(c, frame);
                }
            } else {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = lt(c, V0, V1);
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if full {
                    return streamfile_issue_read_full(c, frame);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x3f;
            let aligned = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            let mut plain = aligned;
            if !aligned {
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x10).lw(c);
                plain = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = u64::MAX;
            }
            if plain {
                c.cycles += 3;
                c.g[V0][0] = sf.off(0x2c).lw(c);
                let v0 = c.g[V0][0];
                sf.off(0x30).sw(c, v0);
            } else {
                // Keep the 64-byte phase: start - (used % 64 - 64).
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f));
                c.g[V0][0] = lt(c, V0, S0);
                c.g[A0][0] = sf.off(0x2c).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32 >> 6) as u32) << 6);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                let a0 = c.g[A0][0];
                sf.off(0x30).sw(c, a0);
            }
            c.cycles += 17;
            c.g[A0][0] = sf.off(0x30).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sf.off(0x54).lw(c);
            SFIR_MEM_COPY.call(c, 0x003e_0584)?;
            // A wrap marker where the data was.
            let old = Addr(sf.off(0x54).lw(c) as u32);
            c.g[A0][0] = old.0 as i32 as u64;
            c.g[V1][0] = 8;
            c.g[V0][0] = SFIR_WRAP;
            old.off(4).sw(c, 8);
            old.sw(c, SFIR_WRAP);
            c.g[A0][0] = sf.off(0x30).lw(c);
            c.g[A1][0] = sf.off(0x50).lw(c);
            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            let (a0, a2) = (c.g[A0][0], c.g[A2][0]);
            sf.off(0x54).sw(c, a0);
            sf.off(0x58).sw(c, a2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            let at_wrap = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(a2 as u32));
            if at_wrap {
                c.cycles += 5;
                c.g[V0][0] = sf.off(0x34).lw(c);
                sf.off(0x50).sw(c, a0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(a2 as u32));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            }
            break 'room;
        }
        // Enough room at the end of the buffer.
        return streamfile_issue_read_fill(c, frame);
    }
    c.cycles += 5;
    c.g[V0][0] = sf.off(0x17c).lw(c);
    c.g[T2][0] = c.g[V0][0];
    c.g[V0][0] = lt(c, A2, V0);
    let full = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if full {
        return streamfile_issue_read_full(c, frame);
    }
    streamfile_issue_read_fill(c, frame)
}

/// [`streamfile_issue_read`]: no room, state 2 (`v0`).
fn streamfile_issue_read_full(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    let v0 = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0x38).sw(c, v0);
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// [`streamfile_issue_read`]: `a2` bytes of room at the write pointer:
/// copy from memory and complete, or queue the file read.
fn streamfile_issue_read_fill(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let sf = Addr(c.g[S1][0] as u32);
    c.cycles += 5;
    c.g[S0][0] = sf.off(0x60).lw(c);
    let src = Addr(c.g[S0][0] as u32);
    c.g[V1][0] = 1;
    c.g[V0][0] = src.off(0x10).lw(c);
    if c.g[V0][0] == 1 {
        // From memory: up to the end of the source.
        c.cycles += 6;
        c.g[A0][0] = sf.off(0x170).lw(c);
        c.g[V1][0] = src.off(0x118).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let past = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if past {
            c.cycles += 2;
            let v0 = c.g[V0][0];
            sf.off(0x178).sw(c, v0);
        } else {
            c.cycles += 1;
            let a2 = c.g[A2][0];
            sf.off(0x178).sw(c, a2);
        }
        c.cycles += 14;
        c.g[A0][0] = sf.off(0x58).lw(c);
        c.g[A1][0] = src.off(0x114).lw(c);
        c.g[A2][0] = sf.off(0x178).lw(c);
        SFIR_MEM_COPY.call(c, 0x003e_0638)?;
        let src = Addr(c.g[S0][0] as u32);
        let sf = Addr(c.g[S1][0] as u32);
        c.g[V0][0] = src.off(0x114).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[V1][0] = sf.off(0x178).lw(c);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let v0 = c.g[V0][0];
        src.off(0x114).sw(c, v0);
        SFIR_ON_READ.call(c, 0x003e_0658)?;
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    // From the file: queue the read, `StreamFile_OnRead` its callback.
    c.g[A0][0] = sf.off(0x16c).lw(c);
    c.cycles += 9;
    c.g[A3][0] = c.g[T2][0];
    c.g[A1][0] = sf.off(0x170).lw(c);
    c.g[T0][0] = c.g[S3][0];
    c.g[A2][0] = sf.off(0x58).lw(c);
    c.g[T1][0] = c.g[S1][0];
    let t2 = c.g[T2][0];
    sf.off(0x178).sw(c, t2);
    SFIR_QUEUE_READ.call(c, 0x003e_067c)?;
    let req = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0x174).sw(c, req);
    if req != 0 {
        c.cycles += 4;
        c.g[A0][0] = req;
        c.g[A1][0] = sx(0x003e_0170);
        SFIR_SET_REQUEST_CALLBACK.call(c, 0x003e_0694)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_PlayPatch
// ---------------------------------------------------------------------------

const SNDPP_NEXT_HANDLE_SERIAL: Callee = Callee {
    addr: 0x003b_b0c8,
    f: ee::s13::f_003bb0c8,
};
const SNDPP_PARSE_PATCH: Callee = Callee {
    addr: 0x003b_aac0,
    f: ee::s13::f_003baac0,
};
const SNDPP_PLAY_PATCH_VOICES: Callee = Callee {
    addr: 0x003b_b140,
    f: ee::s13::f_003bb140,
};
const SNDPP_FIRE_EVENTS: Callee = Callee {
    addr: 0x003b_5f98,
    f: ee::s13::f_003b5f98,
};
const SNDPP_IS_HANDLE_DONE: Callee = Callee {
    addr: 0x003b_8ab8,
    f: ee::s13::f_003b8ab8,
};
const SNDPP_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::f_003ba6e8,
};
/// The voice array pointer (`SNDV_SYSTEM + 0x1f0`).
pub(crate) const SNDPP_VOICES: u32 = 0x0050_aad8;

/// `int SND_PlayPatch(int owner, const u8* patch, int a2, int a3, const
/// Note* note)` (0x3bb588): parses the patch's layers; each whose key and
/// velocity ranges hold the note is played (`SND_PlayPatchVoices`, its
/// events fired). A failure stops what was started. Of the handles still
/// playing, a single one is unlinked; several are linked by a fresh serial
/// (the last one marked as the group's end). Returns the last handle,
/// `-8` without a patch, `-9` if nothing played.
pub fn snd_play_patch(c: &mut Cpu) {
    guest(c, |c| {
        let sp = |c: &Cpu| c.g[SP][0] as u32;
        let add =
            |c: &mut Cpu, r: usize, k: u32| c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(k));
        let lt = |c: &Cpu, a: usize, b: usize| ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64;
        c.cycles += 24;
        let frame = Frame::open(c, 0x1c0);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S3, 0x180);
        frame.save(c, S5, 0x160);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x150);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S7, 0x140);
        c.g[S6][0] = 0;
        frame.save(c, FP, 0x130);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S0, 0x1b0);
        c.g[FP][0] = sx(-9i32 as u32);
        frame.save(c, S1, 0x1a0);
        frame.save(c, S2, 0x190);
        frame.save(c, S4, 0x170);
        frame.save_ra(c, 0x120);
        let (a1, a2, a3) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
        Addr(sp(c) + 0x104).sw(c, a2);
        Addr(sp(c) + 0x108).sw(c, a3);
        Addr(sp(c) + 0x100).sw(c, a1);
        Addr(sp(c) + 0x10c).sw(c, 0);
        Addr(sp(c) + 0x110).sw(c, 0);
        c.g[V0][0] = sx(-8i32 as u32);
        'out: {
            if c.g[V1][0] == 0 {
                break 'out;
            }
            c.cycles += 12;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if c.g[S5][0] == 0 {
                c.g[S5][0] = c.g[V1][0];
            }
            let v0 = c.g[V0][0];
            Addr(sp(c) + 0x100).sw(c, v0);
            let note = Addr(c.g[S7][0] as u32);
            c.g[S2][0] = note.off(2).lb(c);
            c.g[S1][0] = note.off(3).lb(c);
            SNDPP_NEXT_HANDLE_SERIAL.call(c, 0x003b_b600)?;
            let v0 = c.g[V0][0];
            Addr(sp(c) + 0x114).sw(c, v0);
            c.g[S0][0] = sx(sp(c) + 0xd0);
            c.g[V0][0] = sx(sp(c) + 0x100);
            let v0 = c.g[V0][0];
            Addr(sp(c) + 0x118).sw(c, v0);
            c.g[A0][0] = Addr(sp(c) + 0x118).lw(c);
            // Each layer of the patch.
            let failed = 'layers: loop {
                c.cycles += 6;
                c.g[A1][0] = c.g[SP][0];
                SNDPP_PARSE_PATCH.call(c, 0x003b_b620)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[V0][0] = Addr(sp(c) + 0x10c).lw(c);
                let seeded = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(sp(c) + 8).lbu(c);
                if !seeded {
                    // The detune spread, drawn once.
                    c.cycles += 3;
                    c.g[A0][0] = Addr(sp(c)).lh(c);
                    c.g[V0][0] = 1;
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        SNDP_RANDOM_RANGE.call(c, 0x003b_b644)?;
                        let v0 = c.g[V0][0];
                        Addr(sp(c) + 0x110).sw(c, v0);
                        c.g[V0][0] = 1;
                    }
                    c.cycles += 2;
                    let v0 = c.g[V0][0];
                    Addr(sp(c) + 0x10c).sw(c, v0);
                    c.g[V0][0] = Addr(sp(c) + 8).lbu(c);
                }
                // Key (s1) and velocity (s2) within the layer's ranges.
                'skip: {
                    let sext8 = |v: u64| ((v as u32) << 24) as i32 >> 24;
                    c.cycles += 5;
                    c.g[V0][0] = sx(sext8(c.g[V0][0]) as u32);
                    c.g[V0][0] = lt(c, S1, V0);
                    let out = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp(c) + 9).lbu(c);
                    if out {
                        break 'skip;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(sext8(c.g[V0][0]) as u32);
                    c.g[V0][0] = lt(c, V0, S1);
                    let out = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp(c) + 0xa).lbu(c);
                    if out {
                        break 'skip;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(sext8(c.g[V0][0]) as u32);
                    c.g[V0][0] = lt(c, S2, V0);
                    let out = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp(c) + 0xb).lbu(c);
                    if out {
                        break 'skip;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(sext8(c.g[V0][0]) as u32);
                    c.g[V0][0] = lt(c, V0, S2);
                    let out = c.g[V0][0] != 0;
                    c.g[A0][0] = Addr(sp(c) + 0x104).lw(c);
                    if out {
                        break 'skip;
                    }
                    c.cycles += 11;
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A1][0] = Addr(sp(c) + 0x108).lw(c);
                    c.g[A3][0] = c.g[S7][0];
                    c.g[T3][0] = Addr(sp(c) + 0x110).lw(c);
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T1][0] = c.g[S2][0];
                    c.g[T2][0] = c.g[S1][0];
                    SNDPP_PLAY_PATCH_VOICES.call(c, 0x003b_b6c4)?;
                    c.g[FP][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if (c.g[FP][0] as i64) < 0 {
                        break 'layers true;
                    }
                    c.cycles += 5;
                    let h = c.g[FP][0];
                    Addr(c.g[S0][0] as u32).sw(c, h);
                    add(c, S3, 1);
                    add(c, S0, 4);
                    c.g[A1][0] = 1;
                    SNDPP_FIRE_EVENTS.call(c, 0x003b_b6e4)?;
                }
                c.cycles += 2;
                let more = c.g[S4][0] != 0;
                c.g[A0][0] = Addr(sp(c) + 0x118).lw(c);
                if !more {
                    break 'layers false;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_b618);
                }
            };
            'stop: {
                if !failed {
                    // Keep the handles still playing.
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[S3][0] = 0;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 4;
                        c.g[S1][0] = sx(sp(c) + 0xd0);
                        c.g[S0][0] = c.g[V0][0];
                        c.g[S2][0] = c.g[S1][0];
                        loop {
                            c.cycles += 4;
                            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            SNDPP_IS_HANDLE_DONE.call(c, 0x003b_b710)?;
                            let done = c.g[V0][0] != 0;
                            add(c, S0, u32::MAX);
                            if !done {
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                add(c, S3, 1);
                                let v0 = c.g[V0][0];
                                Addr(c.g[S2][0] as u32).sw(c, v0);
                                add(c, S2, 4);
                            }
                            c.cycles += 2;
                            add(c, S1, 4);
                            if c.g[S0][0] == 0 {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003b_b708);
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    if c.g[S3][0] == 0 {
                        break 'stop;
                    }
                    c.cycles += 2;
                    if c.g[S3][0] == 1 {
                        // One: not part of a group.
                        c.cycles += 5;
                        c.g[A0][0] = Addr(sp(c) + 0xd0).lw(c);
                        SNDPP_HANDLE_TO_CHANNEL.call(c, 0x003b_b748)?;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V0][0] = SNDV_STRIDE as u64;
                        if (c.g[V1][0] as i64) >= 0 {
                            c.cycles += 7;
                            c.g[A0][0] = 0x51_0000;
                            c.mult(0, V0, V1, V0);
                            c.g[V1][0] = Addr(SNDPP_VOICES).lw(c);
                            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            let ch = Addr(c.g[S6][0] as u32);
                            ch.off(0x27).sb(c, 0);
                            ch.off(0x26).sb(c, 0);
                        }
                    } else {
                        // Several: one group (the serial), the last its end.
                        c.cycles += 2;
                        c.g[V0][0] = 0x51_0000;
                        if (c.g[S3][0] as i64) > 0 {
                            c.cycles += 4;
                            c.g[S0][0] = c.g[S3][0];
                            c.g[S3][0] = sx(SNDV_SYSTEM);
                            c.g[S2][0] = SNDV_STRIDE as u64;
                            c.g[S1][0] = sx(sp(c) + 0xd0);
                            loop {
                                c.cycles += 5;
                                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                SNDPP_HANDLE_TO_CHANNEL.call(c, 0x003b_b790)?;
                                c.g[V1][0] = c.g[V0][0];
                                let none = (c.g[V1][0] as i64) < 0;
                                c.mult(0, V1, V1, S2);
                                if !none {
                                    c.cycles += 5;
                                    c.g[V0][0] = Addr(c.g[S3][0] as u32).off(0x1f0).lw(c);
                                    c.g[S6][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = Addr(sp(c) + 0x114).lw(c);
                                    let ch = Addr(c.g[S6][0] as u32);
                                    let v0 = c.g[V0][0];
                                    ch.off(0x27).sb(c, 0);
                                    ch.off(0x26).sb(c, v0);
                                }
                                c.cycles += 3;
                                add(c, S0, u32::MAX);
                                add(c, S1, 4);
                                if c.g[S0][0] == 0 {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x003b_b788);
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        Addr(c.g[S6][0] as u32).off(0x27).sb(c, 1);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[FP][0];
                    break 'out;
                }
            }
            // Stop what was started.
            c.cycles += 2;
            c.g[S0][0] = c.g[S3][0];
            if (c.g[S3][0] as i64) > 0 {
                c.cycles += 2;
                c.g[S1][0] = sx(sp(c) + 0xd0);
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                loop {
                    c.cycles += 5;
                    add(c, S0, u32::MAX);
                    add(c, S1, 4);
                    SNDV_STOP.call(c, 0x003b_b7e4)?;
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_b7d8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[FP][0];
        }
        c.cycles += 12;
        for (r, off) in [
            (S0, 0x1b0),
            (S1, 0x1a0),
            (S2, 0x190),
            (S3, 0x180),
            (S4, 0x170),
            (S5, 0x160),
            (S6, 0x150),
            (S7, 0x140),
            (FP, 0x130),
        ] {
            frame.restore(c, r, off);
        }
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayTakeoffSound
// ---------------------------------------------------------------------------

const TAKEOFF_IS_REPLAY_PLAYING: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::f_00288ae0,
};
const TAKEOFF_FIND_RIDER: Callee = Callee {
    addr: 0x0028_5d98,
    f: ee::s07::f_00285d98,
};
const TAKEOFF_GET_SURFACE_SOUND_TYPE: Callee = Callee {
    addr: 0x0029_1710,
    f: ee::s07::f_00291710,
};
const TAKEOFF_GET_CHANNEL_GAIN_PTR: Callee = Callee {
    addr: 0x0028_7968,
    f: ee::s07::f_00287968,
};
const TAKEOFF_GET_RIDER_LISTENER_INDEX: Callee = Callee {
    addr: 0x0028_8a20,
    f: ee::s07::f_00288a20,
};
const TAKEOFF_PLAY_PUSHED_SOUND: Callee = Callee {
    addr: 0x0029_06b8,
    f: ee::s07::f_002906b8,
};
const TAKEOFF_UPDATE_RIDER_AIR_STATE: Callee = Callee {
    addr: 0x0029_4678,
    f: ee::s07::f_00294678,
};
/// `BankMonitor_StopHandle(monitor, handle, 1, f12 0)` (through the table).
const TAKEOFF_STOP_HANDLE: u32 = 0x002a_d5f0;
/// `cBE_getBE` / `BELibrary_GetInterface(be, 0)` (through the table).
const TAKEOFF_GET_BE: u32 = 0x0014_dc80;
const TAKEOFF_GET_INTERFACE: u32 = 0x0014_dd58;
/// `memset` (through the table).
const TAKEOFF_MEMSET: u32 = 0x0041_6210;
/// The game mode byte (2: network / split, `.bss`).
pub(crate) const TAKEOFF_GAME_MODE: u32 = 0x0053_5c11;
/// Audio system fields: `+0x118` the push state (`[0]` the stack), the
/// takeoff handle.
pub(crate) const TAKEOFF_HANDLE: u32 = 0x5fbc;
/// Rider fields: `+0x75c` speed (0..1), `+0x870` its listener, `+0x874` /
/// `+0x87c` local player flags.
pub(crate) const TAKEOFF_RIDER_SPEED: u32 = 0x75c;

/// [`ssxaudio_play_takeoff_sound`]: the slot address of the pushed level
/// in the stack's array at `stack + 0x1d8 + field` (`idx` and `base` as the
/// original uses them).
fn takeoff_level_slot(c: &mut Cpu, idx: usize, base: usize, field: u32, shift: u32) -> Addr {
    let stack = Addr(c.g[S0][0] as u32);
    c.g[idx][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[base][0] = Addr(c.g[S1][0] as u32).off(field).lw(c);
    c.g[idx][0] = sx((c.g[idx][0] as u32) << shift);
    c.g[idx][0] = sx((c.g[idx][0] as u32).wrapping_add(c.g[base][0] as u32));
    Addr(c.g[idx][0] as u32)
}

/// `void SSXAudioSystem_PlayTakeoffSound(SSXAudioSystem* self, Rider*
/// rider)` (0x294170): for the listened-to rider (the replay's or the
/// local player's), stops the previous takeoff sound, pushes a sound level
/// (volume from the speed, 64..127; the surface's takeoff sample; echo
/// 100/90/50; gain and listener of the rider; local players in network
/// mode get the rider's listener) and plays it; then the air state update.
pub fn ssxaudio_play_takeoff_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S5, 0x40);
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[S0][0] = u64::MAX;
        for (r, off) in [
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.save(c, r, off);
        }
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A1][0];
        TAKEOFF_IS_REPLAY_PLAYING.call(c, 0x0029_41ac)?;
        let rider = Addr(c.g[S6][0] as u32);
        // Whose sounds: the replay's (0), the local player's listener, or
        // nobody's (-1).
        if c.g[V0][0] == 0 {
            c.g[V0][0] = rider.off(0x874).lw(c);
            c.cycles += 2;
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = rider.off(0x87c).lw(c);
                c.g[V1][0] = (c.g[V0][0] > 0) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[S0][0] = rider.off(0x870).lw(c);
            }
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0;
        }
        c.cycles += 5;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        TAKEOFF_FIND_RIDER.call(c, 0x0029_41e0)?;
        let found = c.g[V0][0] == c.g[S6][0];
        c.g[A0][0] = c.g[S5][0];
        if found {
            takeoff_push_and_play(c)?;
        }
        c.cycles += 14;
        c.g[A1][0] = c.g[S6][0];
        TAKEOFF_UPDATE_RIDER_AIR_STATE.call(c, 0x0029_4644)?;
        for (r, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, r, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// [`ssxaudio_play_takeoff_sound`] for the listened-to rider (`s5` self,
/// `s6` rider).
fn takeoff_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    let me = Addr(c.g[S5][0] as u32);
    let rider = Addr(c.g[S6][0] as u32);
    // Volume: speed * 127, at least 64, at most 127.
    c.cycles += 9;
    c.g[AT][0] = 0x42fe_0000;
    c.f[1] = 0x42fe_0000;
    c.f[0] = (rider.off(TAKEOFF_RIDER_SPEED).read_fpu(c) * FpuF32(0x42fe_0000)).0;
    c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
    c.g[V1][0] = sx(c.f[1]);
    c.g[A0][0] = ((c.g[V1][0] as i64) < 127) as u64;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 64) as u64;
    let mut top = c.g[A0][0] == 0;
    if !top {
        c.cycles += 2;
        c.g[S7][0] = 0x40;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[S7][0] = c.g[V1][0];
            top = c.g[A0][0] == 0;
        }
    }
    if top {
        c.cycles += 1;
        c.g[S7][0] = 0x7f;
    }
    // Stop the previous takeoff sound if it still plays.
    c.cycles += 8;
    call_ptr(c, PUSHED_SND_LOCK, 0x0029_4228)?;
    c.g[A0][0] = me.off(TAKEOFF_HANDLE).lw(c);
    call_ptr(c, SNDPP_IS_HANDLE_DONE.addr, 0x0029_4230)?;
    c.g[S0][0] = (c.g[V0][0] > 0) as u64;
    call_ptr(c, PUSHED_SND_UNLOCK, 0x0029_4238)?;
    if c.g[S0][0] == 0 {
        c.cycles += 8;
        c.g[V0][0] = me.off(0x118).lw(c);
        c.g[A2][0] = 1;
        c.g[A1][0] = me.off(TAKEOFF_HANDLE).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.f[12] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, TAKEOFF_STOP_HANDLE, 0x0029_425c)?;
    }
    c.g[V0][0] = rider.off(0x874).lw(c);
    // A local player in network mode (not in a replay) gets its listener.
    c.cycles += 3;
    c.g[FP][0] = 0;
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = rider.off(0x87c).lw(c);
        c.g[V1][0] = (c.g[V0][0] > 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S5][0];
    if c.g[V1][0] != 0 {
        c.cycles += 10;
        call_ptr(c, TAKEOFF_GET_BE, 0x0029_4284)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, TAKEOFF_GET_INTERFACE, 0x0029_4290)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = Addr(TAKEOFF_GAME_MODE).lb(c);
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            TAKEOFF_IS_REPLAY_PLAYING.call(c, 0x0029_42ac)?;
            c.g[FP][0] = (c.g[V0][0] < 1) as u64;
        }
        c.g[A0][0] = c.g[S5][0];
    }
    // Push a level: copy the defaults, clear the per-level state.
    c.cycles += 159;
    c.g[A1][0] = c.g[S6][0];
    TAKEOFF_GET_SURFACE_SOUND_TYPE.call(c, 0x0029_42bc)?;
    c.g[S3][0] = 0x18;
    c.g[S0][0] = me.lw(c);
    let stack = Addr(c.g[S0][0] as u32);
    c.g[S2][0] = sx(((c.g[V0][0] as u32) << 3).wrapping_add(2));
    c.g[A1][0] = 0;
    c.g[V0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[S1][0] = sx(stack.0.wrapping_add(0x1d8));
    c.g[A2][0] = 8;
    c.g[S4][0] = 0xa;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let v0 = c.g[V0][0];
    stack.off(PUSHED_DEPTH).sw(c, v0);
    c.g[V1][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[V0][0] = stack.off(0x1f8).lw(c);
    c.mult(0, V1, V1, S3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let base = stack.0;
    c.ldl(V0, base + 0x1e3);
    c.ldr(V0, base + 0x1dc);
    c.ldl(A0, base + 0x1eb);
    c.ldr(A0, base + 0x1e4);
    c.ldl(A3, base + 0x1f3);
    c.ldr(A3, base + 0x1ec);
    let params = c.g[V1][0] as u32;
    c.sdl(V0, params + 7);
    c.sdr(V0, params);
    c.sdl(A0, params + 0xf);
    c.sdr(A0, params + 8);
    c.sdl(A3, params + 0x17);
    c.sdr(A3, params + 0x10);
    c.g[V0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) << 2).wrapping_add(base));
    Addr(c.g[V0][0] as u32).off(0x1fc).sw(c, 0);
    takeoff_level_slot(c, V1, V0, 0x34, 2).sw(c, 0);
    // The volume slot from the params' volume byte.
    c.g[A3][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x20).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S3);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x38).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    let v1 = c.g[V1][0];
    Addr(c.g[A3][0] as u32).sw(c, v1);
    takeoff_level_slot(c, V0, V1, 0x3c, 2).sw(c, 0);
    c.g[A0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A0][0] = sx(((c.g[A0][0] as u32) << 3).wrapping_add(c.g[S1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, TAKEOFF_MEMSET, 0x0029_439c)?;
    let stack = Addr(c.g[S0][0] as u32);
    c.g[V1][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A0][0] = 0x64;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x60).lw(c);
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[T0][0] = 1;
    takeoff_level_slot(c, V0, V1, 0x64, 2).sw(c, 0);
    takeoff_level_slot(c, V1, V0, 0x68, 2).sw(c, 0);
    takeoff_level_slot(c, V0, V1, 0x6c, 2).sw(c, 0);
    // The echo: none yet, delay 100, gain 90%, volume 50%.
    takeoff_level_slot(c, V1, V0, 0x70, 4).off(4).sw(c, 0);
    takeoff_level_slot(c, V0, V1, 0x70, 4).sw(c, 0x64);
    takeoff_level_slot(c, V1, V0, 0x70, 4).off(8).sw(c, 0x5a);
    takeoff_level_slot(c, V0, V1, 0x70, 4).off(0xc).sw(c, 0x32);
    takeoff_level_slot(c, V1, V0, 0x74, 2).sw(c, 0);
    takeoff_level_slot(c, V0, V1, 0x80, 2).sw(c, 0);
    takeoff_level_slot(c, V1, V0, 0x84, 2).sw(c, 0);
    takeoff_level_slot(c, V0, V1, 0x78, 2).sw(c, 0x7f);
    takeoff_level_slot(c, V1, V0, 0x7c, 2).sw(c, 1);
    // Bank 10, the surface's takeoff sample (type * 8 + 2).
    c.g[A0][0] = me.lw(c);
    c.g[V1][0] = me.off(4).lw(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(PUSHED_DEPTH).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, 0xa);
    c.g[V1][0] = me.lw(c);
    c.g[A0][0] = me.off(8).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(PUSHED_DEPTH).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[A0][0] as u32));
    let s2 = c.g[S2][0];
    Addr(c.g[V0][0] as u32).sw(c, s2);
    // The volume, and its clamp into the params.
    c.g[V1][0] = me.off(0x118).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
    let stack = Addr(c.g[A1][0] as u32);
    c.g[V0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A2][0] = sx(stack.0.wrapping_add(0x1d8));
    c.g[V1][0] = stack.off(0x210).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[V1][0] as u32));
    let s7 = c.g[S7][0];
    Addr(c.g[V0][0] as u32).sw(c, s7);
    c.g[A0][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[V1][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A3][0] = stack.off(0x210).lw(c);
    c.mult(0, A0, A0, S3);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = stack.off(0x1f8).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = ((Addr(c.g[V1][0] as u32).lw(c) as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    {
        // (The clamp of `Audio_PlayPushedSound`, its final store charged
        // with the next block.)
        let slot = |c: &mut Cpu| {
            let d = Addr(c.g[A2][0] as u32).off(0x1c).lw(c);
            c.g[V0][0] = sx(((d as u32) << 2).wrapping_add(c.g[A3][0] as u32));
            Addr(c.g[V0][0] as u32)
        };
        'store: {
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                let s = slot(c);
                c.g[V1][0] = s.lw(c);
                c.g[V0][0] = 0;
                if (c.g[V1][0] as i64) < 0 {
                    break 'store;
                }
            }
            c.cycles += 7;
            let s = slot(c);
            c.g[V1][0] = ((s.lw(c) as i64) < 127) as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0x7f;
                break 'store;
            }
            c.cycles += 4;
            let s = slot(c);
            c.g[V0][0] = s.lbu(c);
        }
    }
    // Gain, the rider's position, listener.
    c.cycles += 30;
    let v0 = c.g[V0][0];
    Addr(c.g[A0][0] as u32).sb(c, v0);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 6;
    c.g[S0][0] = me.off(0x118).lw(c);
    TAKEOFF_GET_CHANNEL_GAIN_PTR.call(c, 0x0029_458c)?;
    c.g[A3][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[T0][0] = sx(rider.0.wrapping_add(0x110));
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S6][0];
    let stack = Addr(c.g[A3][0] as u32);
    c.g[V1][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A2][0] = stack.off(0x214).lw(c);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[A2][0] as u32));
    let v0 = c.g[V0][0];
    Addr(c.g[V1][0] as u32).sw(c, v0);
    c.g[A2][0] = me.off(0x118).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(PUSHED_DEPTH).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let t0 = c.g[T0][0];
    Addr(c.g[V1][0] as u32).off(0x1fc).sw(c, t0);
    c.g[S0][0] = me.off(0x118).lw(c);
    TAKEOFF_GET_RIDER_LISTENER_INDEX.call(c, 0x0029_45d0)?;
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    let stack = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = stack.off(PUSHED_DEPTH).lw(c);
    c.g[A1][0] = stack.off(0x23c).lw(c);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) << 2).wrapping_add(c.g[A1][0] as u32));
    let v0 = c.g[V0][0];
    Addr(c.g[V1][0] as u32).sw(c, v0);
    if c.g[FP][0] != 0 {
        c.cycles += 16;
        for (field, reg_val) in [(0x258u32, None), (0x25c, Some(0x870u32))] {
            c.g[A0][0] = me.off(0x118).lw(c);
            c.g[A1][0] = match reg_val {
                None => 1,
                Some(off) => rider.off(off).lw(c),
            };
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            let stack = Addr(c.g[V1][0] as u32);
            c.g[V0][0] = stack.off(PUSHED_DEPTH).lw(c);
            c.g[A0][0] = stack.off(field).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) << 2).wrapping_add(c.g[A0][0] as u32));
            let a1 = c.g[A1][0];
            Addr(c.g[V0][0] as u32).sw(c, a1);
        }
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S5][0];
    TAKEOFF_PLAY_PUSHED_SOUND.call(c, 0x0029_4634)?;
    let v0 = c.g[V0][0];
    Addr(c.g[S5][0] as u32).off(TAKEOFF_HANDLE).sw(c, v0);
    c.g[A0][0] = c.g[S5][0];
    Ok(())
}

// ---------------------------------------------------------------------------
// cStreamMan_ParseChunks
// ---------------------------------------------------------------------------

const SMPC_ALLOC_RESOURCE: Callee = Callee {
    addr: 0x003a_8a90,
    f: ee::s13::f_003a8a90,
};
const SMPC_RESOLVE_RESOURCE: Callee = Callee {
    addr: 0x003a_8cd0,
    f: ee::s13::f_003a8cd0,
};
/// `cStreamMan` chunk parser fields: `+0` the world cache, `+0x94` group,
/// `+0x98` chunks remaining, `+0xa0` bytes of a split header, `+0xa4` the
/// split header, `+0xac` retry the allocation, `+0xb8` id, `+0xbc` key,
/// `+0xc0` bytes left, `+0xc4` resolve an empty chunk, `+0xc8` copying,
/// `+0xcc` the allocation, `+0xd0` the copy destination.
pub(crate) const SMPC_REMAINING: u32 = 0x98;
pub(crate) const SMPC_PARTIAL: u32 = 0xa0;
pub(crate) const SMPC_RETRY: u32 = 0xac;
pub(crate) const SMPC_SIZE: u32 = 0xc0;
const SMPC_ENTRY: u32 = 0x003a_7238;

/// [`cstreamman_parse_chunks`]: `cWorldCache_AllocResource(cache, group,
/// id, key, size)` for the current chunk, its result to `+0xcc`; true
/// when it answered "later" (-1).
fn smpc_alloc(c: &mut Cpu, ret: u32) -> Result<bool, crate::call::Unwound> {
    let me = Addr(c.g[S0][0] as u32);
    c.g[A0][0] = me.lw(c);
    c.g[A1][0] = me.off(0x94).lw(c);
    c.g[A2][0] = me.off(0xb8).lw(c);
    c.g[A3][0] = me.off(0xbc).lw(c);
    c.g[T0][0] = me.off(SMPC_SIZE).lw(c);
    SMPC_ALLOC_RESOURCE.call(c, ret)?;
    let later = c.g[V0][0] == c.g[S4][0];
    let v0 = c.g[V0][0];
    me.off(0xcc).sw(c, v0);
    Ok(later)
}

/// [`cstreamman_parse_chunks`]: the header in the stack buffer (`sp`) to
/// id / key / size (`v0` its 8 bytes).
fn smpc_header(c: &mut Cpu) {
    let me = Addr(c.g[S0][0] as u32);
    let sp = c.g[SP][0] as u32;
    c.g[A0][0] = Addr(sp).lbu(c);
    c.g[V1][0] = Addr(sp + 4).lw(c);
    c.g[V0][0] <<= 24;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] &= c.g[S5][0];
    let (a0, v1, v0) = (c.g[A0][0], c.g[V1][0], c.g[V0][0]);
    me.off(0xb8).sw(c, a0);
    me.off(0xbc).sw(c, v1);
    me.off(SMPC_SIZE).sw(c, v0);
}

/// `int cStreamMan_ParseChunks(cStreamMan* self, int* count, u8** data)`
/// (0x3a7238): consumes streamed bytes: 8-byte chunk headers (id, 24-bit
/// size, key; a header split across calls is kept at `+0xa4`), each
/// chunk's space from the world cache (0 if "later": the caller retries),
/// its bytes copied there (or skipped) and resolved when complete, until
/// no chunks remain; keeps a trailing partial header. Dispatcher-shaped:
/// its loops resume at their heads.
pub fn cstreamman_parse_chunks(c: &mut Cpu) {
    guest(c, |c| {
        let me = |c: &Cpu| Addr(c.g[S0][0] as u32);
        let count = |c: &Cpu| Addr(c.g[S2][0] as u32);
        let data = |c: &Cpu| Addr(c.g[S3][0] as u32);
        let sp = |c: &Cpu| c.g[SP][0] as u32;
        let mut pc = c.pc;
        loop {
            match pc {
                SMPC_ENTRY => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x90);
                    frame.save(c, S4, 0x40);
                    frame.save(c, S5, 0x30);
                    frame.save(c, S0, 0x80);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S2, 0x60);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save(c, S3, 0x50);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S6, 0x20);
                    frame.save(c, S1, 0x70);
                    c.g[S4][0] = u64::MAX;
                    frame.save_ra(c, 0x10);
                    c.g[S6][0] = 1;
                    c.g[S5][0] = 0xff_ffff;
                    c.cycles += 1;
                    c.g[V1][0] = count(c).lw(c);
                    pc = 0x003a_74f0;
                }
                0x003a_7280 => {
                    // Loop head: retry, a new header, or more bytes.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = count(c).lw(c);
                            pc = 0x003a_7414;
                        } else {
                            pc = 0x003a_72c0;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = me(c).lw(c);
                        pc = 0x003a_728c;
                    }
                }
                0x003a_728c => {
                    // Retry the allocation.
                    c.cycles += 7;
                    if smpc_alloc(c, 0x003a_72a0)? {
                        pc = 0x003a_73e8;
                        continue;
                    }
                    c.cycles += 2;
                    me(c).off(SMPC_RETRY).sw(c, 0);
                    if c.g[V0][0] != 0 {
                        pc = 0x003a_73f8;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        pc = 0x003a_7404;
                    }
                }
                0x003a_72c0 => {
                    // A new header: complete a split one or read all 8.
                    c.cycles += 3;
                    c.g[V0][0] = me(c).off(SMPC_PARTIAL).lw(c);
                    c.g[T0][0] = c.g[SP][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        let s4 = c.g[S4][0];
                        Addr(sp(c) + 4).sw(c, s4);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = 7;
                        pc = 0x003a_7378;
                        continue;
                    }
                    c.cycles += 3;
                    let s4 = c.g[S4][0];
                    Addr(sp(c) + 4).sw(c, s4);
                    c.g[A2][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x003a_72fc;
                    } else {
                        c.cycles += 2;
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = sx(me(c).0.wrapping_add(0xa4));
                        pc = 0x003a_72e0;
                    }
                }
                0x003a_72e0 => {
                    // The kept bytes of the split header.
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let v1 = c.g[V1][0];
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    if c.g[V0][0] != 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_72e0) {
                            return Ok(());
                        }
                    } else {
                        pc = 0x003a_72fc;
                    }
                }
                0x003a_72fc => {
                    c.cycles += 4;
                    c.g[A2][0] = me(c).off(SMPC_PARTIAL).lw(c);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp(c)).ld(c);
                    if more {
                        c.cycles += 1;
                        pc = 0x003a_7310;
                    } else {
                        pc = 0x003a_7344;
                    }
                }
                0x003a_7310 => {
                    // The rest of the split header from the data.
                    c.cycles += 12;
                    c.g[V0][0] = data(c).lw(c);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[A1][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    data(c).sw(c, v0);
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    c.g[V0][0] = sx((count(c).lw(c) as u32).wrapping_sub(1));
                    let v0 = c.g[V0][0];
                    count(c).sw(c, v0);
                    if c.g[A1][0] != 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_7310) {
                            return Ok(());
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(sp(c)).ld(c);
                        pc = 0x003a_7344;
                    }
                }
                0x003a_7344 => {
                    c.cycles += 10;
                    smpc_header(c);
                    me(c).off(SMPC_PARTIAL).sw(c, 0);
                    pc = 0x003a_73c8;
                }
                0x003a_7378 => {
                    // A whole header from the data.
                    c.cycles += 11;
                    c.g[V0][0] = data(c).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    data(c).sw(c, v0);
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((count(c).lw(c) as u32).wrapping_sub(1));
                    let v0 = c.g[V0][0];
                    count(c).sw(c, v0);
                    if (c.g[A1][0] as i64) >= 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_7378) {
                            return Ok(());
                        }
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(sp(c)).ld(c);
                        smpc_header(c);
                        pc = 0x003a_73c8;
                    }
                }
                0x003a_73c8 => {
                    c.cycles += 8;
                    if smpc_alloc(c, 0x003a_73e0)? {
                        pc = 0x003a_73e8;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        pc = 0x003a_7404;
                    } else {
                        pc = 0x003a_73f8;
                    }
                }
                0x003a_73e8 => {
                    // "Later": stop, the caller retries.
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x003a_756c;
                }
                0x003a_73f8 => {
                    // Space: copy into it.
                    c.cycles += 3;
                    let (v0, s6) = (c.g[V0][0], c.g[S6][0]);
                    me(c).off(0xd0).sw(c, v0);
                    me(c).off(0xc8).sw(c, s6);
                    c.cycles += 1;
                    c.g[A2][0] = count(c).lw(c);
                    pc = 0x003a_7414;
                }
                0x003a_7404 => {
                    // No space: skip it (an empty chunk still resolves).
                    c.cycles += 2;
                    let size = c.g[V0][0];
                    me(c).off(0xc8).sw(c, 0);
                    if size == 0 {
                        c.cycles += 1;
                        let s6 = c.g[S6][0];
                        me(c).off(0xc4).sw(c, s6);
                    }
                    c.cycles += 1;
                    c.g[A2][0] = count(c).lw(c);
                    pc = 0x003a_7414;
                }
                0x003a_7414 => {
                    // The chunk's bytes in this data.
                    c.cycles += 2;
                    c.g[S1][0] = c.g[A2][0];
                    if c.g[A2][0] == 0 {
                        pc = 0x003a_74d8;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                    c.g[A0][0] = me(c).off(0xc8).lw(c);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    let v0 = c.g[V0][0];
                    me(c).off(SMPC_SIZE).sw(c, v0);
                    if c.g[A0][0] != 0 {
                        c.cycles += 9;
                        c.g[A0][0] = me(c).off(0xd0).lw(c);
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A1][0] = data(c).lw(c);
                        call_ptr(c, SFIR_MEM_COPY.addr, 0x003a_7448)?;
                        c.g[V0][0] = me(c).off(0xd0).lw(c);
                        c.g[V1][0] = me(c).off(SMPC_SIZE).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        let v0 = c.g[V0][0];
                        me(c).off(0xd0).sw(c, v0);
                        if c.g[V1][0] == 0 {
                            // Complete: resolve with the allocation.
                            c.cycles += 11;
                            let m = me(c);
                            c.g[A0][0] = m.lw(c);
                            c.g[A1][0] = m.off(0x94).lw(c);
                            c.g[A2][0] = m.off(0xb8).lw(c);
                            c.g[A3][0] = m.off(0xbc).lw(c);
                            c.g[T0][0] = m.off(0xcc).lw(c);
                            SMPC_RESOLVE_RESOURCE.call(c, 0x003a_7474)?;
                            let m = me(c);
                            c.g[V0][0] = m.off(SMPC_REMAINING).lw(c);
                            m.off(0xcc).sw(c, 0);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            let v0 = c.g[V0][0];
                            m.off(SMPC_REMAINING).sw(c, v0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = data(c).lw(c);
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = data(c).lw(c);
                        } else {
                            // A skipped chunk ends: resolve an empty one.
                            c.cycles += 5;
                            let m = me(c);
                            c.g[V0][0] = m.off(SMPC_REMAINING).lw(c);
                            c.g[V1][0] = m.off(0xc4).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            let v0 = c.g[V0][0];
                            m.off(SMPC_REMAINING).sw(c, v0);
                            if c.g[V1][0] != 0 {
                                c.cycles += 7;
                                c.g[A0][0] = m.lw(c);
                                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x16e0));
                                c.g[A1][0] = m.off(0x94).lw(c);
                                c.g[A2][0] = m.off(0xb8).lw(c);
                                c.g[A3][0] = m.off(0xbc).lw(c);
                                SMPC_RESOLVE_RESOURCE.call(c, 0x003a_74bc)?;
                                me(c).off(0xc4).sw(c, 0);
                            }
                            c.cycles += 1;
                            c.g[V0][0] = data(c).lw(c);
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    let v0 = c.g[V0][0];
                    data(c).sw(c, v0);
                    c.g[V1][0] = sx((count(c).lw(c) as u32).wrapping_sub(c.g[S1][0] as u32));
                    let v1 = c.g[V1][0];
                    count(c).sw(c, v1);
                    pc = 0x003a_74d8;
                }
                0x003a_74d8 => {
                    // All chunks done: drop the rest.
                    c.cycles += 3;
                    c.g[V0][0] = me(c).off(SMPC_REMAINING).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = count(c).lw(c);
                        pc = 0x003a_74f0;
                    } else {
                        c.cycles += 2;
                        count(c).sw(c, 0);
                        pc = 0x003a_7518;
                    }
                }
                0x003a_74f0 => {
                    // Go on while a header fits, a chunk continues or a
                    // retry is due.
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_7280) {
                            return Ok(());
                        }
                        pc = 0x003a_7280;
                        continue;
                    }
                    c.cycles += 2;
                    if (c.g[V1][0] as i64) > 0 {
                        c.cycles += 3;
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        let going = c.g[V0][0] != 0;
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                        if going {
                            if crate::events::back_edge_unwinds(c, 0x003a_7280) {
                                return Ok(());
                            }
                            pc = 0x003a_7280;
                            continue;
                        }
                    } else {
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = me(c).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_728c) {
                            return Ok(());
                        }
                        pc = 0x003a_728c;
                    } else {
                        pc = 0x003a_7518;
                    }
                }
                0x003a_7518 => {
                    // Keep a trailing partial header.
                    c.cycles += 3;
                    c.g[V0][0] = count(c).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = 1;
                        pc = 0x003a_756c;
                        continue;
                    }
                    c.cycles += 4;
                    let v0 = c.g[V0][0];
                    me(c).off(SMPC_PARTIAL).sw(c, v0);
                    c.g[V0][0] = count(c).lw(c);
                    c.g[A1][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x003a_7568;
                    } else {
                        c.cycles += 3;
                        c.g[A2][0] = sx(me(c).0.wrapping_add(0xa4));
                        c.g[V1][0] = data(c).lw(c);
                        pc = 0x003a_7540;
                    }
                }
                0x003a_7540 => {
                    c.cycles += 10;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    Addr(c.g[A0][0] as u32).sb(c, v0);
                    data(c).sw(c, v1);
                    c.g[V0][0] = count(c).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = data(c).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_7540) {
                            return Ok(());
                        }
                    } else {
                        pc = 0x003a_7568;
                    }
                }
                0x003a_7568 => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x003a_756c;
                }
                0x003a_756c => {
                    c.cycles += 10;
                    let frame = Frame { size: 0x90 };
                    for (r, off) in [
                        (S0, 0x80),
                        (S1, 0x70),
                        (S2, 0x60),
                        (S3, 0x50),
                        (S4, 0x40),
                        (S5, 0x30),
                        (S6, 0x20),
                    ] {
                        frame.restore(c, r, off);
                    }
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, SMPC_ENTRY, 0x003a_7594);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_RiderSay2083
// ---------------------------------------------------------------------------

const SAY_IS_SPEECH_VOLUME_ON: Callee = Callee {
    addr: 0x0029_f0a0,
    f: ee::s08::f_0029f0a0,
};
const SAY_IS_TRACK_GROUP4: Callee = Callee {
    addr: 0x002a_3fe8,
    f: ee::s08::f_002a3fe8,
};
const SAY_IS_INSTANCE_READY: Callee = Callee {
    addr: 0x002b_0e28,
    f: ee::s08::f_002b0e28,
};
const SAY_GET_RIDER_SPEECH_MASK: Callee = Callee {
    addr: 0x0029_f330,
    f: ee::s08::f_0029f330,
};
const SAY_TRACK_TO_CATEGORY: Callee = Callee {
    addr: 0x002a_1e20,
    f: ee::s08::f_002a1e20,
};
const SAY_SET_EVENT_CONTEXT: Callee = Callee {
    addr: 0x002b_1458,
    f: ee::s08::f_002b1458,
};
/// Through the table: `SSXAudioSystem_FindViewInRange(self, pos, f12)`,
/// `SSXAudioSystem_GetBELibrary`, `BENewRaceInt_GetTrackInfo`,
/// `Rider_GetCharacter`, `SPCH_MakeId(1, 0, event)`.
const SAY_FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;
const SAY_GET_BE_LIBRARY: u32 = 0x0028_b1e8;
const SAY_GET_TRACK_INFO: u32 = 0x0014_4bc0;
const SAY_GET_CHARACTER: u32 = 0x0012_3128;
const SAY_MAKE_ID: u32 = 0x003d_8008;
/// The speech queue hook (`gp+0x173c`), called `(id, 2, mask, 3)`.
pub(crate) const SAY_QUEUE_HOOK: u32 = 0x173c;

/// `void SSXAudioSystem_RiderSay2083(SSXAudioSystem* self, Rider* rider)`
/// (0x29fcc8): with speech on, outside track group 4 and replays, when the
/// rider is falling (vertical speed `+0x470` below 0) fast enough (its
/// physics' `+0x98` at least the `gp-0x4570` limit) and is the listened-to
/// player (or a local one), the speech instance is ready and a view sees
/// it: says event 0x2084 / 0x2083 / 0x2091 by the track's category.
pub fn ssxaudio_rider_say_2083(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        SAY_IS_SPEECH_VOLUME_ON.call(c, 0x0029_fcf4)?;
        'out: {
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            SAY_IS_TRACK_GROUP4.call(c, 0x0029_fd04)?;
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, TAKEOFF_IS_REPLAY_PLAYING.addr, 0x0029_fd14)?;
            if c.g[V0][0] != 0 {
                break 'out;
            }
            let rider = Addr(c.g[S0][0] as u32);
            // Falling?
            let falling = |c: &mut Cpu, fr: usize| -> bool {
                c.cycles += 6;
                c.f[fr] = rider.off(0x470).lw(c) as u32;
                let other = if fr == 1 { 0 } else { 20 };
                c.f[other] = 0;
                let up = FpuF32(c.f[other]).le(FpuF32(c.f[fr]));
                abi::set_fcc(c, up);
                c.g[V0][0] = 1;
                if !up {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 2;
                !up
            };
            if !falling(c, 1) {
                break 'out;
            }
            c.cycles += 7;
            c.g[V0][0] = rider.off(0x788).lw(c);
            c.f[1] = Addr(gp.wrapping_sub(0x4570)).lw(c) as u32;
            c.f[0] = Addr(c.g[V0][0] as u32).off(0x98).lw(c) as u32;
            let slow = FpuF32(c.f[0]).lt(FpuF32(c.f[1]));
            abi::set_fcc(c, slow);
            if slow {
                break 'out;
            }
            // The listened-to rider, unless it is a remote one.
            c.cycles += 10;
            c.g[V1][0] = rider.off(0x6c0).lw(c);
            c.g[A1][0] = sx(rider.0.wrapping_add(0x6c0));
            c.g[S4][0] = c.g[A1][0];
            c.g[S2][0] = 0;
            let vt = Addr(c.g[V1][0] as u32);
            c.g[A0][0] = vt.off(0x48).lh(c);
            c.g[V0][0] = vt.off(0x4c).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0029_fd7c)?;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = rider.off(0x87c).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 1;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            if c.g[S2][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                call_ptr(c, TAKEOFF_FIND_RIDER.addr, 0x0029_fda0)?;
                if c.g[V0][0] != c.g[S0][0] {
                    break 'out;
                }
            }
            if !falling(c, 0) {
                break 'out;
            }
            c.cycles += 6;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S2][0];
            SAY_IS_INSTANCE_READY.call(c, 0x0029_fddc)?;
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 14;
            c.g[A0][0] = c.g[S0][0];
            SAY_GET_RIDER_SPEECH_MASK.call(c, 0x0029_fdec)?;
            c.g[V1][0] = rider.off(0x6c0).lw(c);
            c.g[S3][0] = c.g[V0][0];
            let vt = Addr(c.g[V1][0] as u32);
            c.g[A0][0] = vt.off(0x28).lh(c);
            c.g[V0][0] = vt.off(0x2c).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0029_fe04)?;
            c.f[12] = Addr(gp.wrapping_add(0x5e8)).lw(c) as u32;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, SAY_FIND_VIEW_IN_RANGE, 0x0029_fe14)?;
            if (c.g[V0][0] as i64) < 0 {
                break 'out;
            }
            // The event by the track's category.
            c.cycles += 14;
            call_ptr(c, SAY_GET_BE_LIBRARY, 0x0029_fe24)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, TAKEOFF_GET_INTERFACE, 0x0029_fe30)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, SAY_GET_TRACK_INFO, 0x0029_fe38)?;
            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S1][0];
            SAY_TRACK_TO_CATEGORY.call(c, 0x0029_fe44)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 2;
            let (event, rets, said_cycles) = match c.g[V1][0] {
                1 => (0x2084u32, [0x0029_fe5c, 0x0029_fe80], 3),
                2 => {
                    c.cycles += 2;
                    (0x2083, [0x0029_fea4, 0x0029_fec8], 2)
                }
                _ => {
                    c.cycles += 2;
                    (0x2091, [0x0029_ff08, 0x0029_ff2c], 9)
                }
            };
            c.cycles += 13;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SAY_GET_CHARACTER, rets[0])?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = c.g[V0][0];
            c.f[12] = c.f[20];
            c.g[A1][0] = 0;
            c.g[A2][0] = event as u64;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            SAY_SET_EVENT_CONTEXT.call(c, rets[1])?;
            let said = c.g[V0][0] != 0;
            c.g[A0][0] = 1;
            if said {
                c.cycles += said_cycles;
                c.g[A1][0] = 0;
                c.g[A2][0] = event as u64;
                if event != 0x2091 {
                    c.cycles += 10;
                }
                let ret = if event == 0x2091 {
                    0x0029_ff40
                } else {
                    0x0029_fee0
                };
                call_ptr(c, SAY_MAKE_ID, ret)?;
                c.g[V1][0] = Addr(gp.wrapping_add(SAY_QUEUE_HOOK)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A1][0] = 2;
                let hook = c.g[V1][0] as u32;
                c.g[A3][0] = 3;
                let ret = if event == 0x2091 {
                    0x0029_ff58
                } else {
                    0x0029_fef8
                };
                call_ptr(c, hook, ret)?;
                if event != 0x2091 {
                    break 'out;
                }
            }
            c.cycles += 1;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate of this module's fakes (in each test machine's RAM).
    const OC371_CTRL: u32 = DATA + 0xf_f000;
    const OC371_MAGIC: u32 = 0x0c37_1a7e;
    /// Scripted results: up to 16 `(target, v0)` pairs (v0 sign-extended).
    const OC371_TAB: u32 = OC371_CTRL + 0x10;
    /// One scripted sequence: target, cursor, then up to 8 values at +0x10.
    const OC371_SEQ: u32 = OC371_CTRL + 0x100;
    /// Scripted voice range: on, lo, hi.
    const OC371_RANGE: u32 = OC371_CTRL + 0x200;

    /// A logged fake (may unwind) whose `v0` comes from the script.
    fn oc371_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for i in 0..16 {
            if c.bus.read32(OC371_TAB + 8 * i) == target {
                c.g[V0][0] = sx(c.bus.read32(OC371_TAB + 8 * i + 4));
            }
        }
        for k in 0..2 {
            let seq = OC371_SEQ + 0x40 * k;
            if c.bus.read32(seq) == target {
                let n = c.bus.read32(seq + 4);
                c.bus.write32(seq + 4, n + 1);
                let last = c.bus.read32(seq + 8).saturating_sub(1);
                c.g[V0][0] = sx(c.bus.read32(seq + 0x10 + 4 * n.min(last)));
            }
        }
        // `SND_GetGroupVoiceRange(group, &lo, &hi)` answers when scripted.
        if target == 0x003c_0c18 && c.bus.read32(OC371_RANGE) != 0 {
            let (lo, hi) = (c.bus.read32(OC371_RANGE + 4), c.bus.read32(OC371_RANGE + 8));
            c.bus.write32(a1, lo);
            c.bus.write32(a2, hi);
        }
    }

    /// Script a sequence of `v0` for calls to `target` (the last value
    /// repeats; at most 8; two sequences).
    fn oc371_sequence(c: &mut Cpu, target: u32, values: &[u32]) {
        let mut seq = OC371_SEQ;
        if c.bus.read32(seq) != 0 && c.bus.read32(seq) != target {
            seq += 0x40;
        }
        c.bus.write32(seq, target);
        c.bus.write32(seq + 4, 0);
        c.bus.write32(seq + 8, values.len().min(8) as u32);
        for (i, v) in values.iter().take(8).enumerate() {
            c.bus.write32(seq + 0x10 + 4 * i as u32, *v);
        }
    }

    /// Script `v0` for calls to `target`.
    fn oc371_script(c: &mut Cpu, target: u32, v0: u32) {
        for i in 0..16 {
            let t = c.bus.read32(OC371_TAB + 8 * i);
            if t == 0 || t == target {
                c.bus.write32(OC371_TAB + 8 * i, target);
                c.bus.write32(OC371_TAB + 8 * i + 4, v0);
                return;
            }
        }
        panic!("oc371 script full");
    }

    /// Fake machine: the gate on, the fakes hooked.
    fn oc371_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc371_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC371_CTRL, OC371_MAGIC);
        // Calls to fake addresses (vtables, hooks) answer from the script too.
        c.hooks.miss = oc371_fake;
        r
    }

    macro_rules! oc371_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC371_CTRL) == OC371_MAGIC {
                    return oc371_fake(c, $addr);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    oc371_gated!(
        oc371_fake_check_reset,
        OC371_PREV_CHECK_RESET,
        0x0011_6120,
        ee::s00::f_00116120__generated
    );
    oc371_gated!(
        oc371_fake_tweak_layer,
        OC371_PREV_TWEAK_LAYER,
        0x0011_63b0,
        ee::s00::f_001163b0__generated
    );
    oc371_gated!(
        oc371_fake_handplant,
        OC371_PREV_HANDPLANT,
        0x0010_7578,
        ee::s00::f_00107578__generated
    );
    oc371_gated!(
        oc371_fake_railslide,
        OC371_PREV_RAILSLIDE,
        0x0010_6848,
        ee::s00::f_00106848__generated
    );
    oc371_gated!(
        oc371_fake_boost,
        OC371_PREV_BOOST,
        0x0011_4130,
        ee::s00::f_00114130__generated
    );
    oc371_gated!(
        oc371_fake_set_sub_state,
        OC371_PREV_SET_SUB_STATE,
        0x0011_fec8,
        ee::s00::f_0011fec8__generated
    );
    oc371_gated!(
        oc371_fake_atan,
        OC371_PREV_ATAN,
        0x0031_c228,
        ee::s10::f_0031c228__generated
    );
    oc371_gated!(
        oc371_fake_anim_id,
        OC371_PREV_ANIM_ID,
        0x0031_2aa0,
        ee::s10::f_00312aa0__generated
    );
    oc371_gated!(
        oc371_fake_anim_entry,
        OC371_PREV_ANIM_ENTRY,
        0x0031_1ae8,
        ee::s10::f_00311ae8__generated
    );
    oc371_gated!(
        oc371_fake_anim_play,
        OC371_PREV_ANIM_PLAY,
        0x0031_28e8,
        ee::s10::f_003128e8__generated
    );
    oc371_gated!(
        oc371_fake_mutex_lock,
        OC371_PREV_MUTEX_LOCK,
        0x003e_5700,
        ee::s14::f_003e5700__generated
    );
    oc371_gated!(
        oc371_fake_mutex_unlock,
        OC371_PREV_MUTEX_UNLOCK,
        0x003e_5760,
        ee::s14::f_003e5760__generated
    );
    oc371_gated!(
        oc371_fake_complete_request,
        OC371_PREV_COMPLETE_REQUEST,
        0x003d_e420,
        ee::s14::f_003de420__generated
    );
    oc371_gated!(
        oc371_fake_platform_open,
        OC371_PREV_PLATFORM_OPEN,
        0x003e_3208,
        ee::s14::f_003e3208__generated
    );
    oc371_gated!(
        oc371_fake_platform_close,
        OC371_PREV_PLATFORM_CLOSE,
        0x003e_32f8,
        ee::s14::f_003e32f8__generated
    );
    oc371_gated!(
        oc371_fake_platform_get_size,
        OC371_PREV_PLATFORM_GET_SIZE,
        0x003e_34e8,
        ee::s14::f_003e34e8__generated
    );
    oc371_gated!(
        oc371_fake_big_find_entry,
        OC371_PREV_BIG_FIND_ENTRY,
        0x003e_2768,
        ee::s14::f_003e2768__generated
    );
    oc371_gated!(
        oc371_fake_free_file_handle,
        OC371_PREV_FREE_FILE_HANDLE,
        0x003d_e8c0,
        ee::s14::f_003de8c0__generated
    );
    oc371_gated!(
        oc371_fake_platform_read,
        OC371_PREV_PLATFORM_READ,
        0x003e_3350,
        ee::s14::f_003e3350__generated
    );
    oc371_gated!(
        oc371_fake_snd_stop,
        OC371_PREV_SND_STOP,
        0x003b_89e0,
        ee::s13::f_003b89e0__generated
    );
    oc371_gated!(
        oc371_fake_next_voice_group,
        OC371_PREV_NEXT_VOICE_GROUP,
        0x003c_6158,
        ee::s13::f_003c6158__generated
    );
    oc371_gated!(
        oc371_fake_group_voice_range,
        OC371_PREV_GROUP_VOICE_RANGE,
        0x003c_0c18,
        ee::s13::f_003c0c18__generated
    );
    oc371_gated!(
        oc371_fake_random_range,
        OC371_PREV_RANDOM_RANGE,
        0x003c_72a0,
        ee::s13::f_003c72a0__generated
    );
    oc371_gated!(
        oc371_fake_midi_to_volume,
        OC371_PREV_MIDI_TO_VOLUME,
        0x003b_c570,
        ee::s13::f_003bc570__generated
    );
    oc371_gated!(
        oc371_fake_snd_random,
        OC371_PREV_SND_RANDOM,
        0x003b_c5c8,
        ee::s13::f_003bc5c8__generated
    );
    oc371_gated!(
        oc371_fake_calc_pitch,
        OC371_PREV_CALC_PITCH,
        0x003b_a938,
        ee::s13::f_003ba938__generated
    );
    oc371_gated!(
        oc371_fake_calc_send,
        OC371_PREV_CALC_SEND,
        0x003b_a740,
        ee::s13::f_003ba740__generated
    );
    oc371_gated!(
        oc371_fake_calc_volume,
        OC371_PREV_CALC_VOLUME,
        0x003b_a7b0,
        ee::s13::f_003ba7b0__generated
    );
    oc371_gated!(
        oc371_fake_start_voices,
        OC371_PREV_START_VOICES,
        0x003c_3ee0,
        ee::s13::f_003c3ee0__generated
    );
    oc371_gated!(
        oc371_fake_channel_release,
        OC371_PREV_CHANNEL_RELEASE,
        0x003b_a550,
        ee::s13::f_003ba550__generated
    );
    oc371_gated!(
        oc371_fake_snd_lock,
        OC371_PREV_SND_LOCK,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated
    );
    oc371_gated!(
        oc371_fake_snd_play,
        OC371_PREV_SND_PLAY,
        0x003b_6820,
        ee::s13::f_003b6820__generated
    );
    oc371_gated!(
        oc371_fake_snd_unlock,
        OC371_PREV_SND_UNLOCK,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc371_gated!(
        oc371_fake_add_instance,
        OC371_PREV_ADD_INSTANCE,
        0x002a_cfe0,
        ee::s08::f_002acfe0__generated
    );
    oc371_gated!(
        oc371_fake_set_bank_sample,
        OC371_PREV_SET_BANK_SAMPLE,
        0x002a_bce8,
        ee::s08::f_002abce8__generated
    );
    oc371_gated!(
        oc371_fake_add_timer,
        OC371_PREV_ADD_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated
    );
    oc371_gated!(
        oc371_fake_ring_in_range,
        OC371_PREV_RING_IN_RANGE,
        0x003d_fc08,
        ee::s14::f_003dfc08__generated
    );
    oc371_gated!(
        oc371_fake_free_request,
        OC371_PREV_FREE_REQUEST,
        0x003d_fe18,
        ee::s14::f_003dfe18__generated
    );
    oc371_gated!(
        oc371_fake_on_read,
        OC371_PREV_ON_READ,
        0x003e_0170,
        ee::s14::f_003e0170__generated
    );
    oc371_gated!(
        oc371_fake_queue_read,
        OC371_PREV_QUEUE_READ,
        0x003d_d648,
        ee::s14::f_003dd648__generated
    );
    oc371_gated!(
        oc371_fake_set_request_callback,
        OC371_PREV_SET_REQUEST_CALLBACK,
        0x003d_cf70,
        ee::s14::f_003dcf70__generated
    );
    oc371_gated!(
        oc371_fake_next_handle_serial,
        OC371_PREV_NEXT_HANDLE_SERIAL,
        0x003b_b0c8,
        ee::s13::f_003bb0c8__generated
    );
    oc371_gated!(
        oc371_fake_parse_next_tag,
        OC371_PREV_PARSE_NEXT_TAG,
        0x003c_7388,
        ee::s13::f_003c7388__generated
    );
    oc371_gated!(
        oc371_fake_fire_events,
        OC371_PREV_FIRE_EVENTS,
        0x003b_5f98,
        ee::s13::f_003b5f98__generated
    );
    oc371_gated!(
        oc371_fake_is_handle_done,
        OC371_PREV_IS_HANDLE_DONE,
        0x003b_8ab8,
        ee::s13::f_003b8ab8__generated
    );
    oc371_gated!(
        oc371_fake_handle_to_channel,
        OC371_PREV_HANDLE_TO_CHANNEL,
        0x003b_a6e8,
        ee::s13::f_003ba6e8__generated
    );
    oc371_gated!(
        oc371_fake_is_replay_playing,
        OC371_PREV_IS_REPLAY_PLAYING,
        0x0028_8ae0,
        ee::s07::f_00288ae0__generated
    );
    oc371_gated!(
        oc371_fake_find_rider,
        OC371_PREV_FIND_RIDER,
        0x0028_5d98,
        ee::s07::f_00285d98__generated
    );
    oc371_gated!(
        oc371_fake_stop_handle,
        OC371_PREV_STOP_HANDLE,
        0x002a_d5f0,
        ee::s08::f_002ad5f0__generated
    );
    oc371_gated!(
        oc371_fake_get_be,
        OC371_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc371_gated!(
        oc371_fake_get_interface,
        OC371_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc371_gated!(
        oc371_fake_surface_sound_type,
        OC371_PREV_SURFACE_SOUND_TYPE,
        0x0029_1710,
        ee::s07::f_00291710__generated
    );
    oc371_gated!(
        oc371_fake_channel_gain_ptr,
        OC371_PREV_CHANNEL_GAIN_PTR,
        0x0028_7968,
        ee::s07::f_00287968__generated
    );
    oc371_gated!(
        oc371_fake_listener_index,
        OC371_PREV_LISTENER_INDEX,
        0x0028_8a20,
        ee::s07::f_00288a20__generated
    );
    oc371_gated!(
        oc371_fake_update_air_state,
        OC371_PREV_UPDATE_AIR_STATE,
        0x0029_4678,
        ee::s07::f_00294678__generated
    );
    oc371_gated!(
        oc371_fake_alloc_resource,
        OC371_PREV_ALLOC_RESOURCE,
        0x003a_8a90,
        ee::s13::f_003a8a90__generated
    );
    oc371_gated!(
        oc371_fake_resolve_resource,
        OC371_PREV_RESOLVE_RESOURCE,
        0x003a_8cd0,
        ee::s13::f_003a8cd0__generated
    );
    oc371_gated!(
        oc371_fake_speech_volume_on,
        OC371_PREV_SPEECH_VOLUME_ON,
        0x0029_f0a0,
        ee::s08::f_0029f0a0__generated
    );
    oc371_gated!(
        oc371_fake_track_group4,
        OC371_PREV_TRACK_GROUP4,
        0x002a_3fe8,
        ee::s08::f_002a3fe8__generated
    );
    oc371_gated!(
        oc371_fake_instance_ready,
        OC371_PREV_INSTANCE_READY,
        0x002b_0e28,
        ee::s08::f_002b0e28__generated
    );
    oc371_gated!(
        oc371_fake_speech_mask,
        OC371_PREV_SPEECH_MASK,
        0x0029_f330,
        ee::s08::f_0029f330__generated
    );
    oc371_gated!(
        oc371_fake_find_view,
        OC371_PREV_FIND_VIEW,
        0x0028_8b40,
        ee::s07::f_00288b40__generated
    );
    oc371_gated!(
        oc371_fake_get_be_library,
        OC371_PREV_GET_BE_LIBRARY,
        0x0028_b1e8,
        ee::s07::f_0028b1e8__generated
    );
    oc371_gated!(
        oc371_fake_track_info,
        OC371_PREV_TRACK_INFO,
        0x0014_4bc0,
        ee::s01::f_00144bc0__generated
    );
    oc371_gated!(
        oc371_fake_get_character,
        OC371_PREV_GET_CHARACTER,
        0x0012_3128,
        ee::s00::f_00123128__generated
    );
    oc371_gated!(
        oc371_fake_set_event_context,
        OC371_PREV_SET_EVENT_CONTEXT,
        0x002b_1458,
        ee::s08::f_002b1458__generated
    );
    oc371_gated!(
        oc371_fake_make_id,
        OC371_PREV_MAKE_ID,
        0x003d_8008,
        ee::s14::f_003d8008__generated
    );
    //@FAKES@

    fn oc371_install_fakes() {
        use std::ptr::addr_of_mut;
        // SAFETY: override slots are plain fn pointers; `hook` serialises.
        {
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00116120),
                oc371_fake_check_reset,
                &OC371_PREV_CHECK_RESET,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_001163B0),
                oc371_fake_tweak_layer,
                &OC371_PREV_TWEAK_LAYER,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00107578),
                oc371_fake_handplant,
                &OC371_PREV_HANDPLANT,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00106848),
                oc371_fake_railslide,
                &OC371_PREV_RAILSLIDE,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00114130),
                oc371_fake_boost,
                &OC371_PREV_BOOST,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                oc371_fake_set_sub_state,
                &OC371_PREV_SET_SUB_STATE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_0031C228),
                oc371_fake_atan,
                &OC371_PREV_ATAN,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00312AA0),
                oc371_fake_anim_id,
                &OC371_PREV_ANIM_ID,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                oc371_fake_anim_entry,
                &OC371_PREV_ANIM_ENTRY,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003128E8),
                oc371_fake_anim_play,
                &OC371_PREV_ANIM_PLAY,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E5700),
                oc371_fake_mutex_lock,
                &OC371_PREV_MUTEX_LOCK,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E5760),
                oc371_fake_mutex_unlock,
                &OC371_PREV_MUTEX_UNLOCK,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DE420),
                oc371_fake_complete_request,
                &OC371_PREV_COMPLETE_REQUEST,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E3208),
                oc371_fake_platform_open,
                &OC371_PREV_PLATFORM_OPEN,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
                oc371_fake_platform_close,
                &OC371_PREV_PLATFORM_CLOSE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
                oc371_fake_platform_get_size,
                &OC371_PREV_PLATFORM_GET_SIZE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E2768),
                oc371_fake_big_find_entry,
                &OC371_PREV_BIG_FIND_ENTRY,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DE8C0),
                oc371_fake_free_file_handle,
                &OC371_PREV_FREE_FILE_HANDLE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E3350),
                oc371_fake_platform_read,
                &OC371_PREV_PLATFORM_READ,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B89E0),
                oc371_fake_snd_stop,
                &OC371_PREV_SND_STOP,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003C6158),
                oc371_fake_next_voice_group,
                &OC371_PREV_NEXT_VOICE_GROUP,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003C0C18),
                oc371_fake_group_voice_range,
                &OC371_PREV_GROUP_VOICE_RANGE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003C72A0),
                oc371_fake_random_range,
                &OC371_PREV_RANDOM_RANGE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BC570),
                oc371_fake_midi_to_volume,
                &OC371_PREV_MIDI_TO_VOLUME,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BC5C8),
                oc371_fake_snd_random,
                &OC371_PREV_SND_RANDOM,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BA938),
                oc371_fake_calc_pitch,
                &OC371_PREV_CALC_PITCH,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BA740),
                oc371_fake_calc_send,
                &OC371_PREV_CALC_SEND,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BA7B0),
                oc371_fake_calc_volume,
                &OC371_PREV_CALC_VOLUME,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003C3EE0),
                oc371_fake_start_voices,
                &OC371_PREV_START_VOICES,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BA550),
                oc371_fake_channel_release,
                &OC371_PREV_CHANNEL_RELEASE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
                oc371_fake_snd_lock,
                &OC371_PREV_SND_LOCK,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B6820),
                oc371_fake_snd_play,
                &OC371_PREV_SND_PLAY,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
                oc371_fake_snd_unlock,
                &OC371_PREV_SND_UNLOCK,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ACFE0),
                oc371_fake_add_instance,
                &OC371_PREV_ADD_INSTANCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ABCE8),
                oc371_fake_set_bank_sample,
                &OC371_PREV_SET_BANK_SAMPLE,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
                oc371_fake_add_timer,
                &OC371_PREV_ADD_TIMER,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DFC08),
                oc371_fake_ring_in_range,
                &OC371_PREV_RING_IN_RANGE,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DFE18),
                oc371_fake_free_request,
                &OC371_PREV_FREE_REQUEST,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003E0170),
                oc371_fake_on_read,
                &OC371_PREV_ON_READ,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DD648),
                oc371_fake_queue_read,
                &OC371_PREV_QUEUE_READ,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
                oc371_fake_set_request_callback,
                &OC371_PREV_SET_REQUEST_CALLBACK,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BB0C8),
                oc371_fake_next_handle_serial,
                &OC371_PREV_NEXT_HANDLE_SERIAL,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003C7388),
                oc371_fake_parse_next_tag,
                &OC371_PREV_PARSE_NEXT_TAG,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B5F98),
                oc371_fake_fire_events,
                &OC371_PREV_FIRE_EVENTS,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003B8AB8),
                oc371_fake_is_handle_done,
                &OC371_PREV_IS_HANDLE_DONE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
                oc371_fake_handle_to_channel,
                &OC371_PREV_HANDLE_TO_CHANNEL,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
                oc371_fake_is_replay_playing,
                &OC371_PREV_IS_REPLAY_PLAYING,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00285D98),
                oc371_fake_find_rider,
                &OC371_PREV_FIND_RIDER,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AD5F0),
                oc371_fake_stop_handle,
                &OC371_PREV_STOP_HANDLE,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                oc371_fake_get_be,
                &OC371_PREV_GET_BE,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                oc371_fake_get_interface,
                &OC371_PREV_GET_INTERFACE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00291710),
                oc371_fake_surface_sound_type,
                &OC371_PREV_SURFACE_SOUND_TYPE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00287968),
                oc371_fake_channel_gain_ptr,
                &OC371_PREV_CHANNEL_GAIN_PTR,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00288A20),
                oc371_fake_listener_index,
                &OC371_PREV_LISTENER_INDEX,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00294678),
                oc371_fake_update_air_state,
                &OC371_PREV_UPDATE_AIR_STATE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A8A90),
                oc371_fake_alloc_resource,
                &OC371_PREV_ALLOC_RESOURCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A8CD0),
                oc371_fake_resolve_resource,
                &OC371_PREV_RESOLVE_RESOURCE,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_0029F0A0),
                oc371_fake_speech_volume_on,
                &OC371_PREV_SPEECH_VOLUME_ON,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002A3FE8),
                oc371_fake_track_group4,
                &OC371_PREV_TRACK_GROUP4,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B0E28),
                oc371_fake_instance_ready,
                &OC371_PREV_INSTANCE_READY,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_0029F330),
                oc371_fake_speech_mask,
                &OC371_PREV_SPEECH_MASK,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00288B40),
                oc371_fake_find_view,
                &OC371_PREV_FIND_VIEW,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B1E8),
                oc371_fake_get_be_library,
                &OC371_PREV_GET_BE_LIBRARY,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
                oc371_fake_track_info,
                &OC371_PREV_TRACK_INFO,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00123128),
                oc371_fake_get_character,
                &OC371_PREV_GET_CHARACTER,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B1458),
                oc371_fake_set_event_context,
                &OC371_PREV_SET_EVENT_CONTEXT,
            );
            fakes::hook(
                addr_of_mut!(ee::s14::OVERRIDE_003D8008),
                oc371_fake_make_id,
                &OC371_PREV_MAKE_ID,
            );
            //@HOOKS@
        }
    }

    /// `f` (bit pattern) or zero, or its negation, by `k`.
    fn oc371_pick_f(r: &mut Rng, k: u32) -> u32 {
        match k % 4 {
            0 => 0,
            1 => r.f32(1.0),
            2 => r.f32(0.2),
            _ => r.edge_f32(),
        }
    }

    /// Every exit (reset path, tweak layer, handplant, railslide), the
    /// hold / id bookkeeping (new hold, other id, no id), the stick in all
    /// quadrants and on the axes, the steer timer (negative, short, long)
    /// and heading, and the animation choice (idle 0x11f with / without
    /// play and landing entry 9, lean 9 / 10, same as current). The
    /// callees are logged fakes with scripted answers, one may unwind.
    #[test]
    fn rider_natural_air_sub_handle_input_matches() {
        oc371_install_fakes();
        covered(
            ee::s00::f_0012f730__generated,
            rider_natural_air_sub_handle_input,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x0012_f730, 8);
                c.pc = 0x0012_f730;
                let me = DATA;
                let input = DATA + 0x100;
                let rider = DATA + 0x1000;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(input);
                // .lit4: 1/31, -pi/2, pi/2, pi, pi, 1/2pi, 2pi, pi/2, 0.05.
                put_words(
                    c,
                    GAME_GP - 0x7704,
                    &[
                        0x3d04_2108,
                        0xbfc9_0fdb,
                        0x3fc9_0fdb,
                        0x4049_0fdb,
                        0x4049_0fdb,
                        0x3e22_f983,
                        0x40c9_0fdb,
                        0x3fc9_0fdb,
                        0x3d4c_cccd,
                    ],
                );
                let k = r.next_u32();
                // Fake answers.
                oc371_script(c, 0x0011_6120, (k % 10 == 0) as u32);
                oc371_script(c, 0x0011_63b0, (k / 10 % 10 == 0) as u32);
                oc371_script(c, 0x0010_7578, (k / 100 % 5 == 0) as u32);
                oc371_script(c, 0x0010_6848, (k / 500 % 10 == 0) as u32);
                let ids = [0x11f, 9, 10, r.next_u32() % 0x200];
                oc371_script(c, 0x0031_2aa0, ids[(r.next_u32() % 4) as usize]);
                oc371_script(
                    c,
                    0x0031_1ae8,
                    if r.next_u32() % 4 == 0 {
                        9
                    } else {
                        r.next_u32() % 16
                    },
                );
                // The input: buttons, stick x (bits 24-29) and y (bits 0-5).
                let mut w = r.next_u32();
                if r.next_u32() % 4 == 0 {
                    w &= !0x3f00_0000;
                }
                let mut w2 = r.next_u32();
                if r.next_u32() % 4 == 0 {
                    w2 &= !0x3f;
                }
                c.bus.write32(input, w);
                c.bus.write32(input + 4, w2);
                let stick_id = match r.next_u32() % 4 {
                    0 => 0xff,
                    1 => 3,
                    _ => r.next_u32() & 0xff,
                };
                c.bus.write8(input + 2, stick_id as u8);
                // The sub state.
                c.bus.write32(me + NAIR_HEADING, r.f32(4.0));
                let timer = match r.next_u32() % 4 {
                    0 => (-1.0f32).to_bits(),
                    1 => r.f32(0.6),
                    _ => r.f32(3.0),
                };
                c.bus.write32(me + NAIR_TIMER, timer);
                c.bus.write32(me + NAIR_TWEAK, r.next_u32() % 2);
                c.bus.write32(me + NAIR_HOLD, r.next_u32() % 2);
                let id = match r.next_u32() % 3 {
                    0 => u32::MAX,
                    1 => 3,
                    _ => r.next_u32() % 8,
                };
                c.bus.write32(me + NAIR_ID, id);
                c.bus.write32(me + NAIR_RIDER, rider);
                // The rider: steer fields (zero for the idle path) and its
                // animation controller.
                fill(c, &mut r, rider + 0x1f0, 0x10);
                let z = r.next_u32();
                for (i, off) in [0x1f0, 0x214, 0x220].into_iter().enumerate() {
                    let v = if z >> i & 3 != 0 {
                        0
                    } else {
                        oc371_pick_f(&mut r, z >> 8)
                    };
                    c.bus.write32(rider + off, v);
                }
                c.bus.write32(rider + NAIR_RIDER_ANIM, DATA + 0x3000);
            },
        );
    }
    /// A NUL-terminated string at `at`.
    fn oc371_put_str(c: &mut Cpu, at: u32, text: &str) {
        for (i, b) in text.bytes().chain([0]).enumerate() {
            c.bus.write8(at + i as u32, b);
        }
    }

    /// Queueing (empty queue, in front, behind several) and serving: busy,
    /// paused, priority too high, cancelled, every command but write (5:
    /// its real `EAFile_PlatformWrite` reaches the sceFs semaphores) and
    /// out-of-range ones. Open / exists with plain paths (open flag on /
    /// off, open fails or not), `big|name` and `|name` searched through
    /// big files (named match or not, found or not). libc runs for real;
    /// the EAFile callees are logged fakes (one may unwind).
    #[test]
    fn eafile_service_queue_matches() {
        oc371_install_fakes();
        covered(
            ee::s14::f_003ddfa8__generated,
            eafile_service_queue,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003d_dfa8, 6);
                c.pc = 0x003d_dfa8;
                put_words(
                    c,
                    EAFS_JUMP_TABLE,
                    &[
                        0x003d_e108,
                        0x003d_e308,
                        0x003d_e350,
                        0x003d_e3b4,
                        0x003d_e3d4,
                        0x003d_e3f0,
                        0x003d_e108,
                        0x003d_e3f0,
                        0x003d_e3f0,
                    ],
                );
                oc371_script(c, 0x003e_3208, (r.next_u32() % 2) as u32);
                oc371_script(c, 0x003e_2768, (r.next_u32() % 3 == 0) as u32);
                let st = EAFS_STATE;
                let n_queued = r.next_u32() % 4;
                let mut prio = r.next_u32() % 3;
                let cmds = [2, 2, 8, 8, 3, 3, 4, 4, 6, 7, 9, 10, 0, 12];
                let names = ["file.dat", "big|name", "|name", "other|name", "|"];
                let mut prev = 0u32;
                for i in 0..n_queued + 1 {
                    let req = DATA + 0x200 + 0x100 * i;
                    fill(c, &mut r, req, 0x10);
                    let cmd = cmds[(r.next_u32() % cmds.len() as u32) as usize];
                    let w = r.next_u32() & !0x00f0_0000 | cmd << 20;
                    c.bus.write32(req, w);
                    c.bus.write32(req + 4, (r.next_u32() % 8 == 0) as u32);
                    c.bus.write32(req + 0x18, r.next_u32() % 4);
                    c.bus.write32(req + 0x1c, r.next_u32() % 3);
                    prio += r.next_u32() % 3;
                    c.bus.write32(req + EAFS_REQ_PRIORITY, prio);
                    // The file handle and its path.
                    let fh = DATA + 0x1000 + 0x100 * i;
                    let handle = if r.next_u32() % 5 == 0 { 0 } else { fh };
                    c.bus.write32(req + EAFS_REQ_HANDLE, handle);
                    c.bus.write32(fh, DATA + 0x3000);
                    c.bus.write32(fh + 4, r.next_u32());
                    c.bus.write32(
                        fh + 8,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32() % 0x1000
                        },
                    );
                    c.bus.write32(fh + 0xc, r.next_u32());
                    let name = names[(r.next_u32() % names.len() as u32) as usize];
                    oc371_put_str(c, fh + 0x10, name);
                    c.bus.write32(req + EAFS_REQ_NEXT, 0);
                    if i < n_queued {
                        if prev == 0 {
                            c.bus.write32(st + 0x30, req);
                        } else {
                            c.bus.write32(prev + EAFS_REQ_NEXT, req);
                        }
                        prev = req;
                    }
                }
                if n_queued == 0 {
                    c.bus.write32(st + 0x30, 0);
                }
                // The new request (last one built), or none.
                let new_req = DATA + 0x200 + 0x100 * n_queued;
                c.bus.write32(new_req + EAFS_REQ_PRIORITY, r.next_u32() % 6);
                c.g[A0][0] = if r.next_u32() % 5 == 0 {
                    0
                } else {
                    sx(new_req)
                };
                c.bus.write32(DATA + 0x3000 + 0xc, DATA + 0x3100);
                c.bus.write32(st + 0x8, r.next_u32() % 8);
                c.bus.write32(st + 0xc, r.next_u32() % 8);
                c.bus.write32(st + 0x10, (r.next_u32() % 8 == 0) as u32);
                c.bus
                    .write32(st + 0x24, if r.next_u32() % 8 == 0 { DATA } else { 0 });
                // The open big files, named "big" or "other".
                let n_big = r.next_u32() % 4;
                let mut next = 0;
                for i in (0..n_big).rev() {
                    let big = DATA + 0x2000 + 0x40 * i;
                    c.bus.write32(big, r.next_u32());
                    c.bus.write32(big + 0xc, DATA + 0x2800 + 0x40 * i);
                    let bn = if r.next_u32() % 2 == 0 {
                        "big"
                    } else {
                        "other"
                    };
                    oc371_put_str(c, DATA + 0x2810 + 0x40 * i, bn);
                    c.bus.write32(big + 0x14, next);
                    next = big;
                }
                c.bus.write32(st + 0x34, next);
            },
        );
    }
    /// Up to four voices out of a random window of 16: free ones (in use
    /// or not, reserved by an earlier pick, older / newer), then stolen
    /// ones by priority (lower, equal with the system flag on / off, too
    /// high, >= 0x65) and age; all found or not (-9); the sort, the serial
    /// wrap, stopping stolen voices (own handle or the master's). The real
    /// `SND_IsVoiceReserved` runs; `SND_Stop` is a logged fake (may unwind).
    #[test]
    fn snd_alloc_voices_matches() {
        oc371_install_fakes();
        crate::difftest::check(
            ee::s13::f_003ba0b0__generated,
            snd_alloc_voices,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003b_a0b0, 4);
                c.pc = 0x003b_a0b0;
                let voices = DATA + 0x1000;
                let n = match seed % 7 {
                    6 => u32::MAX,
                    k => k.min(4),
                };
                c.g[A0][0] = sx(n);
                c.g[A1][0] = (r.next_u32() % 0x70) as u64;
                c.g[A2][0] = sx(DATA + 0x800);
                let lo = r.next_u32() % 4;
                c.g[A3][0] = lo as u64;
                c.g[T0][0] = (lo + r.next_u32() % 13) as u64;
                c.bus.write8(SNDV_SYSTEM + 0x3c, (r.next_u32() % 2) as u8);
                c.bus.write32(SNDV_SYSTEM + 0x180, r.next_u32());
                c.bus.write32(SNDV_SYSTEM + 0x1f0, voices);
                let serial = if r.next_u32() % 4 == 0 {
                    0x7fff_ff40
                } else {
                    r.next_u32() & 0xffff_ff00
                };
                c.bus.write32(SNDV_SERIAL, serial);
                for i in 0..8 {
                    c.bus
                        .write16(SNDV_SELECTED + 2 * i, (r.next_u32() % 16) as u16);
                }
                let free_odds = 1 + r.next_u32() % 4;
                for i in 0..16 {
                    let v = voices + SNDV_STRIDE * i;
                    fill(c, &mut r, v, SNDV_STRIDE / 4);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(v, r.next_u32() | 0x8000_0000);
                    }
                    c.bus.write16(v + 0x28, (r.next_u32() % 16) as u16);
                    let prio = match r.next_u32() % 4 {
                        0 => 0x65 + r.next_u32() % 0x20,
                        _ => r.next_u32() % 0x70,
                    };
                    c.bus.write8(v + 0x2a, prio as u8);
                    c.bus.write32(v + 0x2c, r.next_u32() % 6);
                    c.bus
                        .write8(v + 0x69, (r.next_u32() % free_odds != 0) as u8);
                }
            },
        );
    }
    /// The sound system for a real `SND_AllocVoices`: 16 voices at
    /// `voices`, some free.
    fn oc371_voice_system(c: &mut Cpu, r: &mut Rng, voices: u32) {
        c.bus.write8(SNDV_SYSTEM + 0x3c, (r.next_u32() % 2) as u8);
        c.bus.write32(SNDV_SYSTEM + 0x180, r.next_u32());
        c.bus.write32(SNDV_SYSTEM + 0x1f0, voices);
        c.bus.write32(SNDV_SERIAL, r.next_u32() & 0xffff_ff00);
        for i in 0..8 {
            c.bus
                .write16(SNDV_SELECTED + 2 * i, (r.next_u32() % 16) as u16);
        }
        let free_odds = 1 + r.next_u32() % 4;
        for i in 0..16 {
            let v = voices + SNDV_STRIDE * i;
            fill(c, r, v, SNDV_STRIDE / 4);
            c.bus.write16(v + 0x28, (r.next_u32() % 16) as u16);
            c.bus.write8(v + 0x2a, (r.next_u32() % 0x70) as u8);
            c.bus.write32(v + 0x2c, r.next_u32() % 6);
            c.bus
                .write8(v + 0x69, (r.next_u32() % free_odds != 0) as u8);
        }
    }

    /// Up to three voice groups (the real `SND_AllocVoices` finds voices or
    /// not), every random spread on / off, volume and pan clamps, detune,
    /// negative envelope length, LFO phases, 0-3 voices copied, starting
    /// failing (release, next group) or not. The other callees are logged
    /// fakes (one may unwind).
    #[test]
    fn snd_play_patch_voices_matches() {
        oc371_install_fakes();
        covered(
            ee::s13::f_003bb140__generated,
            snd_play_patch_voices,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003b_b140, 10);
                c.pc = 0x003b_b140;
                let note = DATA + 0x100;
                let patch = DATA + 0x200;
                let env = DATA + 0x400;
                oc371_voice_system(c, &mut r, DATA + 0x1000);
                let groups = r.next_u32() % 4;
                let seq: Vec<u32> = (0..groups).map(|i| 0x10 + i).chain([0]).collect();
                oc371_sequence(c, 0x003c_6158, &seq);
                oc371_script(
                    c,
                    0x003c_3ee0,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0xffff_fff0
                    },
                );
                oc371_script(c, 0x003c_72a0, (r.next_u32() % 0x100).wrapping_sub(0x80));
                oc371_script(c, 0x003b_c5c8, r.next_u32());
                // The voice range [lo, hi) GetGroupVoiceRange would give.
                let frame = crate::difftest::SP - 0xd0;
                let lo = r.next_u32() % 6;
                c.bus.write32(frame, lo);
                c.bus.write32(frame + 4, lo + r.next_u32() % 11);
                fill(c, &mut r, note, 8);
                fill(c, &mut r, patch, 0x30);
                c.bus.write8(patch + 3, (r.next_u32() % 4) as u8);
                c.bus.write8(patch + 0xc, (r.next_u32() % 0x70) as u8);
                for off in [0x12, 0x15, 0x1c, 0x1e] {
                    if r.next_u32() % 2 == 0 {
                        c.bus.write8(patch + off, 0);
                    }
                }
                if r.next_u32() % 2 == 0 {
                    c.bus.write16(patch + 6, 0);
                }
                c.bus.write32(patch + 0x28, env);
                let len = match r.next_u32() % 3 {
                    0 => r.next_u32() | 0x8000_0000,
                    _ => 1 + r.next_u32() % 0x1000,
                };
                c.bus.write32(env, len);
                c.bus.write32(env + 4, r.next_u32());
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(note);
                c.g[T0][0] = sx(patch);
                c.g[T1][0] = (r.next_u32() % 0x80) as u64;
                c.g[T2][0] = (r.next_u32() % 0x80) as u64;
                c.g[T3][0] = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }
    /// Muted, unscaled and scaled volumes (below 0, above 127, in range;
    /// the clamp's three exits), banks loaded or not, play failing (with
    /// and without the keep flag) or not, the instance registered or not,
    /// 0-3 echoes fading out or cut by their count. Callees are logged
    /// fakes (one may unwind).
    #[test]
    fn audio_play_pushed_sound_matches() {
        oc371_install_fakes();
        covered(
            ee::s07::f_002906b8__generated,
            audio_play_pushed_sound,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x0029_06b8, 8);
                c.pc = 0x0029_06b8;
                let p = DATA;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sx(p);
                c.bus.write32(p, sys);
                c.bus.write32(p + 4, DATA + 0x200);
                c.bus.write32(p + 8, DATA + 0x300);
                let d = r.next_u32() % 4;
                c.bus.write32(sys + PUSHED_DEPTH, d);
                for (off, at) in [
                    (0x210, 0x400),
                    (0x214, 0x500),
                    (0x1f8, 0x600),
                    (0x248, 0x700),
                    (0x254, 0x800),
                    (0xacc, 0x900),
                ] {
                    c.bus.write32(sys + off, DATA + at);
                }
                for i in 0..4 {
                    c.bus
                        .write32(sys + 0x1fc + 4 * i, (r.next_u32() % 4 == 0) as u32);
                    c.bus.write32(DATA + 0x200 + 4 * i, r.next_u32() % 4);
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                    let vol = match r.next_u32() % 4 {
                        0 => (r.next_u32() % 50).wrapping_neg(),
                        1 => 127 + r.next_u32() % 100,
                        _ => r.next_u32() % 127,
                    };
                    c.bus.write32(DATA + 0x400 + 4 * i, vol);
                    let scale = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x580 + 4 * i
                    };
                    c.bus.write32(DATA + 0x500 + 4 * i, scale);
                    c.bus.write32(DATA + 0x580 + 4 * i, r.f32(2.0));
                    let e = DATA + 0x700 + 0x10 * i;
                    c.bus.write32(e, r.next_u32() % 30);
                    c.bus.write32(e + 4, r.next_u32() % 4);
                    c.bus.write32(e + 8, r.next_u32() % 120);
                    c.bus.write32(e + 0xc, r.next_u32() % 150);
                    c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32() % 2);
                    let b = DATA + 0x900 + 0x60 * i;
                    c.bus.write32(
                        b,
                        if r.next_u32() % 2 == 0 {
                            1
                        } else {
                            r.next_u32() % 4
                        },
                    );
                    c.bus.write32(b + 4, r.next_u32());
                }
                put_words(c, PUSHED_ECHO_PERIOD, &[r.next_u32(), r.next_u32()]);
                oc371_script(
                    c,
                    0x003b_6820,
                    if r.next_u32() % 3 == 0 {
                        0xffff_fff0
                    } else {
                        5
                    },
                );
                oc371_script(
                    c,
                    0x002a_cfe0,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0xc00
                    },
                );
            },
        );
    }
    /// Markers at the read pointer (wrap, skip, none, empty ring); 0-2
    /// finished requests freed (none, in use, overlapping); room before
    /// the read pointer, at the end, or after moving the pending data
    /// (aligned or not, memory source or not, fitting or not); filling from
    /// memory (clipped at the source's end or not) or queueing a file read
    /// (queued or refused). The real `EA_MemCopy` runs; the other callees
    /// are logged fakes (one may unwind).
    #[test]
    fn streamfile_issue_read_matches() {
        oc371_install_fakes();
        covered(
            ee::s14::f_003e03d0__generated,
            streamfile_issue_read,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003e_03d0, 6);
                c.pc = 0x003e_03d0;
                let sf = DATA;
                let ring = DATA + 0x1_0000;
                let end = ring + 0x1000;
                c.g[A0][0] = sx(sf);
                c.g[A1][0] = r.next_u32() as u64 & 0xff;
                fill(c, &mut r, sf, 0x60);
                let start = ring + (r.next_u32() % 4) * 0x10;
                c.bus.write32(sf + 0x2c, ring);
                c.bus.write32(sf + 0x30, start);
                c.bus.write32(sf + 0x34, end);
                // Write pointer, pending data before it, read pointer anywhere.
                let wr = ring + 0x100 + (r.next_u32() % 0xe00 & !0xf);
                let used = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 0x100
                };
                let pend = wr - used;
                let rd = match r.next_u32() % 4 {
                    0 => pend,
                    1 => wr + 0x10 + (r.next_u32() % 0x200 & !0xf),
                    _ => start + 0x10 + (r.next_u32() % 0xc0 & !0xf),
                };
                c.bus.write32(sf + 0x50, rd);
                c.bus.write32(sf + 0x54, pend);
                c.bus.write32(sf + 0x58, wr);
                c.bus.write32(start, 0x1234);
                if rd != pend {
                    match r.next_u32() % 3 {
                        0 => c.bus.write32(rd, u32::MAX),
                        1 => {
                            c.bus.write32(rd, -2i32 as u32);
                            c.bus.write32(rd + 4, 0x20);
                            c.bus.write32(
                                rd + 0x20,
                                if r.next_u32() % 2 == 0 { u32::MAX } else { 7 },
                            );
                        }
                        _ => c.bus.write32(rd, 5),
                    }
                }
                c.bus
                    .write32(pend, if r.next_u32() % 2 == 0 { u32::MAX } else { 9 });
                // The request list.
                c.bus.write32(sf + 0x5c, DATA + 0x200);
                let req = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    DATA + 0x300
                };
                c.bus.write32(DATA + 0x200 + 0xc, req);
                c.bus
                    .write32(DATA + 0x300 + 4, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(DATA + 0x300 + 0x120, r.next_u32());
                let frees = r.next_u32() % 3;
                let seq: Vec<u32> = (0..frees).map(|_| 0).chain([1]).collect();
                oc371_sequence(c, 0x003d_fc08, &seq);
                // The source.
                let src = DATA + 0x400;
                c.bus.write32(sf + 0x60, src);
                c.bus
                    .write32(src + 0x10, if r.next_u32() % 2 == 0 { 1 } else { 2 });
                c.bus.write32(src + 0x114, DATA + 0x2_0000);
                let pos = r.next_u32() % 0x800;
                c.bus.write32(sf + 0x170, pos);
                c.bus.write32(src + 0x118, pos + r.next_u32() % 0x1000);
                c.bus.write32(sf + 0x17c, r.next_u32() % 0x900);
                oc371_script(
                    c,
                    0x003d_d648,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x500
                    },
                );
            },
        );
    }
    /// One or two patch layers (the real `SND_ParsePatch` with its tag
    /// reader faked: a `0xfe` "more" tag, then the end), key / velocity in
    /// or out of the layer ranges, the real `SND_PlayPatchVoices` (and
    /// `SND_AllocVoices`) succeeding or failing (stop what started), the
    /// handles done or not, one or several kept (channels found or not).
    /// The other callees are logged fakes (one may unwind).
    #[test]
    fn snd_play_patch_matches() {
        oc371_install_fakes();
        crate::difftest::check(
            ee::s13::f_003bb588__generated,
            snd_play_patch,
            300,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003b_b588, 12);
                c.pc = 0x003b_b588;
                let note = DATA + 0x100;
                oc371_voice_system(c, &mut r, DATA + 0x1000);
                c.bus.write32(SNDPP_VOICES, DATA + 0x1000);
                // The tag word `SND_ParsePatch` reads (its frame + 0x304).
                c.bus.write32(crate::difftest::SP - 0x26c, 0xfe);
                let layers = if r.next_u32() % 2 == 0 {
                    vec![0]
                } else {
                    vec![1, 0]
                };
                oc371_sequence(c, 0x003c_7388, &layers);
                let groups: Vec<u32> = match r.next_u32() % 4 {
                    0 => vec![0],
                    1 => vec![0x10, 0],
                    2 => vec![0x10, 0x11, 0, 0x12, 0],
                    _ => vec![0x10, 0, 0x11, 0],
                };
                oc371_sequence(c, 0x003c_6158, &groups);
                c.bus.write32(OC371_RANGE, 1);
                let lo = r.next_u32() % 6;
                c.bus.write32(OC371_RANGE + 4, lo);
                c.bus.write32(OC371_RANGE + 8, lo + r.next_u32() % 11);
                oc371_script(
                    c,
                    0x003c_3ee0,
                    if r.next_u32() % 3 == 0 {
                        0xffff_fff0
                    } else {
                        0
                    },
                );
                oc371_script(c, 0x003b_8ab8, (r.next_u32() % 3 == 0) as u32);
                oc371_script(
                    c,
                    0x003b_a6e8,
                    if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 16
                    },
                );
                fill(c, &mut r, note, 8);
                let key = |r: &mut Rng| match r.next_u32() % 4 {
                    0 => 0x80 + r.next_u32() % 0x80,
                    _ => r.next_u32() % 0x80,
                };
                c.bus.write8(note + 2, key(&mut r) as u8);
                c.bus.write8(note + 3, key(&mut r) as u8);
                c.g[A0][0] = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A1][0] = if r.next_u32() % 8 == 0 {
                    0
                } else {
                    sx(DATA + 0x200)
                };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = sx(note);
            },
        );
    }
    /// The audio system's push stack for `SSXAudioSystem_PlayTakeoffSound`
    /// and a real `Audio_PlayPushedSound`: `me` (`[0]` the stack, `[1]`
    /// banks, `[2]` samples, `+0x118` the push state), the stack's arrays.
    fn oc371_push_stack(c: &mut Cpu, r: &mut Rng, me: u32) {
        let stack = me + 0x1000;
        c.bus.write32(me, stack);
        c.bus.write32(me + 4, me + 0x200);
        c.bus.write32(me + 8, me + 0x300);
        c.bus.write32(me + 0x118, me + 0x80);
        c.bus.write32(me + 0x80, stack);
        fill(c, r, stack + 0x1d8, 0x30);
        c.bus.write32(stack + PUSHED_DEPTH, r.next_u32() % 3);
        for (k, off) in [
            0x1f8u32, 0x20c, 0x210, 0x214, 0x238, 0x23c, 0x240, 0x244, 0x248, 0x24c, 0x250, 0x254,
            0x258, 0x25c, 0xacc,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(stack + off, me + 0x2000 + 0x100 * k as u32);
        }
        fill(c, r, me + 0x2000, 0x400);
        fill(c, r, me + 0x300, 0x10);
        for i in 0..8 {
            c.bus.write32(me + 0x200 + 4 * i, r.next_u32() % 4);
        }
    }

    /// Replay or live, the local player or not (no listener), the rider
    /// found or not; speed below / inside / above the volume range, the old
    /// takeoff sound done or stopped, network mode (local player in a
    /// replay or not), the pushed level with its volume clamp and the real
    /// `Audio_PlayPushedSound`. Other callees are logged fakes (one may
    /// unwind).
    #[test]
    fn ssxaudio_play_takeoff_sound_matches() {
        oc371_install_fakes();
        covered(
            ee::s07::f_00294170__generated,
            ssxaudio_play_takeoff_sound,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x0029_4170, 14);
                c.pc = 0x0029_4170;
                let me = DATA;
                let rider = DATA + 0x8000;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(rider);
                oc371_push_stack(c, &mut r, me);
                c.bus.write32(me + TAKEOFF_HANDLE, r.next_u32());
                let speed = match r.next_u32() % 4 {
                    0 => r.f32(0.5),
                    1 => (1.0f32 + (r.next_u32() % 100) as f32 / 100.0).to_bits(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(rider + TAKEOFF_RIDER_SPEED, speed);
                c.bus.write32(rider + 0x870, r.next_u32() % 4);
                c.bus.write32(rider + 0x874, r.next_u32() % 2);
                c.bus.write32(rider + 0x87c, r.next_u32() % 2);
                c.bus
                    .write8(TAKEOFF_GAME_MODE, if r.next_u32() % 2 == 0 { 2 } else { 1 });
                oc371_script(c, 0x0028_8ae0, (r.next_u32() % 3 == 0) as u32);
                oc371_script(
                    c,
                    0x0028_5d98,
                    if r.next_u32() % 5 == 0 { 0 } else { rider },
                );
                oc371_script(c, 0x003b_8ab8, r.next_u32() % 2);
                oc371_script(c, 0x0029_1710, r.next_u32() % 6);
                oc371_script(
                    c,
                    0x003b_6820,
                    if r.next_u32() % 3 == 0 {
                        0xffff_fff0
                    } else {
                        7
                    },
                );
            },
        );
    }
    /// A stream of chunks (sizes 0-20) in 0-40 bytes: a split header kept
    /// from before (0-7 bytes) or none, a chunk in progress (copying or
    /// skipping, empty ones resolving) or a pending retry; the cache
    /// answering "later", no space or space; chunks running out; a
    /// trailing partial header kept. The real `EA_MemCopy` copies; the
    /// cache callees are logged fakes (one may unwind).
    #[test]
    fn cstreamman_parse_chunks_matches() {
        oc371_install_fakes();
        covered(
            ee::s13::f_003a7238__generated,
            cstreamman_parse_chunks,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003a_7238, 6);
                c.pc = 0x003a_7238;
                let me = DATA;
                let stream = DATA + 0x1000;
                fill(c, &mut r, me, 0x38);
                c.bus.write32(me + SMPC_REMAINING, 1 + r.next_u32() % 4);
                c.bus.write32(
                    me + SMPC_PARTIAL,
                    if r.next_u32() % 3 == 0 {
                        r.next_u32() % 8
                    } else {
                        0
                    },
                );
                c.bus
                    .write32(me + SMPC_RETRY, (r.next_u32() % 5 == 0) as u32);
                c.bus.write32(
                    me + SMPC_SIZE,
                    if r.next_u32() % 3 == 0 {
                        r.next_u32() % 16
                    } else {
                        0
                    },
                );
                c.bus.write32(me + 0xc4, r.next_u32() % 2);
                c.bus.write32(me + 0xc8, r.next_u32() % 2);
                c.bus.write32(me + 0xd0, DATA + 0x8000);
                // The stream: headers (id, 24-bit size, key) and their bytes.
                let mut at = stream;
                while at < stream + 0x40 {
                    let size = r.next_u32() % 21;
                    c.bus.write8(at, r.next_u32() as u8);
                    c.bus.write8(at + 1, size as u8);
                    c.bus.write16(at + 2, 0);
                    c.bus.write32(at + 4, r.next_u32());
                    fill(c, &mut r, at + 8, (size + 3) / 4);
                    at += 8 + size;
                }
                c.bus.write32(DATA + 0x100, r.next_u32() % 41);
                c.bus.write32(DATA + 0x104, stream);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x104);
                let space = match r.next_u32() % 4 {
                    0 => u32::MAX,
                    1 => 0,
                    _ => DATA + 0x9000,
                };
                oc371_script(c, 0x003a_8a90, space);
            },
        );
    }
    /// Every early exit (speech off, track group 4, replay, rising, too
    /// slow, a remote rider not listened to, rising again, speech busy, no
    /// view), local / remote riders, the three track categories (the real
    /// `SSXAudioSystem_TrackToCategory` with its jump table) and the event
    /// context accepted or not. Callees and the rider's methods are logged
    /// fakes (one may unwind).
    #[test]
    fn ssxaudio_rider_say_2083_matches() {
        oc371_install_fakes();
        covered(
            ee::s08::f_0029fcc8__generated,
            ssxaudio_rider_say_2083,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x0029_fcc8, 14);
                c.pc = 0x0029_fcc8;
                let me = DATA;
                let rider = DATA + 0x2000;
                let vt = DATA + 0x5000;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(rider);
                put_words(
                    c,
                    0x0048_3010,
                    &[
                        0x002a_1e44,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e4c,
                        0x002a_1e54,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e54,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e54,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e54,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e54,
                        0x002a_1e44,
                        0x002a_1e44,
                        0x002a_1e4c,
                        0x002a_1e4c,
                        0x002a_1e54,
                    ],
                );
                let k = r.next_u32();
                oc371_script(c, 0x0029_f0a0, (k % 12 != 0) as u32);
                oc371_script(c, 0x002a_3fe8, (k / 12 % 12 == 0) as u32);
                oc371_script(c, 0x0028_8ae0, (k / 144 % 12 == 0) as u32);
                let vel = match r.next_u32() % 6 {
                    0 => r.f32(5.0) & !0x8000_0000,
                    1 => 0,
                    _ => r.f32(5.0) | 0x8000_0000,
                };
                c.bus.write32(rider + 0x470, vel);
                c.bus.write32(rider + 0x788, DATA + 0x3000);
                c.bus.write32(GAME_GP - 0x4570, 0x4080_0000);
                c.bus.write32(GAME_GP + 0x5e8, r.f32(100.0));
                let speed = if r.next_u32() % 5 == 0 {
                    r.f32(4.0) & !0x8000_0000
                } else {
                    (4.0f32 + (r.next_u32() % 100) as f32 / 10.0).to_bits()
                };
                c.bus.write32(DATA + 0x3000 + 0x98, speed);
                // The rider's component at +0x6c0 and its methods.
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x48, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(vt + 0x4c, fakes::FAKE);
                c.bus.write16(vt + 0x28, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(vt + 0x2c, fakes::FAKE + 0x10);
                oc371_script(c, fakes::FAKE, r.next_u32() % 2);
                c.bus.write32(rider + 0x87c, r.next_u32() % 2);
                oc371_script(
                    c,
                    0x0028_5d98,
                    if r.next_u32() % 4 == 0 { 0 } else { rider },
                );
                oc371_script(c, 0x002b_0e28, (r.next_u32() % 6 != 0) as u32);
                oc371_script(
                    c,
                    0x0028_8b40,
                    if r.next_u32() % 6 == 0 { u32::MAX } else { 0 },
                );
                oc371_script(c, 0x0014_4bc0, DATA + 0x600);
                c.bus.write32(DATA + 0x600, r.next_u32() % 24);
                oc371_script(c, 0x002b_1458, r.next_u32() % 2);
                c.bus.write32(GAME_GP + SAY_QUEUE_HOOK, fakes::FAKE + 0x20);
            },
        );
    }
}
