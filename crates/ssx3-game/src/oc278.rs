//! Staging wave 210 (oc278): hand-written replacements for the hottest
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
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0010_5d98, rider_handle_impact, "Rider_HandleImpact")];

// ---------------------------------------------------------------------------
// Rider_HandleImpact (0x105d98)

/// Fields `Rider_HandleImpact` reads and writes.
mod impact {
    /// Rider: the board axes (rows of the world matrix) and origin.
    pub const AXIS_SIDE: u32 = 0x160;
    pub const AXIS_FORWARD: u32 = 0x170;
    pub const AXIS_UP: u32 = 0x180;
    pub const ORIGIN: u32 = 0x190;
    /// `mMatrix.row[2]` (its z is the up-tilt used by the ground clamp).
    pub const MATRIX_ROW2: u32 = 0x1c0;
    pub const VELOCITY: u32 = 0x1e0;
    /// Launch argument for the air predictor (`unk_2E4`).
    pub const LAUNCH_ARG: u32 = 0x2e4;
    /// Last impact normal and the accumulated impact strength.
    pub const IMPULSE: u32 = 0x3e0;
    pub const IMPACT_ACCUM: u32 = 0x3f0;
    /// The rider's interface vtable pointer (methods called with `+0x6c0`).
    pub const VTBL: u32 = 0x6c0;
    pub const HANDLER: u32 = 0x77c;
    pub const ANIM: u32 = 0x784;
    pub const AIR_PREDICTOR: u32 = 0x788;
    /// Vtable slots (`{short delta; short; fn}`).
    pub const VT_VIBRATION_BUMP: u32 = 0x88;
    pub const VT_GET_POSITION: u32 = 0x28;
    pub const VT_GET_VELOCITY: u32 = 0x10;
    /// Handler (owner) fields.
    pub const HANDLER_AIR_STATE: u32 = 0x30;
    pub const HANDLER_HIT_IN_AIR: u32 = 0x314;
    pub const HANDLER_HIT_VELOCITY: u32 = 0x320;
    /// Impact info: contact point, normal, impact speed.
    pub const INFO_POS: u32 = 0x0;
    pub const INFO_NORMAL: u32 = 0x20;
    pub const INFO_NORMAL_Z: u32 = 0x28;
    pub const INFO_SPEED: u32 = 0x30;
    /// Animation id meaning "no wipeout: just bump".
    pub const NO_ANIM: u64 = 0x1b6;
    /// gp-relative constants.
    pub const STEEP_NORMAL_Z: i32 = -0x7f08; // 0.8
    pub const AIR_SIDE_SPEED: i32 = -0x7f04; // 416.532
    pub const AIR_FRONT_SPEED: i32 = -0x7f00; // 833.338
    pub const CLAMP_MIN: i32 = -0x7efc; // 0.2
    pub const CLAMP_MAX: i32 = -0x7ef8; // 1.4
    pub const GROUND_SIDE_SPEED: i32 = -0x7ef4; // 1388.88
    pub const GROUND_FRONT_SPEED: i32 = -0x7ef0; // 2083.23
    pub const FAST_SPEED: i32 = 0x1b94; // D_004A4C84
}

const IMPACT_GET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fee8,
    f: ee::s00::Rider_GetSubState,
};
const IMPACT_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
const IMPACT_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};
const IMPACT_AIR_PREDICTOR_PRIME: Callee = Callee {
    addr: 0x0011_35b8,
    f: ee::s00::AirPredictor_Prime,
};
const IMPACT_START_WIPEOUT: Callee = Callee {
    addr: 0x0010_eb30,
    f: ee::s00::Rider_StartWipeout,
};
const IMPACT_PLAY_BUMP_ANIM: Callee = Callee {
    addr: 0x0010_8388,
    f: ee::s00::Rider_PlayBumpAnim,
};
/// `AnimController_GetLayerSequence` (through the function table).
const IMPACT_ANIM_LAYER_SEQUENCE: u32 = 0x0031_1ae8;
/// `Rand_NextGame` (AIrand, through the function table).
const IMPACT_RAND: u32 = 0x0031_7810;

