//! Staging module oc367 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc367`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::ai::rider::{qcopy, v_add, v_dot, v_length, v_normalize, v_scale, v_sub};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0010_7e70,
    rider_resolve_rider_collision,
    "Rider_ResolveRiderCollision",
)];

/// Field offsets of the rider (`self`) and the other rider that
/// `Rider_ResolveRiderCollision` reads.
pub(crate) mod rrc {
    /// Position (vector).
    pub const POS: u32 = 0x110;
    /// Board axes (three vectors) the wipeout direction is picked from.
    pub const AXIS_Z: u32 = 0x160;
    pub const AXIS_Y: u32 = 0x170;
    pub const AXIS_X: u32 = 0x180;
    /// Facing vector compared with the collision normal in the air.
    pub const FACING: u32 = 0x1c0;
    /// Velocity (vector): the impulse is added here.
    pub const VEL: u32 = 0x1e0;
    /// Launch argument of `AirPredictor_Prime` (float).
    pub const LAUNCH: u32 = 0x2e4;
    /// Ground normal (vector): on the ground the impulse loses this part.
    pub const GROUND_NORMAL: u32 = 0x370;
    /// The rider's actor subobject (its vtable pointer).
    pub const ACTOR: u32 = 0x6c0;
    /// The state handler (`+0x30`: air state).
    pub const HANDLER: u32 = 0x77c;
    /// The air predictor.
    pub const PREDICTOR: u32 = 0x788;
    /// The other rider's index into this rider's collision records
    /// (0x24 bytes each, the distance at `+8`).
    pub const OTHER_INDEX: u32 = 0x86c;
    pub const RECORD_SIZE: u32 = 0x24;
    pub const RECORD_DIST: u32 = 0x8;
    /// Debug flags word (bit 5: every rider hit wipes out).
    pub const DEBUG_FLAGS: u32 = 0x0053_08d0;
    /// `.lit4`: the impulse clamp (555.56), the bump threshold (40.0), the
    /// forced wipeout speed and the wipeout threshold (both 599.97), and
    /// the speed past which a front hit picks from all six falls.
    pub const LIT_CLAMP: u32 = 0x7e6c;
    pub const LIT_BUMP_MIN: u32 = 0x7e68;
    pub const LIT_FORCED: u32 = 0x7e64;
    pub const LIT_WIPEOUT_MIN: u32 = 0x7e60;
    pub const GP_FAST_FALL: u32 = 0x1b94;
}

const RRC_GET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fee8,
    f: ee::s00::Rider_GetSubState,
};
const RRC_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
const RRC_IS_ON_GROUND: Callee = Callee {
    addr: 0x0012_31a8,
    f: ee::s00::Rider_IsOnGround,
};
const RRC_AIR_PREDICTOR_PRIME: Callee = Callee {
    addr: 0x0011_35b8,
    f: ee::s00::AirPredictor_Prime,
};
const RRC_START_WIPEOUT: Callee = Callee {
    addr: 0x0010_eb30,
    f: ee::s00::Rider_StartWipeout,
};
const RRC_PLAY_BUMP_ANIM: Callee = Callee {
    addr: 0x0010_8388,
    f: ee::s00::Rider_PlayBumpAnim,
};
const RRC_REPORT_HIT0: Callee = Callee {
    addr: 0x0010_e228,
    f: ee::s00::Rider_ReportRiderHit0,
};
const RRC_REPORT_HIT1: Callee = Callee {
    addr: 0x0010_e2e8,
    f: ee::s00::Rider_ReportRiderHit1,
};
const RRC_REPORT_HIT2: Callee = Callee {
    addr: 0x0010_e3a8,
    f: ee::s00::Rider_ReportRiderHit2,
};
const RRC_REPORT_HIT3: Callee = Callee {
    addr: 0x0010_e468,
    f: ee::s00::Rider_ReportRiderHit3,
};
/// `Rand_NextGame` (AIrand), through the function table.
const RRC_RAND: u32 = 0x0031_7810;

/// `lwc1 fN, gp - off`.
fn rrc_lit(c: &mut Cpu, f: usize, off: u32) {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lwc1(f, a);
}

