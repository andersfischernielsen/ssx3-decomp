//! Staging wave 208 (oc267): hand-written replacements for the hottest
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
use ssx3_ee_rt::Cpu;
use crate::guest::Addr;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_2500, mesh_anim_update, "MeshAnim_Update"),
    (0x0011_a228, rider_metrix_finish_trick, "RiderMetrix_FinishTrick"),
    (0x002f_04e0, ps2_ai_visual_effects_man_render, "PS2AIVisualEffectsMan_Render"),
    (0x0032_fac0, coll_hull_collide_aabb, "CollHull_CollideAABB"),
    (0x0022_7f58, ssx_app_tick, "SSXApp_Tick"),
    (0x0032_0368, mem_pool64_alloc, "MEM_Pool64Alloc"),
    (0x002e_3110, lens_fx_update_callback, "LensFX_UpdateCallback"),
    (0x0011_27f0, rider_select_race_path, "Rider_SelectRacePath"),
];

// ---------------------------------------------------------------------------
// MeshAnim_Update (0x352500)

/// Debris/mesh-anim instance fields (`sDebris` in instanceman.cpp).
const MESH_ANIM_UPD_VTABLE: u32 = 0xc;
const MESH_ANIM_UPD_MODEL: u32 = 0x18;
/// Remaining life in seconds; the instance kills itself when it runs out.
const MESH_ANIM_UPD_TIMER: u32 = 0x1c;
/// Per-part rotation quaternions (0x10 each).
const MESH_ANIM_UPD_ROT: u32 = 0x30;
/// Per-part spin angles (4 each).
const MESH_ANIM_UPD_SPIN: u32 = 0x330;
/// Per-part velocities (0x10 each); positions are 0x180 below.
const MESH_ANIM_UPD_VEL: u32 = 0x530;
const MESH_ANIM_UPD_GRAVITY: u32 = 0x6b4;
/// Model -> skeleton (count of parts at +4).
const MESH_ANIM_UPD_SKELETON: u32 = 0x80;
/// gp-relative pointer to the game timer (frame time at +0x14).
const MESH_ANIM_UPD_TIMER_GP: u32 = 0x2a74;
/// Vtable slot of the kill method (`vt[34]`).
const MESH_ANIM_UPD_KILL_SLOT: u32 = 0x110;
const MESH_ANIM_UPD_SIN_COS: u32 = 0x0031_be50;

/// `void MeshAnim_Update(sDebris* self)` (0x352500): count the debris life
/// timer down by the frame time and kill the instance (`vt[34](self, 1)`)
/// when it runs out; while alive, spin every part (rotation quaternion
/// times the axis-angle quaternion of half its spin, renormalised) and move
/// it (position += velocity, velocity.z -= gravity * 980 * dt).
pub fn mesh_anim_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        c.mtc1(0, 2);
        frame.save(c, S2, 0x70);
        frame.save(c, S0, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xa0);
        let me = r32(c, S2);
        c.lwc1(1, me + MESH_ANIM_UPD_TIMER);
        c.c_lt_s(2, 1);
        let alive = c.fcc();
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_UPD_TIMER_GP)).lw(c);
        if alive {
            // timer -= dt
            c.cycles += 6;
            c.lwc1(0, r32(c, V0).wrapping_add(0x14));
            c.sub_s(0, 1, 0);
            c.c_le_s(0, 2);
            let expired = c.fcc();
            c.swc1(0, me + MESH_ANIM_UPD_TIMER);
            if expired {
                c.cycles += 8;
                c.g[V0][0] = Addr(me + MESH_ANIM_UPD_VTABLE).lw(c);
                c.g[A1][0] = 0x1;
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt + MESH_ANIM_UPD_KILL_SLOT).lh(c);
                c.g[V1][0] = Addr(vt + MESH_ANIM_UPD_KILL_SLOT + 4).lw(c);
                let kill = r32(c, V1);
                c.g[A0][0] = sx(me.wrapping_add(r32(c, A0)));
                call_ptr(c, kill, 0x0035_256c)?;
                frame.restore(c, S0, 0x90);
                mesh_anim_update_epilogue(c, frame);
                return Ok(());
            }
            c.cycles += 1;
            c.lwc1(1, me + MESH_ANIM_UPD_TIMER);
        }

        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 1);
        let alive = c.fcc();
        frame.restore(c, S0, 0x90);
        if !alive {
            mesh_anim_update_epilogue(c, frame);
            return Ok(());
        }

        // Spin the parts.
        c.cycles += 5;
        c.g[V1][0] = Addr(me + MESH_ANIM_UPD_MODEL).lw(c);
        c.g[S4][0] = Addr(r32(c, V1) + MESH_ANIM_UPD_SKELETON).lw(c);
        c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 20); // f20 = 0.5
            c.g[S0][0] = sx(me.wrapping_add(MESH_ANIM_UPD_ROT));
            c.g[S1][0] = sx(me.wrapping_add(MESH_ANIM_UPD_SPIN));
            loop {
                c.cycles += 48;
                c.lwc1(12, r32(c, S1));
                c.g[A0][0] = sx(sp(c).wrapping_add(0x30)); // &sin
                c.g[A1][0] = sx(sp(c).wrapping_add(0x34)); // &cos
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.mul_s(12, 12, 20);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                call_ptr(c, MESH_ANIM_UPD_SIN_COS, 0x0035_25cc)?;
                // The axis-angle quaternion (s*axis, c) at sp+0x10.
                let rot = r32(c, S0);
                let s = sp(c);
                c.lwc1(2, rot + 0x180);
                c.lwc1(3, rot + 0x184);
                c.lwc1(1, s + 0x30);
                c.lwc1(4, rot + 0x188);
                c.mul_s(2, 1, 2);
                c.lwc1(0, s + 0x34);
                c.mul_s(3, 1, 3);
                c.mul_s(1, 1, 4);
                c.swc1(0, s + 0x1c);
                c.swc1(2, s + 0x10);
                c.swc1(3, s + 0x14);
                c.swc1(1, s + 0x18);
                // rot = rot * q
                c.lqc2(4, rot);
                c.lqc2(5, s + 0x10);
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, s + 0x20);
                c.lq(2, s + 0x20);
                c.sq(2, rot);
                c.sq(2, s);
                // Normalise.
                c.lqc2(3, rot);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, rot);
                c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
                let more = (c.g[S3][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(rot.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_25b0);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
        }

        // Move the parts.
        c.cycles += 2;
        c.g[A0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0xffff_ffff_c475_0000;
            c.mtc1(1, 2); // f2 = -980.0
            c.g[V1][0] = sx(me.wrapping_add(MESH_ANIM_UPD_VEL));
            loop {
                c.cycles += 22;
                let vel = r32(c, V1);
                let pos = vel.wrapping_sub(0x180);
                c.lqc2(3, pos);
                c.lqc2(4, vel);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, pos);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.lwc1(0, r32(c, S2) + MESH_ANIM_UPD_GRAVITY);
                c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_UPD_TIMER_GP)).lw(c);
                c.mul_s(0, 0, 2);
                c.lwc1(1, r32(c, V0).wrapping_add(0x14));
                let s = sp(c);
                Addr(s).sw(c, 0);
                c.mul_s(0, 0, 1);
                Addr(s + 4).sw(c, 0);
                Addr(s + 0xc).sw(c, 0);
                c.swc1(0, s + 8);
                c.lqc2(3, vel);
                c.lqc2(4, s);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, vel);
                c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
                let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = sx(vel.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2688);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        mesh_anim_update_epilogue(c, frame);
        Ok(())
    })
}

