//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): the rider's
//! per-frame ground-state physics update.
//!
//! `RiderGroundState_Update(GroundState* gs)` (0x13d818, 6492 bytes) is the
//! "on the ground" half of the rider controller. It runs before the airborne
//! and post-update steps and produces the frame's accelerations, contact
//! frame and lean/spin outputs:
//!
//! 1. apply the wind force to the rider and clamp the direction quaternion
//!    (`rider+0x1e0`) to the maximum tilt (`rider+0x2e4`);
//! 2. integrate the two ground speeds (`gs+0x4`, `gs+0x8`) towards the track
//!    segment's reference speeds (world -> race manager -> ground entry for
//!    `rider+0x438`, 0xb0 bytes each);
//! 3. build the contact frame from the rider quaternion and blend the
//!    friction, boost, drag and carve accelerations, plus gravity and the
//!    character attribute factor, into the frame's force vector;
//! 4. resolve ground penetration and the vertical speed;
//! 5. pick the bank/lean parameters from animation state, sub-state and the
//!    global curve tables;
//! 6. probe the ground (collision volume at the rider's feet) and, on a hit,
//!    resolve the contact and correct the ground normal and velocity;
//! 7. advance the steering angle (`gs+0x0`, wrapped at 360 degrees) and
//!    rebuild the rider's world matrix.
//!
//! `gs+0x18` is the `Rider`; `gs+0xc` receives the grounded flag.
//!
//! There is no readable C++ body for this function (the reference has only
//! its assembly), so the port follows `generated/ee/s01` block for block:
//! every register write, FPU operation, cycle charge, VU0 macro op, call
//! target and return address is the original's. The VU0 sequences are issued
//! as the original issues them, so VF/ACC/Q and the VU0 flags end identical.
//! Calls use [`crate::call`]: the same callee (through the shard's override
//! slot) with `ra` = the original's return address, and the unwind check
//! after each call.

use crate::call::{call_ptr, Callee};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0013_d818,
    rider_ground_state_update,
    "RiderGroundState_Update",
)];

/// Direct call targets, as the generated body calls them (the shard alias,
/// which dispatches through its override slot).
mod callee {
    use super::{ee, Callee};

    pub const RIDER_GROUND_STATE_COMPUTE_FRICTION_ACCEL: Callee = Callee {
        addr: 0x0013_c878,
        f: ee::s01::RiderGroundState_ComputeFrictionAccel,
    };
    pub const RIDER_GROUND_STATE_COMPUTE_BOOST_ACCEL: Callee = Callee {
        addr: 0x0013_c948,
        f: ee::s01::RiderGroundState_ComputeBoostAccel,
    };
    pub const RIDER_GROUND_STATE_COMPUTE_DRAG: Callee = Callee {
        addr: 0x0013_ccf0,
        f: ee::s01::RiderGroundState_ComputeDrag,
    };
    pub const RIDER_GROUND_STATE_COMPUTE_CARVE_ACCEL: Callee = Callee {
        addr: 0x0013_d028,
        f: ee::s01::RiderGroundState_ComputeCarveAccel,
    };
    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BELIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const BECHARINT_GET_ATTRIB3_FACTOR_2: Callee = Callee {
        addr: 0x0014_8e68,
        f: ee::s01::BECharInt_GetAttrib3Factor_2,
    };
    pub const RIDER_GROUND_STATE_PROBE_GROUND: Callee = Callee {
        addr: 0x0013_d1b8,
        f: ee::s01::RiderGroundState_ProbeGround,
    };
}

// Rider / ground-state layout used below (byte offsets):
//
//   GroundState: +0 steering angle, +4/+8 ground speeds, +0xc grounded flag,
//                +0x18 Rider*.
//   Rider:       +0x110/+0x120 position/quaternion, +0x1e0 direction
//                quaternion, +0x1f0 pitch, +0x208 roll, +0x214 dead zone,
//                +0x220 grip, +0x2c0..+0x2dc lean/speed outputs,
//                +0x2e4 max tilt, +0x300 steering, +0x308 airborne flag,
//                +0x31c lean output, +0x370 contact normal,
//                +0x378 vertical speed, +0x390/+0x3a0/+0x3b0/+0x3d0 contact
//                frame, +0x3f8 drag output, +0x438 ground index,
//                +0x454 ground contact parameter, +0x758 friction output,
//                +0x77c state object, +0x780 model, +0x784 animation,
//                +0x860 overlap list, +0x864 collision group, +0x86c/+0xb34
//                character record ids, +0x874 wind enabled.
//
// The frame at sp holds the accumulated force and collision output.

