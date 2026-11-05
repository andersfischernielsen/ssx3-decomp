//! Staging wave 204 (oc233): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::abi;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_3e28, rider_anim_blend_seq4, "RiderAnim_BlendSeq4"),
    (0x002e_1f70, dynamic_spray_fx_update_emitter8, "cDynamicSprayFX_UpdateEmitter8"),
    (0x002d_1d10, halo_modifier_draw_quad, "HaloModifier_DrawQuad"),
    (0x0022_9ba8, crowd_man_2d_render_sprites, "CrowdMan2d_RenderSprites"),
    (0x002f_0a98, ps2_visual_effects_man_update, "PS2VisualEffectsMan_Update"),
    (0x002e_3130, lens_fx_update_source_visibility, "LensFX_UpdateSourceVisibility"),
    (0x0041_1340, fixunssfdi, "__fixunssfdi"),
    (0x0010_b0e8, computer_actor_rider_update_spin_plan, "ComputerActorRider_UpdateSpinPlan"),
    (0x0028_9688, ssx_audio_system_update_speech_ducking, "SSXAudioSystem_UpdateSpeechDucking"),
    (0x0034_c668, cflag_manager_update, "cFlagManager_Update"),
];

// ------------------------------------------------------- RiderAnim_BlendSeq4

/// `AnimSequence_SetTrackAnim(seq, track, anim)`.
const ANIM_SEQ_SET_TRACK_ANIM: u32 = 0x0031_3c50;
/// `AnimSequence_SetTrackTime(seq, track, time)`.
const ANIM_SEQ_SET_TRACK_TIME: u32 = 0x0031_3cf0;
/// `AnimSequence_SetTrackWeight(seq, track, weight)`.
const ANIM_SEQ_SET_TRACK_WEIGHT: u32 = 0x0031_3d28;
/// `AnimSequence_SetTrackLoop(seq, track, loop)`.
const ANIM_SEQ_SET_TRACK_LOOP: u32 = 0x0031_3d40;
/// `AnimSequence_AdvanceTrack(seq, track, step)`.
const ANIM_SEQ_ADVANCE_TRACK: u32 = 0x0031_35b0;
/// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
const ANIM_SEQ_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
/// `AnimSeqList_Remove(list, seq)`.
const ANIM_SEQ_LIST_REMOVE: u32 = 0x0031_45f8;

/// `int RiderAnim_BlendSeq4(RiderAnim* self, SeqList* list, AnimSequence*
/// seq, int a0, int a1, int a2, int a3, int a4, float dt, float v)`
/// (0x103e28): a five-key blend along `v` in [-1, 1] (e.g. lean or carve
/// amount). `v` picks the two neighbouring keys (`from`, `to`) of
/// `a0..a4` at -1, -0.5, 0, 0.5, 1 and the weight `w` of `from`. If track 0
/// does not already play `from`'s animation, switch it, keeping its
/// relative time. Track 1 plays `to` with weight `1 - w`, synchronised to
/// track 0's phase. Then advance by `dt / 60` and, when the weight fade
/// ends, remove the sequence from `list`. Returns whether it was removed.
pub fn rider_anim_blend_seq4(c: &mut Cpu) {
    /// `gp+0xd8c`: the rider animation bank (animation ids at +0x1030).
    const ANIM_BANK_GP: u32 = 0xd8c;
    const BANK_IDS: u32 = 0x1030;
    /// `gp-0x7f68`: 1/60, ticks to seconds.
    const TICK_GP: u32 = 0x7f68;
    /// AnimSequence fields: track 0's animation id, time, length; track 1's
    /// speed.
    const SEQ_ANIM0: u32 = 0x4;
    const SEQ_TIME0: u32 = 0x8;
    const SEQ_LENGTH0: u32 = 0x10;
    const SEQ_SPEED1: u32 = 0x2c;
    guest(c, |c| {
        c.cycles += 15;
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = 0x3f00_0000;
        c.f[0] = half.0;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x30);
        let v = FpuF32(c.f[13]);
        let above_half = abi::c_lt_s(c, half, v);
        frame.save(c, S2, 0x10);
        frame.save_fpr(c, 22, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.f[22] = c.f[12];
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        // The key pair: `from` in t3, `to` in s1; the weight in f21.
        let w;
        if above_half {
            // (0.5, 1]: a3 -> a4, w = (v - 0.5) * 2 (from is already t3).
            c.cycles += 4;
            let d = v - half;
            c.f[0] = d.0;
            c.g[S1][0] = c.g[T2][0];
            w = d + d;
        } else {
            c.cycles += 5;
            c.f[0] = 0;
            let above_zero = abi::c_lt_s(c, FpuF32::ZERO, v);
            c.g[T3][0] = c.g[T2][0];
            if above_zero {
                // (0, 0.5]: a2 -> a3, w = v * 2.
                c.cycles += 3;
                c.g[S1][0] = c.g[T1][0];
                w = v + v;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                let neg_half = FpuF32(0xbf00_0000);
                c.f[0] = neg_half.0;
                let above_neg_half = abi::c_lt_s(c, neg_half, v);
                c.g[T3][0] = c.g[T1][0];
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = FpuF32::ONE.0;
                if above_neg_half {
                    // (-0.5, 0]: a1 -> a2, w = v * 2 + 1.
                    c.cycles += 6;
                    let twice = v + v;
                    c.f[1] = twice.0;
                    c.g[S1][0] = c.g[T0][0];
                    w = twice + FpuF32::ONE;
                } else {
                    // [-1, -0.5]: a0 -> a1, w = (v + 1) * 2.
                    c.cycles += 6;
                    c.g[T3][0] = c.g[T0][0];
                    c.g[S1][0] = c.g[A3][0];
                    let d = v + FpuF32::ONE;
                    c.f[0] = d.0;
                    w = d + d;
                }
            }
        }
        c.f[21] = w.0;

        // Track 0 plays `from`: switch it unless it already does.
        c.cycles += 7;
        let seq = r32(c, S0);
        let bank = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        c.g[V0][0] = bank;
        let entry = sx((r32(c, T3) << 2).wrapping_add(bank as u32));
        c.g[V1][0] = entry;
        c.g[A0][0] = Addr(seq + SEQ_ANIM0).lw(c);
        c.g[A2][0] = Addr((entry as u32).wrapping_add(BANK_IDS)).lw(c);
        let same = c.g[A2][0] == c.g[A0][0];
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        if !same {
            c.cycles += 12;
            let phase = Addr(seq + SEQ_TIME0)
                .read_fpu(c)
                .div(Addr(seq + SEQ_LENGTH0).read_fpu(c));
            c.f[0] = Addr(seq + SEQ_LENGTH0).read_fpu(c).0;
            c.f[20] = phase.0;
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = 0;
            call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, 0x0010_3f0c)?;
            let time = Addr(seq + SEQ_LENGTH0).read_fpu(c) * FpuF32(c.f[20]);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = 0;
            c.f[12] = time.0;
            call_ptr(c, ANIM_SEQ_SET_TRACK_TIME, 0x0010_3f20)?;
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(ANIM_BANK_GP)).lw(c);
        }

        // Track 1 plays `to` with the complementary weight.
        c.cycles += 39;
        let to = sx((r32(c, S1) << 2).wrapping_add(r32(c, V1)));
        c.g[V0][0] = to;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.g[A2][0] = Addr((to as u32).wrapping_add(BANK_IDS)).lw(c);
        call_ptr(c, ANIM_SEQ_SET_TRACK_ANIM, 0x0010_3f3c)?;
        c.f[12] = c.f[21];
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3f4c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.f[12] = (FpuF32::ONE - FpuF32(c.f[21])).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_WEIGHT, 0x0010_3f64)?;
        c.g[A2][0] = 1;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_LOOP, 0x0010_3f74)?;

        // Advance by dt seconds.
        let tick = Addr(r32(c, GP).wrapping_sub(TICK_GP)).read_fpu(c);
        let step = FpuF32(c.f[22]) * tick;
        c.f[20] = step.0;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 0;
        c.f[12] = step.0;
        call_ptr(c, ANIM_SEQ_ADVANCE_TRACK, 0x0010_3f8c)?;

        // Track 1 follows track 0's phase at its own speed.
        let time0 = Addr(seq + SEQ_TIME0).read_fpu(c);
        let length0 = Addr(seq + SEQ_LENGTH0).read_fpu(c);
        let speed1 = Addr(seq + SEQ_SPEED1).read_fpu(c);
        c.f[1] = length0.0;
        c.f[0] = speed1.0;
        c.g[A0][0] = sx(seq);
        c.g[A1][0] = 1;
        c.f[12] = (time0.div(length0) * speed1).0;
        call_ptr(c, ANIM_SEQ_SET_TRACK_TIME, 0x0010_3fac)?;
        c.f[12] = c.f[20];
        c.g[A0][0] = sx(seq);
        call_ptr(c, ANIM_SEQ_UPDATE_WEIGHT_FADE, 0x0010_3fb8)?;
        let faded = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if faded {
            c.cycles += 3;
            c.g[A1][0] = sx(seq);
            call_ptr(c, ANIM_SEQ_LIST_REMOVE, 0x0010_3fd0)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------- cDynamicSprayFX_UpdateEmitter8

pub(crate) mod spray8 {
    /// The rider (its `+0x6c0` sub-object's virtual slot `+0x10` returns
    /// the velocity vector).
    pub const OBJ: u32 = 0x00;
    /// Spray level (fades in and out).
    pub const LEVEL: u32 = 0x10;
    /// Base of the dynamic emitter array (0x210 bytes per slot).
    pub const P28: u32 = 0x28;
    /// Curve table (`+0x818/0x81c/0x820`: velocity, direction and
    /// speed-scaled direction weights).
    pub const P2C: u32 = 0x2c;
    /// Throw direction.
    pub const DIR: u32 = 0x30;
    /// Spawn colour (its w becomes the alpha).
    pub const COL: u32 = 0x90;
    /// Spray position.
    pub const POS: u32 = 0xa0;
    /// Non-zero while the effect is driven.
    pub const ACTIVE: u32 = 0xb0;
    /// Speed (scales the direction term; below a limit the spray stops).
    pub const SPEED: u32 = 0xbc;
    /// Configuration (`+0x5c`: the level's target).
    pub const CFG: u32 = 0xd0;
    /// The emitter slot this update drives (8 * 0x210).
    pub const SLOT: u32 = 0x1080;
}

/// `cDynamicSprayFX_AddRandomVelocity(fx, vec)`.
const SPRAY_ADD_RANDOM_VELOCITY: Callee = Callee {
    addr: 0x002d_e398,
    f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
};
/// `DynamicEmitter_SpawnAt(em, pos, vel, col, spawn, dt)`.
const DYNAMIC_EMITTER_SPAWN_AT: u32 = 0x0037_17c0;

/// The rider sub-object's velocity getter (virtual slot `+0x10` of
/// `rider+0x6c0`), registers as at 0x2e2004 / 0x2e20a4 / 0x2e20d4.
fn spray8_velocity(c: &mut Cpu, fx: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = c.lw(fx);
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x10));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, target, ret)
}

