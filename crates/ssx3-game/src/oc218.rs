//! Staging wave 201/202 (oc218): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use crate::{abi, guest::Addr};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0013_1620,
        rider_cruise_sub_handle_input,
        "RiderCruiseSub_HandleInput",
    ),
    (
        0x0037_6c58,
        ps2_graphics_man_set_perspective,
        "PS2GraphicsMan_SetPerspective",
    ),
    (
        0x0013_ccf0,
        rider_ground_state_compute_drag,
        "RiderGroundState_ComputeDrag",
    ),
    (0x0041_1cd8, fpadd_parts_d, "_fpadd_parts_d"),
    (
        0x002b_d1b8,
        twpigd_screen_tint_blend,
        "tWPIGD_ScreenTint_Blend",
    ),
    (
        0x0016_68b8,
        chase_camera_algorithm_rotate_by_pitch,
        "ChaseCameraAlgorithm_RotateByPitch",
    ),
    (
        0x002f_3e28,
        c_splash_manager_render,
        "cSplashManager_Render",
    ),
    (
        0x002f_1510,
        rider_base_render_attack_fx,
        "RiderBase_RenderAttackFX",
    ),
];

/// `reg + off` (a field of the object a register points at).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off` (a `.lit4` constant or a global).
#[inline(always)]
fn gpo(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `sp + off` (the stack frame).
#[inline(always)]
fn stk(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

// ------------------------------------------------- RiderCruiseSub_HandleInput

/// `RiderCruiseSub` / `Pad` / `Rider` fields used by
/// `RiderCruiseSub_HandleInput`.
mod cruise {
    /// `sub+0`: the rider.
    pub const RIDER: u32 = 0x0;
    /// `pad+0`: the button word (bit 12 reset, 13 handplant, 14 cheat jump,
    /// 15 jump, 16/17 boost, 18/19 tweak; bits 20..25 the steer axis, 26..31
    /// the lean axis); `pad+4`: the stick word (bits 0..5 the crouch axis,
    /// 6..11 the press axis).
    pub const PAD_BUTTONS: u32 = 0x0;
    pub const PAD_STICKS: u32 = 0x4;
    /// Rider: velocity, forward axis (dotted for "moving backwards").
    pub const VEL: u32 = 0x1e0;
    pub const FORWARD: u32 = 0x3a0;
    /// Rider: the slew targets the animation choice writes (rate, value).
    pub const SLEW0_RATE: u32 = 0x200;
    pub const SLEW0_VALUE: u32 = 0x204;
    pub const SLEW1_RATE: u32 = 0x218;
    pub const SLEW1_VALUE: u32 = 0x21c;
    pub const SLEW2_RATE: u32 = 0x224;
    pub const SLEW2_VALUE: u32 = 0x228;
    /// Rider: the steer input (slewed), the carve input, the lean.
    pub const STEER: u32 = 0x1f0;
    pub const CARVE: u32 = 0x1fc;
    pub const TURN: u32 = 0x214;
    pub const CROUCH: u32 = 0x220;
    pub const SPIN_SPEED: u32 = 0x2dc;
    pub const BOOST_INPUT: u32 = 0x2fc;
    pub const GOOFY: u32 = 0x320;
    pub const SLOPE: u32 = 0x3b8;
    pub const SURFACE: u32 = 0x438;
    pub const ANIM: u32 = 0x784;
}

/// `void RiderCruiseSub_HandleInput(RiderCruiseSub* this, Pad* pad)`
/// (0x131620): the ground cruise sub-state's input, once per tick.
///
/// - The stick axes become steer/press/crouch inputs (6-bit signed fields
///   times 1/31); moving backwards (velocity . forward < 0) drops the crouch.
/// - The finish line, a reset (pad bit 12), leaving the ground, a handplant
///   (bit 13, otherwise a rail jump), a jump (`Rider_TryEnterPrewind`) and a
///   board press each take over and end the function when they start; a
///   handplant attempt or a tweak clamps the steer to +-0.5.
/// - Boost input, the slew targets, the timed-event and idle-look
///   animations.
/// - Then the layer-2 (body) animation: kept on 0x16 (pushing off) and 0x15
///   (turn around), on kinds 10 and 5; otherwise picked from the speed, the
///   turn, the surface, the crouch and the carve (0x5..0x11), with the
///   slew targets set to match, and played when it changed.
///
/// The blocks follow the original's (labels are their guest addresses);
/// the scratch registers are left as it leaves them.
pub fn rider_cruise_sub_handle_input(c: &mut Cpu) {
    use cruise::*;
    guest(c, |c| {
        // The frame (0x60): ra, s0..s2, f20..f23.
        let frame = Frame::open(c, 0x60);
        {
            c.cycles += 41;
            c.lwc1(0, gpo(c, -0x75f4)); // 1/31
            frame.save(c, S0, 0x30);
            frame.save(c, S1, 0x20);
            c.g[S0][0] = c.g[A1][0];
            frame.save_fpr(c, 23, 0x58);
            c.g[S1][0] = c.g[A0][0];
            frame.save_fpr(c, 22, 0x50);
            frame.save_fpr(c, 21, 0x48);
            frame.save_fpr(c, 20, 0x40);
            frame.save(c, S2, 0x10);
            frame.save_ra(c, 0);
            c.mtc1(0, 23); // f23 = 0
            c.g[A0][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[V0][0] = c.lw(at(c, S0, PAD_STICKS));
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 6);
            c.g[A2][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 26) as u32);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            // velocity . forward
            c.lqc2(3, at(c, A2, VEL));
            c.lqc2(5, at(c, A2, FORWARD));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(5, 4);
            c.mtc1(5, 2);
            c.mtc1(3, 3);
            c.cvt_s_w(3, 3);
            c.mtc1(4, 4);
            c.cvt_s_w(4, 4);
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.c_lt_s(2, 23);
            c.mul_s(21, 3, 0); // f21 steer
            c.mul_s(20, 1, 0); // f20 crouch
            let backwards = c.fcc();
            c.mul_s(22, 4, 0); // f22 press
            if backwards {
                c.cycles += 1;
                c.mov_s(20, 23);
            }
        }
        'b_131c0c: {
            c.cycles += 4;
            c.g[A0][0] = c.g[A2][0];
            call_ptr(c, 0x0011_6378, 0x0013_16d0)?; // Rider_TryEnterFinishLine
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 8;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A2][0] = 0;
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_6120, 0x0013_16f0)?; // Rider_CheckResetPath
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0013_1cc0, 0x0013_1700)?; // RiderCruiseSub_CheckAirborne
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            'b_131780: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S0, PAD_BUTTONS)) & 0x2000;
                if c.g[V0][0] != 0 {
                    // Handplant button: try a handplant (a1 = press > 0.5).
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 20);
                    c.c_lt_s(20, 22);
                    let hard = c.fcc();
                    c.g[A1][0] = 0x1;
                    if !hard {
                        c.cycles += 1;
                        c.g[A1][0] = 0;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    call_ptr(c, 0x0010_7578, 0x0013_173c)?; // Rider_TryStartHandplant
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x30);
                        break 'b_131c0c;
                    }
                    // steer = clamp(steer, -0.5, 0.5); press 1, crouch 0.
                    c.cycles += 6;
                    c.g[AT][0] = 0xffff_ffff_bf00_0000;
                    c.mtc1(1, 0);
                    c.c_le_s(0, 21);
                    if c.fcc() {
                        c.min_s(0, 21, 20);
                    }
                    c.cycles += 5;
                    c.mov_s(21, 0);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 22);
                    c.mov_s(20, 23);
                    break 'b_131780;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                call_ptr(c, 0x0010_6848, 0x0013_1778)?; // Rider_TryStartRailslide
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_131c0c;
                }
            }
            c.cycles += 9;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 15) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_62c8, 0x0013_179c)?; // Rider_TryEnterPrewind
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 4;
            let jump = c.lw(at(c, S0, PAD_BUTTONS)) & 0x8000;
            c.g[V0][0] = jump;
            c.g[V0][0] = c.lw(at(c, S0, PAD_STICKS));
            if jump != 0 {
                // Jump held: press 1, crouch 0.
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 22);
                c.mtc1(0, 20);
            }
            c.cycles += 10;
            c.lwc1(0, gpo(c, -0x75f0)); // 1/31
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.mtc1(2, 12);
            c.cvt_s_w(12, 12);
            c.mul_s(12, 12, 0);
            call_ptr(c, 0x0011_61d0, 0x0013_17e0)?; // Rider_TryEnterBoardPress
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 9;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 19) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 18) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_63b0, 0x0013_1804)?; // Rider_UpdateTweakLayerAnim
            if c.g[V0][0] != 0 {
                // A tweak plays: steer = clamp(steer, -0.5, 0.5).
                c.cycles += 8;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1);
                c.c_le_s(0, 21);
                if c.fcc() {
                    c.min_s(0, 21, 1);
                }
                c.cycles += 2;
                c.mov_s(21, 0);
            }
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            cruise_body_anim(c)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCruiseSub_HandleInput` from 0x131834: boost, slews, the timed and
/// idle animations, then the body-layer animation choice (s1 = this, a1 =
/// the pad's buttons, f20..f22 crouch/steer/press). Ends with the final
/// 1-cycle block (before the `lq s0`).
fn cruise_body_anim(c: &mut Cpu) -> crate::call::Flow {
    use cruise::*;
    'b_131be8: {
        'b_1318c4: {
            c.cycles += 25;
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_4130, 0x0013_184c)?; // Rider_HandleBoostInput
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.mov_s(12, 21);
            call_ptr(c, 0x0011_3e80, 0x0013_1858)?; // Rider_SetSlew0Target
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.mov_s(13, 20);
            c.mov_s(12, 22);
            call_ptr(c, 0x0011_3f88, 0x0013_1868)?; // Rider_SetSlew3And4Targets
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            call_ptr(c, 0x0011_5b58, 0x0013_1870)?; // Rider_UpdateTimedEventAnim
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            call_ptr(c, 0x0011_5d48, 0x0013_1878)?; // Rider_UpdateIdleLookAnim
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_2aa0, 0x0013_1888)?; // AnimController_GetLayerAnimId
                                                    // s0 = the anim to play, s2 = the current one.
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x16;
            c.g[S2][0] = c.g[S0][0];
            if c.g[S0][0] != c.g[V0][0] {
                break 'b_1318c4;
            }
            // 0x16 (pushing off): fixed slew rates (1/30), values 0.
            c.cycles += 11;
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.lwc1(0, gpo(c, -0x75ec));
            c.sw(at(c, V1, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V1, SLEW0_RATE));
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V0, SLEW1_RATE));
            c.sw(at(c, V0, SLEW1_VALUE), 0);
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V1, SLEW2_RATE));
            c.sw(at(c, V1, SLEW2_VALUE), 0);
            break 'b_131be8;
        }
        'b_13191c: {
            'b_1318fc: {
                c.cycles += 3;
                c.g[V0][0] = 0x15;
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_1318fc;
                }
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.mtc1(0, 1);
                c.lwc1(0, at(c, A0, SPIN_SPEED));
                c.c_eq_s(0, 1);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_13191c;
                }
                c.cycles += 4;
                call_ptr(c, 0x0011_4cc0, 0x0013_18f4)?; // Rider_TurnAroundIfBackward
                let turned = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                if !turned {
                    break 'b_13191c;
                }
            }
            // 0x15 (turn around).
            c.cycles += 8;
            c.g[S0][0] = 0x15;
            c.lwc1(0, gpo(c, -0x75e8));
            c.sw(at(c, V0, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V0, SLEW0_RATE));
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V1, SLEW1_RATE));
            c.sw(at(c, V1, SLEW1_VALUE), 0);
            break 'b_131be8;
        }
        'b_131944: {
            c.cycles += 6;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_1ae8, 0x0013_1928)?; // AnimController_GetLayerAnimEntry
            c.g[V1][0] = 0xa;
            let other = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            if other {
                break 'b_131944;
            }
            // Kind 10: steer slew only.
            c.cycles += 4;
            c.lwc1(0, gpo(c, -0x75e4));
            c.sw(at(c, V0, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V0, SLEW0_RATE));
            break 'b_131be8;
        }
        'b_131980: {
            c.cycles += 6;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_1ae8, 0x0013_1950)?; // AnimController_GetLayerAnimEntry
            c.g[V1][0] = 0x5;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                break 'b_131980;
            }
            // Kind 5: steer slews toward the steer input.
            c.cycles += 9;
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.lwc1(1, gpo(c, -0x75e0));
            c.lwc1(0, at(c, V0, STEER));
            c.swc1(1, at(c, V0, SLEW0_RATE));
            c.swc1(0, at(c, V0, SLEW0_VALUE));
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(1, at(c, V1, SLEW1_RATE));
            c.sw(at(c, V1, SLEW1_VALUE), 0);
            break 'b_131be8;
        }
        // The free choice: f20 = speed (|velocity|), v1 = rider.
        'b_131b6c: {
            c.cycles += 16;
            c.mtc1(0, 3);
            c.lqc2(3, at(c, V1, VEL));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 20);
            c.lwc1(0, at(c, V1, TURN));
            c.c_eq_s(0, 3);
            let turning = !c.fcc();
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
            if turning {
                break 'b_131b6c;
            }
            'b_131a04: {
                c.cycles += 7;
                c.lwc1(0, gpo(c, -0x75dc));
                c.lwc1(2, at(c, V1, STEER));
                c.lwc1(1, gpo(c, -0x75d8));
                c.c_lt_s(20, 0);
                c.swc1(2, at(c, V1, SLEW0_VALUE));
                let slow = c.fcc();
                c.swc1(1, at(c, V1, SLEW0_RATE));
                if !slow {
                    break 'b_131a04;
                }
                c.cycles += 4;
                c.c_lt_s(3, 22);
                if c.fcc() {
                    c.g[S0][0] = 0x16;
                    break 'b_131be8;
                }
                c.cycles += 6;
                c.lwc1(1, gpo(c, -0x75d4));
                c.abs_s(0, 21);
                c.c_lt_s(1, 0);
                let steering = c.fcc();
                c.g[S0][0] = 0x16;
                if steering {
                    break 'b_131be8;
                }
            }
            'b_131a7c: {
                c.cycles += 5;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.g[V1][0] = 0x4;
                c.g[V0][0] = c.lw(at(c, A0, SURFACE));
                if c.g[V0][0] != c.g[V1][0] {
                    c.lwc1(0, at(c, A0, CARVE));
                    break 'b_131a7c;
                }
                // Surface 4.
                c.cycles += 13;
                c.lwc1(1, at(c, A0, STEER));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.abs_s(3, 1);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 2);
                c.sub_s(0, 0, 3);
                c.lwc1(1, at(c, A0, CROUCH));
                c.mul_s(0, 0, 2);
                c.c_le_s(1, 0);
                let low = c.fcc();
                c.g[S0][0] = 0xe;
                if low {
                    break 'b_131be8;
                }
                c.cycles += 6;
                c.lwc1(1, at(c, A0, BOOST_INPUT));
                c.mtc1(0, 0);
                c.c_lt_s(0, 1);
                let boosting = c.fcc();
                c.g[S0][0] = 0xf;
                if !boosting {
                    break 'b_131be8;
                }
                c.cycles += 4;
                c.lwc1(0, gpo(c, -0x75d0));
                c.c_lt_s(3, 0);
                if !c.fcc() {
                    break 'b_131be8;
                }
                c.cycles += 2;
                c.g[S0][0] = 0x8;
                break 'b_131be8;
            }
            'b_131af0: {
                c.cycles += 6;
                c.lwc1(1, gpo(c, -0x75cc));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_131af0;
                }
                c.cycles += 7;
                c.lwc1(0, at(c, A0, SLOPE));
                c.lwc1(1, gpo(c, -0x75c8));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_131af0;
                }
                c.cycles += 5;
                call_ptr(c, 0x0031_7810, 0x0013_1ab8)?; // Rand_NextGame
                c.g[V0][0] &= 0x1;
                let odd = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                if odd {
                    break 'b_131af0;
                }
                // A carve: 0x10 / 0x11 by side (flipped when goofy).
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c, V0, GOOFY));
                c.lwc1(1, at(c, V0, CARVE));
                if c.g[V1][0] != 0 {
                    c.cycles += 1;
                    c.neg_s(1, 1);
                }
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(1, 0);
                let left = c.fcc();
                c.g[S0][0] = 0x11;
                if !left {
                    break 'b_131be8;
                }
                c.cycles += 2;
                c.g[S0][0] = 0x10;
                break 'b_131be8;
            }
            c.cycles += 13;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(0, at(c, V0, STEER));
            c.abs_s(0, 0);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 3);
            c.sub_s(1, 1, 0);
            c.lwc1(2, at(c, V0, CROUCH));
            c.mul_s(1, 1, 3);
            c.c_le_s(2, 1);
            let low = c.fcc();
            c.g[S0][0] = 0x5;
            if low {
                break 'b_131be8;
            }
            c.cycles += 6;
            c.lwc1(1, at(c, V0, BOOST_INPUT));
            c.mtc1(0, 0);
            c.c_lt_s(0, 1);
            let boosting = c.fcc();
            c.g[S0][0] = 0x8;
            if boosting {
                break 'b_131be8;
            }
            c.cycles += 5;
            c.lwc1(0, gpo(c, -0x75c4));
            c.c_lt_s(20, 0);
            let slow = c.fcc();
            c.g[S0][0] = 0x7;
            if slow {
                break 'b_131be8;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, V0, SURFACE));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = 0x6;
                break 'b_131be8;
            }
            c.cycles += 2;
            break 'b_131be8;
        }
        // Turning: 0xb, or 0xc / 0xd by the turn side (and stance).
        {
            c.cycles += 7;
            c.lwc1(0, gpo(c, -0x75c0));
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            c.swc1(3, at(c, V1, SLEW0_VALUE));
            c.swc1(0, at(c, V1, SLEW0_RATE));
            c.lwc1(0, gpo(c, -0x75bc));
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.lwc1(0, gpo(c, -0x75b8));
            }
        }
        c.cycles += 4;
        c.c_lt_s(0, 20);
        if c.fcc() {
            c.g[S0][0] = 0xb;
            break 'b_131be8;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(at(c, S1, RIDER));
        c.lwc1(1, gpo(c, -0x75b4));
        c.lwc1(2, at(c, V0, TURN));
        c.abs_s(0, 2);
        c.c_lt_s(0, 1);
        let gentle = c.fcc();
        c.g[S0][0] = 0xb;
        if gentle {
            break 'b_131be8;
        }
        'b_131be4: {
            c.cycles += 4;
            c.c_lt_s(3, 2);
            let right = c.fcc();
            c.g[S0][0] = 0xd;
            if right {
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c, V0, GOOFY));
                c.g[V0][0] = 0xc;
                break 'b_131be4;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(at(c, V0, GOOFY));
            c.g[S0][0] = 0xc;
            c.g[V0][0] = 0xd;
        }
        c.cycles += 1;
        if c.g[V1][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
    }
    // Play the chosen animation when it changed.
    c.cycles += 2;
    c.g[A1][0] = c.g[S0][0];
    if c.g[S0][0] != c.g[S2][0] {
        c.cycles += 6;
        c.g[V0][0] = c.lw(at(c, S1, RIDER));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(at(c, V0, ANIM));
        call_ptr(c, 0x0031_28e8, 0x0013_1c08)?; // AnimController_PlayAnim
    }
    c.cycles += 1;
    Ok(())
}

// --------------------------------------------- PS2GraphicsMan_SetPerspective

/// `void PS2GraphicsMan_SetPerspective(PS2GraphicsMan* g, float fov, float
/// near, float far)` (0x376c58): switch the current view (`g+0x10dc`, views
/// of 0x230 bytes at `g+0x6d20`) to a perspective projection. Stores
/// near/far/fov in the view (clearing its ortho flag) and at
/// `g+0x5908`/`0x590c`; reads the view rectangle and the mode's depth
/// constants (mode 0x31 selects the large range); asks the display
/// (`g+0x10d8`, methods at +0x50/+0x58) for the screen size (the
/// `(float)(u32)` conversions halve-and-double values with the sign bit
/// set); takes `Math_SinCos(fov)` for the aspect-corrected focal length (the
/// clamp modes 0..2 of `g+0x6b94` scale it by `g+0x6ba0`/`0x6ba4`); builds
/// the screen matrix (`g+0x6af0`) and viewport quads (`0x6b70`/`0x6b80`),
/// then with `tanf(fov)` the guard-band clip matrix (`0x57c0`), the
/// projection (`0x5860`), the clip quads (`0x5840`/`0x5850`) and the
/// viewport half sizes/offsets (`0x58e0`/`0x58f0`); clamps the scissor
/// (`PS2GraphicsMan_SetScissorClamped`) and resets the constants at
/// `0x5910..0x592c`.
///
/// The FPU sequence is the original's, register for register (every
/// temporary it leaves in f0..f11 is reproduced); the blocks follow its
/// labels.
pub fn ps2_graphics_man_set_perspective(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xc0);
        'b_376d24: {
            'b_376d10: {
                c.cycles += 40;
                c.g[A1][0] = 0x31;
                frame.save(c, S0, 0x50);
                frame.save(c, S2, 0x30);
                c.g[S0][0] = c.g[A0][0];
                frame.save_fpr(c, 27, 0x98);
                c.g[A0][0] = 0x230;
                frame.save_fpr(c, 25, 0x88);
                c.mov_s(27, 12);
                frame.save_fpr(c, 23, 0x78);
                c.mov_s(25, 13);
                frame.save(c, S1, 0x40);
                c.mov_s(23, 14);
                frame.save_ra(c, 0x20);
                frame.save_fpr(c, 31, 0xb8);
                frame.save_fpr(c, 30, 0xb0);
                frame.save_fpr(c, 29, 0xa8);
                frame.save_fpr(c, 28, 0xa0);
                frame.save_fpr(c, 26, 0x90);
                frame.save_fpr(c, 24, 0x80);
                frame.save_fpr(c, 22, 0x70);
                frame.save_fpr(c, 21, 0x68);
                frame.save_fpr(c, 20, 0x60);
                c.g[V0][0] = c.lw(at(c, S0, 0x10dc));
                c.mult(0, 2, 2, 4);
                c.g[V0][0] = sx(at(c, V0, 0x6d20));
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw(at(c, S2, 0x20), 0u64);
                c.swc1(25, at(c, S2, 0x10));
                c.swc1(23, at(c, S2, 0x14));
                c.swc1(27, at(c, S2, 0x18));
                c.g[V0][0] = c.lw(at(c, S0, 0x10dc));
                c.g[V1][0] = c.lw(at(c, S0, 0x5a40));
                c.mult(0, 6, 2, 4);
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.lwc1(28, at(c, V0, 0x6d2c));
                c.lwc1(29, at(c, V0, 0x6d20));
                c.lwc1(30, at(c, V0, 0x6d24));
                let t = c.g[V1][0] != c.g[A1][0];
                c.lwc1(26, at(c, V0, 0x6d28));
                if t {
                    break 'b_376d10;
                } else {
                    c.cycles += 6;
                    c.lwc1(0, gpo(c, -0x27c8));
                    c.lwc1(1, gpo(c, -0x27c4));
                    c.lwc1(20, gpo(c, -0x27c0));
                    c.swc1(0, stk(c, 0x8));
                    c.swc1(1, stk(c, 0xc));
                    break 'b_376d24;
                }
            }
            c.cycles += 5;
            c.lwc1(5, gpo(c, -0x27bc));
            c.lwc1(0, gpo(c, -0x27b8));
            c.lwc1(20, gpo(c, -0x27b4));
            c.swc1(5, stk(c, 0x8));
            c.swc1(0, stk(c, 0xc));
            break 'b_376d24;
        }
        'b_376d88: {
            'b_376d70: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                c.swc1(23, at(c, S0, 0x590c));
                c.swc1(27, at(c, S0, 0x5908));
                c.g[A0][0] = c.lh(at(c, V1, 0x58));
                c.g[V0][0] = c.lw(at(c, V1, 0x5c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00376d40)?;
                c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.lh(at(c, V1, 0x50));
                c.g[V0][0] = c.lw(at(c, V1, 0x54));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00376d58)?;
                let t = (c.g[S1][0] as i64) < 0;
                c.g[A0][0] = c.g[V0][0];
                if t {
                    break 'b_376d70;
                } else {
                    c.cycles += 4;
                    c.mtc1(17, 1);
                    c.cvt_s_w(1, 1);
                    break 'b_376d88;
                }
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S1][0] as u32) >> 1);
            c.g[V0][0] = c.g[S1][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'b_376d88;
        }
        'b_376dbc: {
            'b_376da4: {
                c.cycles += 3;
                c.lwc1(0, gpo(c, -0x27b0));
                let t = (c.g[A0][0] as i64) < 0;
                c.mul_s(0, 1, 0);
                if t {
                    break 'b_376da4;
                } else {
                    c.cycles += 4;
                    c.mtc1(4, 21);
                    c.cvt_s_w(21, 21);
                    break 'b_376dbc;
                }
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
            c.g[V0][0] = c.g[A0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 21);
            c.cvt_s_w(21, 21);
            c.add_s(21, 21, 21);
            break 'b_376dbc;
        }
        'b_376e18: {
            'b_376e14: {
                c.cycles += 16;
                c.div_s(31, 0, 21);
                c.mov_s(12, 27);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[SP][0] | 0x4;
                call_ptr(c, 0x0031be50, 0x00376dd0)?;
                c.lwc1(1, stk(c, 0));
                c.lwc1(2, stk(c, 0x4));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.div_s(1, 1, 2);
                c.g[V1][0] = c.lw(at(c, S0, 0x6b94));
                c.mul_s(0, 26, 0);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                c.div_s(21, 0, 1);
                let t = c.g[V0][0] == 0u64;
                c.mul_s(22, 21, 31);
                if t {
                    break 'b_376e14;
                } else {
                    c.cycles += 2;
                    if (c.g[V1][0] as i64) < 0 {
                        c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                        break 'b_376e18;
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, at(c, S0, 0x6ba0));
                        c.lwc1(1, at(c, S0, 0x6ba4));
                        c.mul_s(21, 21, 0);
                        c.mul_s(22, 22, 1);
                        break 'b_376e14;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
            break 'b_376e18;
        }
        'b_376e54: {
            'b_376e44: {
                c.cycles += 7;
                c.g[A0][0] = c.lh(at(c, V1, 0x50));
                c.g[V0][0] = c.lw(at(c, V1, 0x54));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00376e28)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if (c.g[V0][0] as i64) < 0 {
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    break 'b_376e44;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 1);
                    c.cvt_s_w(1, 1);
                    c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                    break 'b_376e54;
                }
            }
            c.cycles += 4;
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
            break 'b_376e54;
        }
        'b_376e98: {
            'b_376e8c: {
                c.cycles += 10;
                c.g[AT][0] = 0x47000000;
                c.mtc1(1, 0);
                c.g[A0][0] = c.lh(at(c, V1, 0x58));
                c.g[V0][0] = c.lw(at(c, V1, 0x5c));
                c.sub_s(24, 0, 1);
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00376e70)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                if (c.g[V0][0] as i64) < 0 {
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    break 'b_376e8c;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 3);
                    c.cvt_s_w(3, 3);
                    break 'b_376e98;
                }
            }
            c.cycles += 3;
            c.mtc1(2, 3);
            c.cvt_s_w(3, 3);
            c.add_s(3, 3, 3);
            break 'b_376e98;
        }
        'b_376ffc: {
            'b_376ff8: {
                c.cycles += 82;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.neg_s(5, 20);
                c.sub_s(2, 23, 25);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 4);
                c.mul_s(1, 28, 0);
                c.sw(at(c, S0, 0x6b00), 0u64);
                c.mul_s(0, 26, 0);
                c.g[AT][0] = 0x47000000;
                c.mtc1(1, 7);
                c.mul_s(5, 5, 25);
                c.swc1(2, stk(c, 0x10));
                c.add_s(1, 1, 30);
                c.sw(at(c, S0, 0x6b20), 0u64);
                c.add_s(0, 0, 29);
                c.sw(at(c, S0, 0x6af4), 0u64);
                c.div_s(5, 5, 2);
                c.sw(at(c, S0, 0x6b24), 0u64);
                c.sub_s(7, 7, 3);
                c.sw(at(c, S0, 0x6af8), 0u64);
                c.mul_s(0, 0, 4);
                c.sw(at(c, S0, 0x6b08), 0u64);
                c.mul_s(1, 1, 4);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.neg_s(2, 22);
                c.mov_s(12, 27);
                c.mul_s(6, 21, 4);
                c.add_s(1, 1, 7);
                c.swc1(5, at(c, S0, 0x6b18));
                c.mul_s(2, 2, 4);
                c.add_s(0, 0, 24);
                c.swc1(6, at(c, S0, 0x6af0));
                c.neg_s(3, 23);
                c.swc1(1, at(c, S0, 0x6b14));
                c.swc1(2, at(c, S0, 0x6b04));
                c.add_s(6, 29, 26);
                c.swc1(0, at(c, S0, 0x6b10));
                c.add_s(2, 30, 28);
                c.mul_s(3, 3, 5);
                c.lwc1(0, at(c, S2, 0x1c));
                c.mul_s(6, 6, 4);
                c.mul_s(2, 2, 4);
                c.swc1(20, at(c, S0, 0x6b1c));
                c.add_s(3, 3, 0);
                c.sw(at(c, S0, 0x6afc), 0u64);
                c.mul_s(0, 5, 23);
                c.sw(at(c, S0, 0x6b0c), 0u64);
                c.mul_s(1, 29, 4);
                c.sw(at(c, S0, 0x6b2c), 0u64);
                c.div_s(8, 20, 25);
                c.sw(at(c, S0, 0x6b90), 0u64);
                c.div_s(9, 20, 23);
                c.swc1(3, at(c, S0, 0x6b28));
                c.mul_s(4, 30, 4);
                c.mul_s(5, 5, 25);
                c.div_s(21, 26, 28);
                c.add_s(6, 6, 24);
                c.add_s(2, 2, 7);
                c.swc1(8, at(c, S0, 0x6b7c));
                c.add_s(0, 0, 3);
                c.swc1(9, at(c, S0, 0x6b8c));
                c.add_s(1, 1, 24);
                c.swc1(6, at(c, S0, 0x6b80));
                c.add_s(4, 4, 7);
                c.swc1(2, at(c, S0, 0x6b84));
                c.add_s(5, 5, 3);
                c.swc1(0, at(c, S0, 0x6b88));
                c.mul_s(21, 31, 21);
                c.swc1(1, at(c, S0, 0x6b70));
                c.swc1(4, at(c, S0, 0x6b74));
                c.swc1(5, at(c, S0, 0x6b78));
                call_ptr(c, 0x0040d588, 0x00376fc0)?;
                c.div_s(22, 20, 0);
                c.mov_s(12, 27);
                call_ptr(c, 0x0040d588, 0x00376fcc)?;
                c.div_s(20, 20, 0);
                c.g[V1][0] = c.lw(at(c, S0, 0x6b94));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.mul_s(20, 20, 21);
                if t {
                    break 'b_376ff8;
                } else {
                    c.cycles += 2;
                    if (c.g[V1][0] as i64) < 0 {
                        c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                        break 'b_376ffc;
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, at(c, S0, 0x6ba0));
                        c.lwc1(1, at(c, S0, 0x6ba4));
                        c.mul_s(22, 22, 0);
                        c.mul_s(20, 20, 1);
                        break 'b_376ff8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
            break 'b_376ffc;
        }
        'b_377038: {
            'b_377024: {
                c.cycles += 6;
                c.g[A0][0] = c.lh(at(c, V1, 0x50));
                c.g[V0][0] = c.lw(at(c, V1, 0x54));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037700c)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t {
                    break 'b_377024;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 1);
                    c.cvt_s_w(1, 1);
                    break 'b_377038;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'b_377038;
        }
        'b_377090: {
            'b_37707c: {
                c.cycles += 13;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                c.mul_s(1, 1, 0);
                c.mul_s(0, 26, 0);
                c.g[A0][0] = c.lh(at(c, V1, 0x58));
                c.g[V0][0] = c.lw(at(c, V1, 0x5c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sub_s(0, 0, 1);
                let t = c.g[V0][0] as u32;
                c.add_s(21, 0, 29);
                call_ptr(c, t, 0x00377064)?;
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                if t {
                    break 'b_37707c;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 3);
                    c.cvt_s_w(3, 3);
                    break 'b_377090;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 3);
            c.cvt_s_w(3, 3);
            c.add_s(3, 3, 3);
            break 'b_377090;
        }
        c.cycles += 122;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 8);
        c.add_s(4, 23, 23);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 11);
        c.add_s(10, 23, 25);
        c.mul_s(7, 28, 8);
        c.swc1(22, at(c, S0, 0x5860));
        c.mul_s(3, 3, 8);
        c.swc1(11, at(c, S0, 0x588c));
        c.mul_s(4, 4, 25);
        c.sw(at(c, S0, 0x5864), 0u64);
        c.abs_s(2, 21);
        c.g[AT][0] = 0x45000000;
        c.mtc1(1, 1);
        c.sub_s(3, 7, 3);
        c.lwc1(5, stk(c, 0x10));
        c.neg_s(4, 4);
        c.lwc1(9, gpo(c, -0x27ac));
        c.add_s(2, 2, 2);
        c.swc1(20, at(c, S0, 0x5874));
        c.add_s(3, 3, 30);
        c.sw(at(c, S0, 0x5868), 0u64);
        c.abs_s(0, 3);
        c.div_s(10, 10, 5);
        c.div_s(4, 4, 5);
        c.sw(at(c, S0, 0x586c), 0u64);
        c.add_s(0, 0, 0);
        c.sw(at(c, S0, 0x5870), 0u64);
        c.sub_s(2, 1, 2);
        c.sw(at(c, S0, 0x5878), 0u64);
        c.add_s(3, 3, 9);
        c.sw(at(c, S0, 0x587c), 0u64);
        c.add_s(9, 21, 9);
        c.sw(at(c, S0, 0x5880), 0u64);
        c.sub_s(1, 1, 0);
        c.swc1(10, at(c, S0, 0x5888));
        c.neg_s(7, 7);
        c.swc1(4, at(c, S0, 0x5898));
        c.mul_s(0, 26, 8);
        c.swc1(3, at(c, S0, 0x58f4));
        c.div_s(5, 26, 2);
        c.swc1(9, at(c, S0, 0x58f0));
        c.swc1(7, at(c, S0, 0x58e4));
        c.div_s(6, 28, 1);
        c.sw(at(c, S0, 0x5884), 0u64);
        c.mul_s(1, 1, 8);
        c.sw(at(c, S0, 0x5890), 0u64);
        c.mul_s(2, 2, 8);
        c.sw(at(c, S0, 0x5894), 0u64);
        c.mov_s(12, 29);
        c.sw(at(c, S0, 0x589c), 0u64);
        c.neg_s(1, 1);
        c.swc1(0, at(c, S0, 0x58e0));
        c.mul_s(5, 5, 22);
        c.lwc1(0, stk(c, 0x8));
        c.mul_s(6, 6, 20);
        c.sw(at(c, S0, 0x58ec), 0u64);
        c.mov_s(13, 30);
        c.swc1(0, at(c, S0, 0x58e8));
        c.mov_s(14, 26);
        c.mov_s(15, 28);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, at(c, S2, 0x1c));
        c.swc1(5, at(c, S0, 0x57c0));
        c.lwc1(5, stk(c, 0xc));
        c.swc1(6, at(c, S0, 0x57d4));
        c.add_s(0, 5, 0);
        c.sw(at(c, S0, 0x58fc), 0u64);
        c.sw(at(c, S0, 0x57c4), 0u64);
        c.sw(at(c, S0, 0x57c8), 0u64);
        c.swc1(0, at(c, S0, 0x58f8));
        c.sw(at(c, S0, 0x57cc), 0u64);
        c.sw(at(c, S0, 0x57d0), 0u64);
        c.sw(at(c, S0, 0x57d8), 0u64);
        c.sw(at(c, S0, 0x57dc), 0u64);
        c.sw(at(c, S0, 0x57e0), 0u64);
        c.swc1(10, at(c, S0, 0x57e8));
        c.swc1(11, at(c, S0, 0x57ec));
        c.swc1(4, at(c, S0, 0x57f8));
        c.swc1(2, at(c, S0, 0x5840));
        c.swc1(1, at(c, S0, 0x5844));
        c.lwc1(0, stk(c, 0x8));
        c.swc1(9, at(c, S0, 0x5850));
        c.swc1(3, at(c, S0, 0x5854));
        c.swc1(0, at(c, S0, 0x5848));
        c.sw(at(c, S0, 0x57e4), 0u64);
        c.sw(at(c, S0, 0x57f0), 0u64);
        c.sw(at(c, S0, 0x57f4), 0u64);
        c.sw(at(c, S0, 0x57fc), 0u64);
        c.sw(at(c, S0, 0x584c), 0u64);
        c.lwc1(0, at(c, S2, 0x1c));
        c.sw(at(c, S0, 0x585c), 0u64);
        c.add_s(0, 5, 0);
        c.swc1(0, at(c, S0, 0x5858));
        call_ptr(c, 0x00376938, 0x00377210)?; // PS2GraphicsMan_SetScissorClamped
        c.lwc1(1, gpo(c, -0x27a8));
        c.lwc1(2, gpo(c, -0x27a4));
        c.lwc1(0, gpo(c, -0x27a0));
        c.swc1(1, at(c, S0, 0x5910));
        c.swc1(0, at(c, S0, 0x592c));
        c.swc1(2, at(c, S0, 0x5920));
        c.sw(at(c, S0, 0x5924), 0u64);
        c.sw(at(c, S0, 0x5928), 0u64);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 31, 0xb8);
        frame.restore_fpr(c, 30, 0xb0);
        frame.restore_fpr(c, 29, 0xa8);
        frame.restore_fpr(c, 28, 0xa0);
        frame.restore_fpr(c, 27, 0x98);
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

