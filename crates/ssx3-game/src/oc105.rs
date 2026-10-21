//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **`ComputerActorRider_GetControlInput`** (0x0010a768): clears the CPU
//!   rider's control-input block, runs the rubber-band speed adjustment,
//!   resets the animation time scale, then dispatches on the rider's
//!   sub-state to the matching thinking step (cruise, board press, prewind,
//!   natural air, spin, gate, railslide, wipeout). All other sub-states do
//!   nothing. Finally, unless the rider is spinning (sub-state 5), the
//!   "control taken over" word at `+0xe34` is cleared.
//! - **`ComputerActorRider_CruiseFollowPath`** (0x00100680): the cruise
//!   thinking step. It blends the AI's steering into the rider's control
//!   flags, reacts to speed markers (boost when one is near, crouch
//!   otherwise), runs the board-press countdown and resolves the jump state
//!   through the rival's path interface. The generated body is a chain of
//!   sequentially laid out blocks entered by falling out of the previous
//!   one; the `'b_…` labels below keep that shape.
//! - **`vfprintf_dispatch`** (0x0041b658, `vfprintf`): scans a format string
//!   for a conversion that needs floating point (`%E`, `%G`, `%L`, `%e`,
//!   `%f`, `%g`) and tail-calls the full `_vfprintf_r`; otherwise it
//!   tail-calls the integer-only `_vfiprintf_r`. The specifier table at
//!   `0x450000 + 0x6a30` is the original jump table (index `c - 'E'`).

use crate::call::{self, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0010_a768,
        computer_actor_rider_get_control_input,
        "ComputerActorRider_GetControlInput",
    ),
    (
        0x0010_0680,
        computer_actor_rider_cruise_follow_path,
        "ComputerActorRider_CruiseFollowPath",
    ),
    (0x0041_b658, vfprintf_dispatch, "vfprintf"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const RUBBER_BAND: Callee = Callee {
        addr: 0x0010_def0,
        f: ee::s00::ComputerActorRider_ComputeRubberBand,
    };
    pub const SET_ANIM_TIME_SCALE: Callee = Callee {
        addr: 0x0012_0090,
        f: ee::s00::Rider_SetAnimTimeScale,
    };
    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };
    pub const THINK_CRUISE: Callee = Callee {
        addr: 0x0010_a960,
        f: ee::s00::ComputerActorRider_ThinkCruise,
    };
    pub const THINK_BOARD_PRESS: Callee = Callee {
        addr: 0x0010_ad78,
        f: ee::s00::ComputerActorRider_ThinkBoardPress,
    };
    pub const THINK_PREWIND: Callee = Callee {
        addr: 0x0010_aa70,
        f: ee::s00::ComputerActorRider_ThinkPrewind,
    };
    pub const THINK_NATURAL_AIR: Callee = Callee {
        addr: 0x0010_b590,
        f: ee::s00::ComputerActorRider_ThinkNaturalAir,
    };
    pub const THINK_SPIN: Callee = Callee {
        addr: 0x0010_b250,
        f: ee::s00::ComputerActorRider_ThinkSpin,
    };
    pub const THINK_GATE: Callee = Callee {
        addr: 0x0010_b790,
        f: ee::s00::ComputerActorRider_ThinkGate,
    };
    pub const THINK_RAILSLIDE: Callee = Callee {
        addr: 0x0010_aed8,
        f: ee::s00::ComputerActorRider_ThinkRailslide,
    };
    pub const THINK_WIPEOUT: Callee = Callee {
        addr: 0x0010_b750,
        f: ee::s00::ComputerActorRider_ThinkWipeout,
    };

    pub const VFPRINTF_R: Callee = Callee {
        addr: 0x0041_b728,
        f: ee::s15::_vfprintf_r,
    };
    pub const VFIPRINTF_R: Callee = Callee {
        addr: 0x0041_ca70,
        f: ee::s15::_vfiprintf_r,
    };

    /// `EA_MemSet(block, 0, 8)`: the original calls it through the table.
    pub const EA_MEM_SET: u32 = 0x003e_6448;
}

// ---------------------------------------------------------------------------
// Rider fields used here (see docs/investigation/structs/rider.md).

