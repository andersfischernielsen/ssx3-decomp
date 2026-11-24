//! Staging module oc491 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc491`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0016_b180,
    algorithmic_script_camera_algo_update,
    "AlgorithmicScriptCameraAlgo_Update",
)];

/// `AlgorithmicScriptCameraAlgo_Update` (`cScriptRelativeAlgorithm_update`,
/// 0x16b180): the update of an algorithmic (relative) script camera.
///
/// It reads the camera's header (`GetSSXCameraAlgorithmicHeader`, the camera
/// at `cam + 0x20`) and frame (`GetSSXCameraAlgorithmicFrame` into
/// `self.FrameInfo`), finds the subject rider (`cSSXScriptEngine_GetSubjectRider`,
/// else `SSXApp->Game->AI->Riders[0]`), and sets View, Near, Far and Roll
/// (degrees to radians, plus the script object's mBaseRotX). The optimal
/// place is Target (the rider's position plus its front and up rows times
/// the lead distances, scaled by the model's scale) minus the view direction
/// times the optimal distance; the camera's Position moves towards it, by at
/// most `max step` per second. Heading and Pitch are the yaw and elevation of
/// Target - Position, and the jitterer is updated with the focus and roll
/// jitter.
///
/// The body is the generated register and cycle order: the VU0 vector
/// sequences, the FPU scratch registers and the stack slots it writes are all
/// reproduced, because the callers and the RAM comparison observe them. The
/// `atan` (0x31c228) and `sin/cos` (0x31be50) calls and the rider's virtual
/// camera position (slot 5 of its camera object) are real calls.
pub fn algorithmic_script_camera_algo_update(c: &mut Cpu) {
    // Named offsets: the cScriptRelativeAlgorithm fields (self), the camera header,
    // the subject rider and the script object (see the reference struct).
    const RELALGO_NEAR: u32 = 0x4;
    const RELALGO_FAR: u32 = 0x8;
    const RELALGO_SCRIPT_OBJ: u32 = 0x14;
    const RELALGO_TARGET: u32 = 0x20;
    const RELALGO_POSITION: u32 = 0x30;
    const RELALGO_POSITION_W: u32 = 0x3c;
    const RELALGO_ROLL: u32 = 0x40;
    const RELALGO_HEADING: u32 = 0x44;
    const RELALGO_PITCH: u32 = 0x48;
    const RELALGO_FRAME: u32 = 0x4c;
    const RELALGO_FRAME_LEAD_HEIGHT: u32 = 0x50;
    const RELALGO_FRAME_OPT_DIST: u32 = 0x54;
    const RELALGO_FRAME_OPT_PITCH: u32 = 0x58;
    const RELALGO_FRAME_OPT_HEADING: u32 = 0x5c;
    const RELALGO_FRAME_ROLL: u32 = 0x60;
    const RELALGO_FRAME_FOV: u32 = 0x64;
    const RELALGO_FRAME_JITTER_FOCUS: u32 = 0x68;
    const RELALGO_FRAME_JITTER_ROLL: u32 = 0x6c;
    const RELALGO_JITTERER: u32 = 0x70;
    const RELHDR_FAR: u32 = 0x4;
    const RELHDR_BLEND: u32 = 0x14;
    const RELHDR_MAX_STEP: u32 = 0x18;
    const RELHDR_SUBJECT: u32 = 0x1d;
    const RELHDR_WORLD_RELATIVE: u32 = 0x1e;
    const RELRIDER_FRONT: u32 = 0x1b0;
    const RELRIDER_UP: u32 = 0x1c0;
    const RELRIDER_CAMERA_OBJ: u32 = 0x6c0;
    const RELRIDER_MODEL: u32 = 0x780;
    const RELMODEL_SCALE: u32 = 0x140;
    const SCRIPTOBJ_ROT_X: u32 = 0x70;
    const SCRIPTOBJ_ROT_Y: u32 = 0x74;
    const SCRIPTOBJ_ROT_Z: u32 = 0x78;
    const GP_SCRIPT_ENGINE: u32 = 0x84c;
    const GP_SSX_APP: u32 = 0x848;
    const GP_DEG_TO_RAD: u32 = 0x64d0;
    'b_16b1f4: {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x118));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x108));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RELALGO_SCRIPT_OBJ));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0016b1c4);
        c.g[A0][0] = c.g[S0][0];
        c.pc = 0x0027c6a0;
        call_indirect(c, 0x0027c6a0);
        if c.pc != 0x0016b1c4 {
            return;
        }
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GP_SCRIPT_ENGINE));
        c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(RELHDR_SUBJECT));
        c.g[RA][0] = sx(0x0016b1d8);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.pc = 0x00279f18;
        call_indirect(c, 0x00279f18);
        if c.pc != 0x0016b1d8 {
            return;
        }
        c.g[S2][0] = c.g[V0][0];
        let t = c.g[S2][0] != 0u64;
        c.lwc1(1, (c.g[S3][0] as u32));
        if t {
            break 'b_16b1f4;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(GP_SSX_APP));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[S2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
            break 'b_16b1f4;
        }
    }
    'b_16b3a0: {
        'b_16b390: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME));
            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(RELALGO_NEAR));
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(RELHDR_FAR));
            c.g[RA][0] = sx(0x0016b20c);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(RELALGO_FAR));
            c.pc = 0x0027c6f8;
            call_indirect(c, 0x0027c6f8);
            if c.pc != 0x0016b20c {
                return;
            }
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(GP_DEG_TO_RAD));
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_FOV));
            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_ROLL));
            c.mul_s(2, 2, 12);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RELALGO_SCRIPT_OBJ));
            c.mul_s(1, 1, 12);
            c.swc1(2, (c.g[S1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(SCRIPTOBJ_ROT_X));
            c.add_s(1, 1, 0);
            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(RELALGO_ROLL));
            c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(RELHDR_WORLD_RELATIVE));
            if c.g[V0][0] != 0u64 {
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_PITCH));
                break 'b_16b390;
            } else {
                'b_16b300: {
                    'b_16b2f8: {
                        'b_16b2f4: {
                            'b_16b2b4: {
                                c.cycles += 18;
                                c.lq(2, (c.g[S2][0] as u32).wrapping_add(RELRIDER_FRONT));
                                c.g[S0][0] = c.g[SP][0];
                                c.mtc1(0, 22);
                                c.sq(2, (c.g[SP][0] as u32));
                                c.lwc1(
                                    2,
                                    (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_PITCH),
                                );
                                c.lwc1(0, (c.g[SP][0] as u32));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.mul_s(23, 2, 12);
                                c.mul_s(2, 0, 0);
                                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.mul_s(3, 1, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.add_s(2, 2, 3);
                                c.sqrt_s(20, 2);
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                if t {
                                    break 'b_16b2b4;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(0, 21);
                                    if t {
                                        break 'b_16b2f4;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64cc));
                                        let t = !c.fcc();
                                        c.lwc1(21, (c.g[S0][0] as u32));
                                        if t {
                                            break 'b_16b2f8;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64c8));
                                            let t = true;
                                            c.sub_s(23, 0, 23);
                                            if t {
                                                break 'b_16b300;
                                            } else {
                                                break 'b_16b2b4;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_16b2f0: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                c.g[RA][0] = sx(0x0016b2c0);
                                c.pc = 0x0031c228;
                                call_indirect(c, 0x0031c228);
                                if c.pc != 0x0016b2c0 {
                                    return;
                                }
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_16b2f0;
                                } else {
                                    'b_16b2e8: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();
                                        if t {
                                            break 'b_16b2e8;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64c4));
                                            let t = true;
                                            c.add_s(12, 12, 0);
                                            if t {
                                                break 'b_16b2f0;
                                            } else {
                                                break 'b_16b2e8;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64c0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_16b2f0;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(0, 12);
                            break 'b_16b2f4;
                        }
                        c.cycles += 1;
                        c.lwc1(21, (c.g[S0][0] as u32));
                        break 'b_16b2f8;
                    }
                    c.cycles += 2;
                    c.sub_s(23, 0, 23);
                    c.mtc1(0, 22);
                    break 'b_16b300;
                }
                'b_16b388: {
                    'b_16b384: {
                        'b_16b344: {
                            c.cycles += 6;
                            c.lwc1(
                                1,
                                (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_HEADING),
                            );
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64bc));
                            c.c_eq_s(21, 22);
                            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
                            let t = !c.fcc();
                            c.mul_s(24, 1, 0);
                            if t {
                                break 'b_16b344;
                            } else {
                                c.cycles += 4;
                                c.c_eq_s(20, 22);
                                let t = c.fcc();
                                c.mov_s(0, 20);
                                if t {
                                    break 'b_16b384;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(22, 20);
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64b8));
                                    if !c.fcc() {
                                        c.add_s(20, 24, 0);
                                        break 'b_16b388;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64b4));
                                        let t = true;
                                        c.add_s(20, 24, 0);
                                        if t {
                                            break 'b_16b388;
                                        } else {
                                            break 'b_16b344;
                                        }
                                    }
                                }
                            }
                        }
                        'b_16b380: {
                            c.cycles += 7;
                            c.div_s(12, 20, 21);
                            c.g[RA][0] = sx(0x0016b350);
                            c.pc = 0x0031c228;
                            call_indirect(c, 0x0031c228);
                            if c.pc != 0x0016b350 {
                                return;
                            }
                            c.c_lt_s(21, 22);
                            let t = !c.fcc();
                            c.mov_s(12, 0);
                            if t {
                                break 'b_16b380;
                            } else {
                                'b_16b378: {
                                    c.cycles += 3;
                                    c.c_lt_s(22, 20);
                                    let t = !c.fcc();
                                    if t {
                                        break 'b_16b378;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64b0));
                                        let t = true;
                                        c.add_s(12, 12, 0);
                                        if t {
                                            break 'b_16b380;
                                        } else {
                                            break 'b_16b378;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64ac));
                                c.sub_s(12, 12, 0);
                                break 'b_16b380;
                            }
                        }
                        c.cycles += 1;
                        c.mov_s(0, 12);
                        break 'b_16b384;
                    }
                    c.cycles += 1;
                    c.add_s(20, 24, 0);
                    break 'b_16b388;
                }
                c.cycles += 2;
                let t = true;
                c.lwc1(21, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_DIST));
                if t {
                    break 'b_16b3a0;
                } else {
                    break 'b_16b390;
                }
            }
        }
        c.cycles += 4;
        c.lwc1(
            1,
            (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_HEADING),
        );
        c.mul_s(23, 0, 12);
        c.lwc1(21, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_OPT_DIST));
        c.mul_s(20, 1, 12);
        break 'b_16b3a0;
    }
    'b_16b62c: {
        'b_16b628: {
            c.cycles += 132;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(RELRIDER_MODEL));
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(RELRIDER_CAMERA_OBJ));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RELALGO_SCRIPT_OBJ));
            c.lq(2, (c.g[A0][0] as u32).wrapping_add(RELMODEL_SCALE));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(SCRIPTOBJ_ROT_Z));
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(SCRIPTOBJ_ROT_Y));
            c.sq(2, (c.g[SP][0] as u32));
            c.add_s(20, 20, 0);
            c.add_s(23, 23, 1);
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(RELRIDER_CAMERA_OBJ));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0016b3d8);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            call_indirect(c, t);
            if c.pc != 0x0016b3d8 {
                return;
            }
            c.lq(3, (c.g[S2][0] as u32).wrapping_add(RELRIDER_FRONT));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa4));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.mov_s(12, 20);
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mul_s(0, 0, 1);
            c.mfc1(6, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.qmtc2(6, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lqc2(3, (c.g[V0][0] as u32));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(1, (c.g[SP][0] as u32));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(2, (c.g[S2][0] as u32).wrapping_add(RELRIDER_UP));
            c.lwc1(
                0,
                (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_LEAD_HEIGHT),
            );
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.mul_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(2, (c.g[S1][0] as u32).wrapping_add(RELALGO_TARGET));
            c.g[RA][0] = sx(0x0016b47c);
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.pc = 0x0031be50;
            call_indirect(c, 0x0031be50);
            if c.pc != 0x0016b47c {
                return;
            }
            c.mov_s(12, 23);
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa8));
            c.g[RA][0] = sx(0x0016b48c);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
            c.pc = 0x0031be50;
            call_indirect(c, 0x0031be50);
            if c.pc != 0x0016b48c {
                return;
            }
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xac));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
            c.mul_s(1, 0, 1);
            c.mul_s(0, 0, 3);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
            c.neg_s(2, 2);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 4);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.mfc1(3, 21);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x64a8));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(RELALGO_TARGET));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(RELHDR_BLEND));
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(RELHDR_BLEND));
            c.sub_s(0, 4, 0);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x4c));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 3);
            c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(RELHDR_MAX_STEP));
            c.c_lt_s(1, 0);
            let t = !c.fcc();
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_16b628;
            } else {
                c.cycles += 30;
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x64a4));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.mul_s(0, 1, 0);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.mfc1(3, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(2, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
                c.swc1(4, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION_W));
                let t = true;
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t {
                    break 'b_16b62c;
                } else {
                    break 'b_16b628;
                }
            }
        }
        c.cycles += 1;
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
        break 'b_16b62c;
    }
    'b_16b6cc: {
        'b_16b6c4: {
            'b_16b688: {
                c.cycles += 12;
                c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(RELALGO_TARGET));
                c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(RELALGO_POSITION));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.mtc1(0, 22);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x64));
                if t {
                    break 'b_16b688;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_16b6c4;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x64a0));
                        let t = !c.fcc();
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_16b6c4;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x649c));
                            let t = true;
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
                            if t {
                                break 'b_16b6cc;
                            } else {
                                break 'b_16b688;
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            c.g[RA][0] = sx(0x0016b694);
            c.pc = 0x0031c228;
            call_indirect(c, 0x0031c228);
            if c.pc != 0x0016b694 {
                return;
            }
            c.c_lt_s(20, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_16b6c4;
            } else {
                'b_16b6bc: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();
                    if t {
                        break 'b_16b6bc;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6498));
                        let t = true;
                        c.add_s(12, 12, 0);
                        if t {
                            break 'b_16b6c4;
                        } else {
                            break 'b_16b6bc;
                        }
                    }
                }
                c.cycles += 2;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6494));
                c.sub_s(12, 12, 0);
                break 'b_16b6c4;
            }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
        break 'b_16b6cc;
    }
    'b_16b770: {
        'b_16b768: {
            'b_16b728: {
                c.cycles += 12;
                c.mul_s(2, 0, 0);
                c.swc1(12, (c.g[S1][0] as u32).wrapping_add(RELALGO_HEADING));
                c.mul_s(3, 1, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.mtc1(0, 22);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.add_s(2, 2, 3);
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.sqrt_s(20, 2);
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                if t {
                    break 'b_16b728;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(0, 21);
                    if t {
                        break 'b_16b768;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6490));
                        let t = !c.fcc();
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(RELALGO_JITTERER));
                        if t {
                            break 'b_16b768;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x648c));
                            let t = true;
                            c.lwc1(
                                12,
                                (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_JITTER_FOCUS),
                            );
                            if t {
                                break 'b_16b770;
                            } else {
                                break 'b_16b728;
                            }
                        }
                    }
                }
            }
            'b_16b764: {
                c.cycles += 7;
                c.div_s(12, 21, 20);
                c.g[RA][0] = sx(0x0016b734);
                c.pc = 0x0031c228;
                call_indirect(c, 0x0031c228);
                if c.pc != 0x0016b734 {
                    return;
                }
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_16b764;
                } else {
                    'b_16b75c: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_16b75c;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6488));
                            let t = true;
                            c.add_s(12, 12, 0);
                            if t {
                                break 'b_16b764;
                            } else {
                                break 'b_16b75c;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6484));
                    c.sub_s(12, 12, 0);
                    break 'b_16b764;
                }
            }
            c.cycles += 1;
            c.mov_s(0, 12);
            break 'b_16b768;
        }
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(RELALGO_JITTERER));
        c.lwc1(
            12,
            (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_JITTER_FOCUS),
        );
        break 'b_16b770;
    }
    c.cycles += 18;
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(RELALGO_PITCH));
    c.g[RA][0] = sx(0x0016b77c);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x00160028;
    call_indirect(c, 0x00160028);
    if c.pc != 0x0016b77c {
        return;
    }
    c.lwc1(
        12,
        (c.g[S1][0] as u32).wrapping_add(RELALGO_FRAME_JITTER_ROLL),
    );
    c.g[RA][0] = sx(0x0016b788);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x00160130;
    call_indirect(c, 0x00160130);
    if c.pc != 0x0016b788 {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x118));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x108));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as RA_ADDR, SP as SP_ADDR};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const ALG: u32 = DATA + 0x1000;
    const SCRIPT_OBJ: u32 = DATA + 0x2000;
    const CAMERA_ENTITY: u32 = DATA + 0x3000;
    const CAM: u32 = DATA + 0x4000;
    const HDR: u32 = DATA + 0x5000;
    const FRAME_SRC: u32 = DATA + 0x7000;
    const RIDER: u32 = DATA + 0x6000;
    const VTABLE: u32 = DATA + 0x8000;
    const GATE: u32 = DATA + 0x9000;
    const APP: u32 = DATA + 0xa000;
    const GAME: u32 = DATA + 0xa100;
    const AI: u32 = DATA + 0xa200;
    const MODEL: u32 = DATA + 0xb000;
    const GATE_MAGIC: u32 = 0x0491_c0de;

    static PREV_RIDER: AtomicUsize = AtomicUsize::new(0);
    static PREV_FRAME: AtomicUsize = AtomicUsize::new(0);
    static PREV_JITTER_POS: AtomicUsize = AtomicUsize::new(0);
    static PREV_JITTER_ROLL: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake: the subject rider from the fixture (0 = none, then the
    /// SSXApp fallback), recorded through the call log.
    fn fake_subject_rider(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_MAGIC {
            return fall_back(c, &PREV_RIDER, ee::s07::f_00279f18__generated);
        }
        c.g[V0][0] = c.bus.read32(GATE + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated fake: copies the fixture's frame channel values (0x24 bytes)
    /// to the output pointer in a1.
    fn fake_frame(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_MAGIC {
            return fall_back(c, &PREV_FRAME, ee::s07::f_0027c6f8__generated);
        }
        let out = c.g[A1][0] as u32;
        for i in 0..9 {
            let w = c.bus.read32(FRAME_SRC + 4 * i);
            c.bus.write32(out + 4 * i, w);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated fake: the jitterer's positional update does nothing.
    fn fake_jitter_position(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_MAGIC {
            return fall_back(c, &PREV_JITTER_POS, ee::s01::f_00160028__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated fake: the jitterer's roll update does nothing.
    fn fake_jitter_roll(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_MAGIC {
            return fall_back(c, &PREV_JITTER_ROLL, ee::s01::f_00160130__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn install_fakes() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00279F18),
            fake_subject_rider,
            &PREV_RIDER,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027C6F8),
            fake_frame,
            &PREV_FRAME,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00160028),
            fake_jitter_position,
            &PREV_JITTER_POS,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00160130),
            fake_jitter_roll,
            &PREV_JITTER_ROLL,
        );
    }

    /// A float in [lo, hi) as bits.
    fn span(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        let u = (r.next_u32() >> 8) as f32 / 16_777_216.0;
        (lo + u * (hi - lo)).to_bits()
    }

    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0491_6b18, 0);
        c.g[SP][0] = SP_ADDR as u64;
        c.g[RA][0] = RA_ADDR as u64;
        c.g[A0][0] = ALG as u64;
        c.pc = 0x0016_b180;

        // Gate: the fakes answer with the fixture; seed 0 mod 5 finds no
        // subject rider and takes the SSXApp fallback.
        c.bus.write32(GATE, GATE_MAGIC);
        c.bus
            .write32(GATE + 4, if seed % 5 == 0 { 0 } else { RIDER });

        // The algorithm and its script object, camera entity, camera.
        c.bus.write32(ALG + 0x14, SCRIPT_OBJ);
        c.bus.write32(SCRIPT_OBJ + 0x14, CAMERA_ENTITY);
        c.bus.write32(CAMERA_ENTITY + 8, CAM);
        c.bus.write32(CAM + 0x20, HDR);
        for i in 0..3 {
            c.bus
                .write32(SCRIPT_OBJ + 0x70 + 4 * i, span(&mut r, -0.5, 0.5));
        }

        // The camera header: near, far, blend, max step, subject byte and
        // the world-relative flag (both values by seed).
        c.bus.write32(HDR, span(&mut r, 0.1, 2.0));
        c.bus.write32(HDR + 4, span(&mut r, 50.0, 500.0));
        c.bus.write32(HDR + 0x14, span(&mut r, 0.0, 1.0));
        c.bus.write32(HDR + 0x18, span(&mut r, 0.0, 40.0));
        c.bus.write8(HDR + 0x1d, r.next_u32() as u8);
        c.bus.write8(HDR + 0x1e, (seed % 2) as u8);

        // The frame channel values the frame fake copies.
        let frame: [u32; 9] = [
            span(&mut r, -4.0, 4.0),
            span(&mut r, -4.0, 4.0),
            span(&mut r, 2.0, 12.0),
            span(&mut r, -30.0, 30.0),
            span(&mut r, -180.0, 180.0),
            span(&mut r, -20.0, 20.0),
            span(&mut r, 40.0, 90.0),
            span(&mut r, -0.2, 0.2),
            span(&mut r, -0.2, 0.2),
        ];
        for (i, w) in frame.iter().enumerate() {
            c.bus.write32(FRAME_SRC + 4 * i as u32, *w);
        }

        // The subject rider: its front and up rows, its camera object (a
        // vtable whose slot 5 is the miss hook) and its model's scale.
        for i in 0..4 {
            c.bus
                .write32(RIDER + 0x1b0 + 4 * i, span(&mut r, -1.0, 1.0));
            c.bus
                .write32(RIDER + 0x1c0 + 4 * i, span(&mut r, -1.0, 1.0));
            c.bus.write32(MODEL + 0x140 + 4 * i, span(&mut r, 0.5, 2.0));
        }
        c.bus.write32(RIDER + 0x6c0, VTABLE);
        c.bus.write32(RIDER + 0x780, MODEL);
        c.bus.write16(VTABLE + 0x28, 0);
        c.bus.write32(VTABLE + 0x2c, FAKE);

        // SSXApp->Game->AI->Riders[0], the fallback rider.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), APP);
        c.bus.write32(APP + 0x84, GAME);
        c.bus.write32(GAME + 0xc, AI);
        c.bus.write32(AI + 0x28, RIDER);
    }

    #[test]
    fn algorithmic_script_camera_algo_update_matches() {
        install_fakes();
        check(
            ee::s02::f_0016b180__generated,
            algorithmic_script_camera_algo_update,
            200,
            &setup,
        );
    }
}