// --------------------------------------------- RiderGroundState_ComputeDrag

/// `float RiderGroundState_ComputeDrag(RiderGroundState* this, DragCfg* cfg,
/// float speed /*f12*/)` (0x13ccf0): the ground state's along-track drag.
///
/// - Three character factors from the behaviour library (interface 3:
///   attribute factors 0, 1 and 3 of the rider's player/value): `k1 = 1 /
///   (f1 * a + 1)`, `k3 = f3 * b + 1` (`.lit4` gains).
/// - The drag curve: `|speed| * k` looked up in the four points of `cfg`
///   (`+0x90..+0xac`: x/y pairs, linear between them, clamped at the ends),
///   blended toward `k1` by `|speed| / max`.
/// - A ramp on the rider's `+0x378` (0..1 between two thresholds) scales a
///   turn-dependent lift (`|turn|^2 * k3`) and the turn drag.
/// - Surfaces 2, 3 and 13 scale by `0.5 - depth / this->depth`; climbing
///   (`+0x1e8 > +0x3d8`), a stance mismatch without boost, and climbing
///   forward on surface 13 scale by `.lit4` factors.
/// - Result in f0: `((-speed * surface) * (curve + lift) * climb * stance -
///   turnDrag) * f5`.
///
/// The FPU sequence is the original's, register for register; the blocks
/// follow its labels.
pub fn rider_ground_state_compute_drag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'b_13ce64: {
            'b_13ce54: {
                'b_13ce50: {
                    'b_13ce2c: {
                        c.cycles += 60;
                        frame.save(c, S0, 0x20);
                        frame.save(c, S1, 0x10);
                        c.g[S0][0] = c.g[A1][0];
                        frame.save_fpr(c, 23, 0x48);
                        c.g[S1][0] = c.g[A0][0];
                        frame.save_fpr(c, 22, 0x40);
                        frame.save_fpr(c, 21, 0x38);
                        frame.save_fpr(c, 20, 0x30);
                        c.mov_s(21, 12);
                        frame.save_ra(c, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 22);
                        c.g[V0][0] = c.lw(at(c, S1, 0x18));
                        c.lwc1(0, at(c, V0, 0x214));
                        c.abs_s(20, 0);
                        call_ptr(c, 0x0014dc80, 0x0013cd38)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cd44)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x001494c0, 0x0013cd58)?; // BECharInt_GetAttrib0Factor
                        call_ptr(c, 0x0014dc80, 0x0013cd60)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cd6c)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x001493d8, 0x0013cd80)?; // BECharInt_GetAttrib1Factor
                        c.lwc1(1, gpo(c, -0x709c));
                        c.mul_s(0, 0, 1);
                        c.add_s(0, 0, 22);
                        c.div_s(23, 22, 0);
                        call_ptr(c, 0x0014dc80, 0x0013cd98)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cda4)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x00148d80, 0x0013cdb8)?; // BECharInt_GetAttrib3Factor
                        c.lwc1(1, gpo(c, -0x7098));
                        c.abs_s(3, 21);
                        c.lwc1(2, gpo(c, -0x7094));
                        c.mul_s(3, 3, 1);
                        c.lwc1(4, at(c, S0, 0x98));
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(4, 3);
                        let t = !c.fcc();
                        c.add_s(9, 0, 22);
                        if t {
                            break 'b_13ce2c;
                        } else {
                            'b_13ce1c: {
                                c.cycles += 5;
                                c.lwc1(1, at(c, S0, 0xa0));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, at(c, S0, 0x9c));
                                    break 'b_13ce1c;
                                } else {
                                    'b_13ce10: {
                                        c.cycles += 5;
                                        c.lwc1(4, at(c, S0, 0xa8));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, at(c, S0, 0xa4));
                                            break 'b_13ce10;
                                        } else {
                                            c.cycles += 2;
                                            c.lwc1(3, at(c, S0, 0xac));
                                            break 'b_13ce64;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    c.lwc1(0, at(c, S0, 0xac));
                                    break 'b_13ce50;
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, at(c, S0, 0xa4));
                            c.sub_s(1, 1, 4);
                            break 'b_13ce54;
                        }
                    }
                    'b_13ce48: {
                        c.cycles += 5;
                        c.lwc1(1, at(c, S0, 0x90));
                        c.c_lt_s(3, 1);
                        if !c.fcc() {
                            c.lwc1(2, at(c, S0, 0x94));
                            break 'b_13ce48;
                        } else {
                            c.cycles += 2;
                            c.lwc1(3, at(c, S0, 0x94));
                            break 'b_13ce64;
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, at(c, S0, 0x9c));
                    break 'b_13ce50;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13ce54;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(3, 2, 0);
            break 'b_13ce64;
        }
        'b_13ce90: {
            c.cycles += 10;
            c.lwc1(1, gpo(c, -0x7090));
            c.abs_s(0, 21);
            c.mtc1(0, 6);
            c.div_s(0, 0, 1);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 5);
            c.c_le_s(6, 0);
            let t = !c.fcc();
            c.mov_s(1, 6);
            if t {
                break 'b_13ce90;
            } else {
                c.cycles += 1;
                c.min_s(1, 0, 5);
                break 'b_13ce90;
            }
        }
        'b_13cee8: {
            'b_13cee4: {
                c.cycles += 12;
                c.sub_s(0, 5, 1);
                c.g[V0][0] = c.lw(at(c, S1, 0x18));
                c.mul_s(1, 22, 1);
                c.lwc1(4, gpo(c, -0x708c));
                c.lwc1(2, at(c, V0, 0x378));
                c.g[A0][0] = c.g[V0][0];
                c.mul_s(0, 23, 0);
                c.c_le_s(4, 2);
                c.add_s(0, 0, 1);
                c.mul_s(0, 22, 0);
                let t = !c.fcc();
                c.mul_s(3, 3, 0);
                if t {
                    break 'b_13cee4;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, gpo(c, -0x7088));
                    c.c_le_s(2, 0);
                    if !c.fcc() {
                        c.mov_s(4, 5);
                        break 'b_13cee8;
                    } else {
                        c.cycles += 4;
                        c.sub_s(0, 2, 4);
                        c.div_s(4, 0, 4);
                        break 'b_13cee8;
                    }
                }
            }
            c.cycles += 1;
            c.mov_s(4, 6);
            break 'b_13cee8;
        }
        'b_13cf48: {
            'b_13cf2c: {
                c.cycles += 14;
                c.lwc1(0, gpo(c, -0x7084));
                c.lwc1(1, gpo(c, -0x7080));
                c.mul_s(0, 20, 0);
                c.g[V1][0] = c.lw(at(c, A0, 0x438));
                c.mul_s(1, 20, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 8);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                c.mul_s(0, 4, 0);
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                c.mul_s(7, 4, 1);
                c.mul_s(0, 0, 20);
                let t = c.g[V0][0] != 0u64;
                c.mul_s(6, 0, 9);
                if t {
                    break 'b_13cf2c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xd;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.lwc1(0, at(c, A0, 0x1e8));
                        break 'b_13cf48;
                    } else {
                        break 'b_13cf2c;
                    }
                }
            }
            c.cycles += 7;
            c.lwc1(0, at(c, S1, 0x8));
            c.lwc1(1, at(c, A0, 0x454));
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 2);
            c.div_s(1, 1, 0);
            c.sub_s(8, 2, 1);
            c.lwc1(0, at(c, A0, 0x1e8));
            break 'b_13cf48;
        }
        'b_13cf70: {
            c.cycles += 9;
            c.lwc1(1, at(c, A0, 0x3d8));
            c.mtc1(0, 4);
            c.sub_s(0, 0, 1);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 12);
            c.c_lt_s(4, 0);
            let t = !c.fcc();
            c.g[V1][0] = c.lw(at(c, A0, 0x320));
            if t {
                break 'b_13cf70;
            } else {
                c.cycles += 1;
                c.lwc1(12, gpo(c, -0x707c));
                break 'b_13cf70;
            }
        }
        'b_13cf98: {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A0, 0x324));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            let t = c.g[V1][0] == c.g[V0][0];
            if t {
                break 'b_13cf98;
            } else {
                c.cycles += 4;
                c.lwc1(0, at(c, A0, 0x2fc));
                c.c_eq_s(0, 4);
                let t = !c.fcc();
                if t {
                    break 'b_13cf98;
                } else {
                    c.cycles += 1;
                    c.lwc1(2, gpo(c, -0x7078));
                    break 'b_13cf98;
                }
            }
        }
        'b_13cfe4: {
            c.cycles += 7;
            c.mtc1(0, 4);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 5);
            c.c_lt_s(4, 21);
            if !c.fcc() {
                c.neg_s(0, 21);
                break 'b_13cfe4;
            } else {
                'b_13cfe0: {
                    c.cycles += 7;
                    c.lwc1(0, at(c, A0, 0x1e8));
                    c.lwc1(1, at(c, A0, 0x3d8));
                    c.sub_s(0, 0, 1);
                    c.c_lt_s(4, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0xd;
                    if t {
                        break 'b_13cfe0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, A0, 0x438));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.neg_s(0, 21);
                            break 'b_13cfe4;
                        } else {
                            c.cycles += 1;
                            c.lwc1(5, gpo(c, -0x7074));
                            break 'b_13cfe0;
                        }
                    }
                }
                c.cycles += 1;
                c.neg_s(0, 21);
                break 'b_13cfe4;
            }
        }
        c.cycles += 16;
        frame.restore(c, S0, 0x20);
        c.add_s(1, 3, 6);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.mul_s(0, 0, 8);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        c.mul_s(0, 0, 1);
        frame.restore_fpr(c, 20, 0x30);
        c.mul_s(0, 0, 12);
        c.mul_s(0, 0, 2);
        c.sub_s(0, 0, 7);
        c.mul_s(0, 0, 5);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------------ _fpadd_parts_d

