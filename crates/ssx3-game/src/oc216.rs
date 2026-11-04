//! Staging wave 201/202 (oc216): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
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
        0x0013_52a8,
        rider_spin_sub_update_trick,
        "RiderSpinSub_UpdateTrick",
    ),
    (
        0x0013_c948,
        rider_ground_state_compute_boost_accel,
        "RiderGroundState_ComputeBoostAccel",
    ),
    (0x002b_d378, twpigd_sun_blend, "tWPIGD_Sun_Blend"),
    (0x003b_a888, snd_cents_to_pitch, "SND_CentsToPitch"),
    (0x003c_6380, snd_umul32x32, "SND_UMul32x32"),
    (0x0012_0f20, rider_begin_frame, "Rider_BeginFrame"),
    (
        0x0012_e9b8,
        rider_prewind_sub_handle_input,
        "RiderPrewindSub_HandleInput",
    ),
    (
        0x0010_def0,
        computer_actor_rider_compute_rubber_band,
        "ComputerActorRider_ComputeRubberBand",
    ),
    (
        0x002a_c7f0,
        bank_monitor_instance_update_range,
        "BankMonitorInstance_UpdateRange",
    ),
    (0x0012_91e0, ai_render, "AI_Render"),
];

/// `reg + off` (a field of the object a register points at).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off` (a `.lit4` constant or a global).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ===========================================================================
// 0x001352a8 RiderSpinSub_UpdateTrick

/// Fields and callees of [`rider_spin_sub_update_trick`].
pub mod update_trick {
    use super::*;

    /// `RiderSpinSub` (the trick part): the grab-trick state (0 starting,
    /// 1 wind-up playing, 2 held pose reached, 3 holding, 4 hold pose
    /// looping, 5 releasing), the trick index being played, the rider.
    pub const STATE: u32 = 0x4;
    pub const TRICK: u32 = 0x8;
    pub const RIDER: u32 = 0x58;

    /// Rider fields: uber timer (float), uber level, animation controller,
    /// metrix, AI (player) index, uber flags.
    pub const UBER_TIMER: u32 = 0x2f0;
    pub const UBER_LEVEL: u32 = 0x2f4;
    pub const ANIM_CTRL: u32 = 0x784;
    pub const METRIX: u32 = 0x790;
    pub const AI_INDEX: u32 = 0x86c;
    pub const UBER_FLAGS: u32 = 0xb2c;

    /// Animation sequence (`AnimController_GetLayerSequence`): playback
    /// speed (float) and the 64-bit event flags (bit 0 started, 1 at the
    /// hold point, 2 looped, 3 finished).
    pub const SEQ_SPEED: u32 = 0x90;
    pub const SEQ_FLAGS: u32 = 0xb0;

    /// `.lit4` fade-out times (0.33 s on release / end / chain, 0.1 s when
    /// the hold starts).
    pub const FADE_ON_RELEASE: i32 = -0x735c;
    pub const FADE_ON_HOLD: i32 = -0x7358;
    pub const FADE_ON_END: i32 = -0x7354;
    pub const FADE_ON_CHAIN: i32 = -0x7350;

    pub const ANIM_CONTROLLER_FADE_OUT_LAYER: u32 = 0x0031_1e88;
    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
    pub const ANIM_CONTROLLER_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;
    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    pub const BEBAGT_INT_GET_TRICK_ANIM2: Callee = Callee {
        addr: 0x0015_02c8,
        f: ee::s01::f_001502c8,
    };
    pub const BEBAGT_INT_GET_TRICK_ANIM: Callee = Callee {
        addr: 0x0015_0198,
        f: ee::s01::f_00150198,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD0: Callee = Callee {
        addr: 0x0015_00d8,
        f: ee::s01::f_001500d8,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD2: Callee = Callee {
        addr: 0x0015_00f8,
        f: ee::s01::f_001500f8,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD4: Callee = Callee {
        addr: 0x0015_0118,
        f: ee::s01::f_00150118,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD6: Callee = Callee {
        addr: 0x0015_0138,
        f: ee::s01::f_00150138,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD8: Callee = Callee {
        addr: 0x0015_0158,
        f: ee::s01::f_00150158,
    };
    pub const BEBAGT_INT_GET_TRICK_FIELD_A: Callee = Callee {
        addr: 0x0015_0178,
        f: ee::s01::f_00150178,
    };
    pub const BEBAGT_INT_GET_TRICK_VALUE: Callee = Callee {
        addr: 0x0015_03f8,
        f: ee::s01::f_001503f8,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const FLAGS64_TEST: Callee = Callee {
        addr: 0x0014_46a0,
        f: ee::s01::f_001446a0,
    };
    pub const RIDER_ADD_BOOST: u32 = 0x0010_e098;
    pub const RIDER_GET_TRICK_SKILL_FACTOR: u32 = 0x0012_0038;
    pub const RIDER_METRIX_ADD_TRICK: u32 = 0x0011_9708;
    pub const RIDER_METRIX_COMPLETE_TRICK: u32 = 0x0011_97d8;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SSX_AUDIO_SYSTEM_PLAY_GRAB_TRICK_SOUND: u32 = 0x0029_a530;
}

/// `bool RiderSpinSub_UpdateTrick(RiderSpinSub* this, int trick, int held)`
/// (0x1352a8): the grab-trick state machine of an airborne rider, once per
/// tick. `trick` is the requested trick index (-1 none), `held` whether the
/// trick button is held.
///
/// - Reads the rider's trick skill factor (the playback speed of the
///   trick layer) and the BAGT trick interface; `hi` = uber level >= 5.
/// - State 0: a new trick starts its wind-up animation on layer 2 (on top
///   of a playing kind-0x12 trick: blended), no trick stops the layer
///   (anim 0x11f, speed -1); once the wind-up has started (flag 0), 1.
/// - State 1: at the hold point (flag 1) the trick is scored
///   (`RiderMetrix_AddTrick`, `Rider_AddBoost`), the layer is stopped and
///   the "held" animation, if any, starts on layer 1; 2.
/// - State 2: a different trick completes this one and fades layer 1
///   (5); holding fades both layers, plays the hold pose (the uber variant
///   when the uber timer runs and the flag allows), the grab sound; 3.
/// - State 3: still held at the hold point scores again and plays the
///   loop pose (kind 0x13: fields 8/A, else the uber value/anim2 tables);
///   4. Otherwise once the layer reached a hold, loop or end point the
///   trick is scored and completed; 5.
/// - State 4: released or changed: completed, layer 1 faded; 5.
/// - State 5: when layer 2 ends (anim kind 2 or flag 3), a held new trick
///   on a kind-0x14 layer chains (3), else back to 0 with no trick.
///
/// Returns whether layer 2 plays a trick animation (kind 0x12..0x14).
#[allow(unused_labels)]
pub fn rider_spin_sub_update_trick(c: &mut Cpu) {
    use update_trick::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'b_13541c: {
            'b_135418: {
                c.cycles += 28;
                frame.save(c, S1, 0x60);
                frame.save(c, S2, 0x50);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S4, 0x30);
                frame.save(c, S5, 0x20);
                c.g[S4][0] = c.g[A1][0];
                frame.save(c, S6, 0x10);
                frame.save_fpr(c, 21, 0x88);
                c.g[S6][0] = c.g[A2][0];
                frame.save(c, S0, 0x70);
                frame.save(c, S3, 0x40);
                frame.save_ra(c, 0);
                frame.save_fpr(c, 20, 0x80);
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                call_ptr(c, RIDER_GET_TRICK_SKILL_FACTOR, 0x001352e8)?;
                c.mov_s(21, 0);
                BE_LIBRARY_GET_INSTANCE.call(c, 0x001352f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x6;
                BE_LIBRARY_GET_INTERFACE.call(c, 0x001352fc)?;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                c.g[V0][0] = c.lw(at(c, A0, UBER_LEVEL));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[S5][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_135418;
                } else {
                    'b_135414: {
                        'b_135360: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[S4][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw(at(c, S1, TRICK));
                            if t {
                                break 'b_135360;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S4][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_13541c;
                                } else {
                                    c.cycles += 13;
                                    c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135338)?;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A1][0] = 0x11f;
                                    c.g[A2][0] = 0u64;
                                    c.swc1(12, at(c, V0, SEQ_SPEED));
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135358)?;
                                    c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                    break 'b_135414;
                                }
                            }
                        }
                        'b_1353f0: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == c.g[S4][0];
                            c.g[A1][0] = 0x2;
                            if t {
                                break 'b_1353f0;
                            } else {
                                'b_1353dc: {
                                    'b_1353b0: {
                                        c.cycles += 6;
                                        c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                        c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                                        call_ptr(
                                            c,
                                            ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY,
                                            0x00135374,
                                        )?;
                                        c.g[V1][0] = 0x12;
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                        if t {
                                            break 'b_1353b0;
                                        } else {
                                            c.cycles += 12;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_FIELD0.call(c, 0x00135390)?;
                                            c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 12);
                                            c.g[A2][0] = 0x1;
                                            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001353a8)?;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1353dc;
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                    BEBAGT_INT_GET_TRICK_FIELD0.call(c, 0x001353c0)?;
                                    c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001353d8)?;
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    break 'b_1353dc;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001353e8)?;
                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                break 'b_135414;
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                        c.g[A1][0] = 0x2;
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001353fc)?;
                        c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                        c.g[A1][0] = 0u64;
                        FLAGS64_TEST.call(c, 0x00135408)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_135414;
                        } else {
                            c.cycles += 1;
                            c.sw(at(c, S1, STATE), c.g[V0][0]);
                            break 'b_135414;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_135418;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_13541c;
        }
        'b_1354f8: {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_1354f8;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135434)?;
                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                c.g[A1][0] = 0x1;
                FLAGS64_TEST.call(c, 0x00135440)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_1354f8;
                } else {
                    'b_1354f0: {
                        c.cycles += 25;
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                        c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                        BEBAGT_INT_GET_TRICK_FIELD4.call(c, 0x0013545c)?;
                        c.g[A0][0] = c.lw(at(c, S0, METRIX));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135468)?;
                        c.g[A0][0] = c.lw(at(c, S1, RIDER));
                        c.mov_s(12, 0);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, RIDER_ADD_BOOST, 0x00135478)?;
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135488)?;
                        c.sw(at(c, V0, SEQ_SPEED), 0u64);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                        c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                        BEBAGT_INT_GET_TRICK_FIELD2.call(c, 0x001354a0)?;
                        c.g[V1][0] = 0x1b6;
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1354f0;
                        } else {
                            c.cycles += 17;
                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD2.call(c, 0x001354c0)?;
                            c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001354d8)?;
                            c.g[V1][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001354e8)?;
                            c.swc1(21, at(c, V0, SEQ_SPEED));
                            c.g[V0][0] = 0x2;
                            break 'b_1354f0;
                        }
                    }
                    c.cycles += 2;
                    c.sw(at(c, S1, STATE), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_1354f8;
                }
            }
        }
        'b_135674: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135674;
            } else {
                'b_135670: {
                    'b_13566c: {
                        'b_135568: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            let t = c.g[S4][0] == c.g[A2][0];
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_135568;
                            } else {
                                c.cycles += 22;
                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                BEBAGT_INT_GET_TRICK_FIELD4.call(c, 0x0013551c)?;
                                c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135528)?;
                                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                c.mov_s(12, 0);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, RIDER_ADD_BOOST, 0x00135538)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x1;
                                c.lwc1(12, gp(c, FADE_ON_RELEASE));
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x0013554c)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013555c)?;
                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                c.g[V0][0] = 0x5;
                                break 'b_13566c;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S6][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_135670;
                        } else {
                            'b_135640: {
                                'b_135614: {
                                    'b_1355e4: {
                                        c.cycles += 17;
                                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                        c.lwc1(20, gp(c, FADE_ON_HOLD));
                                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                        c.mov_s(12, 20);
                                        call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x00135584)?;
                                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                        c.mov_s(12, 20);
                                        c.g[A1][0] = 0x2;
                                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                        call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x00135598)?;
                                        c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                        c.mtc1(0, 1);
                                        c.lwc1(0, at(c, V1, UBER_TIMER));
                                        c.c_eq_s(0, 1);
                                        if c.fcc() {
                                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1355e4;
                                        } else {
                                            'b_1355e0: {
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lw(at(c, V1, UBER_FLAGS));
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1355e0;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[A1][0] = c.lw(at(c, V1, AI_INDEX));
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A3][0] = c.g[S5][0];
                                                    BEBAGT_INT_GET_TRICK_ANIM
                                                        .call(c, 0x001355d4)?;
                                                    c.g[V1][0] = 0x1b6;
                                                    if c.g[V0][0] != c.g[V1][0] {
                                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                        break 'b_135614;
                                                    } else {
                                                        break 'b_1355e0;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1355e4;
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                    BEBAGT_INT_GET_TRICK_FIELD6.call(c, 0x001355f4)?;
                                    c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x0013560c)?;

                                    break 'b_135640;
                                }
                                c.cycles += 11;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                c.g[A3][0] = c.g[S5][0];
                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135628)?;
                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 12);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135640)?;
                                break 'b_135640;
                            }
                            c.cycles += 11;
                            call_ptr(c, SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x00135648)?;
                            c.g[A1][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, SSX_AUDIO_SYSTEM_PLAY_GRAB_TRICK_SOUND, 0x00135654)?;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135664)?;
                            c.swc1(21, at(c, V0, SEQ_SPEED));
                            c.g[V0][0] = 0x3;
                            break 'b_13566c;
                        }
                    }
                    c.cycles += 1;
                    c.sw(at(c, S1, STATE), c.g[V0][0]);
                    break 'b_135670;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135674;
            }
        }
        'b_135930: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135930;
            } else {
                'b_135928: {
                    'b_1357e0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, TRICK));
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_1357e0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S6][0] == 0u64;

                            if t {
                                break 'b_1357e0;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001356a0)?;
                                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                                c.g[A1][0] = 0x1;
                                FLAGS64_TEST.call(c, 0x001356ac)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                if t {
                                    break 'b_1357e0;
                                } else {
                                    'b_1357d8: {
                                        'b_1357b0: {
                                            'b_135744: {
                                                c.cycles += 11;
                                                c.g[A1][0] = 0x2;
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                                    0x001356c0,
                                                )?;
                                                c.sw(at(c, V0, SEQ_SPEED), 0u64);
                                                c.g[A1][0] = 0x2;
                                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY,
                                                    0x001356d4,
                                                )?;
                                                c.g[V1][0] = 0x13;
                                                let t = c.g[V0][0] != c.g[V1][0];
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                if t {
                                                    break 'b_135744;
                                                } else {
                                                    c.cycles += 19;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                    BEBAGT_INT_GET_TRICK_FIELD_A
                                                        .call(c, 0x001356f0)?;
                                                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(
                                                        c,
                                                        RIDER_METRIX_ADD_TRICK,
                                                        0x001356fc,
                                                    )?;
                                                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                                    c.mov_s(12, 0);
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, RIDER_ADD_BOOST, 0x0013570c)?;
                                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                                    BEBAGT_INT_GET_TRICK_FIELD8
                                                        .call(c, 0x00135720)?;
                                                    c.g[V1][0] = 0x1b6;
                                                    let t = c.g[V0][0] == c.g[V1][0];
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_1357d8;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                        c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                        BEBAGT_INT_GET_TRICK_FIELD8
                                                            .call(c, 0x0013573c)?;
                                                        c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                        break 'b_1357b0;
                                                    }
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x00135758)?;
                                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135764)?;
                                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                            c.mov_s(12, 0);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, RIDER_ADD_BOOST, 0x00135774)?;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_ANIM2.call(c, 0x0013578c)?;
                                            c.g[V1][0] = 0x1b6;
                                            let t = c.g[V0][0] == c.g[V1][0];
                                            c.g[A3][0] = c.g[S5][0];
                                            if t {
                                                break 'b_1357d8;
                                            } else {
                                                c.cycles += 6;
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                BEBAGT_INT_GET_TRICK_ANIM2.call(c, 0x001357ac)?;
                                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                break 'b_1357b0;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 12);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001357c4)?;
                                        c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                                        call_ptr(
                                            c,
                                            ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                            0x001357d4,
                                        )?;
                                        c.swc1(21, at(c, V0, SEQ_SPEED));
                                        break 'b_1357d8;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x4;
                                    break 'b_135928;
                                }
                            }
                        }
                    }
                    'b_135844: {
                        c.cycles += 8;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001357ec)?;
                        c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                        c.g[A1][0] = 0x1;
                        FLAGS64_TEST.call(c, 0x001357f8)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_135844;
                        } else {
                            c.cycles += 8;
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013580c)?;
                            c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                            c.g[A1][0] = 0x3;
                            FLAGS64_TEST.call(c, 0x00135818)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            if t {
                                break 'b_135844;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013582c)?;
                                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                                c.g[A1][0] = 0x2;
                                FLAGS64_TEST.call(c, 0x00135838)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                                    break 'b_135930;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    break 'b_135844;
                                }
                            }
                        }
                    }
                    'b_1358c0: {
                        c.cycles += 6;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x00135850)?;
                        c.g[V1][0] = 0x13;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_1358c0;
                        } else {
                            c.cycles += 25;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x0013586c)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135878)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x00135888)?;
                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x0013589c)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x001358a8)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x001358b8)?;
                            c.g[V0][0] = 0x5;
                            break 'b_135928;
                        }
                    }
                    c.cycles += 26;
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x001358d4)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x001358e0)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x001358f0)?;
                    c.g[S0][0] = c.lw(at(c, S1, RIDER));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x00135908)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135914)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x00135924)?;
                    c.g[V0][0] = 0x5;
                    break 'b_135928;
                }
                c.cycles += 2;
                c.sw(at(c, S1, STATE), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135930;
            }
        }
        'b_135a00: {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                break 'b_135a00;
            } else {
                'b_135950: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S1, TRICK));
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    if t {
                        break 'b_135950;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S6][0] != 0u64;

                        if t {
                            break 'b_135a00;
                        } else {
                            break 'b_135950;
                        }
                    }
                }
                'b_1359d0: {
                    'b_13599c: {
                        c.cycles += 6;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x0013595c)?;
                        c.g[V1][0] = 0x13;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_13599c;
                        } else {
                            c.cycles += 13;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x00135978)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135984)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x00135994)?;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            break 'b_1359d0;
                        }
                    }
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x001359b0)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x001359bc)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x001359cc)?;
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_1359d0;
                }
                c.cycles += 12;
                c.g[A1][0] = 0x1;
                c.lwc1(12, gp(c, FADE_ON_END));
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x001359e0)?;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001359f0)?;
                c.swc1(21, at(c, V0, SEQ_SPEED));
                c.g[V0][0] = 0x5;
                c.sw(at(c, S1, STATE), c.g[V0][0]);
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                break 'b_135a00;
            }
        }
        'b_135afc: {
            c.cycles += 8;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x00135a0c)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw(at(c, S1, STATE));
            c.g[V0][0] = 0x5;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x12));
            if t {
                break 'b_135afc;
            } else {
                'b_135af8: {
                    'b_135a50: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = c.g[S3][0] == c.g[V0][0];
                        c.g[V0][0] = 0x14;
                        if t {
                            break 'b_135a50;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135a3c)?;
                            c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                            c.g[A1][0] = 0x3;
                            FLAGS64_TEST.call(c, 0x00135a48)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x14;
                            if t {
                                break 'b_135af8;
                            } else {
                                break 'b_135a50;
                            }
                        }
                    }
                    'b_135af4: {
                        'b_135af0: {
                            c.cycles += 2;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_135af0;
                            } else {
                                c.cycles += 2;
                                if c.g[S6][0] == 0u64 {
                                    c.sw(at(c, S1, STATE), 0u64);
                                    break 'b_135af4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_135af0;
                                    } else {
                                        'b_135aec: {
                                            c.cycles += 8;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            c.g[A2][0] = c.g[S4][0];
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135a7c)?;
                                            c.g[V1][0] = 0x1b6;
                                            let t = c.g[V0][0] == c.g[V1][0];
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_135aec;
                                            } else {
                                                c.cycles += 25;
                                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                                c.lwc1(12, gp(c, FADE_ON_CHAIN));
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_FADE_OUT_LAYER,
                                                    0x00135a9c,
                                                )?;
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A3][0] = c.g[S5][0];
                                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135ab4)?;
                                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[A2][0] = 0x1;
                                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135acc)?;
                                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A1][0] = 0x2;
                                                c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                                    0x00135adc,
                                                )?;
                                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                                c.g[V0][0] = 0x3;
                                                c.sw(at(c, S1, STATE), c.g[V0][0]);
                                                break 'b_135af8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        break 'b_135af0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(at(c, S1, STATE), 0u64);
                        break 'b_135af4;
                    }
                    c.cycles += 1;
                    c.sw(at(c, S1, TRICK), c.g[V0][0]);
                    break 'b_135af8;
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x12));
                break 'b_135afc;
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x0013c948 RiderGroundState_ComputeBoostAccel

