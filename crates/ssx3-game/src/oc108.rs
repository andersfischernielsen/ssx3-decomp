//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacement for one function, with a differential test against the
//! generated body.
//!
//! `RiderAirState_PostUpdate` (0x00139c88) runs after the air state has
//! updated a rider's motion:
//!
//! - cast a ray from the board (`RiderAirState_CastBoardRay`) and, when it
//!   hits within half a unit and the rider moves towards the board, snap the
//!   rider onto the board: position, board normal, up/side axes (`+0x3a0`,
//!   `+0x3b0`), the trigger/contact object (`+0x430`/`+0x438`), the surface
//!   normal (`+0x454`), the transform (`+0x460`), the landing animation
//!   (`+0x3d0`, `+0x370`) and the blend distances (`+0xaac`, `+0xab0`);
//! - otherwise pick the airborne animation id (`s1`: 0x15c..0x165, 0x169,
//!   the default 0x1b6) from the board angle and the animation state, driving
//!   `Rider_GetSubState`/`SetSubState`/`AnimController_PlayAnim`;
//! - finally update the rider's bounds, collide with the walls and notify the
//!   camera (`0x107888`).
//!
//! The original is one gcc case chain whose basic blocks fall through, so the
//! port uses labelled blocks for the jumps, keeps every cycle charge at the
//! same point, and keeps every register write the original makes before a
//! call (a fake callee logs all of them, and the log is compared).

use crate::abi::r32;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const CAST_BOARD_RAY: Callee = Callee {
        addr: 0x0013_a7b0,
        f: ee::s01::RiderAirState_CastBoardRay,
    };
    pub const PLAY_LAND_ANIM: Callee = Callee {
        addr: 0x0013_a8f8,
        f: ee::s01::RiderAirState_PlayLandAnim,
    };
    pub const PLAY_LAND_ANIM2: Callee = Callee {
        addr: 0x0013_a968,
        f: ee::s01::RiderAirState_PlayLandAnim2,
    };
    pub const COLLIDE_WALLS: Callee = Callee {
        addr: 0x0013_aa48,
        f: ee::s01::RiderAirState_CollideWalls,
    };
    pub const FLAGS64_TEST: Callee = Callee {
        addr: 0x0014_46a0,
        f: ee::s01::Flags64_Test,
    };
    pub const RIDER_UPDATE_BOUNDS: Callee = Callee {
        addr: 0x0011_e150,
        f: ee::s00::Rider_UpdateBounds,
    };
    pub const BE_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BE_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const ATTRIB6_FACTOR: Callee = Callee {
        addr: 0x0014_9120,
        f: ee::s01::BECharInt_GetAttrib6Factor,
    };
}

/// Calls the generator emitted as `call_indirect` (cross-shard `jal`).
mod target {
    /// `Rider_GetSubState(rider)`.
    pub const GET_SUB_STATE: u32 = 0x0011_fee8;
    /// `Rider_SetSubState(rider, state)`.
    pub const SET_SUB_STATE: u32 = 0x0011_fec8;
    /// `Rider_SetMainState(rider, state)`.
    pub const SET_MAIN_STATE: u32 = 0x0011_fe78;
    /// `Rider_CheckResetPath(rider, out, flag)`.
    pub const CHECK_RESET_PATH: u32 = 0x0011_6120;
    /// `Rider_TurnAroundIfBackward(rider)`.
    pub const TURN_AROUND: u32 = 0x0011_4cc0;
    /// `Rider_Translate(rider, ...)`.
    pub const TRANSLATE: u32 = 0x0010_6538;
    /// `Rider_BankTrickBoost(rider, ...)`.
    pub const BANK_TRICK_BOOST: u32 = 0x0010_e910;
    /// `Rider_StartWipeout(rider, a1, a2, a3)`.
    pub const START_WIPEOUT: u32 = 0x0010_eb30;
    /// `AirPredictor_...` (src/ai/airpredictor.cpp).
    pub const AIR_PREDICTOR: u32 = 0x0011_3998;
    /// `RiderAnimBase` layer queries.
    pub const ANIM_LAYER: u32 = 0x0031_1ae8;
    pub const ANIM_LAYER_SEQUENCE: u32 = 0x0031_1b20;
    /// `AnimController_PlayAnim(controller, anim, blend, flags)`.
    pub const PLAY_ANIM: u32 = 0x0031_28e8;
    /// `Rand_NextGame()`.
    pub const RAND_NEXT_GAME: u32 = 0x0031_7810;
    /// `WScriptMan` query.
    pub const WSCRIPT: u32 = 0x0030_98b0;
    /// The air state's wall and camera tail calls.
    pub const AIR_WALLS: u32 = 0x0010_5398;
    pub const AIR_CAMERA: u32 = 0x0010_7888;
}

/// The rider pointer inside the air state.
const RIDER: u32 = 0x4;