/// `void cDynamicSprayFX_UpdateEmitter8(cDynamicSprayFX* fx)` (0x2e1f70):
/// a spray whose level fades in toward the configured target (`+0.01` per
/// tick) while the effect is driven and the target is at least 0.01, and
/// otherwise fades out (`-1/60` per tick, not below 0). The level drops to
/// 0 at once when, undriven, the rider falls faster than 3333 (velocity z),
/// or when the speed is below a limit. While fading out with a level left,
/// one particle spawns in slot 8 at the position (plus a random jitter),
/// alpha `min(1.25 level, 1)` (scaled down while falling), thrown along
/// the rider's velocity and the direction (weighted by the curve table and
/// the speed). Otherwise slot 8 is only kept alive at the position.
pub fn dynamic_spray_fx_update_emitter8(c: &mut Cpu) {
    use spray8::*;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let fx = r32(c, S0);
        let gp = r32(c, GP);
        c.g[V0][0] = c.lw(fx + P28);
        c.g[V1][0] = c.lw(fx + ACTIVE);
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(SLOT));
        let mut fade_in = false;
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = c.lw(fx + CFG);
            c.lwc1(0, gp.wrapping_sub(0x3a6c)); // 0.01
            c.lwc1(2, r32(c, V0).wrapping_add(0x5c));
            c.c_lt_s(2, 0);
            fade_in = !c.fcc();
        }
        if fade_in {
            // level = min(level + 0.01, target); s1 = 1 (no spawn).
            c.lwc1(0, fx + LEVEL);
            c.cycles += 4;
            c.g[S1][0] = 1;
            c.lwc1(1, gp.wrapping_sub(0x3a64)); // 0.01
            c.add_s(0, 0, 1);
            c.min_s(0, 0, 2);
        } else {
            // level = max(level - 1/60, 0).
            c.cycles += 6;
            c.lwc1(0, fx + LEVEL);
            c.lwc1(2, gp.wrapping_sub(0x3a68)); // 1/60
            c.mtc1(0, 1);
            c.sub_s(0, 0, 2);
            c.max_s(0, 0, 1);
        }

        // Cut the level when falling fast (undriven) or too slow.
        c.cycles += 4;
        c.swc1(0, fx + LEVEL);
        c.g[V0][0] = c.lw(fx + ACTIVE);
        let mut cut = false;
        if c.g[V0][0] == 0 {
            c.cycles += 13;
            spray8_velocity(c, fx, 0x002e_2018)?;
            c.lwc1(1, r32(c, V0).wrapping_add(0x8));
            c.lwc1(0, gp.wrapping_sub(0x3a60)); // -3333.33
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.sw(fx + LEVEL, 0);
                cut = true;
            }
        }
        if !cut {
            c.cycles += 6;
            c.lwc1(1, gp.wrapping_sub(0x3a5c)); // speed limit
            c.lwc1(0, fx + SPEED);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 1;
                c.sw(fx + LEVEL, 0);
                cut = true;
            }
        }
        if cut {
            c.cycles += 1;
        }
        c.lq(2, fx + POS);

        // pos -> sp+0.
        c.cycles += 2;
        c.sq(2, sp(c));
        let mut spawn = false;
        if c.g[S1][0] == 0 {
            c.cycles += 5;
            c.lwc1(1, fx + LEVEL);
            c.mtc1(0, 21);
            c.c_lt_s(21, 1);
            spawn = c.fcc();
        }
        if spawn {
            // alpha = clamp(1.25 level, 0, 1).
            c.cycles += 9;
            c.g[AT][0] = 0x3fa0_0000;
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 22);
            c.mul_s(0, 1, 0);
            c.c_le_s(21, 0);
            let pos = c.fcc();
            c.mov_s(2, 21);
            if pos {
                c.cycles += 1;
                c.min_s(2, 0, 22);
            }
            c.cycles += 13;
            c.mov_s(20, 2);
            spray8_velocity(c, fx, 0x002e_20b4)?;
            c.lwc1(0, r32(c, V0).wrapping_add(0x8));
            c.c_lt_s(0, 21);
            if c.fcc() {
                // Falling: alpha *= clamp((z + 3333.33) / 3333.33, 0, 1).
                c.cycles += 15;
                spray8_velocity(c, fx, 0x002e_20e4)?;
                c.lwc1(1, gp.wrapping_sub(0x3a58)); // 3333.33
                c.lwc1(0, r32(c, V0).wrapping_add(0x8));
                c.add_s(0, 0, 1);
                c.div_s(0, 0, 1);
                c.c_le_s(21, 0);
                let pos = c.fcc();
                c.mov_s(2, 21);
                if pos {
                    c.cycles += 1;
                    c.min_s(2, 0, 22);
                }
                c.cycles += 2;
                c.mul_s(20, 20, 2);
            }
            c.lq(2, fx + COL);

            // col (alpha in w) -> sp+0x10; velocity terms.
            c.cycles += 68;
            c.g[A0][0] = c.lw(fx);
            let sp0 = sp(c);
            c.sq(2, sp0 + 0x10);
            c.swc1(20, sp0 + 0x1c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(0x6c0));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x10));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_2134)?;
            let sp0 = sp(c);
            // velocity * curve[0x818] -> sp+0x40.
            c.g[V1][0] = c.lw(fx + P2C);
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = sx(sp0);
            c.lwc1(0, r32(c, V1).wrapping_add(0x818));
            c.mfc1(A2, 0);
            c.lqc2(4, r32(c, V0));
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x50);
            c.lq(2, sp0 + 0x50);
            c.g[V1][0] = c.lw(fx + P2C);
            c.sq(2, sp0 + 0x40);
            // dir * curve[0x81c] -> sp+0x50; sum -> sp+0x30.
            c.lwc1(0, r32(c, V1).wrapping_add(0x81c));
            c.mfc1(A2, 0);
            c.lqc2(4, fx + DIR);
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x60);
            c.lq(2, sp0 + 0x60);
            c.sq(2, sp0 + 0x50);
            c.lqc2(3, sp0 + 0x40);
            c.lqc2(4, sp0 + 0x50);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0 + 0x60);
            c.lq(2, sp0 + 0x60);
            c.g[V1][0] = c.lw(fx + P2C);
            c.sq(2, sp0 + 0x30);
            // dir * curve[0x820] * speed -> sp+0x60; total -> sp+0x20.
            c.lwc1(0, r32(c, V1).wrapping_add(0x820));
            c.mfc1(A2, 0);
            c.lqc2(4, fx + DIR);
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.lwc1(0, fx + SPEED);
            c.sq(2, sp0 + 0x70);
            c.mfc1(V1, 0);
            c.lqc2(4, sp0 + 0x70);
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.sq(2, sp0 + 0x60);
            c.lqc2(3, sp0 + 0x30);
            c.lqc2(4, sp0 + 0x60);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.sq(2, sp0 + 0x20);
            SPRAY_ADD_RANDOM_VELOCITY.call(c, 0x002e_21fc)?;
            // Spawn one particle.
            let sp0 = sp(c);
            c.lwc1(12, r32(c, GP).wrapping_sub(0x3a54)); // 1/60
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(sp0);
            c.g[A2][0] = sx(sp0 + 0x20);
            c.g[A3][0] = sx(sp0 + 0x10);
            c.g[T0][0] = 1;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002e_2218)?;
        } else {
            // Keep slot 8 alive at the position, no spawn.
            c.cycles += 8;
            c.lwc1(12, r32(c, GP).wrapping_sub(0x3a50)); // 1/60
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(fx + POS);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002e_223c)?;
        }
        frame.restore(c, S0, 0xc0);
        c.cycles += 8;
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------- HaloModifier_DrawQuad