/// Fields and callees of [`rider_ground_state_compute_boost_accel`].
pub mod boost_accel {
    use super::*;

    /// The ground state's rider.
    pub const RIDER: u32 = 0x18;
    /// Rider fields: the ground material (>= 17: the slow-snow scale), the
    /// finish time (< 0 while racing), the AI (player) index, the character
    /// slot, the animation controller.
    pub const GROUND_MATERIAL: u32 = 0x434;
    pub const FINISH_TIME: u32 = 0x470;
    pub const AI_INDEX: u32 = 0x86c;
    pub const CHARACTER_SLOT: u32 = 0xb34;
    pub const ANIM_CTRL: u32 = 0x784;
    /// The tuning parameters: base acceleration and boost factor.
    pub const PARAM_ACCEL: u32 = 0x20;
    pub const PARAM_BOOST: u32 = 0x24;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    pub const BE_CHAR_INT_GET_ATTRIB1_FACTOR: Callee = Callee {
        addr: 0x0014_93d8,
        f: ee::s01::f_001493d8,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const MATH_ATAN: u32 = 0x0031_c228;
}

/// `float RiderGroundState_ComputeBoostAccel(RiderGroundState* this,
/// const Tuning* p)` (0x13c948): the forward acceleration of a riding
/// rider this tick (f0); 0 when `+0x214` is non-zero (a brake / air
/// state) or the rider has finished.
///
/// - A ground-material scale (1, or a `.lit4` factor on materials >= 17).
/// - The slope factor: 1 when the rider is flagged (`+0x2d4` bit 4), else
///   from the heading's angle (`Math_Atan` of the `+0x1b0` direction,
///   folded into a half turn) relative to the fall line (`+0x4cc`), faded
///   out past a `.lit4` angle; on materials >= 17 the factor is ignored
///   when negative.
/// - Speed-dependent drag (`+0x2e8`, `+0x2fc`, `+0x1f0`, `+0x3a8`) and,
///   while the speed (VU0 length of `+0x1e0`) is under the boost cap, the
///   character's acceleration attribute (`BECharInt_GetAttrib1Factor`), a
///   boost-pose bonus (layer 2 anim id 0x16) and the carve factor on
///   materials 11..13 (`+0x378`).
/// - Result: `slope * (p.accel * accel * material + rider[0x220] * k)`.
#[allow(unused_labels)]
pub fn rider_ground_state_compute_boost_accel(c: &mut Cpu) {
    use boost_accel::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0xa0);
        'b_13c9a0: {
            c.cycles += 21;
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save_fpr(c, 26, 0x90);
            frame.save_ra(c, 0x20);
            frame.save_fpr(c, 25, 0x88);
            frame.save_fpr(c, 24, 0x80);
            frame.save_fpr(c, 23, 0x78);
            frame.save_fpr(c, 22, 0x70);
            frame.save_fpr(c, 21, 0x68);
            frame.save_fpr(c, 20, 0x60);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 26);
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = c.lw(at(c, V1, GROUND_MATERIAL));
            c.g[S0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
            c.g[S0][0] = c.g[S0][0] ^ 0x1;
            let t = c.g[S0][0] == 0u64;
            c.g[S2][0] = c.g[A1][0];
            if t {
                break 'b_13c9a0;
            } else {
                c.cycles += 1;
                c.lwc1(26, gp(c, -0x70e4));
                break 'b_13c9a0;
            }
        }
        'b_13ccb8: {
            c.cycles += 6;
            c.lwc1(0, at(c, V1, 0x214));
            c.mtc1(0, 23);
            c.c_eq_s(0, 23);
            let t = !c.fcc();
            c.mov_s(0, 23);
            if t {
                break 'b_13ccb8;
            } else {
                c.cycles += 5;
                c.lwc1(0, at(c, V1, FINISH_TIME));
                c.c_le_s(23, 0);
                let t = c.fcc();
                c.mov_s(0, 23);
                if t {
                    break 'b_13ccb8;
                } else {
                    'b_13caf4: {
                        'b_13c9f0: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lh(at(c, V1, 0x2d4));
                            c.mov_s(22, 23);
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] == 0u64;
                            c.mov_s(24, 22);
                            if t {
                                break 'b_13c9f0;
                            } else {
                                c.cycles += 4;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 24);

                                break 'b_13caf4;
                            }
                        }
                        'b_13ca84: {
                            'b_13ca80: {
                                'b_13ca44: {
                                    c.cycles += 10;
                                    c.lq(2, at(c, V1, 0x1b0));
                                    c.lwc1(25, gp(c, -0x70e0));
                                    c.sq(2, frame.at(c, 0));
                                    c.lq(2, at(c, V1, 0x1b0));
                                    c.lwc1(21, frame.at(c, 0x4));
                                    c.sq(2, frame.at(c, 0x10));
                                    c.lwc1(20, frame.at(c, 0x10));
                                    c.c_eq_s(20, 22);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_13ca44;
                                    } else {
                                        c.cycles += 4;
                                        c.c_eq_s(21, 22);
                                        let t = c.fcc();
                                        c.mov_s(12, 21);
                                        if t {
                                            break 'b_13ca80;
                                        } else {
                                            c.cycles += 4;
                                            c.c_le_s(22, 21);
                                            c.lwc1(12, gp(c, -0x70dc));
                                            let t = !c.fcc();
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            if t {
                                                break 'b_13ca84;
                                            } else {
                                                c.cycles += 3;
                                                c.lwc1(12, gp(c, -0x70d8));

                                                break 'b_13ca84;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                call_ptr(c, MATH_ATAN, 0x0013ca50)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_13ca80;
                                } else {
                                    'b_13ca78: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_13ca78;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x70d4));
                                            c.add_s(12, 12, 0);
                                            break 'b_13ca80;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, gp(c, -0x70d0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_13ca80;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            break 'b_13ca84;
                        }
                        'b_13cac4: {
                            c.cycles += 13;
                            c.lwc1(0, gp(c, -0x70cc));
                            c.lwc1(1, at(c, V0, 0x4cc));
                            c.g[V1][0] = c.g[V0][0];
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.sub_s(12, 12, 1);
                            c.mul_s(0, 12, 0);
                            c.add_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.cvt_s_w(1, 1);
                            c.c_lt_s(0, 1);
                            let t = !c.fcc();

                            if t {
                                break 'b_13cac4;
                            } else {
                                c.cycles += 3;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.sub_s(1, 1, 0);
                                break 'b_13cac4;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gp(c, -0x70c8));
                        c.mul_s(0, 1, 0);
                        c.sub_s(0, 12, 0);
                        c.abs_s(12, 0);
                        c.c_lt_s(12, 25);
                        let t = !c.fcc();

                        if t {
                            break 'b_13caf4;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 1);
                            c.div_s(0, 1, 25);
                            c.mul_s(0, 12, 0);
                            c.sub_s(24, 1, 0);
                            break 'b_13caf4;
                        }
                    }
                    'b_13cb1c: {
                        c.cycles += 7;
                        c.mtc1(0, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 25);
                        c.c_le_s(24, 0);
                        if !c.fcc() {
                            c.lwc1(0, at(c, V1, 0x2e8));
                            break 'b_13cb1c;
                        } else {
                            'b_13cb18: {
                                c.cycles += 2;
                                if c.g[S0][0] != 0u64 {
                                    c.mov_s(25, 0);
                                    break 'b_13cb18;
                                } else {
                                    break 'b_13cb18;
                                }
                            }
                            c.cycles += 1;
                            c.lwc1(0, at(c, V1, 0x2e8));
                            break 'b_13cb1c;
                        }
                    }
                    'b_13cb48: {
                        c.cycles += 7;
                        c.mtc1(0, 4);
                        c.lwc1(2, at(c, V1, 0x2fc));
                        c.lwc1(1, gp(c, -0x70c4));
                        c.c_lt_s(4, 0);
                        let t = !c.fcc();
                        c.mul_s(2, 2, 1);
                        if t {
                            break 'b_13cb48;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x40b00000;
                            c.mtc1(1, 0);
                            c.mul_s(0, 24, 0);
                            c.max_s(2, 0, 2);
                            break 'b_13cb48;
                        }
                    }
                    'b_13cbb4: {
                        c.cycles += 4;
                        c.c_lt_s(4, 2);
                        if !c.fcc() {
                            c.lwc1(0, at(c, S2, PARAM_ACCEL));
                            break 'b_13cbb4;
                        } else {
                            c.cycles += 8;
                            c.lwc1(0, at(c, V1, 0x1f0));
                            c.lwc1(1, gp(c, -0x70c0));
                            c.abs_s(0, 0);
                            c.sub_s(0, 1, 0);
                            c.c_lt_s(4, 0);
                            if !c.fcc() {
                                c.lwc1(0, at(c, S2, PARAM_ACCEL));
                                break 'b_13cbb4;
                            } else {
                                'b_13cba4: {
                                    'b_13cba0: {
                                        c.cycles += 5;
                                        c.lwc1(3, at(c, V1, 0x3a8));
                                        c.c_le_s(4, 3);
                                        let t = !c.fcc();
                                        c.mul_s(2, 2, 0);
                                        if t {
                                            break 'b_13cba0;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(0, gp(c, -0x70bc));
                                            c.lwc1(1, gp(c, -0x70b8));
                                            c.mul_s(0, 3, 0);
                                            c.add_s(0, 0, 1);
                                            break 'b_13cba4;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.lwc1(0, gp(c, -0x70b4));
                                    break 'b_13cba4;
                                }
                                c.cycles += 4;
                                c.mul_s(0, 2, 0);
                                c.add_s(0, 23, 0);
                                c.mov_s(22, 0);
                                c.lwc1(0, at(c, S2, PARAM_ACCEL));
                                break 'b_13cbb4;
                            }
                        }
                    }
                    'b_13cc04: {
                        c.cycles += 19;
                        c.lqc2(3, at(c, V1, 0x1e0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(2, 22);
                        c.mtc1(2, 3);
                        c.lwc1(2, gp(c, -0x70b0));
                        c.mul_s(0, 0, 26);
                        c.lwc1(1, gp(c, -0x70ac));
                        c.mul_s(0, 0, 2);
                        c.sub_s(0, 0, 3);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.mov_s(20, 1);
                        if t {
                            break 'b_13cc04;
                        } else {
                            c.cycles += 1;
                            c.mov_s(20, 0);
                            break 'b_13cc04;
                        }
                    }
                    'b_13cc9c: {
                        c.cycles += 5;
                        c.mtc1(0, 21);
                        c.c_lt_s(21, 20);
                        if !c.fcc() {
                            c.lwc1(0, at(c, S2, PARAM_BOOST));
                            break 'b_13cc9c;
                        } else {
                            'b_13cc60: {
                                c.cycles += 17;
                                BE_LIBRARY_GET_INSTANCE.call(c, 0x0013cc20)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                BE_LIBRARY_GET_INTERFACE.call(c, 0x0013cc2c)?;
                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw(at(c, V1, AI_INDEX));
                                c.g[A2][0] = c.lw(at(c, V1, CHARACTER_SLOT));
                                BE_CHAR_INT_GET_ATTRIB1_FACTOR.call(c, 0x0013cc40)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ID, 0x0013cc50)?;
                                c.g[V1][0] = 0x16;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.mov_s(1, 21);
                                if t {
                                    break 'b_13cc60;
                                } else {
                                    c.cycles += 1;
                                    c.lwc1(1, gp(c, -0x70a8));
                                    break 'b_13cc60;
                                }
                            }
                            'b_13cc84: {
                                c.cycles += 6;
                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                c.g[V0][0] = c.lw(at(c, V1, GROUND_MATERIAL));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
                                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_13cc84;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, at(c, V1, 0x378));
                                    c.abs_s(0, 0);
                                    c.mul_s(20, 20, 0);
                                    break 'b_13cc84;
                                }
                            }
                            c.cycles += 6;
                            c.lwc1(0, gp(c, -0x70a4));
                            c.mul_s(0, 1, 0);
                            c.max_s(0, 24, 0);
                            c.mul_s(0, 0, 20);
                            c.add_s(22, 22, 0);
                            c.lwc1(0, at(c, S2, PARAM_BOOST));
                            break 'b_13cc9c;
                        }
                    }
                    c.cycles += 7;
                    c.lwc1(2, at(c, V1, 0x220));
                    c.mul_s(0, 0, 22);
                    c.lwc1(1, gp(c, -0x70a0));
                    c.mul_s(2, 2, 1);
                    c.mul_s(0, 0, 26);
                    c.add_s(0, 0, 2);
                    c.mul_s(0, 25, 0);
                    break 'b_13ccb8;
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 26, 0x90);
        frame.restore_fpr(c, 25, 0x88);
        frame.restore_fpr(c, 24, 0x80);
        frame.restore_fpr(c, 23, 0x78);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x002bd378 tWPIGD_Sun_Blend