/// `fp_number_type* _fpadd_parts_d(fp_number_type* a, fp_number_type* b,
/// fp_number_type* tmp)` (0x411cd8, libgcc soft-float): the core of
/// `__adddf3`/`__subdf3` on unpacked operands (class, sign, exponent,
/// fraction; see `__unpack_d`).
///
/// - A NaN operand is returned as is (`a` first); infinity + infinity of
///   opposite signs returns the static NaN (0x52adc0); an infinity or a zero
///   operand returns the other one; zero + zero writes `tmp` as a copy of
///   `a` with the signs and-ed.
/// - Otherwise the smaller exponent's fraction is shifted right to align
///   (keeping a sticky low bit), or dropped when the exponents are 64 or
///   more apart; the fractions are added or subtracted by sign into `tmp`
///   (negative differences flip the sign), normalised left until the
///   implicit one reaches bit 60 and shifted right once (sticky) when the
///   sum overflowed past it. `tmp` becomes class "number"; returns `tmp`.
///
/// Registers as the original: t0 = a (then a's sign), a1 = b (then b's
/// sign, then the fraction), a3 = the exponent, t2/t3 = b's/a's fraction.
pub fn fpadd_parts_d(c: &mut Cpu) {
    const CLASS: u32 = 0x0;
    const SIGN: u32 = 0x4;
    /// The static NaN result (`0x530000 - 0x5240`).
    const NAN: u64 = 0x0052_adc0;

    'ret_a: {
        c.cycles += 5;
        c.g[T0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, T0, CLASS));
        c.g[V0][0] = (c.g[A0][0] < 2) as u64;
        if c.g[V0][0] != 0 {
            break 'ret_a; // a is a NaN
        }
        c.g[V1][0] = c.lw(at(c, A1, CLASS));
        'ret_b: {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 2) as u64;
            let b_nan = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[A0][0] ^ 0x4;
            if b_nan {
                break 'ret_b;
            }
            c.cycles += 2;
            let a_inf = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] ^ 0x4;
            if a_inf {
                // inf + x: x finite, or the same infinity, gives a.
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'ret_a;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw(at(c, A1, SIGN));
                c.g[V0][0] = c.lw(at(c, T0, SIGN));
                let same = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 0x0053_0000;
                if same {
                    break 'ret_a;
                }
                c.cycles += 2;
                c.g[V0][0] = NAN;
                ret(c);
                return;
            }
            c.cycles += 2;
            let b_inf = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0] ^ 0x2;
            if b_inf {
                break 'ret_b;
            }
            c.cycles += 2;
            let b_zero = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[A0][0] ^ 0x2;
            if b_zero {
                c.cycles += 2;
                let a_nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[A2][0];
                if a_nonzero {
                    break 'ret_a;
                }
                // zero + zero: tmp = a with sign a.sign & b.sign.
                c.cycles += 11;
                c.g[A0][0] = c.ld(at(c, T0, 0));
                c.sd(at(c, A2, 0), c.g[A0][0]);
                c.g[V1][0] = c.ld(at(c, T0, 0x8));
                c.sd(at(c, A2, 0x8), c.g[V1][0]);
                c.g[A0][0] = c.ld(at(c, T0, 0x10));
                c.sd(at(c, A2, 0x10), c.g[A0][0]);
                c.g[V1][0] = c.lw(at(c, T0, SIGN));
                c.g[A0][0] = c.lw(at(c, A1, SIGN));
                c.g[V1][0] &= c.g[A0][0];
                c.sw(at(c, A2, SIGN), c.g[V1][0]);
                ret(c);
                return;
            }
            c.cycles += 2;
            let a_zero = c.g[V0][0] == 0;
            c.g[V0][0] = u64::MAX;
            if a_zero {
                break 'ret_b;
            }
            fpadd_numbers(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

/// `_fpadd_parts_d` on two numbers (from 0x411d74; v0 = -1, t0 = a, a1 = b,
/// a2 = tmp): align, add, normalise, return `tmp`.
fn fpadd_numbers(c: &mut Cpu) {
    const SIGN: u32 = 0x4;
    const EXP: u32 = 0x8;
    const FRAC: u32 = 0x10;
    /// The implicit one's bit (60) times two, minus one: fractions at or
    /// below it are normalised further.
    const NORMAL_LIMIT: u64 = (u64::MAX << 5) >> 4;
    /// Above this the sum carried past bit 60.
    const OVERFLOW_LIMIT: u64 = u64::MAX >> 3;

    // Align the exponents.
    'aligned: {
        'far_apart: {
            c.cycles += 10;
            c.g[A3][0] = c.lw(at(c, T0, EXP)); // a.exp
            c.g[T1][0] = c.lw(at(c, A1, EXP)); // b.exp
            c.g[T3][0] = c.ld(at(c, T0, FRAC)); // a.frac
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T1][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            c.g[V1][0] = ((c.g[V1][0] as i64) < 64) as u64;
            c.g[T2][0] = c.ld(at(c, A1, FRAC)); // b.frac
            if c.g[V1][0] == 0 {
                break 'far_apart;
            }
            // Shift b's fraction right while b.exp < a.exp (sticky bit 0).
            'shift_a: {
                'b_not_smaller: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let b_smaller = c.g[V0][0] != 0;
                    c.g[T0][0] = c.lw(at(c, T0, SIGN));
                    if !b_smaller {
                        break 'b_not_smaller;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.lw(at(c, A1, SIGN));
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = c.g[T2][0] >> 1;
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                        c.g[V1][0] = c.g[T2][0] & 0x1;
                        c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        c.g[T2][0] = c.g[V1][0] | c.g[V0][0];
                        if c.g[A0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0041_1db8);
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    break 'shift_a;
                }
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, A1, SIGN));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
            }
            // Shift a's fraction right by b.exp - a.exp (sticky bit 0).
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'aligned;
            }
            c.cycles += 1;
            c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V1][0] = c.g[T3][0] >> 1;
                c.g[V0][0] = c.g[T3][0] & 0x1;
                c.g[T3][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                if c.g[A3][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0041_1df0);
                }
            }
            c.cycles += 2;
            c.g[A3][0] = c.g[T1][0];
            break 'aligned;
        }
        // 64 or more apart: the smaller operand's fraction drops out.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let b_smaller = c.g[V0][0] != 0;
        c.g[T0][0] = c.lw(at(c, T0, SIGN));
        if b_smaller {
            c.cycles += 3;
            c.g[T2][0] = 0;
            c.g[A1][0] = c.lw(at(c, A1, SIGN));
            break 'aligned;
        }
        c.cycles += 3;
        c.g[A3][0] = c.g[T1][0];
        c.g[A1][0] = c.lw(at(c, A1, SIGN));
        c.g[T3][0] = 0;
    }

    // Add (same signs) or subtract (the negative one from the other).
    'normalised: {
        c.cycles += 2;
        let same_sign = c.g[T0][0] == c.g[A1][0];
        c.g[V0][0] = c.g[T3][0].wrapping_add(c.g[T2][0]);
        if same_sign {
            c.cycles += 4;
            c.sw(at(c, A2, SIGN), c.g[T0][0]);
            c.sw(at(c, A2, EXP), c.g[A3][0]);
            c.g[A1][0] = c.g[V0][0];
            c.sd(at(c, A2, FRAC), c.g[V0][0]);
            break 'normalised;
        }
        c.cycles += 2;
        let a_negative = c.g[T0][0] != 0;
        c.g[V0][0] = c.g[T2][0].wrapping_sub(c.g[T3][0]);
        if !a_negative {
            c.cycles += 1;
            c.g[V0][0] = c.g[T3][0].wrapping_sub(c.g[T2][0]);
        }
        c.cycles += 2;
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0u64.wrapping_sub(c.g[V0][0]);
        c.cycles += 4;
        c.sw(at(c, A2, EXP), c.g[A3][0]);
        if negative {
            c.g[V0][0] = 0x1;
            c.sd(at(c, A2, FRAC), c.g[V1][0]);
            c.sw(at(c, A2, SIGN), c.g[V0][0]);
        } else {
            c.sd(at(c, A2, FRAC), c.g[V0][0]);
            c.sw(at(c, A2, SIGN), 0);
        }
        // Normalise: shift left until the implicit one reaches bit 60.
        c.cycles += 8;
        c.g[A1][0] = c.ld(at(c, A2, FRAC));
        c.g[V0][0] = NORMAL_LIMIT;
        c.g[V1][0] = c.g[A1][0].wrapping_sub(1);
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[A3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            break 'normalised;
        }
        c.cycles += 3;
        c.g[A1][0] = NORMAL_LIMIT;
        loop {
            c.cycles += 9;
            c.g[V0][0] = c.lw(at(c, A2, EXP));
            c.g[A0][0] = c.g[A3][0] << 1;
            c.g[V1][0] = c.g[A0][0].wrapping_sub(1);
            c.sd(at(c, A2, FRAC), c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
            c.sw(at(c, A2, EXP), c.g[V0][0]);
            c.g[A3][0] = c.g[A0][0];
            if c.g[V1][0] != 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0041_1ea0);
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[A0][0];
    }

    // Class "number"; a carry past the implicit one shifts right once.
    c.cycles += 6;
    c.g[V1][0] = 0x3;
    c.g[V0][0] = ((OVERFLOW_LIMIT) < c.g[A1][0]) as u64;
    c.sw(at(c, A2, 0), c.g[V1][0]);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = c.lw(at(c, A2, EXP));
        c.g[A0][0] = c.g[A1][0] >> 1;
        c.g[V1][0] = c.g[A1][0] & 0x1;
        c.g[V1][0] |= c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.sd(at(c, A2, FRAC), c.g[V1][0]);
        c.sw(at(c, A2, EXP), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

// ------------------------------------------------- tWPIGD_ScreenTint_Blend

/// `tWPIGD_ScreenTint` fields: the vptr (hook at +0x218: delta, +0x21c:
/// method), six `{current, target}` pairs from +0x08; the keyframe's six
/// values from +0x04.
mod screen_tint {
    pub const VPTR: u32 = 0x4;
    pub const VT_HOOK_DELTA: u32 = 0x218;
    pub const VT_HOOK: u32 = 0x21c;
    pub const PAIRS: u32 = 0x8;
    pub const KEY_VALUES: u32 = 0x4;
}

/// The screen tint's vtable hook (`this + delta`, method at +0x21c),
/// returning to `ret`; v1 holds the vptr.
fn screen_tint_hook(c: &mut Cpu, ret_to: u32) -> crate::call::Flow {
    use screen_tint::*;
    c.g[A0][0] = c.lh(at(c, V1, VT_HOOK_DELTA));
    c.g[V0][0] = c.lw(at(c, V1, VT_HOOK));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret_to)
}