/// `reg + off` (32-bit address).
#[inline(always)]
fn impact_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off`.
#[inline(always)]
fn impact_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The compiler's inline VU0 dot product: vf4.x = vf3 . vf(other) (x, y, z
/// and w; vf6.x = 1).
fn impact_dot(c: &mut Cpu, other: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(other));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// f0 = |d1|, f1 = |d0|, f2 = |d2| (d0..d2 in f22, f20, f21) and whether
/// |d0| < |d1| (6 cycles).
fn impact_abs_dots(c: &mut Cpu) -> bool {
    c.cycles += 6;
    c.abs_s(0, 20);
    c.abs_s(1, 22);
    c.c_lt_s(1, 0);
    c.abs_s(2, 21);
    c.fcc()
}

/// How a branch of the animation choice ends.
#[derive(Clone, Copy, PartialEq)]
enum ImpactTail {
    /// The end of the choice block (`+1` cycle, v0 = 0x1b6), then `Choose`.
    Picked,
    /// Compare s0 with v0 (0x1b6): wipe out or bump.
    Choose,
    /// Straight to the bump animation.
    Bump,
}

/// Animation ids and return addresses of a side (forward-axis) hit.
struct ImpactSideHit {
    /// Close head-on hit at speed.
    close: u64,
    /// Random head-on wipeouts for rolls 0, 1, 2, 3 and 4..5.
    roll: [u64; 5],
    /// Glancing hit: odd / even random.
    glance: u64,
    glance_even: u64,
    ret_glance: u32,
    ret_roll6: u32,
    ret_roll3: u32,
}

/// Main state 1 (riding): the side-hit set.
const IMPACT_RIDING_SIDE: ImpactSideHit = ImpactSideHit {
    close: 0x156,
    roll: [0x15b, 0x157, 0x158, 0x15a, 0x159],
    glance: 0x155,
    glance_even: 0x154,
    ret_glance: 0x0010_6264,
    ret_roll6: 0x0010_61fc,
    ret_roll3: 0x0010_620c,
};

/// Other main states: the side-hit set.
const IMPACT_GROUND_SIDE: ImpactSideHit = ImpactSideHit {
    close: 0x14c,
    roll: [0x14e, 0x14f, 0x151, 0x14d, 0x150],
    glance: 0x14b,
    glance_even: 0x14a,
    ret_glance: 0x0010_646c,
    ret_roll6: 0x0010_6404,
    ret_roll3: 0x0010_6414,
};

/// `void Rider_HandleImpact(Rider* rider, ImpactInfo* info, int surface)`
/// (0x105d98): the rider hit something. Unless frozen (sub state 9): add
/// `1 - max(info.normal . impulse, 0)` to the impact accumulator and keep
/// the normal; a fatal surface resets the rider to the path. Otherwise rumble
/// (`vibrationBump(speed)`). In the air (main state 2) the hit is only
/// recorded on the handler (and the air predictor re-primed when the
/// handler flies). On the ground it picks a wipeout animation from which
/// board axis the normal mostly faces (side, forward, up), the impact speed
/// (scaled by tilt off the riding state), the distance above the board and
/// the rider's speed (random variants), and starts the wipeout; with no
/// wipeout the bump animation plays.
pub fn rider_handle_impact(c: &mut Cpu) {
    use impact::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x30);
        for (f, off) in [(23, 0xb8), (22, 0xb0), (21, 0xa8), (20, 0xa0)] {
            frame.save_fpr(c, f, off);
        }
        IMPACT_GET_SUB_STATE.call(c, 0x0010_5dd8)?;
        c.g[V1][0] = 9;
        let frozen = c.g[V0][0] == c.g[V1][0];
        frame.restore(c, S0, 0x90);
        if frozen {
            impact_epilogue(c, frame);
            return Ok(());
        }

        // impactAccum += 1 - max(normal . impulse, 0); impulse = normal.
        c.cycles += 21;
        c.mtc1(0, 3);
        c.lqc2(3, impact_at(c, S3, INFO_NORMAL));
        c.lqc2(5, impact_at(c, S1, IMPULSE));
        impact_dot(c, 5);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.g[V1][0] = u64::MAX;
        c.max_s(1, 1, 3);
        c.lwc1(0, impact_at(c, S1, IMPACT_ACCUM));
        c.sub_s(2, 2, 1);
        c.add_s(0, 0, 2);
        c.swc1(0, impact_at(c, S1, IMPACT_ACCUM));
        c.lq(V0, impact_at(c, S3, INFO_NORMAL));
        let no_surface = c.g[S5][0] == c.g[V1][0];
        c.sq(V0, impact_at(c, S1, IMPULSE));
        if !no_surface {
            // world->game->surfaces[surface].fatal (0xb0 bytes each).
            c.cycles += 9;
            c.g[A1][0] = c.lw(impact_gp(c, -0x848));
            c.g[V1][0] = 0xb0;
            c.mult(0, V1, S5, V1);
            c.g[A0][0] = c.lw(impact_at(c, A1, 0x84));
            c.g[V0][0] = c.lw(impact_at(c, A0, 0x44));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.lw(impact_at(c, V0, 0x44));
            let fatal = c.g[A0][0] != 0;
            c.g[A1][0] = 0;
            if fatal {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = 1;
                IMPACT_CHECK_RESET_PATH.call(c, 0x0010_5e68)?;
                frame.restore(c, S0, 0x90);
                impact_epilogue(c, frame);
                return Ok(());
            }
        }

        // vibrationBump(info.speed).
        c.cycles += 12;
        c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
        c.g[S2][0] = sx(impact_at(c, S1, VTBL));
        c.lwc1(12, impact_at(c, S3, INFO_SPEED));
        c.g[A0][0] = c.lh(impact_at(c, V1, VT_VIBRATION_BUMP));
        c.g[V0][0] = c.lw(impact_at(c, V1, VT_VIBRATION_BUMP + 4));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0010_5e8c)?;
        c.g[A0][0] = c.g[S1][0];
        IMPACT_GET_MAIN_STATE.call(c, 0x0010_5e94)?;
        c.g[V1][0] = 2;
        if c.g[V0][0] == c.g[V1][0] {
            impact_in_air(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
            impact_epilogue(c, frame);
            return Ok(());
        }
        c.g[A0][0] = c.g[S1][0];

        // Riding (main state 1): prime the air predictor.
        c.cycles += 4;
        c.g[S4][0] = 1;
        IMPACT_GET_MAIN_STATE.call(c, 0x0010_5f10)?;
        if c.g[V0][0] == c.g[S4][0] {
            c.cycles += 17;
            impact_prime_air_predictor(c, [0x0010_5f2c, 0x0010_5f44, 0x0010_5f58])?;
        }
        c.lq(V0, impact_at(c, S1, AXIS_SIDE));

        impact_measure(c)?;
        c.g[A0][0] = c.lw(impact_at(c, S1, ANIM));
        call_ptr(c, IMPACT_ANIM_LAYER_SEQUENCE, 0x0010_6038)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        IMPACT_GET_SUB_STATE.call(c, 0x0010_6044)?;
        c.g[V1][0] = 0xb;
        let wiping_out = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = NO_ANIM;
        let mut tail = if wiping_out {
            ImpactTail::Choose
        } else {
            impact_pick_anim(c)?
        };

        if tail == ImpactTail::Picked {
            c.cycles += 1;
            c.g[V0][0] = NO_ANIM;
            tail = ImpactTail::Choose;
        }
        if tail == ImpactTail::Choose {
            c.cycles += 2;
            let none = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if !none {
                // Rider_StartWipeout(rider, anim, 0, surface, info).
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = c.g[S3][0];
                c.g[A2][0] = 0;
                IMPACT_START_WIPEOUT.call(c, 0x0010_64ec)?;
                frame.restore(c, S0, 0x90);
                impact_epilogue(c, frame);
                return Ok(());
            }
        }
        // Rider_PlayBumpAnim(rider, info, 1).
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 1;
        IMPACT_PLAY_BUMP_ANIM.call(c, 0x0010_6504)?;
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        impact_epilogue(c, frame);
        Ok(())
    })
}

/// Restore s1-s5, ra, f20-f23 and return (12 cycles).
fn impact_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore_ra(c, 0x30);
    for (f, off) in [(23, 0xb8), (22, 0xb0), (21, 0xa8), (20, 0xa0)] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

/// In the air (main state 2): when the handler flies (air state 1), prime the
/// air predictor; flag the hit on the handler and keep the velocity.
fn impact_in_air(c: &mut Cpu) -> crate::call::Flow {
    use impact::*;
    c.cycles += 5;
    c.g[V1][0] = c.lw(impact_at(c, S1, HANDLER));
    c.g[S3][0] = 1;
    c.g[V0][0] = c.lw(impact_at(c, V1, HANDLER_AIR_STATE));
    if c.g[V0][0] == c.g[S3][0] {
        c.cycles += 18;
        impact_prime_air_predictor(c, [0x0010_5ec8, 0x0010_5ee0, 0x0010_5ef4])?;
        c.g[V1][0] = c.lw(impact_at(c, S1, HANDLER));
    }
    c.sw(impact_at(c, V1, HANDLER_HIT_IN_AIR), c.g[S3][0]);
    c.cycles += 3;
    c.lq(V0, impact_at(c, S1, VELOCITY));
    c.sq(V0, impact_at(c, V1, HANDLER_HIT_VELOCITY));
    Ok(())
}

/// `AirPredictor_Prime(rider->airPredictor, getPosition(), getVelocity(),
/// rider->launchArg)` (s2 = rider + 0x6c0, the method `this` base).
fn impact_prime_air_predictor(c: &mut Cpu, rets: [u32; 3]) -> crate::call::Flow {
    use impact::*;
    c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
    c.g[A0][0] = c.lh(impact_at(c, V1, VT_GET_POSITION));
    c.g[V0][0] = c.lw(impact_at(c, V1, VT_GET_POSITION + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[0])?;
    c.g[V1][0] = c.lw(impact_at(c, S1, VTBL));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.lh(impact_at(c, V1, VT_GET_VELOCITY));
    c.g[V0][0] = c.lw(impact_at(c, V1, VT_GET_VELOCITY + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[1])?;
    c.g[A0][0] = c.lw(impact_at(c, S1, AIR_PREDICTOR));
    c.g[A1][0] = c.g[S0][0];
    c.lwc1(12, impact_at(c, S1, LAUNCH_ARG));
    c.g[A2][0] = c.g[V0][0];
    IMPACT_AIR_PREDICTOR_PRIME.call(c, rets[2])
}

/// The 61-cycle block before the layer query: d0 = normal . side (f22),
/// d1 = normal . forward (f20), d2 = normal . up (f21) and the contact
/// height above the board, (pos - origin) . up (f23). v0 holds the side axis.
fn impact_measure(c: &mut Cpu) -> crate::call::Flow {
    use impact::*;
    c.cycles += 61;
    c.g[A1][0] = 2;
    c.lq(A0, impact_at(c, S1, AXIS_FORWARD));
    c.g[S0][0] = NO_ANIM;
    let s = sp(c);
    c.sq(V0, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A2, 4);
    c.mtc1(A2, 22);
    c.lq(V1, impact_at(c, S1, AXIS_UP));
    c.sq(A0, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A2, 4);
    c.mtc1(A2, 20);
    c.lq(V0, impact_at(c, S1, ORIGIN));
    c.sq(V1, s);
    c.lqc2(3, s);
    c.lqc2(5, impact_at(c, S3, INFO_NORMAL));
    impact_dot(c, 5);
    c.qmfc2(A0, 4);
    c.mtc1(A0, 21);
    c.sq(V0, s);
    c.lqc2(3, impact_at(c, S3, INFO_POS));
    c.lqc2(4, s);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(0x20));
    c.lq(V0, s.wrapping_add(0x20));
    c.lq(V1, impact_at(c, S1, AXIS_UP));
    c.sq(V0, s.wrapping_add(0x10));
    c.sq(V1, s.wrapping_add(0x20));
    c.lqc2(3, s.wrapping_add(0x10));
    c.lqc2(5, s.wrapping_add(0x20));
    impact_dot(c, 5);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 23);
    Ok(())
}

/// The animation choice when the rider is not already wiping out (sub
/// state != 0xb). s2 = the current layer-2 sequence, s0 = 0x1b6.
fn impact_pick_anim(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    IMPACT_GET_MAIN_STATE.call(c, 0x0010_6058)?;
    if c.g[V0][0] == c.g[S4][0] {
        // Riding into a steep wall (normal.z > 0.8)?
        c.cycles += 6;
        c.lwc1(1, impact_at(c, S3, INFO_NORMAL_Z));
        c.lwc1(0, impact_gp(c, STEEP_NORMAL_Z));
        c.c_lt_s(0, 1);
        let steep = c.fcc();
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x14));
        if steep {
            impact_pick_steep(c);
            return Ok(ImpactTail::Picked);
        }
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    IMPACT_GET_MAIN_STATE.call(c, 0x0010_6138)?;
    c.g[V1][0] = 1;
    if c.g[V0][0] != c.g[V1][0] {
        c.lq(V0, impact_at(c, S1, MATRIX_ROW2));
        return impact_pick_ground(c);
    }
    impact_pick_riding(c)
}