/// Fields of [`twpigd_sun_blend`].
pub mod sun_blend {
    /// `tWPIGD_Sun` (a world-painter sun record): the vtable, six float
    /// (current, target) pairs from +0x8, an int pair at +0x38, two more
    /// float pairs at +0x40.
    pub const VTABLE: u32 = 0x4;
    pub const FLOAT_PAIRS: u32 = 0x8;
    pub const INT_PAIR: u32 = 0x38;
    pub const TAIL_PAIRS: u32 = 0x40;
    /// The source record: six floats from +0x4, an int at +0x1c, two floats
    /// at +0x20.
    pub const SRC_FLOATS: u32 = 0x4;
    pub const SRC_INT: u32 = 0x1c;
    pub const SRC_TAIL: u32 = 0x20;
    /// The "changed" notification method (vtable slot).
    pub const NOTIFY_SLOT: u32 = 0x218;
}

/// `void tWPIGD_Sun_Blend(tWPIGD_Sun* this, const SunSource* src, float t)`
/// (0x2bd378): blends the sun parameters toward `src` with weight `t*t`
/// (each pair: `cur = t² src + (1 - t²) cur`, `target = src`; the int
/// pair through float and back), with the record's notify method called
/// before and after.
pub fn twpigd_sun_blend(c: &mut Cpu) {
    use crate::call::VSlot;
    use sun_blend::*;
    guest(c, |c| {
        c.cycles += 103;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let this = Addr(r32(c, S0));
        let src = Addr(r32(c, S1));
        let t = abi::farg(c, 0);
        let w = t * t;
        abi::set_fpr(c, 20, w);

        // Notify before the change.
        let vt = this.off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        let notify = VSlot::read(c, vt, NOTIFY_SLOT);
        c.g[V0][0] = notify.func;
        c.g[A0][0] = notify.this(c.g[S0][0]);
        call_ptr(c, notify.func as u32, 0x002b_d3b0)?;

        c.g[AT][0] = 0x3f80_0000;
        let inv = FpuF32::ONE - w;
        // The old current values, scaled by 1 - t² (f1, f2, f4..f8; the int
        // one converted to float first).
        let first = src.off(SRC_FLOATS).read_fpu(c);
        let mut old = [FpuF32::ZERO; 6];
        for (k, o) in old.iter_mut().enumerate() {
            *o = this.off(FLOAT_PAIRS + 8 * k as u32).read_fpu(c);
        }
        let old_int = FpuF32::from_i32(this.off(INT_PAIR).read_u32(c) as i32);
        let scaled: [FpuF32; 6] = old.map(|o| inv * o);
        let scaled_int = inv * old_int;
        for (k, s) in scaled.iter().enumerate() {
            let k = k as u32;
            let v = if k == 0 {
                first
            } else {
                src.off(SRC_FLOATS + 4 * k).read_fpu(c)
            };
            this.off(FLOAT_PAIRS + 8 * k).write_fpu(c, w * v + *s);
            let target = src.off(SRC_FLOATS + 4 * k).read_fpu(c);
            this.off(FLOAT_PAIRS + 8 * k + 4).write_fpu(c, target);
        }
        // The int pair.
        let v = FpuF32::from_i32(src.off(SRC_INT).read_u32(c) as i32);
        let blended = w * v + scaled_int;
        this.off(INT_PAIR).write_u32(c, blended.to_i32() as u32);
        let target = src.off(SRC_INT).lw(c);
        c.g[V0][0] = target;
        this.off(INT_PAIR + 4).sw(c, target);
        // The two tail pairs.
        let a = w * src.off(SRC_TAIL).read_fpu(c);
        let old_a = inv * this.off(TAIL_PAIRS).read_fpu(c);
        let old_b = this.off(TAIL_PAIRS + 8).read_fpu(c);
        c.g[V1][0] = this.off(VTABLE).lw(c);
        let scaled_b = inv * old_b;
        this.off(TAIL_PAIRS).write_fpu(c, a + old_a);
        let target_a = src.off(SRC_TAIL).read_fpu(c);
        this.off(TAIL_PAIRS + 4).write_fpu(c, target_a);
        let vb = src.off(SRC_TAIL + 4).read_fpu(c);
        let new_b = w * vb + scaled_b;
        this.off(TAIL_PAIRS + 8).write_fpu(c, new_b);
        let target_b = src.off(SRC_TAIL + 4).read_fpu(c);
        this.off(TAIL_PAIRS + 0xc).write_fpu(c, target_b);

        // The registers as the original leaves them at the second call.
        abi::set_fpr(c, 0, target_b);
        abi::set_fpr(c, 1, vb);
        abi::set_fpr(c, 2, old_b);
        abi::set_fpr(c, 3, scaled_b);
        for (reg, s) in [
            (4, scaled[2]),
            (5, scaled[3]),
            (6, scaled[4]),
            (7, scaled[5]),
        ] {
            abi::set_fpr(c, reg, s);
        }
        abi::set_fpr(c, 8, scaled_int);
        abi::set_fpr(c, 20, new_b);

        // Notify after the change.
        let vt = c.g[V1][0];
        let notify = VSlot::read(c, vt, NOTIFY_SLOT);
        c.g[V0][0] = notify.func;
        c.g[A0][0] = notify.this(c.g[S0][0]);
        call_ptr(c, notify.func as u32, 0x002b_d4fc)?;

        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x003ba888 SND_CentsToPitch

/// The pitch fraction table of [`snd_cents_to_pitch`]: 256 bytes, the
/// fraction of a semitone step (`(2^(i/256) - 1) * 256`), at 0x44c480.
pub const CENTS_PITCH_TABLE: u32 = 0x0045_0000 - 0x3b80;

/// `int SND_CentsToPitch(int cents)` (0x3ba888): an SPU pitch (0x1000 =
/// unchanged) for a detune in cents. Whole octaves (1200 cents) double or
/// halve the pitch; the rest is scaled to 1/256 octave steps (`* 0x369d >>
/// 16`, at least -255) and looked up in the fraction table (`(pitch *
/// (0x100 + frac)) >> 8`, one more shift for negative steps).
pub fn snd_cents_to_pitch(c: &mut Cpu) {
    const OCTAVE: u32 = 0x4b0;
    c.cycles += 4;
    c.g[A2][0] = 0x1000;
    c.g[A3][0] = 0x45_0000;
    // Whole octaves up (the first test uses the full register).
    if (c.g[A0][0] as i64) >= 1200 {
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(OCTAVE));
            c.g[A2][0] = sx(r32(c, A2) << 1);
            let below = (c.g[A0][0] as i64) < 1200;
            c.g[V0][0] = below as u64;
            if below {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a898);
            }
        }
        c.cycles += 2;
    } else {
        c.cycles += 1;
    }
    // Whole octaves down.
    loop {
        let low = (c.g[A0][0] as i64) < -1199;
        c.g[V0][0] = low as u64;
        c.cycles += 5;
        if !low {
            break;
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(OCTAVE));
        if event_due(c) {
            deliver_event_here(c, 0x003b_a8c0);
        }
        c.cycles += 1;
        c.g[A2][0] = sx(((r32(c, A2) as i32) >> 1) as u32);
        c.cycles += 1;
    }
    // The 1/256-octave step.
    c.cycles += 9;
    c.g[V1][0] = sx(0xffff_ff00);
    let cents = r32(c, A0);
    let scaled = abi::mult(c, cents, 0x369d);
    c.g[V0][0] = sx(scaled);
    c.g[A0][0] = sx(0xffff_ff01);
    c.g[A1][0] = sx(((scaled as i32) >> 16) as u32);
    let above_min = (c.g[V1][0] as i64) < (c.g[A1][0] as i64);
    c.g[V1][0] = above_min as u64;
    if !above_min {
        c.g[A1][0] = c.g[A0][0];
    }
    let step = c.g[A1][0];
    c.g[V0][0] = sx(CENTS_PITCH_TABLE);
    let (entry, shift) = if (step as i64) >= 0 {
        c.cycles += 5;
        (CENTS_PITCH_TABLE.wrapping_add(step as u32), 8)
    } else {
        c.cycles += 6;
        (
            CENTS_PITCH_TABLE
                .wrapping_add(step as u32)
                .wrapping_add(0x100),
            9,
        )
    };
    let base = CENTS_PITCH_TABLE.wrapping_add(step as u32);
    c.g[V0][0] = sx(base);
    let frac = Addr(entry).lbu(c);
    let pitch = r32(c, A2);
    let product = abi::mult(c, pitch, (frac as u32).wrapping_add(0x100));
    c.g[V1][0] = sx(product);
    c.g[A2][0] = sx(((product as i32) >> shift) as u32);
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ===========================================================================
// 0x003c6380 SND_UMul32x32

/// `__muldi3` (64-bit multiply), the partial-product helper.
pub const MULDI3: u32 = 0x0041_14d0;

/// One partial product of [`snd_umul32x32`]: `__muldi3(x, y)` on the
/// registers `x` and `y`, returning to `ret`.
fn umul_part(c: &mut Cpu, x: usize, y: usize, ret: u32) -> Result<u64, crate::call::Unwound> {
    c.g[A0][0] = c.g[x][0];
    c.g[A1][0] = c.g[y][0];
    call_ptr(c, MULDI3, ret)?;
    Ok(c.g[V0][0])
}

/// `u64 SND_UMul32x32(u32 a, u32 b)` (0x3c6380): the full 64-bit product of
/// two 32-bit values from four 16x16 partial products (each through
/// `__muldi3`): `lo = al*bl`, `m1 = al*bh`, `m2 = ah*bl`, `hi = ah*bh`;
/// the low word `lo + (m1 << 16) + (m2 << 16)`, the high word `hi + (m1 >>
/// 16) + (m2 >> 16)` plus the carry out of the middle 16 bits. The two
/// words meet in a doubleword on the stack (copied once) and come back in
/// v0.
pub fn snd_umul32x32(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 74;
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0x20);
        let (a, b) = (c.g[A0][0], c.g[A1][0]);
        // The 16-bit halves (s0 = al, s2 = bl, s3 = ah, s4 = bh).
        c.g[S0][0] = a & 0xffff;
        c.g[S2][0] = b & 0xffff;
        c.g[S3][0] = ((a as u32) >> 16) as u64;
        c.g[S4][0] = ((b as u32) >> 16) as u64;

        // The partial products (64-bit results in v0).
        let mul = umul_part;
        let lo = mul(c, S0, S2, 0x003c_63c8)?;
        c.g[S1][0] = lo;
        let m1 = mul(c, S0, S4, 0x003c_63d8)?;
        c.g[S0][0] = m1;
        let m2 = mul(c, S3, S2, 0x003c_63e8)?;
        c.g[S2][0] = m2;
        let hi = mul(c, S3, S4, 0x003c_63f8)?;

        // The carry out of bits 16..31 and the two words.
        c.g[A2][0] = m2 >> 16;
        let carry = ((lo >> 16)
            .wrapping_add(m1 & 0xffff)
            .wrapping_add(m2 & 0xffff))
            >> 16;
        let high = (m1 >> 16)
            .wrapping_add(m2 >> 16)
            .wrapping_add(hi)
            .wrapping_add(carry);
        let low = lo.wrapping_add(m1 << 16).wrapping_add(m2 << 16);
        let (high, low) = (sx(high as u32), sx(low as u32));
        c.g[S0][0] = m1 << 16;
        c.g[S1][0] = low;
        c.g[S2][0] = m2 << 16;
        let words = frame.at(c, 0);
        Addr(words).off(4).sw(c, high);
        Addr(words).sw(c, low);
        let both = Addr(words).read_u64_unaligned(c);
        Addr(words).off(0x10).write_u64_unaligned(c, both);
        let low_word = Addr(words).off(0x10).lwu(c);
        let high_word = Addr(words).off(0x14).lwu(c);
        c.g[A0][0] = low_word;
        c.g[A1][0] = 0xffff_ffff;
        c.g[V1][0] = high_word << 32;
        c.g[V0][0] = (hi & 0xffff_ffff_0000_0000 | low_word) & 0xffff_ffff | high_word << 32;

        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x00120f20 Rider_BeginFrame

/// Fields and callees of [`rider_begin_frame`].
pub mod begin_frame {
    use super::*;

    /// Rider fields.
    pub const POSITION: u32 = 0x110;
    pub const COUNTER_3FC: u32 = 0x3fc;
    /// The bounding box the overlap list is gathered for.
    pub const BOUNDS: u32 = 0x400;
    pub const FINISH_TIME: u32 = 0x470;
    /// The ids of the objects visited this frame (-1: none).
    pub const VISITED_IDS: u32 = 0x5b8;
    pub const MODEL: u32 = 0x780;
    pub const OVERLAPS: u32 = 0x860;
    pub const AI_INDEX: u32 = 0x86c;
    pub const BOARD_ROOT_BONE: u32 = 0x8a0;
    /// The model offset (a quadword, reset from the default at 0x4ff120).
    pub const MODEL_OFFSET: u32 = 0x9d0;
    pub const COUNTER_A30: u32 = 0xa30;
    pub const COUNTER_AA4: u32 = 0xaa4;
    /// In a scripted scene (NIS): the ground is probed below the position,
    /// or below the board's root bone when `PROBE_AT_BONE` is set.
    pub const IN_NIS: u32 = 0xac4;
    pub const PROBE_AT_BONE: u32 = 0xafc;
    /// The model's bone matrices (0x20 bytes each).
    pub const MODEL_BONES: u32 = 0x2c;
    /// The overlap list's stamp, compared with the world tree's.
    pub const LIST_STAMP: u32 = 0x4;

    /// `g_App` (gp-0x848) -> game module (+0x84): the spatial tree (+0x20,
    /// its stamp at +0xa0) and the camera control (+0x34, its mode at
    /// +0x70).
    pub const GP_APP: i32 = -0x848;
    pub const APP_MODULE: u32 = 0x84;
    pub const MODULE_TREE: u32 = 0x20;
    pub const TREE_STAMP: u32 = 0xa0;
    pub const MODULE_CONTROL: u32 = 0x34;
    pub const CONTROL_MODE: u32 = 0x70;
    /// `.lit4` 1/60: the finish clock's step.
    pub const GP_TICK: i32 = -0x7920;
    /// The default model offset quadword.
    pub const DEFAULT_MODEL_OFFSET: u32 = 0x0050_0000 - 0xee0;

    pub const AI_GET_FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::f_001298c8,
    };
    pub const OVERLAP_LIST_GATHER_ALL: u32 = 0x0033_2db8;
    pub const RIDER_FIND_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2088,
        f: ee::s00::f_00122088,
    };
    pub const RIDER_IS_CONTROL_MODE3: u32 = 0x0014_48a8;
    pub const RIDER_PROBE_GROUND: Callee = Callee {
        addr: 0x0012_42b0,
        f: ee::s00::f_001242b0,
    };
    pub const RIDER_UPDATE_TARGET_SPEED: Callee = Callee {
        addr: 0x0011_b3f8,
        f: ee::s00::f_0011b3f8,
    };
}