/// The rider's `RiderModel*` (animation time scale at `+0x300`).
const RIDER_MODEL: u32 = 0x18;
/// Non-zero while another system has taken the control input over.
const RIDER_CONTROL_OVERRIDE: u32 = 0xe34;
/// The last sub-state, 0xe is past the last handling step.
const SUB_STATE_LAST: u64 = 0xe;
/// The one sub-state that keeps the override word.
const SUB_STATE_SPIN: u64 = 5;

/// `void ComputerActorRider_GetControlInput(Rider* rider, ControlInput* in)`
/// (0x0010a768): fills `in` for the rider's current sub-state.
pub fn computer_actor_rider_get_control_input(c: &mut Cpu) {
    guest(c, |c| {
        // The original's switch dispatcher reloads the sub-state from the
        // function entry for the (unused) state-0 table slot; keep a loop so
        // that edge exists with its event check.
        'restart: loop {
            c.cycles += 22;
            let frame = Frame::open(c, 0x40);
            c.g[A2][0] = 8;
            c.sq(S0, frame.at(c, 0x30));
            c.sq(S1, frame.at(c, 0x20));
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(S2, frame.at(c, 0x10));
            frame.save_ra(c, 0);
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, callee::EA_MEM_SET, 0x0010_a794)?;

            c.g[A0][0] = c.g[S1][0];
            callee::RUBBER_BAND.call(c, 0x0010_a79c)?;

            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(RIDER_MODEL).lw(c);
            c.mov_s(12, 0);
            callee::SET_ANIM_TIME_SCALE.call(c, 0x0010_a7a8)?;

            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(RIDER_MODEL).lw(c);
            callee::GET_SUB_STATE.call(c, 0x0010_a7b0)?;
            c.g[S2][0] = c.g[V0][0];

            c.g[V0][0] = (c.g[S2][0] < SUB_STATE_LAST) as u64;
            let dispatch = c.g[V0][0] != 0;
            c.g[V0][0] = 0x450000;
            if dispatch {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                // The step for each sub-state (the table's target blocks).
                match c.g[A0][0] as u32 {
                    0x0010_a768 => {
                        // Table slot for sub-state 0: re-read the state.
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_a768);
                        }
                        continue 'restart;
                    }
                    0x0010_a7d8 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_CRUISE.call(c, 0x0010_a7e4)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a7ec => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_PREWIND.call(c, 0x0010_a7f8)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a800 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_GATE.call(c, 0x0010_a80c)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a814 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_NATURAL_AIR.call(c, 0x0010_a820)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a828 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_RAILSLIDE.call(c, 0x0010_a834)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a83c => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_SPIN.call(c, 0x0010_a848)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a850 => {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_WIPEOUT.call(c, 0x0010_a85c)?;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a864 => {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S1][0];
                        callee::THINK_BOARD_PRESS.call(c, 0x0010_a870)?;
                        c.cycles += 1;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    0x0010_a870 => {
                        // Sub-states without a step (3, 9..13).
                        c.cycles += 1;
                        c.g[V0][0] = SUB_STATE_SPIN;
                    }
                    other => {
                        c.jump_out(other, 0x0010_a768, 0x0010_a894);
                        return Ok(());
                    }
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = SUB_STATE_SPIN;
            }

            // Only a spinning rider keeps the "control taken over" word.
            c.cycles += 2;
            if c.g[S2][0] != c.g[V0][0] {
                Addr((c.g[S1][0] as u32).wrapping_add(RIDER_CONTROL_OVERRIDE)).sw(c, 0);
            }

            c.cycles += 6;
            c.lq(16, frame.at(c, 0x30));
            c.lq(17, frame.at(c, 0x20));
            c.lq(18, frame.at(c, 0x10));
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

// ---------------------------------------------------------------------------
// ComputerActorRider_CruiseFollowPath (0x00100680)

mod cruise {
    use super::{ee, Callee};

    pub const GET_STEER_TARGET: Callee = Callee {
        addr: 0x0010_0610,
        f: ee::s00::ComputerActorRider_GetSteerTarget,
    };
    pub const COMPUTE_STEER: Callee = Callee {
        addr: 0x0010_0348,
        f: ee::s00::ComputerActorRider_ComputeSteer,
    };
    pub const FIND_SPEED_MARKER: Callee = Callee {
        addr: 0x0010_bb18,
        f: ee::s00::ComputerActorRider_FindSpeedMarker,
    };
    pub const CROUCH_AND_BOOST: Callee = Callee {
        addr: 0x0010_c140,
        f: ee::s00::ComputerActorRider_CrouchAndBoost,
    };
    pub const SET_CRUISE_CROUCH: Callee = Callee {
        addr: 0x0010_c0a8,
        f: ee::s00::ComputerActorRider_SetCruiseCrouch,
    };
    pub const FIND_BOARD_PRESS_MARKER: Callee = Callee {
        addr: 0x0010_bbf8,
        f: ee::s00::ComputerActorRider_FindBoardPressMarker,
    };
    pub const FIND_JUMP_MARKER: Callee = Callee {
        addr: 0x0010_b980,
        f: ee::s00::ComputerActorRider_FindJumpMarker,
    };
    pub const SELECT_TARGET_RIDER: Callee = Callee {
        addr: 0x0010_bd10,
        f: ee::s00::ComputerActorRider_SelectTargetRider,
    };
}

/// Rider fields the path follower reads and writes.
const R_MODEL: u32 = 0x18;
/// The steering word of the control block (bits 6..11 hold the steering
/// byte, written as `(int)(angle * 31) << 6`).
const R_STEER: u32 = 0x4;
/// The rival slot the AI manager is indexed with.
const R_RIVAL: u32 = 0x80;
/// Distance to the next speed marker (written by the marker search).
const R_MARKER_DIST: u32 = 0xdf0;
/// Distance to the speed marker the crouch check uses.
const R_MARKER_DIST2: u32 = 0xdf8;
/// Board-press countdown.
const R_BOARD_PRESS: u32 = 0xe40;
/// Jump request / jump marker.
const R_JUMP: u32 = 0xe70;
/// `short` jump state.
const R_JUMP_STATE: u32 = 0xf38;
/// The 8-byte jump descriptor copied in.
const R_JUMP_DESC: u32 = 0xf44;
/// The rival's path interface inside the rider entry (`PMF: +0x40 delta,
/// +0x44 function`).
const R_PATH_IFACE: u32 = 0x6c0;
/// Model distances compared with the crouch thresholds.
const M_NEAR: u32 = 0x2f8;
const M_FAR: u32 = 0x2f0;
/// The rider model's forward axis for the distance to a speed marker.
const M_MARKER: u32 = 0x1e0;
/// The steering target the AI wrote into the frame.
const STEER_OUT: u32 = 0x10;

// The jump descriptors the original copies live at `0x450000 - 0x30e0`
// (`0x43cf20`), at `+0x18` (selected) and at `+0x10` (default).

/// `void ComputerActorRider_CruiseFollowPath(ComputerActorRider* rider,
/// ControlInput* in)` (0x00100680): the cruise thinking step. Computes the
/// steering (blending the AI's steering byte into the rider's flags),
/// reacts to speed markers (boost when one is near, crouch otherwise),
/// runs the board-press countdown and then resolves the jump state through
/// the rival's path interface.
///
/// The original's blocks are laid out sequentially in address order and
/// entered by falling out of the previous one; each `'b_…` label below is
/// one of those blocks.
pub fn computer_actor_rider_cruise_follow_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        c.sq(S0, frame.at(c, 0x50));
        c.sq(S1, frame.at(c, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.swc1(20, frame.at(c, 0x60));
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x30);
        c.g[AT][0] = 0x41f8_0000; // 31.0, the steering scale
        c.mtc1(1, 20);
        c.g[A1][0] = c.g[S0][0];
        cruise::GET_STEER_TARGET.call(c, 0x0010_06b0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        cruise::COMPUTE_STEER.call(c, 0x0010_06bc)?;

        // Pack the steering angle (6 bits) into the rider's flags.
        c.mul_s(0, 0, 20);
        c.g[A3][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V1][0] = 0xffff_ffff_fc0f_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[A3][0] = c.g[A3][0] & c.g[V1][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(STEER_OUT));
        c.g[V0][0] = c.g[V0][0] & 0x3f;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
        c.sw(c.g[S1][0] as u32, c.g[A3][0]);
        cruise::FIND_SPEED_MARKER.call(c, 0x0010_06f8)?;

        let no_marker = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        'b_1008e4: {
            'b_1008e0: {
                if !no_marker {
                    'b_100764: {
                        c.cycles += 18;
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x7fcc));
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_MODEL).lw(c);
                        c.add_s(0, 2, 3);
                        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(M_MARKER));
                        vu0_length_sq(c);
                        c.cfc2(3, 22);
                        c.mtc1(3, 1);
                        c.c_lt_s(0, 1);
                        let far = !c.fcc();
                        c.g[V0][0] = Addr(c.g[SP][0] as u32).off(STEER_OUT).lw(c);
                        if far {
                            break 'b_100764;
                        } else {
                            c.cycles += 2;
                            let no_target = c.g[V0][0] == 0;
                            c.g[V1][0] = 0xffff_ffff_ffff_ffc0;
                            if no_target {
                                break 'b_1008e0;
                            } else {
                                // Crouch: clear the boost bits of the flags.
                                c.cycles += 5;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                c.g[V0][0] = c.g[V0][0] | 0x1f;
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V0][0]);
                                break 'b_1008e0;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.sub_s(0, 2, 3);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        break 'b_1008e4;
                    } else {
                        'b_100790: {
                            c.cycles += 2;
                            let no_target = c.g[V0][0] == 0;
                            c.g[A1][0] = c.g[S1][0];
                            if no_target {
                                break 'b_100790;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                cruise::CROUCH_AND_BOOST.call(c, 0x0010_0788)?;
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                                break 'b_1008e4;
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        cruise::SET_CRUISE_CROUCH.call(c, 0x0010_0798)?;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
                        break 'b_1008e4;
                    }
                }
                {
                    // Board-press marker, written to the frame's +0x14.
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                    cruise::FIND_BOARD_PRESS_MARKER.call(c, 0x0010_07a8)?;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).off(0x14).lw(c);
                    if !none {
                        'b_1007c8: {
                            c.cycles += 4;
                            c.g[AT][0] = 0x3f80_0000; // 1.0
                            c.mtc1(1, 0);
                            let first = c.g[V0][0] == 0;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_BOARD_PRESS).lw(c);
                            if !first {
                                c.cycles += 2;
                                c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
                                c.mtc1(1, 0);
                            }
                        }
                        'b_100804: {
                            c.cycles += 2;
                            let idle = c.g[V0][0] != 0;
                            c.g[A0][0] = 0x3c;
                            if !idle {
                                // Start the press: 60 frames, opposite steer.
                                c.cycles += 13;
                                c.neg_s(0, 0);
                                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = 0xffff_ffff_ffff_f03f;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.mul_s(0, 0, 20);
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                c.g[V0][0] = c.g[V0][0] & 0x3f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS), c.g[A0][0]);
                                break 'b_1008e0;
                            }
                        }
                        'b_10084c: {
                            // Count the press down, steering opposite.
                            c.cycles += 2;
                            let expired = (c.g[V0][0] as i64) <= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            if !expired {
                                'b_10081c: {
                                    c.cycles += 2;
                                    let still = (c.g[V0][0] as i64) > 0;
                                    c.sw(
                                        (c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS),
                                        c.g[V0][0],
                                    );
                                    if !still {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                                        c.sw(
                                            (c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS),
                                            c.g[V0][0],
                                        );
                                    }
                                }
                                c.cycles += 12;
                                c.neg_s(0, 0);
                                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                                c.g[V0][0] = 0xffff_ffff_ffff_f03f;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.mul_s(0, 0, 20);
                                c.cvt_w_s(1, 0);
                                c.mfc1(2, 1);
                                c.g[V0][0] = c.g[V0][0] & 0x3f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[V1][0]);
                                break 'b_1008e0;
                            }
                        }
                        c.cycles += 11;
                        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(R_STEER).lw(c);
                        c.mul_s(0, 0, 20);
                        c.g[V1][0] = 0xffff_ffff_ffff_f03f;
                        c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                        c.cvt_w_s(1, 0);
                        c.mfc1(2, 1);
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(R_STEER), c.g[A0][0]);
                        break 'b_1008e0;
                    }
                }
                {
                    // No board press: drop the counter and crouch when the
                    // marker distance is outside the model's two thresholds.
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(R_BOARD_PRESS), 0);
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(R_MODEL).lw(c);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7fc8));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(M_NEAR));
                    c.c_le_s(1, 0);
                    let skip = !c.fcc();
                    c.g[A1][0] = c.g[S1][0];
                    if !skip {
                        c.cycles += 5;
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(M_FAR));
                        c.mtc1(0, 0); // f0 = 0.0
                        c.c_lt_s(0, 1);
                        let skip = !c.fcc();
                        if !skip {
                            c.cycles += 7;
                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST2));
                            c.g[AT][0] = 0x4248_0000; // 50.0
                            c.mtc1(1, 0); // f0 = 50.0
                            c.c_lt_s(0, 1);
                            let near = !c.fcc();
                            c.g[V1][0] = 0x2_0000;
                            if !near {
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = c.g[S1][0];
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                cruise::SET_CRUISE_CROUCH.call(c, 0x0010_08e0)?;
                break 'b_1008e0;
            }
            c.cycles += 1;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(R_MARKER_DIST));
            break 'b_1008e4;
        }

        // ------------------------------------------------------------ jumps
        'b_1009cc: {
            'b_1009c8: {
                'b_100934: {
                    // Look for a jump marker (five out pointers).
                    c.cycles += 11;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x28));
                    cruise::FIND_JUMP_MARKER.call(c, 0x0010_0908)?;
                    let none = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if !none {
                        c.cycles += 9;
                        c.g[V1][0] = 0x44_0000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(R_JUMP), c.g[V0][0]);
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30e0));
                        let src = c.g[A2][0] as u32;
                        jump_desc_copy(c, 2, src, 7, 0);
                        c.sh((c.g[S0][0] as u32).wrapping_add(R_JUMP_STATE), 0);
                        break 'b_1009c8;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
                cruise::SELECT_TARGET_RIDER.call(c, 0x0010_0940)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = Addr(c.g[SP][0] as u32).off(0x2c).lw(c);
                if !none {
                    'b_1009c4: {
                        'b_10096c: {
                            c.cycles += 2;
                            let empty = c.g[V0][0] == 0;
                            c.g[V0][0] = 0x44_0000;
                            if !empty {
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30e0));
                                let src = c.g[V0][0] as u32;
                                jump_desc_copy(c, 7, src, 0x1f, 0x18);
                                c.g[V1][0] = 3;
                                break 'b_1009c4;
                            }
                        }
                        // Ask the rival's path interface whether a jump is on.
                        c.cycles += 15;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(R_RIVAL).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
                        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(R_PATH_IFACE).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(R_PATH_IFACE));
                        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x40).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call::call_ptr(c, target, 0x0010_09a0)?;
                        if c.g[V0][0] != 0 {
                            c.lq(16, frame.at(c, 0x50));
                            break 'b_1009cc;
                        }
                        c.cycles += 7;
                        c.g[V0][0] = 0x44_0000;
                        c.g[V1][0] = 2;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30e0));
                        let src = c.g[V0][0] as u32;
                        jump_desc_copy(c, 7, src, 0x17, 0x10);
                        break 'b_1009c4;
                    }
                    c.cycles += 1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(R_JUMP_STATE), c.g[V1][0]);
                    break 'b_1009c8;
                }
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x50));
            break 'b_1009cc;
        }

        c.cycles += 5;
        c.lq(17, frame.at(c, 0x40));
        frame.restore_ra(c, 0x30);
        c.lwc1(20, frame.at(c, 0x60));
        frame.close(c);
        Ok(())
    });
}