/// `void HaloModifier_DrawQuad(const Vec4* pos, void* tex, int mode, float
/// size, float spin)` (0x2d1d10): draw a camera-facing halo sprite. Unless
/// the world's visibility test (virtual `+0x2d8` of `world+0x10d8`) rejects
/// `pos`, append a 0x14-byte GS packet to the render context's packet
/// (`ctx+0xe84`): the 0x14 bytes before the cursor are copied after it,
/// the GS state template at 0x501420 after that, the header's fields
/// patched (additive alpha blend, primitive 0x1c, ...) and the texture word
/// `ctx[0xf50 + 4 (mode + 0x25)]` stored at +0x10. Then the quad: with no
/// spin (`spin == 0`) one call to the drawer's sprite method (`+0x270`),
/// else its rotated-quad method (`+0x278`) with angle `spin * k`, and for
/// modes 1 and 4 a second, mirrored one (`-spin * k`). The cursor then
/// steps back by 0x14.
pub fn halo_modifier_draw_quad(c: &mut Cpu) {
    /// `gp-0x854`: the world; `gp+0x2a90`: the render context.
    const WORLD_GP: u32 = 0x854;
    const CTX_GP: u32 = 0x2a90;
    /// The render context's packet cursor, drawer and texture words.
    const CTX_PACKET: u32 = 0xe84;
    const CTX_DRAWER: u32 = 0x10d8;
    const CTX_TEXTURES: u32 = 0xf50;
    /// The GS state template copied into each halo packet.
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x120);
        c.g[A3][0] = c.lw(r32(c, GP).wrapping_sub(WORLD_GP));
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x108);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x100);
        c.f[23] = c.f[13];
        frame.save(c, S1, 0xd0);
        c.f[22] = c.f[12];
        frame.save(c, S4, 0xa0);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 24, 0x110);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        // world->drawer->IsCulled(pos) (virtual +0x2d8).
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(CTX_DRAWER));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x2d8));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2dc));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002d_1d78)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xe0);
        } else {
            draw_halo_packet(c)?;
            frame.restore(c, S0, 0xe0);
        }
        c.cycles += 14;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 24, 0x110);
        frame.restore_fpr(c, 23, 0x108);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });

    /// The visible case (0x2d1d80..0x2d2020): build the packet, draw.
    fn draw_halo_packet(c: &mut Cpu) -> crate::call::Flow {
        c.cycles += 98;
        // The quad's colour (tex[0..4] rotated: w, x, y, z) -> sp+0.
        let tex = r32(c, S0);
        let sp0 = sp(c);
        c.lwc1(3, tex.wrapping_add(0x8));
        c.g[A0][0] = 0x50_0000;
        c.lwc1(0, tex.wrapping_add(0xc));
        c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
        c.lwc1(1, tex);
        c.lwc1(2, tex.wrapping_add(0x4));
        c.g[T4][0] = 0x40_0000;
        c.swc1(0, sp0);
        c.g[T5][0] = 0xffff_ffff_ffff_fffc;
        c.swc1(1, sp0 + 0x4);
        c.g[A3][0] = 0xffff_ffff_e000_03ff;
        c.swc1(2, sp0 + 0x8);
        c.swc1(3, sp0 + 0xc);
        c.g[T6][0] = 0xffff_ffff_ffff_ff83;
        let ctx = c.lw(r32(c, GP).wrapping_add(CTX_GP));
        c.g[S1][0] = ctx;
        let ctx = ctx as u32;
        c.g[T7][0] = 0xffff_ffff_ffff_fc1f;
        c.g[S0][0] = 0x80_0000;
        // Repeat the previous 0x14-byte packet after the cursor.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V0][0] = cur;
        let cur = cur as u32;
        c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
        c.g[T1][0] = 0xffff_ffff_ffcf_0000;
        c.g[T3][0] = 0xffff_ffff_fff0_0000;
        c.ldl(V1, cur.wrapping_add(0x7));
        c.ldr(V1, cur);
        c.ldl(A2, cur.wrapping_add(0xf));
        c.ldr(A2, cur.wrapping_add(0x8));
        c.g[T2][0] = c.lw(cur.wrapping_add(0x10));
        c.sdl(V1, cur.wrapping_add(0x1b));
        c.sdr(V1, cur.wrapping_add(0x14));
        c.sdl(A2, cur.wrapping_add(0x23));
        c.sdr(A2, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[T2][0]);
        c.g[T1][0] |= 0xffff;
        c.g[T3][0] |= 0xfff;
        c.g[T2][0] = 0x1_4000;
        // Advance the cursor; copy the state template into the new packet.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V1][0] = cur;
        let cur = cur as u32;
        c.g[A2][0] = sx(r32(c, S3).wrapping_add(0x25));
        c.f[21] = 0;
        c.g[V0][0] = sx(cur.wrapping_add(0x14));
        c.g[A2][0] = sx(r32(c, A2) << 2);
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        c.g[A2][0] = sx(ctx.wrapping_add(r32(c, A2)));
        c.g[S4][0] = sx(STATE_TEMPLATE);
        c.ldl(S5, STATE_TEMPLATE + 0x7);
        c.ldr(S5, STATE_TEMPLATE);
        c.ldl(S6, STATE_TEMPLATE + 0xf);
        c.ldr(S6, STATE_TEMPLATE + 0x8);
        c.g[V0][0] = c.lw(STATE_TEMPLATE + 0x10);
        c.sdl(S5, cur.wrapping_add(0x1b));
        c.sdr(S5, cur.wrapping_add(0x14));
        c.sdl(S6, cur.wrapping_add(0x23));
        c.sdr(S6, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[V0][0]);
        // spin == 0 ?
        c.c_eq_s(23, 21);
        // Patch the header words of the packet.
        let patch = |c: &mut Cpu, p: usize, off: u32, and: usize, or: Option<u64>| {
            c.g[p][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            let at = r32(c, p).wrapping_add(off);
            c.g[V0][0] = c.lw(at);
            c.g[V0][0] &= c.g[and][0];
            if let Some(v) = or {
                c.g[V0][0] |= v;
            }
            c.sw(at, c.g[V0][0]);
        };
        let (t4, s0, t2) = (c.g[T4][0], c.g[S0][0], c.g[T2][0]);
        patch(c, A0, 0x4, A1, Some(t4));
        patch(c, V1, 0x4, T5, None);
        patch(c, A0, 0x8, A3, None);
        patch(c, V1, 0x4, T6, Some(0x1c));
        patch(c, A0, 0x8, T7, Some(0x100));
        patch(c, A1, 0x4, T0, Some(s0));
        patch(c, V1, 0x4, T1, None);
        patch(c, A0, 0x4, T3, Some(t2));
        // The texture word at +0x10.
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(CTX_TEXTURES));
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        let no_spin = c.fcc();
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[V1][0]);

        if no_spin {
            // drawer->Sprite(pos, &{size, size}, &{0, 0}, &{1, 1}, &colour).
            c.cycles += 19;
            let sp0 = sp(c);
            c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
            c.g[A1][0] = c.g[S2][0];
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            c.g[A2][0] = sx(sp0 + 0x10);
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x270));
            c.g[A3][0] = sx(sp0 + 0x20);
            c.swc1(22, sp0 + 0x14);
            c.g[T0][0] = sx(sp0 + 0x60);
            c.swc1(21, sp0 + 0x24);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            c.swc1(0, sp0 + 0x64);
            c.swc1(22, sp0 + 0x10);
            c.swc1(21, sp0 + 0x20);
            c.swc1(0, sp0 + 0x60);
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x274));
            let target = r32(c, V0);
            c.g[T1][0] = c.g[SP][0];
            call_ptr(c, target, 0x002d_2010)?;
        } else {
            // drawer->RotatedSprite(pos, &{size, size}, &{0, 0}, &{1, 1},
            // &colour, 1, spin * k).
            c.cycles += 25;
            let sp0 = sp(c);
            c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
            c.g[S0][0] = sx(sp0 + 0x10);
            c.g[AT][0] = 0x3f80_0000;
            c.f[20] = 0x3f80_0000;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(24, r32(c, GP).wrapping_sub(0x3f98));
            c.g[A3][0] = sx(sp0 + 0x20);
            c.swc1(22, sp0 + 0x10);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            c.swc1(22, sp0 + 0x14);
            c.mul_s(12, 23, 24);
            c.swc1(21, sp0 + 0x20);
            c.g[T0][0] = sx(sp0 + 0x30);
            c.swc1(21, sp0 + 0x24);
            c.g[T1][0] = c.g[SP][0];
            c.swc1(20, sp0 + 0x30);
            c.swc1(20, sp0 + 0x34);
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
            let target = r32(c, V0);
            c.g[T2][0] = 1;
            call_ptr(c, target, 0x002d_1f60)?;
            c.g[V1][0] = 1;
            c.g[V0][0] = 4;
            let mut mirrored = c.g[S3][0] == 1;
            if !mirrored {
                c.cycles += 2;
                mirrored = c.g[S3][0] == 4;
            }
            if mirrored {
                // The mirrored quad (angle -spin * k).
                c.cycles += 21;
                let sp0 = sp(c);
                c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
                c.f[12] = c.f[23] ^ 0x8000_0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
                c.g[A3][0] = sx(sp0 + 0x40);
                c.swc1(22, sp0 + 0x14);
                c.mul_s(12, 12, 24);
                c.swc1(21, sp0 + 0x44);
                c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
                c.swc1(20, sp0 + 0x54);
                c.g[T0][0] = sx(sp0 + 0x50);
                c.swc1(22, sp0 + 0x10);
                c.g[T1][0] = c.g[SP][0];
                c.swc1(21, sp0 + 0x40);
                c.swc1(20, sp0 + 0x50);
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
                let target = r32(c, V0);
                c.g[T2][0] = 1;
                call_ptr(c, target, 0x002d_1fc0)?;
            }
        }
        // Step the cursor back over the drawn packet.
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        Ok(())
    }
}

// ------------------------------------------------- CrowdMan2d_RenderSprites

/// `void CrowdMan2d_RenderSprites(CrowdMan2d* self)` (0x229ba8): draw the
/// 40 2D crowd sprites. Each sprite is a 0x20-byte record from `self+0x2220`
/// (position first, its visible count at +0x10); for each visible one call
/// the drawer's sprite method (virtual `+0x270` of `ctx+0x10d8`) with the
/// position, a 100x100 size, zero UVs, unit UV size and white. Before the
/// first one a GS state packet is appended at the render context's packet
/// cursor (`ctx+0xe84`, as in `HaloModifier_DrawQuad`: previous 0x14 bytes
/// repeated, the 0x501420 template, header patched, texture word
/// `ctx+0x1054` at +0x10); after the last one the cursor steps back.
pub fn crowd_man_2d_render_sprites(c: &mut Cpu) {
    /// `gp+0x2a90`: the render context.
    const CTX_GP: u32 = 0x2a90;
    const CTX_PACKET: u32 = 0xe84;
    const CTX_DRAWER: u32 = 0x10d8;
    const CTX_CROWD_TEXTURE: u32 = 0x1054;
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    /// The sprite records.
    const SPRITES: u32 = 0x2220;
    const SPRITE_VISIBLE: u32 = 0x10;
    const SPRITE_SIZE: u32 = 0x20;
    /// Loop head (0x229c48), for event delivery.
    const HEAD: u32 = 0x0022_9c48;
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0xf0);
        let one = 0x3f80_0000u32;
        let hundred = 0x42c8_0000u32;
        c.f[0] = one;
        c.g[AT][0] = sx(hundred);
        c.f[1] = hundred;
        c.g[T2][0] = 0;
        frame.save(c, S0, 0xe0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        // The header patch masks.
        c.g[S3][0] = 0xffff_ffff_e000_03ff;
        c.g[S4][0] = 0xffff_ffff_fff0_0fff;
        c.g[S5][0] = 0xffff_ffff_ffcf_ffff;
        c.g[S6][0] = 0xffff_ffff_fe7f_ffff;
        c.g[S7][0] = 0xffff_ffff_ffbf_ffff;
        let this = r32(c, A0);
        c.g[S2][0] = sx(this.wrapping_add(SPRITES + SPRITE_VISIBLE));
        c.g[S1][0] = sx(this.wrapping_add(SPRITES));
        let ctx = c.lw(r32(c, GP).wrapping_add(CTX_GP));
        c.g[S0][0] = ctx;
        let ctx = ctx as u32;
        // The sprite arguments: UV (0, 0) at sp+0, UV size (1, 1) at
        // sp+0x10, colour (1, 1, 1, 1) at sp+0x20, size (100, 100) at sp+0x30.
        let sp0 = sp(c);
        c.sw(sp0, 0);
        c.sw(sp0 + 0x4, 0);
        c.g[FP][0] = 0x27;
        for off in [0x10, 0x14, 0x20, 0x24, 0x28, 0x2c] {
            c.sw(sp0 + off, one as u64);
        }
        c.sw(sp0 + 0x30, hundred as u64);
        c.sw(sp0 + 0x34, hundred as u64);

        loop {
            c.cycles += 3;
            let visible = c.lw(r32(c, S2));
            c.g[V0][0] = visible;
            if (visible as i64) > 0 {
                c.cycles += 2;
                if c.g[T2][0] == 0 {
                    c.cycles += 80;
                    c.g[T2][0] = 1;
                    append_state_packet(c, ctx);
                }
                c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
                // drawer->Sprite(pos, &size, &uv, &uv_size, &colour).
                c.cycles += 12;
                let sp0 = sp(c);
                let vt = r32(c, V0);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(sp0 + 0x30);
                c.g[A3][0] = c.g[SP][0];
                c.g[A0][0] = c.lh(vt.wrapping_add(0x270));
                c.g[T0][0] = sx(sp0 + 0x10);
                c.g[V1][0] = c.lw(vt.wrapping_add(0x274));
                c.g[T1][0] = sx(sp0 + 0x20);
                c.sq(T2, sp0 + 0x40);
                let target = r32(c, V1);
                c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0022_9dc4)?;
                c.lq(T2, sp(c) + 0x40);
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(SPRITE_SIZE));
            c.cycles += 3;
            c.g[FP][0] = sx(r32(c, FP).wrapping_sub(1));
            let more = (c.g[FP][0] as i64) >= 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(SPRITE_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 2;
        if c.g[T2][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        }
        frame.restore(c, S0, 0xe0);
        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });

    /// The state packet before the first sprite (0x229c74..0x229d98).
    fn append_state_packet(c: &mut Cpu, ctx: u32) {
        // Repeat the previous 0x14 bytes after the cursor.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V0][0] = cur;
        let cur = cur as u32;
        c.g[A1][0] = 0x50_0000;
        c.g[A2][0] = 0x1_0000;
        c.ldl(V1, cur.wrapping_add(0x7));
        c.ldr(V1, cur);
        c.ldl(A0, cur.wrapping_add(0xf));
        c.ldr(A0, cur.wrapping_add(0x8));
        c.g[A3][0] = c.lw(cur.wrapping_add(0x10));
        c.sdl(V1, cur.wrapping_add(0x1b));
        c.sdr(V1, cur.wrapping_add(0x14));
        c.sdl(A0, cur.wrapping_add(0x23));
        c.sdr(A0, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[A3][0]);
        c.g[A2][0] = 0x1_4000;
        // Advance the cursor; the state template after it.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V1][0] = cur;
        let cur = cur as u32;
        c.g[V0][0] = sx(cur.wrapping_add(0x14));
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        c.g[T0][0] = sx(STATE_TEMPLATE);
        c.ldl(V0, STATE_TEMPLATE + 0x7);
        c.ldr(V0, STATE_TEMPLATE);
        c.ldl(A0, STATE_TEMPLATE + 0xf);
        c.ldr(A0, STATE_TEMPLATE + 0x8);
        c.g[A3][0] = c.lw(STATE_TEMPLATE + 0x10);
        c.sdl(V0, cur.wrapping_add(0x1b));
        c.sdr(V0, cur.wrapping_add(0x14));
        c.sdl(A0, cur.wrapping_add(0x23));
        c.sdr(A0, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[A3][0]);
        c.g[A3][0] = 0x40_0000;
        c.g[T0][0] = 0xffff_ffff_ffff_fffc;
        // The crowd texture word at +0x10.
        c.g[A0][0] = c.lw(ctx.wrapping_add(CTX_CROWD_TEXTURE));
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[A0][0]);
        // Patch the header words: (pointer register, offset, mask, or).
        let patch = |c: &mut Cpu, p: usize, off: u32, mask: u64, or: u64| {
            c.g[p][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            let at = r32(c, p).wrapping_add(off);
            c.g[V0][0] = c.lw(at);
            c.g[V0][0] &= mask;
            c.g[V0][0] |= or;
            c.sw(at, c.g[V0][0]);
        };
        c.g[A0][0] = 0xffff_ffff_ffff_fff3;
        patch(c, V1, 0x0, 0xffff_ffff_ffff_fff3, 0xc);
        let (s7, s6, s5, s4, s3) = (c.g[S7][0], c.g[S6][0], c.g[S5][0], c.g[S4][0], c.g[S3][0]);
        patch(c, A0, 0x4, s7, 0x40_0000);
        patch(c, V1, 0x4, s6, 0);
        patch(c, A0, 0x4, s5, 0);
        patch(c, V1, 0x4, s4, 0x1_4000);
        patch(c, A0, 0x4, 0xffff_ffff_ffff_fffc, 0x1);
        patch(c, V1, 0x8, s3, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_fc1f;
        patch(c, A0, 0x8, 0xffff_ffff_ffff_fc1f, 0x80);
        patch(c, V1, 0x4, 0xffff_ffff_ffff_ff83, 0x1c);
        c.g[A0][0] = 0xffff_ffff_ffff_ff83;
    }
}