/// `void Rider_BeginFrame(Rider* r)` (0x120f20): per-tick setup of a rider.
///
/// - Clears two per-frame counters.
/// - Re-gathers the overlap list for the rider's bounds and finds the
///   nearby lights (`OverlapList_GatherAll`, `Rider_FindNearbyLights`) on
///   the rider's turn (frame counter % 3 == AI index % 3), when the list's
///   stamp differs from the world tree's, or when the camera control is in
///   mode 3 or 1.
/// - In a scripted scene, probes the ground below the position (or below
///   the board root bone).
/// - Resets the model offset, a counter and the visited ids (-1), updates
///   the target speed, and advances the finish clock by 1/60 once the
///   rider has finished.
pub fn rider_begin_frame(c: &mut Cpu) {
    use begin_frame::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 14;
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let rider = Addr(r32(c, S0));
        rider.off(COUNTER_3FC).sw(c, 0);
        rider.off(COUNTER_AA4).sw(c, 0);
        AI_GET_FRAME_COUNTER.call(c, 0x0012_0f44)?;

        // Whose turn: frame % 3 against the AI index % 3 (HI of each div).
        c.g[V1][0] = 0x3;
        c.g[A1][0] = 0x3;
        c.div(0, V0, V1);
        c.cycles += 6;
        c.g[A0][0] = rider.off(AI_INDEX).lw(c);
        c.g[A2][0] = c.hi[0];
        c.div(0, A0, A1);
        c.g[V0][0] = c.hi[0];
        let my_turn = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = Addr(gp(c, GP_APP)).lw(c);
        let gather = if my_turn {
            c.cycles += 1;
            true
        } else {
            c.cycles += 7;
            let list = rider.off(OVERLAPS).lw(c);
            c.g[V1][0] = list;
            let module = Addr(c.g[V0][0] as u32).off(APP_MODULE).lw(c);
            c.g[A1][0] = module;
            c.g[A0][0] = Addr(list as u32).off(LIST_STAMP).lw(c);
            c.g[V0][0] = Addr(module as u32).off(MODULE_TREE).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(TREE_STAMP).lw(c);
            if c.g[A0][0] != c.g[V1][0] {
                true
            } else {
                // Up to date: only while the camera control is in mode 3 or 1.
                c.cycles += 6;
                c.g[S1][0] = Addr(module as u32).off(MODULE_CONTROL).lw(c);
                c.g[S2][0] = 0;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, RIDER_IS_CONTROL_MODE3, 0x0012_0f9c)?;
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = 0x1;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S1)).off(CONTROL_MODE).lw(c) ^ 0x1;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = 0x1;
                    }
                }
                c.cycles += 2;
                let busy = c.g[S2][0] != 0;
                if busy {
                    c.cycles += 1;
                }
                busy
            }
        };
        if gather {
            c.g[A0][0] = rider.off(OVERLAPS).lw(c);
            c.cycles += 5;
            c.g[A1][0] = sx(rider.off(BOUNDS).0);
            call_ptr(c, OVERLAP_LIST_GATHER_ALL, 0x0012_0fc8)?;
            c.g[A0][0] = c.g[S0][0];
            RIDER_FIND_NEARBY_LIGHTS.call(c, 0x0012_0fd0)?;
        }

        // A scripted scene: probe the ground.
        c.g[V0][0] = rider.off(IN_NIS).lw(c);
        c.cycles += 2;
        let in_nis = c.g[V0][0] != 0;
        c.g[V0][0] = 0x50_0000;
        if in_nis {
            c.cycles += 3;
            c.g[V0][0] = rider.off(PROBE_AT_BONE).lw(c);
            c.g[A1][0] = sx(rider.off(POSITION).0);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                let model = rider.off(MODEL).lw(c);
                c.g[V0][0] = model;
                let bone = rider.off(BOARD_ROOT_BONE).lw(c);
                let bones = Addr(model as u32).off(MODEL_BONES).lw(c);
                c.g[A0][0] = bones;
                c.g[V1][0] = sx((bone as u32) << 5);
                c.g[A1][0] = sx((bones as u32).wrapping_add((bone as u32) << 5));
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            RIDER_PROBE_GROUND.call(c, 0x0012_1004)?;
            c.g[V0][0] = 0x50_0000;
        }

        // Resets, then the target speed.
        c.cycles += 15;
        c.g[A1][0] = u64::MAX;
        c.g[V0][0] = sx(DEFAULT_MODEL_OFFSET);
        c.lq(V1, DEFAULT_MODEL_OFFSET);
        c.g[A0][0] = c.g[S0][0];
        rider.off(COUNTER_A30).sw(c, 0);
        c.sq(V1, rider.off(MODEL_OFFSET).0);
        rider.off(VISITED_IDS).sw(c, u64::MAX);
        RIDER_UPDATE_TARGET_SPEED.call(c, 0x0012_102c)?;

        // The finish clock runs once the line is crossed.
        let t = rider.off(FINISH_TIME).read_fpu(c);
        abi::set_fpr(c, 1, t);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        let finished = FpuF32::ZERO.le(t);
        abi::set_fcc(c, finished);
        if finished {
            c.cycles += 4;
            let step = Addr(gp(c, GP_TICK)).read_fpu(c);
            let t = t + step;
            abi::set_fpr(c, 0, t);
            rider.off(FINISH_TIME).write_fpu(c, t);
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x0012e9b8 RiderPrewindSub_HandleInput

/// Fields and callees of [`rider_prewind_sub_handle_input`].
pub mod prewind_input {
    use super::*;

    /// `RiderPrewindSub`: the rider.
    pub const RIDER: u32 = 0x0;
    /// The pad: buttons (bit 12 reset, 13 prewind held, 14 boost; the
    /// trick stick's 6-bit axes at bits 15 and 21) and sticks (6-bit axes
    /// at bits 0 and 6).
    pub const PAD_BUTTONS: u32 = 0x0;
    pub const PAD_STICKS: u32 = 0x4;

    /// Rider fields: the launch speed, the lean (slew 0: value, rate,
    /// target) and pitch (slew 5) controls, the turn input, the prewind
    /// direction (0 lean, else pitch; 3 and 4 tilt the animation), the
    /// animation controller.
    pub const LAUNCH_SPEED: u32 = 0x220;
    pub const SLEW0_VALUE: u32 = 0x2a4;
    pub const SLEW0_RATE: u32 = 0x2a8;
    pub const SLEW0_TARGET: u32 = 0x2ac;
    pub const SLEW5_VALUE: u32 = 0x2b0;
    pub const SLEW5_RATE: u32 = 0x2b4;
    pub const SLEW5_TARGET: u32 = 0x2b8;
    pub const TURN_INPUT: u32 = 0x2dc;
    pub const PREWIND_DIR: u32 = 0x328;
    pub const ANIM_CTRL: u32 = 0x784;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    pub const INPUT_QUANTIZE_STICK_DIRECTION: Callee = Callee {
        addr: 0x0011_58b8,
        f: ee::s00::f_001158b8,
    };
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };
    pub const RIDER_EMPTY_STANCE_HOOK: Callee = Callee {
        addr: 0x0011_6930,
        f: ee::s00::f_00116930,
    };
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::f_0011fe98,
    };
    pub const RIDER_HANDLE_BOOST_INPUT: Callee = Callee {
        addr: 0x0011_4130,
        f: ee::s00::f_00114130,
    };
    pub const RIDER_LAUNCH_INTO_AIR: Callee = Callee {
        addr: 0x0011_4298,
        f: ee::s00::f_00114298,
    };
    pub const RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM: Callee = Callee {
        addr: 0x0012_ee30,
        f: ee::s00::f_0012ee30,
    };
    pub const RIDER_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::f_0011fe78,
    };
    pub const RIDER_SET_SLEW0_TARGET: Callee = Callee {
        addr: 0x0011_3e80,
        f: ee::s00::f_00113e80,
    };
    pub const RIDER_SET_SLEW3_AND4_TARGETS: Callee = Callee {
        addr: 0x0011_3f88,
        f: ee::s00::f_00113f88,
    };
    pub const RIDER_SET_SLEW5_TARGET: Callee = Callee {
        addr: 0x0011_3f38,
        f: ee::s00::f_00113f38,
    };
    pub const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::f_0011fec8,
    };
    pub const RIDER_TRY_ENTER_FINISH_LINE: Callee = Callee {
        addr: 0x0011_6378,
        f: ee::s00::f_00116378,
    };
    pub const RIDER_TRY_START_RAILSLIDE: Callee = Callee {
        addr: 0x0010_6848,
        f: ee::s00::f_00106848,
    };
    pub const RIDER_TURN_AROUND_IF_BACKWARD: Callee = Callee {
        addr: 0x0011_4cc0,
        f: ee::s00::f_00114cc0,
    };
    pub const SSX_AUDIO_SYSTEM_PLAY_TAKEOFF_SOUND: u32 = 0x0029_4170;
}