/// Riding into a steep wall: sequences 0x14/0x15 -> 0x169; else by the
/// dominant axis: forward 0x162/0x163, up 0x15e (or 0x15f from sequences
/// 0x12/0x13), side 0x165/0x164. v0 = sequence - 0x14.
fn impact_pick_steep(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S0][0] = 0x169;
        return;
    }
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(20, 0);
            let back = c.fcc();
            c.g[S0][0] = 0x163;
            if back {
                c.cycles += 2;
                c.g[S0][0] = 0x162;
            }
            return;
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(21, 0);
            let below = c.fcc();
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x12));
            if below {
                c.cycles += 2;
                c.g[S0][0] = 0x15e;
                return;
            }
            c.cycles += 5;
            c.g[V1][0] = 1;
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = 0x15f;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            return;
        }
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x164;
    if left {
        c.cycles += 2;
        c.g[S0][0] = 0x165;
    }
}

/// Riding (main state 1), not steep: forward hits above 416.5 (side
/// set), up hits 0x15e / 0x15f (sequences 0x12..0x15), side hits above 833.3
/// 0x153/0x152; any wipeout from sequences 0x14/0x15 becomes 0x169.
fn impact_pick_riding(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    let via_end = impact_pick_riding_anim(c)?;
    if via_end {
        c.cycles += 1;
        c.g[V0][0] = NO_ANIM;
    }
    c.cycles += 2;
    let none = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x14));
    if none {
        return Ok(ImpactTail::Bump);
    }
    c.cycles += 5;
    c.g[V1][0] = 1;
    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[A0][0] = 0x169;
    if c.g[V1][0] == 0 {
        c.g[S0][0] = c.g[A0][0];
    }
    Ok(ImpactTail::Picked)
}