// ---------------------------------------------- PS2VisualEffectsMan_Update

/// `PS2VisualEffectsMan_ApplyWorldPainter(self, index)`.
const VFX_APPLY_WORLD_PAINTER: Callee = Callee {
    addr: 0x002f_00a0,
    f: ee::s09::PS2VisualEffectsMan_ApplyWorldPainter,
};

/// `void PS2VisualEffectsMan_Update(PS2VisualEffectsMan* self)` (0x2f0a98):
/// reset the 17 effect objects (`self->objs[0..17]`, virtual slot `+0x10`
/// each), then for every player view `i` (the count at
/// `world->app(+0x84)->game(+0x84)->players(+0x10)`) apply the world
/// painter's parameters for the view's painter index (`gp+0xab8[i]`) and
/// reset the drawer's view slot `i` (virtual `+0x398` of `ctx+0x10d8`,
/// `ctx` at `gp-0x854`).
pub fn ps2_visual_effects_man_update(c: &mut Cpu) {
    /// `gp-0x848`: the world; `gp-0x854`: the render context.
    const WORLD_GP: u32 = 0x848;
    const CTX_GP: u32 = 0x854;
    /// `gp+0xab8`: the painter index of each view.
    const VIEW_PAINTER_GP: u32 = 0xab8;
    /// Loop head (0x2f0c70), for event delivery.
    const HEAD: u32 = 0x002f_0c70;
    /// The number of player views.
    fn view_count(c: &mut Cpu) -> u64 {
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(WORLD_GP));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.lw(r32(c, A0).wrapping_add(0x10))
    }
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        for k in 0..17u32 {
            // objs[k]->Reset() (virtual +0x10).
            let obj = c.lw(this.wrapping_add(4 * k));
            c.g[A1][0] = obj;
            c.g[V0][0] = c.lw(obj as u32);
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
            let target = r32(c, V1);
            c.g[A0][0] = sx((obj as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002f_0acc + 0x18 * k)?;
        }
        let count = view_count(c);
        c.g[V0][0] = count;
        if (count as i64) > 0 {
            c.cycles += 3;
            c.g[S1][0] = sx(r32(c, GP).wrapping_add(VIEW_PAINTER_GP));
            c.g[A1][0] = c.lw(r32(c, S1));
            loop {
                c.cycles += 18;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                VFX_APPLY_WORLD_PAINTER.call(c, 0x002f_0c7c)?;
                // ctx->drawer->ResetView(i) (virtual +0x398).
                let ctx = c.lw(r32(c, GP).wrapping_sub(CTX_GP));
                c.g[A2][0] = ctx;
                c.g[A1][0] = c.g[S2][0];
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = c.lw((ctx as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x398));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x39c));
                let target = r32(c, V1);
                c.g[A0][0] = sx((ctx as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002f_0c9c)?;
                let count = view_count(c);
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    c.cycles += 1;
                    break;
                }
                c.g[A1][0] = c.lw(r32(c, S1));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------- LensFX_UpdateSourceVisibility

/// `void LensFX_UpdateSourceVisibility(LensFX* self)` (0x2e3130): refresh
/// how visible each lens-flare source is. Nothing while `+0x6494` is set.
/// When the occlusion test is on (`gp+0xa28`), each of the `+0x6490`
/// sources (0x210-byte records from `+0x6070`) whose flag at `+0x2e` is set
/// asks the occlusion query (virtual `+0x20` of `+0x649c`) and stores its
/// result, or the source's fallback (`+0x6694`), at `+0x6274`. Then for
/// each source the per-element visibility of its element list (count at
/// `+0x60 + 4 i`, 0x30-byte elements at `+0x6088 + 0x3000 i`): queried
/// elements (flag at +0xe) store the query result, or else copy the value
/// from the source's own element (at `+0x6068 + 0x3000 i`), into their
/// target's `+0x40`.
pub fn lens_fx_update_source_visibility(c: &mut Cpu) {
    /// LensFX fields.
    const BUSY: u32 = 0x6494;
    const SOURCE_COUNT: u32 = 0x6490;
    const OCCLUSION: u32 = 0x649c;
    const SOURCES: u32 = 0x6070;
    const SOURCE_SIZE: u32 = 0x210;
    const SOURCE_FALLBACK: u32 = 0x6694;
    const SOURCE_QUERIED: u32 = 0x609e;
    const SOURCE_VISIBILITY: u32 = 0x6274;
    const ELEMENT_COUNTS: u32 = 0x60;
    const ELEMENT_SIZE: u32 = 0x30;
    /// `gp+0xa28`: occlusion queries enabled.
    const OCCLUSION_ON_GP: u32 = 0xa28;
    /// Loop heads, for event delivery.
    const SOURCE_HEAD: u32 = 0x002e_3190;
    const LIST_HEAD: u32 = 0x002e_3228;
    const ELEMENT_HEAD: u32 = 0x002e_3280;

    /// `occlusion->Query(self, at, &out)` (virtual +0x20) with `out` at
    /// `sp+out_off` zeroed first; `a1` is already the query position.
    fn query(c: &mut Cpu, this: u32, out_off: u32, ret: u32) -> crate::call::Flow {
        c.g[V1][0] = c.lw(this.wrapping_add(OCCLUSION));
        c.sw(sp(c) + out_off, 0);
        c.g[A2][0] = if out_off == 0 {
            c.g[SP][0]
        } else {
            sx(sp(c) + out_off)
        };
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x20));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x24));
        let target = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret)
    }

    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S0, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let this = r32(c, S2);
        c.g[V0][0] = c.lw(this.wrapping_add(BUSY));
        frame.restore(c, S0, 0xb0);
        if c.g[V0][0] == 0 {
            // The sources' own visibility.
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(OCCLUSION_ON_GP));
            c.g[S5][0] = 0;
            c.g[V1][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S4][0] = 0;
                if (c.g[V1][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[S3][0] = sx(this.wrapping_add(SOURCE_FALLBACK));
                    c.g[S1][0] = sx(this.wrapping_add(SOURCES));
                    c.g[S0][0] = c.g[S2][0];
                    loop {
                        c.cycles += 3;
                        let src = r32(c, S0);
                        c.g[V0][0] = c.lhu(src.wrapping_add(SOURCE_QUERIED));
                        c.g[A1][0] = c.g[S1][0];
                        if c.g[V0][0] != 0 {
                            c.cycles += 9;
                            query(c, this, 0, 0x002e_31b8)?;
                            c.lwc1(0, sp(c));
                            if c.g[V0][0] == 0 {
                                c.cycles += 1;
                                c.lwc1(0, r32(c, S3));
                            }
                            c.cycles += 1;
                            c.swc1(0, src.wrapping_add(SOURCE_VISIBILITY));
                        }
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
                        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                        c.g[S3][0] = sx(r32(c, S3).wrapping_add(SOURCE_SIZE));
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(SOURCE_SIZE));
                        c.g[V1][0] = c.g[V0][0];
                        let more = (c.g[S4][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[S0][0] = sx(src.wrapping_add(SOURCE_SIZE));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, SOURCE_HEAD);
                        }
                    }
                }
            }

            // The element lists.
            c.cycles += 2;
            c.g[A0][0] = 0;
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 14;
                let sp0 = sp(c);
                c.g[V0][0] = 0x6000;
                c.g[V1][0] = sx(r32(c, S5) << 3);
                c.g[A1][0] = c.g[V0][0] | 0x88;
                c.sw(sp0 + 0x8, c.g[V1][0]);
                c.mult(0, V0, S5, V0);
                c.sw(sp0 + 0x10, c.g[A1][0]);
                c.g[V1][0] = sx(this.wrapping_add(ELEMENT_COUNTS));
                c.sw(sp0 + 0xc, c.g[V1][0]);
                c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x88));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x68));
                c.sw(sp0 + 0x14, c.g[A1][0]);
                c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
                c.sw(sp0 + 0x18, c.g[V0][0]);
                c.g[V0][0] = sx(r32(c, A0) << 2);
                loop {
                    // List a0: its count, the element pointers.
                    c.cycles += 9;
                    let sp0 = sp(c);
                    c.g[V1][0] = c.lw(sp0 + 0x8);
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw(sp0 + 0xc);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[S4][0] = 0;
                    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[FP][0] = sx(r32(c, A0).wrapping_add(1));
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 13;
                        c.g[V0][0] = 0x3000;
                        c.g[S7][0] = c.g[V1][0];
                        c.mult(0, A0, A0, V0);
                        c.g[V1][0] = c.lw(sp0 + 0x10);
                        c.g[A1][0] = c.lw(sp0 + 0x14);
                        c.g[S5][0] = c.g[S6][0];
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        c.g[S3][0] = sx(r32(c, V0).wrapping_add(this));
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                        c.g[V0][0] = c.lw(sp0 + 0x18);
                        c.g[S0][0] = sx(r32(c, V1).wrapping_add(this));
                        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        loop {
                            c.cycles += 3;
                            let el = r32(c, S0);
                            c.g[V0][0] = c.lhu(el.wrapping_add(0xe));
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] != 0 {
                                c.cycles += 9;
                                query(c, this, 0x4, 0x002e_32a8)?;
                                c.lwc1(0, sp(c) + 0x4);
                                if c.g[V0][0] != 0 {
                                    // The query result.
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(el);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                                    c.swc1(0, r32(c, V0).wrapping_add(0x40));
                                } else {
                                    // The source element's value.
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw(r32(c, S3));
                                    c.g[V1][0] = c.lw(el);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                                    c.lwc1(0, r32(c, V0).wrapping_add(0x40));
                                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
                                    c.swc1(0, r32(c, V1).wrapping_add(0x40));
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lw(r32(c, S7));
                            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                            c.g[S3][0] = sx(r32(c, S3).wrapping_add(ELEMENT_SIZE));
                            c.g[S0][0] = sx(el.wrapping_add(ELEMENT_SIZE));
                            let more = (c.g[S4][0] as i64) < (c.g[V0][0] as i64);
                            c.g[V0][0] = more as u64;
                            c.g[S1][0] = sx(r32(c, S1).wrapping_add(ELEMENT_SIZE));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, ELEMENT_HEAD);
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
                    c.g[A0][0] = c.g[FP][0];
                    let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = sx(r32(c, A0) << 2);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LIST_HEAD);
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------------------------ __fixunssfdi

/// libgcc soft-float helpers called by `__fixunssfdi`.
const GCC_EXTENDSFDF2: Callee = Callee { addr: 0x0041_3af8, f: ee::s15::__extendsfdf2 };
const GCC_CMPDF2: Callee = Callee { addr: 0x0041_2500, f: ee::s15::__cmpdf2 };
const GCC_MULDF3: Callee = Callee { addr: 0x0041_1fd8, f: ee::s15::__muldf3 };
const GCC_FIXUNSDFSI: Callee = Callee { addr: 0x0041_26a0, f: ee::s15::__fixunsdfsi };
const GCC_FLOATDIDF: Callee = Callee { addr: 0x0041_1438, f: ee::s15::__floatdidf };
const GCC_ADDDF3: Callee = Callee { addr: 0x0041_1f18, f: ee::s15::__adddf3 };
const GCC_SUBDF3: Callee = Callee { addr: 0x0041_1f70, f: ee::s15::__subdf3 };

/// `unsigned long long __fixunssfdi(float x)` (0x411340, libgcc): the float
/// widened to a soft double `d`; negative gives 0. Otherwise the high word
/// `hi = (unsigned)(d * 2^-32)`, `d -= (double)(hi << 32)` (converted via
/// `__floatdidf`, halving with the low bit kept when the value is >= 2^63),
/// and the low word added (or, for a negative remainder, subtracted).
pub fn fixunssfdi(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let sp0 = sp(c);
        c.sd(sp0 + 0x10, c.g[S1][0]);
        c.sd(sp0 + 0x30, c.g[RA][0]);
        c.sd(sp0 + 0x20, c.g[S2][0]);
        c.sd(sp0, c.g[S0][0]);
        GCC_EXTENDSFDF2.call(c, 0x0041_1358)?;
        let d = c.g[V0][0];
        c.g[S1][0] = d;
        c.g[A1][0] = 0;
        c.g[A0][0] = d;
        GCC_CMPDF2.call(c, 0x0041_1368)?;
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 0;
        if !negative {
            // hi = (unsigned)(d * 2^-32), as the high word of s0.
            c.cycles += 9;
            c.g[A1][0] = 0xf7c0u64 << 46; // 2^-32
            c.g[A0][0] = d;
            GCC_MULDF3.call(c, 0x0041_1380)?;
            c.g[A0][0] = c.g[V0][0];
            GCC_FIXUNSDFSI.call(c, 0x0041_1388)?;
            let hi = c.g[V0][0] << 32;
            c.g[S0][0] = hi;
            c.g[V0][0] = hi & 1;
            if (hi as i64) >= 0 {
                c.cycles += 4;
                c.g[A0][0] = hi;
                GCC_FLOATDIDF.call(c, 0x0041_139c)?;
            } else {
                // Above 2^63: convert half (keeping the low bit), double it.
                c.cycles += 7;
                c.g[A0][0] = (hi >> 1) | c.g[V0][0];
                GCC_FLOATDIDF.call(c, 0x0041_13b0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[A0][0];
                GCC_ADDDF3.call(c, 0x0041_13bc)?;
            }
            // rest = d - (double)hi.
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 9;
            c.g[A1][0] = c.g[V0][0];
            GCC_SUBDF3.call(c, 0x0041_13c8)?;
            c.g[S2][0] = 0;
            let rest = c.g[V0][0];
            c.g[S1][0] = rest;
            c.g[A1][0] = 0;
            c.g[A0][0] = rest;
            GCC_CMPDF2.call(c, 0x0041_13dc)?;
            let nonneg = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[S2][0];
            if nonneg {
                c.cycles += 5;
                c.g[A0][0] = rest;
                GCC_FIXUNSDFSI.call(c, 0x0041_140c)?;
                let lo = (c.g[V0][0] << 32) >> 32;
                c.g[V0][0] = lo;
                c.g[S0][0] = c.g[S0][0].wrapping_add(lo);
            } else {
                c.cycles += 8;
                c.g[A1][0] = rest;
                GCC_SUBDF3.call(c, 0x0041_13ec)?;
                c.g[A0][0] = c.g[V0][0];
                GCC_FIXUNSDFSI.call(c, 0x0041_13f4)?;
                let lo = (c.g[V0][0] << 32) >> 32;
                c.g[V0][0] = lo;
                c.g[S0][0] = c.g[S0][0].wrapping_sub(lo);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        let sp0 = sp(c);
        c.g[RA][0] = c.ld(sp0 + 0x30);
        c.g[S2][0] = c.ld(sp0 + 0x20);
        c.g[S1][0] = c.ld(sp0 + 0x10);
        c.g[S0][0] = c.ld(sp0);
        frame.close(c);
        Ok(())
    });
}

// --------------------------------------- ComputerActorRider_UpdateSpinPlan

/// `RiderSpinSub_GetFlipProgress(spin, &a, &b)` and
/// `RiderSpinSub_GetSpinProgress(spin, &c, &d)`: the heights still needed
/// to finish the current flip / spin (and with a margin).
const RIDER_SPIN_GET_FLIP_PROGRESS: u32 = 0x0013_5cb0;
const RIDER_SPIN_GET_SPIN_PROGRESS: u32 = 0x0013_5db0;

/// `void ComputerActorRider_UpdateSpinPlan(ComputerActorRider* ai, float
/// height)` (0x10b0e8): drop the AI's planned flip (`+0xe38`) and spin
/// (`+0xe3c`) when there is no longer height for them. Both go when the AI
/// must land (`+0xe20`) and is lower than 1. Otherwise, while planning
/// (`+0xe10`): the flip goes when `height + k0` is below the flip's need,
/// or below its margin while the need is under `k1`; the spin likewise
/// (with `k2`, `k3`), and dropping the spin drops the flip too.
pub fn computer_actor_rider_update_spin_plan(c: &mut Cpu) {
    /// AI fields.
    const RIDER: u32 = 0x18;
    const PLANNING: u32 = 0xe10;
    const MUST_LAND: u32 = 0xe20;
    const FLIP: u32 = 0xe38;
    const SPIN: u32 = 0xe3c;
    /// `rider+0x77c`: the rider state; its spin sub-state at +0x230.
    const RIDER_STATE: u32 = 0x77c;
    const SPIN_SUB: u32 = 0x230;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.f[20] = c.f[12];
        let ai = r32(c, S0);
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_FLIP_PROGRESS, 0x0010_b118)?;
        c.g[V0][0] = c.lw(ai.wrapping_add(RIDER));
        c.g[A1][0] = c.g[SP][0] | 0x8;
        c.g[A2][0] = c.g[SP][0] | 0xc;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RIDER_STATE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SPIN_SUB));
        call_ptr(c, RIDER_SPIN_GET_SPIN_PROGRESS, 0x0010_b130)?;
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(ai.wrapping_add(MUST_LAND));
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(20, 0);
            if c.fcc() {
                c.g[V1][0] = 1;
            }
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            // Too low to finish anything: drop both.
            c.cycles += 3;
            c.sw(ai.wrapping_add(FLIP), 0);
            c.sw(ai.wrapping_add(SPIN), 0);
            c.cycles += 1;
        } else {
            c.g[V0][0] = c.lw(ai.wrapping_add(PLANNING));
            c.cycles += 2;
            if c.g[V0][0] != 0 && plan_checks(c, ai, sp0) {
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });

    /// The flip and spin checks (0x10b170..0x10b23c); true when they end
    /// through the shared exit (0x10b23c, one more cycle).
    fn plan_checks(c: &mut Cpu, ai: u32, sp0: u32) -> bool {
        let gp = r32(c, GP);
        // The flip: needs a (sp+0), b (sp+4).
        c.cycles += 6;
        c.lwc1(0, ai.wrapping_add(FLIP));
        c.mtc1(0, 3);
        c.c_eq_s(0, 3);
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_sub(0x7e20));
            c.lwc1(2, sp0);
            c.add_s(1, 20, 0);
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.swc1(3, ai.wrapping_add(FLIP));
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.lwc1(0, sp0 + 0x4);
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.cycles += 6;
                    c.lwc1(0, gp.wrapping_sub(0x7e1c));
                    c.sub_s(0, 2, 0);
                    c.c_lt_s(0, 3);
                    if c.fcc() {
                        c.cycles += 1;
                        c.swc1(3, ai.wrapping_add(FLIP));
                        c.cycles += 1;
                    }
                }
            }
        }
        c.lwc1(0, ai.wrapping_add(SPIN));

        // The spin: needs c (sp+8), d (sp+0xc).
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        if c.fcc() {
            return false;
        }
        c.cycles += 7;
        c.lwc1(0, gp.wrapping_sub(0x7e18));
        c.lwc1(2, sp0 + 0x8);
        c.add_s(12, 20, 0);
        c.c_lt_s(12, 2);
        if !c.fcc() {
            c.cycles += 5;
            c.lwc1(0, sp0 + 0xc);
            c.c_lt_s(12, 0);
            if !c.fcc() {
                return false;
            }
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x7e14));
            c.sub_s(0, 2, 0);
            c.c_lt_s(0, 1);
            if !c.fcc() {
                return false;
            }
            c.cycles += 1;
        }
        c.swc1(1, ai.wrapping_add(FLIP));
        c.cycles += 1;
        c.swc1(1, ai.wrapping_add(SPIN));
        true
    }
}