/// `void tWPIGD_ScreenTint::Blend(const tWPIGD_ScreenTint* key, float t)`
/// (0x2bd1b8): blends the tint's six settings toward a keyframe with the
/// eased weight `w = t^2`: each pair `{current, target}` (+0x08 + 8i) gets
/// `current = w * key[i] + (1 - w) * current` and `target = key[i]` (`key`
/// from key+0x04). Calls its vtable hook (+0x218) before and after. Frame
/// 0x40 (s0, s1, f20); 76 cycles charged at entry.
///
/// The loads, stores and FPU temporaries (f0..f7) follow the original's
/// order (the key may alias the tint).
pub fn twpigd_screen_tint_blend(c: &mut Cpu) {
    use screen_tint::*;
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        // f20 = w = t * t
        c.mov_s(20, 12);
        c.g[V1][0] = c.lw(at(c, S0, VPTR));
        c.mul_s(20, 20, 20);
        screen_tint_hook(c, 0x002b_d1f0)?;

        let cur = |c: &Cpu, i: u32| at(c, S0, PAIRS + 8 * i);
        let tgt = |c: &Cpu, i: u32| at(c, S0, PAIRS + 8 * i + 4);
        let key = |c: &Cpu, i: u32| at(c, S1, KEY_VALUES + 4 * i);
        // f2 = 1 - w; the (1 - w) * current terms of pairs 1..5 go to
        // f3, f4, f6, f7, f2 (as the compiler scheduled them).
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.lwc1(1, key(c, 0));
        c.sub_s(2, 2, 20);
        c.lwc1(0, cur(c, 0));
        c.mul_s(1, 20, 1);
        c.lwc1(3, cur(c, 1));
        c.lwc1(4, cur(c, 2));
        c.mul_s(0, 2, 0);
        c.lwc1(6, cur(c, 3));
        c.mul_s(3, 2, 3);
        c.lwc1(7, cur(c, 4));
        c.mul_s(4, 2, 4);
        c.lwc1(5, cur(c, 5));
        c.add_s(1, 1, 0);
        c.g[V1][0] = c.lw(at(c, S0, VPTR));
        c.mul_s(6, 2, 6);
        c.mul_s(7, 2, 7);
        c.swc1(1, cur(c, 0));
        c.mul_s(2, 2, 5);
        c.lwc1(1, key(c, 0));
        c.swc1(1, tgt(c, 0));
        // Pairs 1..4: current = w * key + (1 - w) * current; target = key.
        for (i, scaled) in [(1, 3), (2, 4), (3, 6), (4, 7)] {
            c.lwc1(0, key(c, i));
            c.mul_s(0, 20, 0);
            c.add_s(0, 0, scaled);
            c.swc1(0, cur(c, i));
            c.lwc1(1, key(c, i));
            c.swc1(1, tgt(c, i));
        }
        // Pair 5 (through f20).
        c.lwc1(0, key(c, 5));
        c.mul_s(20, 20, 0);
        c.add_s(20, 20, 2);
        c.swc1(20, cur(c, 5));
        c.lwc1(0, key(c, 5));
        c.swc1(0, tgt(c, 5));
        screen_tint_hook(c, 0x002b_d2d0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------- ChaseCameraAlgorithm_RotateByPitch

/// `void ChaseCameraAlgorithm_RotateByPitch(ChaseCameraAlgorithm* this,
/// Vec4* out)` (0x1668b8): the chase camera's look direction. Starting from
/// the identity (0x4ff1a0), rotates by `-this->+0x54` about the axis at
/// 0x4ff150 and then by `-this->+0x50` about the axis at 0x4ff160 (each an
/// axis-angle matrix from `Math_SinCos`, `R = (1-c) a a^T + c I + s [a]x`,
/// multiplied on with VU0), and writes the first row to `out`.
///
/// Inlined `vu0` matrix copies and multiplies: the VU0 macro ops, stack
/// layout (the matrix at sp, its copy at sp+0x40, the rotation at
/// sp+0x40..0x70) and FPU temporaries are the original's.
pub fn chase_camera_algorithm_rotate_by_pitch(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x130);
        c.cycles += 233;
        c.g[V0][0] = 0x500000; // 0x4ff1a0: the identity matrix
        frame.save(c, S5, 0xc0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
        frame.save(c, S6, 0xb0);
        c.g[S5][0] = c.g[A0][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save(c, S2, 0xf0);
        frame.save(c, S3, 0xe0);
        frame.save(c, S4, 0xd0);
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 20, 0x120);
        c.lqc2(1, (c.g[V0][0] as u32));
        c.lqc2(2, at(c, V0, 0x10));
        c.lqc2(3, at(c, V0, 0x20));
        c.lqc2(4, at(c, V0, 0x30));
        c.sqc2(1, stk(c, 0));
        c.sqc2(2, stk(c, 0x10));
        c.sqc2(3, stk(c, 0x20));
        c.sqc2(4, stk(c, 0x30));
        c.lwc1(12, at(c, S5, 0x54));
        c.g[S0][0] = 0x500000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0)); // 0x4ff150: the first axis
        c.g[S4][0] = c.g[SP][0];
        c.neg_s(12, 12);
        c.g[S3][0] = sx(stk(c, 0x40));
        c.lqc2(1, (c.g[S4][0] as u32));
        c.lqc2(2, at(c, S4, 0x10));
        c.lqc2(3, at(c, S4, 0x20));
        c.lqc2(4, at(c, S4, 0x30));
        c.sqc2(1, (c.g[S3][0] as u32));
        c.sqc2(2, at(c, S3, 0x10));
        c.sqc2(3, at(c, S3, 0x20));
        c.sqc2(4, at(c, S3, 0x30));
        c.g[S2][0] = 0x500000;
        c.lq(3, stk(c, 0x70));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xed0));
        c.g[A0][0] = sx(stk(c, 0x90));
        c.lq(2, (c.g[S2][0] as u32));
        c.g[A1][0] = sx(stk(c, 0x94));
        c.sq(3, stk(c, 0x80));
        c.sq(2, stk(c, 0x70));
        call_ptr(c, 0x0031be50, 0x0016696c)?; // Math_SinCos(-yaw)
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.lwc1(11, stk(c, 0x94));
        c.lwc1(3, at(c, S1, 0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
        c.lwc1(7, at(c, S1, 0x4));
        c.lwc1(8, stk(c, 0x90));
        c.mul_s(2, 1, 3);
        c.lq(2, stk(c, 0x80));
        c.mul_s(4, 1, 0);
        c.sw(stk(c, 0x30), 0u64);
        c.mul_s(1, 1, 7);
        c.sw(stk(c, 0x34), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(stk(c, 0x38), 0u64);
        c.mul_s(12, 8, 0);
        c.swc1(20, stk(c, 0x3c)); // the rotation's w = 1
        c.mul_s(9, 2, 3);
        c.sq(2, stk(c, 0x30));
        c.mul_s(10, 4, 3);
        c.sw(stk(c, 0xc), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(stk(c, 0x1c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(stk(c, 0x2c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, stk(c, 0x8));
        c.add_s(6, 6, 11);
        c.swc1(0, stk(c, 0x4));
        c.add_s(4, 4, 13);
        c.swc1(2, stk(c, 0x18));
        c.sub_s(10, 10, 8);
        c.swc1(9, stk(c, 0x28));
        c.add_s(1, 1, 11);
        c.swc1(6, stk(c, 0));
        c.add_s(3, 3, 12);
        c.swc1(4, stk(c, 0x10));
        c.swc1(10, stk(c, 0x20));
        c.swc1(1, stk(c, 0x14));
        c.swc1(3, stk(c, 0x24));
        c.lqc2(4, (c.g[S4][0] as u32));
        c.lqc2(5, at(c, S4, 0x10));
        c.lqc2(6, at(c, S4, 0x20));
        c.lqc2(7, at(c, S4, 0x30));
        c.lqc2(8, (c.g[S3][0] as u32));
        c.lqc2(9, at(c, S3, 0x10));
        c.lqc2(10, at(c, S3, 0x20));
        c.lqc2(11, at(c, S3, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, (c.g[S4][0] as u32));
        c.sqc2(13, at(c, S4, 0x10));
        c.sqc2(14, at(c, S4, 0x20));
        c.sqc2(15, at(c, S4, 0x30));
        c.lwc1(12, at(c, S5, 0x50));
        c.g[S0][0] = 0x500000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0)); // 0x4ff160: the second axis
        c.g[S4][0] = c.g[SP][0];
        c.neg_s(12, 12);
        c.lqc2(1, (c.g[S4][0] as u32));
        c.lqc2(2, at(c, S4, 0x10));
        c.lqc2(3, at(c, S4, 0x20));
        c.lqc2(4, at(c, S4, 0x30));
        c.sqc2(1, (c.g[S3][0] as u32));
        c.sqc2(2, at(c, S3, 0x10));
        c.sqc2(3, at(c, S3, 0x20));
        c.sqc2(4, at(c, S3, 0x30));
        c.lq(2, (c.g[S2][0] as u32));
        c.g[A0][0] = sx(stk(c, 0x98));
        c.lq(3, stk(c, 0x70));
        c.g[A1][0] = sx(stk(c, 0x9c));
        c.sq(2, stk(c, 0x70));
        c.sq(3, stk(c, 0x80));
        call_ptr(c, 0x0031be50, 0x00166af4)?; // Math_SinCos(-pitch)
        c.lwc1(11, stk(c, 0x9c));
        c.lwc1(3, at(c, S1, 0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(7, at(c, S1, 0x4));
        c.lwc1(8, stk(c, 0x98));
        c.mul_s(2, 1, 3);
        c.lq(2, stk(c, 0x80));
        c.mul_s(4, 1, 0);
        c.swc1(20, stk(c, 0x3c)); // the rotation's w = 1
        c.mul_s(1, 1, 7);
        c.sw(stk(c, 0x30), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(stk(c, 0x34), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(stk(c, 0x38), 0u64);
        c.mul_s(9, 2, 3);
        c.sq(2, stk(c, 0x30));
        c.mul_s(10, 4, 3);
        c.sw(stk(c, 0xc), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(stk(c, 0x1c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(stk(c, 0x2c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, stk(c, 0x8));
        c.add_s(6, 6, 11);
        c.swc1(0, stk(c, 0x4));
        c.add_s(4, 4, 13);
        c.swc1(2, stk(c, 0x18));
        c.sub_s(10, 10, 8);
        c.swc1(9, stk(c, 0x28));
        c.add_s(1, 1, 11);
        c.swc1(6, stk(c, 0));
        c.add_s(3, 3, 12);
        c.swc1(4, stk(c, 0x10));
        c.swc1(10, stk(c, 0x20));
        c.swc1(1, stk(c, 0x14));
        c.swc1(3, stk(c, 0x24));
        c.lqc2(4, (c.g[S4][0] as u32));
        c.lqc2(5, at(c, S4, 0x10));
        c.lqc2(6, at(c, S4, 0x20));
        c.lqc2(7, at(c, S4, 0x30));
        c.lqc2(8, (c.g[S3][0] as u32));
        c.lqc2(9, at(c, S3, 0x10));
        c.lqc2(10, at(c, S3, 0x20));
        c.lqc2(11, at(c, S3, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, (c.g[S4][0] as u32));
        c.sqc2(13, at(c, S4, 0x10));
        c.sqc2(14, at(c, S4, 0x20));
        c.sqc2(15, at(c, S4, 0x30));
        c.lq(2, stk(c, 0));
        frame.restore(c, S0, 0x110);
        c.sq(2, (c.g[S6][0] as u32));
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 20, 0x120);
        c.sq(2, stk(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------------- cSplashManager_Render

/// `void cSplashManager::Render()` (0x2f3e28): draws the snow splashes when
/// splashes are on (`gp+0x115c`, `gp+0x1160`) and the manager belongs to
/// the current view (`this+0x10` against the game module's view id,
/// `app->+0x84->+0x84->+0x14`). With `gp+0xa70` set it hands over to its
/// own virtual (`this->vt+0x70`) instead. Otherwise, through the graphics
/// manager (`gp+0x2a90`) and its display's virtuals: push the render state
/// (+0xa8), read the screen origin (+0xe0) into the drop/crystal context
/// (sp+0x10), set a 640x480 ortho view (+0xd0), (+0xf8), load the identity
/// (+0x110, 0x4ff1a0); copy the two 20-byte GS records (the current one
/// and the template at 0x501420) into the packet (`gfx+0xe84`) and patch
/// the second one's register fields; render every drop
/// (`cSplashManager_RenderDrop`, `this+0x14` of them at `this+0x1c`,
/// 0x3c bytes each) and crystal (`cSplashManager_RenderCrystal`,
/// `this+0x18` at `this+0x724`, 0x4c each); drop the record again and pop
/// (+0x100, +0xb0).
///
/// The register use (including the mask constants left in a1..t6) and the
/// blocks follow the original's.
pub fn c_splash_manager_render(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'b_2f4104: {
            'b_2f4100: {
                'b_2f40fc: {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(at(c, GP, 0x115c));
                    frame.save(c, S3, 0x30);
                    frame.save(c, S0, 0x60);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S1, 0x50);
                    frame.save(c, S2, 0x40);
                    let t = c.g[V0][0] == 0u64;
                    frame.save_ra(c, 0x20);
                    if t {
                        break 'b_2f40fc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, 0x1160));
                        let t = c.g[V0][0] == 0u64;
                        frame.restore(c, S0, 0x60);
                        if t {
                            break 'b_2f4100;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(gpo(c, -0x848));
                            c.g[A1][0] = c.lw(at(c, S3, 0x10));
                            c.g[V0][0] = c.lw(at(c, A0, 0x84));
                            c.g[V1][0] = c.lw(at(c, V0, 0x84));
                            c.g[A0][0] = c.lw(at(c, V1, 0x14));
                            let t = c.g[A1][0] != c.g[A0][0];
                            frame.restore(c, S1, 0x50);
                            if t {
                                break 'b_2f4104;
                            } else {
                                'b_2f3e9c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(at(c, GP, 0xa70));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S1][0] = c.lw(at(c, GP, 0x2a90));
                                    if t {
                                        break 'b_2f3e9c;
                                    } else {
                                        c.cycles += 7;
                                        c.g[V0][0] = c.lw(at(c, S3, 0xc));
                                        c.g[A0][0] = c.lh(at(c, V0, 0x70));
                                        c.g[V1][0] = c.lw(at(c, V0, 0x74));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x002f3e94)?;
                                        frame.restore(c, S0, 0x60);
                                        break 'b_2f4100;
                                    }
                                }
                                'b_2f409c: {
                                    c.cycles += 118;
                                    c.g[S2][0] = 0u64;
                                    c.g[V0][0] = c.lw(at(c, S1, 0x10d8));
                                    c.g[A0][0] = c.lh(at(c, V0, 0xa8));
                                    c.g[V1][0] = c.lw(at(c, V0, 0xac));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x002f3eb4)?;
                                    c.g[V0][0] = c.lw(at(c, S1, 0x10d8));
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A1][0] = c.lh(at(c, V0, 0xe0));
                                    c.g[V1][0] = c.lw(at(c, V0, 0xe4));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    call_ptr(c, t, 0x002f3ecc)?;
                                    c.lwc1(0, stk(c, 0));
                                    c.g[A2][0] = 0u64;
                                    c.lwc1(1, stk(c, 0x4));
                                    c.g[A1][0] = 0x1;
                                    c.swc1(0, stk(c, 0x10));
                                    c.swc1(1, stk(c, 0x14));
                                    c.mtc1(0, 12);
                                    c.g[V0][0] = c.lw(at(c, S1, 0x10d8));
                                    c.g[AT][0] = 0x44200000;
                                    c.mtc1(1, 14);
                                    c.mov_s(13, 12);
                                    c.g[A0][0] = c.lh(at(c, V0, 0xd0));
                                    c.mov_s(16, 12);
                                    c.g[AT][0] = 0x43f00000;
                                    c.mtc1(1, 15);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 17);
                                    c.g[V1][0] = c.lw(at(c, V0, 0xd4));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x002f3f1c)?;
                                    c.g[V0][0] = c.lw(at(c, S1, 0x10d8));
                                    c.g[A0][0] = c.lh(at(c, V0, 0xf8));
                                    c.g[V1][0] = c.lw(at(c, V0, 0xfc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x002f3f30)?;
                                    c.g[A2][0] = c.lw(at(c, S1, 0x10d8));
                                    c.g[A1][0] = 0x500000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
                                    c.g[A0][0] = c.lh(at(c, A2, 0x110));
                                    c.g[V0][0] = c.lw(at(c, A2, 0x114));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x002f3f4c)?;
                                    c.g[V1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[A0][0] = 0x500000;
                                    c.g[T2][0] = 0xfffffffffffffc1f;
                                    c.g[A1][0] = 0xfffffffffe7f0000;
                                    c.ldl(2, at(c, V1, 0x7));
                                    c.ldr(2, (c.g[V1][0] as u32));
                                    c.ldl(6, at(c, V1, 0xf));
                                    c.ldr(6, at(c, V1, 0x8));
                                    c.g[A3][0] = c.lw(at(c, V1, 0x10));
                                    c.sdl(2, at(c, V1, 0x1b));
                                    c.sdr(2, at(c, V1, 0x14));
                                    c.sdl(6, at(c, V1, 0x23));
                                    c.sdr(6, at(c, V1, 0x1c));
                                    c.sw(at(c, V1, 0x24), c.g[A3][0]);
                                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                                    c.g[T3][0] = 0x1000000;
                                    c.g[A2][0] = 0xffffffffffcf0000;
                                    c.g[V1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[A2][0] = c.g[A2][0] | 0xffff;
                                    c.g[A3][0] = 0xfffffffffff00000;
                                    c.g[T4][0] = 0xfffffffffffffffc;
                                    c.g[V0][0] = sx(at(c, V1, 0x14));
                                    c.g[A3][0] = c.g[A3][0] | 0xfff;
                                    c.sw(at(c, S1, 0xe84), c.g[V0][0]);
                                    c.g[T0][0] = 0xffffffffe0000000;
                                    c.g[V0][0] = sx(at(c, A0, 0x1420));
                                    c.ldl(9, at(c, V0, 0x7));
                                    c.ldr(9, (c.g[V0][0] as u32));
                                    c.ldl(13, at(c, V0, 0xf));
                                    c.ldr(13, at(c, V0, 0x8));
                                    c.g[T6][0] = c.lw(at(c, V0, 0x10));
                                    c.sdl(9, at(c, V1, 0x1b));
                                    c.sdr(9, at(c, V1, 0x14));
                                    c.sdl(13, at(c, V1, 0x23));
                                    c.sdr(13, at(c, V1, 0x1c));
                                    c.sw(at(c, V1, 0x24), c.g[T6][0]);
                                    c.g[T0][0] = c.g[T0][0] | 0x3ff;
                                    c.g[T1][0] = 0xffffffffffbf0000;
                                    c.g[T5][0] = 0x400000;
                                    c.g[V1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[T1][0] = c.g[T1][0] | 0xffff;
                                    c.g[V0][0] = c.lw(at(c, V1, 0x8));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                                    c.g[V0][0] = c.g[V0][0] | 0x100;
                                    c.sw(at(c, V1, 0x8), c.g[V0][0]);
                                    c.g[A0][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, A0, 0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                                    c.sw(at(c, A0, 0x4), c.g[V0][0]);
                                    c.g[V1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, V1, 0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                                    c.sw(at(c, V1, 0x4), c.g[V0][0]);
                                    c.g[A0][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, A0, 0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                    c.sw(at(c, A0, 0x4), c.g[V0][0]);
                                    c.g[V1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, V1, 0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                                    c.sw(at(c, V1, 0x4), c.g[V0][0]);
                                    c.g[A0][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, A0, 0x8));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                    c.sw(at(c, A0, 0x8), c.g[V0][0]);
                                    c.g[A1][0] = c.lw(at(c, S1, 0xe84));
                                    c.g[V0][0] = c.lw(at(c, A1, 0x4));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
                                    c.sw(at(c, A1, 0x4), c.g[V0][0]);
                                    c.g[V1][0] = c.lw(at(c, S3, 0x14));
                                    if (c.g[V1][0] as i64) <= 0 {
                                        c.g[V0][0] = c.lw(at(c, S3, 0x18));
                                        break 'b_2f409c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[S0][0] = sx(at(c, S3, 0x1c));
                                        'l_2f4078: loop {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = sx(stk(c, 0x10));
                                            call_ptr(c, 0x002f2c30, 0x002f4084)?; // cSplashManager_RenderDrop
                                            c.g[S2][0] = sx(at(c, S2, 0x1));
                                            c.g[V0][0] = c.lw(at(c, S3, 0x14));
                                            c.g[V0][0] =
                                                ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S0][0] = sx(at(c, S0, 0x3c));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002f4078);
                                                }
                                                continue 'l_2f4078;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw(at(c, S3, 0x18));
                                                break 'b_2f409c;
                                            }
                                        }
                                    }
                                }
                                'b_2f40c8: {
                                    c.cycles += 2;
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[S2][0] = 0u64;
                                    if t {
                                        break 'b_2f40c8;
                                    } else {
                                        c.cycles += 1;
                                        c.g[S0][0] = sx(at(c, S3, 0x724));
                                        'l_2f40a8: loop {
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = sx(stk(c, 0x10));
                                            call_ptr(c, 0x002f3418, 0x002f40b4)?; // cSplashManager_RenderCrystal
                                            c.g[S2][0] = sx(at(c, S2, 0x1));
                                            c.g[V0][0] = c.lw(at(c, S3, 0x18));
                                            c.g[V0][0] =
                                                ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S0][0] = sx(at(c, S0, 0x4c));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002f40a8);
                                                }
                                                continue 'l_2f40a8;
                                            } else {
                                                break 'b_2f40c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 13;
                                c.g[V0][0] = c.lw(at(c, S1, 0xe84));
                                c.g[A1][0] = c.lw(at(c, S1, 0x10d8));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                                c.sw(at(c, S1, 0xe84), c.g[V0][0]);
                                c.g[A0][0] = c.lh(at(c, A1, 0x100));
                                c.g[V1][0] = c.lw(at(c, A1, 0x104));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x002f40e8)?;
                                c.g[V0][0] = c.lw(at(c, S1, 0x10d8));
                                c.g[A0][0] = c.lh(at(c, V0, 0xb0));
                                c.g[V1][0] = c.lw(at(c, V0, 0xb4));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x002f40fc)?;
                                break 'b_2f40fc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
                break 'b_2f4100;
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x50);
            break 'b_2f4104;
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------- RiderBase_RenderAttackFX

/// `void RiderBase_RenderAttackFX(AttackFX* this)` (0x2f1510): while the
/// attack effect is active (`this+0xa0`), draw its four sprites. Pushes a
/// GS state record (copies the current 20-byte record in the graphics
/// manager's packet, `gfx+0xe84`, `gfx = gp-0x854`), loads the template at
/// 0x501420 into it, sets the frame buffer (`gfx+0xfb0`) and patches its
/// blend/test fields; then per sprite `i` the display's `DrawSprite`
/// (virtual +0x270/+0x274) with position `this+0x10+16i`, size `(s, s)`
/// (`s = this+0x90+4i`), uv `(0, 0)..(1, 1)` and colour `this+0x50+16i`;
/// finally pops the record.
///
/// The register use (the mask constants in a2..t7, the GPR temporaries of
/// the record copies) and the blocks follow the original's.
pub fn rider_base_render_attack_fx(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xb0);
        'b_2f173c: {
            c.cycles += 13;
            c.g[T8][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save_ra(c, 0x30);
            frame.save_fpr(c, 20, 0xa0);
            c.g[V0][0] = c.lw(at(c, T8, 0xa0));
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw(gpo(c, -0x854));
            if t {
                break 'b_2f173c;
            } else {
                c.cycles += 99;
                c.g[A1][0] = 0x500000;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[T0][0] = 0xffffffffffcf0000;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[T0][0] = c.g[T0][0] | 0xffff;
                c.g[T1][0] = 0xfffffffffff00000;
                c.ldl(4, at(c, V0, 0x7));
                c.ldr(4, (c.g[V0][0] as u32));
                c.ldl(7, at(c, V0, 0xf));
                c.ldr(7, at(c, V0, 0x8));
                c.g[T2][0] = c.lw(at(c, V0, 0x10));
                c.sdl(4, at(c, V0, 0x1b));
                c.sdr(4, at(c, V0, 0x14));
                c.sdl(7, at(c, V0, 0x23));
                c.sdr(7, at(c, V0, 0x1c));
                c.sw(at(c, V0, 0x24), c.g[T2][0]);
                c.g[T1][0] = c.g[T1][0] | 0xfff;
                c.g[A3][0] = 0x10000;
                c.g[T4][0] = 0xfffffffffffffffc;
                c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                c.g[A3][0] = c.g[A3][0] | 0x4000;
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[T2][0] = 0xffffffffe0000000;
                c.g[V0][0] = sx(at(c, V0, 0x14));
                c.g[T2][0] = c.g[T2][0] | 0x3ff;
                c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                c.g[T5][0] = 0xfffffffffffffc1f;
                c.g[T6][0] = 0xffffffffffffff83;
                c.g[T3][0] = 0xffffffffffbf0000;
                c.g[V0][0] = c.lw(at(c, A0, 0xe84));
                c.g[T3][0] = c.g[T3][0] | 0xffff;
                c.g[T7][0] = 0x400000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[A0][0] = sx(at(c, A1, 0x1420));
                c.ldl(16, at(c, A0, 0x7));
                c.ldr(16, (c.g[A0][0] as u32));
                c.ldl(17, at(c, A0, 0xf));
                c.ldr(17, at(c, A0, 0x8));
                c.g[V1][0] = c.lw(at(c, A0, 0x10));
                c.sdl(16, at(c, V0, 0x7));
                c.sdr(16, (c.g[V0][0] as u32));
                c.sdl(17, at(c, V0, 0xf));
                c.sdr(17, at(c, V0, 0x8));
                c.sw(at(c, V0, 0x10), c.g[V1][0]);
                c.g[S2][0] = sx(at(c, T8, 0x10));
                c.g[S5][0] = sx(stk(c, 0x10));
                c.g[S4][0] = sx(stk(c, 0x20));
                c.g[A0][0] = c.lw(gpo(c, -0x854));
                c.g[S1][0] = sx(at(c, T8, 0x50));
                c.g[S0][0] = sx(at(c, T8, 0x90));
                c.g[S3][0] = 0x3;
                c.g[V1][0] = c.lw(at(c, A0, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0xfb0));
                c.sh(at(c, V1, 0x10), c.g[V0][0]);
                c.g[A1][0] = c.lw(gpo(c, -0x854));
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] | 0xc;
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A2, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw(at(c, A2, 0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.sw(at(c, V1, 0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                c.g[V0][0] = c.g[V0][0] | 0xe0;
                c.sw(at(c, A0, 0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, V1, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x1c;
                c.sw(at(c, V1, 0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, A1, 0xe84));
                c.g[V0][0] = c.lw(at(c, A0, 0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
                c.sw(at(c, A0, 0x4), c.g[V0][0]);
                c.g[V0][0] = c.lw(gpo(c, -0x854));
                'l_2f16d0: loop {
                    c.cycles += 23;
                    c.g[A1][0] = c.g[S2][0];
                    c.lwc1(0, (c.g[S0][0] as u32));
                    c.g[T1][0] = c.g[S1][0];
                    c.g[V1][0] = c.lw(at(c, V0, 0x10d8));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = c.g[S4][0];
                    c.g[A0][0] = c.lh(at(c, V1, 0x270));
                    c.g[S1][0] = sx(at(c, S1, 0x10));
                    c.swc1(0, stk(c, 0x4));
                    c.g[S0][0] = sx(at(c, S0, 0x4));
                    c.swc1(0, stk(c, 0));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw(stk(c, 0x10), 0u64);
                    c.g[S2][0] = sx(at(c, S2, 0x10));
                    c.sw(stk(c, 0x14), 0u64);
                    c.swc1(20, stk(c, 0x20)); // uv max (1, 1)
                    c.swc1(20, stk(c, 0x24));
                    c.g[V0][0] = c.lw(at(c, V1, 0x274));
                    let t = c.g[V0][0] as u32;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, t, 0x002f1724)?; // display->DrawSprite
                    let t = (c.g[S3][0] as i64) >= 0;
                    c.g[V0][0] = c.lw(gpo(c, -0x854));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f16d0);
                        }
                        continue 'l_2f16d0;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(gpo(c, -0x854));
                        c.g[V0][0] = c.lw(at(c, V1, 0xe84));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                        c.sw(at(c, V1, 0xe84), c.g[V0][0]);
                        break 'b_2f173c;
                    }
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- 0x00131620 RiderCruiseSub_HandleInput --------------------------------

    mod cruise_input {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

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
        /// The static animation table (28-byte entries; `+0` the kind
        /// `AnimController_GetLayerAnimEntry` returns).
        const ANIM_TABLE: u32 = 0x0044_6990;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// "No animation" (kind 0 without a table lookup).
        const NO_ANIM: u32 = 0x1b6;
        const FAKE_BE: u32 = FAKE;
        /// The controller's "current animation kind": 0x207 makes
        /// `AnimController_PlayAnim` stop early.
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed stick field: centred a third of the time.
        fn axis(r: &mut Rng) -> u32 {
            if r.next_u32() % 3 == 0 {
                0
            } else {
                r.next_u32() & 0x3f
            }
        }

        /// A float that is 0 a third of the time.
        fn maybe0(r: &mut Rng, range: f32) -> u32 {
            if r.next_u32() % 3 == 0 {
                0
            } else {
                r.f32(range)
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_1620, 0);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // The behaviour library and the world chain (as in the spin
            // sub-state's fixture).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE_BE);
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD_84);
            w(c, WORLD_84 + 0x44, WORLD_TABLE);

            // The pad: never reset (12) or the cheat jump (14); the other
            // buttons and the axes at random.
            let buttons = (r.next_u32() & !0xfff0_5000) | axis(r) << 20 | axis(r) << 26;
            let sticks = axis(r) | axis(r) << 6 | (r.next_u32() & 0xffff_f000);
            w(c, PAD, buttons);
            w(c, PAD + 4, sticks);
            w(c, THIS, RIDER);

            // The rider: main state 2 (not in the air: no airborne switch,
            // no rail search), racing (finish timer -1), jump re-armed (no
            // prewind), board-press blocked (+0x328 = 3), no timed event,
            // the idle clock at 0.
            w(c, RIDER + 0x77c, STATE);
            w(c, STATE + 0xde0, 2);
            w(c, RIDER + 0x470, f(-1.0));
            w(c, RIDER + 0x360, f(1.0));
            w(c, RIDER + 0x328, 3 + r.next_u32() % 2);
            w(c, RIDER + 0x358, 0);
            w(c, RIDER + 0x35c, 0);
            w(c, RIDER + 0x784, CTRL);
            w(c, RIDER + 0x788, ANIM_STATE);
            w(c, RIDER + 0x790, METRIX);
            w(c, RIDER + 0x860, OVERLAPS);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x2f4, r.next_u32() % 10);
            w(c, RIDER + 0x86c, 0);
            w(c, RIDER + 0x28c, maybe0(r, 1.0));
            w(c, RIDER + 0x298, maybe0(r, 1.0));
            // The fields the animation choice reads.
            let speed = (r.next_u32() % 2600) as f32;
            let turn = maybe0(r, 1.6);
            let spin = maybe0(r, 1.0);
            // Backwards only when `Rider_TurnAroundIfBackward` (spin speed 0)
            // gives up on its own (turn != 0): it would rebuild the matrix.
            let backwards = (turn != 0 || spin != 0) && r.next_u32() % 3 == 0;
            put_vec(c, RIDER + 0x1e0, [f(speed * 0.6), f(speed * 0.8), 0, 0]);
            let s = if backwards { -1.0 } else { 1.0 };
            put_vec(
                c,
                RIDER + 0x3a0,
                [f(0.6 * s), f(0.8 * s), 0, f(r.next_u32() as f32 * 1e-9)],
            );
            w(c, RIDER + 0x214, turn);
            w(c, RIDER + 0x2dc, spin);
            w(c, RIDER + 0x1f0, maybe0(r, 1.0));
            w(c, RIDER + 0x1fc, maybe0(r, 1.0));
            w(c, RIDER + 0x220, maybe0(r, 1.0));
            w(c, RIDER + 0x3b8, maybe0(r, 1.0));
            w(c, RIDER + 0x320, r.next_u32() % 2);
            w(
                c,
                RIDER + 0x438,
                [2, 3, 4, 4, 0, 1][(r.next_u32() % 6) as usize],
            );
            put_vec(c, RIDER + 0x180, [f(1.0), 0, 0, 0]);
            put_vec(c, ANIM_STATE + 0x20, [f(-1.0), 0, 0, 0]);
            w(c, METRIX + 0x14, r.f32(1.0));

            // The animation controller: layers 0/1 idle ("no animation"),
            // layer 2 on 0x16, 0x15, a choice result (0x5..0x11) or another
            // id, whose table kind is 10, 5 or other.
            let id = match r.next_u32() % 5 {
                0 => 0x16,
                1 => 0x15,
                2 => 0x200,
                _ => 0x5 + r.next_u32() % 0xd,
            };
            w(c, CTRL, NO_ANIM);
            w(c, CTRL + 4, NO_ANIM);
            w(c, CTRL + 8, id);
            const KINDS: [u32; 5] = [10, 5, 0, 2, 0];
            w(
                c,
                ANIM_TABLE + id * 0x1c,
                KINDS[(r.next_u32() % 5) as usize],
            );
            w(c, CTRL + 0x1c, f(1.0));
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            for layer in 0..3 {
                w(c, SEQ_LISTS + 8 * layer + 4, SEQ);
            }
            w(c, SEQ + 0xc0, r.next_u32() % 2);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            // The generated body (no override slot yet) dispatches on `pc`.
            c.pc = 0x0013_1620;
        }

        /// `RiderCruiseSub_HandleInput` against the generated body: random
        /// pad input (handplant attempts, jumps, tweaks, boosts), moving
        /// forward or backwards at speeds around every threshold, turning or
        /// not, every surface class and layer-2 animation (0x16, 0x15, kinds
        /// 10 and 5, the choice results), with the real callees on short
        /// paths (no state change: the rider is never airborne, re-armed,
        /// board-press blocked, racing).
        #[test]
        fn rider_cruise_sub_handle_input_matches() {
            check(
                ssx3_ee::s00::f_00131620__generated,
                rider_cruise_sub_handle_input,
                300,
                &setup,
            );
        }
    }

    // -- 0x00376c58 PS2GraphicsMan_SetPerspective -----------------------------

    /// The fake-call hook with sign-extended results, so a forced result
    /// with bit 31 set reads negative as a 64-bit value.
    fn set_perspective_fake(c: &mut Cpu, target: u32) {
        crate::difftest::fakes::fake_call(c, target);
        c.g[V0][0] = sx(c.g[V0][0] as u32);
    }

    /// `PS2GraphicsMan_SetPerspective`: a graphics manager at DATA whose
    /// display has fake width/height methods (forced results reach the
    /// halving paths of all six conversions; some calls unwind), both depth
    /// modes, both view slots, every clamp mode, plausible and edge-case
    /// near/far; the real `Math_SinCos`, `tanf` and `SetScissorClamped` run
    /// (the game image provides their constants).
    #[test]
    fn ps2_graphics_man_set_perspective_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{elf_image, put_words, DATA};
        fn f(x: f32) -> u32 {
            x.to_bits()
        }
        fn uf(r: &mut crate::difftest::Rng, lo: f32, hi: f32) -> u32 {
            f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
        }
        const DISPLAY: u32 = 0x10d8;
        const CUR_VIEW: u32 = 0x10dc;
        const MODE: u32 = 0x5a40;
        const VIEWS: u32 = 0x6d20;
        const VIEW_STRIDE: u32 = 0x230;
        covered(
            ee::s12::f_00376c58__generated,
            ps2_graphics_man_set_perspective,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x37_6c58, 6);
                c.hooks.miss = set_perspective_fake;
                c.pc = 0x0037_6c58;
                let gfx = DATA;
                // Display: {delta 0, width()} at +0x50, {delta 0, height()} at +0x58.
                let display = DATA + 0x8000;
                c.bus.write16(display + 0x50, 0);
                c.bus.write32(display + 0x54, FAKE);
                c.bus.write16(display + 0x58, 0);
                c.bus.write32(display + 0x5c, FAKE + 0x10);
                c.bus.write32(gfx + DISPLAY, display);
                let idx = seed % 2;
                c.bus.write32(gfx + CUR_VIEW, idx);
                let mode = if seed % 3 == 0 { 0x31 } else { 0x10 + seed % 4 };
                c.bus.write32(gfx + MODE, mode);
                // The view rectangle (x, y, w, h) and the view's +0x1c.
                let view = gfx + VIEWS + idx * VIEW_STRIDE;
                let rect = [
                    uf(&mut r, -64.0, 64.0),
                    uf(&mut r, -64.0, 64.0),
                    uf(&mut r, 1.0, 640.0),
                    uf(&mut r, 1.0, 480.0),
                ];
                put_words(c, view, &rect);
                c.bus.write32(view + 0x1c, uf(&mut r, 0.0, 4.0));
                // The clamp mode (0..2 scale the focal lengths, <0 and >=3
                // do not) and the ratios.
                let clamp = match seed % 4 {
                    0 => 0,
                    1 => 2,
                    2 => 3,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(gfx + 0x6b94, clamp);
                c.bus.write32(gfx + 0x6b98, f(0.75));
                c.bus.write32(gfx + 0x6b9c, f(0.5));
                c.bus.write32(gfx + 0x6ba0, uf(&mut r, 0.5, 1.5));
                c.bus.write32(gfx + 0x6ba4, uf(&mut r, 0.5, 1.5));
                // Forced display sizes for the six calls (height, width,
                // width, height, width, height): bit 31 takes the halving
                // path of the raw conversions (calls 0, 1, 4, 5), bit 28
                // that of the `<< 3` ones (calls 2, 3); 0 keeps the fake's
                // value.
                let forced: [u32; 6] = match seed % 4 {
                    1 => [
                        0x8000_01e1,
                        0x8000_0280,
                        0x1000_0140,
                        0x1000_00f0,
                        0x8000_0281,
                        0x8000_01e0,
                    ],
                    2 => [0x8000_0141, 0, 0x1000_0280, 0, 0x8000_0141, 0],
                    3 => [0, 0x8000_0101, 0, 0x1000_0100, 0, 0x8000_0101],
                    _ => [0x1e0, 0x280, 0x140, 0xf0, 0x280, 0x1e0],
                };
                put_words(c, LOG_RESULTS, &forced);
                // fov, near, far.
                c.f[12] = uf(&mut r, 0.2, 2.5);
                if seed % 6 == 5 {
                    c.f[13] = r.edge_f32();
                    c.f[14] = r.edge_f32();
                } else {
                    c.f[13] = uf(&mut r, 0.05, 20.0);
                    c.f[14] = uf(&mut r, 50.0, 20000.0);
                }
                c.g[A0][0] = sx(gfx);
            },
        );
    }

    // -- 0x0013ccf0 RiderGroundState_ComputeDrag ------------------------------

    /// `RiderGroundState_ComputeDrag`: speeds below, between and beyond the
    /// four curve points, every ramp band, the surface classes (2, 3, 13,
    /// other), climbing or not, stance mismatches with and without boost;
    /// the behaviour library's interface getter is a fake (some calls
    /// unwind), the attribute-factor callees are real on the game image's
    /// tables (values > 0, or <= 0 for the `attrib / 5` default).
    #[test]
    fn rider_ground_state_compute_drag_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{elf_image, put_words, DATA, GAME_GP};
        fn f(x: f32) -> u32 {
            x.to_bits()
        }
        fn uf(r: &mut crate::difftest::Rng, lo: f32, hi: f32) -> u32 {
            f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
        }
        const THIS: u32 = DATA;
        const CFG: u32 = DATA + 0x400;
        const RIDER: u32 = DATA + 0x1000;
        const BE_LIBRARY: u32 = DATA + 0x3000;
        const BE_OBJ: u32 = DATA + 0x3100;
        const BE_VT: u32 = DATA + 0x3200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        const CHARACTER_MAX: u32 = 0x0053_08d8;
        covered(
            ee::s01::f_0013ccf0__generated,
            rider_ground_state_compute_drag,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x13_ccf0, 3);
                let r = &mut r;
                c.pc = 0x0013_ccf0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The behaviour library: interface getter (vtable slot +8:
                // delta 0, method a fake).
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, BE_INTERFACES, BE_OBJ);
                w(c, BE_OBJ + 4, BE_VT);
                c.bus.write16(BE_VT + 8, 0);
                w(c, BE_VT + 0xc, FAKE);
                // Character maxima (non-zero divisors).
                for ch in 0..16 {
                    for n in 0..7 {
                        c.bus.write8(
                            CHARACTER_MAX + ch * 15 + 8 + n,
                            1 + (r.next_u32() % 20) as u8,
                        );
                    }
                }

                // The state: its rider and the surface depth divisor.
                w(c, THIS + 0x18, RIDER);
                w(c, THIS + 0x8, uf(r, 1.0, 50.0));
                // The curve: four increasing x with their y.
                let mut x = 0.0f32;
                for i in 0..4 {
                    x += 5.0 + (r.next_u32() % 20) as f32;
                    put_words(c, CFG + 0x90 + 8 * i, &[f(x), uf(r, 0.0, 30.0)]);
                }
                // The rider.
                w(
                    c,
                    RIDER + 0x214,
                    if seed % 5 == 0 { 0 } else { uf(r, -1.5, 1.5) },
                );
                w(c, RIDER + 0x86c, r.next_u32() % 6);
                w(
                    c,
                    RIDER + 0xb34,
                    if seed % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 5
                    },
                );
                w(c, RIDER + 0x378, uf(r, 0.0, 0.3));
                w(
                    c,
                    RIDER + 0x438,
                    [2, 3, 13, 13, 0, 7][(r.next_u32() % 6) as usize],
                );
                w(c, RIDER + 0x454, uf(r, -5.0, 5.0));
                w(c, RIDER + 0x1e8, uf(r, -10.0, 10.0));
                w(c, RIDER + 0x3d8, uf(r, -10.0, 10.0));
                w(c, RIDER + 0x320, r.next_u32() % 2);
                w(c, RIDER + 0x324, r.next_u32() % 2);
                w(
                    c,
                    RIDER + 0x2fc,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        uf(r, 0.0, 1.0)
                    },
                );

                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(CFG);
                // Speeds across the curve (|speed| * 0.036 vs the x points).
                c.f[12] = if seed % 9 == 0 {
                    r.edge_f32()
                } else {
                    uf(r, -2500.0, 2500.0)
                };
            },
        );
    }

    // -- 0x00411cd8 _fpadd_parts_d --------------------------------------------

    /// `_fpadd_parts_d` against the generated body: every class pair (NaNs,
    /// zeros, infinities of equal and opposite signs, numbers), exponents
    /// equal, close (alignment shifts both ways) and 64 or more apart,
    /// same and opposite signs (sums that carry past bit 60, differences
    /// that cancel partly or completely and normalise left).
    #[test]
    fn fpadd_parts_d_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const A: u32 = DATA;
        const B: u32 = DATA + 0x40;
        const TMP: u32 = DATA + 0x80;
        check(ee::s15::f_00411cd8__generated, fpadd_parts_d, 400, &|c, seed| {
            let mut r = Rng(0x0041_1cd8 ^ (seed as u64) << 24 ^ 0x5bd1_e995);
            scramble(c, &mut r);
            c.pc = 0x0041_1cd8;
            let class = |r: &mut Rng| match r.next_u32() % 10 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                _ => 3,
            };
            let frac = |r: &mut Rng| {
                let f = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                match r.next_u32() % 4 {
                    0 => f, // anything (NaN payloads, unnormalised)
                    _ => (f & ((1u64 << 60) - 1)) | (1u64 << 60),
                }
            };
            let ea = (r.next_u32() % 200) as i32 - 100;
            let eb = match r.next_u32() % 4 {
                0 => ea,
                1 => ea + (r.next_u32() % 70) as i32 - 35,
                2 => ea + 64 + (r.next_u32() % 10) as i32,
                _ => ea - 64 - (r.next_u32() % 10) as i32,
            };
            let fa = frac(&mut r);
            // Near-cancellation: b's fraction close to a's.
            let fb = if seed % 5 == 0 {
                fa ^ (r.next_u32() as u64 & 0xffff)
            } else {
                frac(&mut r)
            };
            for (p, cl, e, fr) in [(A, class(&mut r), ea, fa), (B, class(&mut r), eb, fb)] {
                c.bus.write32(p, cl);
                c.bus.write32(p + 4, r.next_u32() % 2);
                c.bus.write32(p + 8, e as u32);
                c.bus.write64(p + 0x10, fr);
            }
            if seed % 7 == 0 {
                // Equal operands of opposite sign: exact cancellation.
                let (cl, sign, e) = (c.bus.read32(A), c.bus.read32(A + 4), c.bus.read32(A + 8));
                let fr = c.bus.read64(A + 0x10);
                c.bus.write32(B, cl);
                c.bus.write32(B + 4, sign ^ 1);
                c.bus.write32(B + 8, e);
                c.bus.write64(B + 0x10, fr);
            }
            for k in 0..6 {
                c.bus.write32(TMP + 4 * k, r.next_u32());
            }
            c.g[A0][0] = sx(A);
            c.g[A1][0] = sx(B);
            c.g[A2][0] = sx(TMP);
        });
    }

    // -- 0x002bd1b8 tWPIGD_ScreenTint_Blend -----------------------------------

    /// `tWPIGD_ScreenTint_Blend`: the tint at a random data address with six
    /// random `{current, target}` pairs (edge-case floats on some), a
    /// keyframe with random values (on one seed in seven it overlaps the
    /// tint's pairs, which checks the load/store order), t (f12) in
    /// [-1.2, 1.2) or an edge case, and a vtable whose hook is a fake
    /// (random this-delta) that unwinds on either call on some seeds.
    #[test]
    fn twpigd_screen_tint_blend_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, FAKE};
        use screen_tint::*;
        covered(
            ee::s08::f_002bd1b8__generated,
            twpigd_screen_tint_blend,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_d1b8, 2);
                c.pc = 0x002b_d1b8;
                let this = ptr(&mut r) & !0xfff;
                for i in 0..12 {
                    let v = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(500.0)
                    };
                    c.bus.write32(this + PAIRS + 4 * i, v);
                }
                let key = if seed % 7 == 0 {
                    this + 4 * (r.next_u32() % 12)
                } else {
                    let k = this + 0x400;
                    for i in 0..6 {
                        let v = if r.next_u32() % 8 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(500.0)
                        };
                        c.bus.write32(k + KEY_VALUES + 4 * i, v);
                    }
                    k
                };
                let vt = this + 0x800;
                c.bus
                    .write16(vt + VT_HOOK_DELTA, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + VT_HOOK, FAKE);
                c.bus.write32(this + VPTR, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = key as u64;
                c.f[12] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.2)
                };
            },
        );
    }

    // -- 0x001668b8 ChaseCameraAlgorithm_RotateByPitch ------------------------

    /// `ChaseCameraAlgorithm_RotateByPitch`: random and edge-case angles at
    /// +0x50/+0x54 (the game image provides the identity, the axes and
    /// `Math_SinCos`'s tables); the stack scratch starts random.
    #[test]
    fn chase_camera_algorithm_rotate_by_pitch_matches() {
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA, SP};
        const THIS: u32 = DATA;
        const OUT: u32 = DATA + 0x100;
        check(
            ee::s02::f_001668b8__generated,
            chase_camera_algorithm_rotate_by_pitch,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0016_68b8 ^ (seed as u64) << 20 ^ 0x2545_f491);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x0016_68b8;
                fill(c, &mut r, SP - 0x140, 0x50);
                let angle = |r: &mut Rng| {
                    if r.next_u32() % 8 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(7.0)
                    }
                };
                let pitch = angle(&mut r);
                let yaw = angle(&mut r);
                c.bus.write32(THIS + 0x50, pitch);
                c.bus.write32(THIS + 0x54, yaw);
                fill(c, &mut r, OUT, 4);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OUT);
            },
        );
    }

    // -- 0x002f3e28 cSplashManager_Render -------------------------------------

    /// `cSplashManager_Render`: the splash flags on and off, the manager in
    /// and out of the current view, the hand-over virtual, and the full
    /// render with 0..3 drops and crystals (the real `RenderDrop` /
    /// `RenderCrystal` draw through the same fake display methods); the
    /// display virtuals are fakes (some calls unwind).
    #[test]
    fn c_splash_manager_render_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const THIS: u32 = DATA;
        const VT2: u32 = DATA + 0x1000;
        const APP: u32 = DATA + 0x1100;
        const M1: u32 = DATA + 0x1200;
        const M2: u32 = DATA + 0x1300;
        const GFX: u32 = DATA + 0x2_0000;
        const DISP: u32 = DATA + 0x3_0000;
        const PKT: u32 = DATA + 0x4_0000;
        const TEMPLATE: u32 = 0x0050_1420;
        covered(
            ee::s09::f_002f3e28__generated,
            c_splash_manager_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_3e28, 10);
                let r = &mut r;
                c.pc = 0x002f_3e28;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let gp = |o: i32| GAME_GP.wrapping_add(o as u32);
                // Splash flags (mostly on), the hand-over flag (mostly off).
                w(c, gp(0x115c), (seed % 9 != 0) as u32);
                w(c, gp(0x1160), (seed % 11 != 0) as u32);
                w(c, gp(0xa70), (seed % 7 == 0) as u32);
                // The current view id and the manager's.
                w(c, gp(-0x848), APP);
                w(c, APP + 0x84, M1);
                w(c, M1 + 0x84, M2);
                let view = r.next_u32() % 4;
                w(c, M2 + 0x14, view);
                w(c, THIS + 0x10, if seed % 8 == 0 { view + 1 } else { view });
                w(c, THIS + 0xc, VT2);
                c.bus.write16(VT2 + 0x70, (r.next_u32() as i16 & !3) as u16);
                w(c, VT2 + 0x74, FAKE + 0x800);
                // The graphics manager, its display (every virtual a fake),
                // the packet and the GS template.
                w(c, gp(0x2a90), GFX);
                w(c, gp(-0x854), GFX);
                w(c, GFX + 0x10d8, DISP);
                // (`gfx+0x10d8` points straight at the method slots.)
                for k in 0..0x80 {
                    c.bus.write16(DISP + 8 * k, 0);
                    w(c, DISP + 8 * k + 4, FAKE + 0x10 * k);
                }
                w(c, GFX + 0xe84, PKT + 4 * (r.next_u32() % 8));
                w(c, GFX + 0x1058, r.next_u32());
                fill(c, r, PKT, 0x40);
                fill(c, r, TEMPLATE, 5);
                // The render-once flags of the drop/crystal renderers.
                for o in [0x2408, 0x2418, 0x2428, 0x2438] {
                    w(c, gp(o), r.next_u32() % 2);
                }
                // Drops and crystals.
                w(c, THIS + 0x14, r.next_u32() % 4);
                w(c, THIS + 0x18, r.next_u32() % 4);
                for i in 0..(3 * 0x3c / 4) {
                    w(c, THIS + 0x1c + 4 * i, r.f32(100.0));
                }
                for i in 0..(3 * 0x4c / 4) {
                    w(c, THIS + 0x724 + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    // -- 0x002f1510 RiderBase_RenderAttackFX ----------------------------------

    /// `RiderBase_RenderAttackFX`: inactive and active effects with random
    /// sprites, a random packet record and template, the display's
    /// `DrawSprite` a fake (some calls unwind).
    #[test]
    fn rider_base_render_attack_fx_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{fill, DATA, GAME_GP};
        const THIS: u32 = DATA;
        const GFX: u32 = DATA + 0x1_0000;
        const DISP: u32 = DATA + 0x2_0000;
        const PKT: u32 = DATA + 0x3_0000;
        const TEMPLATE: u32 = 0x0050_1420;
        covered(
            ee::s09::f_002f1510__generated,
            rider_base_render_attack_fx,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_1510, 4);
                let r = &mut r;
                c.pc = 0x002f_1510;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP.wrapping_sub(0x854), GFX);
                w(c, GFX + 0x10d8, DISP);
                c.bus
                    .write16(DISP + 0x270, (r.next_u32() as i16 & !3) as u16);
                w(c, DISP + 0x274, FAKE);
                w(c, GFX + 0xe84, PKT + 4 * (r.next_u32() % 8));
                w(c, GFX + 0xfb0, r.next_u32());
                fill(c, r, PKT, 0x40);
                fill(c, r, TEMPLATE, 5);
                fill(c, r, THIS, 0x30);
                for i in 0..4 {
                    w(c, THIS + 0x90 + 4 * i, r.f32(64.0));
                }
                w(
                    c,
                    THIS + 0xa0,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.g[A0][0] = sx(THIS);
            },
        );
    }
}