/// `void Rider_ResolveRiderCollision(Rider* self, Rider* other, vec4* n,
/// bool flag, float speed)` (0x107e70): pushes the rider away from another
/// rider along the normal `n` with the clamped impulse `speed`
/// (|speed| <= 555.56; on the ground without the part along the ground
/// normal; in the air it primes the air predictor and, when moving into the
/// hit, ignores the speed). Unless the rider is in the air state 2 or sub
/// state 9 (10 skips everything), a hit of at least 40 plays a bump
/// animation and a hit past 599.97 (or always, with debug flag bit 5) a
/// wipeout picked from the board axis closest to `n` (falls forward from a
/// random choice, back, sideways); then reports the hit to the other rider
/// (`Rider_ReportRiderHit0..3`, by kind and `flag`).
///
/// Registers: s2 = self, s4 = other, s3 = n, s5 = flag, f20 = speed,
/// f21 = |speed| (the hit strength), f22 = clamped impulse.
pub fn rider_resolve_rider_collision(c: &mut Cpu) {
    use rrc::*;
    guest(c, |c| {
        // Prologue.
        c.cycles += 20;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x90);
        c.g[S4][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0xf0);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0xe0);
        c.mov_s(20, 12);
        frame.save(c, S1, 0xd0);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 21, 0xf8);
        RRC_GET_SUB_STATE.call(c, 0x0010_7eb4)?;
        c.g[V1][0] = 0xa;
        frame.restore(c, S0, 0xe0);
        if c.g[V0][0] != 0xa {
            rrc_body(c, frame)?;
        }
        // Epilogue (s0 already restored on every path).
        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// Everything after the sub-state-10 check; returns with s0 restored.
fn rrc_body(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use rrc::*;
    let n = c.g[S3][0] as u32;

    // f21 = |speed|, f22 = speed clamped to +-555.56.
    c.cycles += 7;
    rrc_lit(c, 1, LIT_CLAMP);
    c.abs_s(21, 20);
    c.neg_s(0, 1);
    c.c_le_s(0, 20);
    let above_min = c.fcc();
    c.mov_s(22, 0);
    if above_min {
        c.cycles += 1;
        c.min_s(22, 20, 1);
    }

    // impulse = n * f22 (at sp+0x10, copied to sp).
    c.cycles += 11;
    let s10 = frame.at(c, 0x10);
    v_scale(c, V0, 22, n, s10);
    c.g[A0][0] = c.g[S2][0];
    let s00 = frame.at(c, 0);
    qcopy(c, V0, s10, s00);
    RRC_IS_ON_GROUND.call(c, 0x0010_7f04)?;
    let on_ground = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S2][0];
    let me = c.g[S2][0] as u32;
    if on_ground {
        // vel += impulse - ground_normal * (impulse . ground_normal).
        c.cycles += 28;
        let s00 = frame.at(c, 0);
        v_dot(c, s00, me + GROUND_NORMAL);
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        let s30 = frame.at(c, 0x30);
        v_scale(c, V1, 0, me + GROUND_NORMAL, s30);
        let s20 = frame.at(c, 0x20);
        qcopy(c, V0, s30, s20);
        v_sub(c, s00, s20, s30);
        let s10 = frame.at(c, 0x10);
        qcopy(c, V0, s30, s10);
        v_add(c, me + VEL, s10, me + VEL);
    } else {
        // vel += impulse; in the air (main state 1, or 2 with air state 1)
        // prime the air predictor.
        c.cycles += 8;
        let s00 = frame.at(c, 0);
        v_add(c, me + VEL, s00, me + VEL);
        c.g[S0][0] = 1;
        RRC_GET_MAIN_STATE.call(c, 0x0010_7f94)?;
        let mut air = false;
        if c.g[V0][0] == c.g[S0][0] {
            c.g[V1][0] = Addr(me + ACTOR).lw(c);
            air = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            RRC_GET_MAIN_STATE.call(c, 0x0010_7fa4)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] == 2 {
                c.cycles += 4;
                c.g[V0][0] = Addr(me + HANDLER).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x30)).lw(c);
                if c.g[V1][0] == c.g[S0][0] {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(me + ACTOR).lw(c);
                    air = true;
                }
            }
        }
        if air {
            rrc_prime_air(c, frame)?;
        }
    }

    // Air state 2 and sub state 9 stop here; so do hits under 40.
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    RRC_GET_MAIN_STATE.call(c, 0x0010_8050)?;
    c.g[V1][0] = 2;
    if c.g[V0][0] == 2 {
        frame.restore(c, S0, 0xe0);
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    RRC_GET_SUB_STATE.call(c, 0x0010_8064)?;
    c.g[V1][0] = 9;
    if c.g[V0][0] == 9 {
        frame.restore(c, S0, 0xe0);
        return Ok(());
    }
    c.cycles += 5;
    rrc_lit(c, 0, LIT_BUMP_MIN);
    c.c_lt_s(21, 0);
    if c.fcc() {
        frame.restore(c, S0, 0xe0);
        return Ok(());
    }

    rrc_hit(c, frame)
}