/// `void RiderGroundState_Update(GroundState* gs)` (0x13d818).
pub fn rider_ground_state_update(c: &mut Cpu) {
    // --------------------------------------------------------------------------
    // Prologue: save the callee-saved state, apply wind, and clamp the
    // direction quaternion (+0x1e0) to the maximum tilt (+0x2e4) about up.
    // --------------------------------------------------------------------------
    'b_13d8cc: {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
        c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x208));
        c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1d8));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        if call_ptr(c, 0x00125970, 0x0013d870).is_err() {
            return;
        }
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 23);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x2e4));
        c.c_lt_s(0, 23);
        if !c.fcc() {
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13d8cc;
        } else {
            c.cycles += 7;
            c.div_s(0, 0, 23);
            c.mfc1(2, 0);
            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13d8cc;
        }
    }
    // --------------------------------------------------------------------------
    // Integrate the forward ground speed (gs+0x4) towards the track's
    // reference speed (ground entry +0x14), limited by gs+0x8 * 100.
    // --------------------------------------------------------------------------
    'b_13d944: {
        'b_13d930: {
            c.cycles += 23;
            c.g[A2][0] = 0xb0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x438));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.mult(0, 3, 3, 6);
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x300));
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7064));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x780));
            c.mul_s(25, 1, 0);
            c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 1);
            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
            c.lwc1(28, (c.g[V0][0] as u32).wrapping_add(0x140));
            c.mul_s(3, 25, 1);
            c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.mul_s(2, 28, 0);
            c.add_s(0, 2, 3);
            c.c_lt_s(0, 4);
            if !c.fcc() {
                c.sub_s(0, 2, 3);
                break 'b_13d930;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 4, 3);
                if t {
                    break 'b_13d944;
                } else {
                    break 'b_13d930;
                }
            }
        }
        c.cycles += 4;
        c.c_lt_s(4, 0);
        let t = !c.fcc();
        c.mov_s(1, 2);
        if t {
            break 'b_13d944;
        } else {
            c.cycles += 1;
            c.add_s(1, 4, 3);
            break 'b_13d944;
        }
    }
    // --------------------------------------------------------------------------
    // Integrate the lateral ground speed (gs+0x8) the same way
    // (ground entry +0x18).
    // --------------------------------------------------------------------------
    'b_13d98c: {
        'b_13d97c: {
            c.cycles += 12;
            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 0);
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x18));
            c.mul_s(3, 25, 0);
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.mul_s(1, 28, 1);
            c.add_s(0, 1, 3);
            c.c_lt_s(0, 2);
            if !c.fcc() {
                c.sub_s(0, 1, 3);
                break 'b_13d97c;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 2, 3);
                if t {
                    break 'b_13d98c;
                } else {
                    break 'b_13d97c;
                }
            }
        }
        c.cycles += 4;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.add_s(1, 2, 3);
            break 'b_13d98c;
        } else {
            break 'b_13d98c;
        }
    }
    // --------------------------------------------------------------------------
    // Ground frame: the rider's normal and the three ground axes projected
    // through the contact quaternion (+0x370/+0x3a0/+0x3b0), then the friction,
    // boost, drag and carve accelerations.
    // --------------------------------------------------------------------------
    'b_13dab8: {
        c.cycles += 70;
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.mov_s(14, 25);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S1][0];
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x3d0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(3, (c.g[SP][0] as u32));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x370));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(6, 4);
        c.mtc1(6, 31);
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3a0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 20);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1f0));
        c.mov_s(24, 31);
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x454));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3b0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 21);
        c.mov_s(13, 24);
        c.neg_s(27, 0);
        if callee::RIDER_GROUND_STATE_COMPUTE_FRICTION_ACCEL
            .call(c, 0x0013da40)
            .is_err()
        {
            return;
        }
        c.mov_s(22, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        if callee::RIDER_GROUND_STATE_COMPUTE_BOOST_ACCEL
            .call(c, 0x0013da50)
            .is_err()
        {
            return;
        }
        c.lwc1(13, (c.g[S2][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.mov_s(12, 20);
        c.div_s(13, 22, 13);
        c.mov_s(30, 0);
        if callee::RIDER_GROUND_STATE_COMPUTE_DRAG
            .call(c, 0x0013da6c)
            .is_err()
        {
            return;
        }
        c.mov_s(26, 0);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.mov_s(14, 27);
        if callee::RIDER_GROUND_STATE_COMPUTE_CARVE_ACCEL
            .call(c, 0x0013da88)
            .is_err()
        {
            return;
        }
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mov_s(29, 0);
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x378));
        c.c_lt_s(0, 1);
        let t = !c.fcc();

        if t {
            break 'b_13dab8;
        } else {
            c.cycles += 4;
            c.c_lt_s(22, 1);
            if c.fcc() {
                c.mov_s(22, 1);
                break 'b_13dab8;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13dab8;
            }
        }
    }
    // --------------------------------------------------------------------------
    // Lean-curve lookup 1: interpolate the global curve (gp-0x1fa8) at
    // |velocity| * gp-0x7060.
    // --------------------------------------------------------------------------
    'b_13db60: {
        'b_13db5c: {
            'b_13db4c: {
                'b_13db48: {
                    'b_13db28: {
                        c.cycles += 9;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7060));
                        c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x3f8));
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fa8));
                        c.mul_s(3, 23, 0);
                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.c_lt_s(4, 3);
                        if !c.fcc() {
                            c.lwc1(1, (c.g[V0][0] as u32));
                            break 'b_13db28;
                        } else {
                            'b_13db18: {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                    break 'b_13db18;
                                } else {
                                    'b_13db0c: {
                                        c.cycles += 5;
                                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                            break 'b_13db0c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(21, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                            if t {
                                                break 'b_13db60;
                                            } else {
                                                break 'b_13db0c;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    let t = true;
                                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_13db48;
                                    } else {
                                        break 'b_13db18;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = true;
                            c.sub_s(1, 1, 4);
                            if t {
                                break 'b_13db4c;
                            } else {
                                break 'b_13db28;
                            }
                        }
                    }
                    'b_13db40: {
                        c.cycles += 4;
                        c.c_lt_s(3, 1);
                        if !c.fcc() {
                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                            break 'b_13db40;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.lwc1(21, (c.g[V0][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_13db5c;
                            } else {
                                break 'b_13db40;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                    break 'b_13db48;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13db4c;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(21, 2, 0);
            break 'b_13db5c;
        }
        c.cycles += 1;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x705c));
        break 'b_13db60;
    }
    // --------------------------------------------------------------------------
    // Lean-curve lookup 2: interpolate the global curve (gp-0x1fa0) at
    // |velocity| * gp-0x705c.
    // --------------------------------------------------------------------------
    'b_13dbfc: {
        'b_13dbec: {
            'b_13dbe8: {
                'b_13dbc8: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fa0));
                    c.mul_s(3, 23, 0);
                    c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.c_lt_s(4, 3);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[V0][0] as u32));
                        break 'b_13dbc8;
                    } else {
                        'b_13dbb8: {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(1, 3);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                break 'b_13dbb8;
                            } else {
                                'b_13dbac: {
                                    c.cycles += 5;
                                    c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(4, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        break 'b_13dbac;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_13dbfc;
                                        } else {
                                            break 'b_13dbac;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.sub_s(3, 3, 1);
                                let t = true;
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                if t {
                                    break 'b_13dbe8;
                                } else {
                                    break 'b_13dbb8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.sub_s(3, 3, 4);
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = true;
                        c.sub_s(1, 1, 4);
                        if t {
                            break 'b_13dbec;
                        } else {
                            break 'b_13dbc8;
                        }
                    }
                }
                'b_13dbe0: {
                    c.cycles += 4;
                    c.c_lt_s(3, 1);
                    if !c.fcc() {
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        break 'b_13dbe0;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_13dbfc;
                        } else {
                            break 'b_13dbe0;
                        }
                    }
                }
                c.cycles += 2;
                c.sub_s(3, 3, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                break 'b_13dbe8;
            }
            c.cycles += 1;
            c.sub_s(1, 4, 1);
            break 'b_13dbec;
        }
        c.cycles += 4;
        c.sub_s(0, 0, 2);
        c.mul_s(0, 0, 3);
        c.div_s(0, 0, 1);
        c.add_s(20, 2, 0);
        break 'b_13dbfc;
    }
    // --------------------------------------------------------------------------
    // Character attribute factor (BE library), the lean sine/cosine through
    // Math_SinCos, and the accumulated force vector kept in the frame.
    // --------------------------------------------------------------------------
    'b_13de08: {
        c.cycles += 117;
        if callee::BELIBRARY_GET_INSTANCE.call(c, 0x0013dc04).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        if callee::BELIBRARY_GET_INTERFACE.call(c, 0x0013dc10).is_err() {
            return;
        }
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x86c));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb34));
        if callee::BECHARINT_GET_ATTRIB3_FACTOR_2
            .call(c, 0x0013dc24)
            .is_err()
        {
            return;
        }
        c.sub_s(1, 20, 21);
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7058));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x144));
        c.mul_s(0, 0, 1);
        c.add_s(0, 0, 1);
        c.add_s(0, 21, 0);
        c.mul_s(12, 12, 0);
        c.mul_s(12, 12, 2);
        c.mul_s(12, 12, 27);
        if call_ptr(c, 0x0031be50, 0x0013dc54).is_err() {
            return;
        }
        c.lwc1(1, (c.g[S2][0] as u32));
        c.add_s(3, 26, 30);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x144));
        c.add_s(1, 1, 1);
        c.mfc1(4, 0);
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.min_s(1, 22, 1);
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.div_s(1, 1, 0);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mfc1(4, 2);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3b0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sub_s(22, 22, 1);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(0, (c.g[S2][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
        c.neg_s(0, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.mfc1(4, 22);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.mfc1(3, 1);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.mfc1(4, 3);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3a0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.mfc1(4, 29);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3b0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x308));
        if c.g[V1][0] != 0u64 {
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13de08;
        } else {
            c.cycles += 14;
            c.g[AT][0] = 0xffffffffc47a0000;
            c.mtc1(1, 0);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x370));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13de08;
        }
    }
    // --------------------------------------------------------------------------
    // Vertical speed / penetration correction: move the rider out of the
    // ground along the contact normal and damp the vertical speed.
    // --------------------------------------------------------------------------
    'b_13df34: {
        'b_13df30: {
            c.cycles += 10;
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x454));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7054));
            c.add_s(1, 0, 1);
            c.mtc1(0, 3);
            c.max_s(1, 1, 2);
            c.c_lt_s(1, 3);
            if !c.fcc() {
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df30;
            } else {
                'b_13dec8: {
                    c.cycles += 18;
                    c.mfc1(3, 1);
                    c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x370));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.c_lt_s(31, 3);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x454));
                    c.sub_s(0, 0, 1);
                    let t = !c.fcc();
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x454));
                    if t {
                        break 'b_13dec8;
                    } else {
                        c.cycles += 20;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.mfc1(4, 24);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3d0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                        break 'b_13dec8;
                    }
                }
                c.cycles += 14;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(0, 3);
                if !c.fcc() {
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x308));
                    break 'b_13df34;
                } else {
                    c.cycles += 12;
                    c.mfc1(2, 0);
                    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13df30;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x308));
        break 'b_13df34;
    }
    // --------------------------------------------------------------------------
    // Landing and bounce handling for the corrected vertical speed.
    // --------------------------------------------------------------------------
    'b_13df90: {
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13df90;
        } else {
            c.cycles += 6;
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x454));
            c.mtc1(0, 1);
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df90;
            } else {
                c.cycles += 15;
                c.mfc1(2, 0);
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x454));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df90;
            }
        }
    }
    'b_13e080: {
        c.cycles += 25;
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[SP][0] as u32));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x390));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 3);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x214));
        c.abs_s(0, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();

        if t {
            break 'b_13e080;
        } else {
            c.cycles += 4;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7050));
            c.c_lt_s(2, 0);
            let t = !c.fcc();

            if t {
                break 'b_13e080;
            } else {
                c.cycles += 7;
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x370));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x704c));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.c_lt_s(1, 0);
                let t = !c.fcc();

                if t {
                    break 'b_13e080;
                } else {
                    'b_13e03c: {
                        c.cycles += 6;
                        c.g[AT][0] = 0x40000000;
                        c.mtc1(1, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7048));
                        c.c_lt_s(2, 0);
                        let t = !c.fcc();

                        if t {
                            break 'b_13e03c;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 1);
                            break 'b_13e03c;
                        }
                    }
                    c.cycles += 17;
                    c.mfc1(2, 1);
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32));
                    c.g[A0][0] = 0x500000;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
                    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x3d0));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(3, (c.g[A1][0] as u32).wrapping_add(0x1e0));
                    c.lq(2, (c.g[A0][0] as u32));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_13e080;
                }
            }
        }
    }
    // --------------------------------------------------------------------------
    // Animation checks: layer entry 10/4/5 or anim id 0x16 zero the lean.
    // --------------------------------------------------------------------------
    'b_13e1a4: {
        c.cycles += 72;
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 4);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mul_s(4, 25, 4);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mfc1(4, 25);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 5);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.mfc1(2, 25);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[AT][0] = 0xffffffffc1a00000;
        c.mtc1(1, 6);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sub_s(4, 1, 4);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x3d0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(3, (c.g[SP][0] as u32));
        c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x390));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.sub_s(0, 3, 0);
        c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x220));
        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0x2c8));
        c.sub_s(1, 1, 2);
        c.mul_s(0, 0, 5);
        c.mul_s(7, 2, 5);
        c.mul_s(1, 1, 6);
        c.add_s(20, 3, 0);
        c.mul_s(20, 20, 4);
        c.c_le_s(1, 20);
        let t = !c.fcc();
        c.mov_s(0, 1);
        if t {
            break 'b_13e1a4;
        } else {
            c.cycles += 1;
            c.min_s(0, 20, 7);
            break 'b_13e1a4;
        }
    }
    'b_13e20c: {
        'b_13e208: {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
            c.mov_s(20, 0);
            c.g[A1][0] = 0x2;
            if call_ptr(c, 0x00311ae8, 0x0013e1b4).is_err() {
                return;
            }
            c.g[V1][0] = 0xa;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 0x2;
            if t {
                break 'b_13e208;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                if call_ptr(c, 0x00311ae8, 0x0013e1cc).is_err() {
                    return;
                }
                c.g[V1][0] = 0x4;
                let t = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 0x2;
                if t {
                    break 'b_13e208;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                    if call_ptr(c, 0x00311ae8, 0x0013e1e4).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x5;
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = 0x2;
                    if t {
                        break 'b_13e208;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                        if call_ptr(c, 0x00312aa0, 0x0013e1fc).is_err() {
                            return;
                        }
                        c.g[V1][0] = 0x16;
                        let t = c.g[V0][0] != c.g[V1][0];

                        if t {
                            break 'b_13e20c;
                        } else {
                            break 'b_13e208;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.mtc1(0, 20);
        break 'b_13e20c;
    }
    // --------------------------------------------------------------------------
    // Sub-state / speed gate for the bank-angle (Math_Asin) block.
    // --------------------------------------------------------------------------
    'b_13e714: {
        'b_13e298: {
            c.cycles += 24;
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 0);
            c.mul_s(1, 20, 28);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.mul_s(0, 25, 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2d0));
            c.mtc1(0, 23);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2cc));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 24);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            if call_ptr(c, 0x0011fee8, 0x0013e25c).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x2;
            let t = c.g[S0][0] != c.g[V0][0];

            if t {
                break 'b_13e298;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0011fee8, 0x0013e274).is_err() {
                    return;
                }
                if c.g[V0][0] != c.g[S0][0] {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13e714;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7044));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x378));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13e714;
                    } else {
                        break 'b_13e298;
                    }
                }
            }
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7040));
        c.c_lt_s(0, 24);
        if !c.fcc() {
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13e714;
        } else {
            'b_13e340: {
                c.cycles += 37;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lqc2(4, (c.g[SP][0] as u32));
                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3a0));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.mov_s(12, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.mfc1(3, 24);
                c.qmtc2(3, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(4, 4);
                c.mtc1(4, 1);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.min_s(12, 1, 2);
                    break 'b_13e340;
                } else {
                    break 'b_13e340;
                }
            }
            // --------------------------------------------------------------------------
            // Bank angle from the lateral/forward ratio (Math_Asin) and its two
            // curve lookups.
            // --------------------------------------------------------------------------
            'b_13e3f0: {
                'b_13e3e0: {
                    'b_13e3dc: {
                        'b_13e3b8: {
                            c.cycles += 11;
                            if call_ptr(c, 0x0031c128, 0x0013e348).is_err() {
                                return;
                            }
                            c.neg_s(8, 0);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fc0));
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x703c));
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.mul_s(3, 24, 0);
                            c.c_lt_s(4, 3);
                            let t = !c.fcc();
                            c.lwc1(9, (c.g[S2][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_13e3b8;
                            } else {
                                'b_13e3a8: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                        break 'b_13e3a8;
                                    } else {
                                        'b_13e39c: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 3);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                break 'b_13e39c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13e3f0;
                                                } else {
                                                    break 'b_13e39c;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.sub_s(3, 3, 1);
                                        let t = true;
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_13e3dc;
                                        } else {
                                            break 'b_13e3a8;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.sub_s(3, 3, 4);
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_13e3e0;
                                } else {
                                    break 'b_13e3b8;
                                }
                            }
                        }
                        'b_13e3d4: {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32));
                            c.c_lt_s(3, 1);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                                break 'b_13e3d4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_13e3f0;
                                } else {
                                    break 'b_13e3d4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sub_s(3, 3, 1);
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                        break 'b_13e3dc;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_13e3e0;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(7, 2, 0);
                break 'b_13e3f0;
            }
            'b_13e4d8: {
                'b_13e4c8: {
                    'b_13e4c4: {
                        'b_13e4a4: {
                            c.cycles += 26;
                            c.mul_s(0, 27, 27);
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 5);
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7038));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.sub_s(0, 5, 0);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x7034));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x220));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7030));
                            c.mul_s(0, 0, 4);
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x702c));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7028));
                            c.mul_s(1, 1, 2);
                            c.mul_s(3, 7, 3);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fb8));
                            c.mul_s(0, 9, 0);
                            c.mul_s(2, 24, 4);
                            c.add_s(1, 1, 5);
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.add_s(0, 0, 6);
                            c.c_lt_s(4, 2);
                            c.mul_s(0, 0, 27);
                            c.mul_s(5, 0, 3);
                            c.div_s(5, 5, 1);
                            if !c.fcc() {
                                c.lwc1(1, (c.g[V0][0] as u32));
                                break 'b_13e4a4;
                            } else {
                                'b_13e494: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 2);
                                    if !c.fcc() {
                                        c.sub_s(3, 2, 4);
                                        break 'b_13e494;
                                    } else {
                                        'b_13e488: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 2);
                                            if !c.fcc() {
                                                c.sub_s(3, 2, 1);
                                                break 'b_13e488;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13e4d8;
                                                } else {
                                                    break 'b_13e488;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        let t = true;
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        if t {
                                            break 'b_13e4c4;
                                        } else {
                                            break 'b_13e494;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_13e4c8;
                                } else {
                                    break 'b_13e4a4;
                                }
                            }
                        }
                        'b_13e4bc: {
                            c.cycles += 4;
                            c.c_lt_s(2, 1);
                            if !c.fcc() {
                                c.sub_s(3, 2, 1);
                                break 'b_13e4bc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_13e4d8;
                                } else {
                                    break 'b_13e4bc;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        break 'b_13e4c4;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_13e4c8;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(1, 2, 0);
                break 'b_13e4d8;
            }
            'b_13e51c: {
                c.cycles += 16;
                c.mul_s(5, 5, 1);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.mtc1(0, 0);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x3a0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.add_s(8, 8, 5);
                if t {
                    break 'b_13e51c;
                } else {
                    c.cycles += 1;
                    c.neg_s(8, 8);
                    break 'b_13e51c;
                }
            }
            'b_13e5e4: {
                'b_13e5dc: {
                    c.cycles += 32;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7024));
                    c.g[V0][0] = 0x500000;
                    c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                    c.mul_s(0, 24, 0);
                    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                    c.lqc2(5, (c.g[V0][0] as u32));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7020));
                    c.mul_s(1, 1, 0);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.c_lt_s(2, 3);
                    c.mul_s(1, 1, 24);
                    c.mul_s(7, 1, 25);
                    let t = !c.fcc();
                    c.min_s(7, 7, 0);
                    if t {
                        break 'b_13e5dc;
                    } else {
                        c.cycles += 16;
                        c.mfc1(2, 3);
                        c.qmtc2(2, 3);
                        c.vu0().div(0, 3, 3, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().waitq();
                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                        c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x90));
                        let t = true;
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        if t {
                            break 'b_13e5e4;
                        } else {
                            break 'b_13e5dc;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
                break 'b_13e5e4;
            }
            'b_13e6cc: {
                'b_13e69c: {
                    'b_13e684: {
                        c.cycles += 32;
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 3);
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x701c));
                        c.abs_s(3, 3);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 5);
                        c.div_s(3, 3, 24);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.div_s(0, 24, 0);
                        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x28));
                        c.mtc1(0, 2);
                        c.add_s(1, 1, 1);
                        c.lwc1(6, (c.g[S2][0] as u32).wrapping_add(0x2c));
                        c.c_le_s(8, 2);
                        c.sub_s(5, 5, 3);
                        c.min_s(0, 0, 4);
                        c.mul_s(0, 0, 5);
                        c.mul_s(0, 0, 1);
                        c.max_s(0, 0, 2);
                        c.add_s(6, 6, 0);
                        let t = !c.fcc();
                        c.min_s(6, 6, 4);
                        if t {
                            break 'b_13e684;
                        } else {
                            c.cycles += 6;
                            c.neg_s(0, 27);
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7018));
                            c.mul_s(0, 0, 1);
                            c.c_le_s(6, 0);
                            let t = c.fcc();

                            if t {
                                break 'b_13e69c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.mul_s(7, 7, 6);
                                if t {
                                    break 'b_13e6cc;
                                } else {
                                    break 'b_13e684;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7014));
                    c.mul_s(0, 27, 0);
                    c.c_le_s(6, 0);
                    if !c.fcc() {
                        c.mul_s(7, 7, 6);
                        break 'b_13e6cc;
                    } else {
                        break 'b_13e69c;
                    }
                }
                'b_13e6c0: {
                    c.cycles += 4;
                    c.mtc1(0, 0);
                    c.c_le_s(8, 0);
                    let t = !c.fcc();

                    if t {
                        break 'b_13e6c0;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7010));
                        c.neg_s(1, 27);
                        c.mul_s(0, 7, 0);
                        let t = true;
                        c.mul_s(7, 0, 1);
                        if t {
                            break 'b_13e6cc;
                        } else {
                            break 'b_13e6c0;
                        }
                    }
                }
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x700c));
                c.mul_s(0, 7, 0);
                c.mul_s(7, 0, 27);
                break 'b_13e6cc;
            }
            'b_13e6e8: {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.c_eq_s(0, 1);
                if !c.fcc() {
                    c.mov_s(7, 1);
                    break 'b_13e6e8;
                } else {
                    break 'b_13e6e8;
                }
            }
            'b_13e70c: {
                c.cycles += 9;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7008));
                c.mul_s(1, 8, 7);
                c.mul_s(2, 25, 0);
                c.neg_s(0, 0);
                c.mul_s(0, 25, 0);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.min_s(0, 1, 2);
                    break 'b_13e70c;
                } else {
                    break 'b_13e70c;
                }
            }
            c.cycles += 2;
            c.mov_s(23, 0);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13e714;
        }
    }
    // --------------------------------------------------------------------------
    // Carve-angle sign handling and the clamping helpers.
    // --------------------------------------------------------------------------
    'b_13ea68: {
        c.cycles += 5;
        c.mtc1(0, 22);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x2dc));
        c.c_eq_s(0, 22);
        let t = c.fcc();

        if t {
            break 'b_13ea68;
        } else {
            'b_13e8fc: {
                'b_13e8f8: {
                    'b_13e8f4: {
                        'b_13e8ec: {
                            'b_13e8e0: {
                                'b_13e8b8: {
                                    'b_13e89c: {
                                        'b_13e838: {
                                            c.cycles += 27;
                                            c.lqc2(3, (c.g[SP][0] as u32));
                                            c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.lwc1(26, (c.g[GP][0] as u32).wrapping_sub(0x7004));
                                            c.mfc1(2, 0);
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.qmtc2(2, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.lqc2(3, (c.g[SP][0] as u32));
                                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x320));
                                            let t = c.g[V1][0] == 0u64;

                                            if t {
                                                break 'b_13e838;
                                            } else {
                                                'b_13e808: {
                                                    c.cycles += 23;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3a0),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3b0),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(3, 4);
                                                    c.mtc1(3, 1);
                                                    c.neg_s(20, 0);
                                                    c.c_eq_s(20, 22);
                                                    let t = !c.fcc();
                                                    c.neg_s(21, 1);
                                                    if t {
                                                        break 'b_13e808;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_eq_s(21, 22);
                                                        let t = c.fcc();
                                                        c.mov_s(12, 21);
                                                        if t {
                                                            break 'b_13e8f8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_13e89c;
                                                            } else {
                                                                break 'b_13e808;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.div_s(12, 21, 20);
                                                if call_ptr(c, 0x0031c228, 0x0013e814).is_err() {
                                                    return;
                                                }
                                                c.c_lt_s(20, 22);
                                                let t = !c.fcc();
                                                c.mov_s(1, 0);
                                                if t {
                                                    break 'b_13e8f4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.c_lt_s(22, 21);
                                                    let t = c.fcc();

                                                    if t {
                                                        break 'b_13e8e0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_13e8ec;
                                                        } else {
                                                            break 'b_13e838;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 21;
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x3a0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 20);
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x3b0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 21);
                                        c.c_eq_s(20, 22);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_13e8b8;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(21, 22);
                                            let t = c.fcc();
                                            c.mov_s(12, 21);
                                            if t {
                                                break 'b_13e8f8;
                                            } else {
                                                break 'b_13e89c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7000));
                                    let t = !c.fcc();
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_13e8fc;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6ffc));
                                        let t = true;

                                        if t {
                                            break 'b_13e8fc;
                                        } else {
                                            break 'b_13e8b8;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                if call_ptr(c, 0x0031c228, 0x0013e8c4).is_err() {
                                    return;
                                }
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(1, 0);
                                if t {
                                    break 'b_13e8f4;
                                } else {
                                    c.cycles += 3;
                                    c.c_lt_s(22, 21);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_13e8ec;
                                    } else {
                                        break 'b_13e8e0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff8));
                            let t = true;
                            c.add_s(1, 1, 0);
                            if t {
                                break 'b_13e8f4;
                            } else {
                                break 'b_13e8ec;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff4));
                        c.sub_s(1, 1, 0);
                        break 'b_13e8f4;
                    }
                    c.cycles += 1;
                    c.mov_s(12, 1);
                    break 'b_13e8f8;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13e8fc;
            }
            'b_13e960: {
                c.cycles += 7;
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.mul_s(0, 12, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.neg_s(12, 12);
                    break 'b_13e960;
                } else {
                    'b_13e954: {
                        c.cycles += 12;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff0));
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fec));
                        c.sub_s(12, 0, 12);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 2);
                        c.mul_s(1, 12, 1);
                        c.add_s(1, 1, 2);
                        c.cvt_w_s(2, 1);
                        c.cvt_s_w(2, 2);
                        c.c_lt_s(1, 2);
                        let t = !c.fcc();

                        if t {
                            break 'b_13e954;
                        } else {
                            c.cycles += 3;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.sub_s(2, 2, 0);
                            break 'b_13e954;
                        }
                    }
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fe8));
                    c.mul_s(0, 2, 0);
                    c.sub_s(12, 12, 0);
                    break 'b_13e960;
                }
            }
            'b_13e9d4: {
                c.cycles += 6;
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fe4));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();

                if t {
                    break 'b_13e9d4;
                } else {
                    'b_13e994: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(12, 0);
                        if !c.fcc() {
                            c.add_s(0, 12, 1);
                            break 'b_13e994;
                        } else {
                            c.cycles += 2;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fe0));
                            c.add_s(0, 12, 0);
                            break 'b_13e994;
                        }
                    }
                    'b_13e9cc: {
                        'b_13e9b4: {
                            c.cycles += 6;
                            c.mul_s(3, 26, 25);
                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                            c.c_lt_s(3, 2);
                            let t = !c.fcc();
                            c.mov_s(12, 0);
                            if t {
                                break 'b_13e9b4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sub_s(1, 2, 3);
                                if t {
                                    break 'b_13e9cc;
                                } else {
                                    break 'b_13e9b4;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.neg_s(0, 3);
                        c.mtc1(0, 1);
                        c.c_lt_s(2, 0);
                        if c.fcc() {
                            c.add_s(1, 2, 3);
                            break 'b_13e9cc;
                        } else {
                            break 'b_13e9cc;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                    break 'b_13e9d4;
                }
            }
            'b_13e9f8: {
                c.cycles += 6;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fdc));
                c.abs_s(1, 12);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.neg_s(1, 26);
                    break 'b_13e9f8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x2dc), 0u64);
                    if t {
                        break 'b_13ea68;
                    } else {
                        break 'b_13e9f8;
                    }
                }
            }
            'b_13ea18: {
                c.cycles += 7;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fd8));
                c.div_s(0, 26, 0);
                c.mul_s(0, 12, 0);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.mov_s(2, 1);
                if t {
                    break 'b_13ea18;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 26);
                    break 'b_13ea18;
                }
            }
            'b_13ea44: {
                c.cycles += 6;
                c.neg_s(0, 25);
                c.mtc1(0, 1);
                c.mul_s(0, 0, 2);
                c.c_lt_s(0, 1);
                let t = !c.fcc();

                if t {
                    break 'b_13ea44;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(23, 1);
                    let t = !c.fcc();

                    if t {
                        break 'b_13ea44;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.min_s(23, 23, 0);
                        if t {
                            break 'b_13ea68;
                        } else {
                            break 'b_13ea44;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.add_s(23, 23, 0);
                break 'b_13ea68;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 23);
                if !c.fcc() {
                    c.add_s(23, 23, 0);
                    break 'b_13ea68;
                } else {
                    c.cycles += 1;
                    c.max_s(23, 23, 0);
                    break 'b_13ea68;
                }
            }
        }
    }
    'b_13eab0: {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 24);
        if !c.fcc() {
            c.mov_s(5, 0);
            break 'b_13eab0;
        } else {
            c.cycles += 13;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.div_s(5, 0, 24);
            break 'b_13eab0;
        }
    }
    'b_13eb64: {
        'b_13eb60: {
            'b_13eb50: {
                'b_13eb4c: {
                    'b_13eb28: {
                        c.cycles += 11;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fd4));
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f98));
                        c.mul_s(3, 24, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fd0));
                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.sub_s(5, 5, 1);
                        c.mtc1(0, 0);
                        c.c_lt_s(4, 3);
                        let t = !c.fcc();
                        c.max_s(5, 5, 0);
                        if t {
                            break 'b_13eb28;
                        } else {
                            'b_13eb18: {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                    break 'b_13eb18;
                                } else {
                                    'b_13eb0c: {
                                        c.cycles += 5;
                                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                            break 'b_13eb0c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                            if t {
                                                break 'b_13eb60;
                                            } else {
                                                break 'b_13eb0c;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    let t = true;
                                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_13eb4c;
                                    } else {
                                        break 'b_13eb18;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = true;
                            c.sub_s(1, 1, 4);
                            if t {
                                break 'b_13eb50;
                            } else {
                                break 'b_13eb28;
                            }
                        }
                    }
                    'b_13eb44: {
                        c.cycles += 5;
                        c.lwc1(1, (c.g[V0][0] as u32));
                        c.c_lt_s(3, 1);
                        let t = !c.fcc();
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_13eb44;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_13eb64;
                            } else {
                                break 'b_13eb44;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                    break 'b_13eb4c;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13eb50;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(2, 2, 0);
            break 'b_13eb60;
        }
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        break 'b_13eb64;
    }
    // --------------------------------------------------------------------------
    // Probe the ground with a collision volume at the rider's feet; on a hit
    // resolve the contact and record the surface.
    // --------------------------------------------------------------------------
    'b_13eb98: {
        c.cycles += 9;
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1f0));
        c.mul_s(0, 5, 0);
        c.mul_s(0, 0, 2);
        c.sub_s(23, 23, 0);
        c.c_eq_s(23, 1);
        let t = c.fcc();
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        if t {
            break 'b_13eb98;
        } else {
            c.cycles += 4;
            c.mov_s(12, 23);
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x370));
            if call_ptr(c, 0x0011dfe0, 0x0013eb94).is_err() {
                return;
            }
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
            break 'b_13eb98;
        }
    }
    'b_13ebf8: {
        'b_13ebf4: {
            'b_13ebc4: {
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x148));
                if callee::RIDER_GROUND_STATE_PROBE_GROUND
                    .call(c, 0x0013eba8)
                    .is_err()
                {
                    return;
                }
                c.g[S3][0] = c.g[V0][0];
                let t = c.g[S3][0] != 0u64;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_13ebc4;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x308));
                    let t = true;
                    c.g[S4][0] = (c.g[V1][0] < 0x1u64) as u64;
                    if t {
                        break 'b_13ebf4;
                    } else {
                        break 'b_13ebc4;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13ebf8;
            } else {
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                let t = c.g[A3][0] == 0u64;
                c.g[A1][0] = c.g[S0][0];
                if t {
                    break 'b_13ebf4;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0013ebf4);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.pc = t;
                    call_indirect(c, t);
                    if c.pc != 0x0013ebf4 {
                        return;
                    }
                    break 'b_13ebf4;
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        break 'b_13ebf8;
    }
    // --------------------------------------------------------------------------
    // Ground normal and velocity correction after the probe.
    // --------------------------------------------------------------------------
    'b_13ed2c: {
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x438));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        if c.g[V0][0] != 0u64 {
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
            break 'b_13ed2c;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            if c.g[V1][0] == c.g[V0][0] {
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
                break 'b_13ed2c;
            } else {
                'b_13eca4: {
                    c.cycles += 35;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fcc));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x370));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    c.g[AT][0] = 0xffffffffc2200000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 0, 1);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x370));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.c_lt_s(0, 2);
                    if c.fcc() {
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
                        break 'b_13eca4;
                    } else {
                        break 'b_13eca4;
                    }
                }
                'b_13ed0c: {
                    c.cycles += 19;
                    c.mtc1(0, 0);
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 1);
                    c.c_eq_s(1, 0);
                    if c.fcc() {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13ed0c;
                    } else {
                        c.cycles += 7;
                        c.div_s(0, 3, 1);
                        c.mfc1(2, 0);
                        c.lqc2(4, (c.g[SP][0] as u32));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13ed0c;
                    }
                }
                c.cycles += 8;
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x3d0));
                c.lqc2(4, (c.g[SP][0] as u32));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
                break 'b_13ed2c;
            }
        }
    }
    // --------------------------------------------------------------------------
    // Set the grounded flag (gs+0xc) and skip the rest of the frame.
    // --------------------------------------------------------------------------
    'b_13f11c: {
        // --------------------------------------------------------------------------
        // Banking about the contact normal (Math_Atan / Math_Asin) and the
        // ground-normal torque.
        // --------------------------------------------------------------------------
        'b_13ed68: {
            'b_13ed4c: {
                c.cycles += 2;
                let t = c.g[S3][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                if t {
                    break 'b_13ed4c;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.g[V0][0];
                    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x454));
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x148));
                    if t {
                        break 'b_13ed68;
                    } else {
                        break 'b_13ed4c;
                    }
                }
            }
            'b_13ed60: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x308));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_13ed60;
                } else {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;

                    if t {
                        break 'b_13f11c;
                    } else {
                        break 'b_13ed60;
                    }
                }
            }
            c.cycles += 2;
            let t = true;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            if t {
                break 'b_13f11c;
            } else {
                break 'b_13ed68;
            }
        }
        'b_13eea0: {
            c.cycles += 6;
            c.g[AT][0] = 0x40a00000;
            c.mtc1(1, 0);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13eea0;
            } else {
                c.cycles += 52;
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x378));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x70), 0u64);
                c.mul_s(0, 0, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x74), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fc8));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x1c0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eea0;
                } else {
                    c.cycles += 20;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.min_s(12, 0, 12);
                    if call_ptr(c, 0x0031c128, 0x0013ee7c).is_err() {
                        return;
                    }
                    c.neg_s(0, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fc4));
                    c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.mul_s(0, 0, 1);
                    c.mul_s(12, 0, 12);
                    if call_ptr(c, 0x0011dfe0, 0x0013ee9c).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eea0;
                }
            }
        }
        'b_13eef4: {
            c.cycles += 15;
            c.g[A1][0] = 0x2;
            c.g[S0][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
            if call_ptr(c, 0x00311ae8, 0x0013eeb0).is_err() {
                return;
            }
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = 0x2;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
            if call_ptr(c, 0x00312aa0, 0x0013eec4).is_err() {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[S2][0] = c.g[V0][0];
            if call_ptr(c, 0x0011fee8, 0x0013eed0).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            let t = c.g[V0][0] == c.g[A0][0];

            if t {
                break 'b_13eef4;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x438));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = 0u64;
                }
                break 'b_13eef4;
            }
        }
        'b_13f028: {
            'b_13f018: {
                'b_13ef28: {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_13ef28;
                    } else {
                        'b_13ef20: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xa;
                            let t = c.g[S3][0] == c.g[V0][0];
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_13ef20;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S3][0] == c.g[V0][0];
                                c.g[V0][0] = 0x16;
                                if t {
                                    break 'b_13ef20;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == c.g[V0][0];
                                    c.g[V0][0] = c.g[S2][0] ^ 0x2;
                                    if t {
                                        break 'b_13ef20;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V1][0] = c.g[A0][0];
                                        if c.g[V0][0] == 0 {
                                            c.g[V1][0] = 0u64;
                                        }
                                        break 'b_13ef20;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[S0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            break 'b_13f018;
                        } else {
                            break 'b_13ef28;
                        }
                    }
                }
                'b_13eff4: {
                    'b_13eff0: {
                        'b_13efe0: {
                            'b_13efdc: {
                                'b_13efbc: {
                                    c.cycles += 18;
                                    c.lqc2(3, (c.g[SP][0] as u32));
                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                    c.vu0().sqrt(4, 0);
                                    c.vu0().waitq();
                                    c.cfc2(2, 22);
                                    c.mtc1(2, 1);
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fc0));
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fc8));
                                    c.mul_s(1, 1, 0);
                                    c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(5, 1);
                                    if !c.fcc() {
                                        c.lwc1(4, (c.g[V0][0] as u32));
                                        break 'b_13efbc;
                                    } else {
                                        'b_13efac: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x10));
                                            c.c_lt_s(4, 1);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                                break 'b_13efac;
                                            } else {
                                                'b_13efa0: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        5,
                                                        (c.g[V0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.c_lt_s(5, 1);
                                                    if !c.fcc() {
                                                        c.lwc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        break 'b_13efa0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x1c),
                                                        );
                                                        if t {
                                                            break 'b_13eff0;
                                                        } else {
                                                            break 'b_13efa0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.sub_s(3, 1, 4);
                                                let t = true;
                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13efdc;
                                                } else {
                                                    break 'b_13efac;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.sub_s(3, 1, 5);
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        let t = true;
                                        c.sub_s(1, 4, 5);
                                        if t {
                                            break 'b_13efe0;
                                        } else {
                                            break 'b_13efbc;
                                        }
                                    }
                                }
                                'b_13efd4: {
                                    c.cycles += 4;
                                    c.c_lt_s(1, 4);
                                    let t = !c.fcc();
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                                    if t {
                                        break 'b_13efd4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                        if t {
                                            break 'b_13eff4;
                                        } else {
                                            break 'b_13efd4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.sub_s(3, 1, 4);
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                                break 'b_13efdc;
                            }
                            c.cycles += 1;
                            c.sub_s(1, 5, 4);
                            break 'b_13efe0;
                        }
                        c.cycles += 4;
                        c.sub_s(0, 0, 2);
                        c.mul_s(0, 0, 3);
                        c.div_s(0, 0, 1);
                        c.add_s(2, 2, 0);
                        break 'b_13eff0;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eff4;
                }
                c.cycles += 9;
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x208));
                c.mul_s(1, 25, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x210));
                c.sub_s(0, 0, 2);
                c.abs_s(0, 0);
                let t = true;
                c.mul_s(0, 0, 1);
                if t {
                    break 'b_13f028;
                } else {
                    break 'b_13f018;
                }
            }
            c.cycles += 4;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 0);
            c.mul_s(0, 25, 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x210), 0u64);
            break 'b_13f028;
        }
        // --------------------------------------------------------------------------
        // Lean, pitch and spin outputs plus the speed curves.
        // --------------------------------------------------------------------------
        'b_13f064: {
            'b_13f050: {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20c));
                if t {
                    break 'b_13f050;
                } else {
                    c.cycles += 8;
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.mul_s(0, 25, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    let t = true;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2c4));
                    if t {
                        break 'b_13f064;
                    } else {
                        break 'b_13f050;
                    }
                }
            }
            c.cycles += 5;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 0);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.mul_s(0, 25, 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x2c4), 0u64);
            break 'b_13f064;
        }
        // --------------------------------------------------------------------------
        // Advance the steering angle (gs+0x0, wrapped at 360 degrees) and
        // rebuild the rider's world matrix.
        // --------------------------------------------------------------------------
        'b_13f0cc: {
            c.cycles += 25;
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c0));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fbc));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 3);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.abs_s(1, 27);
            c.mul_s(3, 3, 2);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 4);
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x214));
            c.abs_s(0, 0);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fb8));
            c.add_s(1, 1, 0);
            c.c_lt_s(4, 3);
            let t = !c.fcc();
            c.add_s(0, 1, 2);
            if t {
                break 'b_13f0cc;
            } else {
                c.cycles += 1;
                c.mov_s(3, 4);
                break 'b_13f0cc;
            }
        }
        'b_13f0fc: {
            c.cycles += 10;
            c.mul_s(20, 3, 0);
            c.lwc1(1, (c.g[S1][0] as u32));
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fb4));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fb0));
            c.mul_s(0, 20, 0);
            c.add_s(0, 1, 0);
            c.c_le_s(2, 0);
            let t = !c.fcc();
            c.swc1(0, (c.g[S1][0] as u32));
            if t {
                break 'b_13f0fc;
            } else {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, (c.g[S1][0] as u32));
                break 'b_13f0fc;
            }
        }
        c.cycles += 8;
        c.lwc1(12, (c.g[S1][0] as u32));
        if call_ptr(c, 0x0031c040, 0x0013f104).is_err() {
            return;
        }
        c.g[AT][0] = 0x40f00000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mul_s(0, 0, 1);
        c.mul_s(0, 20, 0);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x31c));
        break 'b_13f11c;
    }
    c.cycles += 22;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
    if call_ptr(c, 0x0011e098, 0x0013f124).is_err() {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
    c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x208));
    c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x200));
    c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1f8));
    c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1f0));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1e8));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1e0));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1d8));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Differential test.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, elf_image, fill, DATA, GAME_GP, SP};

    const SEEDS: u32 = 200;

    /// The function's argument (`GroundState*`).
    const GS: u32 = DATA + 0x1000;
    const RIDER: u32 = DATA + 0x2000;
    const MODEL: u32 = DATA + 0x3400;
    const ANIM: u32 = DATA + 0x3600;
    const STATE: u32 = DATA + 0x4000;
    const WORLD: u32 = DATA + 0x5000;
    const RACE: u32 = DATA + 0x5100;
    const GROUND: u32 = DATA + 0x6000;
    const OVERLAP: u32 = DATA + 0x7000;
    const CURVES: u32 = DATA + 0x8000;
    const BELIB: u32 = DATA + 0x9000;
    const IFACE: u32 = DATA + 0x9100;
    const VT: u32 = DATA + 0x9200;
    /// `(&DAT_005308a8)[i]`, the BE interface array indexed by `gp-0x1efc`.
    const IFACE_ARRAY: u32 = 0x0053_08a8;

    /// The per-frame ground-state update, with the world/race/ground tables,
    /// the rider's model/animation/state objects, the BE library interface
    /// (whose one vtable method is a fake) and the global speed-curve
    /// pointers the real callees read. The collision volume misses (empty
    /// overlap list), so the ground probe takes its short, no-contact path.
    ///
    /// The contact branches (a ProbeGround hit, and the `rider+0x438` values
    /// 2/3/0xd it leaves) need real collision geometry and a populated
    /// overlap list, so the fixture cannot reach them; everything before the
    /// probe is covered. The port keeps those branches verbatim.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0013_d818, 0);
        elf_image(c);
        // The generated frame: when the probe's volume misses it leaves the
        // slot at +0x110 alone, so start from a zeroed frame instead of the
        // random bytes `machine` puts there (it would be followed as a
        // pointer by the contact callback path).
        for i in 0..0x210 / 4 {
            c.bus.write32(SP - 0x210 + 4 * i, 0);
        }
        // Ground state.
        c.bus.write32(GS, r.f32(3.0)); // steering angle
        c.bus.write32(GS + 4, r.f32(30.0)); // forward speed
        c.bus.write32(GS + 8, r.f32(30.0)); // lateral speed
        c.bus.write32(GS + 0xc, 0xdead_beef); // grounded flag, set by the tail
        c.bus.write32(GS + 0x18, RIDER);
        c.g[A0][0] = sx(GS);
        // Rider: small floats everywhere, then the fields that are pointers,
        // flags or indices.
        fill(c, &mut r, RIDER, 0xc00 / 4);
        c.bus
            .write32(RIDER + 0x214, if seed % 3 == 0 { 0 } else { r.f32(1.0) }); // lean dead zone
        c.bus.write32(RIDER + 0x2e4, r.f32(0.9)); // maximum tilt
        c.bus.write32(RIDER + 0x378, r.f32(20.0)); // vertical speed
        c.bus.write32(RIDER + 0x454, r.f32(10.0)); // ground contact parameter
        c.bus.write32(RIDER + 0x780, MODEL);
        c.bus.write32(RIDER + 0x784, ANIM);
        c.bus.write32(RIDER + 0x77c, STATE);
        c.bus.write32(RIDER + 0x860, OVERLAP);
        c.bus.write32(RIDER + 0x864, 0);
        c.bus.write32(RIDER + 0x874, 0); // wind disabled
        c.bus.write32(RIDER + 0x86c, 0);
        c.bus.write32(RIDER + 0xb34, 0);
        c.bus.write32(RIDER + 0x438, seed % 4); // ground segment index
        fill(c, &mut r, MODEL, 0x200 / 4);
        fill(c, &mut r, ANIM, 0x40 / 4);
        c.bus.write32(ANIM + 8, 0x1b6); // "no entry" -> the anim-table call is skipped
        fill(c, &mut r, STATE, 0xe00 / 4);
        c.bus.write32(STATE + 0xde4, seed % 5); // sub-state
                                                // World -> race manager -> the ground segments.
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, RACE);
        c.bus.write32(RACE + 0x44, GROUND);
        fill(c, &mut r, GROUND, 4 * 0xb0 / 4);
        // An empty overlap list: CollideVolume finds nothing.
        for i in 0..0x200 / 4 {
            c.bus.write32(OVERLAP + 4 * i, 0);
        }
        // The six animation speed/lean curve structs.
        for (k, off) in [0x1fa8u32, 0x1fa0, 0x1fb8, 0x1fc0, 0x1f98, 0x1fc8]
            .into_iter()
            .enumerate()
        {
            let a = CURVES + 0x40 * k as u32;
            fill(c, &mut r, a, 8);
            c.bus.write32(GAME_GP - off, a);
        }
        // The BE library singleton and interface slot 0 (the generated
        // GetInterface indexes it by `gp-0x1efc`), whose vtable method is
        // unmapped so the fake call intercepts it.
        c.bus.write32(GAME_GP - 0x1f04, BELIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        // A non-zero character attribute divisor (the table is in .bss).
        c.bus.write8(0x0053_08e3, 10);
        c.bus.write32(IFACE_ARRAY, IFACE);
        c.bus.write32(IFACE + 4, VT);
        c.bus.write16(VT + 8, 0);
        c.bus.write32(VT + 0xc, 0x7000_0000);
    }

    #[test]
    fn rider_ground_state_update_matches() {
        for seed in 0..SEEDS {
            let r = compare(ee::s01::f_0013d818__generated, rider_ground_state_update, &|c| {
                setup(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lo/hi: {:#?}", r.lohi);
        }
    }
}