/// `void RiderPrewindSub_HandleInput(RiderPrewindSub* this, Pad* pad)`
/// (0x12e9b8): the input of a rider crouching for a jump (prewind), once
/// per tick.
///
/// - Crossing the finish line, a path reset (pad bit 12) or a rail jump
///   end the function.
/// - Released (bit 13 clear): the stick direction is quantized into the
///   lean / pitch slews; prewind directions 3 and 4 reset the animation
///   tilt (`Math_SinCos` of -0); the release animation (table 0x43d840);
///   direction 0; unless already airborne (main state 1) the rider
///   launches (`Rider_LaunchIntoAir` at `+0x220`, main state 1, take-off
///   sound); slews zeroed, sub-state 5.
/// - Held: boost input (bit 14), slews 3/4 to (1, 0); the stick axes
///   (scaled, clamped to ±0.5) lean or pitch. On a kind-10 layer-2
///   animation or anim id 0x15 the slews are parked (rate 1/12); a centred
///   rider turning around plays anim 0x15; otherwise the trick stick's
///   axes (±0.2 dead zone, to -1/0/1) set the slew targets with rates by
///   the distance (5 per second), and the directional animation (table
///   0x43d788) plays.
#[allow(unused_labels)]
pub fn rider_prewind_sub_handle_input(c: &mut Cpu) {
    use prewind_input::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        'b_12ee0c: {
            c.cycles += 14;
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S1, 0x40);
            c.g[S2][0] = c.g[A0][0];
            frame.save_ra(c, 0x20);
            frame.save_fpr(c, 22, 0x70);
            frame.save_fpr(c, 21, 0x68);
            frame.save_fpr(c, 20, 0x60);
            c.g[A0][0] = c.lw(at(c, S2, RIDER));
            RIDER_TRY_ENTER_FINISH_LINE.call(c, 0x0012e9e8)?;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x50);
                break 'b_12ee0c;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                RIDER_CHECK_RESET_PATH.call(c, 0x0012ea08)?;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x50);
                    break 'b_12ee0c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                    RIDER_TRY_START_RAILSLIDE.call(c, 0x0012ea18)?;
                    if c.g[V0][0] != 0u64 {
                        frame.restore(c, S0, 0x50);
                        break 'b_12ee0c;
                    } else {
                        'b_12eb50: {
                            c.cycles += 4;
                            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                            c.g[V0][0] = c.g[A1][0] & 0x2000;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
                            if t {
                                break 'b_12eb50;
                            } else {
                                'b_12eac4: {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(12, gp(c, -0x7790));
                                    c.lwc1(13, gp(c, -0x778c));
                                    c.g[A1][0] = sx(at(c, A0, SLEW5_VALUE));
                                    c.g[A2][0] = sx(at(c, A0, SLEW0_VALUE));
                                    INPUT_QUANTIZE_STICK_DIRECTION.call(c, 0x0012ea48)?;
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.g[V0][0] = c.lw(at(c, V1, PREWIND_DIR));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x500000;
                                    if t {
                                        break 'b_12eac4;
                                    } else {
                                        c.cycles += 25;
                                        c.mtc1(0, 12);
                                        c.g[S1][0] = c.lw(at(c, V1, ANIM_CTRL));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
                                        c.lq(3, r32(c, V0));
                                        c.neg_s(12, 12);
                                        c.g[A0][0] = sx(at(c, SP, 0x10));
                                        c.g[A1][0] = sx(at(c, SP, 0x14));
                                        c.sq(3, at(c, S1, 0x30));
                                        c.g[S0][0] = 0x500000;
                                        call_ptr(c, MATH_SIN_COS, 0x0012ea88)?;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
                                        c.lwc1(0, frame.at(c, 0x10));
                                        c.lwc1(1, at(c, V0, 0x8));
                                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xea0));
                                        c.lwc1(4, at(c, V0, 0x4));
                                        c.mul_s(1, 0, 1);
                                        c.mul_s(2, 0, 2);
                                        c.lwc1(3, frame.at(c, 0x14));
                                        c.mul_s(0, 0, 4);
                                        c.swc1(3, frame.at(c, 0xc));
                                        c.swc1(1, frame.at(c, 0x8));
                                        c.swc1(2, frame.at(c, 0));
                                        c.swc1(0, frame.at(c, 0x4));
                                        c.lq(2, frame.at(c, 0));
                                        c.sq(2, at(c, S1, 0x40));
                                        break 'b_12eac4;
                                    }
                                }
                                'b_12eb1c: {
                                    c.cycles += 13;
                                    c.g[A1][0] = 0x440000;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x27c0));
                                    RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM.call(c, 0x0012ead4)?;
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.sw(at(c, V0, PREWIND_DIR), 0u64);
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    RIDER_EMPTY_STANCE_HOOK.call(c, 0x0012eae4)?;
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    RIDER_GET_MAIN_STATE.call(c, 0x0012eaec)?;
                                    c.g[V1][0] = 0x1;
                                    let t = c.g[V0][0] == c.g[V1][0];

                                    if t {
                                        break 'b_12eb1c;
                                    } else {
                                        c.cycles += 9;
                                        c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                        c.lwc1(12, at(c, A0, LAUNCH_SPEED));
                                        RIDER_LAUNCH_INTO_AIR.call(c, 0x0012eb04)?;
                                        c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                        c.g[A1][0] = 0x1;
                                        RIDER_SET_MAIN_STATE.call(c, 0x0012eb10)?;
                                        c.g[A0][0] = c.lw(gp(c, 0x410));
                                        c.g[A1][0] = c.lw(at(c, S2, RIDER));
                                        call_ptr(
                                            c,
                                            SSX_AUDIO_SYSTEM_PLAY_TAKEOFF_SOUND,
                                            0x0012eb1c,
                                        )?;
                                        break 'b_12eb1c;
                                    }
                                }
                                c.cycles += 13;
                                c.mtc1(0, 20);
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.mov_s(12, 20);
                                RIDER_SET_SLEW0_TARGET.call(c, 0x0012eb2c)?;
                                c.mov_s(12, 20);
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.mov_s(13, 12);
                                RIDER_SET_SLEW3_AND4_TARGETS.call(c, 0x0012eb3c)?;
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.g[A1][0] = 0x5;
                                RIDER_SET_SUB_STATE.call(c, 0x0012eb48)?;
                                frame.restore(c, S0, 0x50);
                                break 'b_12ee0c;
                            }
                        }
                        'b_12ebb8: {
                            c.cycles += 25;
                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                            c.g[A1][0] = c.g[A1][0] & 0x1;
                            c.g[A2][0] = 0u64;
                            RIDER_HANDLE_BOOST_INPUT.call(c, 0x0012eb60)?;
                            c.mtc1(0, 20);
                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 12);
                            c.mov_s(13, 20);
                            RIDER_SET_SLEW3_AND4_TARGETS.call(c, 0x0012eb78)?;
                            c.g[V1][0] = c.lw(at(c, S0, PAD_STICKS));
                            c.lwc1(3, gp(c, -0x7788));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 26);
                            c.g[AT][0] = 0xffffffffbf000000;
                            c.mtc1(1, 2);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 1);
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 3);
                            c.c_le_s(2, 0);
                            let t = !c.fcc();
                            c.mov_s(22, 2);
                            if t {
                                break 'b_12ebb8;
                            } else {
                                c.cycles += 1;
                                c.min_s(22, 0, 1);
                                break 'b_12ebb8;
                            }
                        }
                        'b_12ebe0: {
                            c.cycles += 9;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 20);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 3);
                            c.c_le_s(2, 0);
                            let t = !c.fcc();
                            c.mov_s(21, 2);
                            if t {
                                break 'b_12ebe0;
                            } else {
                                c.cycles += 1;
                                c.min_s(21, 0, 1);
                                break 'b_12ebe0;
                            }
                        }
                        'b_12ee08: {
                            'b_12ec44: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x0012ebf0)?;
                                c.g[V1][0] = 0xa;
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    break 'b_12ec44;
                                } else {
                                    'b_12ec28: {
                                        'b_12ec1c: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                            c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                            let t = c.g[V0][0] != 0u64;

                                            if t {
                                                break 'b_12ec1c;
                                            } else {
                                                c.cycles += 4;
                                                c.mov_s(12, 22);
                                                RIDER_SET_SLEW0_TARGET.call(c, 0x0012ec14)?;
                                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                                break 'b_12ec28;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.mov_s(12, 21);
                                        RIDER_SET_SLEW5_TARGET.call(c, 0x0012ec24)?;
                                        c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ec28;
                                    }
                                    c.cycles += 7;
                                    c.lwc1(0, gp(c, -0x7784));
                                    c.sw(at(c, V0, SLEW0_TARGET), 0u64);
                                    c.swc1(0, at(c, V0, SLEW0_RATE));
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.swc1(0, at(c, V1, SLEW5_RATE));
                                    c.sw(at(c, V1, SLEW5_TARGET), 0u64);
                                    break 'b_12ee08;
                                }
                            }
                            'b_12ec84: {
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ID, 0x0012ec50)?;
                                c.g[V1][0] = 0x15;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                if t {
                                    break 'b_12ec84;
                                } else {
                                    c.cycles += 10;
                                    c.mov_s(12, 20);
                                    RIDER_SET_SLEW0_TARGET.call(c, 0x0012ec64)?;
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(0, gp(c, -0x7780));
                                    c.swc1(20, at(c, V0, SLEW0_TARGET));
                                    c.swc1(0, at(c, V0, SLEW0_RATE));
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.swc1(0, at(c, V1, SLEW5_RATE));
                                    c.swc1(20, at(c, V1, SLEW5_TARGET));
                                    break 'b_12ee08;
                                }
                            }
                            'b_12ecd8: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    break 'b_12ecd8;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, at(c, A0, TURN_INPUT));
                                    c.c_eq_s(0, 20);
                                    if !c.fcc() {
                                        c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ecd8;
                                    } else {
                                        'b_12ecd4: {
                                            c.cycles += 4;
                                            RIDER_TURN_AROUND_IF_BACKWARD.call(c, 0x0012ecac)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x15;
                                            if t {
                                                break 'b_12ecd4;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[A2][0] = 0u64;
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x0012eccc)?;
                                                frame.restore(c, S0, 0x50);
                                                break 'b_12ee0c;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ecd8;
                                    }
                                }
                            }
                            'b_12ecec: {
                                c.cycles += 4;
                                c.lwc1(5, gp(c, -0x777c));
                                c.g[V0][0] = c.lw(at(c, V1, PREWIND_DIR));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.g[V1][0];
                                if t {
                                    break 'b_12ecec;
                                } else {
                                    c.cycles += 1;
                                    c.lwc1(5, gp(c, -0x7778));
                                    break 'b_12ecec;
                                }
                            }
                            'b_12ed58: {
                                'b_12ed3c: {
                                    c.cycles += 16;
                                    c.g[V1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                                    c.lwc1(3, gp(c, -0x7774));
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 11);
                                    c.lwc1(2, gp(c, -0x7770));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.mtc1(2, 1);
                                    c.cvt_s_w(1, 1);
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                                    c.mtc1(3, 0);
                                    c.cvt_s_w(0, 0);
                                    c.mul_s(4, 1, 3);
                                    c.c_lt_s(4, 2);
                                    let t = !c.fcc();
                                    c.mul_s(3, 0, 3);
                                    if t {
                                        break 'b_12ed3c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 4);

                                        break 'b_12ed58;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x776c));
                                c.c_lt_s(0, 4);
                                c.mtc1(0, 4);
                                let t = !c.fcc();

                                if t {
                                    break 'b_12ed58;
                                } else {
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 4);
                                    break 'b_12ed58;
                                }
                            }
                            'b_12ed94: {
                                'b_12ed78: {
                                    c.cycles += 4;
                                    c.lwc1(0, gp(c, -0x7768));
                                    c.c_lt_s(3, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_12ed78;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 3);
                                        c.lwc1(0, at(c, A0, SLEW0_VALUE));
                                        break 'b_12ed94;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x7764));
                                c.c_lt_s(0, 3);
                                c.mtc1(0, 3);
                                let t = !c.fcc();
                                c.lwc1(0, at(c, A0, SLEW0_VALUE));
                                if t {
                                    break 'b_12ed94;
                                } else {
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 3);
                                    break 'b_12ed94;
                                }
                            }
                            'b_12edfc: {
                                'b_12edf0: {
                                    c.cycles += 19;
                                    c.lwc1(1, gp(c, -0x7760));
                                    c.sub_s(0, 4, 0);
                                    c.swc1(4, at(c, A0, SLEW0_TARGET));
                                    c.abs_s(0, 0);
                                    c.mul_s(0, 5, 0);
                                    c.mul_s(0, 0, 1);
                                    c.swc1(0, at(c, A0, SLEW0_RATE));
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(0, at(c, V0, SLEW5_VALUE));
                                    c.swc1(3, at(c, V0, SLEW5_TARGET));
                                    c.sub_s(0, 3, 0);
                                    c.abs_s(0, 0);
                                    c.mul_s(0, 5, 0);
                                    c.mul_s(0, 0, 1);
                                    c.swc1(0, at(c, V0, SLEW5_RATE));
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_12edf0;
                                    } else {
                                        c.cycles += 4;
                                        c.mov_s(12, 22);
                                        RIDER_SET_SLEW0_TARGET.call(c, 0x0012ede8)?;
                                        c.g[A1][0] = 0x440000;
                                        break 'b_12edfc;
                                    }
                                }
                                c.cycles += 3;
                                c.mov_s(12, 21);
                                RIDER_SET_SLEW5_TARGET.call(c, 0x0012edf8)?;
                                c.g[A1][0] = 0x440000;
                                break 'b_12edfc;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2878));
                            RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM.call(c, 0x0012ee08)?;
                            break 'b_12ee08;
                        }
                        c.cycles += 1;
                        frame.restore(c, S0, 0x50);
                        break 'b_12ee0c;
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x0010def0 ComputerActorRider_ComputeRubberBand

/// Fields and callees of [`computer_actor_rider_compute_rubber_band`].
pub mod rubber_band {
    use super::*;

    /// The computer actor's rubber-band parameters (`+0x18`): the catch-up
    /// and hold-back distances, the mode (0 off, 1 hold back, 2 catch up,
    /// 3 both), its own distance to the finish.
    pub const PARAMS: u32 = 0x18;
    pub const CATCH_UP_DIST: u32 = 0xdc;
    pub const HOLD_BACK_DIST: u32 = 0xe0;
    pub const MODE: u32 = 0xe4;
    pub const DIST_TO_FINISH: u32 = 0x4d0;
    /// The game-mode byte (2: no rubber band) at 0x535c11.
    pub const GAME_MODE: u32 = 0x0053_0000 + 0x5c11;
    /// `.lit4`: the minimum distance (50000), the catch-up and hold-back
    /// gains (1.1, 1/1.1).
    pub const GP_MIN_DIST: i32 = -0x7cc4;
    pub const GP_CATCH_UP_GAIN: i32 = -0x7cc0;
    pub const GP_HOLD_BACK_GAIN: i32 = -0x7cbc;

    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const COMPUTER_ACTOR_RIDER_GET_HUMAN_RIDER: Callee = Callee {
        addr: 0x0010_deb0,
        f: ee::s00::f_0010deb0,
    };
    pub const RIDER_GET_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3128,
        f: ee::s00::f_00113128,
    };
}