/// MeshAnim_Update's shared epilogue (0x3526e4).
fn mesh_anim_update_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// RiderMetrix_FinishTrick (0x11a228)

/// Direct callees of RiderMetrix_FinishTrick (all in shard s00).
mod finish_trick_callee {
    use super::*;
    pub const REMOVE_HUD_MESSAGE: Callee = Callee { addr: 0x0011_79e0, f: ee::s00::f_001179e0 };
    pub const ALL_PLAYERS_FINISHED: Callee = Callee { addr: 0x0012_a250, f: ee::s00::f_0012a250 };
    pub const BUILD_TRICK_DESC: Callee = Callee { addr: 0x0011_a8c8, f: ee::s00::f_0011a8c8 };
    pub const GET_ROUNDED_TRICK_POINTS: Callee = Callee { addr: 0x0011_7948, f: ee::s00::f_00117948 };
    pub const LOOKUP_BONUS_TRICK: Callee = Callee { addr: 0x0011_b1a8, f: ee::s00::f_0011b1a8 };
    pub const SHOW_HUD_NUMBER: Callee = Callee { addr: 0x0011_7b88, f: ee::s00::f_00117b88 };
    pub const COUNT_TRICK_REPEATS: Callee = Callee { addr: 0x0011_90f0, f: ee::s00::f_001190f0 };
    pub const SHOW_TRICK_NAME: Callee = Callee { addr: 0x0011_8ff8, f: ee::s00::f_00118ff8 };
    pub const GET_TRICK_SCORE: Callee = Callee { addr: 0x0011_7990, f: ee::s00::f_00117990 };
    pub const GET_SCORE_TIER: Callee = Callee { addr: 0x0011_9310, f: ee::s00::f_00119310 };
    pub const GET_TRICK_BONUS: Callee = Callee { addr: 0x0011_7908, f: ee::s00::f_00117908 };
    pub const ADD_COMBO_POINTS: Callee = Callee { addr: 0x0011_7638, f: ee::s00::f_00117638 };
    pub const GET_TRICK_POINTS: Callee = Callee { addr: 0x0011_7900, f: ee::s00::f_00117900 };
    pub const AWARD_POINTS: Callee = Callee { addr: 0x0011_9ef8, f: ee::s00::f_00119ef8 };
    pub const UPDATE_PEAK_A4: Callee = Callee { addr: 0x0011_7708, f: ee::s00::f_00117708 };
    /// Cross-shard calls (through the function table).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_ON_BONUS_TRICK: u32 = 0x0029_b7e0;
    pub const AUDIO_ON_TRICK_LANDED: u32 = 0x0029_b430;
    pub const BE_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
}

/// The event settings (`D_00535BC8`): kind at +0x48, game type at +0x49.
const FINISH_TRICK_EVENT_SETTINGS: u32 = 0x0053_5bc8;

/// `float RiderMetrix_FinishTrick(RiderMetrix* m, int stance, int alternate,
/// int style, int flag, int takeoff)` (0x11a228): clear the trick HUD
/// messages; unless every player has finished, build the landed trick's id
/// and, when it scored, bank it: a bonus-trick bonus (score `+0x14`, HUD
/// and audio), repeats (dividing the points), combo points, the per-run
/// statistics (points `+0x198`, bonus `+0x19c`, count `+0x110`, grade
/// counts `+0x16c`, best trick `+0x18c/+0x190`, spin/flip degrees, air
/// time, rails, peaks), the landing sound, then the HUD numbers (or, in a
/// Conquer-the-Mountain free ride, career points). Finally peaks/`+0xa4`
/// when the landing is clean. Returns the trick points over the repeats.
pub fn rider_metrix_finish_trick(c: &mut Cpu) {
    use finish_trick_callee as f;
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xc0);
        c.g[S0][0] = c.g[A1][0]; // stance
        c.g[S1][0] = c.g[A0][0]; // m
        c.g[S2][0] = c.g[A2][0]; // alternate
        c.g[S7][0] = c.g[A3][0]; // style
        c.g[FP][0] = c.g[T0][0]; // flag
        Addr(sp(c) + 0x10).sw(c, c.g[T1][0]); // takeoff
        let m = r32(c, S1);

        // Clear the trick HUD messages.
        let codes: [(u64, u32); 8] = [
            (0x1, 0x0011_a278),
            (0x2, 0x0011_a284),
            (0x4, 0x0011_a290),
            (0x1c, 0x0011_a29c),
            (0x1d, 0x0011_a2a8),
            (0x1e, 0x0011_a2b4),
            (0x1f, 0x0011_a2c0),
            (0x20, 0x0011_a2cc),
        ];
        for (code, ret_addr) in codes {
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = code;
            f::REMOVE_HUD_MESSAGE.call(c, ret_addr)?;
        }

        // Nothing is scored once everybody has finished.
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0) + 0x84).lw(c);
        c.g[A0][0] = Addr(r32(c, V1) + 0xc).lw(c);
        f::ALL_PLAYERS_FINISHED.call(c, 0x0011_a2dc)?;
        let finished = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[S0][0];
        if finished {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0xb0);
            finish_trick_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 12;
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0]; // &id
        c.g[T0][0] = c.g[S7][0];
        c.g[T1][0] = c.g[FP][0];
        f::BUILD_TRICK_DESC.call(c, 0x0011_a308)?;
        c.mtc1(0, 20); // delta = 0
        c.g[S0][0] = c.g[V0][0]; // valid
        c.g[A0][0] = c.g[S1][0];
        f::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_a318)?;
        let scored = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = Addr(sp(c) + 0x10).lw(c);
        if scored {
            finish_trick_bank(c, m)?;
        }

        // The clean-landing check (v0 = takeoff).
        c.cycles += 2;
        let check_peaks;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(m + 0x20).lw(c);
            c.cycles += 2;
            let busy = c.g[V0][0] != 0;
            c.mov_s(0, 20);
            if busy {
                check_peaks = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(m + 0x28).lw(c);
                let busy = c.g[V0][0] != 0;
                frame.restore(c, S0, 0xb0);
                if busy {
                    finish_trick_epilogue(c, frame);
                    return Ok(());
                }
                c.cycles += 1;
                c.lwc1(0, m + 0x14);
                check_peaks = true;
            }
        } else {
            c.cycles += 2;
            let styled = c.g[S7][0] != 0;
            c.mov_s(0, 20);
            if styled {
                check_peaks = false;
            } else {
                c.cycles += 2;
                if c.g[FP][0] == 0 {
                    c.lwc1(0, m + 0x14);
                    check_peaks = true;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xb0);
                    finish_trick_epilogue(c, frame);
                    return Ok(());
                }
            }
        }
        if check_peaks {
            c.cycles += 4;
            c.mtc1(0, 1);
            c.c_lt_s(1, 0);
            let mut update = c.fcc(); // score > 0
            if !update {
                c.cycles += 5;
                c.lwc1(0, m + 0xa4);
                c.c_lt_s(0, 1);
                update = c.fcc(); // +0xa4 < 0
                c.mov_s(0, 20);
            }
            if update {
                c.cycles += 5;
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 12); // 1.5
                c.g[A0][0] = c.g[S1][0];
                f::UPDATE_PEAK_A4.call(c, 0x0011_a76c)?;
                c.mov_s(0, 20);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0xb0);
        finish_trick_epilogue(c, frame);
        Ok(())
    })
}