// ---------------------------------------- SSXAudioSystem_UpdateSpeechDucking

/// `SSXAudioSystem_GetSpeechMode(self, 0)` (0x29f088).
const AUDIO_GET_SPEECH_MODE: u32 = 0x0029_f088;
/// `SSXAudioSystem_SetDuckGain(self, float gain)`.
const AUDIO_SET_DUCK_GAIN: Callee = Callee {
    addr: 0x0028_9830,
    f: ee::s07::SSXAudioSystem_SetDuckGain,
};

/// `void SSXAudioSystem_UpdateSpeechDucking(SSXAudioSystem* self)`
/// (0x289688): duck the music under speech. While a speech envelope plays
/// (`+0x6c58`), every other tick its next level is looked up (index
/// `tick * rate / stream rate`; past the end the envelope stops) and turned
/// into a target gain (1 below the first threshold, else 1 or 0 by mode);
/// the gain then ramps there (`+0x6c64` per tick) over the following tick.
/// The gain is applied by `SetDuckGain` on the tick the target is set and
/// while ramping (`+0x6c60 <= 0`).
pub fn ssx_audio_system_update_speech_ducking(c: &mut Cpu) {
    /// SSXAudioSystem fields.
    const TICK: u32 = 0x6864;
    const LEVELS: u32 = 0x6868;
    const LEVEL_COUNT: u32 = 0x6c50;
    const LEVEL_RATE: u32 = 0x6c54;
    const PLAYING: u32 = 0x6c58;
    const STEP_STATE: u32 = 0x6c60;
    const GAIN_STEP: u32 = 0x6c64;
    const GAIN: u32 = 0x6c68;
    const TARGET: u32 = 0x6c6c;
    /// `gp+0x2a74`: the speech stream (its rate at +0x2c).
    const STREAM_GP: u32 = 0x2a74;
    const ONE: u32 = 0x3f80_0000;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        let at = |o: u32| this.wrapping_add(o);
        c.g[V0][0] = c.lw(at(PLAYING));
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            let tick = c.lw(at(TICK));
            c.g[A0][0] = tick;
            let t = tick as u32;
            let even = sx((((t.wrapping_add(t >> 31) as i32) >> 1) as u32) << 1);
            c.g[V0][0] = even;
            if tick == even {
                c.cycles += 16;
                c.g[V1][0] = c.lw(at(LEVEL_RATE));
                c.lwc1(0, at(TARGET));
                c.mult(0, V1, A0, V1);
                c.sw(at(STEP_STATE), 0);
                c.swc1(0, at(GAIN));
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(STREAM_GP));
                c.g[V0][0] = c.lw(at(LEVEL_COUNT));
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(1, r32(c, A0).wrapping_add(0x2c));
                c.div_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(V1, 1);
                let in_range = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(this.wrapping_add(r32(c, V1)));
                if !in_range {
                    // Past the end: stop, back to full gain.
                    c.cycles += 4;
                    c.g[AT][0] = ONE as u64;
                    c.f[0] = ONE;
                    c.sw(at(PLAYING), 0);
                    c.swc1(0, at(TARGET));
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = c.lb(r32(c, V0).wrapping_add(LEVELS));
                    c.g[A0][0] = c.g[S0][0];
                    if (c.g[S1][0] as i64) <= 0 {
                        c.cycles += 4;
                        c.g[AT][0] = ONE as u64;
                        c.f[0] = ONE;
                        c.swc1(0, at(TARGET));
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = 0;
                        call_ptr(c, AUDIO_GET_SPEECH_MODE, 0x0028_9718)?;
                        c.g[V1][0] = c.g[V0][0];
                        // Thresholds (v0, a0, v1) and gains (f1 above the
                        // first, f0 above the last).
                        if c.g[V1][0] == 2 {
                            c.cycles += 7;
                            c.g[V0][0] = 0;
                            c.g[A0][0] = 0;
                            c.g[AT][0] = ONE as u64;
                            c.f[1] = ONE;
                            c.f[0] = 0;
                            c.g[V1][0] = 0x1e;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = ONE as u64;
                            c.f[0] = ONE;
                            c.g[V0][0] = 0x1e;
                            c.g[A0][0] = 0x28;
                            c.g[V1][0] = 0x32;
                            c.f[1] = c.f[0];
                        }
                        c.cycles += 3;
                        let level = c.g[S1][0] as i64;
                        let below0 = level < c.g[V0][0] as i64;
                        c.g[V0][0] = (level < c.g[A0][0] as i64) as u64;
                        if below0 {
                            c.cycles += 4;
                            c.g[AT][0] = ONE as u64;
                            c.f[0] = ONE;
                            c.swc1(0, at(TARGET));
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != 0 {
                                c.swc1(1, at(TARGET));
                            } else {
                                c.cycles += 3;
                                let below2 = level < c.g[V1][0] as i64;
                                c.g[V0][0] = below2 as u64;
                                if !below2 {
                                    c.swc1(0, at(TARGET));
                                } else {
                                    c.cycles += 2;
                                    c.swc1(1, at(TARGET));
                                }
                            }
                        }
                    }
                }
                // The ramp step toward the new target.
                c.cycles += 5;
                c.lwc1(0, at(TARGET));
                c.lwc1(1, at(GAIN));
                c.sub_s(0, 0, 1);
                c.swc1(0, at(GAIN_STEP));
            }
            c.g[V0][0] = c.lw(at(TICK));
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(at(TICK), c.g[V0][0]);
        }
        c.g[V0][0] = c.lw(at(STEP_STATE));

        c.cycles += 2;
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                // Ramping: gain += step.
                c.lwc1(0, at(GAIN));
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, at(GAIN_STEP));
                c.add_s(0, 0, 1);
                c.mov_s(12, 0);
                c.swc1(0, at(GAIN));
            } else {
                c.cycles += 3;
                c.lwc1(12, at(TARGET));
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 6;
            AUDIO_SET_DUCK_GAIN.call(c, 0x0028_980c)?;
            c.g[V0][0] = c.lw(at(STEP_STATE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(at(STEP_STATE), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------- cFlagManager_Update

/// `World_GetPeakNumber()` (0x2d1ba0): the current peak, 1-based.
const WORLD_GET_PEAK_NUMBER: u32 = 0x002d_1ba0;
/// `BXrand()`: the next random word.
const BX_RAND: u32 = 0x0031_77f0;
/// `cFlagManager_GroupUpdate(group)`.
const FLAG_MANAGER_GROUP_UPDATE: Callee = Callee {
    addr: 0x0034_b818,
    f: ee::s11::cFlagManager_GroupUpdate,
};

/// `void cFlagManager_Update(cFlagManager* self)` (0x34c668): the wind
/// shared by all flags, then each active flag group. The gust amplitude
/// grows with the peak (0.15, 0.3 on peak 2, 0.45 on peak 3). The gust
/// phase `t` (`+0x1c`) advances by one over a second (`1 / rate`, the
/// rate at `gp+0x2a74 + 0x10`); when it wraps, the base (`+0x14`) takes
/// the old target offset and a new offset (`+0x18`) is drawn in
/// `[-amp, amp]`, clamped so `base + offset` stays in [0, 1]. The wind
/// (`+0x10`) is `base + offset * t`. Then the 15 groups (0x188 bytes from
/// `+0x20`) whose `+0x5c` is set are updated.
pub fn cflag_manager_update(c: &mut Cpu) {
    /// cFlagManager fields.
    const WIND: u32 = 0x10;
    const BASE: u32 = 0x14;
    const OFFSET: u32 = 0x18;
    const PHASE: u32 = 0x1c;
    const GROUPS: u32 = 0x20;
    const GROUP_SIZE: u32 = 0x188;
    const GROUP_ACTIVE: u32 = 0x5c;
    /// `gp+0x2a74`: the timing object (ticks per second at +0x10).
    const TIMING_GP: u32 = 0x2a74;
    /// Loop head (0x34c7b0), for event delivery.
    const HEAD: u32 = 0x0034_c7b0;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x20);
        frame.save_fpr(c, 22, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        call_ptr(c, WORLD_GET_PEAK_NUMBER, 0x0034_c690)?;
        let this = r32(c, S1);
        let gp = r32(c, GP);
        // The gust amplitude by peak (f22).
        c.lwc1(22, gp.wrapping_sub(0x2ae4)); // 0.15
        let peak = c.g[V0][0];
        c.g[V1][0] = peak;
        c.g[V0][0] = ((peak as i64) < 3) as u64;
        if peak == 2 {
            c.cycles += 3;
            c.lwc1(22, gp.wrapping_sub(0x2ae0)); // 0.3
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if peak == 3 {
                    c.cycles += 1;
                    c.lwc1(22, gp.wrapping_sub(0x2adc)); // 0.45
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.g[V0][0] = c.lw(gp.wrapping_add(TIMING_GP));

        // t += 1 / rate.
        c.cycles += 11;
        c.g[AT][0] = 0x3f80_0000;
        c.f[21] = 0x3f80_0000;
        c.lwc1(0, r32(c, V0).wrapping_add(0x10));
        c.cvt_s_w(0, 0);
        c.lwc1(1, this.wrapping_add(PHASE));
        c.div_s(0, 21, 0);
        c.add_s(1, 1, 0);
        c.c_le_s(21, 1);
        let wrapped = c.fcc();
        c.swc1(1, this.wrapping_add(PHASE));
        let mut clamped_high = false;
        if wrapped {
            // t -= 1; base += offset; offset = -amp + 2 amp * rand01.
            c.cycles += 25;
            c.sub_s(2, 1, 21);
            c.lwc1(0, this.wrapping_add(BASE));
            c.lwc1(1, this.wrapping_add(OFFSET));
            c.neg_s(20, 22);
            c.add_s(0, 0, 1);
            c.swc1(2, this.wrapping_add(PHASE));
            c.swc1(0, this.wrapping_add(BASE));
            call_ptr(c, BX_RAND, 0x0034_c71c)?;
            c.g[V1][0] = 0x7f_ffff;
            c.g[A0][0] = 0x3f80_0000;
            c.sub_s(1, 22, 20);
            c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
            c.lwc1(3, this.wrapping_add(BASE));
            c.f[2] = 0;
            c.mtc1(V0, 4);
            c.sub_s(0, 4, 21);
            c.mul_s(1, 1, 0);
            c.add_s(20, 20, 1);
            c.add_s(0, 3, 20);
            c.c_lt_s(0, 2);
            let below = c.fcc();
            c.swc1(20, this.wrapping_add(OFFSET));
            if below {
                c.cycles += 2;
                c.neg_s(0, 3);
                c.swc1(0, this.wrapping_add(OFFSET));
            }
            c.cycles += 7;
            c.lwc1(1, this.wrapping_add(BASE));
            c.lwc1(0, this.wrapping_add(OFFSET));
            c.add_s(0, 1, 0);
            c.c_lt_s(21, 0);
            if c.fcc() {
                c.cycles += 2;
                c.sub_s(0, 21, 1);
                c.swc1(0, this.wrapping_add(OFFSET));
                clamped_high = true;
            }
        }
        if !wrapped || clamped_high {
            c.cycles += 1;
        }
        c.lwc1(0, this.wrapping_add(OFFSET));

        // wind = base + offset * t.
        c.cycles += 8;
        c.g[S0][0] = sx(this.wrapping_add(GROUPS));
        c.lwc1(2, this.wrapping_add(PHASE));
        c.g[S2][0] = 0xe;
        c.lwc1(1, this.wrapping_add(BASE));
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 0);
        c.swc1(1, this.wrapping_add(WIND));
        loop {
            c.cycles += 3;
            let group = r32(c, S0);
            c.g[V0][0] = c.lw(group.wrapping_add(GROUP_ACTIVE));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                FLAG_MANAGER_GROUP_UPDATE.call(c, 0x0034_c7c4)?;
            }
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(group.wrapping_add(GROUP_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------- RiderAnim_BlendSeq4

    /// `RiderAnim_BlendSeq4`: a sequence whose track 0 sometimes already
    /// plays the `from` key, `v` in each of the four key intervals (and on
    /// their edges), and a weight fade that sometimes ends (list removal).
    /// The real track setters, advance, fade and list removal run.
    #[test]
    fn rider_anim_blend_seq4_matches() {
        use crate::difftest::fakes::{machine, ptr};
        use crate::difftest::{check, elf_image, GAME_GP};
        check(ee::s00::f_00103e28__generated, rider_anim_blend_seq4, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0010_3e28, 0);
            elf_image(c);
            c.pc = 0x0010_3e28;
            // The animation bank: ids 0x100 + key at +0x1030.
            let bank = ptr(&mut r);
            c.bus.write32(GAME_GP + 0xd8c, bank);
            for k in 0..8 {
                c.bus.write32(bank + 0x1030 + 4 * k, 0x100 + k * 0x101);
            }
            let keys: Vec<u32> = (0..5).map(|_| r.next_u32() % 8).collect();
            for (reg, k) in [A3, T0, T1, T2, T3].into_iter().zip(&keys) {
                c.g[reg][0] = *k as u64;
            }
            let seq = ptr(&mut r);
            let list = ptr(&mut r);
            c.g[A0][0] = sx(ptr(&mut r));
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(seq);
            for off in [0x8, 0xc, 0x10, 0x2c, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                let v = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.edge_f32(),
                    _ => r.f32(3.0),
                };
                c.bus.write32(seq + off, v);
            }
            // Track 0 already plays one of the keys (or something else).
            let cur = if seed % 3 == 0 {
                r.next_u32()
            } else {
                0x100 + keys[(r.next_u32() % 5) as usize] * 0x101
            };
            c.bus.write32(seq + 0x4, cur);
            c.bus.write32(seq + 0x1c, r.next_u32() & 1);
            c.bus.write32(seq + 0xa0, r.next_u32() & 1);
            let mut at = list + 4;
            for _ in 0..seed % 3 {
                let other = ptr(&mut r) + 0x8;
                c.bus.write32(at, other);
                at = other + 200;
            }
            let vs = [-1.0f32, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
            c.f[13] = match seed % 11 {
                9 => r.edge_f32(),
                10 => r.f32(1.0),
                i => vs[i as usize].to_bits(),
            };
            c.f[12] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(3.0) };
        });
    }

    // ------------------------------------------- cDynamicSprayFX_UpdateEmitter8

    /// Emitter 8: driven or not, the target above or below 0.01, the
    /// fall-speed and speed cuts, the level's clamps and the falling alpha
    /// scale; the velocity getter is a fake returning a chosen vector, the
    /// random jitter and `DynamicEmitter_SpawnAt` on slot 8 are real.
    #[test]
    fn dynamic_spray_fx_update_emitter8_matches() {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::put_spray_slot;
        use spray8::*;
        let f = |x: f32| x.to_bits();
        check(
            ee::s09::f_002e1f70__generated,
            dynamic_spray_fx_update_emitter8,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e1f, 3);
                elf_image(c);
                c.pc = 0x002e_1f70;
                let (fx, obj, cfg, em, vt, curve, vel) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x5200,
                    DATA + 0x3000,
                    DATA + 0x5400,
                    DATA + 0x5600,
                    DATA + 0x5800,
                );
                let (world, g, players) = (DATA + 0x6000, DATA + 0x6200, DATA + 0x6400);
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + CFG, cfg);
                for k in 0..4u32 {
                    c.bus.write32(fx + DIR + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + POS + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + 0x40 + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + 0x50 + 4 * k, r.f32(1.0));
                    c.bus.write32(vel + 4 * k, r.f32(100.0));
                }
                c.bus.write32(
                    vel + 8,
                    f([-5000.0f32, -1000.0, 50.0, -3333.0][(seed % 4) as usize]),
                );
                c.bus.write32(fx + ACTIVE, (seed % 3 != 0) as u32);
                c.bus.write32(
                    cfg + 0x5c,
                    f([0.005f32, 0.5, 2.0][((seed / 3) % 3) as usize]),
                );
                c.bus.write32(
                    fx + LEVEL,
                    match (seed / 9) % 5 {
                        0 => 0,
                        1 => f(0.01),
                        2 => f(0.5),
                        3 => f(1.5),
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(
                    fx + SPEED,
                    f([100.0f32, 300.0, 2000.0][((seed / 5) % 3) as usize]),
                );
                c.bus.write32(fx + 0x12c, r.f32(1.0));
                for off in [0x818u32, 0x81c, 0x820] {
                    c.bus.write32(curve + off, r.f32(3.0));
                }
                c.bus.write32(obj + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                for n in 0..4 {
                    c.bus.write32(LOG_RESULTS + 4 * n, vel);
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
                c.g[A0][0] = sx(fx);
            },
        );
    }

    // ---------------------------------------------------- HaloModifier_DrawQuad

    /// `HaloModifier_DrawQuad`: culled or visible (a real `return 0` method
    /// or a fake), no spin / spin, modes 0..5 (1 and 4 draw the mirrored
    /// quad), a packet cursor at various alignments (the unaligned copies).
    /// The drawer's methods are fakes that log their arguments.
    #[test]
    fn halo_modifier_draw_quad_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(ee::s09::f_002d1d10__generated, halo_modifier_draw_quad, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2d1d, 3);
            c.pc = 0x002d_1d10;
            let (world, wvt, ctx, cvt, tex, pos) = (
                DATA + 0x10000,
                DATA + 0x14000,
                DATA + 0x18000,
                DATA + 0x1c000,
                DATA + 0x100,
                DATA + 0x200,
            );
            let cur = DATA + 0x2000 + 4 * (seed % 4);
            c.bus.write32(GAME_GP.wrapping_sub(0x854), world);
            c.bus.write32(world + 0x10d8, wvt);
            c.bus.write16(wvt + 0x2d8, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(
                wvt + 0x2dc,
                if seed % 5 == 0 { FAKE } else { RETURN_ZERO },
            );
            c.bus.write32(GAME_GP + 0x2a90, ctx);
            fill(c, &mut r, ctx + 0xf50, 0x30);
            c.bus.write32(ctx + 0xe84, cur);
            c.bus.write32(ctx + 0x10d8, cvt);
            for (k, off) in [0x270u32, 0x278].into_iter().enumerate() {
                c.bus.write16(cvt + off, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(cvt + off + 4, FAKE + 0x10 * (k as u32 + 1));
            }
            fill(c, &mut r, cur - 0x20, 0x20);
            fill(c, &mut r, 0x0050_1420, 6);
            fill(c, &mut r, tex, 4);
            fill(c, &mut r, pos, 4);
            c.bus.write32(GAME_GP.wrapping_sub(0x3f98), r.f32(1.0));
            c.g[A0][0] = sx(pos);
            c.g[A1][0] = sx(tex);
            c.g[A2][0] = sx((seed / 2) % 6);
            c.f[12] = r.f32(50.0);
            c.f[13] = if seed % 3 == 0 { 0 } else { r.f32(6.0) };
        });
    }

    // ------------------------------------------------- CrowdMan2d_RenderSprites

    /// `CrowdMan2d_RenderSprites`: 40 sprite records with random visible
    /// counts (none visible for some seeds: no packet), the drawer's sprite
    /// method a fake (one of the calls may unwind), the packet cursor at
    /// various alignments.
    #[test]
    fn crowd_man_2d_render_sprites_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        check(ee::s05::f_00229ba8__generated, crowd_man_2d_render_sprites, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x229b, 40);
            c.pc = 0x0022_9ba8;
            let (this, ctx, vt) = (DATA, DATA + 0x10000, DATA + 0x14000);
            let cur = DATA + 0x8000 + 4 * (seed % 4);
            for i in 0..40u32 {
                let rec = this + 0x2220 + 0x20 * i;
                fill(c, &mut r, rec, 4);
                let visible = match (seed % 4, r.next_u32() % 3) {
                    (0, _) => 0,
                    (_, 0) => 0,
                    (_, 1) => r.next_u32(),
                    _ => 1,
                };
                c.bus.write32(rec + 0x10, visible);
            }
            c.bus.write32(GAME_GP + 0x2a90, ctx);
            c.bus.write32(ctx + 0xe84, cur);
            c.bus.write32(ctx + 0x1054, r.next_u32());
            c.bus.write32(ctx + 0x10d8, vt);
            c.bus.write16(vt + 0x270, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x274, FAKE);
            fill(c, &mut r, cur - 0x20, 0x20);
            fill(c, &mut r, 0x0050_1420, 6);
            c.g[A0][0] = sx(this);
        });
    }

    // ---------------------------------------------- PS2VisualEffectsMan_Update

    /// `PS2VisualEffectsMan_Update`: the 17 resets and the drawer's view
    /// resets are fakes (one may unwind); 0..3 views, each through the real
    /// `PS2VisualEffectsMan_ApplyWorldPainter` (its fixture from
    /// `renderstateman`'s test: painter descriptors whose methods are
    /// `MenuSys_IsActive`).
    #[test]
    fn ps2_visual_effects_man_update_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        check(ee::s09::f_002f0a98__generated, ps2_visual_effects_man_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2f0a, 20);
            c.pc = 0x002f_0a98;
            let g = GAME_GP;
            let (vt, ctx, players) = (DATA + 0x6000, DATA + 0x7000, DATA + 0x2300);
            // The 17 effect objects share one vtable whose Reset is a fake.
            for k in 0..17u32 {
                let obj = DATA + 0x5000 + 0x40 * k;
                c.bus.write32(DATA + 4 * k, obj);
                c.bus.write32(obj, vt);
            }
            c.bus.write16(vt + 0x10, (r.next_u32() & 0x3c) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            c.bus.write32(g - 0x854, ctx);
            c.bus.write32(ctx + 0x10d8, vt);
            c.bus.write16(vt + 0x398, (r.next_u32() & 0x3c) as u16);
            c.bus.write32(vt + 0x39c, FAKE + 0x10);
            // world -> app (+0x84) -> players (+0x84; the view count at
            // +0x10) and game module (+0x28) -> handle.
            c.bus.write32(g - 0x848, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x2100);
            c.bus.write32(DATA + 0x2100 + 0x84, players);
            c.bus.write32(
                players + 0x10,
                match seed % 5 {
                    4 => (-(r.next_u32() as i32 % 3).abs()) as u32,
                    k => k,
                },
            );
            for i in 0..4u32 {
                c.bus.write32(g + 0xab8 + 4 * i, r.next_u32() % 4);
            }
            c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
            c.bus.write32(DATA + 0x2200, r.next_u32() & 1);
            c.bus.write32(g + 0x2a90, DATA + 0x4000);
            c.bus.write32(g + 0xa70, 0);
            for off in (0..0x40u32).step_by(4) {
                c.bus.write32(0x004f_a370 + off, DATA + 0x1000);
            }
            c.bus.write32(0x004f_a370 + 0x24, r.edge_f32());
            c.bus.write32(DATA + 0x1000, DATA + 0x1100);
            c.bus.write32(DATA + 0x1104, DATA + 0x1200);
            c.bus.write32(DATA + 0x1100, 0);
            for slot in (0x30..0x1b8u32).step_by(8) {
                c.bus.write16(DATA + 0x1200 + slot, 0);
                c.bus.write32(DATA + 0x1200 + slot + 4, 0x002c_c098);
            }
            c.bus.write32(g + 0x1238, (seed % 3 == 0) as u32);
            c.bus.write32(g + 0x14ec, (seed % 7 == 0) as u32);
            c.bus.write32(g + 0x12dc, (seed % 4 == 1) as u32);
            c.bus.write32(g + 0xa6c, r.next_u32());
            for off in [
                0x1244u32, 0x1248, 0x123c, 0x1240, 0x151c, 0x1524, 0x12e4, 0x12e8, 0x12ec,
                0x12f0, 0x12f4, 0x1300, 0x1304, 0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508,
            ] {
                c.bus.write32(g + off, r.next_u32());
            }
            for off in (0..0x260u32).step_by(4) {
                c.bus.write32(DATA + 0x4000 + off, r.next_u32());
            }
            c.g[A0][0] = sx(DATA);
        });
    }

    // ------------------------------------------- LensFX_UpdateSourceVisibility

    /// `LensFX_UpdateSourceVisibility`: busy or not, occlusion on or off,
    /// 0..2 sources with queried / unqueried flags, element lists of -1..3
    /// elements; the occlusion query is a fake (a pointer, maybe unwinding)
    /// or a real `return 0` (the fallback paths).
    #[test]
    fn lens_fx_update_source_visibility_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(
            ee::s09::f_002e3130__generated,
            lens_fx_update_source_visibility,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e31, 8);
                c.pc = 0x002e_3130;
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
            },
        );
    }

    // ------------------------------------------------------------ __fixunssfdi

    /// `__fixunssfdi` through the real soft-float helpers: negative, zero,
    /// small, fractional, above 2^32 and above 2^63 inputs, and edge bit
    /// patterns (denormals, exponent 255).
    #[test]
    fn fixunssfdi_matches() {
        use crate::difftest::{check, scramble, Rng};
        check(ee::s15::f_00411340__generated, fixunssfdi, 300, &|c, seed| {
            let mut r = Rng(0x0041_1340 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.pc = 0x0041_1340;
            let x: u32 = match seed % 8 {
                0 => r.edge_f32(),
                1 => r.f32(1000.0),
                2 => ((r.next_u32() % 1_000_000) as f32 + 0.75).to_bits(),
                3 => (4.3e9f32 * (1.0 + (r.next_u32() % 1000) as f32)).to_bits(),
                4 => (9.3e18f32 + (r.next_u32() % 1000) as f32 * 1e16).to_bits(),
                5 => (1.7e19f32).to_bits(),
                6 => ((r.next_u32() % 64) as f32).to_bits(),
                _ => r.next_u32(),
            };
            c.f[12] = x;
        });
    }

    // --------------------------------------- ComputerActorRider_UpdateSpinPlan

    /// `ComputerActorRider_UpdateSpinPlan`: the real flip/spin progress
    /// queries (their attribute factor through a fake BE interface, which
    /// may unwind), must-land on/off, planning on/off, planned flip/spin
    /// zero or not, heights and `.lit4` margins chosen to reach each drop.
    #[test]
    fn computer_actor_rider_update_spin_plan_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        const AI: u32 = DATA + 0x9_0000;
        const RIDER: u32 = DATA + 0x9_1000;
        const STATE: u32 = DATA + 0x9_3000;
        const MODEL: u32 = DATA + 0x9_4000;
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(
            ee::s00::f_0010b0e8__generated,
            computer_actor_rider_update_spin_plan,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_b0e8, 2);
                c.pc = 0x0010_b0e8;
                let pick = |r: &mut crate::difftest::Rng, v: &[u32]| {
                    v[(r.next_u32() as usize) % v.len()]
                };
                let f = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
                // The progress queries' own `.lit4` constants.
                for off in [0x7330u32, 0x732c, 0x7328, 0x7324, 0x7320, 0x731c] {
                    c.bus.write32(GAME_GP - off, f(0.5));
                }
                // The plan's margins.
                for off in [0x7e14u32, 0x7e18, 0x7e1c, 0x7e20] {
                    c.bus.write32(
                        GAME_GP - off,
                        pick(&mut r, &[f(0.0), f(0.5), f(2.0), f(-1.0)]),
                    );
                }
                c.bus.write32(AI + 0x18, RIDER);
                c.bus.write32(RIDER + 0x77c, STATE);
                let x = STATE + 0x230;
                for off in [0x10u32, 0x14, 0x18, 0x1c, 0x38, 0x3c] {
                    c.bus.write32(x + off, r.f32(4.0));
                }
                c.bus.write32(x + 0x58, MODEL);
                c.bus.write32(MODEL + 0x2ec, r.f32(2.0));
                c.bus.write32(MODEL + 0x86c, r.next_u32() % 4);
                c.bus.write32(MODEL + 0xb34, 1 + r.next_u32() % 3);
                c.bus.write32(AI + 0xe20, pick(&mut r, &[0, 0, 1]));
                c.bus.write32(AI + 0xe10, pick(&mut r, &[0, 1, 1, 1]));
                c.bus.write32(AI + 0xe38, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.bus.write32(AI + 0xe3c, pick(&mut r, &[0, f(1.0), f(1.0)]));
                c.f[12] = pick(&mut r, &[f(0.5), f(2.0), f(-3.0), f(10.0)]);
                c.g[A0][0] = sx(AI);
            },
        );
    }

    // ---------------------------------------- SSXAudioSystem_UpdateSpeechDucking

    /// `SSXAudioSystem_UpdateSpeechDucking`: playing or not, even and odd
    /// ticks, envelope indices in and past the range, levels <= 0 and across
    /// the thresholds of speech modes 2, 3 and others, ramp states -1/0/1;
    /// the real speech-mode getter, `SetDuckGain` and `SetFadeGain` run.
    #[test]
    fn ssx_audio_system_update_speech_ducking_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s07::f_00289688__generated,
            ssx_audio_system_update_speech_ducking,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9688, 0);
                c.pc = 0x0028_9688;
                let (this, stream) = (DATA + 0x100, DATA + 0x9000);
                let f = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP.wrapping_sub(0x4620), 0x3c88_8893);
                c.bus.write32(GAME_GP + 0x2a74, stream);
                c.bus.write32(
                    stream + 0x2c,
                    if seed % 9 == 0 { r.edge_f32() } else { f(1.0 + (r.next_u32() % 4) as f32) },
                );
                c.bus.write32(this + 0x6c58, (seed % 4 != 0) as u32);
                c.bus.write32(this + 0x6864, r.next_u32() % 20);
                c.bus.write32(this + 0x6c54, 1 + r.next_u32() % 4);
                c.bus.write32(this + 0x6c50, r.next_u32() % 100);
                for i in 0..0x60u32 {
                    let v = (r.next_u32() % 90) as i32 - 10;
                    c.bus.write8(this + 0x6868 + i, v as u8);
                }
                c.bus.write32(this + 0x5738, [2u32, 3, 7][(seed % 3) as usize]);
                c.bus.write32(
                    this + 0x6c60,
                    [0xffff_ffffu32, 0, 1, 0xffff_fffe][((seed / 3) % 4) as usize],
                );
                for off in [0x6c64u32, 0x6c68, 0x6c6c] {
                    c.bus.write32(this + off, r.f32(1.0));
                }
                let cur = if seed % 2 == 0 { c.bus.read32(this + 0x6c6c) } else { r.f32(1.0) };
                c.bus.write32(this + 0x6c5c, cur);
                for off in [0x63c4u32, 0x63c8, 0x63cc, 0x63d0] {
                    c.bus.write32(this + off, r.f32(1.0));
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ---------------------------------------------------- cFlagManager_Update

    /// `cFlagManager_Update`: peaks 1..4 (the real `World_GetPeakNumber`
    /// through a BE library dummy whose interface is a fake), the gust phase
    /// wrapping or not, new offsets clamped low and high (real `BXrand`),
    /// and no or one active group (the real `cFlagManager_GroupUpdate` on
    /// its own test's fixture, a fake mesh method).
    #[test]
    fn cflag_manager_update_matches() {
        use crate::difftest::fakes::{machine, vtable, FAKE};
        use crate::difftest::{check, DATA, GAME_GP};
        const BE_LIBRARY: u32 = DATA + 0x8_6000;
        const BE_IFACE: u32 = DATA + 0x8_7000;
        const BE_VTABLE: u32 = DATA + 0x8_7100;
        check(ee::s11::f_0034c668__generated, cflag_manager_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_c668, 3);
            c.pc = 0x0034_c668;
            let f = |x: f32| x.to_bits();
            let this = DATA + 0x1_0000;
            // The peak: BE library dummy, interface 0 a fake, the track
            // table's peak word (0-based) for track 0.
            c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, BE_IFACE);
            c.bus.write32(BE_IFACE + 4, BE_VTABLE);
            c.bus.write16(BE_VTABLE + 8, 0);
            c.bus.write32(BE_VTABLE + 0xc, FAKE + 0x80);
            c.bus.write32(0x0053_5c08, 0);
            c.bus.write32(0x0043_d950 + 0x54, seed % 4);
            for (off, v) in [(0x2ae4u32, 0.15f32), (0x2ae0, 0.3), (0x2adc, 0.45)] {
                c.bus.write32(GAME_GP - off, f(v));
            }
            let timing = DATA + 0x9000;
            c.bus.write32(GAME_GP + 0x2a74, timing);
            c.bus.write32(timing + 0x10, [60u32, 30, 1, 2][((seed / 4) % 4) as usize]);
            c.bus.write32(GAME_GP + 0x0a0c, r.next_u32());
            c.bus.write32(this + 0x1c, f([0.5f32, 0.99, 0.0, 0.7][((seed / 16) % 4) as usize]));
            c.bus.write32(this + 0x14, f([0.0f32, 0.5, 1.0, 0.95][((seed / 2) % 4) as usize]));
            c.bus.write32(this + 0x18, r.f32(0.5));
            c.bus.write32(this + 0x10, r.f32(1.0));
            for i in 0..15u32 {
                c.bus.write32(this + 0x20 + 0x188 * i + 0x5c, 0);
            }
            if seed % 3 != 0 {
                // One active group (cFlagManager_GroupUpdate's fixture).
                let g = this + 0x20 + 0x188 * (r.next_u32() % 15);
                let (world, w84, mesh) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x2000);
                c.bus.write32(GAME_GP.wrapping_add(0x1150), 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, w84);
                c.bus.write32(w84 + 0x70, DATA + 0x4200);
                c.bus.write32(w84 + 0xc, DATA + 0x4300);
                let frame = r.next_u32() % 8;
                c.bus.write32(DATA + 0x4308, frame);
                c.bus.write32(g + 0x5c, 1);
                c.bus.write32(g + 0x60, if seed % 5 == 0 { 0 } else { mesh });
                c.bus.write32(g + 0x90, (frame + seed) & 1);
                c.bus.write32(g + 0x4c, r.edge_f32());
                c.bus.write32(DATA + 0x4200 + 0x10, r.edge_f32());
                for i in 0..4 {
                    c.bus.write32(g + 4 + 4 * i, r.edge_f32());
                    c.bus.write32(g + 0x78 + 4 * i, r.edge_f32());
                }
                c.bus.write32(g + 0x50, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                c.bus.write32(g + 0x54, if seed % 4 == 1 { 0 } else { r.f32(1.0) });
                c.bus.write32(g + 0x88, r.f32(1.0));
                c.bus.write32(g + 0x8c, r.f32(1.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x2b00), 0x3a83_126f); // 0.001
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mesh + 4, vt);
            }
            c.g[A0][0] = sx(this);
        });
    }
}