/// Rider fields this function reads or writes.
mod rider {
    pub const POS: u32 = 0x110;
    pub const VEL: u32 = 0x1e0;
    pub const ANIM: u32 = 0x784;
    pub const TRANSFORM: u32 = 0x370;
    pub const UP: u32 = 0x3a0;
    pub const SIDE: u32 = 0x3b0;
    pub const LAND_ANIM: u32 = 0x3d0;
    pub const TRIGGER: u32 = 0x430;
    pub const CONTACT: u32 = 0x438;
    pub const NORMAL: u32 = 0x454;
    pub const XFORM: u32 = 0x460;
    pub const WALLS: u32 = 0xaa0;
    pub const BLEND_A: u32 = 0xaac;
    pub const BLEND_B: u32 = 0xab0;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// The low 32 bits of a GPR (what the generated `as u32` reads).
#[inline(always)]
fn g(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `*motion->rider` as the original loads it.
#[inline(always)]
fn rider(c: &mut Cpu, motion: u32) -> u64 {
    Addr(p(motion, RIDER)).lw(c)
}

/// `lq r, src; sq r, dst` (a quadword moved through a GPR).
#[inline(always)]
fn qcopy(c: &mut Cpu, r: usize, src: u32, dst: u32) {
    c.lq(r, src);
    c.sq(r, dst);
}

/// The `sw a, src` of the original.
#[inline(always)]
fn st(c: &mut Cpu, a: u32, src: usize) {
    let v = c.g[src][0];
    c.sw(a, v);
}

// ---------------------------------------------------------------------------
// VU0 sequences (macro mode), exactly as the original issues them.

/// The tail of the dot-product sequence: `vf6` is `(1, 1, 1, 1)` and `vf4`
/// holds the products, so `vf4.x` ends as the dot product.
#[inline(always)]
fn dot_tail(c: &mut Cpu) {
    let vu = c.vu0();
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf4.x = v · v` (`vf3 = v`).
#[inline(always)]
fn dot_self(c: &mut Cpu, v: u32) {
    c.lqc2(3, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    dot_tail(c);
}

/// `vf4.x = a · b` (`vf3 = a`, `vf5 = b`).
#[inline(always)]
fn dot_ab(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    dot_tail(c);
}

/// `f[dst] = |v|` through Q (`vsqrt`, `cfc2`, `mtc1`).
#[inline(always)]
fn v_length(c: &mut Cpu, v: u32, via: usize, dst: usize) {
    dot_self(c, v);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(via, 22);
    c.mtc1(via, dst);
}

/// `out = v / |v|` (`vrsqrt`).
#[inline(always)]
fn v_normalize(c: &mut Cpu, v: u32, out: u32) {
    dot_self(c, v);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `out = a - b` (`vf3`, `vf4` → `vf5`).
#[inline(always)]
fn v_sub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = a + b` (`vf3`, `vf4` → `vf5`).
#[inline(always)]
fn v_add(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

// ---------------------------------------------------------------------------
// RiderAirState_PostUpdate.

/// `void RiderAirState_PostUpdate(AirState* motion)` (0x139c88): board
/// contact, animation choice, bounds/wall update and camera notification.
pub fn rider_air_state_post_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S0, 0x120);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x138);
        frame.save(c, S1, 0x110);
        frame.save(c, S2, 0x100);
        frame.save_ra(c, 0xf0);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 20, 0x130);
        callee::CAST_BOARD_RAY.call(c, 0x0013_9cb4)?;
        let motion = r32(c, S0);
        // The frame is fixed from here on: keep its addresses as plain locals
        // so they can be used next to `&mut Cpu` arguments.
        let sp = frame.at(c, 0);
        // `f21 = 0` (`mtc1 $zero, f21`), `f1 = 0.5`: the ray must reach half a
        // unit for the board contact to count.
        c.mtc1(0, 21);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.c_le_s(1, 0);
        'func: {
            'body: {
                if !c.fcc() {
                    c.g[A0][0] = rider(c, motion);
                    break 'body;
                }
                c.cycles += 20;
                // `d = rider.pos - ray hit`; `f21 = d · hit.velocity`, negative
                // while the rider still moves onto the board.
                c.g[V0][0] = rider(c, motion);
                c.lqc2(3, p(g(c, V0), rider::VEL));
                c.lqc2(4, p(sp, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp, 0x90));
                qcopy(c, 3, p(sp, 0x90), p(sp, 0x80));
                dot_ab(c, p(sp, 0x80), p(sp, 0x10));
                c.qmfc2(2, 4);
                c.mtc1(2, 21);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.g[A0][0] = rider(c, motion);
                if t {
                    break 'body;
                }
                c.cycles += 4;
                c.mtc1(0, 0);
                c.c_lt_s(21, 0);
                if !c.fcc() {
                    break 'body;
                }