/// RiderMetrix_FinishTrick's scoring part (0x11a320..0x11a700): bank the
/// trick; leaves v0 = takeoff (reloaded from the frame).
fn finish_trick_bank(c: &mut Cpu, m: u32) -> crate::call::Flow {
    use finish_trick_callee as f;
    c.cycles += 2;
    c.g[S6][0] = 0; // repeats
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        f::LOOKUP_BONUS_TRICK.call(c, 0x0011_a334)?;
        let bonus = (c.g[V0][0] as i64) > 0;
        c.g[A1][0] = 0x32;
        if bonus {
            c.cycles += 18;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.lwc1(2, r32(c, GP).wrapping_sub(0x79f4));
            c.lwc1(1, m + 0x14);
            c.g[A0][0] = c.g[S1][0];
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(1, 12); // 1.5
            c.mul_s(0, 0, 2);
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0;
            c.add_s(1, 1, 0);
            c.swc1(1, m + 0x14);
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a370)?;
            call_ptr(c, f::AUDIO_GET_INSTANCE, 0x0011_a378)?;
            c.g[A1][0] = Addr(m + 0x1ac).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, f::AUDIO_ON_BONUS_TRICK, 0x0011_a384)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        f::COUNT_TRICK_REPEATS.call(c, 0x0011_a390)?;
        c.g[S6][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = (0 < (c.g[S6][0] as i64)) as u64;
        f::SHOW_TRICK_NAME.call(c, 0x0011_a3a4)?;
    }

    c.cycles += 18;
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = sx(r32(c, S6).wrapping_add(1)); // divisor
    f::GET_ROUNDED_TRICK_POINTS.call(c, 0x0011_a3b0)?;
    c.g[S3][0] = c.g[V0][0]; // pending
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_SCORE.call(c, 0x0011_a3bc)?;
    c.g[S2][0] = c.g[V0][0]; // points
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S1][0];
    f::GET_SCORE_TIER.call(c, 0x0011_a3cc)?;
    c.g[S5][0] = c.g[V0][0]; // grade
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_BONUS.call(c, 0x0011_a3d8)?;
    c.div(0, S3, S0);
    c.g[S4][0] = c.g[V0][0]; // inverted
    c.g[A0][0] = c.g[S1][0];

    c.cycles += 10;
    c.g[A1][0] = c.lo[0];
    f::ADD_COMBO_POINTS.call(c, 0x0011_a3f8)?;
    c.g[A0][0] = c.g[S1][0];
    f::GET_TRICK_POINTS.call(c, 0x0011_a400)?;
    c.div(0, S2, S0);
    c.g[A2][0] = 0xa;
    c.g[V0][0] = sx(r32(c, S5) << 2);

    // Totals, grade counts, delta = points / divisor.
    c.cycles += 27;
    c.g[A1][0] = sx(m.wrapping_add(0x16c).wrapping_add(r32(c, V0)));
    c.g[V1][0] = Addr(m + 0x19c).lw(c);
    c.g[V0][0] = Addr(m + 0x110).lw(c);
    c.mtc1(S0, 1);
    c.cvt_s_w(1, 1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
    c.g[A0][0] = Addr(m + 0x198).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(m + 0x19c).sw(c, c.g[V1][0]);
    Addr(m + 0x110).sw(c, c.g[V0][0]);
    c.div_s(20, 0, 1);
    c.g[A3][0] = c.lo[0];
    c.g[S2][0] = c.g[A3][0];
    c.div(0, S2, A2);
    c.g[V0][0] = c.hi[0];
    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, S4)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    Addr(m + 0x198).sw(c, c.g[A0][0]);
    let grade_count = r32(c, A1);
    c.g[V0][0] = Addr(grade_count).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(grade_count).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(m + 0x18c).lw(c);
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S3][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        // New best trick: points and id.
        c.cycles += 5;
        Addr(m + 0x18c).sw(c, c.g[S3][0]);
        let s = sp(c);
        c.ldl(2, s + 7);
        c.ldr(2, s);
        c.sdl(2, m + 0x197);
        c.sdr(2, m + 0x190);
    }

    // Spin and flip in whole degrees, rounded to half turns.
    c.cycles += 5;
    c.lwc1(1, r32(c, GP).wrapping_sub(0x79f0));
    c.g[V1][0] = 0xb4;
    c.lwc1(0, m + 0x34);
    c.cycles += 8;
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    c.div(0, A0, V1);
    c.g[V1][0] = c.hi[0];
    let low = (c.g[V1][0] as i64) < -90;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
    if low {
        c.cycles += 2;
        c.g[A1][0] = sx(r32(c, V0).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        let small = (c.g[V1][0] as i64) < 91;
        c.g[V0][0] = small as u64;
        c.g[A1][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
            c.g[A1][0] = sx(r32(c, V0).wrapping_add(0xb4));
        }
    }
    c.cycles += 5;
    c.lwc1(1, r32(c, GP).wrapping_sub(0x79ec));
    c.g[V1][0] = 0xb4;
    c.lwc1(0, m + 0x38);
    c.cycles += 8;
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    c.div(0, A0, V1);
    c.g[A3][0] = c.hi[0];
    let low = (c.g[A3][0] as i64) < -90;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
    if low {
        c.cycles += 2;
        c.g[A3][0] = sx(r32(c, V0).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        let small = (c.g[A3][0] as i64) < 91;
        c.g[V0][0] = small as u64;
        if small {
            c.g[A3][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A3)));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(0xb4));
        }
    }
    // abs(spin) in v0, abs(flip) in a3.
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(0u32.wrapping_sub(r32(c, V0)));
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(m + 0x100).lw(c);
    c.g[A1][0] = Addr(m + 0xfc).lw(c);
    if (c.g[A3][0] as i64) < 0 {
        c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, A3)));
    }

    c.cycles += 18;
    c.lwc1(1, m + 0x144);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.lwc1(0, m + 0x44);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.g[A2][0] = Addr(m + 0x104).lw(c);
    c.g[A0][0] = Addr(m + 0x108).lw(c);
    c.add_s(1, 1, 0);
    c.g[V0][0] = Addr(m + 0x4c).lw(c);
    c.g[A3][0] = Addr(m + 0x80).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[T0][0] = Addr(m + 0x54).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    Addr(m + 0xfc).sw(c, c.g[A1][0]);
    Addr(m + 0x100).sw(c, c.g[V1][0]);
    c.swc1(1, m + 0x144);
    Addr(m + 0x104).sw(c, c.g[A2][0]);
    let rails = (c.g[T0][0] as i64) > 0;
    Addr(m + 0x108).sw(c, c.g[A0][0]);
    if rails {
        c.cycles += 7;
        call_ptr(c, f::AUDIO_GET_INSTANCE, 0x0011_a5a0)?;
        c.g[A1][0] = Addr(m + 0x1ac).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(m + 0x54).lw(c);
        c.g[A3][0] = Addr(m + 0x114).lw(c);
        call_ptr(c, f::AUDIO_ON_TRICK_LANDED, 0x0011_a5b4)?;
    }

    // Running sums and peaks.
    c.cycles += 44;
    c.lwc1(4, m + 0x14c);
    c.lwc1(5, m + 0x150);
    c.lwc1(6, m + 0x154);
    c.lwc1(7, m + 0x158);
    c.g[T1][0] = Addr(m + 0x84).lw(c);
    c.g[V1][0] = Addr(m + 0x58).lw(c);
    c.g[A1][0] = Addr(m + 0x114).lw(c);
    c.g[T2][0] = sx(r32(c, S2).wrapping_add(r32(c, T1)));
    c.g[A2][0] = Addr(m + 0x118).lw(c);
    c.g[S0][0] = sx(r32(c, T2).wrapping_add(r32(c, S4))); // total
    c.g[A3][0] = Addr(m + 0x11c).lw(c);
    c.g[T0][0] = Addr(m + 0x198).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
    c.g[A0][0] = Addr(m + 0x74).lw(c);
    c.g[V0][0] = Addr(m + 0x54).lw(c);
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, T1)));
    c.lwc1(3, m + 0x30);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    c.lwc1(2, m + 0x48);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    c.lwc1(1, m + 0x24);
    c.max_s(3, 3, 4);
    c.lwc1(0, m + 0x78);
    c.max_s(2, 2, 5);
    c.max_s(1, 1, 6);
    Addr(m + 0x114).sw(c, c.g[A1][0]);
    c.max_s(0, 0, 7);
    Addr(m + 0x118).sw(c, c.g[A2][0]);
    Addr(m + 0x11c).sw(c, c.g[A3][0]);
    c.swc1(3, m + 0x14c);
    c.swc1(2, m + 0x150);
    c.swc1(1, m + 0x154);
    c.swc1(0, m + 0x158);
    Addr(m + 0x198).sw(c, c.g[T0][0]);
    call_ptr(c, f::BE_GET_INSTANCE, 0x0011_a640)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, f::BE_GET_INTERFACE, 0x0011_a64c)?;
    c.g[V1][0] = sx(FINISH_TRICK_EVENT_SETTINGS);
    c.g[A0][0] = 0x4;
    c.g[V0][0] = Addr(FINISH_TRICK_EVENT_SETTINGS + 0x48).lb(c);
    let mountain = c.g[V0][0] == c.g[A0][0];
    c.g[A1][0] = 0;
    if mountain {
        c.cycles += 2;
        c.g[V0][0] = Addr(FINISH_TRICK_EVENT_SETTINGS + 0x49).lb(c);
        c.g[A1][0] = (c.g[V0][0] < 1) as u64;
    }

    c.cycles += 2;
    let career = c.g[A1][0] != 0;
    c.g[V0][0] = 0x1f4;
    if !career {
        c.cycles += 2;
        let repeated = (c.g[S6][0] as i64) > 0;
        c.g[A3][0] = c.g[S5][0];
        if repeated {
            c.cycles += 8;
            c.g[AT][0] = 0x4020_0000;
            c.mtc1(1, 12); // 2.5
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x24;
            c.g[A2][0] = c.g[S0][0];
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a694)?;
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x4020_0000;
            c.mtc1(1, 12); // 2.5
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x23;
            c.g[A2][0] = c.g[S0][0];
            f::SHOW_HUD_NUMBER.call(c, 0x0011_a6b4)?;
        }
        c.cycles += 8;
        c.lwc1(12, r32(c, GP).wrapping_sub(0x79e8));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x18;
        c.g[A3][0] = 0;
        f::SHOW_HUD_NUMBER.call(c, 0x0011_a6cc)?;
    } else {
        c.cycles += 4;
        c.g[V1][0] = 0x14;
        c.div(0, S0, V0);
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.lo[0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 21) as u64;
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[V1][0];
        }
        f::AWARD_POINTS.call(c, 0x0011_a6fc)?;
    }
    c.g[V0][0] = Addr(sp(c) + 0x10).lw(c);
    Ok(())
}