/// The air branch: `AirPredictor_Prime(predictor, actor->getPosition(),
/// actor->getDirection(), self->launch)`, then the hit strength drops to 0
/// when the rider moves into the hit (`(n . facing) * impulse > 0`).
/// v1 holds the actor's vtable.
fn rrc_prime_air(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use rrc::*;
    c.cycles += 33;
    let me = c.g[S2][0] as u32;
    c.g[S0][0] = sx(me.wrapping_add(ACTOR));
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x28)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x2c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0010_7fd8)?;
    let me = c.g[S2][0] as u32;
    c.g[V1][0] = Addr(me + ACTOR).lw(c);
    c.g[S1][0] = c.g[V0][0];
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x10)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x14)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0010_7ff0)?;
    let me = c.g[S2][0] as u32;
    c.g[A0][0] = Addr(me + PREDICTOR).lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.lwc1(12, me + LAUNCH);
    c.g[A1][0] = c.g[S1][0];
    RRC_AIR_PREDICTOR_PRIME.call(c, 0x0010_8004)?;
    let me = c.g[S2][0] as u32;
    let n = c.g[S3][0] as u32;
    let s10 = frame.at(c, 0x10);
    c.lq(V0, me + FACING);
    c.mtc1(0, 1);
    c.sq(V0, s10);
    v_dot(c, n, s10);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.mul_s(0, 0, 22);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.mov_s(21, 1);
    }
    Ok(())
}

