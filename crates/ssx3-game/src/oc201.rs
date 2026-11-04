//! Staging wave 201 (oc201): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

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
        0x0013_3308,
        rider_spin_sub_handle_input,
        "RiderSpinSub_HandleInput",
    ),
    (0x002e_4f50, snow_box_update, "SnowBox_Update"),
];

// ---------------------------------------------------------------------------
// 0x00133308 RiderSpinSub_HandleInput

/// Callees of [`rider_spin_sub_handle_input`] (guest addresses; the
/// `Callee`s go through their generated functions' override slots).
mod spin_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// `Rider_CheckResetPath(rider, force, reason)`.
    pub const RIDER_CHECK_RESET_PATH: u32 = 0x0011_6120;
    /// `Rider_TryStartHandplant(rider, bool dir)`.
    pub const RIDER_TRY_START_HANDPLANT: u32 = 0x0010_7578;
    /// `Rider_TryStartRailslide(rider)`: jumps onto a rail found near the
    /// rider (`Rider_FindRail`); 1 when it did.
    pub const RIDER_TRY_START_RAILSLIDE: u32 = 0x0010_6848;
    /// `Rider_HandleBoostInput(rider, 0, 0)`.
    pub const RIDER_HANDLE_BOOST_INPUT: u32 = 0x0011_4130;
    /// `AnimController_GetLayerAnimId(ctrl, layer)`.
    pub const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    /// `Input_QuantizeStickDirection(f12, f13, float* x, float* y)`.
    pub const INPUT_QUANTIZE_STICK_DIRECTION: u32 = 0x0011_58b8;
    /// `RiderMetrix_StartSpinTrick(metrix, bool switched)`: returns boost.
    pub const METRIX_START_SPIN_TRICK: u32 = 0x0011_9c98;
    /// `Rider_AddBoost(rider, f12 amount, 1)`.
    pub const RIDER_ADD_BOOST: u32 = 0x0010_e098;
    /// `Math_Atan(f12)`.
    pub const MATH_ATAN: u32 = 0x0031_c228;
    /// `RiderMetrix_SetSpinAmount(metrix, f12)`: returns boost.
    pub const METRIX_SET_SPIN_AMOUNT: u32 = 0x0011_9898;
    /// `RiderMetrix_SetFlipAmount(metrix, f12)`: returns boost.
    pub const METRIX_SET_FLIP_AMOUNT: u32 = 0x0011_98d8;
    /// `AnimController_GetLayerAnimEntry(ctrl, layer)`.
    pub const ANIM_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
    /// `AnimController_IsLayerAnimFinished(ctrl, layer)`.
    pub const ANIM_IS_LAYER_ANIM_FINISHED: u32 = 0x0031_2ae8;
    /// `AnimController_GetCurrentAnimLength(ctrl, id)`.
    pub const ANIM_GET_CURRENT_ANIM_LENGTH: u32 = 0x0031_2790;
    /// `AnimController_PlayAnim(ctrl, id, f12 blend, 0)`.
    pub const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const UPDATE_TRICK: Callee = Callee {
        addr: 0x0013_52a8,
        f: ee::s01::RiderSpinSub_UpdateTrick,
    };
    pub const TRY_REVERSE: Callee = Callee {
        addr: 0x0013_5be0,
        f: ee::s01::RiderSpinSub_TryReverse,
    };
    pub const SET_SPIN_DIRECTION: Callee = Callee {
        addr: 0x0013_5b30,
        f: ee::s01::RiderSpinSub_SetSpinDirection,
    };
    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const BE_CHAR_INT_GET_ATTRIB4_FACTOR: Callee = Callee {
        addr: 0x0014_95a8,
        f: ee::s01::BECharInt_GetAttrib4Factor,
    };
}

/// RiderSpinSub fields (`this`, kept in s0) and the rider fields it
/// drives. Floats unless noted; "spin" is the yaw axis, "flip" the pitch
/// axis.
mod spin_sub {
    /// Spin-hold mode (int): 0 none, 2 hold button with the stick out, 1
    /// hold released, 3 parked while a trick plays.
    pub const MODE: u32 = 0x0;
    /// Spin phase (int): 3 idle, 0 started, 1 axes moving, 2 settling.
    pub const PHASE: u32 = 0xc;
    pub const FLIP_TARGET: u32 = 0x10;
    pub const SPIN_TARGET: u32 = 0x14;
    pub const FLIP: u32 = 0x18;
    pub const SPIN: u32 = 0x1c;
    /// Accumulated rotation (for the metrix and the HUD).
    pub const SPIN_TOTAL: u32 = 0x20;
    pub const FLIP_TOTAL: u32 = 0x24;
    /// The smoothed body lean from the stick (rider `+0x2a0` / `+0x294`).
    pub const LEAN_X: u32 = 0x28;
    pub const LEAN_Y: u32 = 0x2c;
    pub const SPIN_AMOUNT: u32 = 0x30;
    pub const FLIP_AMOUNT: u32 = 0x34;
    /// Rotation rates (rider `+0x2dc` / `+0x2e0` follow them).
    pub const SPIN_RATE: u32 = 0x38;
    pub const FLIP_RATE: u32 = 0x3c;
    /// Set (int 1) once a further half turn was queued.
    pub const TURNED: u32 = 0x44;
    /// How long the stick has held each axis.
    pub const SPIN_HELD: u32 = 0x48;
    pub const FLIP_HELD: u32 = 0x4c;
    /// The stick angle (`atan`, or a fixed angle on an axis).
    pub const STICK_ANGLE: u32 = 0x50;
    /// Time since the stick was released.
    pub const RELEASED: u32 = 0x54;
    /// The owning rider.
    pub const RIDER: u32 = 0x58;

    /// Rider: the animation controller (layer 2 is the trick layer;
    /// `+0x1c` its playback speed).
    pub const R_ANIM: u32 = 0x784;
    /// Rider: the animation state (`+0xac` kind, `+0x98`/`+0xa0` times).
    pub const R_ANIM_STATE: u32 = 0x788;
    pub const R_METRIX: u32 = 0x790;
    pub const R_28C: u32 = 0x28c;
    pub const R_LEAN_Y_BLEND: u32 = 0x290;
    pub const R_LEAN_Y: u32 = 0x294;
    pub const R_298: u32 = 0x298;
    pub const R_LEAN_X_BLEND: u32 = 0x29c;
    pub const R_LEAN_X: u32 = 0x2a0;
    pub const R_SPIN_SPEED: u32 = 0x2dc;
    pub const R_FLIP_SPEED: u32 = 0x2e0;
    /// Rider: positive scales both rotation speeds.
    pub const R_2EC: u32 = 0x2ec;
    /// Rider: stance (int, non-zero mirrors the spin direction) and the
    /// stance at take-off.
    pub const R_STANCE: u32 = 0x320;
    pub const R_STANCE_TAKEOFF: u32 = 0x324;
    /// Rider: lean direction from the stick (int 0, 1, 2).
    pub const R_LEAN_DIR: u32 = 0x330;
    /// Rider: the behaviour-engine player and character ids.
    pub const R_PLAYER: u32 = 0x86c;
    pub const R_CHARACTER: u32 = 0xb34;
}

/// The pad state (`a1`, kept in s1 until the animation choice): button
/// bits (bit 12 reset, 13 handplant, 14 trick, 15 spin hold; the spin
/// stick axis in bits 24..29, the trick byte at +2) and the packed axes
/// at +4 (flip axis bits 0..5, lean axes 6..11 / 12..17, lean direction
/// bits 18..19).
mod spin_pad {
    pub const PAD_BUTTONS: u32 = 0x0;
    pub const PAD_STICKS: u32 = 0x4;
}

/// Stack scratch of [`rider_spin_sub_handle_input`]: the scaled stick
/// axes (`Input_QuantizeStickDirection` writes them back).
const STICK_FLIP: u32 = 0x0;
const STICK_SPIN: u32 = 0x4;