/// RiderMetrix_FinishTrick's epilogue (0x11a774).
fn finish_trick_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 12;
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// PS2AIVisualEffectsMan_Render (0x2f04e0)

/// `void PS2AIVisualEffectsMan_Render(Wrapper* self)` (0x2f04e0): forward to
/// the wrapped effects manager's render method (`(*self)->vt[3]()`, vtable
/// slot 0x18).
pub fn ps2_ai_visual_effects_man_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, A0)).lw(c);
        let inner = r32(c, A1);
        c.g[V0][0] = Addr(inner).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt + 0x18).lh(c);
        c.g[V1][0] = Addr(vt + 0x1c).lw(c);
        let render = r32(c, V1);
        c.g[A0][0] = sx(inner.wrapping_add(r32(c, A0)));
        call_ptr(c, render, 0x002f_0500)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CollHull_CollideAABB (0x32fac0)

/// The hull's sphere tree.
const COLL_HULL_SPHERE: u32 = 0x60;
/// Hit record: normal at +0, push direction at +0x10, its length at +0x40.
const COLL_HULL_HIT_DIR: u32 = 0x10;
const COLL_HULL_HIT_LEN: u32 = 0x40;
const COLL_HULL_COLLIDE_BOX: u32 = 0x0032_b2b8;

/// `bool CollHull_CollideAABB(CollHull* self, Vec4* boxMin, Vec4* boxMax,
/// Hit* hit)` (0x32fac0): collide the hull's sphere tree with the box
/// (`RiderSphereTree_CollideBox(sphere, min, max, &hit->dir, hit)`); on a
/// hit store the push direction's length at `+0x40` and normalise it (VU0
/// `vsqrt`, then `vdiv` by the length). Returns whether it hit.
pub fn coll_hull_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[A3][0] = sx(r32(c, S0).wrapping_add(COLL_HULL_HIT_DIR));
        c.g[T0][0] = c.g[S0][0];
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(COLL_HULL_SPHERE)).lw(c);
        call_ptr(c, COLL_HULL_COLLIDE_BOX, 0x0032_fae0)?;
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if hit {
            c.cycles += 19;
            let dir = r32(c, S0).wrapping_add(COLL_HULL_HIT_DIR);
            // len = |dir.xyz|
            c.lqc2(3, dir);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            c.mtc1(V0, 0);
            c.swc1(0, r32(c, S0).wrapping_add(COLL_HULL_HIT_LEN));
            c.g[V0][0] = 0x1;
            // dir /= len
            c.mfc1(V1, 0);
            c.qmtc2(V1, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, dir);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, dir);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXApp_Tick (0x227f58)