/// The riding choice proper; true when it leaves through the block end
/// (`+1`, v0 = 0x1b6), false when it jumps to the compare with v0 = 0x1b6.
fn impact_pick_riding_anim(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use impact::*;
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 6;
            c.lwc1(1, impact_gp(c, AIR_SIDE_SPEED));
            c.lwc1(0, impact_at(c, S3, INFO_SPEED));
            c.c_lt_s(1, 0);
            let hard = c.fcc();
            c.g[V0][0] = NO_ANIM;
            if !hard {
                return Ok(false);
            }
            return impact_side_hit(c, &IMPACT_RIDING_SIDE);
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(21, 0);
            let below = c.fcc();
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x12));
            if below {
                c.cycles += 2;
                c.g[S0][0] = 0x15e;
                return Ok(true);
            }
            c.cycles += 5;
            c.g[V1][0] = 3;
            c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            c.g[A0][0] = 0x15f;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            return Ok(true);
        }
    }
    c.cycles += 6;
    c.lwc1(1, impact_gp(c, AIR_FRONT_SPEED));
    c.lwc1(0, impact_at(c, S3, INFO_SPEED));
    c.c_lt_s(1, 0);
    let hard = c.fcc();
    c.g[V0][0] = NO_ANIM;
    if !hard {
        return Ok(false);
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x152;
    if !left {
        return Ok(false);
    }
    c.cycles += 1;
    c.g[S0][0] = 0x153;
    Ok(true)
}