/// `reg + off` (a field of the object a register points at).
#[inline(always)]
fn spin_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off` (a `.lit4` constant or global).
#[inline(always)]
fn spin_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `sp + off` (the stack scratch).
#[inline(always)]
fn spin_stack(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `void RiderSpinSub_HandleInput(RiderSpinSub* this, Pad* pad)`
/// (0x133308): the rider's in-air spin/flip input, once per tick.
///
/// - Reset (`Rider_CheckResetPath`, pad bit 12), handplant (bit 13 while
///   idle) and a rail jump (`Rider_TryStartRailslide`) take over and end
///   the function when they start.
/// - The stick axes (6-bit signed fields scaled by a `.lit4` constant)
///   and the spin-hold bit update the hold mode; `RiderSpinSub_UpdateTrick`
///   runs (it returns non-zero while a trick plays); the lean direction
///   goes to the rider; `Rider_HandleBoostInput`.
/// - Phase 3 (idle): a stick push starts a spin (unless the layer plays
///   anim 0x120 or `RiderSpinSub_TryReverse` takes it): quantized stick
///   direction, `RiderSpinSub_SetSpinDirection`, start rates,
///   `RiderMetrix_StartSpinTrick` and its boost.
/// - Phases 0..2 move the targets (`+0x10`/`+0x14`) by half turns while the
///   stick is held, count the hold times, and end the spin (phase 3) once
///   the rotation reached its targets with the stick released.
/// - The rotation speeds (rider `+0x2dc`/`+0x2e0`) come from the rates,
///   the distance to the target and the character's attribute factor
///   (`BECharInt_GetAttrib4Factor`); the lean is smoothed; the rotation
///   steps toward the targets; the amounts go to the metrix (boost).
/// - Finally picks the trick-layer animation (spin poses 0x10d..0x114 by
///   stick angle, flip poses 0x125..0x128, tweaks 0x129..0x130, landing
///   blends 0x132..0x139 from the current pose (a jump table), 0x131 /
///   0x11f) and plays it with `AnimController_PlayAnim` when it changed.
///
/// The control flow follows the original's blocks (labels are their guest
/// addresses); cycles are charged per block as the original does, and the
/// scratch registers (v0/v1/a*/f*) are left as it leaves them.
pub fn rider_spin_sub_handle_input(c: &mut Cpu) {
    use spin_pad::*;
    use spin_sub::*;
    let at = spin_at;
    let gp = spin_gp;
    let stack = spin_stack;
    guest(c, |c| {
        // The frame (0xb0): s0..s4, ra, f20..f27; sp+0 / sp+4 hold the
        // stick axes.
        let frame = Frame::open(c, 0xb0);
        'b_134c74: {
            c.cycles += 25;
            c.g[A2][0] = 0u64;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x40);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            frame.save_fpr(c, 27, 0xa8);
            frame.save_fpr(c, 26, 0xa0);
            frame.save_fpr(c, 25, 0x98);
            frame.save_fpr(c, 24, 0x90);
            frame.save_fpr(c, 23, 0x88);
            frame.save_fpr(c, 22, 0x80);
            frame.save_fpr(c, 21, 0x78);
            frame.save_fpr(c, 20, 0x70);
            c.g[A1][0] = c.lw(at(c, S1, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S0, RIDER));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, spin_callee::RIDER_CHECK_RESET_PATH, 0x00133364)?;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x60);
                break 'b_134c74;
            } else {
                'b_1333d0: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S1, PAD_BUTTONS));
                    c.g[V0][0] = c.g[V0][0] & 0x2000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_1333d0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_1333d0;
                        } else {
                            'b_1333c0: {
                                c.cycles += 13;
                                c.g[V0][0] = c.lw(at(c, S1, PAD_STICKS));
                                c.lwc1(2, gp(c, -0x7528));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_lt_s(1, 0);
                                let t = c.fcc();
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_1333c0;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = 0u64;
                                    break 'b_1333c0;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            call_ptr(c, spin_callee::RIDER_TRY_START_HANDPLANT, 0x001333c8)?;
                            if c.g[V0][0] != 0u64 {
                                frame.restore(c, S0, 0x60);
                                break 'b_134c74;
                            } else {
                                break 'b_1333d0;
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw(at(c, S0, RIDER));
                call_ptr(c, spin_callee::RIDER_TRY_START_RAILSLIDE, 0x001333d8)?;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x60);
                    break 'b_134c74;
                } else {
                    'b_1334bc: {
                        'b_1334b8: {
                            'b_1334b4: {
                                'b_13348c: {
                                    c.cycles += 28;
                                    c.g[A0][0] = c.lw(at(c, S1, PAD_STICKS));
                                    c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 26);
                                    c.lwc1(0, gp(c, -0x7524));
                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                    c.mtc1(3, 1);
                                    c.cvt_s_w(1, 1);
                                    c.mtc1(2, 2);
                                    c.cvt_s_w(2, 2);
                                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 14);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 26) as u32);
                                    c.mul_s(4, 1, 0);
                                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 26) as u32);
                                    c.mul_s(3, 2, 0);
                                    c.g[V0][0] = c.lw(at(c, S0, MODE));
                                    c.mtc1(4, 2);
                                    c.cvt_s_w(2, 2);
                                    c.mtc1(5, 1);
                                    c.cvt_s_w(1, 1);
                                    c.g[A0][0] = c.lb(at(c, S1, PAD_BUTTONS + 2));
                                    c.swc1(4, stack(c, STICK_FLIP));
                                    c.swc1(3, stack(c, STICK_SPIN));
                                    c.mul_s(23, 2, 0);
                                    let t = c.g[V0][0] != 0u64;
                                    c.mul_s(24, 1, 0);
                                    if t {
                                        break 'b_13348c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A2][0] & 0x8000;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] = c.lw(at(c, S0, MODE));
                                            break 'b_1334b8;
                                        } else {
                                            'b_133484: {
                                                c.cycles += 5;
                                                c.mtc1(0, 0);
                                                c.c_eq_s(24, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_133484;
                                                } else {
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 0);
                                                    if c.fcc() {
                                                        c.g[V1][0] = c.lw(at(c, S0, MODE));
                                                        break 'b_1334bc;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x2;
                                                        break 'b_133484;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw(at(c, S0, MODE), c.g[V0][0]);
                                            break 'b_1334b4;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_eq_s(3, 0);
                                if !c.fcc() {
                                    c.sw(at(c, S0, MODE), 0u64);
                                    break 'b_1334b4;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(4, 0);
                                    if c.fcc() {
                                        c.g[V1][0] = c.lw(at(c, S0, MODE));
                                        break 'b_1334b8;
                                    } else {
                                        c.cycles += 1;
                                        c.sw(at(c, S0, MODE), 0u64);
                                        break 'b_1334b4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334b8;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x2;
                        break 'b_1334bc;
                    }
                    'b_1334e4: {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334e4;
                        } else {
                            'b_1334e0: {
                                'b_1334dc: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(at(c, S1, PAD_BUTTONS));
                                    c.g[V0][0] = c.g[V0][0] & 0x8000;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1334dc;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = 0xffffffffffffffff;
                                        break 'b_1334e0;
                                    }
                                }
                                c.cycles += 1;
                                c.sw(at(c, S0, MODE), c.g[V0][0]);
                                break 'b_1334e0;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334e4;
                        }
                    }
                    'b_133560: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                            break 'b_133560;
                        } else {
                            'b_133528: {
                                c.cycles += 4;
                                c.mtc1(0, 0);
                                c.c_eq_s(24, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133528;
                                } else {
                                    c.cycles += 3;
                                    c.c_eq_s(23, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133528;
                                    } else {
                                        c.cycles += 6;
                                        c.lwc1(1, at(c, S0, RELEASED));
                                        c.lwc1(0, gp(c, -0x7520));
                                        c.c_lt_s(0, 1);
                                        if c.fcc() {
                                            c.sw(at(c, S0, MODE), 0u64);
                                            break 'b_133528;
                                        } else {
                                            break 'b_133528;
                                        }
                                    }
                                }
                            }
                            'b_133540: {
                                c.cycles += 6;
                                c.mtc1(0, 1);
                                c.lwc1(0, stack(c, STICK_SPIN));
                                c.c_eq_s(0, 1);
                                if c.fcc() {
                                    c.swc1(24, stack(c, STICK_SPIN));
                                    break 'b_133540;
                                } else {
                                    break 'b_133540;
                                }
                            }
                            'b_133554: {
                                c.cycles += 5;
                                c.lwc1(0, stack(c, STICK_FLIP));
                                c.c_eq_s(0, 1);
                                if c.fcc() {
                                    c.swc1(23, stack(c, STICK_FLIP));
                                    break 'b_133554;
                                } else {
                                    break 'b_133554;
                                }
                            }
                            c.cycles += 3;
                            c.mov_s(23, 1);
                            c.mov_s(24, 23);
                            c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                            break 'b_133560;
                        }
                    }
                    'b_133594: {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[A0][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 14) as u32);
                        c.g[A2][0] = c.g[A2][0] & 0x1;
                        spin_callee::UPDATE_TRICK.call(c, 0x00133574)?;
                        c.g[S2][0] = c.g[V0][0];
                        if c.g[S2][0] != 0u64 {
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_133594;
                        } else {
                            'b_133590: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw(at(c, S0, MODE));
                                c.g[V0][0] = 0x3;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.sw(at(c, S0, MODE), 0u64);
                                    break 'b_133590;
                                } else {
                                    break 'b_133590;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_133594;
                        }
                    }
                    'b_1335d8: {
                        'b_1335d0: {
                            c.cycles += 7;
                            c.g[A0][0] = 0xffffffffffffffff;
                            c.sw(at(c, V0, R_LEAN_DIR), 0u64);
                            c.g[V1][0] = c.lw(at(c, S1, PAD_STICKS));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 12);
                            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 30) as u32);
                            let t = c.g[V1][0] == c.g[A0][0];
                            c.lwc1(27, gp(c, 0x1bac));
                            if t {
                                break 'b_1335d0;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x1;
                                c.mtc1(0, 0);
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_1335d8;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);

                                    break 'b_1335d8;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 0);
                        break 'b_1335d8;
                    }
                    'b_13363c: {
                        c.cycles += 5;
                        c.mtc1(0, 2);
                        c.c_eq_s(0, 2);
                        if c.fcc() {
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_13363c;
                        } else {
                            'b_133634: {
                                'b_133630: {
                                    'b_133624: {
                                        c.cycles += 6;
                                        c.lwc1(1, at(c, S0, LEAN_Y));
                                        c.mov_s(23, 0);
                                        c.c_lt_s(1, 2);
                                        let t = c.fcc();
                                        c.lwc1(27, gp(c, 0x1bb0));
                                        if t {
                                            break 'b_133624;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(1, 2);
                                            if !c.fcc() {
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_133630;
                                            } else {
                                                c.cycles += 4;
                                                c.c_lt_s(23, 2);
                                                if !c.fcc() {
                                                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                    break 'b_133630;
                                                } else {
                                                    break 'b_133624;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                    c.g[V0][0] = 0x2;
                                    break 'b_133634;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0x1;
                                break 'b_133634;
                            }
                            c.cycles += 2;
                            c.sw(at(c, V1, R_LEAN_DIR), c.g[V0][0]);
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_13363c;
                        }
                    }
                    'b_133750: {
                        'b_13374c: {
                            'b_13366c: {
                                c.cycles += 5;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, spin_callee::RIDER_HANDLE_BOOST_INPUT, 0x00133648)?;
                                let t = c.g[S2][0] == 0u64;
                                c.g[S3][0] = c.lw(at(c, S0, PHASE));
                                if t {
                                    break 'b_13366c;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(0, 23);
                                    c.g[V0][0] = c.lw(at(c, S0, MODE));
                                    let t = c.g[V0][0] != 0u64;
                                    c.mov_s(24, 23);
                                    if t {
                                        break 'b_13374c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x3;
                                        c.sw(at(c, S0, MODE), c.g[V0][0]);
                                        break 'b_13374c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            if c.g[S3][0] != c.g[V0][0] {
                                c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                break 'b_133750;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x00133688)?;
                                c.g[V1][0] = 0x120;
                                if c.g[V0][0] == c.g[V1][0] {
                                    c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                    break 'b_133750;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    spin_callee::TRY_REVERSE.call(c, 0x0013369c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                        break 'b_133750;
                                    } else {
                                        'b_1336cc: {
                                            c.cycles += 6;
                                            c.mtc1(0, 20);
                                            c.lwc1(0, stack(c, STICK_SPIN));
                                            c.c_eq_s(0, 20);
                                            let t = !c.fcc();
                                            c.lwc1(0, stack(c, STICK_FLIP));
                                            if t {
                                                break 'b_1336cc;
                                            } else {
                                                c.cycles += 4;
                                                c.c_eq_s(0, 20);
                                                if c.fcc() {
                                                    c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                                    break 'b_133750;
                                                } else {
                                                    break 'b_1336cc;
                                                }
                                            }
                                        }
                                        c.cycles += 32;
                                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                                        c.g[A1][0] = c.g[SP][0]; // &stick_flip
                                        c.lwc1(12, gp(c, -0x751c));
                                        c.g[A2][0] = c.g[SP][0] | 0x4; // &stick_spin
                                        c.mov_s(13, 20);
                                        call_ptr(
                                            c,
                                            spin_callee::INPUT_QUANTIZE_STICK_DIRECTION,
                                            0x001336e4,
                                        )?;
                                        c.lwc1(12, stack(c, STICK_FLIP));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.lwc1(13, stack(c, STICK_SPIN));
                                        spin_callee::SET_SPIN_DIRECTION.call(c, 0x001336f4)?;
                                        c.lwc1(1, gp(c, -0x7518));
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.swc1(20, at(c, S0, FLIP_AMOUNT));
                                        c.swc1(20, at(c, S0, SPIN_AMOUNT));
                                        c.swc1(1, at(c, S0, SPIN_RATE));
                                        c.swc1(1, at(c, V0, R_SPIN_SPEED));
                                        c.lwc1(0, gp(c, -0x7514));
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.swc1(0, at(c, S0, FLIP_RATE));
                                        c.swc1(0, at(c, V0, R_FLIP_SPEED));
                                        c.sw(at(c, S0, PHASE), 0u64);
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.g[A1][0] = c.lw(at(c, V0, R_STANCE));
                                        c.g[V1][0] = c.lw(at(c, V0, R_STANCE_TAKEOFF));
                                        c.g[A0][0] = c.lw(at(c, V0, R_METRIX));
                                        c.g[A1][0] = c.g[A1][0] ^ c.g[V1][0];
                                        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
                                        call_ptr(
                                            c,
                                            spin_callee::METRIX_START_SPIN_TRICK,
                                            0x0013373c,
                                        )?;
                                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                                        c.mov_s(12, 0);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013374c)?;
                                        break 'b_13374c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                        break 'b_133750;
                    }
                    'b_133c40: {
                        'b_13383c: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_13383c;
                            } else {
                                'b_1337b0: {
                                    c.cycles += 5;
                                    c.lwc1(0, at(c, S0, SPIN_TARGET));
                                    c.mtc1(0, 1);
                                    c.c_eq_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_1337b0;
                                    } else {
                                        c.cycles += 4;
                                        c.lwc1(0, at(c, S0, FLIP_TARGET));
                                        c.c_eq_s(0, 1);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_1337b0;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, SPIN));
                                            c.swc1(1, stack(c, STICK_SPIN));
                                            c.c_eq_s(0, 1);
                                            let t = !c.fcc();
                                            c.swc1(1, stack(c, STICK_FLIP));
                                            if t {
                                                break 'b_133c40;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, FLIP));
                                                c.c_eq_s(0, 1);
                                                let t = !c.fcc();
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_133c40;
                                                } else {
                                                    c.cycles += 2;
                                                    c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                    break 'b_133c40;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_133810: {
                                    c.cycles += 11;
                                    c.lwc1(0, gp(c, -0x7510));
                                    c.lwc1(3, gp(c, 0x1ba4));
                                    c.lwc1(1, at(c, S0, SPIN));
                                    c.lwc1(2, at(c, S0, SPIN_TARGET));
                                    c.sub_s(0, 0, 3);
                                    c.sub_s(1, 1, 2);
                                    c.abs_s(1, 1);
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.lwc1(0, stack(c, STICK_SPIN));
                                    if t {
                                        break 'b_133810;
                                    } else {
                                        'b_13380c: {
                                            c.cycles += 10;
                                            c.lwc1(0, gp(c, -0x750c));
                                            c.lwc1(1, at(c, S0, FLIP));
                                            c.lwc1(2, at(c, S0, FLIP_TARGET));
                                            c.sub_s(0, 0, 3);
                                            c.sub_s(1, 1, 2);
                                            c.abs_s(1, 1);
                                            c.c_lt_s(1, 0);
                                            let t = !c.fcc();
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_13380c;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                break 'b_133c40;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        break 'b_133810;
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 1);
                                c.c_eq_s(0, 1);
                                let t = !c.fcc();
                                c.lwc1(0, stack(c, STICK_FLIP));
                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 3;
                                    c.c_eq_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133c40;
                                    } else {
                                        c.cycles += 3;
                                        c.swc1(1, at(c, S0, FLIP_TARGET));
                                        c.swc1(1, at(c, S0, SPIN_TARGET));
                                        break 'b_133c40;
                                    }
                                }
                            }
                        }
                        'b_133b68: {
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = 0x2;
                                break 'b_133b68;
                            } else {
                                'b_133954: {
                                    'b_133950: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        c.mtc1(0, 5);
                                        c.c_eq_s(0, 5);
                                        let t = !c.fcc();
                                        c.lwc1(0, stack(c, STICK_FLIP));
                                        if t {
                                            break 'b_133950;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(0, 5);
                                            if !c.fcc() {
                                                c.sw(at(c, S0, RELEASED), 0u64);
                                                break 'b_133954;
                                            } else {
                                                'b_1338b0: {
                                                    c.cycles += 12;
                                                    c.lwc1(3, at(c, S0, SPIN_TARGET));
                                                    c.lwc1(0, at(c, S0, SPIN));
                                                    c.lwc1(1, at(c, S0, RELEASED));
                                                    c.lwc1(2, gp(c, -0x7508));
                                                    c.sub_s(0, 3, 0);
                                                    c.lwc1(4, gp(c, -0x7504));
                                                    c.abs_s(0, 0);
                                                    c.add_s(1, 1, 2);
                                                    c.c_lt_s(4, 0);
                                                    let t = !c.fcc();
                                                    c.swc1(1, at(c, S0, RELEASED));
                                                    if t {
                                                        break 'b_1338b0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_lt_s(3, 5);
                                                        if !c.fcc() {
                                                            c.sub_s(3, 3, 4);
                                                            break 'b_1338b0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.add_s(3, 3, 4);
                                                            break 'b_1338b0;
                                                        }
                                                    }
                                                }
                                                'b_1338f0: {
                                                    c.cycles += 9;
                                                    c.lwc1(2, at(c, S0, FLIP_TARGET));
                                                    c.lwc1(0, at(c, S0, FLIP));
                                                    c.lwc1(1, gp(c, -0x7500));
                                                    c.sub_s(0, 2, 0);
                                                    c.abs_s(0, 0);
                                                    c.c_lt_s(1, 0);
                                                    if !c.fcc() {
                                                        c.lwc1(1, at(c, S0, RELEASED));
                                                        break 'b_1338f0;
                                                    } else {
                                                        'b_1338ec: {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 0);
                                                            c.c_lt_s(2, 0);
                                                            if !c.fcc() {
                                                                c.sub_s(2, 2, 1);
                                                                break 'b_1338ec;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.add_s(2, 2, 1);
                                                                break 'b_1338ec;
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.lwc1(1, at(c, S0, RELEASED));
                                                        break 'b_1338f0;
                                                    }
                                                }
                                                'b_133918: {
                                                    c.cycles += 6;
                                                    c.lwc1(0, gp(c, -0x74fc));
                                                    c.mtc1(0, 4);
                                                    c.c_lt_s(0, 1);
                                                    if !c.fcc() {
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_133918;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.swc1(3, at(c, S0, SPIN_TARGET));
                                                        c.swc1(2, at(c, S0, FLIP_TARGET));
                                                        c.lwc1(4, gp(c, 0x1ba0));
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_133918;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.sub_s(0, 3, 0);
                                                c.abs_s(0, 0);
                                                c.c_le_s(0, 4);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_133c40;
                                                } else {
                                                    c.cycles += 7;
                                                    c.lwc1(0, at(c, S0, FLIP));
                                                    c.sub_s(0, 2, 0);
                                                    c.abs_s(0, 0);
                                                    c.c_le_s(0, 4);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_133c40;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                        break 'b_133c40;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw(at(c, S0, RELEASED), 0u64);
                                    break 'b_133954;
                                }
                                'b_133980: {
                                    'b_133974: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        c.lwc1(1, at(c, S0, RELEASED));
                                        c.c_eq_s(0, 1);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, SPIN_HELD));
                                            break 'b_133974;
                                        } else {
                                            c.cycles += 2;
                                            c.swc1(1, at(c, S0, SPIN_HELD));
                                            break 'b_133980;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.lwc1(1, gp(c, -0x74f8));
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, at(c, S0, SPIN_HELD));
                                    break 'b_133980;
                                }
                                'b_1339ac: {
                                    'b_1339a0: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_FLIP));
                                        c.mtc1(0, 1);
                                        c.c_eq_s(0, 1);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, FLIP_HELD));
                                            break 'b_1339a0;
                                        } else {
                                            c.cycles += 2;
                                            c.swc1(1, at(c, S0, FLIP_HELD));
                                            break 'b_1339ac;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.lwc1(1, gp(c, -0x74f4));
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, at(c, S0, FLIP_HELD));
                                    break 'b_1339ac;
                                }
                                'b_133a8c: {
                                    c.cycles += 10;
                                    c.lwc1(0, at(c, S0, SPIN));
                                    c.g[V1][0] = sx(at(c, S0, SPIN_TARGET));
                                    c.lwc1(2, at(c, S0, SPIN_TARGET));
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.sub_s(0, 0, 2);
                                    c.abs_s(0, 0);
                                    c.c_le_s(0, 1);
                                    if !c.fcc() {
                                        c.lwc1(0, at(c, S0, FLIP));
                                        break 'b_133a8c;
                                    } else {
                                        'b_1339ec: {
                                            c.cycles += 5;
                                            c.mtc1(0, 1);
                                            c.c_eq_s(2, 1);
                                            let t = !c.fcc();
                                            c.mov_s(3, 1);
                                            if t {
                                                break 'b_1339ec;
                                            } else {
                                                c.cycles += 1;
                                                c.lwc1(3, gp(c, -0x74f0));
                                                break 'b_1339ec;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, at(c, S0, SPIN_HELD));
                                        c.c_lt_s(3, 0);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, FLIP));
                                            break 'b_133a8c;
                                        } else {
                                            'b_133a54: {
                                                'b_133a50: {
                                                    'b_133a30: {
                                                        c.cycles += 4;
                                                        c.c_eq_s(2, 1);
                                                        let t = !c.fcc();
                                                        c.lwc1(0, stack(c, STICK_SPIN));
                                                        if t {
                                                            break 'b_133a30;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = 0x1;
                                                            c.c_lt_s(0, 1);
                                                            c.lwc1(0, gp(c, -0x74ec));
                                                            let t = !c.fcc();
                                                            c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                                            if t {
                                                                break 'b_133a50;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(0, gp(c, -0x74e8));
                                                                c.swc1(0, at(c, V1, 0));
                                                                break 'b_133a54;
                                                            }
                                                        }
                                                    }
                                                    'b_133a48: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(2, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_133a48;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x74e4));
                                                            c.sub_s(0, 2, 0);
                                                            break 'b_133a50;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x74e0));
                                                    c.add_s(0, 2, 0);
                                                    break 'b_133a50;
                                                }
                                                c.cycles += 1;
                                                c.swc1(0, at(c, V1, 0));
                                                break 'b_133a54;
                                            }
                                            c.cycles += 7;
                                            c.lwc1(0, at(c, S0, SPIN_TARGET));
                                            c.lwc1(1, gp(c, 0x1bb4));
                                            c.abs_s(0, 0);
                                            c.c_lt_s(1, 0);
                                            if !c.fcc() {
                                                c.lwc1(0, at(c, S0, FLIP));
                                                break 'b_133a8c;
                                            } else {
                                                c.cycles += 7;
                                                c.lwc1(0, at(c, S0, SPIN_RATE));
                                                c.lwc1(2, gp(c, -0x74dc));
                                                c.lwc1(1, gp(c, -0x74d8));
                                                c.mul_s(0, 0, 2);
                                                c.max_s(0, 0, 1);
                                                c.swc1(0, at(c, S0, SPIN_RATE));
                                                c.lwc1(0, at(c, S0, FLIP));
                                                break 'b_133a8c;
                                            }
                                        }
                                    }
                                }
                                'b_133b58: {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx(at(c, S0, FLIP_TARGET));
                                    c.lwc1(2, at(c, S0, FLIP_TARGET));
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.sub_s(0, 0, 2);
                                    c.abs_s(0, 0);
                                    c.c_le_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133b58;
                                    } else {
                                        'b_133ac4: {
                                            c.cycles += 5;
                                            c.mtc1(0, 1);
                                            c.c_eq_s(2, 1);
                                            let t = !c.fcc();
                                            c.mov_s(3, 1);
                                            if t {
                                                break 'b_133ac4;
                                            } else {
                                                c.cycles += 1;
                                                c.lwc1(3, gp(c, -0x74d4));
                                                break 'b_133ac4;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.lwc1(0, at(c, S0, FLIP_HELD));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_133b58;
                                        } else {
                                            'b_133b28: {
                                                'b_133b24: {
                                                    'b_133b04: {
                                                        c.cycles += 4;
                                                        c.c_eq_s(2, 1);
                                                        let t = !c.fcc();
                                                        c.lwc1(0, stack(c, STICK_FLIP));
                                                        if t {
                                                            break 'b_133b04;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = 0x1;
                                                            c.c_lt_s(0, 1);
                                                            c.lwc1(0, gp(c, -0x74d0));
                                                            let t = !c.fcc();
                                                            c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                                            if t {
                                                                break 'b_133b24;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(0, gp(c, -0x74cc));
                                                                c.swc1(0, at(c, V1, 0));
                                                                break 'b_133b28;
                                                            }
                                                        }
                                                    }
                                                    'b_133b1c: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(2, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_133b1c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x74c8));
                                                            c.sub_s(0, 2, 0);
                                                            break 'b_133b24;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x74c4));
                                                    c.add_s(0, 2, 0);
                                                    break 'b_133b24;
                                                }
                                                c.cycles += 1;
                                                c.swc1(0, at(c, V1, 0));
                                                break 'b_133b28;
                                            }
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, FLIP_TARGET));
                                            c.lwc1(1, gp(c, 0x1bb8));
                                            c.abs_s(0, 0);
                                            c.c_lt_s(1, 0);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_133b58;
                                            } else {
                                                c.cycles += 6;
                                                c.lwc1(0, at(c, S0, FLIP_RATE));
                                                c.lwc1(2, gp(c, -0x74c0));
                                                c.lwc1(1, gp(c, -0x74bc));
                                                c.mul_s(0, 0, 2);
                                                c.max_s(0, 0, 1);
                                                c.swc1(0, at(c, S0, FLIP_RATE));
                                                break 'b_133b58;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 2;
                                    c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                    break 'b_133c40;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_133c40;
                        } else {
                            'b_133bc0: {
                                c.cycles += 7;
                                c.lwc1(1, at(c, S0, SPIN));
                                c.lwc1(0, at(c, S0, SPIN_TARGET));
                                c.sw(at(c, S0, MODE), 0u64);
                                c.c_eq_s(1, 0);
                                c.sw(stack(c, STICK_SPIN), 0u64);
                                let t = !c.fcc();
                                c.sw(stack(c, STICK_FLIP), 0u64);
                                if t {
                                    break 'b_133bc0;
                                } else {
                                    c.cycles += 6;
                                    c.lwc1(1, at(c, S0, FLIP));
                                    c.lwc1(0, at(c, S0, FLIP_TARGET));
                                    c.c_eq_s(1, 0);
                                    let t = !c.fcc();
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_133bc0;
                                    } else {
                                        c.cycles += 7;
                                        c.sw(at(c, S0, SPIN), 0u64);
                                        c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                        c.sw(at(c, S0, SPIN_TARGET), 0u64);
                                        c.sw(at(c, S0, FLIP), 0u64);
                                        c.sw(at(c, S0, FLIP_TARGET), 0u64);
                                        c.sw(at(c, S0, TURNED), 0u64);
                                        break 'b_133c40;
                                    }
                                }
                            }
                            'b_133bf4: {
                                c.cycles += 2;
                                if c.g[S2][0] != 0u64 {
                                    c.lwc1(0, at(c, S0, FLIP_TARGET));
                                    break 'b_133bf4;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                    call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x00133bd8)?;
                                    c.g[V1][0] = 0x120;
                                    if c.g[V0][0] == c.g[V1][0] {
                                        c.lwc1(0, at(c, S0, FLIP_TARGET));
                                        break 'b_133bf4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        spin_callee::TRY_REVERSE.call(c, 0x00133bec)?;

                                        break 'b_133c40;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(1, at(c, S0, FLIP));
                            c.c_eq_s(0, 1);
                            let t = c.fcc();

                            if t {
                                break 'b_133c40;
                            } else {
                                'b_133c24: {
                                    c.cycles += 4;
                                    c.mtc1(0, 0);
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133c24;
                                    } else {
                                        c.cycles += 4;
                                        c.c_lt_s(0, 23);
                                        if c.fcc() {
                                            c.neg_s(23, 23);
                                            break 'b_133c40;
                                        } else {
                                            break 'b_133c24;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 4;
                                    c.c_lt_s(23, 0);
                                    if c.fcc() {
                                        c.neg_s(23, 23);
                                        break 'b_133c40;
                                    } else {
                                        break 'b_133c40;
                                    }
                                }
                            }
                        }
                    }
                    'b_133d70: {
                        'b_133cec: {
                            'b_133ce8: {
                                c.cycles += 34;
                                spin_callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x00133c48)?;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                spin_callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00133c5c)?;
                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x00133c70)?;
                                c.lwc1(1, gp(c, -0x74b8));
                                c.mul_s(0, 0, 1);
                                c.add_s(0, 0, 20);
                                c.div_s(25, 20, 0);
                                spin_callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x00133c88)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                spin_callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00133c94)?;
                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x00133ca8)?;
                                c.lwc1(1, gp(c, -0x74b4));
                                c.mtc1(0, 3);
                                c.lwc1(2, stack(c, STICK_FLIP));
                                c.mul_s(0, 0, 1);
                                c.c_eq_s(2, 3);
                                let t = !c.fcc();
                                c.add_s(26, 0, 20);
                                if t {
                                    break 'b_133ce8;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, stack(c, STICK_SPIN));
                                    c.c_eq_s(0, 3);
                                    let t = !c.fcc();
                                    c.lwc1(20, stack(c, STICK_SPIN));
                                    if t {
                                        break 'b_133cec;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, gp(c, -0x74b0));
                                        c.swc1(0, at(c, S0, STICK_ANGLE));
                                        break 'b_133d70;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.lwc1(20, stack(c, STICK_SPIN));
                            break 'b_133cec;
                        }
                        'b_133d6c: {
                            'b_133d2c: {
                                c.cycles += 5;
                                c.mtc1(0, 22);
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                c.lwc1(21, stack(c, STICK_FLIP));
                                if t {
                                    break 'b_133d2c;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(0, 21);
                                    if t {
                                        break 'b_133d6c;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(0, gp(c, -0x74ac));
                                        if !c.fcc() {
                                            c.swc1(0, at(c, S0, STICK_ANGLE));
                                            break 'b_133d70;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x74a8));
                                            c.swc1(0, at(c, S0, STICK_ANGLE));
                                            break 'b_133d70;
                                        }
                                    }
                                }
                            }
                            'b_133d68: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                call_ptr(c, spin_callee::MATH_ATAN, 0x00133d38)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_133d68;
                                } else {
                                    'b_133d60: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_133d60;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x74a4));
                                            c.add_s(12, 12, 0);
                                            break 'b_133d68;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, gp(c, -0x74a0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_133d68;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(0, 12);
                            break 'b_133d6c;
                        }
                        c.cycles += 1;
                        c.swc1(0, at(c, S0, STICK_ANGLE));
                        break 'b_133d70;
                    }
                    'b_133eb0: {
                        'b_133eac: {
                            'b_133d94: {
                                c.cycles += 6;
                                c.lwc1(1, at(c, S0, SPIN_TARGET));
                                c.lwc1(4, at(c, S0, SPIN));
                                c.c_eq_s(1, 4);
                                let t = !c.fcc();
                                c.lwc1(2, stack(c, STICK_SPIN));
                                if t {
                                    break 'b_133d94;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.sw(at(c, V0, R_SPIN_SPEED), 0u64);
                                    break 'b_133eac;
                                }
                            }
                            'b_133dc4: {
                                c.cycles += 5;
                                c.mtc1(0, 3);
                                c.mul_s(0, 2, 1);
                                c.c_lt_s(0, 3);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133dc4;
                                } else {
                                    c.cycles += 7;
                                    c.lwc1(0, gp(c, -0x749c));
                                    c.lwc1(1, at(c, S0, SPIN_RATE));
                                    c.mul_s(0, 25, 0);
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.min_s(1, 1, 0);
                                    c.swc1(1, at(c, V0, R_SPIN_SPEED));
                                    break 'b_133eac;
                                }
                            }
                            'b_133e70: {
                                'b_133de4: {
                                    c.cycles += 4;
                                    c.c_eq_s(2, 3);
                                    let t = c.fcc();
                                    c.lwc1(0, stack(c, STICK_FLIP));
                                    if t {
                                        break 'b_133de4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.lwc1(0, at(c, S0, SPIN_RATE));
                                        c.swc1(0, at(c, V0, R_SPIN_SPEED));
                                        break 'b_133e70;
                                    }
                                }
                                'b_133e28: {
                                    c.cycles += 5;
                                    c.sub_s(5, 4, 1);
                                    c.c_eq_s(0, 3);
                                    let t = !c.fcc();
                                    c.lwc1(4, at(c, S0, SPIN_RATE));
                                    if t {
                                        break 'b_133e28;
                                    } else {
                                        c.cycles += 12;
                                        c.lwc1(2, gp(c, -0x7498));
                                        c.abs_s(0, 5);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 1);
                                        c.mul_s(0, 0, 2);
                                        c.lwc1(3, gp(c, -0x7494));
                                        c.lwc1(2, gp(c, -0x7490));
                                        c.sub_s(1, 1, 0);
                                        c.mul_s(0, 0, 3);
                                        c.mul_s(1, 1, 2);
                                        c.add_s(0, 0, 1);
                                        c.max_s(4, 0, 4);
                                        break 'b_133e28;
                                    }
                                }
                                'b_133e6c: {
                                    'b_133e68: {
                                        c.cycles += 10;
                                        c.lwc1(1, gp(c, 0x1ba0));
                                        c.abs_s(0, 5);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 2);
                                        c.div_s(3, 0, 1);
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.c_le_s(3, 2);
                                        let t = !c.fcc();
                                        c.g[V0][0] = sx(at(c, V0, R_SPIN_SPEED));
                                        if t {
                                            break 'b_133e68;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(0, gp(c, -0x748c));
                                            c.lwc1(1, gp(c, -0x7488));
                                            c.mul_s(0, 3, 0);
                                            c.add_s(0, 0, 1);
                                            c.mul_s(0, 4, 0);
                                            break 'b_133e6c;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.mov_s(0, 4);
                                    break 'b_133e6c;
                                }
                                c.cycles += 1;
                                c.swc1(0, at(c, V0, 0));
                                break 'b_133e70;
                            }
                            c.cycles += 11;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.mtc1(0, 1);
                            c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                            c.mul_s(0, 0, 26);
                            c.swc1(0, at(c, V0, R_SPIN_SPEED));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.lwc1(0, at(c, V0, R_2EC));
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                break 'b_133eb0;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                                c.lwc1(1, gp(c, -0x7484));
                                c.mul_s(0, 0, 1);
                                c.swc1(0, at(c, V0, R_SPIN_SPEED));
                                break 'b_133eac;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, at(c, S0, FLIP_TARGET));
                        break 'b_133eb0;
                    }
                    'b_133fe4: {
                        'b_133ed0: {
                            c.cycles += 5;
                            c.lwc1(4, at(c, S0, FLIP));
                            c.c_eq_s(1, 4);
                            let t = !c.fcc();
                            c.lwc1(2, stack(c, STICK_FLIP));
                            if t {
                                break 'b_133ed0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.sw(at(c, V0, R_FLIP_SPEED), 0u64);
                                break 'b_133fe4;
                            }
                        }
                        'b_133f00: {
                            c.cycles += 5;
                            c.mtc1(0, 3);
                            c.mul_s(0, 2, 1);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_133f00;
                            } else {
                                c.cycles += 7;
                                c.lwc1(0, gp(c, -0x7480));
                                c.lwc1(1, at(c, S0, FLIP_RATE));
                                c.mul_s(0, 25, 0);
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.min_s(1, 1, 0);
                                c.swc1(1, at(c, V0, R_FLIP_SPEED));
                                break 'b_133fe4;
                            }
                        }
                        'b_133fac: {
                            'b_133f20: {
                                c.cycles += 4;
                                c.c_eq_s(2, 3);
                                let t = c.fcc();
                                c.lwc1(0, stack(c, STICK_SPIN));
                                if t {
                                    break 'b_133f20;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.lwc1(0, at(c, S0, FLIP_RATE));
                                    c.swc1(0, at(c, V0, R_FLIP_SPEED));
                                    break 'b_133fac;
                                }
                            }
                            'b_133f64: {
                                c.cycles += 5;
                                c.sub_s(5, 4, 1);
                                c.c_eq_s(0, 3);
                                let t = !c.fcc();
                                c.lwc1(4, at(c, S0, FLIP_RATE));
                                if t {
                                    break 'b_133f64;
                                } else {
                                    c.cycles += 12;
                                    c.lwc1(2, gp(c, -0x747c));
                                    c.abs_s(0, 5);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 1);
                                    c.mul_s(0, 0, 2);
                                    c.lwc1(3, gp(c, -0x7478));
                                    c.lwc1(2, gp(c, -0x7474));
                                    c.sub_s(1, 1, 0);
                                    c.mul_s(0, 0, 3);
                                    c.mul_s(1, 1, 2);
                                    c.add_s(0, 0, 1);
                                    c.max_s(4, 0, 4);
                                    break 'b_133f64;
                                }
                            }
                            'b_133fa8: {
                                'b_133fa4: {
                                    c.cycles += 10;
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.abs_s(0, 5);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.div_s(3, 0, 1);
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.c_le_s(3, 2);
                                    let t = !c.fcc();
                                    c.g[V0][0] = sx(at(c, V0, R_FLIP_SPEED));
                                    if t {
                                        break 'b_133fa4;
                                    } else {
                                        c.cycles += 6;
                                        c.lwc1(0, gp(c, -0x7470));
                                        c.lwc1(1, gp(c, -0x746c));
                                        c.mul_s(0, 3, 0);
                                        c.add_s(0, 0, 1);
                                        c.mul_s(0, 4, 0);
                                        break 'b_133fa8;
                                    }
                                }
                                c.cycles += 1;
                                c.mov_s(0, 4);
                                break 'b_133fa8;
                            }
                            c.cycles += 1;
                            c.swc1(0, at(c, V0, 0));
                            break 'b_133fac;
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        c.mtc1(0, 1);
                        c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                        c.mul_s(0, 0, 26);
                        c.swc1(0, at(c, V0, R_FLIP_SPEED));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        c.lwc1(0, at(c, V0, R_2EC));
                        c.c_lt_s(1, 0);
                        let t = !c.fcc();

                        if t {
                            break 'b_133fe4;
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                            c.lwc1(1, gp(c, -0x7468));
                            c.mul_s(0, 0, 1);
                            c.swc1(0, at(c, V0, R_FLIP_SPEED));
                            break 'b_133fe4;
                        }
                    }
                    'b_133ffc: {
                        c.cycles += 5;
                        c.lwc1(1, gp(c, -0x7464));
                        c.abs_s(0, 24);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();

                        if t {
                            break 'b_133ffc;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 24);
                            break 'b_133ffc;
                        }
                    }
                    'b_134010: {
                        c.cycles += 4;
                        c.abs_s(0, 23);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();

                        if t {
                            break 'b_134010;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 23);
                            break 'b_134010;
                        }
                    }
                    'b_134064: {
                        'b_13403c: {
                            c.cycles += 6;
                            c.mtc1(0, 0);
                            c.lwc1(9, at(c, S0, SPIN));
                            c.c_lt_s(24, 0);
                            let t = !c.fcc();
                            c.lwc1(8, at(c, S0, FLIP));
                            if t {
                                break 'b_13403c;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, gp(c, 0x1ba8));
                                c.lwc1(4, at(c, S0, LEAN_X));
                                c.mov_s(5, 0);
                                c.neg_s(3, 0);
                                break 'b_134064;
                            }
                        }
                        'b_134058: {
                            c.cycles += 4;
                            c.c_le_s(24, 0);
                            let t = !c.fcc();
                            c.lwc1(5, gp(c, 0x1ba8));
                            if t {
                                break 'b_134058;
                            } else {
                                c.cycles += 3;
                                c.lwc1(3, at(c, S0, LEAN_X));
                                c.mov_s(4, 3);
                                break 'b_134064;
                            }
                        }
                        c.cycles += 3;
                        c.lwc1(3, gp(c, 0x1ba8));
                        c.lwc1(4, at(c, S0, LEAN_X));
                        c.mov_s(5, 3);
                        break 'b_134064;
                    }
                    'b_1340a0: {
                        c.cycles += 14;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.sub_s(0, 3, 4);
                        c.abs_s(0, 0);
                        c.lwc1(2, gp(c, -0x7460));
                        c.mul_s(0, 0, 1);
                        c.abs_s(6, 24);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 1);
                        c.div_s(0, 0, 5);
                        c.c_le_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(7, 2);
                        if t {
                            break 'b_1340a0;
                        } else {
                            c.cycles += 1;
                            c.min_s(7, 0, 1);
                            break 'b_1340a0;
                        }
                    }
                    'b_1340ec: {
                        'b_1340d8: {
                            c.cycles += 12;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.lwc1(1, gp(c, -0x745c));
                            c.mul_s(0, 6, 2);
                            c.mul_s(1, 7, 1);
                            c.add_s(0, 0, 2);
                            c.mul_s(1, 0, 1);
                            c.add_s(2, 3, 1);
                            c.c_lt_s(2, 4);
                            if !c.fcc() {
                                c.sub_s(0, 3, 1);
                                break 'b_1340d8;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 4, 1);
                                break 'b_1340ec;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(4, 0);
                        let t = !c.fcc();
                        c.mov_s(2, 3);
                        if t {
                            break 'b_1340ec;
                        } else {
                            c.cycles += 1;
                            c.add_s(2, 4, 1);
                            break 'b_1340ec;
                        }
                    }
                    'b_134138: {
                        'b_134130: {
                            'b_134114: {
                                c.cycles += 6;
                                c.lwc1(0, at(c, S0, SPIN));
                                c.lwc1(1, at(c, S0, SPIN_TARGET));
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();
                                c.swc1(2, at(c, S0, LEAN_X));
                                if t {
                                    break 'b_134114;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 0, 2);
                                    c.sub_s(0, 0, 4);
                                    c.min_s(0, 1, 0);
                                    break 'b_134130;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                break 'b_134138;
                            } else {
                                c.cycles += 3;
                                c.add_s(0, 0, 2);
                                c.sub_s(0, 0, 4);
                                c.max_s(0, 1, 0);
                                break 'b_134130;
                            }
                        }
                        c.cycles += 2;
                        c.swc1(0, at(c, S0, SPIN));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        break 'b_134138;
                    }
                    'b_134178: {
                        'b_134168: {
                            c.cycles += 10;
                            c.lwc1(1, gp(c, -0x7458));
                            c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                            c.lwc1(2, at(c, S0, SPIN_TARGET));
                            c.mul_s(3, 0, 1);
                            c.lwc1(1, at(c, S0, SPIN));
                            c.add_s(0, 2, 3);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.sub_s(0, 2, 3);
                                break 'b_134168;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 1, 3);
                                break 'b_134178;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(2, 1, 3);
                            break 'b_134178;
                        } else {
                            break 'b_134178;
                        }
                    }
                    'b_1341c4: {
                        'b_1341a8: {
                            c.cycles += 9;
                            c.sub_s(1, 2, 1);
                            c.lwc1(0, at(c, S0, SPIN_TOTAL));
                            c.mtc1(0, 3);
                            c.swc1(2, at(c, S0, SPIN));
                            c.add_s(0, 0, 1);
                            c.c_lt_s(23, 3);
                            let t = !c.fcc();
                            c.swc1(0, at(c, S0, SPIN_TOTAL));
                            if t {
                                break 'b_1341a8;
                            } else {
                                c.cycles += 3;
                                c.neg_s(4, 27);
                                c.lwc1(5, at(c, S0, LEAN_Y));
                                break 'b_1341c4;
                            }
                        }
                        'b_1341c0: {
                            c.cycles += 4;
                            c.c_le_s(23, 3);
                            let t = !c.fcc();
                            c.lwc1(5, at(c, S0, LEAN_Y));
                            if t {
                                break 'b_1341c0;
                            } else {
                                c.cycles += 2;
                                c.mov_s(4, 5);
                                break 'b_1341c4;
                            }
                        }
                        c.cycles += 1;
                        c.mov_s(4, 27);
                        break 'b_1341c4;
                    }
                    'b_1341dc: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 4);
                        let t = !c.fcc();
                        c.lwc1(0, gp(c, 0x1bbc));
                        if t {
                            break 'b_1341dc;
                        } else {
                            c.cycles += 1;
                            c.min_s(4, 0, 4);
                            break 'b_1341dc;
                        }
                    }
                    'b_13421c: {
                        c.cycles += 15;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.sub_s(0, 4, 5);
                        c.abs_s(0, 0);
                        c.lwc1(3, gp(c, 0x1bac));
                        c.mul_s(0, 0, 1);
                        c.lwc1(2, gp(c, -0x7454));
                        c.abs_s(6, 23);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 1);
                        c.div_s(0, 0, 3);
                        c.c_le_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(7, 2);
                        if t {
                            break 'b_13421c;
                        } else {
                            c.cycles += 1;
                            c.min_s(7, 0, 1);
                            break 'b_13421c;
                        }
                    }
                    'b_134268: {
                        'b_134254: {
                            c.cycles += 12;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.lwc1(1, gp(c, -0x7450));
                            c.mul_s(0, 6, 2);
                            c.mul_s(1, 7, 1);
                            c.add_s(0, 0, 2);
                            c.mul_s(1, 0, 1);
                            c.add_s(2, 4, 1);
                            c.c_lt_s(2, 5);
                            if !c.fcc() {
                                c.sub_s(0, 4, 1);
                                break 'b_134254;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 5, 1);
                                break 'b_134268;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(5, 0);
                        let t = !c.fcc();
                        c.mov_s(2, 4);
                        if t {
                            break 'b_134268;
                        } else {
                            c.cycles += 1;
                            c.add_s(2, 5, 1);
                            break 'b_134268;
                        }
                    }
                    'b_1342b4: {
                        'b_1342ac: {
                            'b_134290: {
                                c.cycles += 6;
                                c.lwc1(0, at(c, S0, FLIP));
                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();
                                c.swc1(2, at(c, S0, LEAN_Y));
                                if t {
                                    break 'b_134290;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 0, 2);
                                    c.sub_s(0, 0, 5);
                                    c.min_s(0, 1, 0);
                                    break 'b_1342ac;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                break 'b_1342b4;
                            } else {
                                c.cycles += 3;
                                c.add_s(0, 0, 2);
                                c.sub_s(0, 0, 5);
                                c.max_s(0, 1, 0);
                                break 'b_1342ac;
                            }
                        }
                        c.cycles += 2;
                        c.swc1(0, at(c, S0, FLIP));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        break 'b_1342b4;
                    }
                    'b_1342f8: {
                        'b_1342e4: {
                            c.cycles += 10;
                            c.lwc1(1, gp(c, -0x744c));
                            c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                            c.lwc1(4, at(c, S0, FLIP_TARGET));
                            c.mul_s(1, 0, 1);
                            c.lwc1(2, at(c, S0, FLIP));
                            c.add_s(0, 4, 1);
                            c.c_lt_s(0, 2);
                            if !c.fcc() {
                                c.sub_s(0, 4, 1);
                                break 'b_1342e4;
                            } else {
                                c.cycles += 2;
                                c.sub_s(6, 2, 1);
                                break 'b_1342f8;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(6, 4);
                        if t {
                            break 'b_1342f8;
                        } else {
                            c.cycles += 1;
                            c.add_s(6, 2, 1);
                            break 'b_1342f8;
                        }
                    }
                    'b_134340: {
                        c.cycles += 18;
                        c.lwc1(1, at(c, S0, SPIN));
                        c.sub_s(4, 6, 2);
                        c.sub_s(5, 6, 8);
                        c.lwc1(2, at(c, S0, FLIP_TOTAL));
                        c.sub_s(1, 1, 9);
                        c.lwc1(0, at(c, S0, SPIN_AMOUNT));
                        c.lwc1(3, at(c, S0, FLIP_AMOUNT));
                        c.add_s(2, 2, 4);
                        c.swc1(6, at(c, S0, FLIP));
                        c.add_s(0, 0, 1);
                        c.g[V1][0] = c.lw(at(c, S0, RIDER));
                        c.add_s(3, 3, 5);
                        c.swc1(2, at(c, S0, FLIP_TOTAL));
                        c.swc1(0, at(c, S0, SPIN_AMOUNT));
                        c.swc1(3, at(c, S0, FLIP_AMOUNT));
                        c.g[V0][0] = c.lw(at(c, V1, R_STANCE_TAKEOFF));
                        if c.g[V0][0] != 0u64 {
                            c.neg_s(0, 0);
                            break 'b_134340;
                        } else {
                            break 'b_134340;
                        }
                    }
                    'b_134c70: {
                        'b_134c3c: {
                            c.cycles += 39;
                            c.g[A0][0] = c.lw(at(c, V1, R_METRIX));
                            c.mov_s(12, 0);
                            call_ptr(c, spin_callee::METRIX_SET_SPIN_AMOUNT, 0x0013434c)?;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013435c)?;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.lwc1(12, at(c, S0, FLIP_AMOUNT));
                            c.g[A0][0] = c.lw(at(c, V0, R_METRIX));
                            call_ptr(c, spin_callee::METRIX_SET_FLIP_AMOUNT, 0x0013436c)?;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013437c)?;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                            call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x0013438c)?;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 3);
                            c.g[S1][0] = c.g[V0][0];
                            c.lwc1(2, gp(c, 0x1ba8));
                            c.g[S4][0] = c.g[S1][0];
                            c.lwc1(1, at(c, S0, LEAN_X));
                            c.div_s(2, 3, 2);
                            c.lwc1(0, gp(c, 0x1bac));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.div_s(3, 3, 0);
                            c.lwc1(4, gp(c, -0x7448));
                            c.swc1(4, at(c, V0, R_LEAN_X_BLEND));
                            c.mul_s(1, 1, 2);
                            c.swc1(1, at(c, V0, R_LEAN_X));
                            c.lwc1(0, at(c, S0, LEAN_Y));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.mul_s(0, 0, 3);
                            c.swc1(4, at(c, V0, R_LEAN_Y_BLEND));
                            let t = c.g[S2][0] != 0u64;
                            c.swc1(0, at(c, V0, R_LEAN_Y));
                            if t {
                                break 'b_134c3c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x120;
                                let t = c.g[S1][0] == c.g[V0][0];

                                if t {
                                    break 'b_134c3c;
                                } else {
                                    'b_134554: {
                                        'b_134418: {
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, LEAN_X));
                                            c.mtc1(0, 22);
                                            c.c_eq_s(0, 22);
                                            if !c.fcc() {
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_134418;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, LEAN_Y));
                                                c.c_eq_s(0, 22);
                                                let t = c.fcc();
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_134554;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                    break 'b_134418;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x2;
                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                        call_ptr(
                                            c,
                                            spin_callee::ANIM_LAYER_ANIM_ENTRY,
                                            0x00134424,
                                        )?;
                                        c.g[V1][0] = 0x9;
                                        let t = c.g[V0][0] == c.g[V1][0];

                                        if t {
                                            break 'b_134c3c;
                                        } else {
                                            'b_134444: {
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                c.g[V0][0] = c.lw(at(c, V1, R_STANCE));
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, at(c, V1, R_298));
                                                if t {
                                                    break 'b_134444;
                                                } else {
                                                    c.cycles += 1;
                                                    c.neg_s(20, 20);
                                                    break 'b_134444;
                                                }
                                            }
                                            'b_1344bc: {
                                                'b_134480: {
                                                    c.cycles += 4;
                                                    c.c_eq_s(20, 22);
                                                    let t = !c.fcc();
                                                    c.lwc1(21, at(c, V1, R_28C));
                                                    if t {
                                                        break 'b_134480;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_eq_s(21, 22);
                                                        let t = c.fcc();
                                                        c.mov_s(12, 21);
                                                        if t {
                                                            break 'b_1344bc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.c_le_s(22, 21);
                                                            c.lwc1(12, gp(c, -0x7444));
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_1344bc;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(12, gp(c, -0x7440));

                                                                break 'b_1344bc;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.div_s(12, 21, 20);
                                                call_ptr(c, spin_callee::MATH_ATAN, 0x0013448c)?;
                                                c.c_lt_s(20, 22);
                                                let t = !c.fcc();
                                                c.mov_s(12, 0);
                                                if t {
                                                    break 'b_1344bc;
                                                } else {
                                                    'b_1344b4: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(22, 21);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_1344b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x743c));
                                                            c.add_s(12, 12, 0);
                                                            break 'b_1344bc;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x7438));
                                                    c.sub_s(12, 12, 0);
                                                    break 'b_1344bc;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.lwc1(1, gp(c, -0x7434));
                                            c.abs_s(0, 12);
                                            c.c_lt_s(1, 0);
                                            let t = c.fcc();
                                            c.g[S1][0] = 0x129;
                                            if t {
                                                break 'b_134c3c;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, gp(c, -0x7430));
                                                c.c_lt_s(0, 12);
                                                let t = c.fcc();
                                                c.g[S1][0] = 0x12a;
                                                if t {
                                                    break 'b_134c3c;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(0, gp(c, -0x742c));
                                                    c.c_lt_s(0, 12);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x12b;
                                                    if t {
                                                        break 'b_134c3c;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(0, gp(c, -0x7428));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();
                                                        c.g[S1][0] = 0x12c;
                                                        if t {
                                                            break 'b_134c3c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x7424));
                                                            c.c_lt_s(12, 0);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x130;
                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.lwc1(0, gp(c, -0x7420));
                                                                c.c_lt_s(12, 0);
                                                                let t = c.fcc();
                                                                c.g[S1][0] = 0x12f;
                                                                if t {
                                                                    break 'b_134c3c;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x741c));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = !c.fcc();
                                                                    c.g[S1][0] = 0x12d;
                                                                    if t {
                                                                        break 'b_134c3c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[S1][0] = 0x12e;
                                                                        break 'b_134c3c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    'b_134748: {
                                        c.cycles += 2;
                                        let t = c.g[S3][0] != c.g[V0][0];
                                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                        if t {
                                            break 'b_134748;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_134748;
                                            } else {
                                                'b_134650: {
                                                    'b_134648: {
                                                        'b_13463c: {
                                                            'b_134614: {
                                                                'b_1345f8: {
                                                                    'b_1345d4: {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] =
                                                                            c.lw(at(c, S0, RIDER));
                                                                        c.g[V1][0] = c.lw(at(
                                                                            c, V0, R_STANCE,
                                                                        ));
                                                                        let t = c.g[V1][0] == 0u64;
                                                                        c.lwc1(
                                                                            0,
                                                                            stack(c, STICK_SPIN),
                                                                        );
                                                                        if t {
                                                                            break 'b_1345d4;
                                                                        } else {
                                                                            'b_1345a4: {
                                                                                c.cycles += 5;
                                                                                c.neg_s(20, 0);
                                                                                c.c_eq_s(20, 22);
                                                                                let t = !c.fcc();
                                                                                c.lwc1(
                                                                                    21,
                                                                                    stack(
                                                                                        c,
                                                                                        STICK_FLIP,
                                                                                    ),
                                                                                );
                                                                                if t {
                                                                                    break 'b_1345a4;
                                                                                } else {
                                                                                    c.cycles += 4;
                                                                                    c.c_eq_s(
                                                                                        21, 22,
                                                                                    );
                                                                                    let t = c.fcc();
                                                                                    c.mov_s(12, 21);
                                                                                    if t {
                                                                                        break 'b_134650;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;

                                                                                        break 'b_1345f8;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 7;
                                                                            c.div_s(12, 21, 20);
                                                                            call_ptr(c, spin_callee::MATH_ATAN, 0x001345b0)?;
                                                                            c.c_lt_s(20, 22);
                                                                            let t = !c.fcc();
                                                                            c.mov_s(12, 0);
                                                                            if t {
                                                                                break 'b_134650;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.c_lt_s(22, 21);
                                                                                let t = c.fcc();

                                                                                if t {
                                                                                    break 'b_13463c;
                                                                                } else {
                                                                                    c.cycles += 2;

                                                                                    break 'b_134648;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.lwc1(
                                                                        20,
                                                                        stack(c, STICK_SPIN),
                                                                    );
                                                                    c.c_eq_s(20, 22);
                                                                    let t = !c.fcc();
                                                                    c.lwc1(
                                                                        21,
                                                                        stack(c, STICK_FLIP),
                                                                    );
                                                                    if t {
                                                                        break 'b_134614;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(21, 22);
                                                                        let t = c.fcc();
                                                                        c.mov_s(12, 21);
                                                                        if t {
                                                                            break 'b_134650;
                                                                        } else {
                                                                            break 'b_1345f8;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_le_s(22, 21);
                                                                c.lwc1(12, gp(c, -0x7418));
                                                                let t = !c.fcc();

                                                                if t {
                                                                    break 'b_134650;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.lwc1(12, gp(c, -0x7414));

                                                                    break 'b_134650;
                                                                }
                                                            }
                                                            c.cycles += 7;
                                                            c.div_s(12, 21, 20);
                                                            call_ptr(
                                                                c,
                                                                spin_callee::MATH_ATAN,
                                                                0x00134620,
                                                            )?;
                                                            c.c_lt_s(20, 22);
                                                            let t = !c.fcc();
                                                            c.mov_s(12, 0);
                                                            if t {
                                                                break 'b_134650;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.c_lt_s(22, 21);
                                                                let t = !c.fcc();

                                                                if t {
                                                                    break 'b_134648;
                                                                } else {
                                                                    break 'b_13463c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.lwc1(0, gp(c, -0x7410));
                                                        c.add_s(12, 12, 0);
                                                        break 'b_134650;
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x740c));
                                                    c.sub_s(12, 12, 0);
                                                    break 'b_134650;
                                                }
                                                'b_1346e4: {
                                                    c.cycles += 6;
                                                    c.lwc1(1, gp(c, -0x7408));
                                                    c.abs_s(0, 12);
                                                    c.c_lt_s(1, 0);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x10e;
                                                    if t {
                                                        break 'b_1346e4;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(0, gp(c, -0x7404));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();
                                                        c.g[S1][0] = 0x112;
                                                        if t {
                                                            break 'b_1346e4;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x7400));
                                                            c.c_lt_s(0, 12);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x110;
                                                            if t {
                                                                break 'b_1346e4;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.lwc1(0, gp(c, -0x73fc));
                                                                c.c_lt_s(0, 12);
                                                                let t = c.fcc();
                                                                c.g[S1][0] = 0x111;
                                                                if t {
                                                                    break 'b_1346e4;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x73f8));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = c.fcc();
                                                                    c.g[S1][0] = 0x114;
                                                                    if t {
                                                                        break 'b_1346e4;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(0, gp(c, -0x73f4));
                                                                        c.c_lt_s(12, 0);
                                                                        let t = c.fcc();
                                                                        c.g[S1][0] = 0x10f;
                                                                        if t {
                                                                            break 'b_1346e4;
                                                                        } else {
                                                                            c.cycles += 5;
                                                                            c.lwc1(
                                                                                0,
                                                                                gp(c, -0x73f0),
                                                                            );
                                                                            c.c_lt_s(12, 0);
                                                                            let t = !c.fcc();
                                                                            c.g[S1][0] = 0x10d;
                                                                            if t {
                                                                                break 'b_1346e4;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S1][0] = 0x113;
                                                                                break 'b_1346e4;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 25;
                                                spin_callee::BE_LIBRARY_GET_INSTANCE
                                                    .call(c, 0x001346ec)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0x3;
                                                spin_callee::BE_LIBRARY_GET_INTERFACE
                                                    .call(c, 0x001346f8)?;
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR
                                                    .call(c, 0x0013470c)?;
                                                c.lwc1(1, gp(c, -0x73ec));
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                c.mul_s(0, 0, 1);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.lwc1(1, at(c, V0, R_FLIP_SPEED));
                                                c.lwc1(2, at(c, V0, R_SPIN_SPEED));
                                                c.add_s(0, 0, 3);
                                                c.lwc1(4, gp(c, -0x73e8));
                                                c.max_s(1, 1, 2);
                                                c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                c.mul_s(0, 0, 1);
                                                c.mul_s(0, 0, 4);
                                                c.swc1(0, at(c, V1, 0x1c));
                                                break 'b_134c3c;
                                            }
                                        }
                                    }
                                    'b_13477c: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_13477c;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(0, at(c, S0, SPIN_TARGET));
                                            c.mtc1(0, 1);
                                            c.c_eq_s(0, 1);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_13477c;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, FLIP_TARGET));
                                                c.c_eq_s(0, 1);
                                                if c.fcc() {
                                                    c.g[S1][0] = 0x11f;
                                                    break 'b_134c3c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = 0x1;
                                                    break 'b_13477c;
                                                }
                                            }
                                        }
                                    }
                                    'b_1349d4: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_1349d4;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(at(c, S0, TURNED));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x2;
                                            if t {
                                                break 'b_134c3c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                call_ptr(
                                                    c,
                                                    spin_callee::ANIM_LAYER_ANIM_ENTRY,
                                                    0x0013479c,
                                                )?;
                                                c.g[V1][0] = 0x2;
                                                let t = c.g[V0][0] != c.g[V1][0];

                                                if t {
                                                    break 'b_134c3c;
                                                } else {
                                                    'b_1347d8: {
                                                        c.cycles += 6;
                                                        c.lwc1(1, at(c, S0, SPIN));
                                                        c.lwc1(0, at(c, S0, SPIN_TARGET));
                                                        c.c_eq_s(1, 0);
                                                        if !c.fcc() {
                                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                            break 'b_1347d8;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(1, at(c, S0, FLIP));
                                                            c.lwc1(0, at(c, S0, FLIP_TARGET));
                                                            c.c_eq_s(1, 0);
                                                            let t = c.fcc();

                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                                break 'b_1347d8;
                                                            }
                                                        }
                                                    }
                                                    'b_1347fc: {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = 0x2;
                                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                        call_ptr(c, spin_callee::ANIM_IS_LAYER_ANIM_FINISHED, 0x001347e4)?;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.lwc1(1, at(c, S0, FLIP_TARGET));
                                                            break 'b_1347fc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x11f;
                                                            let t = c.g[S1][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                                                break 'b_1347fc;
                                                            }
                                                        }
                                                    }
                                                    'b_13483c: {
                                                        'b_134838: {
                                                            'b_134830: {
                                                                c.cycles += 5;
                                                                c.lwc1(0, at(c, S0, FLIP));
                                                                c.c_le_s(0, 1);
                                                                if !c.fcc() {
                                                                    c.lwc1(
                                                                        1,
                                                                        at(c, S0, SPIN_TARGET),
                                                                    );
                                                                    break 'b_134830;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.c_lt_s(0, 1);
                                                                    c.mtc1(0, 22);
                                                                    let t = !c.fcc();
                                                                    c.lwc1(
                                                                        1,
                                                                        at(c, S0, SPIN_TARGET),
                                                                    );
                                                                    if t {
                                                                        break 'b_134838;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 22);
                                                                        c.lwc1(0, at(c, S0, SPIN));
                                                                        break 'b_13483c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            c.g[AT][0] = 0xffffffffbf800000;
                                                            c.mtc1(1, 22);
                                                            break 'b_134838;
                                                        }
                                                        c.cycles += 1;
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_13483c;
                                                    }
                                                    'b_134878: {
                                                        'b_134874: {
                                                            'b_13486c: {
                                                                c.cycles += 4;
                                                                c.c_le_s(0, 1);
                                                                let t = !c.fcc();
                                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                                if t {
                                                                    break 'b_13486c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.c_lt_s(0, 1);
                                                                    c.mtc1(0, 21);
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134874;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 21);
                                                                        c.g[V1][0] = c.lw(at(
                                                                            c, V0, R_STANCE,
                                                                        ));
                                                                        break 'b_134878;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            c.g[AT][0] = 0xffffffffbf800000;
                                                            c.mtc1(1, 21);
                                                            break 'b_134874;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = c.lw(at(c, V0, R_STANCE));
                                                        break 'b_134878;
                                                    }
                                                    'b_134958: {
                                                        'b_134950: {
                                                            'b_134944: {
                                                                'b_13491c: {
                                                                    'b_134904: {
                                                                        'b_1348e0: {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[V1][0] == 0u64;

                                                                            if t {
                                                                                break 'b_1348e0;
                                                                            } else {
                                                                                'b_1348b0: {
                                                                                    c.cycles += 5;
                                                                                    c.neg_s(20, 21);
                                                                                    c.mtc1(0, 21);
                                                                                    c.c_eq_s(
                                                                                        20, 21,
                                                                                    );
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_1348b0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.c_eq_s(
                                                                                            22, 21,
                                                                                        );
                                                                                        let t =
                                                                                            c.fcc();
                                                                                        c.mov_s(
                                                                                            12, 22,
                                                                                        );
                                                                                        if t {
                                                                                            break 'b_134958;
                                                                                        } else {
                                                                                            c.cycles += 3;
                                                                                            c.c_le_s(21, 22);

                                                                                            break 'b_134904;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 7;
                                                                                c.div_s(12, 22, 20);
                                                                                call_ptr(c, spin_callee::MATH_ATAN, 0x001348bc)?;
                                                                                c.c_lt_s(20, 21);
                                                                                let t = !c.fcc();
                                                                                c.mov_s(12, 0);
                                                                                if t {
                                                                                    break 'b_134958;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.c_lt_s(
                                                                                        21, 22,
                                                                                    );
                                                                                    let t = c.fcc();

                                                                                    if t {
                                                                                        break 'b_134944;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;

                                                                                        break 'b_134950;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.mtc1(0, 20);
                                                                        c.c_eq_s(21, 20);
                                                                        let t = !c.fcc();

                                                                        if t {
                                                                            break 'b_13491c;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.c_eq_s(22, 20);
                                                                            let t = c.fcc();
                                                                            c.mov_s(12, 22);
                                                                            if t {
                                                                                break 'b_134958;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.c_le_s(20, 22);
                                                                                break 'b_134904;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.lwc1(12, gp(c, -0x73e4));
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134958;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.lwc1(12, gp(c, -0x73e0));

                                                                        break 'b_134958;
                                                                    }
                                                                }
                                                                c.cycles += 7;
                                                                c.div_s(12, 22, 21);
                                                                call_ptr(
                                                                    c,
                                                                    spin_callee::MATH_ATAN,
                                                                    0x00134928,
                                                                )?;
                                                                c.c_lt_s(21, 20);
                                                                let t = !c.fcc();
                                                                c.mov_s(12, 0);
                                                                if t {
                                                                    break 'b_134958;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.c_lt_s(20, 22);
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134950;
                                                                    } else {
                                                                        break 'b_134944;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x73dc));
                                                            c.add_s(12, 12, 0);
                                                            break 'b_134958;
                                                        }
                                                        c.cycles += 2;
                                                        c.lwc1(0, gp(c, -0x73d8));
                                                        c.sub_s(12, 12, 0);
                                                        break 'b_134958;
                                                    }
                                                    c.cycles += 6;
                                                    c.lwc1(1, gp(c, -0x73d4));
                                                    c.abs_s(0, 12);
                                                    c.c_lt_s(1, 0);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x126;
                                                    if t {
                                                        break 'b_134c3c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gp(c, -0x73d0));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();

                                                        if t {
                                                            break 'b_134c3c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x73cc));
                                                            c.c_lt_s(0, 12);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x125;
                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(0, gp(c, -0x73c8));
                                                                c.c_lt_s(0, 12);
                                                                let t = c.fcc();

                                                                if t {
                                                                    break 'b_134c3c;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x73c4));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = c.fcc();
                                                                    c.g[S1][0] = 0x128;
                                                                    if t {
                                                                        break 'b_134c3c;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(0, gp(c, -0x73c0));
                                                                        c.c_lt_s(12, 0);
                                                                        if !c.fcc() {
                                                                            c.g[S1][0] = 0x127;
                                                                            break 'b_134c3c;
                                                                        } else {
                                                                            c.cycles += 2;

                                                                            break 'b_134c3c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    'b_134b80: {
                                        c.cycles += 2;
                                        if c.g[S3][0] == c.g[V0][0] {
                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                            break 'b_134b80;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_134b80;
                                            } else {
                                                'b_134a50: {
                                                    'b_134a4c: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_sub(0x10d));
                                                        c.g[V0][0] = (c.g[V1][0] < 0x1cu64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x460000;
                                                        if t {
                                                            break 'b_134a4c;
                                                        } else {
                                                            'b_134a44: {
                                                                'b_134a3c: {
                                                                    'b_134a34: {
                                                                        'b_134a2c: {
                                                                            'b_134a24: {
                                                                                'b_134a1c: {
                                                                                    'b_134a14: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7e60));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                        // The landing blend for the current pose: the original's jump table
                                                                                        // (.rodata 0x4581a0, indexed by `id - 0x10d`), read as it does.
                                                                                        c.g[A0][0] = c.lw(at(c, V1, 0));
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            as u32;
                                                                                        match t {
                                                                                            0x00134a0c => {
                                                                                                c.cycles += 2;
                                                                                                c.g[S1][0] = 0x132;
                                                                                                break 'b_134a50;
                                                                                            }
                                                                                            0x00134a14 => {
                                                                                                break 'b_134a14;
                                                                                            }
                                                                                            0x00134a1c => {
                                                                                                break 'b_134a1c;
                                                                                            }
                                                                                            0x00134a24 => {
                                                                                                break 'b_134a24;
                                                                                            }
                                                                                            0x00134a2c => {
                                                                                                break 'b_134a2c;
                                                                                            }
                                                                                            0x00134a34 => {
                                                                                                break 'b_134a34;
                                                                                            }
                                                                                            0x00134a3c => {
                                                                                                break 'b_134a3c;
                                                                                            }
                                                                                            0x00134a44 => {
                                                                                                break 'b_134a44;
                                                                                            }
                                                                                            0x00134a4c => {
                                                                                                break 'b_134a4c;
                                                                                            }
                                                                                            _ => {
                                                                                                c.jump_out(t, 0x00133308, 0x00134cb0);
                                                                                                return Ok(());
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 2;
                                                                                    c.g[S1][0] =
                                                                                        0x133;
                                                                                    break 'b_134a50;
                                                                                }
                                                                                c.cycles += 2;
                                                                                c.g[S1][0] = 0x134;
                                                                                break 'b_134a50;
                                                                            }
                                                                            c.cycles += 2;
                                                                            c.g[S1][0] = 0x135;
                                                                            break 'b_134a50;
                                                                        }
                                                                        c.cycles += 2;
                                                                        c.g[S1][0] = 0x136;
                                                                        break 'b_134a50;
                                                                    }
                                                                    c.cycles += 2;
                                                                    c.g[S1][0] = 0x137;
                                                                    break 'b_134a50;
                                                                }
                                                                c.cycles += 2;
                                                                c.g[S1][0] = 0x138;
                                                                break 'b_134a50;
                                                            }
                                                            c.cycles += 2;
                                                            c.g[S1][0] = 0x139;
                                                            break 'b_134a50;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[S1][0] = 0x11f;
                                                    break 'b_134a50;
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == c.g[S4][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_134c70;
                                                } else {
                                                    'b_134aec: {
                                                        c.cycles += 37;
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 21);
                                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                        call_ptr(c, spin_callee::ANIM_GET_CURRENT_ANIM_LENGTH, 0x00134a6c)?;
                                                        c.lwc1(1, at(c, S0, SPIN_TARGET));
                                                        c.mov_s(22, 0);
                                                        c.lwc1(5, at(c, S0, SPIN));
                                                        c.lwc1(2, at(c, S0, FLIP_TARGET));
                                                        c.lwc1(4, at(c, S0, FLIP));
                                                        c.sub_s(5, 5, 1);
                                                        c.lwc1(0, gp(c, -0x73bc));
                                                        c.abs_s(5, 5);
                                                        c.sub_s(4, 4, 2);
                                                        c.lwc1(9, gp(c, -0x73b8));
                                                        c.mul_s(1, 5, 0);
                                                        c.abs_s(4, 4);
                                                        c.mul_s(0, 4, 0);
                                                        c.lwc1(6, gp(c, -0x73b4));
                                                        c.lwc1(8, at(c, S0, SPIN_RATE));
                                                        c.sub_s(3, 21, 1);
                                                        c.lwc1(7, at(c, S0, FLIP_RATE));
                                                        c.sub_s(2, 21, 0);
                                                        c.mul_s(0, 0, 9);
                                                        c.mul_s(3, 3, 6);
                                                        c.mul_s(2, 2, 6);
                                                        c.mul_s(1, 1, 9);
                                                        c.add_s(0, 0, 2);
                                                        c.add_s(1, 1, 3);
                                                        c.max_s(0, 0, 7);
                                                        c.max_s(1, 1, 8);
                                                        c.div_s(4, 4, 0);
                                                        c.div_s(20, 5, 1);
                                                        c.c_lt_s(20, 4);
                                                        if c.fcc() {
                                                            c.mov_s(20, 4);
                                                            break 'b_134aec;
                                                        } else {
                                                            break 'b_134aec;
                                                        }
                                                    }
                                                    'b_134b0c: {
                                                        c.cycles += 6;
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        c.mtc1(0, 1);
                                                        c.lwc1(0, at(c, V0, R_2EC));
                                                        c.c_lt_s(1, 0);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_134b0c;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.lwc1(0, gp(c, -0x73b0));
                                                            c.mul_s(20, 20, 0);
                                                            break 'b_134b0c;
                                                        }
                                                    }
                                                    'b_134b68: {
                                                        c.cycles += 22;
                                                        spin_callee::BE_LIBRARY_GET_INSTANCE
                                                            .call(c, 0x00134b14)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0x3;
                                                        spin_callee::BE_LIBRARY_GET_INTERFACE
                                                            .call(c, 0x00134b20)?;
                                                        c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                                        c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                                        spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR
                                                            .call(c, 0x00134b34)?;
                                                        c.lwc1(1, gp(c, -0x73ac));
                                                        c.lwc1(2, gp(c, -0x73a8));
                                                        c.mul_s(0, 0, 1);
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 1);
                                                        c.add_s(0, 0, 21);
                                                        c.div_s(0, 21, 0);
                                                        c.mul_s(20, 20, 0);
                                                        c.c_lt_s(2, 20);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        if t {
                                                            break 'b_134b68;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.div_s(1, 22, 20);
                                                            break 'b_134b68;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[AT][0] = 0x40000000;
                                                    c.mtc1(1, 0);
                                                    c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                    c.min_s(0, 1, 0);
                                                    c.swc1(0, at(c, V1, 0x1c));
                                                    break 'b_134c3c;
                                                }
                                            }
                                        }
                                    }
                                    'b_134ba8: {
                                        'b_134ba4: {
                                            c.cycles += 6;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = 0x1;
                                            c.g[V1][0] = c.lw(at(c, V0, R_ANIM_STATE));
                                            c.g[V1][0] = c.lw(at(c, V1, 0xac));
                                            let t = c.g[V1][0] == c.g[A0][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            if t {
                                                break 'b_134ba4;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[V1][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_134ba8;
                                                } else {
                                                    break 'b_134ba4;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = 0x1;
                                        break 'b_134ba8;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    c.g[A1][0] = 0x131;
                                    if t {
                                        break 'b_134c3c;
                                    } else {
                                        'b_134c24: {
                                            'b_134c20: {
                                                c.cycles += 9;
                                                c.g[V0][0] = c.lw(at(c, A2, R_ANIM_STATE));
                                                c.g[A0][0] = c.lw(at(c, A2, R_ANIM));
                                                c.lwc1(1, at(c, V0, 0xa0));
                                                c.lwc1(0, at(c, V0, 0x98));
                                                c.sub_s(20, 0, 1);
                                                call_ptr(
                                                    c,
                                                    spin_callee::ANIM_GET_CURRENT_ANIM_LENGTH,
                                                    0x00134bc8,
                                                )?;
                                                c.g[V0][0] = 0x11f;
                                                let t = c.g[S1][0] != c.g[V0][0];
                                                c.mov_s(1, 0);
                                                if t {
                                                    break 'b_134c20;
                                                } else {
                                                    c.cycles += 4;
                                                    c.c_le_s(20, 1);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x131;
                                                    if t {
                                                        break 'b_134c24;
                                                    } else {
                                                        'b_134c04: {
                                                            c.cycles += 7;
                                                            c.lwc1(0, gp(c, -0x73a4));
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 2);
                                                            c.c_lt_s(0, 20);
                                                            let t = !c.fcc();
                                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                            if t {
                                                                break 'b_134c04;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.div_s(2, 1, 20);
                                                                break 'b_134c04;
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[S1][0] = 0x131;
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 0);
                                                        c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                        c.min_s(0, 2, 0);
                                                        c.swc1(0, at(c, V1, 0x1c));
                                                        break 'b_134c3c;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = 0x131;
                                            break 'b_134c24;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_134c3c;
                                        } else {
                                            c.cycles += 4;
                                            c.c_lt_s(1, 20);
                                            if c.fcc() {
                                                c.g[S1][0] = 0x11f;
                                                break 'b_134c3c;
                                            } else {
                                                break 'b_134c3c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[S4][0];
                        c.g[A1][0] = c.g[S1][0];
                        if t {
                            break 'b_134c70;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                            call_ptr(c, spin_callee::ANIM_PLAY_ANIM, 0x00134c5c)?;
                            c.g[V1][0] = c.lw(at(c, S0, RIDER));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[V0][0] = c.lw(at(c, V1, R_ANIM));
                            c.swc1(0, at(c, V0, 0x1c));
                            break 'b_134c70;
                        }
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    break 'b_134c74;
                }
            }
        }
        c.cycles += 15;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 27, 0xa8);
        frame.restore_fpr(c, 26, 0xa0);
        frame.restore_fpr(c, 25, 0x98);
        frame.restore_fpr(c, 24, 0x90);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002e4f50 SnowBox_Update

/// `SnowBox` fields read and written by [`snow_box_update`] (the box of
/// falling snow/rain flakes that wraps around the camera), and the weather
/// record it is fed with.
mod snow_update {
    /// Weather mode: 0 snow (record `f[0..2]`, colour `c0`), 1 the second
    /// kind (record `f[2..4]`, colour `c1`), anything else keeps the
    /// current parameters.
    pub const MODE: u32 = 0x04;
    /// Number of flakes drawn (`(int)(density * k)`).
    pub const FLAKE_COUNT: u32 = 0x08;
    /// Side length of the box (each position component wraps into
    /// `(-size/2, size/2]`).
    pub const SIZE: u32 = 0x0c;
    /// Flake speed (scale of a new drift velocity).
    pub const SPEED: u32 = 0x10;
    /// Copied from the record's `f[4]` in mode 0.
    pub const PARAM_14: u32 = 0x14;
    /// First component of the colour (`c0.x`/`c1.x`).
    pub const COLOR_X: u32 = 0x18;
    /// Wind factor of this box (times the record's `f[7]`).
    pub const WIND_SCALE: u32 = 0x1c;
    /// Box offset (x, y, z, w), wrapped into the box.
    pub const POS: u32 = 0x20;
    /// Drift velocity being blended toward.
    pub const VEL: u32 = 0x30;
    /// Previous drift velocity (blended from).
    pub const PREV_VEL: u32 = 0x40;
    /// Seconds left until a new drift velocity is picked.
    pub const TIMER: u32 = 0x90;
    /// Length of the current drift period (seconds).
    pub const PERIOD: u32 = 0x94;
    /// Remaining colour components (`c.y`, `c.z`, `c.w`).
    pub const COLOR_YZW: u32 = 0xa4;
    /// Non-zero: snap: the previous velocity becomes the new one (cleared).
    pub const SNAP: u32 = 0xb0;

    /// The weather record (`sWakeRec_5DA0`): `float f[8]; Vec4 c0, c1;`.
    pub const REC_DENSITY0: u32 = 0x00;
    pub const REC_SPEED0: u32 = 0x04;
    pub const REC_DENSITY1: u32 = 0x08;
    pub const REC_SPEED1: u32 = 0x0c;
    pub const REC_F4: u32 = 0x10;
    /// Drift heading (scaled into the 512-entry sine table).
    pub const REC_HEADING: u32 = 0x14;
    /// How much of the random part goes into a new velocity (0..1).
    pub const REC_RANDOMNESS: u32 = 0x18;
    pub const REC_WIND: u32 = 0x1c;
    pub const REC_C0: u32 = 0x20;
    pub const REC_C1: u32 = 0x30;

    /// gp+0x2a74: the frame-rate object (its int rate at +0x10).
    pub const GP_RATE_OBJ: i32 = 0x2a74;
    pub const RATE: u32 = 0x10;
    /// gp+0xa0c: the snow's random seed (a float in [1, 2) as bits).
    pub const GP_SEED: i32 = 0xa0c;
    /// gp-0x3a18, gp-0x3a14: the heading-to-table-index factors.
    pub const GP_HEADING_K1: i32 = -0x3a18;
    pub const GP_HEADING_K2: i32 = -0x3a14;
    /// The global wind vector.
    pub const WIND: u32 = 0x004f_f160;
    /// The 512-entry sine table (cos at +0x80 entries).
    pub const SIN_TABLE: u32 = 0x0050_4fb8;

    /// The seed's linear congruential step.
    pub const LCG_MUL: u32 = 0x0001_8fcd;
    pub const LCG_ADD: u32 = 0x00e9_507c;
    pub const MANTISSA: u32 = 0x007f_ffff;
    pub const ONE_BITS: u32 = 0x3f80_0000;
}

/// `void SnowBox_Update(SnowBox* box, const sWakeRec_5DA0* rec)` (0x2e4f50):
/// the per-frame update of the weather flake box.
///
/// 1. Mode 0/1: take density (flake count), speed and colour from the
///    record (the second set for mode 1).
/// 2. Count the drift timer down by one tick (`1 / rate`); when it runs
///    out, pick a new drift velocity (a heading from the record plus a
///    random part, four steps of the seed LCG) and a new period in [1, 5).
/// 3. Move the box offset by the velocity blended from the previous one
///    (`lerp(prev, vel, 1 - timer/period)`) and the wind, both times the
///    tick, then wrap it into the box (as `SnowBox_WrapOffset`).
pub fn snow_box_update(c: &mut Cpu) {
    use crate::game::visualfx::boardwakefx::{
        copy_q, fconst, fr, frac_times, gp, ldf, p, setf, vadd_2,
    };
    use snow_update::*;

    c.cycles += 5;
    c.g[T2][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
    let this = r32(c, T2);
    let rec = r32(c, A1);
    let mode = c.lw(p(this, MODE));
    c.g[V1][0] = mode;
    c.g[T3][0] = c.g[A1][0];

    // Step 1: the weather parameters of the mode.
    if mode == 0 {
        c.cycles += 20;
        c.lwc1(1, p(rec, REC_C0));
        let k = fconst(c, 3, 0x43fa_0000); // 500
        c.swc1(1, p(this, COLOR_X));
        for i in 0..3u32 {
            c.lwc1(i as usize, p(rec, REC_C0 + 4 + 4 * i));
            c.swc1(i as usize, p(this, COLOR_YZW + 4 * i));
        }
        let n = ldf(c, 0, p(rec, REC_DENSITY0)) * k;
        setf(c, 0, n);
        c.cvt_w_s(1, 0);
        c.swc1(1, p(this, FLAKE_COUNT));
        let s = ldf(c, 1, p(rec, REC_SPEED0)) * k;
        setf(c, 1, s);
        c.swc1(1, p(this, SPEED));
        c.lwc1(0, p(rec, REC_F4));
        c.swc1(0, p(this, PARAM_14));
        c.cycles += 1;
        c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
        if mode == 1 {
            c.cycles += 19;
            c.lwc1(0, p(rec, REC_C1));
            let k_count = fconst(c, 3, 0x40c0_0000); // 6
            c.swc1(0, p(this, COLOR_X));
            let k_speed = fconst(c, 4, 0x437a_0000); // 250
            for i in 0..3u32 {
                c.lwc1(i as usize, p(rec, REC_C1 + 4 + 4 * i));
                c.swc1(i as usize, p(this, COLOR_YZW + 4 * i));
            }
            let n = ldf(c, 0, p(rec, REC_DENSITY1)) * k_count;
            setf(c, 0, n);
            c.cvt_w_s(1, 0);
            c.swc1(1, p(this, FLAKE_COUNT));
            let s = ldf(c, 1, p(rec, REC_SPEED1)) * k_speed;
            setf(c, 1, s);
            c.swc1(1, p(this, SPEED));
            c.cycles += 1;
            c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
        } else {
            c.cycles += 2;
        }
    }

    // Step 2: the drift timer (f11 = 1, f12 = tick, f13 = 0 stay live).
    c.cycles += 12;
    let one = fconst(c, 11, 0x3f80_0000);
    let rate_obj = r32(c, V0);
    c.lwc1(1, p(rate_obj, RATE));
    c.cvt_s_w(1, 1);
    let timer = ldf(c, 0, p(this, TIMER));
    c.f[13] = 0;
    let dt = one.div(fr(c, 1));
    setf(c, 12, dt);
    setf(c, 0, timer - dt);
    c.c_lt_s(0, 13);
    let expired = c.fcc();
    c.swc1(0, p(this, TIMER));
    if expired {
        snow_box_new_drift(c, this, rec);
    }

    // Step 3: move by the blended velocity and the wind, then wrap.
    c.cycles += 66;
    let period = ldf(c, 2, p(this, PERIOD));
    c.g[A0][0] = sx(WIND);
    let timer = ldf(c, 1, p(this, TIMER));
    let one = fconst(c, 0, 0x3f80_0000);
    let t = timer.div(period);
    setf(c, 1, t);
    fconst(c, 3, 0x3f00_0000); // 0.5
    setf(c, 0, one - t);
    let sp = sp(c);
    snow_scale_q(c, 0, p(this, VEL), p(sp, 0x30), p(sp, 0x20)); // vel * (1 - t)
    snow_scale_q(c, 1, p(this, PREV_VEL), p(sp, 0x40), p(sp, 0x30)); // prev * t
    vadd_2(c, p(sp, 0x20), p(sp, 0x30), p(sp, 0x40));
    copy_q(c, V0, p(sp, 0x40), p(sp, 0x10));
    snow_scale_q(c, 12, p(sp, 0x10), p(sp, 0x40), sp); // * tick
    vadd_2(c, p(this, POS), sp, p(this, POS));
    let wind = ldf(c, 0, p(this, WIND_SCALE)) * ldf(c, 1, p(rec, REC_WIND));
    setf(c, 0, wind);
    let wind = wind * fr(c, 12);
    setf(c, 0, wind);
    snow_scale_q(c, 0, WIND, p(sp, 0x10), sp); // wind * scale * tick
    vadd_2(c, p(this, POS), sp, p(this, POS));

    // The wrap: x in f4, y in f3, z in f5; results in f4, f6, f7.
    c.lwc1(2, p(this, SIZE));
    c.lwc1(4, p(this, POS));
    c.mul_s(0, 2, 3); // f0 = h = size * 0.5
    c.lwc1(6, p(this, POS + 4));
    c.lwc1(7, p(this, POS + 8));
    c.mov_s(3, 6);
    c.c_lt_s(0, 4); // h < x ?
    let x_above = c.fcc();
    c.mov_s(5, 7);
    'x: {
        'x_tail: {
            if x_above {
                c.cycles += 9;
                let (v, s) = (fr(c, 4), fr(c, 2));
                let r = frac_times(c, v, s);
                setf(c, 4, r - s);
                break 'x_tail;
            }
            c.cycles += 5;
            c.neg_s(0, 0); // f0 = -h
            c.c_lt_s(4, 0); // x < -h ?
            if !c.fcc() {
                c.lwc1(0, p(this, SIZE));
                break 'x;
            }
            c.cycles += 8;
            let (v, s) = (fr(c, 4), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 4, r + s);
        }
        c.cycles += 1;
        c.lwc1(0, p(this, SIZE));
    }
    'y: {
        'y_below: {
            c.cycles += 8;
            fconst(c, 1, 0x3f00_0000);
            c.mov_s(2, 0); // f2 = size
            c.mul_s(0, 0, 1); // f0 = h
            c.c_lt_s(0, 3); // h < y ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'y_below;
            }
            c.cycles += 9;
            let (v, s) = (fr(c, 3), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 6, r - s);
            break 'y;
        }
        c.cycles += 3;
        c.c_lt_s(3, 0); // y < -h ?
        if !c.fcc() {
            break 'y;
        }
        c.cycles += 8;
        let (v, s) = (fr(c, 3), fr(c, 2));
        let r = frac_times(c, v, s);
        setf(c, 6, r + s);
    }
    'z: {
        'z_below: {
            c.cycles += 7;
            fconst(c, 0, 0x3f00_0000);
            c.mul_s(0, 2, 0); // f0 = h
            c.c_lt_s(0, 5); // h < z ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'z_below;
            }
            c.cycles += 9;
            let (v, s) = (fr(c, 5), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 7, r - s);
            break 'z;
        }
        c.cycles += 3;
        c.c_lt_s(5, 0); // z < -h ?
        if !c.fcc() {
            break 'z;
        }
        c.cycles += 8;
        let (v, s) = (fr(c, 5), fr(c, 2));
        let r = frac_times(c, v, s);
        setf(c, 7, r + s);
    }
    c.cycles += 10;
    fconst(c, 0, 0x3f80_0000); // w = 1
    c.swc1(4, sp);
    c.swc1(6, p(sp, 4));
    c.swc1(7, p(sp, 8));
    c.swc1(0, p(sp, 0xc));
    copy_q(c, V0, sp, p(this, POS));
    c.g[SP][0] = sx(sp.wrapping_add(0x50));
    ret(c);
}

/// `mfc1 v1, f; lqc2 vf4, src; qmtc2 v1, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, tmp; lq v0, tmp; sq v0, out`: `out = src * f` through the
/// stack slot `tmp`.
#[inline(always)]
fn snow_scale_q(c: &mut Cpu, f: usize, src: u32, tmp: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(V1, f);
    c.lqc2(4, src);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(V0, tmp);
    c.sq(V0, out);
}

/// One step of the snow seed: `((s * 0x18fcd + 0xe9507c) & 0x7fffff) |
/// 0x3f800000` (a float in [1, 2)); the `mult` leaves LO/HI.
#[inline(always)]
fn snow_lcg(c: &mut Cpu, s: u32) -> u32 {
    use snow_update::*;
    let lo = crate::abi::mult(c, s, LCG_MUL);
    lo.wrapping_add(LCG_ADD) & MANTISSA | ONE_BITS
}