/// gp-relative pointer to the pad queue.
const SSX_APP_PAD_QUEUE_GP: u32 = 0x850;
const SSX_APP_PAD_QUEUE_POLL: u32 = 0x0032_6b88;

/// `void cSSXApp::timerCallback()` (0x227f58, `SSXApp_Tick`): the app's
/// timer tick polls the pad queue (`PadQueue_Poll`) if there is one.
pub fn ssx_app_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(SSX_APP_PAD_QUEUE_GP)).lw(c);
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, SSX_APP_PAD_QUEUE_POLL, 0x0022_7f70)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MEM_Pool64Alloc (0x320368)

/// Free-list head of the 64-byte pool.
const MEM_POOL64_FREE: u32 = 0x3_2000;
const MEM_POOL64_FALLBACK: Callee = Callee { addr: 0x0031_9e48, f: ee::s10::f_00319e48 };

/// `void* MEM_Pool64Alloc(Pool64* pool)` (0x320368): pop a block off the
/// 64-byte pool's free list (`pool + 0x32000`); when it is empty fall back
/// to `MEM_PoolAlloc(0x44, a1)` (a1 is whatever the caller left there).
pub fn mem_pool64_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let pool = r32(c, A0);
        c.g[V0][0] = sx(pool.wrapping_add(0x3_0000));
        let head = pool.wrapping_add(MEM_POOL64_FREE);
        let free = Addr(head).lw(c);
        c.g[V1][0] = 0x3_0000;
        if free == 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x44;
            MEM_POOL64_FALLBACK.call(c, 0x0032_038c)?;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(head);
            c.g[V0][0] = free;
            c.g[A0][0] = Addr(free as u32).lw(c);
            Addr(head).sw(c, c.g[A0][0]);
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LensFX_UpdateCallback (0x2e3110)

/// `void LensFX_UpdateCallback(LensFX* self)` (0x2e3110): the lens-flare
/// manager's per-frame callback; forwards to
/// `LensFX_UpdateSourceVisibility(self)` (gcc's 7-cycle wrapper shape).
pub fn lens_fx_update_callback(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee { addr: 0x002e_3130, f: ee::s09::f_002e3130 }),
        0x002e_3120,
    );
}

// ---------------------------------------------------------------------------
// Rider_SelectRacePath (0x1127f0)

/// Rider fields used by the race-path choice.
const SELECT_PATH_POS: u32 = 0x110;
const SELECT_PATH_VEL: u32 = 0x1e0;
const SELECT_PATH_TRACK_DIST: u32 = 0x4d0;
const SELECT_PATH_CURRENT: u32 = 0xab4;
const SELECT_PATH_AI_STATE: u32 = 0xac0;
/// The path data (`D_004D33A0`).
const SELECT_PATH_DATA: u32 = 0x004d_33a0;
const SELECT_PATH_LOOK_AHEAD: Callee = Callee { addr: 0x0011_2588, f: ee::s00::f_00112588 };
const SELECT_PATH_FIND_NEAR: u32 = 0x0026_b178;
const SELECT_PATH_CLOSEST_POINT: u32 = 0x0026_a428;
const SELECT_PATH_POINT_AT: u32 = 0x0026_ab20;