/// The VU0 macro sequence that writes the squared length of the vector
/// loaded into VF3 to the accumulator (`madd` into VF4) and its square root
/// into Q (`sqrt`); the original then reads Q with `cfc2 rt, vi22`.
#[inline]
fn vu0_length_sq(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// `ldl/ldr` an 8-byte descriptor at `src + lo` / `src + hi` into GPR `reg`,
/// then `sdl/sdr` it to the rider's jump descriptor (the `ldl`/`sdl` at +7,
/// the `ldr`/`sdr` at +0).
#[inline]
fn jump_desc_copy(c: &mut Cpu, reg: usize, src: u32, lo: u32, hi: u32) {
    c.ldl(reg, src.wrapping_add(lo));
    c.ldr(reg, src.wrapping_add(hi));
    let dst = (c.g[S0][0] as u32).wrapping_add(R_JUMP_DESC);
    c.sdl(reg, dst.wrapping_add(7));
    c.sdr(reg, dst);
}

// ---------------------------------------------------------------------------
// vfprintf_dispatch (0x0041b658)

/// The conversion table the original indexes with the specifier: word
/// `c - 'E'` is the address of the handler block (0x450000 + 0x6a30).
const SPEC_TABLE: u32 = 0x0045_6a30;
const SPEC_FIRST: u32 = 0x45; // 'E'
const SPEC_COUNT: u64 = 0x23;

/// The two handler blocks of the table: the float-capable formatter and
/// "keep scanning".
const HANDLER_VFPRINTF: u32 = 0x0041_b6e8;
const HANDLER_CONTINUE: u32 = 0x0041_b6fc;

/// `int vfprintf(FILE* fp, const char* fmt, va_list ap)` (0x0041b658): choose
/// between `_vfprintf_r` (a float conversion is present) and
/// `_vfiprintf_r` (integer conversions only). The format is scanned once:
/// `%` starts a specifier, the flags/width/precision (anything below `'A'`)
/// are skipped, and the first byte at or above `'A'` is looked up in the
/// specifier table.
pub fn vfprintf_dispatch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[T0][0] as u32).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[T0][0];
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lb(c);
            'scan: loop {
                c.cycles += 3;
                c.g[V0][0] = 0x25; // '%'
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                    let trailing = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if !trailing {
                        // Skip flags/width/precision (bytes below 'A'); stop
                        // at the first specifier byte (or the terminator).
                        loop {
                            c.cycles += 4;
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 65i64) as u64;
                            if c.g[V0][0] != 0 {
                                c.cycles += 3;
                                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(1)).lb(c);
                                if c.g[V0][0] != 0 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0041_b698);
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                            }
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
                            break;
                        }
                        c.cycles += 6;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(SPEC_FIRST));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.g[V1][0] = (c.g[A0][0] < SPEC_COUNT) as u64;
                        let out_of_range = c.g[V1][0] == 0;
                        c.g[V0][0] = 0x4a0000;
                        if !out_of_range {
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ce0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            match c.g[A0][0] as u32 {
                                HANDLER_VFPRINTF => {
                                    c.cycles += 5;
                                    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x54)).lw(c);
                                    c.g[A2][0] = c.g[T0][0];
                                    callee::VFPRINTF_R.call(c, 0x0041_b6f4)?;
                                    c.g[RA][0] = Addr(call::sp(c)).ld(c);
                                    c.cycles += 2;
                                    frame.close(c);
                                    return Ok(());
                                }
                                HANDLER_CONTINUE => {}
                                other => {
                                    c.jump_out(other, 0x0041_b658, 0x0041_b724);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    }
                }
                // Back edge: look for the next '%' (or the terminator).
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).lb(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0041_b680);
                    }
                    continue 'scan;
                }
                break;
            }
        }
        // Tail call into the chosen formatter.
        c.cycles += 4;
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x54)).lw(c);
        c.g[A2][0] = c.g[T0][0];
        callee::VFIPRINTF_R.call(c, 0x0041_b718)?;
        c.g[RA][0] = Addr(call::sp(c)).ld(c);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fill, put_vec, put_words, DATA, GAME_GP};

    const SEEDS: u32 = 240;

    // ---------------------------------------------------------------- rider

    // Fixture addresses (well above the fake callees' returned pointers,
    // `DATA + (hash & 0x3fff0)`).
    const RIDER: u32 = DATA + 0x8_0000;
    const STATE: u32 = DATA + 0x8_1000;
    const MODEL: u32 = DATA + 0x8_2000;
    const INPUT: u32 = DATA + 0x8_3000;
    const HUMAN: u32 = DATA + 0x8_4000;
    const WORLD: u32 = DATA + 0x8_5000;
    const BE_LIBRARY: u32 = DATA + 0x8_6000;
    const BE_IFACE: u32 = DATA + 0x8_7000;
    const BE_VTABLE: u32 = DATA + 0x8_7100;

    /// A CPU rider (at `RIDER`) whose model (`MODEL`) is at `+0x18`; the
    /// model's sub-state machine (`STATE`, read through `+0x77c`) announces
    /// sub-state `seed % 16` — every arm of the dispatch plus the two
    /// "no step" cases. The world/AI chain (gp - 0x848) has no riders; the
    /// BE library and its interface table exist so `ComputeRubberBand` can
    /// run both of its paths. Fake callee results are forced small: they are
    /// used as loop bounds by the thinking steps.
    fn rider_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 8);
        put_words(c, LOG_RESULTS, &[1, 2, 1, 3, 1, 2, 1, 4]);

        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD + 0x100);
        c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
        c.bus.write32(WORLD + 0x200 + 0x78, 0);
        c.bus.write32(WORLD + 0x200 + 0x7c, 0);
        c.bus
            .write32(WORLD + 0x200 + 0x28, if seed % 2 == 0 { HUMAN } else { 0 });

        // The BE library singleton (its hot path) and interface slot 0,
        // whose virtual method is a fake.
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, BE_IFACE);
        c.bus.write32(BE_IFACE + 4, BE_VTABLE);
        c.bus.write16(BE_VTABLE + 8, 0);
        c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);

        c.bus.write32(RIDER + 0x18, MODEL);
        // `Rider_GetSubState` is called with the model (`*(rider + 0x18)`),
        // which reads its state machine at `+0x77c` and the sub-state at
        // its `+0xde4`.
        c.bus.write32(MODEL + 0x77c, STATE);
        c.bus.write32(STATE + 0xde4, seed % 16);

        // `ComputeRubberBand`: the model's sub-state (`0` exits early), the
        // "is a human rider" words the rider scan at `MODEL` reads, and the
        // distance that decides between its two tails.
        c.bus.write32(
            MODEL + 0xe4,
            if seed % 3 == 0 {
                0
            } else {
                r.next_u32() % 4 + 1
            },
        );
        let scans = seed % 2 == 0;
        c.bus.write32(MODEL, scans as u32);
        c.bus.write32(MODEL + 4, scans as u32);
        c.bus.write32(
            MODEL + 0x4d0,
            if seed % 4 == 0 {
                r.f32(1000.0)
            } else {
                0xc000_0000
            },
        );
        c.bus.write32(HUMAN + 0x4d0, r.f32(1000.0));
        c.bus.write8(0x0053_5c11, (seed % 3) as u8);
        // The AI frame counter and the shove cooldown the steps read.
        c.bus.write32(WORLD + 0x200 + 8, r.next_u32() % 32);
        c.bus.write32(RIDER + 0xf30, r.edge_f32());
        c.bus.write32(RIDER + 0xf34, r.next_u32() % 2);

        c.g[A0][0] = RIDER as u64;
        c.g[A1][0] = INPUT as u64;
        c.pc = 0x0010_a768;
    }

    #[test]
    fn computer_actor_rider_get_control_input_matches() {
        check(
            ssx3_ee::s00::f_0010a768__generated,
            computer_actor_rider_get_control_input,
            SEEDS,
            &|c, seed| rider_setup(c, seed, 0x105),
        );
    }

    // -------------------------------------------------------------- vfprintf

    /// Format strings that reach every scan path. `%K` is made to select a
    /// table word that is not a handler (the `jump_out` arm, which the test
    /// machine turns into a fake call); `%M` is a real "keep scanning" word;
    /// `%L` is a real `_vfprintf_r` word and, being only a length modifier,
    /// leaves the formatter at the end of the string (no conversion argument
    /// is fetched, so no float formatting runs).
    const FORMATS: [&[u8]; 9] = [
        b"", b"%", b"a%", b"a%K", b"%1K", b"%M%K", b"%K", b"%%K", b"%L",
    ];

    /// The specifier table word for `c` (the original indexes it by
    /// `c - 'E'`).
    fn spec_slot(ch: u8) -> u32 {
        SPEC_TABLE + (ch as u32 - SPEC_FIRST) * 4
    }

    fn printf_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x106, 0);
        // The locale, the stdio globals and the specifier table are the
        // game's own data.
        elf_image(c);
        // FILE at DATA: zeroed, only the reent pointer at +0x54 is used.
        c.bus.write32(DATA + 0x54, DATA + 0x800);
        let fmt = DATA + 0x100;
        let bytes = FORMATS[seed as usize % FORMATS.len()];
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(fmt + i as u32, *b);
        }
        c.bus.write8(fmt + bytes.len() as u32, 0);
        // Break the 'K' slot: the dispatcher must take its out-of-range arm.
        c.bus.write32(spec_slot(b'K'), 0xdead_beef);
        // 'M' keeps the real "keep scanning" word; make sure it is there.
        c.bus.write32(spec_slot(b'M'), 0x0041_b6fc);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = fmt as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        c.pc = 0x0041_b658;
    }

    #[test]
    fn vfprintf_dispatch_matches() {
        check(
            ssx3_ee::s15::f_0041b658__generated,
            vfprintf_dispatch,
            SEEDS,
            &printf_setup,
        );
    }

    // ------------------------------------------------------- cruise follow

    const RIDER_ENTRY: u32 = DATA + 0x8_8000;
    const PMF: u32 = DATA + 0x8_9000;

    /// The cruise step's inputs: the rider (`a0`: model at `+0x18`, the
    /// marker/jump fields) and its control block (`a1`: flags at `+0`,
    /// steering at `+4`, model and rival slot at `+0x18`/`+0x80`). The AI
    /// manager (gp - 0x848 -> world -> game module -> AI) has a few riders
    /// whose `+0x6c0` path interface is a fake PMF, so the "is a jump on"
    /// answer comes from the test machine.
    fn cruise_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x107, 6);
        put_words(c, LOG_RESULTS, &[1, 2, 1, 3, 1, 1, 2, 1]);

        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD + 0x100);
        c.bus.write32(WORLD + 0x100 + 0xc, WORLD + 0x200);
        c.bus.write32(WORLD + 0x200 + 0x28, WORLD + 0x300);
        let riders = if seed % 3 == 0 { 0 } else { 1 + seed % 3 };
        c.bus.write32(WORLD + 0x200 + 0x78, riders);
        c.bus.write32(WORLD + 0x200 + 0x7c, riders);
        c.bus.write32(WORLD + 0x200 + 8, r.next_u32() % 64);
        for i in 0..3u32 {
            let entry = RIDER_ENTRY + 0x100 * i;
            c.bus.write32(WORLD + 0x300 + 4 * i, entry);
            c.bus.write32(entry + 0x6c0, PMF);
            fill(c, &mut r, entry, 8);
        }
        c.bus.write16(PMF + 0x40, 0);
        c.bus.write32(PMF + 0x44, FAKE + 0x20);

        // The rider: model, marker distances, board press, jump state.
        c.bus.write32(RIDER + 0x18, MODEL);
        c.bus.write32(RIDER + 0xdf0, r.edge_f32());
        c.bus.write32(RIDER + 0xdf8, r.edge_f32());
        c.bus.write32(RIDER + 0xe40, r.next_u32() % 4);
        c.bus.write16(RIDER + 0xf38, (r.next_u32() % 4) as u16);
        c.bus.write32(RIDER + 0xe70, r.next_u32());
        // The control block: flags, steering byte, model, rival slot.
        c.bus.write32(STATE, r.next_u32() & !0x000c_0000);
        c.bus.write32(STATE + 0x4, r.next_u32());
        c.bus.write32(STATE + 0x18, MODEL);
        c.bus.write32(STATE + 0x80, r.next_u32() % 3);

        // The model: the marker list owner (a fake virtual at +0xab8), the
        // forward axis, the crouch distances and the steer-target corners.
        c.bus.write32(MODEL + 0xab8, 0);
        put_vec(
            c,
            MODEL + 0x1e0,
            [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
        );
        c.bus.write32(MODEL + 0x4c0, r.f32(100.0));
        c.bus.write32(MODEL + 0x4c4, r.f32(100.0));
        c.bus.write32(MODEL + 0x2f0, r.f32(1000.0));
        c.bus.write32(MODEL + 0x2f8, r.f32(1000.0));
        c.bus.write32(MODEL + 0x110, r.edge_f32());
        c.bus.write32(MODEL + 0x490, r.edge_f32());
        c.bus.write32(MODEL + 0x4b0, r.edge_f32());

        c.g[A0][0] = RIDER as u64;
        c.g[A1][0] = STATE as u64;
        c.pc = 0x0010_0680;
    }

    #[test]
    fn computer_actor_rider_cruise_follow_path_matches() {
        check(
            ssx3_ee::s00::f_00100680__generated,
            computer_actor_rider_cruise_follow_path,
            SEEDS,
            &cruise_setup,
        );
    }
}