/// A hit of at least 40: the `RiderHitInfo` at sp+0x40 (contact point
/// `pos + n * record.dist`, velocity direction, `n`, impulse), then a bump
/// or a wipeout and the report to the other rider.
fn rrc_hit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use rrc::*;
    c.cycles += 40;
    let me = c.g[S2][0] as u32;
    let n = c.g[S3][0] as u32;
    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(OTHER_INDEX)).lw(c);
    c.g[V1][0] = RECORD_SIZE as u64;
    c.g[A1][0] = 0x0053_0000;
    c.mult(0, A0, V0, V1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(me));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(RECORD_DIST));
    let s30 = frame.at(c, 0x30);
    let s20 = frame.at(c, 0x20);
    let s10 = frame.at(c, 0x10);
    v_scale(c, V1, 0, n, s30);
    qcopy(c, V0, s30, s20);
    v_add(c, me + POS, s20, s30);
    c.lq(V0, s30);
    c.sq(V0, s10);
    let info = frame.at(c, 0x40);
    c.sq(V0, info);
    v_normalize(c, me + VEL, s20);
    c.g[V0][0] = Addr(DEBUG_FLAGS).lw(c);
    c.lq(V1, n);
    c.lq(A0, s20);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
    c.g[V0][0] &= 1;
    let s60 = frame.at(c, 0x60);
    c.sq(V1, s60);
    let s50 = frame.at(c, 0x50);
    c.sq(A0, s50);
    let s70 = frame.at(c, 0x70);
    c.swc1(22, s70);
    let forced = c.g[V0][0] != 0;
    c.sq(A0, s10);
    if forced {
        c.cycles += 1;
        rrc_lit(c, 21, LIT_FORCED);
    }

    c.cycles += 5;
    rrc_lit(c, 0, LIT_WIPEOUT_MIN);
    c.c_lt_s(0, 21);
    let wipeout = c.fcc();
    c.g[A0][0] = c.g[S2][0];
    if !wipeout {
        // Rider_PlayBumpAnim(self, &info, 0), then report hit 2 / 0.
        c.cycles += 5;
        c.g[A1][0] = sx(frame.at(c, 0x40));
        c.g[A2][0] = 0;
        RRC_PLAY_BUMP_ANIM.call(c, 0x0010_8338)?;
        let flag = c.g[S5][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if flag {
            c.cycles += 4;
            c.g[A1][0] = c.g[S4][0];
            RRC_REPORT_HIT2.call(c, 0x0010_8348)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[S4][0];
            RRC_REPORT_HIT0.call(c, 0x0010_8358)?;
        }
        frame.restore(c, S0, 0xe0);
        return Ok(());
    }

    rrc_pick_wipeout(c, frame)?;

    // Rider_StartWipeout(self, anim, flag, 0, &info), then report hit 3 / 1.
    c.cycles += 7;
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = 0;
    c.g[T0][0] = sx(frame.at(c, 0x40));
    RRC_START_WIPEOUT.call(c, 0x0010_8304)?;
    let flag = c.g[S5][0] != 0;
    c.g[A0][0] = c.g[S2][0];
    c.cycles += 4;
    c.g[A1][0] = c.g[S4][0];
    if flag {
        RRC_REPORT_HIT3.call(c, 0x0010_8314)?;
    } else {
        RRC_REPORT_HIT1.call(c, 0x0010_8324)?;
    }
    frame.restore(c, S0, 0xe0);
    Ok(())
}