/// `void Rider_SelectRacePath(Rider* self, bool skipCurrent)` (0x1127f0):
/// pick the race path for the AI. The probe point is the position, moved
/// ahead along the velocity by the look-ahead distance
/// (`Rider_GetAIPathLookAheadDist(self, 1)`); of up to three nearby track
/// paths (`PathData_FindNearTrackPaths`), optionally skipping the current
/// one, choose the one minimising |pos - closest point|^2 + |probe - point
/// 796 further along|^2 (`Path_GetClosestPoint`, `Path_GetPointAtDistance`).
/// If the current path's distance disagrees with the rider's by more than
/// 1592, the AI state's `+0x14` is reset to -1. The choice goes to `+0xab4`.
pub fn rider_select_race_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x110);
        c.mtc1(0, 0);
        frame.save(c, S1, 0xd0);
        frame.save(c, S4, 0xa0);
        frame.save_fpr(c, 21, 0xf8);
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 22, 0x100);
        frame.save_fpr(c, 20, 0xf0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        let me = r32(c, S1);
        // f20 = |velocity.xyz|
        c.lqc2(3, me.wrapping_add(SELECT_PATH_VEL));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 20);
        c.lwc1(21, r32(c, GP).wrapping_sub(0x7c80)); // best = 1e37
        c.c_eq_s(20, 0);
        let still = c.fcc();
        c.g[S3][0] = Addr(me.wrapping_add(SELECT_PATH_CURRENT)).lw(c); // best path
        let s = sp(c);
        if still {
            c.cycles += 3;
            c.lq(V0, me.wrapping_add(SELECT_PATH_POS));
            c.sq(V0, s + 0x20);
        } else {
            // probe = pos + vel * (lookahead / |vel|)
            c.cycles += 18;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x1;
            SELECT_PATH_LOOK_AHEAD.call(c, 0x0011_2880)?;
            let me = r32(c, S1);
            c.div_s(0, 0, 20);
            c.mfc1(V1, 0);
            c.lqc2(4, me.wrapping_add(SELECT_PATH_VEL));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s + 0x40);
            c.lqc2(3, me.wrapping_add(SELECT_PATH_POS));
            c.lqc2(4, s + 0x40);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s + 0x50);
            c.lq(V0, s + 0x50);
            c.sq(V0, s + 0x20);
            c.sq(V0, s + 0x30);
        }

        // Up to three track paths near the probe (into sp+0).
        c.cycles += 9;
        c.g[A1][0] = sx(s + 0x20);
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = sx(SELECT_PATH_DATA);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x3;
        call_ptr(c, SELECT_PATH_FIND_NEAR, 0x0011_28d8)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x44c7_0000;
            c.mtc1(AT, 22); // 1592.0
            c.g[AT][0] = 0x4447_0000;
            c.mtc1(AT, 20); // 796.0
            c.g[S5][0] = u64::MAX;
            c.g[S0][0] = c.g[SP][0];
            loop {
                let slot = r32(c, S0);
                c.cycles += 2;
                let mut skip = false;
                if c.g[S4][0] != 0 {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(slot).lw(c);
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SELECT_PATH_CURRENT)).lw(c);
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                        skip = true;
                    } else {
                        c.cycles += 1;
                    }
                }
                if !skip {
                    c.g[A0][0] = Addr(slot).lw(c);
                    // Closest point (sp+0x10) and its distance along the path.
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = sx(s + 0x10);
                    c.g[A3][0] = sx(s + 0x60);
                    c.g[T0][0] = 0x1;
                    call_ptr(c, SELECT_PATH_CLOSEST_POINT, 0x0011_2928)?;
                    let me = r32(c, S1);
                    c.g[V1][0] = Addr(slot).lw(c);
                    c.g[V0][0] = Addr(me.wrapping_add(SELECT_PATH_CURRENT)).lw(c);
                    let current = c.g[V1][0] == c.g[V0][0];
                    c.mov_s(12, 0);
                    let mut lost = true;
                    if current {
                        c.cycles += 9;
                        c.lwc1(0, r32(c, V1).wrapping_add(0x38));
                        c.lwc1(1, me.wrapping_add(SELECT_PATH_TRACK_DIST));
                        c.sub_s(0, 0, 1);
                        c.sub_s(0, 0, 12);
                        c.abs_s(0, 0);
                        c.c_lt_s(22, 0);
                        if c.fcc() {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(me.wrapping_add(SELECT_PATH_AI_STATE)).lw(c);
                            Addr(r32(c, V0).wrapping_add(0x14)).sw(c, c.g[S5][0]);
                        } else {
                            lost = false;
                        }
                    }
                    if lost {
                        c.cycles += 1;
                    }
                    c.add_s(12, 12, 20); // t + 796

                    // dist = |pos - closest|^2 + |probe - ahead|^2
                    c.cycles += 39;
                    c.g[A1][0] = Addr(slot).lw(c);
                    c.g[A0][0] = sx(s + 0x40);
                    c.g[A2][0] = 0;
                    call_ptr(c, SELECT_PATH_POINT_AT, 0x0011_2978)?;
                    let me = r32(c, S1);
                    c.lqc2(3, s + 0x20);
                    c.lqc2(4, s + 0x40);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, s + 0x50);
                    c.lq(V0, s + 0x50);
                    c.sq(V0, s + 0x30);
                    c.lqc2(3, me.wrapping_add(SELECT_PATH_POS));
                    c.lqc2(4, s + 0x10);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, s + 0x50);
                    c.lq(V0, s + 0x50);
                    c.sq(V0, s + 0x40);
                    c.lqc2(3, s + 0x40);
                    c.lqc2(5, s + 0x40);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(V1, 4);
                    c.mtc1(V1, 1);
                    c.lqc2(3, s + 0x30);
                    c.lqc2(5, s + 0x30);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(V0, 4);
                    c.mtc1(V0, 0);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 21);
                    let better = c.fcc();
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                    if better {
                        c.cycles += 2;
                        c.g[S3][0] = Addr(slot).lw(c);
                        c.mov_s(21, 1);
                    }
                }
                c.cycles += 2;
                let more = c.g[S2][0] != 0;
                c.g[S0][0] = sx(slot.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_28f8);
                }
            }
        }

        c.cycles += 14;
        Addr(r32(c, S1).wrapping_add(SELECT_PATH_CURRENT)).sw(c, c.g[S3][0]);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{elf_image, put_vec, DATA, GAME_GP};

    /// MeshAnim_Update: a debris instance whose timer is <= 0 (idle), runs
    /// out this frame (kill method, a fake that unwinds on some seeds) or
    /// stays alive; 0..5 parts (or a negative count) spun through the real
    /// Math_SinCos and moved under gravity.
    #[test]
    fn mesh_anim_update_matches() {
        covered(ee::s11::f_00352500__generated, mesh_anim_update, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_2500, 1);
            elf_image(c);
            let me = DATA + 0x1000;
            let model = DATA + 0x3000;
            let skeleton = DATA + 0x3100;
            let timer_obj = DATA + 0x3200;
            let vt = DATA + 0x3400;
            let dt = 1.0f32 / 60.0;
            c.bus.write32(GAME_GP + MESH_ANIM_UPD_TIMER_GP, timer_obj);
            c.bus.write32(timer_obj + 0x14, dt.to_bits());
            let timer = match seed % 4 {
                0 => -((r.next_u32() % 3) as f32) * 0.5,
                1 => dt * 0.5,
                _ => 1.0 + (r.next_u32() % 100) as f32 * 0.01,
            };
            c.bus.write32(me + MESH_ANIM_UPD_TIMER, timer.to_bits());
            c.bus.write32(me + MESH_ANIM_UPD_VTABLE, vt);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MESH_ANIM_UPD_KILL_SLOT, delta as u16);
            c.bus.write32(vt + MESH_ANIM_UPD_KILL_SLOT + 4, FAKE);
            c.bus.write32(me + MESH_ANIM_UPD_MODEL, model);
            c.bus.write32(model + MESH_ANIM_UPD_SKELETON, skeleton);
            let parts: i32 = match seed % 7 {
                0 => 0,
                1 => -1,
                _ => (r.next_u32() % 5 + 1) as i32,
            };
            c.bus.write32(skeleton + 4, parts as u32);
            c.bus.write32(me + MESH_ANIM_UPD_GRAVITY, r.f32(2.0));
            for i in 0..6 {
                put_vec(c, me + MESH_ANIM_UPD_ROT + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0.5f32.to_bits()]);
                put_vec(c, me + MESH_ANIM_UPD_ROT + 0x180 + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.next_u32()]);
                c.bus.write32(me + MESH_ANIM_UPD_SPIN + 4 * i, r.f32(6.0));
                put_vec(c, me + MESH_ANIM_UPD_VEL - 0x180 + 0x10 * i,
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.next_u32()]);
                put_vec(c, me + MESH_ANIM_UPD_VEL + 0x10 * i,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.next_u32()]);
            }
            c.g[A0][0] = sx(me);
            c.pc = 0x0035_2500;
        });
    }

    /// RiderMetrix_FinishTrick with its real callees: a world whose single
    /// player has (not) finished, a zeroed BELibrary dummy and audio system,
    /// a metrix record (cleared, then random score/points/spin/flip/peaks,
    /// best trick, landing flags), the event settings (Conquer-the-Mountain
    /// free ride or not) and every takeoff/style/flag combination.
    #[test]
    fn rider_metrix_finish_trick_matches() {
        crate::difftest::check(ee::s00::f_0011a228__generated, rider_metrix_finish_trick, 160, &|c, seed| {
            let mut r = machine(c, seed, 0x11_a228, 0);
            elf_image(c);
            let be = DATA + 0x20_0000;
            let world = DATA + 0x21_0000;
            let game = world + 0x1000;
            let race = world + 0x2000;
            let player = world + 0x3000;
            let player_state = world + 0x4000;
            let audio = DATA + 0x22_0000;
            for a in [be, world, audio] {
                for i in 0..0x1000 / 4 {
                    c.bus.write32(a + 4 * i, 0);
                }
            }
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(GAME_GP + 0x410, audio);
            c.bus.write32(world + 0x84, game);
            c.bus.write32(game + 0xc, race);
            // One player; finished when its +0x470 is >= 0.
            c.bus.write32(race + 0x7c, 1);
            c.bus.write32(race + 0x40, player);
            c.bus.write32(player + 0x18, player_state);
            let finished = seed % 10 == 0;
            c.bus.write32(player_state + 0x470, (if finished { 1.0f32 } else { -1.0 }).to_bits());
            // The event settings.
            let kind = if seed % 3 == 0 { 4 } else { (r.next_u32() % 4) as u8 };
            c.bus.write8(FINISH_TRICK_EVENT_SETTINGS + 0x48, kind);
            c.bus.write8(FINISH_TRICK_EVENT_SETTINGS + 0x49, (seed / 3 % 2) as u8);

            let m = DATA + 0x1000;
            for i in 0..0x200 / 4 {
                c.bus.write32(m + 4 * i, 0);
            }
            let score = match seed % 5 {
                0 => 0.0f32,
                1 => -((r.next_u32() % 50) as f32),
                _ => (r.next_u32() % 500) as f32,
            };
            c.bus.write32(m + 0x14, score.to_bits());
            // Trick points (0 on some seeds: nothing scored).
            let points = if seed % 7 == 0 { 0.0f32 } else { (r.next_u32() % 4000) as f32 };
            c.bus.write32(m + 0x1c4, points.to_bits());
            c.bus.write32(m + 0x34, r.f32(20.0));
            c.bus.write32(m + 0x38, r.f32(20.0));
            for off in [0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144] {
                c.bus.write32(m + off, r.f32(10.0));
            }
            for off in [0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c] {
                c.bus.write32(m + off, r.next_u32() % 100);
            }
            for off in [0x198, 0x19c, 0x110, 0x18c] {
                c.bus.write32(m + off, r.next_u32() % 3000);
            }
            c.bus.write32(m + 0xa4, r.f32(2.0));
            c.bus.write32(m + 0x20, if seed % 4 == 1 { 1 } else { 0 });
            c.bus.write32(m + 0x28, if seed % 6 == 2 { 1 } else { 0 });
            c.g[A0][0] = sx(m);
            c.g[A1][0] = (r.next_u32() % 2) as u64;
            c.g[A2][0] = (r.next_u32() % 2) as u64;
            c.g[A3][0] = if seed % 4 == 3 { 1 } else { 0 };
            c.g[T0][0] = if seed % 8 == 5 { 1 } else { 0 };
            c.g[T1][0] = if seed % 2 == 0 { 1 } else { 0 };
            c.pc = 0x0011_a228;
        });
    }

    /// PS2AIVisualEffectsMan_Render: the wrapped manager's render method is
    /// a fake (random this-delta; unwinding on some seeds).
    #[test]
    fn ps2_ai_visual_effects_man_render_matches() {
        covered(ee::s09::f_002f04e0__generated, ps2_ai_visual_effects_man_render, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2f_04e0, 1);
            let wrapper = DATA + 0x100;
            let inner = DATA + 0x200;
            let vt = DATA + 0x400;
            c.bus.write32(wrapper, inner);
            c.bus.write32(inner, vt);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + 0x18, delta as u16);
            c.bus.write32(vt + 0x1c, FAKE);
            c.g[A0][0] = sx(wrapper);
            c.pc = 0x002f_04e0;
        });
    }

    /// CollHull_CollideAABB with the real RiderSphereTree_CollideBox (the
    /// fixture of its own test): boxes around the origin, the hull's sphere
    /// inside (hit: length and normalised direction) or outside (miss),
    /// with or without a sub-sphere mask.
    #[test]
    fn coll_hull_collide_aabb_matches() {
        use crate::difftest::{fill, put_words, scramble, Rng};
        const HULL: u32 = DATA + 0x1000;
        const BOX: u32 = DATA + 0x2000;
        const SPHERE: u32 = DATA + 0x3000;
        const HIT: u32 = DATA + 0x4000;
        crate::difftest::check(ee::s11::f_0032fac0__generated, coll_hull_collide_aabb, 300, &|c, seed| {
            let mut r = Rng(0x32_fac0 ^ (seed as u64) << 24);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x0032_fac0;
            put_words(c, GAME_GP - 0x2df0, &[0x7149_f2ca, 0x7149_f2ca]);
            let one = 1.0f32.to_bits();
            put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
            put_vec(c, 0x004f_f150, [0, one, 0, 0]);
            put_vec(c, 0x004f_f160, [0, 0, one, 0]);
            let coord = |r: &mut Rng, lo: f32, hi: f32| {
                (lo + (r.next_u32() % 1000) as f32 * (hi - lo) / 1000.0).to_bits()
            };
            let outside = seed % 5 == 0;
            for i in 0..3 {
                c.bus.write32(BOX + 4 * i, coord(&mut r, -10.0, 0.0));
                c.bus.write32(BOX + 0x10 + 4 * i, coord(&mut r, 0.0, 10.0));
                let centre = if outside { coord(&mut r, 20.0, 30.0) } else { coord(&mut r, -9.0, 9.0) };
                c.bus.write32(SPHERE + 0x10 + 4 * i, centre);
            }
            c.bus.write32(SPHERE + 0x1c, one);
            c.bus.write32(SPHERE + 0x20, coord(&mut r, 0.0, 5.0));
            let (mask, count) = if seed % 3 == 0 {
                (0, r.next_u32())
            } else {
                ((r.next_u32() & 0x7f), (r.next_u32() % 8).wrapping_sub(1))
            };
            c.bus.write32(SPHERE + 0x28, mask);
            c.bus.write32(SPHERE + 0x2c, count);
            fill(c, &mut r, HIT, 0x14);
            c.bus.write32(HULL + COLL_HULL_SPHERE, SPHERE);
            c.g[A0][0] = sx(HULL);
            c.g[A1][0] = sx(BOX);
            c.g[A2][0] = sx(BOX + 0x10);
            c.g[A3][0] = sx(HIT);
        });
    }

    /// SSXApp_Tick: no pad queue, or one with the real PadQueue_Poll (the
    /// fixture of its own test: 0..4 devices, or none, whose poll methods
    /// are fakes unwinding on some seeds).
    #[test]
    fn ssx_app_tick_matches() {
        crate::difftest::check(ee::s05::f_00227f58__generated, ssx_app_tick, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x22_7f58, 4);
            c.pc = 0x0022_7f58;
            let q = DATA;
            let present = seed % 5 != 0;
            c.bus.write32(GAME_GP - SSX_APP_PAD_QUEUE_GP, if present { q } else { 0 });
            let n = if seed % 9 == 0 { u32::MAX } else { seed % 5 };
            c.bus.write32(q + 0x2ee8, n);
            c.bus.write32(q + 0x2ee4, if seed % 3 == 0 { 29 } else { r.next_u32() % 30 });
            for i in 0..4u32 {
                let dev = DATA + 0x4000 + 0x40 * i;
                let vt = DATA + 0x5000 + 0x40 * i;
                c.bus.write32(q + 0x2eec + 4 * i, dev);
                c.bus.write32(dev, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE + 0x10 * i);
            }
        });
    }

    /// MEM_Pool64Alloc: a pool with free blocks (pop) or none (the real
    /// MEM_PoolAlloc(0x44, a1) on the fixture of its own test: size-class
    /// pools ready or not, a real dlmalloc heap behind MEM_AllocInternal;
    /// a1 an alignment 0..128).
    #[test]
    fn mem_pool64_alloc_matches() {
        use crate::game::bx::cubicspline::tests::oc238_run;
        crate::difftest::check(ee::s10::f_00320368__generated, mem_pool64_alloc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x32_0368, 0);
            let pool64 = DATA + 0x20_0000;
            let free = seed % 3 != 0;
            if free {
                let block = DATA + 0x30_0000 + 0x40 * (r.next_u32() % 16);
                c.bus.write32(pool64 + MEM_POOL64_FREE, block);
                c.bus.write32(block, if seed % 2 == 0 { 0 } else { block + 0x40 });
            } else {
                c.bus.write32(pool64 + MEM_POOL64_FREE, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + 0xdd8, 1);
                    const POOLS: [(u32, u32, u32); 7] = [
                        (0x2a70, 0x800, 0x40),
                        (0x2a60, 0x2000, 0x40),
                        (0x2a6c, 0x6000, 0x40),
                        (0x2a7c, 0x32000, 0x40),
                        (0x2a84, 0x25800, 0x80),
                        (0x2a78, 0x20000, 0x100),
                        (0x2a68, 0x20000, 0x200),
                    ];
                    let mut pool = DATA + 0xc_0000;
                    let mut base = DATA + 0x8_0000;
                    for (slot, head, stride) in POOLS {
                        for n in 0..64u32 {
                            let node = base + stride * n;
                            c.bus.write32(node, if n == 63 { 0 } else { node + stride });
                        }
                        c.bus.write32(pool + head, base);
                        c.bus.write32(GAME_GP + slot, pool);
                        pool += 0x100;
                        base += 0x8000;
                    }
                } else {
                    c.bus.write32(GAME_GP + 0xdd8, 0);
                }
            }
            c.g[A0][0] = sx(pool64);
            c.g[A1][0] = [0u64, 4, 8, 16, 32, 64, 128][(r.next_u32() % 7) as usize];
            c.pc = 0x0032_0368;
        });
    }

    /// LensFX_UpdateCallback with the real LensFX_UpdateSourceVisibility on
    /// the fixture of its own test (sources with 0..3 elements, occlusion
    /// queries through a fake or a real return-0 method, disabled flags).
    #[test]
    fn lens_fx_update_callback_matches() {
        use crate::difftest::fill;
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        crate::difftest::check(ee::s09::f_002e3110__generated, lens_fx_update_callback, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_3110, 8);
            c.pc = 0x002e_3110;
            let (this, vt, targets) = (DATA, DATA + 0x10000, DATA + 0x20000);
            let target = |r: &mut crate::difftest::Rng| targets + (r.next_u32() & 0xff0);
            fill(c, &mut r, targets, 0x420);
            c.bus.write32(this + 0x6494, (seed % 7 == 0) as u32);
            c.bus.write32(GAME_GP + 0xa28, (seed % 5 != 1) as u32);
            let count = match seed % 4 {
                3 => (r.next_u32() % 3) as i32 - 1,
                k => k as i32,
            };
            c.bus.write32(this + 0x6490, count as u32);
            c.bus.write32(this + 0x649c, vt);
            c.bus.write16(vt + 0x20, (r.next_u32() & 0x3c) as u16);
            c.bus.write32(vt + 0x24, if seed % 3 == 0 { RETURN_ZERO } else { FAKE });
            for i in 0..3u32 {
                let src = this + 0x210 * i;
                c.bus.write16(src + 0x609e, (r.next_u32() % 3 != 0) as u16);
                c.bus.write32(src + 0x6694, r.f32(1.0));
                c.bus.write32(src + 0x6274, r.f32(1.0));
                let n = (r.next_u32() % 5) as i32 - 1;
                c.bus.write32(this + 0x60 + 4 * i, n as u32);
                for k in 0..4u32 {
                    let el = this + 0x88 + 0x3000 * i + 0x30 * k;
                    let t = target(&mut r);
                    c.bus.write32(el, t);
                    c.bus.write16(el + 0xe, (r.next_u32() % 3 != 0) as u16);
                    let t = target(&mut r);
                    c.bus.write32(el + 0x6000, t);
                }
            }
            c.g[A0][0] = sx(this);
        });
    }

    /// Rider_SelectRacePath with its real callees (look-ahead distance,
    /// PathData_FindNearTrackPaths, Path_GetClosestPoint,
    /// Path_GetPointAtDistance) on the game's literals: 0..4 track paths of
    /// 1..4 segments whose bounding boxes contain the probe or not; the
    /// rider standing still or moving, its current path one of them (with
    /// a track distance near or far from the closest point's) or another;
    /// skipping the current path or not.
    #[test]
    fn rider_select_race_path_matches() {
        crate::difftest::check(ee::s00::f_001127f0__generated, rider_select_race_path, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_27f0, 0);
            elf_image(c);
            let me = DATA + 0x1000;
            let ai_state = DATA + 0x3000;
            let tracks = DATA + 0x1_0000;
            let n = match seed % 6 {
                0 => 0,
                5 => 4,
                k => k,
            };
            c.bus.write32(SELECT_PATH_DATA + 0x10, n);
            c.bus.write32(SELECT_PATH_DATA + 0x14, tracks);
            for i in 0..4u32 {
                let t = tracks + 0x3c * i;
                let segs = DATA + 0x2_0000 + 0x100 * i;
                c.bus.write32(t + 8, 1 + r.next_u32() % 4);
                for k in 0..3 {
                    c.bus.write32(t + 0xc + 4 * k, r.f32(50.0));
                }
                c.bus.write32(t + 0x18, segs);
                let inside = r.next_u32() % 4 != 0;
                for k in 0..3 {
                    let (lo, hi) = if inside { (-200.0f32, 200.0f32) } else { (300.0, 400.0) };
                    c.bus.write32(t + 0x1c + 4 * k, lo.to_bits());
                    c.bus.write32(t + 0x28 + 4 * k, hi.to_bits());
                }
                c.bus.write32(t + 0x38, ((r.next_u32() % 4000) as f32).to_bits());
                for k in 0..4 {
                    let a = (r.next_u32() % 628) as f32 / 100.0;
                    let z = (r.next_u32() % 100) as f32 / 400.0;
                    let seg = segs + 0x10 * k;
                    c.bus.write32(seg, a.cos().to_bits());
                    c.bus.write32(seg + 4, a.sin().to_bits());
                    c.bus.write32(seg + 8, z.to_bits());
                    c.bus.write32(seg + 0xc, (10.0 + (r.next_u32() % 5000) as f32 / 100.0).to_bits());
                }
            }
            put_vec(c, me + SELECT_PATH_POS, [r.f32(60.0), r.f32(60.0), r.f32(60.0), 1.0f32.to_bits()]);
            let vel = if seed % 4 == 0 {
                [0, 0, 0, 0]
            } else {
                [r.f32(30.0), r.f32(30.0), r.f32(30.0), 0]
            };
            put_vec(c, me + SELECT_PATH_VEL, vel);
            let current = match r.next_u32() % 3 {
                0 => DATA + 0x5000,
                _ => tracks + 0x3c * (r.next_u32() % 3),
            };
            c.bus.write32(me + SELECT_PATH_CURRENT, current);
            c.bus.write32(me + SELECT_PATH_AI_STATE, ai_state);
            c.bus.write32(ai_state + 0x14, r.next_u32());
            // Near (within 1592 of the current path's base) or far.
            let base = f32::from_bits(c.bus.read32(current + 0x38));
            let near = if seed % 3 == 0 { base + 5000.0 } else { base - 20.0 };
            c.bus.write32(me + SELECT_PATH_TRACK_DIST, near.to_bits());
            c.g[A0][0] = sx(me);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
            c.pc = 0x0011_27f0;
        });
    }
}