                // ---- 0x139d7c: touching the board ---------------------------
                c.cycles += 15;
                let vel = p(g(c, A0), rider::VEL);
                v_length(c, vel, 2, 22);
                call_ptr(c, target::GET_SUB_STATE, 0x0013_9d60)?;
                c.g[V1][0] = 0x5;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = rider(c, motion);
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = rider(c, motion);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, target::SET_SUB_STATE, 0x0013_9d78)?;
                    c.g[A1][0] = rider(c, motion);
                }

                // ---- 0x139de8: the trigger object under the board -----------
                'object: {
                    'join: {
                        c.cycles += 14;
                        // AirPredictor(*(rider + 0x788), rider + 0x110, rider +
                        // 0x1e0).
                        c.g[A0][0] = Addr(p(g(c, A1), 0x788)).lw(c);
                        c.g[A2][0] = sx(p(g(c, A1), rider::VEL));
                        c.g[A1][0] = sx(p(g(c, A1), rider::POS));
                        call_ptr(c, target::AIR_PREDICTOR, 0x0013_9d8c)?;
                        // Trigger entry `module->+0x44 + frame->+0x4c`, 0xb0
                        // bytes per entry.
                        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                        c.g[A1][0] = 0xb0;
                        c.g[V0][0] = Addr(p(sp, 0x4c)).lw(c);
                        c.g[A0][0] = Addr(p(g(c, V1), 0x84)).lw(c);
                        c.mult(0, 2, 2, 5);
                        c.g[V1][0] = Addr(p(g(c, A0), 0x44)).lw(c);
                        c.g[S1][0] = sx(p(g(c, V1), g(c, V0)));
                        c.g[A0][0] = Addr(p(g(c, S1), 0x44)).lw(c);
                        if c.g[A0][0] != 0 {
                            c.g[A0][0] = rider(c, motion);
                            break 'join;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(sp, 0x54)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = rider(c, motion);
                            break 'object;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p(g(c, V0), 0xa)).lh(c);
                        c.g[V0][0] = c.g[V0][0] & 0x2;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = rider(c, motion);
                            break 'object;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = rider(c, motion);
                        break 'join;
                    }
                    // The trigger was taken: reset the path and leave.
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 1;
                    call_ptr(c, target::CHECK_RESET_PATH, 0x0013_9de0)?;
                    break 'func;
                }

                // ---- 0x139e6c: blend the contact point ----------------------
                c.cycles += 22;
                c.lqc2(3, p(sp, 0));
                c.lqc2(4, p(g(c, V0), rider::POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp, 0x90));
                c.mtc1(0, 2);
                qcopy(c, 2, p(sp, 0x90), p(sp, 0x80));
                dot_ab(c, p(sp, 0x80), p(sp, 0x10));
                c.qmfc2(3, 4);
                c.mtc1(3, 1);
                c.lwc1(0, p(g(c, S1), 0x18));
                c.sub_s(1, 1, 0);
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    c.g[V1][0] = rider(c, motion);
                } else {
                    c.cycles += 11;
                    // `frame+0x10 *= f1` (the blend distance), then move.
                    c.mfc1(2, 1);
                    c.lqc2(4, p(sp, 0x10));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x90));
                    c.g[A1][0] = sx(p(sp, 0x80));
                    c.lq(2, p(sp, 0x90));
                    c.g[A0][0] = rider(c, motion);
                    c.sq(2, p(sp, 0x80));
                    call_ptr(c, target::TRANSLATE, 0x0013_9e68)?;
                    c.g[V1][0] = rider(c, motion);
                }

                // ---- 0x139edc: landing on the board -------------------------
                'bounce: {
                    'bounce_join: {
                        c.cycles += 14;
                        c.neg_s(0, 21);
                        c.lq(2, p(sp, 0x10));
                        c.sq(2, p(g(c, V1), rider::TRANSFORM));
                        c.g[A0][0] = rider(c, motion);
                        c.lq(2, p(sp, 0x20));
                        c.sq(2, p(g(c, A0), rider::LAND_ANIM));
                        c.g[V1][0] = rider(c, motion);
                        c.swc1(0, p(g(c, V1), 0x770));
                        c.g[A0][0] = rider(c, motion);
                        c.lq(2, p(g(c, A0), rider::TRANSFORM));
                        c.sq(2, p(g(c, A0), 0x380));
                        c.g[V0][0] = Addr(p(sp, 0x54)).lw(c);
                        let t = c.g[V0][0] == 0;
                        c.mov_s(1, 0);
                        if t {
                            break 'bounce_join;
                        }
                        c.cycles += 10;
                        c.g[V1][0] = Addr(p(g(c, V0), 0x150)).lw(c);
                        c.g[V0][0] = rider(c, motion);
                        let at = p(g(c, V0), rider::TRIGGER);
                        st(c, at, V1);
                        c.lwc1(0, p(sp, 0x6c));
                        c.g[A0][0] = rider(c, motion);
                        c.swc1(0, p(g(c, A0), rider::BLEND_A));
                        c.g[V0][0] = rider(c, motion);
                        c.lwc1(0, p(sp, 0x70));
                        c.swc1(0, p(g(c, V0), rider::BLEND_B));
                        break 'bounce;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = rider(c, motion);
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    let at = p(g(c, V1), rider::TRIGGER);
                    st(c, at, V0);
                }

                // ---- 0x139fa0: bounce the velocity off the board ------------
                c.cycles += 35;
                c.g[V1][0] = rider(c, motion);
                c.mfc1(4, 1);
                c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp, 0x90));
                c.lq(2, p(sp, 0x90));
                c.sq(2, p(sp, 0x80));
                c.lwc1(0, p(g(c, S1), 0x3c));
                c.mfc1(3, 0);
                c.lqc2(4, p(sp, 0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp, 0x80));
                c.g[V0][0] = rider(c, motion);
                let vel = p(g(c, V0), rider::VEL);
                v_add(c, vel, p(sp, 0x80), vel);
                c.g[V1][0] = rider(c, motion);
                let vel1 = p(g(c, V1), rider::VEL);
                let xf = p(g(c, V1), rider::TRANSFORM);
                dot_ab(c, vel1, xf);
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.lwc1(1, p(g(c, S1), 0x40));
                c.neg_s(0, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[A0][0] = Addr(p(sp, 0x50)).lw(c);
                if !t {
                    c.cycles += 14;
                    c.sub_s(0, 0, 1);
                    c.mfc1(2, 0);
                    c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0xa0));
                    c.lq(2, p(sp, 0xa0));
                    c.g[V1][0] = rider(c, motion);
                    c.sq(2, p(sp, 0x90));
                    let vel2 = p(g(c, V1), rider::VEL);
                    v_add(c, vel2, p(sp, 0x90), vel2);
                    c.g[A0][0] = Addr(p(sp, 0x50)).lw(c);
                }

                // ---- 0x13a020: drive the trigger animation ------------------
                'trigger: {
                    c.cycles += 2;
                    if c.g[A0][0] != 0 {
                        c.cycles += 3;
                        c.g[T0][0] = Addr(p(g(c, A0), 0xc)).lw(c);
                        let t = c.g[T0][0] == 0;
                        c.g[A1][0] = c.g[SP][0];
                        if !t {
                            c.cycles += 26;
                            // `frame+0x80` scaled by the trigger's `+0x158`
                            // and the world rate, then the object's callback.
                            c.g[A3][0] = Addr(p(g(c, T0), 0xc)).lw(c);
                            c.g[AT][0] = 0xffff_ffff_bf80_0000;
                            c.mtc1(1, 0);
                            c.g[A2][0] = sx(p(sp, 0x90));
                            c.g[A0][0] = Addr(p(g(c, A3), 0x158)).lh(c);
                            c.mfc1(3, 0);
                            c.lqc2(4, p(sp, 0x80));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, p(sp, 0xb0));
                            c.lq(2, p(sp, 0xb0));
                            let a = p(g(c, T0), g(c, A0));
                            c.g[A0][0] = sx(a);
                            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                            c.sq(2, p(sp, 0xa0));
                            c.lwc1(0, p(g(c, V1), 0x10));
                            c.cvt_s_w(0, 0);
                            c.mfc1(8, 0);
                            c.lqc2(4, p(sp, 0xa0));
                            c.qmtc2(8, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, p(sp, 0xb0));
                            c.lq(2, p(sp, 0xb0));
                            c.sq(2, p(sp, 0x90));
                            c.g[V1][0] = Addr(p(g(c, A3), 0x15c)).lw(c);
                            let target = g(c, V1);
                            call_ptr(c, target, 0x0013_a01c)?;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = rider(c, motion);
                        break 'trigger;
                    }
                    c.g[A0][0] = rider(c, motion);
                    break 'trigger;
                }

                // ---- 0x13a4cc: the airborne animation -----------------------
                'case: {
                    'state: {
                        // 0x13a1bc: the animation state decides the pose.
                        'anim: {
                            // 0x13a1a8
                            'seq: {
                                // 0x13a17c: board normal, up and side axes.
                                c.cycles += 81;
                                c.g[A1][0] = 0x2;
                                c.g[S1][0] = 0x1b6;
                                c.lq(2, p(g(c, A0), 0x1b0));
                                c.sq(2, p(sp, 0xb0));
                                c.lq(3, p(g(c, A0), 0x1b0));
                                c.sq(3, p(sp, 0xd0));
                                let xf = p(g(c, A0), rider::TRANSFORM);
                                dot_ab(c, p(sp, 0xd0), xf);
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, p(g(c, A0), rider::TRANSFORM));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0xc0));
                                v_sub(c, p(sp, 0xb0), p(sp, 0xc0), p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0xa0));
                                v_normalize(c, p(sp, 0xa0), p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0x90));
                                c.sq(2, p(g(c, A0), rider::UP));
                                c.g[V1][0] = rider(c, motion);
                                c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                                c.lqc2(5, p(g(c, V1), rider::UP));
                                c.vu0().opmula(4, 5);
                                c.vu0().opmsub(6, 5, 4);
                                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                c.sqc2(6, p(sp, 0xa0));
                                c.lq(2, p(sp, 0xa0));
                                c.sq(2, p(sp, 0x90));
                                c.sq(2, p(g(c, V1), rider::SIDE));
                                c.g[A0][0] = rider(c, motion);
                                c.g[V0][0] = Addr(p(sp, 0x4c)).lw(c);
                                let at = p(g(c, A0), rider::CONTACT);
                                st(c, at, V0);
                                c.g[A2][0] = rider(c, motion);
                                c.lq(2, p(sp, 0));
                                c.sq(2, p(g(c, A2), rider::XFORM));
                                c.g[V1][0] = rider(c, motion);
                                c.lqc2(3, p(g(c, V1), rider::POS));
                                c.lqc2(4, p(sp, 0));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, p(sp, 0xa0));
                                c.lq(4, p(sp, 0xa0));
                                c.g[V0][0] = rider(c, motion);
                                c.sq(4, p(sp, 0x90));
                                let xf = p(g(c, V0), rider::TRANSFORM);
                                dot_ab(c, p(sp, 0x90), xf);
                                c.qmfc2(3, 4);
                                c.mtc1(3, 0);
                                c.swc1(0, p(g(c, V0), rider::NORMAL));
                                c.g[V1][0] = rider(c, motion);
                                c.g[A0][0] = Addr(p(g(c, V1), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a158)?;
                                c.g[V1][0] = 0x14;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = rider(c, motion);
                                if !t {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a170)?;
                                    c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                    c.g[A1][0] = 0x2;
                                    break 'seq;
                                }
                                // 0x13a188: the air (not board) animation.
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a188)?;
                                c.g[V1][0] = 0x15;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = rider(c, motion);
                                if t {
                                    break 'anim;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a1a0)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0x3;
                                break 'seq;
                            }
                            // 0x13a1b0: test the animation layer's flags.
                            c.cycles += 5;
                            callee::FLAGS64_TEST.call(c, 0x0013_a1b0)?;
                            c.g[V1][0] = 0x169;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = c.g[V1][0];
                            }
                            break 'state;
                        }

                        // ---- 0x13a258: wall/rail animation selection ---------
                        'wall: {
                            'kind: {
                                // 0x13a1f8
                                'first: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x2;
                                    c.g[S2][0] = 0;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER, 0x0013_a1cc)?;
                                    c.g[V1][0] = 0x13;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = rider(c, motion);
                                    if t {
                                        break 'first;
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a1e4)?;
                                    c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                    c.g[A1][0] = 0x2;
                                    callee::FLAGS64_TEST.call(c, 0x0013_a1f0)?;
                                    c.g[S2][0] = (c.g[V0][0] < 1) as u64;
                                    break 'kind;
                                }
                                // 0x13a204
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a204)?;
                                c.g[V1][0] = 0x12;
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[A0][0] = rider(c, motion);
                                    break 'wall;
                                }
                                c.cycles += 9;
                                c.g[V0][0] = rider(c, motion);
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a220)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0x2;
                                callee::FLAGS64_TEST.call(c, 0x0013_a22c)?;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = rider(c, motion);
                                    break 'wall;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = rider(c, motion);
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a244)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0;
                                callee::FLAGS64_TEST.call(c, 0x0013_a250)?;
                                c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                                break 'kind;
                            }
                            // 0x13a258
                            c.cycles += 1;
                            c.g[A0][0] = rider(c, motion);
                            break 'wall;
                        }

                        // ---- 0x13a298: board side check ----------------------
                        c.cycles += 16;
                        c.mtc1(0, 5);
                        c.lq(2, p(g(c, A0), 0x1c0));
                        c.sq(2, p(sp, 0x90));
                        let xf = p(g(c, A0), rider::TRANSFORM);
                        dot_ab(c, p(sp, 0x90), xf);
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 5);
                        if c.fcc() {
                            c.g[S2][0] = 0x1;
                        }

                        // ---- 0x13a424: pick the pose from the board angle ----
                        {
                            c.cycles += 2;
                            if c.g[S2][0] == 0 {
                                c.g[V0][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                            } else {
                                // Up/down/side components of the board normal.
                                'angle: {
                                    c.cycles += 38;
                                    let xf = p(g(c, A0), rider::TRANSFORM);
                                    c.lq(2, p(g(c, A0), 0x1a0));
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 4);
                                    c.lq(2, p(g(c, A0), 0x1b0));
                                    c.abs_s(0, 4);
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 2);
                                    c.abs_s(3, 2);
                                    c.lq(2, p(g(c, A0), 0x1c0));
                                    c.c_lt_s(0, 3);
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 1);
                                    if !c.fcc() {
                                        break 'angle;
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 1);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'angle;
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 5);
                                    let t = !c.fcc();
                                    c.g[S1][0] = 0x163;
                                    if !t {
                                        c.cycles += 2;
                                        c.g[S1][0] = 0x162;
                                    }
                                    break 'state;
                                }
                                // 0x13a408: no usable board angle: use the state.
                                'back: {
                                    c.cycles += 5;
                                    c.abs_s(3, 1);
                                    c.abs_s(0, 4);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'back;
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 2);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'back;
                                    }
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_lt_s(1, 0);
                                    let t = c.fcc();
                                    c.g[S1][0] = 0x15e;
                                    if t {
                                        break 'state;
                                    }
                                    // 0x13a3f0: a random 0x15f/0x160/0x161.
                                    {
                                        // 0x13a3e4
                                        'rand_odd: {
                                            c.cycles += 15;
                                            c.g[V0][0] = rider(c, motion);
                                            c.lwc1(1, gp(c, -0x71bc));
                                            let vel = p(g(c, V0), rider::VEL);
                                            v_length(c, vel, 3, 0);
                                            c.c_lt_s(1, 0);
                                            if !c.fcc() {
                                                break 'rand_odd;
                                            }
                                            c.cycles += 4;
                                            c.g[S1][0] = 0x160;
                                            call_ptr(c, target::RAND_NEXT_GAME, 0x0013_a3dc)?;
                                            c.g[V0][0] = c.g[V0][0] & 0x3;
                                        }
                                        // 0x13a3ec
                                        c.cycles += 3;
                                        c.g[S1][0] = 0x160;
                                        call_ptr(c, target::RAND_NEXT_GAME, 0x0013_a3ec)?;
                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0;
                                    c.g[V1][0] = c.g[V0][0] ^ 0x1;
                                    if t {
                                        break 'state;
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = 0x15f;
                                    c.g[V0][0] = 0x161;
                                    if c.g[V1][0] == 0 {
                                        c.g[S1][0] = c.g[V0][0];
                                    }
                                    break 'state;
                                }
                                // 0x13a414: last resort, the animation state.
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(4, 0);
                                let t = !c.fcc();
                                c.g[S1][0] = 0x164;
                                if !t {
                                    c.cycles += 2;
                                    c.g[S1][0] = 0x165;
                                }
                                break 'state;
                            }
                        }
                        // 0x13a4b0
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x1b6;
                        if t {
                            break 'case;
                        }
                        'boost: {
                            c.cycles += 31;
                            c.lq(2, p(g(c, A0), 0x1c0));
                            c.sq(2, p(sp, 0x90));
                            let up = p(g(c, A0), rider::UP);
                            dot_ab(c, p(sp, 0x90), up);
                            c.qmfc2(3, 4);
                            c.mtc1(3, 20);
                            callee::BE_INSTANCE.call(c, 0x0013_a460)?;
                            c.g[A0][0] = sx(g(c, V0));
                            c.g[A1][0] = 0x3;
                            callee::BE_INTERFACE.call(c, 0x0013_a46c)?;
                            c.g[V1][0] = rider(c, motion);
                            c.g[A0][0] = sx(g(c, V0));
                            c.g[A2][0] = Addr(p(g(c, V1), 0xb34)).lw(c);
                            c.g[A1][0] = Addr(p(g(c, V1), 0x86c)).lw(c);
                            callee::ATTRIB6_FACTOR.call(c, 0x0013_a480)?;
                            c.lwc1(2, gp(c, -0x71b8));
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(1, 3);
                            c.mul_s(0, 0, 2);
                            c.lwc1(1, gp(c, -0x71b4));
                            c.add_s(2, 0, 3);
                            c.mul_s(1, 2, 1);
                            c.c_lt_s(1, 20);
                            if !c.fcc() {
                                break 'boost;
                            }
                            c.cycles += 2;
                            c.g[S1][0] = 0x15c;
                            break 'state;
                        }
                        // 0x13a4da
                        c.cycles += 6;
                        c.lwc1(0, gp(c, -0x71b0));
                        c.mul_s(0, 2, 0);
                        c.c_lt_s(20, 0);
                        if c.fcc() {
                            c.g[S1][0] = 0x15d;
                        }
                        break 'state;
                    }
                    // 0x13a4c8 exit: the default animation.
                    c.cycles += 1;
                    c.g[V0][0] = 0x1b6;
                }

                // ---- 0x13a540: apply the chosen animation -------------------
                c.cycles += 2;
                let t = c.g[S1][0] == c.g[V0][0];
                c.lq(2, p(sp, 0));
                if !t {
                    c.cycles += 27;
                    c.abs_s(0, 21);
                    c.g[V1][0] = rider(c, motion);
                    c.g[A1][0] = c.g[S1][0];
                    c.sq(2, p(sp, 0x90));
                    c.g[A2][0] = 0;
                    let vel = p(g(c, V1), rider::VEL);
                    v_normalize(c, vel, p(sp, 0xe0));
                    c.g[T0][0] = sx(p(sp, 0x90));
                    c.lq(3, p(sp, 0xe0));
                    c.lq(2, p(sp, 0x10));
                    c.g[A0][0] = rider(c, motion);
                    c.sq(3, p(sp, 0xa0));
                    c.sq(2, p(sp, 0xb0));
                    c.swc1(0, p(sp, 0xc0));
                    c.sq(3, p(sp, 0xd0));
                    c.g[A3][0] = Addr(p(g(c, A0), rider::CONTACT)).lw(c);
                    call_ptr(c, target::START_WIPEOUT, 0x0013_a538)?;
                    c.g[A0][0] = rider(c, motion);
                    break 'body;
                }

                // ---- 0x13a560: the "on the ground" flag --------------------
                c.cycles += 7;
                c.g[A0][0] = rider(c, motion);
                c.lwc1(1, gp(c, -0x71ac));
                c.lwc1(0, p(g(c, A0), 0x378));
                c.c_lt_s(0, 1);
                let t = c.fcc();
                c.g[A1][0] = 0x1;
                if !t {
                    c.cycles += 1;
                    c.g[A1][0] = 0;
                }

                // ---- 0x13a684: trick / landing animation -------------------
                'land_anim: {
                    c.cycles += 13;
                    c.g[A3][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                    c.mov_s(12, 22);
                    c.g[A2][0] = 0;
                    c.g[T0][0] = c.g[SP][0];
                    call_ptr(c, target::BANK_TRICK_BOOST, 0x0013_a574)?;
                    c.g[A0][0] = rider(c, motion);
                    c.g[A1][0] = 0;
                    call_ptr(c, target::SET_MAIN_STATE, 0x0013_a580)?;
                    c.g[A0][0] = rider(c, motion);
                    call_ptr(c, target::GET_SUB_STATE, 0x0013_a588)?;
                    c.g[V1][0] = 0xc;
                    if c.g[V0][0] == c.g[V1][0] {
                        c.g[V1][0] = rider(c, motion);
                        break 'land_anim;
                    }
                    'land2: {
                        'sub: {
                            c.cycles += 5;
                            c.g[A0][0] = rider(c, motion);
                            call_ptr(c, target::GET_SUB_STATE, 0x0013_a59c)?;
                            c.g[V1][0] = 0x2;
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[A0][0] = rider(c, motion);
                                break 'sub;
                            }
                            c.cycles += 7;
                            c.g[A0][0] = rider(c, motion);
                            call_ptr(c, target::TURN_AROUND, 0x0013_a5b0)?;
                            c.lwc1(0, gp(c, -0x71a8));
                            c.c_lt_s(21, 0);
                            let t = !c.fcc();
                            c.mov_s(12, 21);
                            if t {
                                break 'land2;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::PLAY_LAND_ANIM2.call(c, 0x0013_a5cc)?;
                            c.g[V1][0] = rider(c, motion);
                            break 'land_anim;
                        }
                        // 0x13a644
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if !t {
                            c.cycles += 5;
                            c.g[A0][0] = rider(c, motion);
                            call_ptr(c, target::GET_SUB_STATE, 0x0013_a5e8)?;
                            c.g[A0][0] = 0x1;
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V1][0] = rider(c, motion);
                            if t {
                                break 'land_anim;
                            }
                            'land: {
                                'land_state: {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(p(g(c, V1), 0x330)).lw(c);
                                    let t = c.g[V0][0] != c.g[A0][0];
                                    c.g[A0][0] = Addr(p(g(c, V1), rider::ANIM)).lw(c);
                                    if t {
                                        break 'land_state;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x1a;
                                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0;
                                    call_ptr(c, target::PLAY_ANIM, 0x0013_a614)?;
                                    c.g[A0][0] = rider(c, motion);
                                    break 'land;
                                }
                                // 0x13a624
                                c.cycles += 6;
                                c.g[A1][0] = 0x22;
                                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                c.mtc1(1, 12);
                                c.g[A2][0] = 0;
                                call_ptr(c, target::PLAY_ANIM, 0x0013_a630)?;
                                c.g[A0][0] = rider(c, motion);
                            }
                            // 0x13a63c
                            c.cycles += 4;
                            c.g[A1][0] = 0x1;
                            call_ptr(c, target::SET_SUB_STATE, 0x0013_a63c)?;
                            c.g[V1][0] = rider(c, motion);
                            break 'land_anim;
                        }
                        // 0x13a678
                        c.cycles += 9;
                        call_ptr(c, target::TURN_AROUND, 0x0013_a64c)?;
                        c.g[A0][0] = rider(c, motion);
                        c.g[A1][0] = 0;
                        call_ptr(c, target::SET_SUB_STATE, 0x0013_a658)?;
                        c.g[A0][0] = rider(c, motion);
                        call_ptr(c, target::TURN_AROUND, 0x0013_a660)?;
                        let t = c.g[V0][0] == 0;
                        c.mov_s(12, 21);
                        if !t {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::PLAY_LAND_ANIM.call(c, 0x0013_a670)?;
                            c.g[V1][0] = rider(c, motion);
                            break 'land_anim;
                        }
                        // 0x13a680
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        callee::PLAY_LAND_ANIM2.call(c, 0x0013_a680)?;
                        break 'land2;
                    }
                    // 0x13a680 exit
                    c.cycles += 1;
                    c.g[V1][0] = rider(c, motion);
                }

                // ---- 0x13a700: cap the downward speed -----------------------
                c.cycles += 31;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.lq(2, p(g(c, V1), 0x1b0));
                c.mov_s(2, 1);
                c.sq(2, p(sp, 0x90));
                let vel = p(g(c, V1), rider::VEL);
                v_normalize(c, vel, p(sp, 0xb0));
                c.lq(2, p(sp, 0xb0));
                c.sq(2, p(sp, 0xa0));
                dot_ab(c, p(sp, 0x90), p(sp, 0xa0));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.abs_s(0, 0);
                c.c_le_s(1, 0);
                if c.fcc() {
                    c.min_s(2, 0, 2);
                }
                // 0x13a710
                c.cycles += 9;
                c.g[V0][0] = rider(c, motion);
                c.mfc1(3, 2);
                c.lqc2(4, p(g(c, V0), rider::VEL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(g(c, V0), rider::VEL));
                c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
                call_ptr(c, target::WSCRIPT, 0x0013_a720)?;
                c.g[A0][0] = rider(c, motion);
                break 'body;
            }

            // ---- 0x13a784: bounds, walls and camera -------------------------
            'walls: {
                'collide: {
                    c.cycles += 16;
                    c.g[A1][0] = 0;
                    callee::RIDER_UPDATE_BOUNDS.call(c, 0x0013_a72c)?;
                    c.g[V1][0] = rider(c, motion);
                    c.g[S1][0] = sx(p(sp, 0x80));
                    c.g[A2][0] = Addr(gp(c, -0x1fd0)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = Addr(p(g(c, V1), rider::WALLS)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    let at = p(g(c, V0), 0x28);
                    st(c, at, A2);
                    callee::COLLIDE_WALLS.call(c, 0x0013_a74c)?;
                    c.g[A1][0] = rider(c, motion);
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A0][0] = Addr(p(g(c, A1), rider::WALLS)).lw(c);
                    let t = c.g[V0][0] == 0;
                    let at = p(g(c, A0), 0x28);
                    st(c, at, V1);
                    if t {
                        break 'collide;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = rider(c, motion);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, target::AIR_WALLS, 0x0013_a770)?;
                    break 'walls;
                }
                // 0x13a784
                c.cycles += 3;
                c.g[A0][0] = rider(c, motion);
                c.g[A1][0] = 0;
                call_ptr(c, target::AIR_WALLS, 0x0013_a784)?;
            }
            // 0x13a78c: the camera trigger manager sees the final position.
            c.cycles += 3;
            c.g[A0][0] = rider(c, motion);
            call_ptr(c, target::AIR_CAMERA, 0x0013_a78c)?;
            break 'func;
        }

        // ---- epilogue -------------------------------------------------------
        c.cycles += 8;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore_ra(c, 0xf0);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, put_vec, Rng, DATA, GAME_GP};

    /// The air state.
    const MOTION: u32 = DATA;
    /// Its rider.
    const RIDER: u32 = DATA + 0x1000;
    /// The rider's model (CastBoardRay reads its transform base).
    const MODEL: u32 = DATA + 0x2000;
    /// The board transform the ray starts from.
    const BOARD: u32 = DATA + 0x4000;
    /// The rider's wall-collision object.
    const WALLS: u32 = DATA + 0x6000;
    /// The rider state object (`rider->+0x77c`).
    const STATE: u32 = DATA + 0x7000;
    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world (no geometry, no riders).
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A rider with a board transform, a trigger entry and a zeroed world;
    /// both the board-contact path and the air path are reachable.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x139c_88, 6);
        // The BE singleton (else it reads the disc), the world and its module
        // (`+0x84`), the module's race manager (`+0xc`) with no riders.
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        w(c, WORLD + 0x84, WORLD + 0x1000);
        w(c, WORLD + 0x1000 + 0xc, WORLD + 0x2000);
        w(c, WORLD + 0x2000 + 0x7c, 0);
        // The "wall flag" CollideWalls leaves at `rider->+0xaa0 + 0x28`.
        w(c, GAME_GP.wrapping_sub(0x1fd0), 0x10);
        c.g[A0][0] = MOTION as u64;
        w(c, MOTION + RIDER, RIDER);
        // CastBoardRay: `model->+0x2c + (rider->+0x8a0 << 5)` is the board
        // transform; `+0x860`/`+0x864` are its line-test arguments.
        w(c, RIDER + 0x780, MODEL);
        w(c, MODEL + 0x2c, BOARD);
        w(c, RIDER + 0x8a0, (seed % 4) as u32);
        w(c, RIDER + 0x860, BOARD);
        w(c, RIDER + 0x864, 0);
        // The trigger entry / animation object: null, so the animation helpers
        // take their "nothing to do" path.
        w(c, RIDER + 0x788, 0);
        w(c, RIDER + 0x784, 0);
        w(c, RIDER + 0x330, if seed % 3 == 0 { 0 } else { 1 });
        w(c, RIDER + 0x378, r.f32(100.0));
        w(c, RIDER + 0xaa0, WALLS);
        // `Rider_GetSubState(rider)` is `*(*(rider+0x77c) + 0xde4)`: the air
        // sub-state 9 makes `RiderAirState_CollideWalls` return at once (the
        // whole wall-collision body needs real level geometry).
        w(c, RIDER + 0x77c, STATE);
        w(c, STATE + 0xde4, 9);
        // The board plane, the rider's position/velocity and its transform.
        let q = |r: &mut Rng| [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000];
        put_vec(c, BOARD, q(&mut r));
        put_vec(c, RIDER + 0x1a0, q(&mut r));
        put_vec(c, RIDER + 0x1b0, q(&mut r));
        put_vec(c, RIDER + 0x1c0, q(&mut r));
        put_vec(c, RIDER + 0x110, q(&mut r));
        put_vec(c, RIDER + 0x1e0, q(&mut r));
        put_vec(c, RIDER + 0x370, q(&mut r));
    }

    
}