/// Other main states: the speed limits scale with
/// k = clamp(row2.z + 1 - |normal.z|, 0.2, 1.4) (f4); forward hits above
/// k * 1388.9 (side set), up hits nothing, side hits above k * 2083.2
/// 0x149/0x148. v0 holds `mMatrix.row[2]`.
fn impact_pick_ground(c: &mut Cpu) -> Result<ImpactTail, crate::call::Unwound> {
    use impact::*;
    c.cycles += 15;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    let s = sp(c);
    c.sq(V0, s.wrapping_add(0x20));
    c.lwc1(2, impact_gp(c, CLAMP_MIN));
    c.lwc1(0, s.wrapping_add(0x28));
    c.lwc1(1, impact_at(c, S3, INFO_NORMAL_Z));
    c.mov_s(4, 2);
    c.add_s(0, 0, 3);
    c.abs_s(1, 1);
    c.lwc1(3, impact_gp(c, CLAMP_MAX));
    c.sub_s(0, 0, 1);
    c.c_le_s(2, 0);
    if c.fcc() {
        c.min_s(4, 0, 3);
    }
    if impact_abs_dots(c) {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.cycles += 7;
            c.lwc1(0, impact_gp(c, GROUND_SIDE_SPEED));
            c.lwc1(1, impact_at(c, S3, INFO_SPEED));
            c.mul_s(0, 4, 0);
            c.c_lt_s(0, 1);
            let hard = c.fcc();
            c.g[V0][0] = NO_ANIM;
            if !hard {
                return Ok(ImpactTail::Choose);
            }
            return Ok(if impact_side_hit(c, &IMPACT_GROUND_SIDE)? {
                ImpactTail::Picked
            } else {
                ImpactTail::Choose
            });
        }
    }
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 4;
        c.c_lt_s(0, 2);
        let up = c.fcc();
        c.g[V0][0] = NO_ANIM;
        if up {
            return Ok(ImpactTail::Choose);
        }
    }
    c.cycles += 7;
    c.lwc1(0, impact_gp(c, GROUND_FRONT_SPEED));
    c.lwc1(1, impact_at(c, S3, INFO_SPEED));
    c.mul_s(0, 4, 0);
    c.c_lt_s(0, 1);
    let hard = c.fcc();
    c.g[V0][0] = NO_ANIM;
    if !hard {
        return Ok(ImpactTail::Choose);
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(22, 0);
    let left = c.fcc();
    c.g[S0][0] = 0x148;
    if !left {
        return Ok(ImpactTail::Choose);
    }
    c.cycles += 1;
    c.g[S0][0] = 0x149;
    Ok(ImpactTail::Picked)
}