/// The expired-timer block of [`snow_box_update`]: the previous velocity
/// becomes the current one, a new velocity is picked (`speed` along the
/// record's heading, mixed with a random vector in the speed cube by the
/// record's randomness) with a new period in [1, 5), and with `SNAP` set the
/// blend starts at the new velocity.
fn snow_box_new_drift(c: &mut Cpu, this: u32, rec: u32) {
    use crate::game::visualfx::boardwakefx::{copy_q, fr, gp, p, setf};
    use crate::ps2f::FpuF32;
    use snow_update::*;

    c.cycles += 93;
    let seed_at = gp(c, GP_SEED);
    let seed = c.lw(seed_at) as u32;
    copy_q(c, V1, p(this, VEL), p(this, PREV_VEL));
    let speed = FpuF32(c.lw(p(this, SPEED)) as u32);
    let heading = FpuF32(c.lw(p(rec, REC_HEADING)) as u32);
    let k1 = FpuF32(c.lw(gp(c, GP_HEADING_K1)) as u32);
    let k2 = FpuF32(c.lw(gp(c, GP_HEADING_K2)) as u32);
    let mix = FpuF32(c.lw(p(rec, REC_RANDOMNESS)) as u32);
    let one = fr(c, 11);

    // Four LCG steps, each stored back as the seed.
    let r1 = snow_lcg(c, seed);
    c.sw(seed_at, r1 as u64);
    let r2 = snow_lcg(c, r1);
    c.sw(seed_at, r2 as u64);
    let r3 = snow_lcg(c, r2);
    c.sw(seed_at, r3 as u64);
    let r4 = snow_lcg(c, r3);

    // The heading's table index (cvt.w through f1 into v0).
    setf(c, 0, heading * k1 * k2);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    let idx = r32(c, V0) & 0x1ff;
    let sin = FpuF32(c.lw(SIN_TABLE + idx * 4) as u32);
    let cos = FpuF32(c.lw(SIN_TABLE + (idx + 0x80) * 4) as u32);

    let low = -speed;
    let span = speed - low;
    let u1 = FpuF32(r1) - one;
    let u2 = FpuF32(r2) - one;
    let u3 = FpuF32(r3) - one;
    let x0 = low + span * u1;
    let y0 = low + span * u2;
    let span_z = span * u3;
    let z0 = low + span_z;
    let sin_s = speed * sin;
    let cos_s = speed * cos;
    let keep = one - mix;
    let mix_x = mix * x0;
    let mix_y = mix * y0;
    let vx = cos_s * keep + mix_x;
    let vy = sin_s * keep + mix_y;
    let vz = mix * z0;

    c.g[V0][0] = c.lw(p(this, SNAP));
    let sp = sp(c);
    c.swc1(13, p(sp, 0x1c));
    c.sw(p(sp, 4), sin_s.0 as u64);
    c.sw(sp, cos_s.0 as u64);
    c.swc1(13, p(sp, 8));
    c.swc1(13, p(sp, 0xc));
    c.sw(p(sp, 0x14), vy.0 as u64);
    c.sw(p(sp, 0x18), vz.0 as u64);
    c.sw(p(sp, 0x10), vx.0 as u64);
    copy_q(c, V1, p(sp, 0x10), p(this, VEL));
    c.sw(seed_at, r4 as u64);
    let four = FpuF32(0x4080_0000);
    let period = (FpuF32(r4) - one) * four + one;
    c.sw(p(this, TIMER), period.0 as u64);
    c.sw(p(this, PERIOD), period.0 as u64);

    // The registers the original leaves.
    c.g[AT][0] = 0x4080_0000;
    c.g[A0][0] = r4 as u64;
    c.g[A1][0] = sx(SIN_TABLE);
    c.g[A2][0] = LCG_ADD as u64;
    c.g[A3][0] = LCG_MUL as u64;
    c.g[T0][0] = MANTISSA as u64;
    c.g[T1][0] = ONE_BITS as u64;
    setf(c, 0, period);
    setf(c, 1, mix_y);
    setf(c, 2, u3);
    setf(c, 3, cos);
    setf(c, 4, mix_x);
    setf(c, 5, vx);
    setf(c, 6, span_z);
    setf(c, 7, vz);
    setf(c, 8, z0);
    setf(c, 9, keep);
    setf(c, 10, four);

    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.sq(V1, p(this, PREV_VEL));
        c.sw(p(this, SNAP), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- 0x00133308 RiderSpinSub_HandleInput -------------------------------

    mod spin_input {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x5_0000;
        const PAD: u32 = DATA + 0x5_0100;
        const RIDER: u32 = DATA + 0x5_1000;
        const STATE: u32 = DATA + 0x5_2000;
        const CTRL: u32 = DATA + 0x5_3000;
        const CTRL_VT: u32 = DATA + 0x5_3100;
        const SEQ_LISTS: u32 = DATA + 0x5_3200;
        const SEQ: u32 = DATA + 0x5_3300;
        const ANIM_STATE: u32 = DATA + 0x5_4000;
        const METRIX: u32 = DATA + 0x5_5000;
        const OVERLAPS: u32 = DATA + 0x5_6000;
        const BE_LIBRARY: u32 = DATA + 0x5_7000;
        const BE_OBJ: u32 = DATA + 0x5_7100;
        const BE_VT: u32 = DATA + 0x5_7200;
        const WORLD: u32 = DATA + 0x5_8000;
        const WORLD_84: u32 = DATA + 0x5_8100;
        const WORLD_TABLE: u32 = DATA + 0x5_8200;
        /// The static animation table (28-byte entries; `+0` the entry kind
        /// `AnimController_GetLayerAnimEntry` returns).
        const ANIM_TABLE: u32 = 0x0044_6990;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// Fake methods: the behaviour interface getter, and the controller's
        /// "current animation kind" (answers 0x207: `PlayAnim` and
        /// `GetCurrentAnimLength` stop there).
        const FAKE_BE: u32 = FAKE;
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;

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

        /// A 6-bit signed stick field: centred half of the time.
        fn axis(r: &mut Rng) -> u32 {
            if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() & 0x3f
            }
        }

        /// An angle-like float: 0, a half / whole turn, or random.
        fn angle(r: &mut Rng) -> u32 {
            match r.next_u32() % 6 {
                0 | 1 => 0,
                2 => f(std::f32::consts::PI),
                3 => f(-std::f32::consts::PI),
                4 => f(2.0 * std::f32::consts::PI),
                _ => r.f32(7.0),
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_3308, 6);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Runtime-computed globals past the image (gp+0x1ba0..): half
            // turn, epsilon, lean limits, gains, full turns, clamp.
            for (off, v) in [
                (0x1ba0, std::f32::consts::PI),
                (0x1ba4, 0.01),
                (0x1ba8, 0.6),
                (0x1bac, 0.5),
                (0x1bb0, 0.3),
                (0x1bb4, 2.0 * std::f32::consts::PI),
                (0x1bb8, 2.0 * std::f32::consts::PI),
                (0x1bbc, 1.0),
            ] {
                w(c, GAME_GP + off, f(v));
            }
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
            // The world chain `Rider_TryAirReverse` reads (entry 0: zero).
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD_84);
            w(c, WORLD_84 + 0x44, WORLD_TABLE);

            // The pad: never the reset bit (12); the other buttons, the
            // trick byte and the axes at random, centred often. Every third
            // seed has no lean input at all (and no lean): the animation
            // choice at the end runs only then.
            let calm = seed % 3 == 0;
            let buttons = (r.next_u32() & !0x3f00_1000) | axis(r) << 24;
            let mut sticks = axis(r) | axis(r) << 6 | axis(r) << 12 | (r.next_u32() & 3) << 18;
            sticks |= r.next_u32() & 0xfff0_0000;
            if calm {
                sticks &= !0x000f_ffc0;
            }
            w(c, PAD, buttons);
            w(c, PAD + 4, sticks);

            // The spin sub-state.
            w(c, THIS + 0x0, r.next_u32() % 5); // MODE
            w(c, THIS + 0x4, 6); // trick state: `UpdateTrick` only reads the layer
            w(c, THIS + 0x8, r.next_u32());
            w(c, THIS + 0xc, r.next_u32() % 5); // PHASE
            let spin_target = angle(r);
            let flip_target = angle(r);
            w(c, THIS + 0x10, flip_target);
            w(c, THIS + 0x14, spin_target);
            w(
                c,
                THIS + 0x18,
                if r.next_u32() % 2 == 0 {
                    flip_target
                } else {
                    angle(r)
                },
            );
            w(
                c,
                THIS + 0x1c,
                if r.next_u32() % 2 == 0 {
                    spin_target
                } else {
                    angle(r)
                },
            );
            for off in [0x20, 0x24, 0x30, 0x34] {
                w(c, THIS + off, r.f32(10.0));
            }
            let (lean_x, lean_y) = if calm { (0, 0) } else { (angle(r), angle(r)) };
            w(c, THIS + 0x28, lean_x);
            w(c, THIS + 0x2c, lean_y);
            w(c, THIS + 0x38, r.f32(8.0));
            w(c, THIS + 0x3c, r.f32(8.0));
            w(c, THIS + 0x44, r.next_u32() % 2);
            for off in [0x48, 0x4c, 0x50, 0x54] {
                w(
                    c,
                    THIS + off,
                    if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
                );
            }
            w(c, THIS + 0x58, RIDER);

            // The rider: in the air (main state 2: no rail or handplant
            // search), no boost (`Rider_AddBoost` returns at once).
            w(c, RIDER + 0x77c, STATE);
            w(c, STATE + 0xde0, 2);
            w(c, RIDER + 0x784, CTRL);
            w(c, RIDER + 0x788, ANIM_STATE);
            w(c, RIDER + 0x790, METRIX);
            w(c, RIDER + 0x860, OVERLAPS);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x2f4, r.next_u32() % 10);
            w(c, RIDER + 0x86c, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x2dc, r.f32(5.0));
            w(c, RIDER + 0x2e0, r.f32(5.0));
            w(
                c,
                RIDER + 0x2ec,
                if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) },
            );
            w(c, RIDER + 0x320, r.next_u32() % 2);
            w(c, RIDER + 0x324, r.next_u32() % 2);
            w(
                c,
                RIDER + 0x28c,
                if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
            );
            w(
                c,
                RIDER + 0x298,
                if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
            );
            // `Rider_TryAirReverse` (anim kinds 1 and 3) gives up: the
            // rider's up vector points against the reference.
            put_vec(c, RIDER + 0x180, [f(1.0), 0, 0, 0]);
            put_vec(c, ANIM_STATE + 0x20, [f(-1.0), 0, 0, 0]);
            w(c, ANIM_STATE + 0xac, r.next_u32() % 4);
            w(c, ANIM_STATE + 0x98, r.f32(2.0));
            w(c, ANIM_STATE + 0xa0, r.f32(2.0));
            // The metrix: zeroed (no trick to announce); the spin flag.
            w(c, METRIX + 0x14, r.f32(1.0));

            // The animation controller: layer 2 plays `id` (spin poses
            // 0x10d..0x128, 0x11f, 0x120, 0x131, tweaks, "none"), whose
            // table kind is a trick (0x12..0x14), 2, 9 or other.
            const IDS: [u32; 6] = [0x11f, 0x120, 0x131, 0x129, 0x1b6, 0x200];
            let id = if r.next_u32() % 2 == 0 {
                0x10d + r.next_u32() % 0x1c
            } else {
                IDS[(r.next_u32() % 6) as usize]
            };
            const KINDS: [u32; 7] = [0x12, 0x13, 0x14, 2, 2, 9, 0];
            w(
                c,
                ANIM_TABLE + id * 0x1c,
                KINDS[(r.next_u32() % 7) as usize],
            );
            w(c, CTRL + 8, id);
            w(c, CTRL + 0x1c, f(1.0));
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            w(c, SEQ_LISTS + 0x14, SEQ);
            w(c, SEQ + 0xc0, r.next_u32() % 2);

            // Rare paths made common (on top of the random state, all with
            // no lean and no trick on the layer):
            match seed % 9 {
                // The spin settles this tick (phase 1 → 2, stick centred,
                // rotation at its targets): the landing-blend jump table on
                // a spin pose (or the default).
                3 => {
                    w(c, THIS + 0xc, 1);
                    w(c, PAD, buttons & !0x3f00_0000);
                    w(c, PAD + 4, sticks & !0x3f);
                    w(c, THIS + 0x18, flip_target);
                    w(c, THIS + 0x1c, spin_target);
                    w(c, THIS + 0x54, f(10.0));
                    let id = if seed % 2 == 0 {
                        0x10d + r.next_u32() % 0x1c
                    } else {
                        0x200
                    };
                    w(c, CTRL + 8, id);
                    w(c, ANIM_TABLE + id * 0x1c, if seed % 4 == 1 { 0 } else { 2 });
                }
                // A further half turn queued in phase 1 with the layer on a
                // kind-2 entry: the flip-pose choice.
                6 => {
                    w(c, THIS + 0xc, 1);
                    w(c, THIS + 0x44, 1);
                    let id = 0x10d + r.next_u32() % 0x1c;
                    w(c, CTRL + 8, id);
                    w(c, ANIM_TABLE + id * 0x1c, 2);
                }
                _ => {}
            }

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            // The generated body (no override slot yet) dispatches on `pc`:
            // enter at the start.
            c.pc = 0x0013_3308;
        }

        /// `RiderSpinSub_HandleInput`: every hold mode and spin phase,
        /// centred and pushed stick axes, targets reached or not, with and
        /// without a trick on the layer, the animation choices (including
        /// the landing-blend jump table) and fake callees that unwind.
        #[test]
        fn rider_spin_sub_handle_input_matches() {
            check(
                ssx3_ee::s01::f_00133308__generated,
                rider_spin_sub_handle_input,
                300,
                &setup,
            );
        }
    }

    // -- 0x002e4f50 SnowBox_Update ------------------------------------------

    /// `SnowBox_Update` moves the weather flake box: every mode (0, 1,
    /// other), the drift timer running out or not, the snap flag, and each
    /// position component ending inside, above or below the box (wrap).
    #[test]
    fn snow_box_update_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use snow_update::*;

        const BOX: u32 = DATA;
        const REC: u32 = DATA + 0x100;
        const RATE_OBJ: u32 = DATA + 0x200;
        let f = |x: f32| x.to_bits();
        let gp = |o: i32| GAME_GP.wrapping_add(o as u32);

        check(ssx3_ee::s09::f_002e4f50__generated, snow_box_update, 300, &|c, seed| {
            let mut r = Rng(0x002e_4f50 ^ (seed as u64) << 20 ^ 0x9e37_79b9);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x002e_4f50;

            // Globals: the frame rate, the seed, the heading factors, the
            // wind and the sine table.
            c.bus.write32(gp(GP_RATE_OBJ), RATE_OBJ);
            c.bus.write32(RATE_OBJ + RATE, if seed % 2 == 0 { 60 } else { 30 });
            c.bus.write32(gp(GP_SEED), r.next_u32());
            c.bus.write32(gp(GP_HEADING_K1), f(81.487_33));
            c.bus.write32(gp(GP_HEADING_K2), f(1.0 + (r.next_u32() % 4) as f32));
            for k in 0..4 {
                c.bus.write32(WIND + 4 * k, r.f32(2.0));
            }
            for i in 0..0x280 {
                c.bus.write32(SIN_TABLE + 4 * i, r.f32(1.0));
            }

            // The weather record.
            c.bus.write32(REC + REC_DENSITY0, f((r.next_u32() % 1000) as f32 * 0.002));
            c.bus.write32(REC + REC_SPEED0, f((r.next_u32() % 100) as f32 * 0.0002));
            c.bus.write32(REC + REC_DENSITY1, f((r.next_u32() % 1000) as f32 * 0.3));
            c.bus.write32(REC + REC_SPEED1, f((r.next_u32() % 100) as f32 * 0.0004));
            c.bus.write32(REC + REC_F4, r.f32(10.0));
            c.bus.write32(REC + REC_HEADING, r.f32(6.0));
            c.bus.write32(REC + REC_RANDOMNESS, f((r.next_u32() % 101) as f32 * 0.01));
            c.bus.write32(REC + REC_WIND, r.f32(1.0));
            for k in 0..8 {
                c.bus.write32(REC + REC_C0 + 4 * k, r.f32(255.0));
            }

            // The box: mode by seed % 3, timer expiring on (seed / 3) % 2,
            // snap on (seed / 6) % 2.
            let mode = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => [2, 7, 0xffff_ffff][(seed / 3 % 3) as usize],
            };
            c.bus.write32(BOX + MODE, mode);
            c.bus.write32(BOX + SPEED, f((r.next_u32() % 100) as f32 * 0.1));
            c.bus.write32(BOX + WIND_SCALE, r.f32(1.0));
            let timer = if seed / 3 % 2 == 0 { 0.001 } else { 0.5 + (r.next_u32() % 4) as f32 };
            c.bus.write32(BOX + TIMER, f(timer));
            c.bus.write32(BOX + PERIOD, f(1.0 + (r.next_u32() % 4) as f32));
            c.bus.write32(BOX + SNAP, if seed / 6 % 2 == 0 { 0 } else { r.next_u32() | 1 });
            for k in 0..4 {
                c.bus.write32(BOX + VEL + 4 * k, r.f32(5.0));
                c.bus.write32(BOX + PREV_VEL + 4 * k, r.f32(5.0));
            }
            // One position per component with its own wrap case (inside,
            // above +h, below -h); the move per tick stays well under 1.
            let size = 2.0 + (r.next_u32() % 200) as f32 * 0.25;
            let h = size * 0.5;
            c.bus.write32(BOX + SIZE, f(size));
            for k in 0..3u32 {
                let v = match r.next_u32() % 3 {
                    0 => (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * h,
                    1 => h + 2.0 + (r.next_u32() % 10) as f32,
                    _ => -h - 2.0 - (r.next_u32() % 10) as f32,
                };
                c.bus.write32(BOX + POS + 4 * k, f(v));
            }
            c.bus.write32(BOX + POS + 0xc, r.f32(2.0));

            c.g[A0][0] = sx(BOX);
            c.g[A1][0] = sx(REC);
        });
    }
}