/// The wipeout animation (into a1) from the board axis most aligned with
/// `n`: the up axis (dy) falls forward (a random one of three, or six when
/// moving fast) or back (0x14a/0x14b at random), the side axis 0x14b, the
/// forward axis 0x149 / 0x148 by its sign.
fn rrc_pick_wipeout(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use rrc::*;
    c.cycles += 38;
    let me = c.g[S2][0] as u32;
    let n = c.g[S3][0] as u32;
    let s10 = frame.at(c, 0x10);
    c.lq(V0, me + AXIS_Z);
    c.lq(V1, me + AXIS_Y);
    c.sq(V0, s10);
    v_dot(c, s10, n);
    c.qmfc2(A0, 4);
    c.mtc1(A0, 4); // f4 = dz
    c.lq(V0, me + AXIS_X);
    c.sq(V1, s10);
    c.abs_s(0, 4);
    v_dot(c, s10, n);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 1); // f1 = dy
    c.abs_s(3, 1);
    c.sq(V0, s10);
    c.c_lt_s(0, 3);
    v_dot(c, s10, n);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 2); // f2 = dx
    let mut dy_major = c.fcc();
    if dy_major {
        c.cycles += 4;
        c.abs_s(0, 2);
        c.c_lt_s(0, 3);
        dy_major = c.fcc();
    }
    if dy_major {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let fall_forward = c.fcc();
        let fast = (c.g[GP][0] as u32).wrapping_add(GP_FAST_FALL);
        c.lwc1(0, fast);
        if fall_forward {
            c.cycles += 13;
            let me = c.g[S2][0] as u32;
            let _ = v_length(c, me + VEL, V0, 1);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 4;
                call_ptr(c, RRC_RAND, 0x0010_8234)?;
                c.g[V1][0] = 6;
            } else {
                c.cycles += 3;
                call_ptr(c, RRC_RAND, 0x0010_8244)?;
                c.g[V1][0] = 3;
            }
            c.cycles += 3;
            c.divu(0, V0, V1);
            c.cycles += 3;
            c.g[A0][0] = c.hi[0];
            let k = c.g[A0][0];
            c.g[V0][0] = 1;
            if k == 0 {
                c.cycles += 2;
                c.g[A1][0] = 0x14e;
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 2;
            if k == 1 {
                c.cycles += 2;
                c.g[A1][0] = 0x14f;
                return Ok(());
            }
            c.cycles += 2;
            c.g[A1][0] = 0x151;
            if k == 2 {
                return Ok(());
            }
            c.cycles += 5;
            c.g[V1][0] = k ^ 3;
            c.g[A1][0] = 0x150;
            c.g[V0][0] = 0x14d;
            if c.g[V1][0] == 0 {
                c.g[A1][0] = 0x14d;
            }
            return Ok(());
        }
        c.cycles += 7;
        call_ptr(c, RRC_RAND, 0x0010_829c)?;
        c.g[V0][0] &= 1;
        c.g[A1][0] = 0x14b;
        c.g[V1][0] = 0x14a;
        if c.g[V0][0] == 0 {
            c.g[A1][0] = 0x14a;
        }
        return Ok(());
    }

    c.cycles += 5;
    c.abs_s(2, 2);
    c.abs_s(0, 4);
    c.c_lt_s(0, 2);
    if c.fcc() {
        c.cycles += 5;
        c.abs_s(0, 1);
        c.c_lt_s(0, 2);
        c.g[A1][0] = 0x14b;
        if c.fcc() {
            return Ok(());
        }
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(4, 0);
    c.g[A1][0] = 0x148;
    if c.fcc() {
        c.cycles += 1;
        c.g[A1][0] = 0x149;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{put_vec, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests' machines fall back).
    pub(crate) const RRC_CTRL: u32 = DATA + 0x20_5000;
    pub(crate) const RRC_MAGIC: u32 = 0x7e70_c011;

    macro_rules! rrc_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(RRC_CTRL) != RRC_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    rrc_gated!(
        rrc_fake_start_wipeout,
        RRC_PREV_START_WIPEOUT,
        0x0010_eb30,
        ee::s00::f_0010eb30__generated
    );
    rrc_gated!(
        rrc_fake_air_predictor_prime,
        RRC_PREV_AIR_PREDICTOR_PRIME,
        0x0011_35b8,
        ee::s00::f_001135b8__generated
    );
    rrc_gated!(
        rrc_fake_hit0,
        RRC_PREV_HIT0,
        0x0010_e228,
        ee::s00::f_0010e228__generated
    );
    rrc_gated!(
        rrc_fake_hit1,
        RRC_PREV_HIT1,
        0x0010_e2e8,
        ee::s00::f_0010e2e8__generated
    );
    rrc_gated!(
        rrc_fake_hit2,
        RRC_PREV_HIT2,
        0x0010_e3a8,
        ee::s00::f_0010e3a8__generated
    );
    rrc_gated!(
        rrc_fake_hit3,
        RRC_PREV_HIT3,
        0x0010_e468,
        ee::s00::f_0010e468__generated
    );

    /// Rider_StartWipeout, AirPredictor_Prime and the four hit reports
    /// become logging fakes (they reach animation, sound and the game).
    pub(crate) fn rrc_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010EB30),
            rrc_fake_start_wipeout,
            &RRC_PREV_START_WIPEOUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_001135B8),
            rrc_fake_air_predictor_prime,
            &RRC_PREV_AIR_PREDICTOR_PRIME,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E228),
            rrc_fake_hit0,
            &RRC_PREV_HIT0,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E2E8),
            rrc_fake_hit1,
            &RRC_PREV_HIT1,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E3A8),
            rrc_fake_hit2,
            &RRC_PREV_HIT2,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010E468),
            rrc_fake_hit3,
            &RRC_PREV_HIT3,
        );
    }

    /// `Rider_ResolveRiderCollision` against the generated body. Sub state
    /// 10 (skip), 9 (impulse only) or one that keeps Rider_PlayBumpAnim on
    /// its early return (3..8, 0xb); main state 0 (ground), 1 (air), 2
    /// (air state 0/1: ground or predictor) or 3; impulses around the
    /// clamp, the bump and the wipeout thresholds (and edge values);
    /// random board axes, normals and velocities (all wipeout directions),
    /// the debug flag bit 5, the fast-fall threshold, `flag` 0/1/random.
    /// The actor's methods are table fakes, Rand_NextGame is real (the
    /// game's Rand state), the other callees are gated fakes, some
    /// unwinding.
    #[test]
    fn rider_resolve_rider_collision_matches() {
        rrc_install_fakes();
        covered(
            ee::s00::f_00107e70__generated,
            rider_resolve_rider_collision,
            600,
            &|c, seed| {
                let base = DATA + 0x20_0000;
                let me = base;
                let other = base + 0x1000;
                let n = base + 0x2000;
                let handler = base + 0x3000;
                let mut r = machine(c, seed, 0x107e70, 4);
                c.pc = 0x0010_7e70;
                c.bus.write32(RRC_CTRL, RRC_MAGIC);
                let f = |x: f32| x.to_bits();

                // .lit4 constants and the fast-fall threshold.
                for (off, v) in [
                    (rrc::LIT_CLAMP, 0x440a_e38e),
                    (rrc::LIT_BUMP_MIN, 0x421f_fb37),
                    (rrc::LIT_FORCED, 0x4415_fe56),
                    (rrc::LIT_WIPEOUT_MIN, 0x4415_fe56),
                ] {
                    c.bus.write32(GAME_GP - off, v);
                }
                let fast = match seed % 3 {
                    0 => 0x0042_fdd0,
                    1 => f(300.0),
                    _ => r.f32(600.0),
                };
                c.bus.write32(GAME_GP + rrc::GP_FAST_FALL, fast);
                // Rand_NextGame state.
                for i in 0..8u32 {
                    c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
                }
                c.bus.write32(rrc::DEBUG_FLAGS, r.next_u32());

                // The rider: random floats, then the fields.
                for i in 0..0x240 {
                    c.bus.write32(me + 4 * i, r.f32(2.0));
                }
                for off in [rrc::POS, rrc::FACING, rrc::VEL] {
                    put_vec(
                        c,
                        me + off,
                        [r.f32(300.0), r.f32(300.0), r.f32(300.0), r.f32(1.0)],
                    );
                }
                if seed % 11 == 0 {
                    put_vec(c, me + rrc::VEL, [0, 0, 0, 0]);
                }
                c.bus.write32(me + rrc::LAUNCH, r.edge_f32());
                c.bus.write32(me + rrc::HANDLER, handler);
                c.bus.write32(me + rrc::PREDICTOR, r.next_u32());
                let vt = fakes::vtable(c, &mut r, &[0x28, 0x10]);
                c.bus.write32(me + rrc::ACTOR, vt);
                // Record index (records at +8 inside the rider, before POS).
                c.bus.write32(other + rrc::OTHER_INDEX, r.next_u32() % 7);
                for i in 0..7 {
                    c.bus
                        .write32(me + i * rrc::RECORD_SIZE + rrc::RECORD_DIST, r.f32(3.0));
                }

                // States.
                let sub = match seed % 9 {
                    0 => 10,
                    1 => 9,
                    k => [3, 4, 5, 6, 8, 0xb, 0x20][(k as usize + seed as usize / 9) % 7],
                };
                let main = match seed / 9 % 6 {
                    0 => 0,
                    1 | 2 => 1,
                    3 | 4 => 2,
                    _ => 3,
                };
                c.bus.write32(handler + 0xde4, sub);
                c.bus.write32(handler + 0xde0, main);
                c.bus.write32(handler + 0x30, r.next_u32() % 2);

                // Normal and impulse.
                put_vec(c, n, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(0.1)]);
                c.f[12] = match seed % 7 {
                    0 => r.f32(40.0),
                    1 => r.f32(100.0),
                    2 | 3 => r.f32(1200.0),
                    4 => r.f32(600.0),
                    5 => r.edge_f32(),
                    _ => r.f32(5000.0),
                };
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(other);
                c.g[A2][0] = sx(n);
                c.g[A3][0] = match seed % 5 {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    _ => (r.next_u32() as u64) << 32 | 2,
                };
            },
        );
    }
}