/// A hard hit on the forward axis. Facing it (d1 >= 0): a random glance
/// (odd/even). Head-on: close (height < 60) and fast (|velocity| > the fast
/// speed) -> `close` (returns false: straight to the compare); else a
/// random head-on wipeout out of 6 (fast) or 3. True: through the block end.
fn impact_side_hit(c: &mut Cpu, ids: &ImpactSideHit) -> Result<bool, crate::call::Unwound> {
    use impact::*;
    c.cycles += 4;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    if !c.fcc() {
        c.cycles += 6;
        c.g[S0][0] = ids.glance;
        call_ptr(c, IMPACT_RAND, ids.ret_glance)?;
        c.g[V0][0] &= 1;
        c.g[V1][0] = ids.glance_even;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = c.g[V1][0];
        }
        return Ok(true);
    }
    // f2 = |velocity| (VU0 sqrt), f0 = 60.
    c.cycles += 16;
    c.g[AT][0] = 0x4270_0000;
    c.mtc1(AT, 0);
    c.lqc2(3, impact_at(c, S1, VELOCITY));
    impact_dot(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    c.mtc1(V0, 2);
    c.c_lt_s(23, 0);
    let close = c.fcc();
    c.lwc1(1, impact_gp(c, FAST_SPEED));
    if close {
        c.cycles += 4;
        c.c_lt_s(1, 2);
        c.g[S0][0] = ids.close;
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = NO_ANIM;
            return Ok(false);
        }
    } else {
        c.cycles += 1;
        c.c_lt_s(1, 2);
    }
    c.cycles += 2;
    if c.fcc() {
        c.cycles += 4;
        call_ptr(c, IMPACT_RAND, ids.ret_roll6)?;
        c.g[V1][0] = 6;
    } else {
        c.cycles += 3;
        call_ptr(c, IMPACT_RAND, ids.ret_roll3)?;
        c.g[V1][0] = 3;
    }
    c.cycles += 3;
    c.divu(0, V0, V1);
    c.cycles += 3;
    c.g[A0][0] = c.hi[0];
    c.g[V0][0] = 1;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[S0][0] = ids.roll[0];
        return Ok(true);
    }
    c.cycles += 2;
    let not_one = c.g[A0][0] != c.g[V0][0];
    c.g[V0][0] = 2;
    if !not_one {
        c.cycles += 2;
        c.g[S0][0] = ids.roll[1];
        return Ok(true);
    }
    c.cycles += 2;
    let two = c.g[A0][0] == c.g[V0][0];
    c.g[S0][0] = ids.roll[2];
    if two {
        return Ok(true);
    }
    c.cycles += 5;
    c.g[V1][0] = c.g[A0][0] ^ 3;
    c.g[S0][0] = ids.roll[4];
    c.g[V0][0] = ids.roll[3];
    if c.g[V1][0] == 0 {
        c.g[S0][0] = c.g[V0][0];
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rider_HandleImpact fixture (all structures at DATA + 0x200000, away
    /// from the fakes' random vtable). Sub state 9 (frozen), 0xb (already
    /// wiping out: bump) or other; main state 1 (riding), 2 (in the air,
    /// handler air state 1 or not) or 3 (other); surface -1 or a non-fatal
    /// one; random board axes, normals (steep ones too), impact speeds
    /// around every limit, contact heights around 60, velocities around the
    /// fast speed, layer-2 sequences 0x12..0x15 or others, and the game's
    /// Rand state. The rider's methods (vibrationBump, getPosition,
    /// getVelocity) are fakes; AirPredictor_Prime, the layer query and
    /// Rand_NextGame are real. Rider_PlayBumpAnim returns at once (main state
    /// not 0/4). Rider_StartWipeout is bounded by making its first call (the
    /// vibrationBump fake again) unwind: every seed unwinds at or before that
    /// fake call index.
    ///
    /// Not reached: a fatal surface (Rider_CheckResetPath reaches audio and
    /// the disc-loading request loop).
    #[test]
    fn rider_handle_impact_matches() {
        use crate::difftest::fakes::{self, covered, machine};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        covered(ee::s00::f_00105d98__generated, rider_handle_impact, 400, &|c, seed| {
            let base = DATA + 0x20_0000;
            let rider = base;
            let info = base + 0x1000;
            let handler = base + 0x2000;
            let anim = base + 0x3000;
            let pred = base + 0x4000;
            let world = base + 0x5000;
            let game = base + 0x5100;
            let surfaces = base + 0x6000;
            let pos_v = base + 0x8000;
            let vel_v = base + 0x8100;
            let mut r = machine(c, seed, 0x105d98, 4);
            c.pc = 0x0010_5d98;
            let f = |x: f32| x.to_bits();

            // gp constants (.lit4) and the fast-speed global.
            for (off, v) in [
                (-0x7f08i32, 0.8f32),
                (-0x7f04, 416.532),
                (-0x7f00, 833.337_6),
                (-0x7efc, 0.2),
                (-0x7ef8, 1.4),
                (-0x7ef4, 1388.880_9),
                (-0x7ef0, 2083.227_3),
                (0x1b94, 500.0),
            ] {
                c.bus.write32(GAME_GP.wrapping_add(off as u32), f(v));
            }
            // Rand_NextGame state.
            for i in 0..6u32 {
                c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
            }

            // States.
            let sub = match seed % 8 {
                0 => 9,
                1 => 0xb,
                _ => r.next_u32() % 9,
            };
            let main = match seed / 8 % 5 {
                0 => 2,
                1 | 2 => 1,
                _ => 3,
            };
            c.bus.write32(rider + 0x77c, handler);
            c.bus.write32(handler + 0xde4, sub);
            c.bus.write32(handler + 0xde0, main);
            c.bus.write32(handler + 0x30, r.next_u32() % 2);

            // Methods: vibrationBump 0x88, getPosition 0x28, getVelocity 0x10.
            let vt = fakes::vtable(c, &mut r, &[0x88, 0x28, 0x10]);
            c.bus.write32(rider + 0x6c0, vt);
            c.bus.write32(fakes::LOG_RESULTS + 4, pos_v);
            c.bus.write32(fakes::LOG_RESULTS + 8, vel_v);
            for i in 0..8 {
                c.bus.write32(pos_v + 4 * i, r.f32(500.0));
                c.bus.write32(vel_v + 4 * i, r.f32(500.0));
            }
            // Unwind at or before Rider_StartWipeout's first call.
            let last = if main == 1 { 3 } else { 1 };
            let unwind = if r.next_u32() % 4 == 0 {
                r.next_u32() % (last + 1)
            } else {
                last
            };
            c.bus.write32(fakes::LOG_UNWIND_AT, unwind);

            // Air predictor and its launch argument.
            c.bus.write32(rider + 0x788, pred);
            for i in 0..64 {
                c.bus.write32(pred + 4 * i, r.next_u32());
            }
            c.bus.write32(rider + 0x2e4, r.edge_f32());
            for i in 0..4 {
                c.bus.write32(0x004f_f160 + i * 4, r.next_u32());
                c.bus.write32(0x004f_f120 + i * 4, r.next_u32());
            }
            for i in 0..2 {
                c.bus.write32(0x004a_5960 + i * 4, r.next_u32());
            }

            // Surface: -1 or a non-fatal entry of the surface table.
            let surface: u64 = if seed % 3 == 0 {
                u64::MAX
            } else {
                (r.next_u32() % 0x12) as u64
            };
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, game);
            c.bus.write32(game + 0x44, surfaces);
            for k in 0..0x12 {
                c.bus.write32(surfaces + k * 0xb0 + 0x44, 0);
            }

            // Animation controller: layer 2 sequence (via AnimTable_GetEntry).
            c.bus.write32(rider + 0x784, anim);
            let idle = r.next_u32() % 6 == 0;
            c.bus.write32(anim + 8, if idle { 0x1b6 } else { 3 });
            let cur = [0x12u32, 0x13, 0x14, 0x15, 0x11, 0x16, 0x40][(r.next_u32() % 7) as usize];
            c.bus.write32(0x0044_6990 + 3 * 0x1c, cur);

            // Board axes, origin, matrix row 2 and velocity.
            for off in [0x160u32, 0x170, 0x180] {
                put_vec(c, rider + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            }
            put_vec(
                c,
                rider + 0x190,
                [r.f32(100.0), r.f32(100.0), r.f32(100.0), f(1.0)],
            );
            put_vec(c, rider + 0x1c0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            let speed = [0.0f32, 100.0, 400.0, 600.0, 3000.0][(r.next_u32() % 5) as usize];
            put_vec(
                c,
                rider + 0x1e0,
                [r.f32(speed), r.f32(speed), r.f32(speed), 0],
            );
            put_vec(c, rider + 0x3e0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            c.bus.write32(rider + 0x3f0, r.f32(3.0));

            // Impact info: contact point near the origin (heights around
            // 60), normal (steep on some seeds), impact speed.
            put_vec(c, info, [r.f32(150.0), r.f32(150.0), r.f32(150.0), f(1.0)]);
            let nz = if seed % 4 == 1 {
                f(0.81 + (r.next_u32() % 19) as f32 * 0.01)
            } else {
                r.f32(1.0)
            };
            put_vec(c, info + 0x20, [r.f32(1.0), r.f32(1.0), nz, 0]);
            let vn = [100.0f32, 420.0, 900.0, 1500.0, 2500.0, 4000.0][(r.next_u32() % 6) as usize];
            c.bus.write32(info + 0x30, f(vn));

            c.g[A0][0] = sx(rider);
            c.g[A1][0] = sx(info);
            c.g[A2][0] = surface;
        });
    }
}