/// `float ComputerActorRider_ComputeRubberBand(ComputerActorRider* this)`
/// (0x10def0): the computer rider's speed factor (f0) against the human
/// rider: 1 without a human, with the mode off, in game mode 2, or within
/// 50000 of the finish. Otherwise with `d` = the human's distance to the
/// finish minus its own: catching up (`d < catch_up`, mode 2/3) gives
/// `1.1 * (1 + (d - k) / k)`; holding back (`hold_back < d`, mode 1/3)
/// `(1/1.1) / (1 + (d - k2) / k2)`; else 1.
pub fn computer_actor_rider_compute_rubber_band(c: &mut Cpu) {
    use rubber_band::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 13;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x0010_df04)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0010_df10)?;
        c.g[A0][0] = c.g[S0][0];
        COMPUTER_ACTOR_RIDER_GET_HUMAN_RIDER.call(c, 0x0010_df18)?;
        c.g[A2][0] = c.g[V0][0];
        let this = Addr(r32(c, S0));

        // Whether the rubber band applies at all.
        let mut applies = false;
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            let params = this.off(PARAMS).lw(c);
            c.g[A1][0] = params;
            let mode = Addr(params as u32).off(MODE).lw(c);
            c.g[V0][0] = 0x53_0000;
            if mode != 0 {
                c.cycles += 4;
                c.g[A0][0] = 0x2;
                c.g[V1][0] = Addr(GAME_MODE).lb(c);
                if c.g[V1][0] != 0x2 {
                    c.cycles += 5;
                    let own = Addr(params as u32).off(DIST_TO_FINISH).read_fpu(c);
                    let min = Addr(gp(c, GP_MIN_DIST)).read_fpu(c);
                    abi::set_fpr(c, 1, own);
                    abi::set_fpr(c, 0, min);
                    applies = !abi::c_lt_s(c, own, min);
                }
            }
        }
        if !applies {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            abi::set_fpr(c, 0, FpuF32::ONE);
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // d = the human's distance to the finish minus ours.
        c.cycles += 10;
        c.g[A0][0] = c.g[A2][0];
        RIDER_GET_DIST_TO_FINISH.call(c, 0x0010_df70)?;
        let params = Addr(this.off(PARAMS).lw(c) as u32);
        c.g[V0][0] = sx(params.0);
        let own = params.off(DIST_TO_FINISH).read_fpu(c);
        let k = params.off(CATCH_UP_DIST).read_fpu(c);
        abi::set_fpr(c, 1, own);
        abi::set_fpr(c, 3, k);
        let d = FpuF32(c.f[0]) - own;
        abi::set_fpr(c, 0, d);
        let mut factor = None;
        if abi::c_lt_s(c, d, k) {
            c.cycles += 5;
            let mode = params.off(MODE).lw(c);
            let catch_up = sx((mode as u32).wrapping_sub(2)) < 2;
            c.g[V0][0] = catch_up as u64;
            if catch_up {
                c.cycles += 8;
                c.g[AT][0] = 0x3f80_0000;
                abi::set_fpr(c, 1, FpuF32::ONE);
                let gain = Addr(gp(c, GP_CATCH_UP_GAIN)).read_fpu(c);
                abi::set_fpr(c, 2, gain);
                factor = Some(((d - k).div(k) + FpuF32::ONE) * gain);
            }
        }
        let factor = match factor {
            Some(f) => f,
            None => {
                c.g[A0][0] = sx(params.0);
                c.cycles += 5;
                let k2 = params.off(HOLD_BACK_DIST).read_fpu(c);
                abi::set_fpr(c, 3, k2);
                let beyond = abi::c_lt_s(c, k2, d);
                c.g[V0][0] = 0x1;
                let mut hold_back = false;
                if beyond {
                    c.cycles += 3;
                    let mode = params.off(MODE).lw(c);
                    c.g[A0][0] = mode;
                    c.g[V0][0] = 0x3;
                    if mode == 1 {
                        hold_back = true;
                    } else {
                        c.cycles += 2;
                        hold_back = mode == 3;
                    }
                }
                if hold_back {
                    c.cycles += 9;
                    c.g[AT][0] = 0x3f80_0000;
                    abi::set_fpr(c, 1, FpuF32::ONE);
                    let gain = Addr(gp(c, GP_HOLD_BACK_GAIN)).read_fpu(c);
                    abi::set_fpr(c, 2, gain);
                    FpuF32::ONE.div((d - k2).div(k2) + FpuF32::ONE) * gain
                } else {
                    c.cycles += 2;
                    c.g[AT][0] = 0x3f80_0000;
                    FpuF32::ONE
                }
            }
        };
        abi::set_fpr(c, 0, factor);
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x002ac7f0 BankMonitorInstance_UpdateRange

/// Fields and callees of [`bank_monitor_instance_update_range`].
pub mod bank_range {
    use super::*;

    /// `BankMonitorInstance`: playing flag, audible range (0: always on).
    pub const PLAYING: u32 = 0x8;
    pub const RANGE: u32 = 0x8c;

    pub const BANK_MONITOR_INSTANCE_FADE_OUT: Callee = Callee {
        addr: 0x002a_c180,
        f: ee::s08::f_002ac180,
    };
    pub const BANK_MONITOR_INSTANCE_PLAY: Callee = Callee {
        addr: 0x002a_c220,
        f: ee::s08::f_002ac220,
    };
}

/// `bool BankMonitorInstance_UpdateRange(BankMonitorInstance* this, float
/// dist)` (0x2ac7f0): starts or fades a positional sound by distance.
/// Without a range: 1. Playing: beyond the range it fades out (0), else 1.
/// Stopped: inside the range it starts playing (1), else 0.
pub fn bank_monitor_instance_update_range(c: &mut Cpu) {
    use bank_range::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 8;
        frame.save_ra(c, 0);
        let this = Addr(abi::arg(c, 0));
        let dist = abi::farg(c, 0);
        let range = this.off(RANGE).read_fpu(c);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        abi::set_fpr(c, 1, range);
        let no_range = range.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_range);
        c.g[V0][0] = 0x1;
        if !no_range {
            c.cycles += 3;
            c.g[V0][0] = this.off(PLAYING).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let beyond = abi::c_lt_s(c, range, dist);
                c.g[V0][0] = 0x1;
                if beyond {
                    c.cycles += 4;
                    BANK_MONITOR_INSTANCE_FADE_OUT.call(c, 0x002a_c834)?;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 4;
                let inside = abi::c_lt_s(c, dist, range);
                c.g[V0][0] = 0;
                if inside {
                    c.cycles += 3;
                    BANK_MONITOR_INSTANCE_PLAY.call(c, 0x002a_c854)?;
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ===========================================================================
// 0x001291e0 AI_Render

/// Fields and callees of [`ai_render`].
pub mod ai_render_fields {
    use super::*;

    /// The AI manager: the riders whose light slot is checked (count,
    /// array), whether only one view is drawn, the human actors (count,
    /// array of two), the viewed actor, the computer actors (shown?, count,
    /// array), the other actors (shown?, count, array). Actors point at
    /// their rider at +0x18.
    pub const SHOW_COMPUTERS: u32 = 0x14;
    pub const RENDER_LIST: u32 = 0x28;
    pub const HUMANS: u32 = 0x40;
    pub const COMPUTERS: u32 = 0x48;
    pub const OTHERS: u32 = 0x5c;
    pub const VIEWED: u32 = 0x70;
    pub const RENDER_COUNT: u32 = 0x78;
    pub const HUMAN_COUNT: u32 = 0x7c;
    pub const COMPUTER_COUNT: u32 = 0x80;
    pub const OTHER_COUNT: u32 = 0x84;
    pub const SINGLE_VIEW: u32 = 0x88;
    pub const SHOW_OTHERS: u32 = 0x8c;
    pub const ACTOR_RIDER: u32 = 0x18;

    /// Rider fields: its owner (`RiderBase`, the effects), visible flag,
    /// light slot (-1: none yet).
    pub const RIDER_OWNER: u32 = 0x77c;
    pub const RIDER_VISIBLE: u32 = 0x884;
    pub const RIDER_LIGHT_SLOT: u32 = 0x888;

    /// The owner's effects.
    pub const WAKE_FX: u32 = 0x3b0;
    pub const SPARK_FX: u32 = 0x470;
    pub const TRAIL_FX: u32 = 0x520;
    pub const SPEED_STREAMER_FX: u32 = 0x610;
    pub const POWER_UP_TRAIL: u32 = 0x9c0;
    pub const AIR_STREAMER_FX: u32 = 0xad0;
    pub const ANGER_FX: u32 = 0xaf0;
    pub const PULSE_FX: u32 = 0xb00;
    pub const SPRAY_FX: u32 = 0xb40;
    pub const ATTACK_FX: u32 = 0xc70;
    pub const CHEAT_CODE: u32 = 0xd20;

    /// The graphics manager (gp-0x854) and its vtable's light-slot
    /// allocator; `g_App` (gp-0x848) -> game module -> camera control (its
    /// mode at +0x70).
    pub const GP_GFX: i32 = -0x854;
    pub const GFX_VTABLE: u32 = 0x10d8;
    pub const GFX_ALLOC_LIGHT_SLOT: u32 = 0x390;
    pub const GP_APP: i32 = -0x848;
    pub const APP_MODULE: u32 = 0x84;
    pub const MODULE_CONTROL: u32 = 0x34;
    pub const CONTROL_MODE: u32 = 0x70;

    pub const C_AIR_STREAMER_FX_RENDER: u32 = 0x002e_f950;
    pub const C_ANGER_FX_RENDER: u32 = 0x002d_5048;
    pub const C_BOARD_SPARK_FX_RENDER: u32 = 0x002d_b478;
    pub const C_BOARD_TRAIL_FX_RENDER_BATCH: u32 = 0x002e_a538;
    pub const C_BOARD_WAKE_FX_RENDER: u32 = 0x002d_dab8;
    pub const C_DYNAMIC_SPRAY_FX_RENDER: u32 = 0x002e_24d0;
    pub const C_PLAYER_PULSE_FX_RENDER: u32 = 0x002e_3af8;
    pub const C_SPEED_BOOST_STREAMER_FX_RENDER: u32 = 0x002e_7a10;
    pub const RIDER_BASE_RENDER_ATTACK_FX: u32 = 0x002f_1510;
    pub const RIDER_BASE_RENDER_CHEAT_CODE: u32 = 0x002f_6908;
    pub const RIDER_BASE_RENDER_POWER_UP_TRAIL: u32 = 0x002e_b198;
    pub const RIDER_PREPARE_LIGHTING: Callee = Callee {
        addr: 0x0012_25c0,
        f: ee::s00::f_001225c0,
    };
    pub const RIDER_RENDER_ATTACHED_FX: Callee = Callee {
        addr: 0x0012_2638,
        f: ee::s00::f_00122638,
    };
    pub const RIDER_RENDER_MODEL: Callee = Callee {
        addr: 0x0012_2448,
        f: ee::s00::f_00122448,
    };
    pub const RIDER_RENDER_SHADOW: Callee = Callee {
        addr: 0x0012_25f0,
        f: ee::s00::f_001225f0,
    };
    pub const RIDER_UPDATE_VISIBILITY: Callee = Callee {
        addr: 0x0012_2278,
        f: ee::s00::f_00122278,
    };
}

/// `void AI_Render(AI* this)` (0x1291e0): draws the riders of a frame.
///
/// - Riders without a light slot (not visible, slot -1) get one from the
///   graphics manager (+4).
/// - The riders drawn (up to 8, on the stack): in a single view with
///   humans, the first human's rider (camera control mode 0) or the viewed
///   actor's; else every human's, and the computers' and others' when
///   shown (null actors give null riders).
/// - Per rider, in passes: visibility (`Rider_UpdateVisibility`, kept on
///   the stack with the owner), lighting, model and shadow of the visible
///   ones, the board trails as one batch, the owner's pulse, cheat code,
///   wake, air streamer, speed streamer and power-up trail effects; the
///   sparks, anger, spray and attack effects of the visible ones; the
///   attached effects of all.
#[allow(unused_labels)]
pub fn ai_render(c: &mut Cpu) {
    use ai_render_fields::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x100);
        'b_12926c: {
            c.cycles += 12;
            frame.save(c, S1, 0xe0);
            frame.save(c, S2, 0xd0);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0xb0);
            c.g[S2][0] = 0u64;
            frame.save(c, S0, 0xf0);
            frame.save(c, S3, 0xc0);
            frame.save_ra(c, 0xa0);
            c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S4][0] = sx(at(c, SP, 0x60));
            if t {
                break 'b_12926c;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0xffffffffffffffff;
                c.g[S0][0] = sx(at(c, S1, RENDER_LIST));
                'l_129218: loop {
                    'b_12925c: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(r32(c, S0));
                        c.g[V0][0] = c.lw(at(c, V1, RIDER_VISIBLE));
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                            break 'b_12925c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, V1, RIDER_LIGHT_SLOT));
                            if c.g[V0][0] != c.g[S3][0] {
                                c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                                break 'b_12925c;
                            } else {
                                c.cycles += 10;
                                c.g[A1][0] = c.lw(gp(c, GP_GFX));
                                c.g[V1][0] = c.lw(at(c, A1, GFX_VTABLE));
                                c.g[A0][0] = c.lh(at(c, V1, GFX_ALLOC_LIGHT_SLOT));
                                c.g[V0][0] = c.lw(at(c, V1, GFX_ALLOC_LIGHT_SLOT + 4));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(r32(c, A0)));
                                call_ptr(c, t, 0x0012924c)?;
                                c.g[V1][0] = c.lw(r32(c, S0));
                                c.g[V0][0] = sx(at(c, V0, 0x4));
                                c.sw(at(c, V1, RIDER_LIGHT_SLOT), c.g[V0][0]);
                                c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                                break 'b_12925c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx(at(c, S0, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129218);
                        }
                        continue 'l_129218;
                    } else {
                        break 'b_12926c;
                    }
                }
            }
        }
        'b_1293c0: {
            'b_1292dc: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S1, SINGLE_VIEW));
                c.g[S2][0] = 0u64;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V1][0] = c.lw(at(c, S1, HUMAN_COUNT));
                if t {
                    break 'b_1292dc;
                } else {
                    c.cycles += 2;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = c.lw(gp(c, GP_APP));
                    if t {
                        break 'b_1292dc;
                    } else {
                        'b_1292bc: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(at(c, V0, APP_MODULE));
                            c.g[V0][0] = c.lw(at(c, V1, MODULE_CONTROL));
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                break 'b_1292bc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, V0, CONTROL_MODE));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                if t {
                                    break 'b_1292bc;
                                } else {
                                    'b_1292b4: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw(at(c, S1, HUMANS));
                                        c.g[S2][0] = 0x1;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1292b4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = c.lw(at(c, A0, ACTOR_RIDER));
                                            break 'b_1292b4;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.sw(r32(c, SP), c.g[V0][0]);
                                    break 'b_1293c0;
                                }
                            }
                        }
                        'b_1292d4: {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw(at(c, S1, VIEWED));
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(r32(c, V0)));
                            c.g[S2][0] = 0x1;
                            let t = c.g[A0][0] == 0u64;
                            c.g[V1][0] = 0u64;
                            if t {
                                break 'b_1292d4;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = c.lw(at(c, A0, ACTOR_RIDER));
                                break 'b_1292d4;
                            }
                        }
                        c.cycles += 2;
                        c.sw(r32(c, V0), c.g[V1][0]);
                        break 'b_1293c0;
                    }
                }
            }
            'b_129320: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_129320;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                    c.g[S0][0] = c.lw(at(c, S1, HUMAN_COUNT));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = sx(at(c, S1, HUMANS));
                    'l_1292f8: loop {
                        'b_129310: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(r32(c, A1));
                            c.g[V1][0] = sx(at(c, V1, 0x4));
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_129310;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                break 'b_129310;
                            }
                        }
                        c.cycles += 4;
                        c.sw(r32(c, V1), c.g[A0][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S0][0] != 0u64;
                        c.g[A1][0] = sx(at(c, A1, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001292f8);
                            }
                            continue 'l_1292f8;
                        } else {
                            break 'b_129320;
                        }
                    }
                }
            }
            'b_129374: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(c, S1, SHOW_COMPUTERS));
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.lw(at(c, S1, SHOW_OTHERS));
                    break 'b_129374;
                } else {
                    'b_129370: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(at(c, S1, COMPUTER_COUNT));
                        let t = (c.g[A0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                        if t {
                            break 'b_129370;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                            c.g[S0][0] = c.g[A0][0];
                            c.g[A1][0] = sx(at(c, S1, COMPUTERS));
                            'l_129348: loop {
                                'b_129360: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw(r32(c, A1));
                                    c.g[V1][0] = sx(at(c, V1, 0x4));
                                    c.g[S2][0] = sx(at(c, S2, 0x1));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0u64;
                                    if t {
                                        break 'b_129360;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                        break 'b_129360;
                                    }
                                }
                                c.cycles += 4;
                                c.sw(r32(c, V1), c.g[A0][0]);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S0][0] != 0u64;
                                c.g[A1][0] = sx(at(c, A1, 0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00129348);
                                    }
                                    continue 'l_129348;
                                } else {
                                    break 'b_129370;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S1, SHOW_OTHERS));
                    break 'b_129374;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1293c0;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.lw(at(c, S1, OTHER_COUNT));
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_1293c0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = sx(at(c, S1, OTHERS));
                    c.g[S0][0] = c.g[A1][0];
                    'l_129398: loop {
                        'b_1293b0: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(at(c, V1, 0x4));
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1293b0;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                break 'b_1293b0;
                            }
                        }
                        c.cycles += 4;
                        c.sw(r32(c, V1), c.g[A1][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S0][0] != 0u64;
                        c.g[A0][0] = sx(at(c, A0, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00129398);
                            }
                            continue 'l_129398;
                        } else {
                            break 'b_1293c0;
                        }
                    }
                }
            }
        }
        'b_1293f4: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1293f4;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_1293d0: loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(r32(c, S1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    RIDER_UPDATE_VISIBILITY.call(c, 0x001293dc)?;
                    c.sw(at(c, S1, 0x40), c.g[V0][0]);
                    c.g[V0][0] = c.lw(r32(c, S1));
                    c.g[V1][0] = c.lw(at(c, V0, RIDER_OWNER));
                    c.sw(at(c, S1, 0x20), c.g[V1][0]);
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001293d0);
                        }
                        continue 'l_1293d0;
                    } else {
                        break 'b_1293f4;
                    }
                }
            }
        }
        'b_129428: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129428;
            } else {
                c.cycles += 3;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129408: loop {
                    'b_129420: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129420;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_PREPARE_LIGHTING.call(c, 0x0012941c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129420;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129408);
                        }
                        continue 'l_129408;
                    } else {
                        break 'b_129428;
                    }
                }
            }
        }
        'b_129458: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129458;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129438: loop {
                    'b_129450: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129450;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_RENDER_MODEL.call(c, 0x0012944c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129450;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129438);
                        }
                        continue 'l_129438;
                    } else {
                        break 'b_129458;
                    }
                }
            }
        }
        'b_1294b8: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[A1][0] = c.g[S4][0];
            if t {
                break 'b_1294b8;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129468: loop {
                    'b_129480: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129480;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_RENDER_SHADOW.call(c, 0x0012947c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129480;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129468);
                        }
                        continue 'l_129468;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S2][0] as i64) <= 0;
                        c.g[A1][0] = c.g[S4][0];
                        if t {
                            break 'b_1294b8;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = sx(at(c, SP, 0x20));
                            c.g[S0][0] = c.g[S2][0];
                            'l_129498: loop {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw(r32(c, A0));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                c.g[A0][0] = sx(at(c, A0, 0x4));
                                c.g[V0][0] = sx(at(c, V1, TRAIL_FX));
                                if c.g[V1][0] == 0 {
                                    c.g[V0][0] = 0u64;
                                }
                                c.sw(r32(c, A1), c.g[V0][0]);
                                let t = c.g[S0][0] != 0u64;
                                c.g[A1][0] = sx(at(c, A1, 0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00129498);
                                    }
                                    continue 'l_129498;
                                } else {
                                    break 'b_1294b8;
                                }
                            }
                        }
                    }
                }
            }
        }
        'b_1294f0: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, C_BOARD_TRAIL_FX_RENDER_BATCH, 0x001294c4)?;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1294f0;
            } else {
                c.cycles += 3;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1294d8: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, PULSE_FX));
                    call_ptr(c, C_PLAYER_PULSE_FX_RENDER, 0x001294e8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001294d8);
                        }
                        continue 'l_1294d8;
                    } else {
                        break 'b_1294f0;
                    }
                }
            }
        }
        'b_129520: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129520;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129508: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, CHEAT_CODE));
                    call_ptr(c, RIDER_BASE_RENDER_CHEAT_CODE, 0x00129518)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129508);
                        }
                        continue 'l_129508;
                    } else {
                        break 'b_129520;
                    }
                }
            }
        }
        'b_129550: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129550;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129538: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, WAKE_FX));
                    call_ptr(c, C_BOARD_WAKE_FX_RENDER, 0x00129548)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129538);
                        }
                        continue 'l_129538;
                    } else {
                        break 'b_129550;
                    }
                }
            }
        }
        'b_129580: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129580;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129568: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, AIR_STREAMER_FX));
                    call_ptr(c, C_AIR_STREAMER_FX_RENDER, 0x00129578)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129568);
                        }
                        continue 'l_129568;
                    } else {
                        break 'b_129580;
                    }
                }
            }
        }
        'b_1295b0: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1295b0;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129598: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, SPEED_STREAMER_FX));
                    call_ptr(c, C_SPEED_BOOST_STREAMER_FX_RENDER, 0x001295a8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129598);
                        }
                        continue 'l_129598;
                    } else {
                        break 'b_1295b0;
                    }
                }
            }
        }
        'b_1295e0: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1295e0;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1295c8: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, POWER_UP_TRAIL));
                    call_ptr(c, RIDER_BASE_RENDER_POWER_UP_TRAIL, 0x001295d8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001295c8);
                        }
                        continue 'l_1295c8;
                    } else {
                        break 'b_1295e0;
                    }
                }
            }
        }
        'b_129614: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129614;
            } else {
                c.cycles += 2;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                'l_1295f0: loop {
                    'b_12960c: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_12960c;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, SPARK_FX));
                            call_ptr(c, C_BOARD_SPARK_FX_RENDER, 0x00129608)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_12960c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001295f0);
                        }
                        continue 'l_1295f0;
                    } else {
                        break 'b_129614;
                    }
                }
            }
        }
        'b_12964c: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_12964c;
            } else {
                c.cycles += 3;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                'l_129628: loop {
                    'b_129644: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129644;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, ANGER_FX));
                            call_ptr(c, C_ANGER_FX_RENDER, 0x00129640)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129644;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129628);
                        }
                        continue 'l_129628;
                    } else {
                        break 'b_12964c;
                    }
                }
            }
        }
        'b_12967c: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_12967c;
            } else {
                c.cycles += 1;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                'l_129658: loop {
                    'b_129674: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129674;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, SPRAY_FX));
                            call_ptr(c, C_DYNAMIC_SPRAY_FX_RENDER, 0x00129670)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129674;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129658);
                        }
                        continue 'l_129658;
                    } else {
                        break 'b_12967c;
                    }
                }
            }
        }
        'b_1296ac: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_1296ac;
            } else {
                c.cycles += 1;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                'l_129688: loop {
                    'b_1296a4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_1296a4;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, ATTACK_FX));
                            call_ptr(c, RIDER_BASE_RENDER_ATTACK_FX, 0x001296a0)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_1296a4;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129688);
                        }
                        continue 'l_129688;
                    } else {
                        break 'b_1296ac;
                    }
                }
            }
        }
        'b_1296d4: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_1296d4;
            } else {
                c.cycles += 3;
                c.g[S1][0] = c.g[SP][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1296c0: loop {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    RIDER_RENDER_ATTACHED_FX.call(c, 0x001296cc)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001296c0);
                        }
                        continue 'l_1296c0;
                    } else {
                        break 'b_1296d4;
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- 0x001352a8 RiderSpinSub_UpdateTrick ---------------------------------

    mod update_trick_fixture {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x5_0000;
        const RIDER: u32 = DATA + 0x5_1000;
        const CTRL: u32 = DATA + 0x5_3000;
        const CTRL_VT: u32 = DATA + 0x5_3100;
        const SEQ_LISTS: u32 = DATA + 0x5_3200;
        const SEQ1: u32 = DATA + 0x5_3300;
        const SEQ2: u32 = DATA + 0x5_3400;
        const METRIX: u32 = DATA + 0x5_5000;
        const BE_LIBRARY: u32 = DATA + 0x5_7000;
        const BE_OBJ: u32 = DATA + 0x5_7100;
        const BE_VT: u32 = DATA + 0x5_7200;
        const AUDIO: u32 = DATA + 0x5_9000;
        /// The static animation table (28-byte entries; `+0` the kind
        /// `AnimController_GetLayerAnimEntry` returns).
        const ANIM_TABLE: u32 = 0x0044_6990;
        /// The BAGT trick table (0x14-byte entries of halfword fields).
        const TRICK_TABLE: u32 = 0x0045_aeb8;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// Fake methods: the behaviour interface getter, and the controller's
        /// "current animation kind" (answers 0x207: `PlayAnim` stops there).
        const FAKE_BE: u32 = FAKE;
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;
        /// Animation ids of layer 2 by kind: a wind-up (0x12), hold loop
        /// (0x13), chain point (0x14), an ended one (2), anything else (0).
        const KINDS: [u32; 5] = [0x12, 0x13, 0x14, 2, 0];

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        /// The fake hook: the animation-kind method answers 0x207.
        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_52a8, 8);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The behaviour library: the singleton exists, interface 0 is an
            // object whose getter is a fake; the character tables give a
            // divisor (character 0 of player 0).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE_BE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            // The audio system (the grab sound returns at once: the rider
            // has no sound slot).
            w(c, GAME_GP + 0x410, AUDIO);

            // The rider: `Rider_AddBoost` returns at once (state 3).
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x874, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x2f4, r.next_u32() % 10); // uber level
            w(
                c,
                RIDER + 0x2f0,
                if r.next_u32() % 2 == 0 { 0 } else { f(1.0) },
            );
            w(c, RIDER + 0xb2c, r.next_u32() % 4); // uber flags
            w(c, RIDER + 0x784, CTRL);
            w(c, RIDER + 0x790, METRIX);
            w(c, RIDER + 0x86c, 0);
            w(c, METRIX + 0x1ac, RIDER);

            // The animation controller: layer 1 and 2 sequences with random
            // event flags; layer 2 plays an animation of a random kind (or
            // none, 0x1b6).
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            w(c, SEQ_LISTS + 0xc, SEQ1);
            w(c, SEQ_LISTS + 0x14, SEQ2);
            c.bus.write64(SEQ1 + 0xb0, (r.next_u32() & 0xf) as u64);
            let flags = match seed % 5 {
                0 => 0,
                1 => 0xf,
                _ => r.next_u32() & 0xf,
            };
            c.bus.write64(SEQ2 + 0xb0, flags as u64);
            w(c, CTRL + 4, 0x10d + r.next_u32() % 0x1c);
            let id = if r.next_u32() % 8 == 0 {
                0x1b6
            } else {
                0x10d + r.next_u32() % 0x1c
            };
            w(c, CTRL + 8, id);
            if id != 0x1b6 {
                let kind = KINDS[(r.next_u32() % 5) as usize];
                w(c, ANIM_TABLE + id * 0x1c, kind);
            }

            // The trick table: tricks 0..7, each field an animation id or
            // "none" (0x1b6).
            for t in 0..8 {
                for k in 0..6 {
                    let v = if r.next_u32() % 3 == 0 {
                        0x1b6
                    } else {
                        0x10d + r.next_u32() % 0x1c
                    };
                    c.bus.write16(TRICK_TABLE + t * 0x14 + k * 2, v as u16);
                }
            }

            // The spin sub-state: every trick state (6: none of them), the
            // current trick (or -1), the request the same, another or none,
            // held or not.
            let current = if r.next_u32() % 4 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 8
            };
            let trick = match (seed / 5) % 4 {
                0 => u32::MAX,
                1 | 2 => current,
                _ => r.next_u32() % 8,
            };
            let held = if (seed / 3) % 2 == 0 {
                0
            } else {
                1 + r.next_u32() as u64
            };
            w(c, THIS + 0x4, (seed / 7) % 7);
            w(c, THIS + 0x8, current);
            w(c, THIS + 0x58, RIDER);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(trick);
            c.g[A2][0] = held;
            // The generated body (no override slot yet) dispatches on `pc`:
            // enter at the start.
            c.pc = 0x0013_52a8;
        }

        /// `RiderSpinSub_UpdateTrick`: every trick state, the trick
        /// requested, changed or dropped, held or released, the layer's
        /// event flags and animation kinds, trick-table entries with and
        /// without animations, and fake callees that unwind.
        #[test]
        fn rider_spin_sub_update_trick_matches() {
            check(
                ssx3_ee::s01::f_001352a8__generated,
                rider_spin_sub_update_trick,
                300,
                &setup,
            );
        }
    }

    // -- 0x0013c948 RiderGroundState_ComputeBoostAccel -----------------------

    mod boost_accel_fixture {
        use super::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0x6_0000;
        const RIDER: u32 = DATA + 0x6_1000;
        const PARAMS: u32 = DATA + 0x6_2000;
        const CTRL: u32 = DATA + 0x6_3000;
        const BE_LIBRARY: u32 = DATA + 0x6_7000;
        const BE_OBJ: u32 = DATA + 0x6_7100;
        const BE_VT: u32 = DATA + 0x6_7200;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        /// A float that is 0 a third of the time, else in [-range, range).
        fn maybe0(r: &mut Rng, range: f32) -> u32 {
            if r.next_u32() % 3 == 0 {
                0
            } else {
                r.f32(range)
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_c948, 2);
            let r = &mut r;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The behaviour library (interface getter: a fake).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);

            w(c, THIS + 0x18, RIDER);
            // The rider: on the ground (`+0x214` zero) and racing most of
            // the time; materials below 11, 11..13, 14..16 and >= 17.
            w(c, RIDER + 0x434, r.next_u32() % 20);
            w(c, RIDER + 0x214, if seed % 8 == 0 { r.f32(2.0) } else { 0 });
            w(
                c,
                RIDER + 0x470,
                if seed % 7 == 0 { f(1.0) } else { f(-1.0) },
            );
            c.bus.write16(RIDER + 0x2d4, (r.next_u32() & 0x30) as u16);
            // The heading: either component 0 at times, any quadrant.
            put_vec(
                c,
                RIDER + 0x1b0,
                [maybe0(r, 2.0), maybe0(r, 2.0), r.f32(2.0), r.f32(2.0)],
            );
            w(c, RIDER + 0x4cc, r.f32(4.0));
            w(c, RIDER + 0x2e8, maybe0(r, 2.0));
            w(c, RIDER + 0x2fc, r.f32(2.0));
            w(c, RIDER + 0x1f0, r.f32(1.5));
            w(c, RIDER + 0x3a8, r.f32(3.0));
            w(c, RIDER + 0x220, r.f32(2.0));
            w(c, RIDER + 0x378, r.f32(2.0));
            let speed = if seed % 2 == 0 { 2.0 } else { 60.0 };
            put_vec(
                c,
                RIDER + 0x1e0,
                [r.f32(speed), r.f32(speed), r.f32(speed), r.f32(1.0)],
            );
            w(c, RIDER + 0x86c, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x784, CTRL);
            w(c, CTRL + 8, if r.next_u32() % 2 == 0 { 0x16 } else { 0x20 });
            w(c, PARAMS + 0x20, r.f32(3.0));
            w(c, PARAMS + 0x24, r.f32(3.0));

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PARAMS);
            c.pc = 0x0013_c948;
        }

        /// `RiderGroundState_ComputeBoostAccel`: off the ground, finished,
        /// flagged, every heading quadrant and axis case, every material
        /// band, under and over the speed cap, the boost pose.
        #[test]
        fn rider_ground_state_compute_boost_accel_matches() {
            check(
                ssx3_ee::s01::f_0013c948__generated,
                rider_ground_state_compute_boost_accel,
                300,
                &setup,
            );
        }
    }

    // -- 0x002bd378 tWPIGD_Sun_Blend -----------------------------------------

    /// `tWPIGD_Sun_Blend`: random records and weights (edge floats too),
    /// a fake notify method that unwinds at either call.
    #[test]
    fn twpigd_sun_blend_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{fill, DATA};
        const THIS: u32 = DATA + 0x30_0000;
        const SRC: u32 = DATA + 0x30_1000;
        covered(
            ssx3_ee::s08::f_002bd378__generated,
            twpigd_sun_blend,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_d378, 2);
                let vt = vtable(c, &mut r, &[0x218]);
                fill(c, &mut r, THIS, 0x14);
                fill(c, &mut r, SRC, 0x0a);
                for k in 0..6 {
                    c.bus.write32(THIS + 8 + 8 * k, r.f32(100.0));
                    c.bus.write32(SRC + 4 + 4 * k, r.f32(100.0));
                }
                c.bus.write32(THIS + 0x38, r.next_u32() % 2000);
                c.bus
                    .write32(SRC + 0x1c, (r.next_u32() % 4000).wrapping_sub(2000));
                for k in 0..2 {
                    c.bus.write32(THIS + 0x40 + 8 * k, r.f32(100.0));
                    c.bus.write32(SRC + 0x20 + 4 * k, r.edge_f32());
                }
                c.bus.write32(THIS + 4, vt);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(SRC);
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                c.pc = 0x002b_d378;
            },
        );
    }

    // -- 0x003ba888 SND_CentsToPitch -----------------------------------------

    /// `SND_CentsToPitch`: octaves up and down (several), positive and
    /// negative steps, the -255 clamp, a full 64-bit register on entry.
    #[test]
    fn snd_cents_to_pitch_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng};
        check_lohi(
            ssx3_ee::s13::f_003ba888__generated,
            snd_cents_to_pitch,
            300,
            &|c, seed| {
                let mut r = Rng(0x003b_a888 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                fill(c, &mut r, CENTS_PITCH_TABLE, 0x40);
                let cents: i32 = match seed % 6 {
                    0 => (r.next_u32() % 1200) as i32 - 600,
                    1 => 1200 * ((r.next_u32() % 6) as i32 - 3),
                    2 => -1199 - (r.next_u32() % 3) as i32,
                    _ => (r.next_u32() % 20000) as i32 - 10000,
                };
                c.g[A0][0] = if seed % 10 == 9 {
                    (r.next_u32() as u64) << 32 | (cents as u32) as u64
                } else {
                    sx(cents as u32)
                };
                c.pc = 0x003b_a888;
            },
        );
    }

    // -- 0x003c6380 SND_UMul32x32 --------------------------------------------

    /// `SND_UMul32x32`: random and edge operands (zero, all ones, carries
    /// out of the middle bits), junk in the upper register halves.
    #[test]
    fn snd_umul32x32_matches() {
        use crate::difftest::{check_lohi, scramble, Rng};
        check_lohi(ssx3_ee::s13::f_003c6380__generated, snd_umul32x32, 300, &|c, seed| {
            let mut r = Rng(0x003c_6380 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let pick = |r: &mut Rng| match r.next_u32() % 5 {
                0 => 0,
                1 => u32::MAX,
                2 => 0xffff,
                3 => 0xffff_0000 | r.next_u32() & 0xffff,
                _ => r.next_u32(),
            };
            let (a, b) = (pick(&mut r), pick(&mut r));
            c.g[A0][0] = (r.next_u32() as u64) << 32 | a as u64;
            c.g[A1][0] = if seed % 2 == 0 { sx(b) } else { b as u64 };
            c.pc = 0x003c_6380;
        });
    }

    // -- 0x00120f20 Rider_BeginFrame -----------------------------------------

    mod begin_frame_fixture {
        use super::*;
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::{Rng, DATA, GAME_GP};
        use crate::game::ai::ridermetrix::tests::target_speed_fixture;

        /// The rider of `Rider_UpdateTargetSpeed`'s fixture (random fields,
        /// a course, the back end), plus the objects below (zeroed RAM
        /// except the fields written).
        const RIDER: u32 = target_speed_fixture::RIDER;
        const FRAMES: u32 = DATA + 0x24_0000;
        const TREE2: u32 = DATA + 0x24_0100;
        const CONTROL: u32 = DATA + 0x24_0200;
        const LIGHTS: u32 = DATA + 0x24_0300;
        const MODEL: u32 = DATA + 0x24_1000;
        const BONES: u32 = DATA + 0x24_2000;
        const LIST: u32 = DATA + 0x24_4000;
        const TREE: u32 = DATA + 0x24_5000;
        const PROBE_VT: u32 = DATA + 0x24_6000;

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = target_speed_fixture::setup(c, seed, 3);
            let r = &mut r;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // The game module the fixture's world points at: the frame
            // counter, the spatial tree's stamp, the camera control, the
            // light manager.
            let app = c.bus.read32(GAME_GP - 0x848);
            let module = c.bus.read32(app + 0x84);
            let ai = c.bus.read32(RIDER + 0x86c);
            // The rider's turn on every third seed.
            let frame = ai + 3 * (r.next_u32() % 100) + [0, 1, 2][(seed % 3) as usize];
            w(c, module + 0xc, FRAMES);
            w(c, FRAMES + 8, frame);
            w(c, module + 0x20, TREE2);
            let stamp = r.next_u32();
            w(c, TREE2 + 0xa0, stamp);
            w(c, module + 0x34, CONTROL);
            w(c, CONTROL + 0x70, (seed / 3) % 4);
            w(c, module + 0x90, LIGHTS);
            // The overlap list (empty tree), stamped as the world's or not.
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, TREE);
            w(
                c,
                LIST + 4,
                if (seed / 12) % 2 == 0 {
                    stamp
                } else {
                    stamp ^ 1
                },
            );
            w(c, LIST + 8, 0);
            // The model and its bones (hips and board root bone 0..3).
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x89c, r.next_u32() % 4);
            w(c, RIDER + 0x8a0, r.next_u32() % 4);
            // A scripted scene on every other seed (probe at the bone or
            // the position); the probe's shape method is a fake.
            w(c, RIDER + 0xac4, (seed / 24) % 2);
            w(c, RIDER + 0xafc, r.next_u32() % 2);
            w(c, RIDER + 0xb00, r.next_u32() % 2);
            w(c, RIDER + 0x6c0, PROBE_VT);
            w(c, PROBE_VT + 0x10, 0);
            w(c, PROBE_VT + 0x14, FAKE);
            // Racing or finished; the finish clock's step.
            w(
                c,
                RIDER + 0x470,
                if seed % 5 == 0 {
                    r.f32(100.0) & 0x7fff_ffff
                } else {
                    (-1.0f32).to_bits()
                },
            );
            w(c, GAME_GP - 0x7920, (1.0f32 / 60.0).to_bits());
            c.g[A0][0] = sx(RIDER);
            c.pc = 0x0012_0f20;
        }

        /// `Rider_BeginFrame`: the rider's turn or not, the list stamped as
        /// the world's or not, every camera control mode, in a scripted
        /// scene (probe at the bone or the position) or not, racing or
        /// finished; real callees (the target speed's fakes may unwind).
        #[test]
        fn rider_begin_frame_matches() {
            check_lohi(ssx3_ee::s00::f_00120f20__generated, rider_begin_frame, 300, &setup);
        }
    }

    // -- 0x0012e9b8 RiderPrewindSub_HandleInput ------------------------------

    mod prewind_input_fixture {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x7_0000;
        const PAD: u32 = DATA + 0x7_0100;
        const RIDER: u32 = DATA + 0x7_1000;
        const OWNER: u32 = DATA + 0x7_2000;
        const CTRL: u32 = DATA + 0x7_3000;
        const CTRL_VT: u32 = DATA + 0x7_3100;
        const SEQ_LISTS: u32 = DATA + 0x7_3200;
        const SEQ: u32 = DATA + 0x7_3300;
        const MODEL: u32 = DATA + 0x7_4000;
        const BONES: u32 = DATA + 0x7_4100;
        const LIST: u32 = DATA + 0x7_5000;
        const LIST_TREE: u32 = DATA + 0x7_5400;
        const METRIX: u32 = DATA + 0x7_6000;
        const AUDIO: u32 = DATA + 0x7_7000;
        const BE_LIBRARY: u32 = DATA + 0x7_8000;
        const BE_OBJ: u32 = DATA + 0x7_8100;
        const BE_VT: u32 = DATA + 0x7_8200;
        /// The static animation table (28-byte entries; `+0` the kind).
        const ANIM_TABLE: u32 = 0x0044_6990;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// The controller's "current animation kind" method: answers 0x207
        /// (`PlayAnim` stops there).
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;
        /// Layer-2 animation ids: a kind-10 one, id 0x15, an ordinary one.
        const ID_KIND10: u32 = 0x30;
        const ID_PLAIN: u32 = 0x31;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed axis: centred, inside or outside the dead zone.
        fn axis(r: &mut Rng) -> u32 {
            match r.next_u32() % 4 {
                0 => 0,
                1 => 3,
                2 => 0x3d,
                _ => r.next_u32() & 0x3f,
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_e9b8, 8);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);

            // The back end: the library exists, interface getters are fakes.
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            w(c, GAME_GP + 0x410, AUDIO);

            // The pad: never the reset bit; held (bit 13) on two of three
            // seeds; the axes centred, small or large.
            let held = seed % 3 != 0;
            let mut buttons = r.next_u32() & !0x0000_3000;
            buttons &= !(0x3f << 15 | 0x3f << 21);
            buttons |= axis(r) << 15 | axis(r) << 21;
            if held {
                buttons |= 0x2000;
            }
            w(c, PAD, buttons);
            w(c, PAD + 4, (r.next_u32() & !0xfff) | axis(r) | axis(r) << 6);

            w(c, THIS, RIDER);
            // The rider: racing (finished on every 16th seed), main state 2
            // (no rail search), 1 (airborne: no launch) or 0.
            w(
                c,
                RIDER + 0x470,
                if seed % 16 == 15 { f(1.0) } else { f(-1.0) },
            );
            w(c, RIDER + 0x77c, OWNER);
            w(c, OWNER + 0xde0, [2, 2, 1, 0][((seed / 3) % 4) as usize]);
            w(c, RIDER + 0x328, (seed / 12) % 5);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, LIST_TREE);
            w(c, RIDER + 0x790, METRIX);
            w(c, METRIX + 0x1ac, RIDER);
            for off in [0x220, 0x2a4, 0x2b0] {
                w(c, RIDER + off, r.f32(2.0));
            }
            w(
                c,
                RIDER + 0x2dc,
                if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) },
            );
            put_vec(c, RIDER + 0x1e0, [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
            for off in [0x100, 0x110, 0x120, 0x130] {
                put_vec(c, RIDER + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), f(1.0)]);
            }

            // The animation controller: layer 2 plays a kind-10 animation,
            // id 0x15 or an ordinary one.
            w(c, RIDER + 0x784, CTRL);
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            for k in 0..4 {
                w(c, SEQ_LISTS + 8 * k + 4, SEQ);
            }
            w(c, ANIM_TABLE + ID_KIND10 * 0x1c, 10);
            w(c, ANIM_TABLE + ID_PLAIN * 0x1c, 0);
            w(c, ANIM_TABLE + 0x15 * 0x1c, 0);
            let id = [ID_KIND10, 0x15, ID_PLAIN, ID_PLAIN][(seed % 4) as usize];
            w(c, CTRL + 8, id);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            c.pc = 0x0012_e9b8;
        }

        /// `RiderPrewindSub_HandleInput`: finished or not, released (with
        /// and without the launch, every prewind direction) or held (kind
        /// 10, anim 0x15, turn-around, the trick stick's dead zone), real
        /// callees with fakes that unwind.
        #[test]
        fn rider_prewind_sub_handle_input_matches() {
            check(
                ssx3_ee::s00::f_0012e9b8__generated,
                rider_prewind_sub_handle_input,
                300,
                &setup,
            );
        }
    }

    // -- 0x0010def0 ComputerActorRider_ComputeRubberBand ---------------------

    /// `ComputerActorRider_ComputeRubberBand`: no human (no slot, or an
    /// empty one), every mode, game mode 2, near the finish, catching up,
    /// holding back and in between; the back end's interface getter is a
    /// fake that may unwind.
    #[test]
    fn computer_actor_rider_compute_rubber_band_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, put_words, DATA, GAME_GP};
        const THIS: u32 = DATA + 0x8_0000;
        const PARAMS: u32 = DATA + 0x8_1000;
        const APP: u32 = DATA + 0x8_2000;
        const MODULE: u32 = DATA + 0x8_2100;
        const PLAYERS: u32 = DATA + 0x8_2200;
        const HUMAN: u32 = DATA + 0x8_3000;
        const BE_LIBRARY: u32 = DATA + 0x8_4000;
        const BE_OBJ: u32 = DATA + 0x8_4100;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        let f = |x: f32| x.to_bits();
        check(
            ssx3_ee::s00::f_0010def0__generated,
            computer_actor_rider_compute_rubber_band,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_def0, 2);
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The back end: the library exists, interface 0's getter is
                // a fake.
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, BE_INTERFACES, BE_OBJ);
                let vt = vtable(c, &mut r, &[8]);
                w(c, BE_OBJ + 4, vt);
                put_words(c, GAME_GP - 0x7cc4, &[f(50000.0), f(1.1), f(1.0 / 1.1)]);

                // The player slots: the human in slot 0..5 (or none, 6).
                let human = seed % 7;
                for i in 0..6 {
                    let (a, b) = if i == human {
                        (1 + r.next_u32() % 9, 1 + r.next_u32() % 9)
                    } else if r.next_u32() % 2 == 0 {
                        (0, r.next_u32() % 3)
                    } else {
                        (r.next_u32() % 3, 0)
                    };
                    w(c, PARAMS + 0x24 * i, a);
                    w(c, PARAMS + 0x24 * i + 4, b);
                }
                w(c, GAME_GP - 0x848, APP);
                w(c, APP + 0x84, MODULE);
                w(c, MODULE + 0xc, PLAYERS);
                if human < 6 {
                    let p = if seed % 11 == 0 { 0 } else { HUMAN };
                    w(c, PLAYERS + 4 * human + 0x28, p);
                }

                // Mode, game mode, distances.
                w(c, THIS + 0x18, PARAMS);
                w(c, PARAMS + 0xe4, (seed / 7) % 4);
                c.bus
                    .write8(0x0053_5c11, if seed % 13 == 0 { 2 } else { 1 });
                let own = if seed % 9 == 0 {
                    (r.next_u32() % 50000) as f32
                } else {
                    50000.0 + (r.next_u32() % 100000) as f32
                };
                w(c, PARAMS + 0x4d0, f(own));
                let k = 100.0 + (r.next_u32() % 5000) as f32;
                let k2 = 100.0 + (r.next_u32() % 5000) as f32;
                w(c, PARAMS + 0xdc, f(k));
                w(c, PARAMS + 0xe0, f(k2));
                let delta = (r.next_u32() % 20000) as f32 - 10000.0;
                w(c, HUMAN + 0x4d0, f(own + delta));

                c.g[A0][0] = sx(THIS);
                c.pc = 0x0010_def0;
            },
        );
    }

    // -- 0x002ac7f0 BankMonitorInstance_UpdateRange --------------------------

    /// `BankMonitorInstance_UpdateRange`: no range, playing inside and
    /// beyond, stopped inside and beyond (the fade-out and play callees on
    /// their short paths: already fading, or a state without a voice).
    #[test]
    fn bank_monitor_instance_update_range_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const THIS: u32 = DATA + 0x9_0000;
        check(
            ssx3_ee::s08::f_002ac7f0__generated,
            bank_monitor_instance_update_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_c7f0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                let range = if seed % 5 == 0 {
                    0
                } else {
                    (10.0 + (r.next_u32() % 100) as f32).to_bits()
                };
                c.bus.write32(THIS + 0x8c, range);
                c.bus
                    .write32(THIS + 0x8, (seed / 5) % 2 * (1 + r.next_u32() % 4));
                c.bus.write32(THIS, [0, 5][(r.next_u32() % 2) as usize]);
                c.bus.write32(THIS + 0x54, r.next_u32() % 2);
                c.g[A0][0] = sx(THIS);
                c.f[12] = match r.next_u32() % 4 {
                    0 => r.edge_f32(),
                    1 => range,
                    _ => ((r.next_u32() % 200) as f32).to_bits(),
                };
                c.pc = 0x002a_c7f0;
            },
        );
    }

    // -- 0x001291e0 AI_Render -------------------------------------------------

    mod ai_render_fixture {
        use super::*;
        use crate::difftest::fakes::{machine, vtable, FAKE};
        use crate::difftest::{check_lohi, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0xa_0000;
        const ACTORS: u32 = DATA + 0xa_1000;
        const RIDERS: u32 = DATA + 0xa_4000;
        const OWNERS: u32 = DATA + 0xb_0000;
        const GFX: u32 = DATA + 0xc_0000;
        const GFX_VT: u32 = DATA + 0xc_2000;
        const PACKET: u32 = DATA + 0xc_3000;
        const APP: u32 = DATA + 0xc_4000;
        const MODULE: u32 = DATA + 0xc_4100;
        const FRAMES: u32 = DATA + 0xc_4200;
        const CONTROL: u32 = DATA + 0xc_4300;
        /// Rider `i` (0x1000 bytes each) and its owner (0x1000 bytes each,
        /// zeroed: every effect is idle).
        fn rider(i: u32) -> u32 {
            RIDERS + 0x1000 * i
        }
        fn actor(i: u32) -> u32 {
            ACTORS + 0x40 * i
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_91e0, 4);
            let r = &mut r;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // The graphics manager: a packet buffer, the light-slot
            // allocator and the trail batch's draw method are fakes.
            w(c, GAME_GP - 0x854, GFX);
            w(c, GFX + 0xe84, PACKET);
            w(c, GFX + 0x10d8, GFX_VT);
            for (slot, k) in [(0x390, 0), (0x2c8, 1)] {
                c.bus.write16(GFX_VT + slot, 0);
                w(c, GFX_VT + slot + 4, FAKE + 0x10 * k);
            }
            // The world: the frame counter object (no pulse), the camera
            // control (mode 0 or not, or none).
            w(c, GAME_GP - 0x848, APP);
            w(c, APP + 0x84, MODULE);
            w(c, MODULE + 0xc, FRAMES);
            let control = match r.next_u32() % 3 {
                0 => 0,
                _ => CONTROL,
            };
            w(c, MODULE + 0x34, control);
            w(c, CONTROL + 0x70, r.next_u32() % 2);

            // Eight riders, none visible (`Rider_UpdateVisibility` gives
            // 0): an owner or none, a light slot or not yet (-1).
            for i in 0..8 {
                let p = rider(i);
                let owner = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    OWNERS + 0x1000 * i
                };
                w(c, p + 0x77c, owner);
                w(
                    c,
                    p + 0x888,
                    if r.next_u32() % 2 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    },
                );
                w(c, p + 0x880, r.next_u32() % 8);
                // Every eighth rider has the visible flag, but not state 7.
                w(c, p + 0x884, (r.next_u32() % 8 == 0) as u32);
                if c.bus.read32(p + 0x880) == 7 {
                    w(c, p + 0x880, 6);
                }
            }
            // Actors (some null) pointing at riders.
            let mut next = 0;
            let mut actor_ptr = |c: &mut Cpu, r: &mut Rng| {
                let i = next;
                next += 1;
                if r.next_u32() % 5 == 0 {
                    0
                } else {
                    w(c, actor(i) + 0x18, rider(i % 8));
                    actor(i)
                }
            };

            // The manager: the light-slot list, the human / computer /
            // other actors and their counts (at most 8 riders in all).
            let listed = seed % 5;
            w(c, THIS + 0x78, listed);
            for i in 0..listed {
                w(c, THIS + 0x28 + 4 * i, rider(i));
            }
            let humans = (seed / 5) % 3;
            w(c, THIS + 0x7c, humans);
            for i in 0..2 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x40 + 4 * i, a);
            }
            let viewed = actor_ptr(c, r);
            w(c, THIS + 0x70, viewed);
            w(c, THIS + 0x88, (seed / 15) % 2);
            w(c, THIS + 0x14, r.next_u32() % 2);
            w(c, THIS + 0x80, r.next_u32() % 4);
            for i in 0..3 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x48 + 4 * i, a);
            }
            w(c, THIS + 0x8c, r.next_u32() % 2);
            w(c, THIS + 0x84, r.next_u32() % 4);
            for i in 0..3 {
                let a = actor_ptr(c, r);
                w(c, THIS + 0x5c + 4 * i, a);
            }

            c.g[A0][0] = sx(THIS);
            c.pc = 0x0012_91e0;
        }

        /// `AI_Render` with no rider visible: light slots handed out, every
        /// way the drawn riders are chosen (single view by the camera mode,
        /// humans, computers, others, null actors), the per-rider passes on
        /// idle effects, the trail batch; fakes that unwind.
        #[test]
        fn ai_render_matches() {
            check_lohi(ssx3_ee::s00::f_001291e0__generated, ai_render, 300, &setup);
        }
    }
}
