//! Staging wave 205 (oc240): hand-written replacements for the hottest
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
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_ef30, be_get_rider_char_db_weight, "BE_GetRiderCharDBWeight"),
    (0x0012_5b18, player_apply_rumble, "Player_ApplyRumble"),
    (0x0037_1380, dynamic_emitter_render, "DynamicEmitter_Render"),
    (0x002e_a538, cboard_trail_fx_render_batch, "cBoardTrailFX_RenderBatch"),
    (0x002e_a900, ccut_scene_transition_man_update, "cCutSceneTransitionMan_Update"),
    (
        0x0010_bbf8,
        computer_actor_rider_find_board_press_marker,
        "ComputerActorRider_FindBoardPressMarker",
    ),
    (0x0037_4298, bezier_patch_cache_tess_uvs, "BezierPatchCache_TessUVs"),
    (0x002b_5d78, world_trigger_manager_update, "WorldTriggerManager_Update"),
    (0x003e_4db8, ea_timer_rcnt1_handler, "EATimer_Rcnt1Handler"),
    (0x0026_a428, path_get_closest_point, "Path_GetClosestPoint"),
];

// ---------------------------------------------------------------------------
// BE_GetRiderCharDBWeight (0x14ef30)

/// `int8_t BE_GetRiderCharacter(int player)` (0x14a080).
const BE_GET_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};

/// The character database: 0x88-byte records (`cBECharacterInterface`).
const CHAR_DB: u32 = 0x0053_0970;
const CHAR_DB_RECORD: u32 = 0x88;
/// Character record: the rider's weight class.
const CHAR_WEIGHT: u32 = 0x40;

/// `int BE_GetRiderCharDBWeight(void*, int riderIndex)` (0x14ef30,
/// `cBECharacterInterface::getWeight`): the weight word of the rider's
/// character-DB record. The record base is kept in s0 across the call.
pub fn be_get_rider_char_db_weight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(CHAR_DB);
        BE_GET_RIDER_CHARACTER.call(c, 0x0014_ef4c)?;
        c.g[V1][0] = CHAR_DB_RECORD as u64;
        frame.restore_ra(c, 0);
        let character = c.g[V0][0] as u32;
        let off = abi::mult(c, character, CHAR_DB_RECORD);
        c.g[V0][0] = sx(off);
        let record = CHAR_DB.wrapping_add(off);
        c.g[S0][0] = sx(record);
        c.g[V0][0] = Addr(record).off(CHAR_WEIGHT).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Player_ApplyRumble (0x125b18)

/// Rumble state: `float level[2]` (heavy motor, light motor).
const RUMBLE_HEAVY: u32 = 0x0;
const RUMBLE_LIGHT: u32 = 0x4;
/// `.lit4` constants (gp-relative): heavy decay factor and offset, light
/// decay factor, heavy and light scales of the two motor outputs.
const LIT_HEAVY_DECAY: u32 = 0x7860;
const LIT_HEAVY_DROP: u32 = 0x785c;
const LIT_LIGHT_DECAY: u32 = 0x7858;
const LIT_M0_HEAVY_DIV: u32 = 0x7854;
const LIT_M0_LIGHT_SCALE: u32 = 0x7850;
const LIT_M1_HEAVY_DIV: u32 = 0x784c;
const LIT_M1_LIGHT_SCALE: u32 = 0x7848;
/// Pad vtable slots: `{int16 delta, fn}` pairs.
const VT_STATUS: u32 = 0x18;
const VT_SET_MOTOR: u32 = 0x20;
/// The pad's "stop motors" helper (0x326cf0).
const PAD_STOP_MOTORS: u32 = 0x0032_6cf0;

fn gp_lit(c: &mut Cpu, neg: u32) -> FpuF32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(neg)).read_fpu(c)
}

/// Call the pad's `vt[slot]` method on `this = s1 + delta` (registers set
/// as the original: v0/v1 holds the function, a0 the adjusted `this`).
fn pad_vt_load(c: &mut Cpu, slot: u32, fn_reg: usize, vt_reg: usize) -> u32 {
    let pad = c.g[S1][0] as u32;
    c.g[vt_reg][0] = Addr(pad).lw(c);
    let vt = c.g[vt_reg][0] as u32;
    c.g[A0][0] = Addr(vt).off(slot).lh(c);
    c.g[fn_reg][0] = Addr(vt).off(slot + 4).lw(c);
    c.g[A0][0] = sx(pad.wrapping_add(c.g[A0][0] as u32));
    c.g[fn_reg][0] as u32
}

/// One motor output: `max(max((light - light_off) * light_scale,
/// (heavy - heavy_off) / heavy_div), 0)` into f12, then `vt[4](pad, f12,
/// motor)`.
#[allow(clippy::too_many_arguments)]
fn set_motor(
    c: &mut Cpu,
    state: Addr,
    motor: u64,
    heavy_off: u32,
    light_off: u32,
    lit_div: u32,
    lit_scale: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[A1][0] = motor;
    c.g[AT][0] = heavy_off as u64;
    let heavy = state.off(RUMBLE_HEAVY).read_fpu(c);
    let div = gp_lit(c, lit_div);
    let heavy_term = (heavy - FpuF32(heavy_off)).div(div);
    let light = state.off(RUMBLE_LIGHT).read_fpu(c);
    c.g[AT][0] = light_off as u64;
    let scale = gp_lit(c, lit_scale);
    let light_term = (light - FpuF32(light_off)) * scale;
    let target = pad_vt_load(c, VT_SET_MOTOR, V1, V0);
    let zero = FpuF32(c.f[20]);
    let level = light_term.max(heavy_term).max(zero);
    c.f[0] = light_off;
    c.f[1] = heavy_term.0;
    c.f[2] = div.0;
    c.f[3] = scale.0;
    c.f[12] = level.0;
    call_ptr(c, target, ret)
}

/// `void Player_ApplyRumble(float* state, Pad* pad, int on)` (0x125b18):
/// the per-frame controller rumble. Off: zero both levels and stop the
/// motors. On: decay the heavy level (`x * k - d`, floored at 0) and the
/// light level (`x * k`); if the pad reports status 2 (vibration capable),
/// drive motor 0 and motor 1 from the two levels.
pub fn player_apply_rumble(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let state = Addr(c.g[S0][0] as u32);
        if c.g[A2][0] == 0 {
            // Rumble off: clear both levels, stop the motors.
            c.cycles += 7;
            c.f[0] = 0;
            c.g[A0][0] = c.g[S1][0];
            state.off(RUMBLE_HEAVY).write_fpu(c, FpuF32::ZERO);
            state.off(RUMBLE_LIGHT).write_fpu(c, FpuF32::ZERO);
            call_ptr(c, PAD_STOP_MOTORS, 0x0012_5b4c)?;
        } else {
            // Decay the heavy level, floored at zero.
            c.cycles += 10;
            let heavy = state.off(RUMBLE_HEAVY).read_fpu(c);
            let k = gp_lit(c, LIT_HEAVY_DECAY);
            let drop = gp_lit(c, LIT_HEAVY_DROP);
            let decayed = heavy * k - drop;
            let zero = FpuF32::ZERO;
            c.f[20] = zero.0;
            let ge = zero.le(decayed);
            abi::set_fcc(c, ge);
            let new_heavy = if ge {
                c.cycles += 1;
                decayed
            } else {
                zero
            };
            c.f[0] = decayed.0;
            c.f[1] = k.0;
            c.f[2] = new_heavy.0;
            // Decay the light level.
            c.cycles += 13;
            let light = state.off(RUMBLE_LIGHT).read_fpu(c);
            let k = gp_lit(c, LIT_LIGHT_DECAY);
            state.off(RUMBLE_HEAVY).write_fpu(c, new_heavy);
            let new_light = light * k;
            c.f[0] = new_light.0;
            c.f[1] = k.0;
            state.off(RUMBLE_LIGHT).write_fpu(c, new_light);
            let status = pad_vt_load(c, VT_STATUS, V0, V1);
            call_ptr(c, status, 0x0012_5ba8)?;
            c.g[V1][0] = 2;
            if c.g[V0][0] == 2 {
                c.cycles += 41;
                set_motor(
                    c,
                    state,
                    0,
                    0x4000_0000,
                    0x3f00_0000,
                    LIT_M0_HEAVY_DIV,
                    LIT_M0_LIGHT_SCALE,
                    0x0012_5c04,
                )?;
                set_motor(
                    c,
                    state,
                    1,
                    0x42c8_0000,
                    0x428c_0000,
                    LIT_M1_HEAVY_DIV,
                    LIT_M1_LIGHT_SCALE,
                    0x0012_5c54,
                )?;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DynamicEmitter_Render (0x371380)

/// Emitter fields: texture base index, animation frame (float), blend kind,
/// "visible" and "has particles" flags, and the render state block
/// (`rs`: 0x14 bytes at 0x1e4, flags A/B at 0x1e8/0x1ec, texture id 0x1f4).
const EM_TEX_BASE: u32 = 0x4;
const EM_KIND: u32 = 0x8;
const EM_FRAME: u32 = 0x10;
const EM_ACTIVE: u32 = 0x174;
const EM_PARTICLES: u32 = 0x1e0;
const EM_RS: u32 = 0x1e4;
const EM_RS_FLAGS_A: u32 = 0x1e8;
const EM_RS_FLAGS_B: u32 = 0x1ec;
const EM_RS_TEX: u32 = 0x1f4;
/// The particle context (`gp+0x2a90` holds the pointer): texture-id table,
/// render-state stack top (0x14-byte entries) and the renderer's vtable.
const PT_CTX: u32 = 0x2a90;
const PT_TEX_IDS: u32 = 0xf50;
const PT_RS_TOP: u32 = 0xe84;
const PT_VTABLE: u32 = 0x10d8;
const PT_VT_DRAW: u32 = 0x298;
const RS_SIZE: u32 = 0x14;
/// `D_0044B420`: blend mode per emitter kind.
const KIND_BLEND: u32 = 0x0044_b420;

/// Copy one 0x14-byte render state (`ldl/ldr` pairs and a word, through
/// registers `r1`, `r2` and a2).
fn copy_render_state(c: &mut Cpu, src: u32, dst: u32, r1: usize, r2: usize) {
    c.g[r1][0] = Addr(src).read_u64_unaligned(c);
    c.g[r2][0] = Addr(src).off(8).read_u64_unaligned(c);
    c.g[A2][0] = Addr(src).off(0x10).lw(c);
    let (a, b) = (c.g[r1][0], c.g[r2][0]);
    Addr(dst).write_u64_unaligned(c, a);
    Addr(dst).off(8).write_u64_unaligned(c, b);
    let w = c.g[A2][0];
    Addr(dst).off(0x10).sw(c, w);
}

/// `void DynamicEmitter_Render(Emitter* self, int blend)` (0x371380): if
/// the emitter is active and has particles and its current animation frame
/// has a texture, fold the kind's blend mode and `blend` into its render
/// state, push that state on the particle context's stack, draw through the
/// renderer's `vt[0x298]` and pop the state again.
pub fn dynamic_emitter_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = c.g[A0][0];
        let em = Addr(c.g[T0][0] as u32);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = em.off(EM_ACTIVE).lw(c);
        c.g[A3][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = em.off(EM_PARTICLES).lw(c);
            let gp = c.g[GP][0] as u32;
            c.g[S0][0] = Addr(gp.wrapping_add(PT_CTX)).lw(c);
            let ctx = c.g[S0][0] as u32;
            if c.g[V0][0] != 0 {
                // The texture of the current animation frame.
                c.cycles += 10;
                let anim_frame = em.off(EM_FRAME).read_fpu(c);
                c.f[0] = anim_frame.0;
                c.f[1] = anim_frame.to_i32() as u32;
                c.g[V1][0] = sx(c.f[1]);
                let base = em.off(EM_TEX_BASE).lw(c) as u32;
                let idx = base.wrapping_add(c.f[1]);
                c.g[V0][0] = sx(ctx.wrapping_add(idx << 2));
                c.g[T1][0] = Addr(ctx.wrapping_add(idx << 2)).off(PT_TEX_IDS).lw(c);
                c.g[V0][0] = 0x0045_0000;
                if (c.g[T1][0] as i64) >= 0 {
                    c.cycles += 53;
                    // Fold the blend mode bits into the render state.
                    let kind = em.off(EM_KIND).lw(c) as u32;
                    c.g[V1][0] = sx((kind << 2).wrapping_add(KIND_BLEND));
                    let mode = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    let flags_a = em.off(EM_RS_FLAGS_A).lw(c);
                    let flags_b = em.off(EM_RS_FLAGS_B).lw(c);
                    c.g[A0][0] = 0xffff_ffff_ffff_ff83;
                    c.g[V1][0] = 0xffff_ffff_ffff_fc1f;
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 5) & 0x3e0;
                    c.g[V0][0] = sx(mode << 2) & 0x7c;
                    c.g[A1][0] = (flags_a & 0xffff_ffff_ffff_ff83) | c.g[V0][0];
                    c.g[A2][0] = (flags_b & 0xffff_ffff_ffff_fc1f) | c.g[A3][0];
                    let (fa, fb) = (c.g[A1][0], c.g[A2][0]);
                    em.off(EM_RS_FLAGS_A).sw(c, fa);
                    em.off(EM_RS_FLAGS_B).sw(c, fb);
                    c.g[A1][0] = c.g[T0][0];
                    let tex = c.g[T1][0];
                    em.off(EM_RS_TEX).sh(c, tex);
                    // Push: duplicate the top entry, then overwrite it.
                    c.g[V0][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
                    let top = c.g[V0][0] as u32;
                    copy_render_state(c, top, top.wrapping_add(RS_SIZE), V1, A0);
                    c.g[V1][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
                    let top = c.g[V1][0] as u32;
                    c.g[V0][0] = sx(top.wrapping_add(RS_SIZE));
                    let new_top = c.g[V0][0];
                    Addr(ctx).off(PT_RS_TOP).sw(c, new_top);
                    copy_render_state(c, em.0.wrapping_add(EM_RS), top.wrapping_add(RS_SIZE), V0, A0);
                    // Draw through the renderer.
                    c.g[V0][0] = Addr(ctx).off(PT_VTABLE).lw(c);
                    let vt = Addr(c.g[V0][0] as u32);
                    let delta = vt.off(PT_VT_DRAW).lh(c);
                    c.g[V1][0] = vt.off(PT_VT_DRAW + 4).lw(c);
                    c.g[A0][0] = sx(ctx.wrapping_add(delta as u32));
                    let draw = c.g[V1][0] as u32;
                    call_ptr(c, draw, 0x0037_1498)?;
                    // Pop.
                    c.g[V0][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(RS_SIZE));
                    let popped = c.g[V0][0];
                    Addr(ctx).off(PT_RS_TOP).sw(c, popped);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cBoardTrailFX_RenderBatch (0x2ea538)

/// `gp-0x854`: the render context (`D_004A289C`); `gp+0x14d8`: the trail
/// texture scale divisor (`D_004A45C8`).
const GP_RENDER_CTX: u32 = 0x854;
const GP_TRAIL_TEX_DIV: u32 = 0x14d8;
/// Board trail: ring of points at +0x10, ring head +0x8c, point count +0x90.
const TRAIL_POINTS: u32 = 0x10;
const TRAIL_HEAD: u32 = 0x8c;
const TRAIL_LEN: u32 = 0x90;
/// Ring size of a trail (points).
const TRAIL_RING: u32 = 0x36;
/// Renderer vtable slot drawing a batch of trail strips.
const RC_VT_DRAW_TRAILS: u32 = 0x2c8;
/// At most 16 strips of 0x14 bytes (points, start, count, 0, scale) on the
/// stack.
const MAX_STRIPS: i64 = 16;
const STRIP_SIZE: u32 = 0x14;

fn render_ctx(c: &mut Cpu) -> u32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(GP_RENDER_CTX)).lw(c) as u32
}

/// `void cBoardTrailFX_RenderBatch(Trail** trails, int n)` (0x2ea538):
/// push the render state, collect a strip (visible ring segment, clipped to
/// 0x32 points) for each trail with at least two points (at most 16), draw
/// them through the renderer's `vt[0x2c8](ctx, strips, count)`, pop the
/// state. The loop keeps its state in the original's registers (a0 trail
/// cursor, a1 remaining, t0 strip cursor, t2 count) across its back-edge
/// event check.
pub fn cboard_trail_fx_render_batch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x150);
        let ctx = render_ctx(c);
        c.g[A2][0] = sx(ctx);
        frame.save_ra(c, 0x140);
        c.g[T2][0] = 0;
        // Push: duplicate the top render state.
        c.g[V1][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
        let top = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(top).read_u64_unaligned(c);
        c.g[A3][0] = Addr(top).off(8).read_u64_unaligned(c);
        c.g[T0][0] = Addr(top).off(0x10).lw(c);
        let (q0, q1, w) = (c.g[V0][0], c.g[A3][0], c.g[T0][0]);
        Addr(top).off(RS_SIZE).write_u64_unaligned(c, q0);
        Addr(top).off(RS_SIZE + 8).write_u64_unaligned(c, q1);
        Addr(top).off(RS_SIZE + 0x10).sw(c, w);
        c.g[V0][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(RS_SIZE));
        let new_top = c.g[V0][0];
        Addr(ctx).off(PT_RS_TOP).sw(c, new_top);
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 7;
            let gp = c.g[GP][0] as u32;
            c.g[T4][0] = Addr(gp.wrapping_add(GP_TRAIL_TEX_DIV)).lw(c);
            c.g[T3][0] = TRAIL_RING as u64;
            c.g[AT][0] = 0x3f80_0000;
            c.f[1] = FpuF32::ONE.0;
            c.g[T5][0] = 0;
            c.g[T0][0] = c.g[SP][0];
            loop {
                c.cycles += 5;
                c.g[T1][0] = Addr(c.g[A0][0] as u32).lw(c);
                let trail = Addr(c.g[T1][0] as u32);
                c.g[A2][0] = trail.off(TRAIL_LEN).lw(c);
                let len = c.g[A2][0] as u32;
                c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
                if c.g[V0][0] == 0 {
                    // The visible segment: drop the oldest points beyond 0x32.
                    c.cycles += 6;
                    let head = trail.off(TRAIL_HEAD).lw(c) as u32;
                    c.g[V0][0] = sx(len.wrapping_sub(0x39));
                    c.g[A3][0] = sx(len.wrapping_sub(0x32));
                    c.g[A2][0] = sx(len.wrapping_sub(2));
                    c.g[V1][0] = sx(head.wrapping_sub(c.g[V0][0] as u32));
                    if (c.g[A3][0] as i64) > 0 {
                        c.cycles += 2;
                        let over = c.g[A3][0] as u32;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(over));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(over));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 5;
                        let (start, ring) = (c.g[V1][0] as u32, c.g[T3][0] as u32);
                        abi::div(c, start, ring);
                        let div = FpuF32::from_i32(c.g[T4][0] as u32 as i32);
                        c.cycles += 5;
                        c.g[V0][0] = ((c.g[T2][0] as i64) < MAX_STRIPS) as u64;
                        let scale = FpuF32(c.f[1]).div(div);
                        c.f[0] = scale.0;
                        c.g[V1][0] = c.hi[0];
                        let room = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(TRAIL_POINTS));
                        if room {
                            c.cycles += 7;
                            let strip = Addr(c.g[T0][0] as u32);
                            let (st, pts, cnt) = (c.g[V1][0], c.g[V0][0], c.g[A2][0]);
                            strip.off(4).sw(c, st);
                            strip.sw(c, pts);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
                            strip.off(8).sw(c, cnt);
                            strip.off(0x10).write_fpu(c, scale);
                            strip.off(0xc).sw(c, 0);
                            c.g[T0][0] = sx(strip.0.wrapping_add(STRIP_SIZE));
                        }
                        c.cycles += 1;
                    }
                }
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                let more = (c.g[A1][0] as i64) > 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_a5a0);
                }
            }
        }
        // Draw the strips.
        c.cycles += 15;
        let ctx = render_ctx(c);
        c.g[A3][0] = sx(ctx);
        c.g[A2][0] = c.g[T2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(ctx).off(PT_VTABLE).lw(c);
        let vt = Addr(c.g[V0][0] as u32);
        let delta = vt.off(RC_VT_DRAW_TRAILS).lh(c) as u32;
        c.g[V1][0] = vt.off(RC_VT_DRAW_TRAILS + 4).lw(c);
        c.g[A0][0] = sx(ctx.wrapping_add(delta));
        let draw = c.g[V1][0] as u32;
        call_ptr(c, draw, 0x002e_a650)?;
        // Pop.
        let ctx = render_ctx(c);
        c.g[A0][0] = sx(ctx);
        frame.restore_ra(c, 0x140);
        c.g[V0][0] = Addr(ctx).off(PT_RS_TOP).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(RS_SIZE));
        let popped = c.g[V0][0];
        Addr(ctx).off(PT_RS_TOP).sw(c, popped);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCutSceneTransitionMan_Update (0x2ea900)

/// `cCutSceneTransitionMan_ForceOff` / `_ForceOn` (0x2ea820 / 0x2ea860).
const CUT_FORCE_OFF: Callee = Callee {
    addr: 0x002e_a820,
    f: ee::s09::f_002ea820,
};
const CUT_FORCE_ON: Callee = Callee {
    addr: 0x002e_a860,
    f: ee::s09::f_002ea860,
};
/// Transition manager fields: mode (2 = fading out, 3 = fading in), time,
/// duration, paused count, colour id, `from` RGBA (0x24..0x30), `to` RGBA
/// (0x38..0x44).
const CUT_MODE: u32 = 0x10;
const CUT_TIME: u32 = 0x14;
const CUT_DURATION: u32 = 0x18;
const CUT_PAUSED: u32 = 0x1c;
const CUT_COLOUR_ID: u32 = 0x20;
const CUT_FROM: u32 = 0x24;
const CUT_TO: u32 = 0x38;
/// `gp+0x2a74`: the game clock (`+0x10`: ticks per second).
const GP_CLOCK: u32 = 0x2a74;
const CLOCK_RATE: u32 = 0x10;
/// Renderer vtable slot setting the full-screen fade colour.
const RC_VT_SET_FADE: u32 = 0x148;

/// `void cCutSceneTransitionMan_Update(cCutSceneTransitionMan* self)`
/// (0x2ea900): unless paused, advance a fade (mode 2 or 3) by one tick;
/// when the duration has passed finish it (ForceOff for 2, ForceOn for 3),
/// otherwise send the interpolated colour (fade-out runs the factor
/// backwards) to the renderer's `vt[0x148]`.
pub fn ccut_scene_transition_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save_ra(c, 0x20);
        let this = Addr(c.g[A0][0] as u32);
        c.g[V0][0] = this.off(CUT_PAUSED).lw(c);
        frame.restore_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V1][0] = this.off(CUT_MODE).lw(c);
            let mode = c.g[V1][0];
            c.g[V0][0] = (sx((mode as u32).wrapping_sub(2)) < 2) as u64;
            let fading = c.g[V0][0] != 0;
            let gp = c.g[GP][0] as u32;
            c.g[V0][0] = Addr(gp.wrapping_add(GP_CLOCK)).lw(c);
            if fading {
                // Advance the fade time by one tick.
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                let one = FpuF32::ONE;
                let rate = Addr(c.g[V0][0] as u32).off(CLOCK_RATE).read_u32(c);
                let step = one.div(FpuF32::from_i32(rate as i32));
                let time = this.off(CUT_TIME).read_fpu(c) + step;
                let duration = this.off(CUT_DURATION).read_fpu(c);
                let done = duration.lt(time);
                abi::set_fcc(c, done);
                this.off(CUT_TIME).write_fpu(c, time);
                c.f[0] = step.0;
                c.f[1] = time.0;
                c.f[2] = duration.0;
                c.f[3] = one.0;
                if done {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    c.cycles += 4;
                    if mode == 2 {
                        CUT_FORCE_OFF.call(c, 0x002e_a96c)?;
                    } else {
                        CUT_FORCE_ON.call(c, 0x002e_a97c)?;
                    }
                    frame.restore_ra(c, 0x20);
                } else {
                    // Interpolate the colour.
                    c.cycles += 4;
                    let mut k = time.div(duration);
                    c.g[V0][0] = 2;
                    if mode == 2 {
                        k = one - k;
                    }
                    c.cycles += 34;
                    let from = this.off(CUT_FROM);
                    let to = this.off(CUT_TO);
                    let f = [0u32, 4, 8, 0xc].map(|o| from.off(o).read_fpu(c));
                    let t = [0u32, 4, 8, 0xc].map(|o| to.off(o).read_fpu(c));
                    let d = [0, 1, 2, 3].map(|i| (t[i] - f[i]) * k);
                    let col = [0, 1, 2, 3].map(|i| f[i] + d[i]);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = this.off(CUT_COLOUR_ID).lw(c);
                    c.g[A2][0] = Addr(gp.wrapping_add(PT_CTX)).lw(c);
                    let out = Addr(sp(c));
                    let id = c.g[V0][0];
                    out.sw(c, id);
                    for i in 0..4 {
                        out.off(4 + 4 * i as u32).write_fpu(c, col[i]);
                    }
                    // Registers as the original leaves them: f6/f7/f4/f5 the
                    // colour (r, g, b, a), f3/f0/f1/f2 the scaled deltas.
                    c.f[6] = col[0].0;
                    c.f[7] = col[1].0;
                    c.f[4] = col[2].0;
                    c.f[5] = col[3].0;
                    c.f[3] = d[0].0;
                    c.f[0] = d[1].0;
                    c.f[1] = d[2].0;
                    c.f[2] = d[3].0;
                    c.f[8] = k.0;
                    let rc = c.g[A2][0] as u32;
                    c.g[V0][0] = Addr(rc).off(PT_VTABLE).lw(c);
                    let vt = Addr(c.g[V0][0] as u32);
                    let delta = vt.off(RC_VT_SET_FADE).lh(c) as u32;
                    c.g[V1][0] = vt.off(RC_VT_SET_FADE + 4).lw(c);
                    c.g[A0][0] = sx(rc.wrapping_add(delta));
                    let set_fade = c.g[V1][0] as u32;
                    call_ptr(c, set_fade, 0x002e_aa18)?;
                    frame.restore_ra(c, 0x20);
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ComputerActorRider_FindBoardPressMarker (0x10bbf8)

/// Computer rider: its rider (+0x18); rider: the course query object
/// (+0xab8) and the track position (+0x4c0) and distance (+0x4c4).
const CAR_RIDER: u32 = 0x18;
const RIDER_QUERY: u32 = 0xab8;
const RIDER_TRACK_POS: u32 = 0x4c0;
const RIDER_TRACK_DIST: u32 = 0x4c4;
/// The query object's vtable pointer (+0x34) and its "markers in range"
/// slot `{delta, fn}` at +0x8.
const QUERY_VTABLE: u32 = 0x34;
const QUERY_VT_MARKERS: u32 = 0x8;
/// Marker hits on the stack: 12 of 0x10 bytes `{type, value, ..}`.
const MAX_HITS: u64 = 12;
const HIT_SIZE: u32 = 0x10;
const MARKER_BOARD_PRESS: u64 = 0x13;

/// `bool ComputerActorRider_FindBoardPressMarker(ComputerActorRider* self,
/// int* press)` (0x10bbf8): ask the course for up to 12 markers between the
/// rider's track position and 50 units ahead; on the first board-press
/// marker (type 0x13) store whether its value is positive and return 1,
/// else `*press = 0` and return 0. The scan keeps its state in the
/// original's registers (v1 cursor, a0 index, a1 count) across the
/// back-edge event check.
pub fn computer_actor_rider_find_board_press_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x4248_0000;
        let ahead = FpuF32(0x4248_0000);
        c.f[0] = ahead.0;
        frame.save(c, S0, 0xd0);
        c.g[A2][0] = MAX_HITS;
        frame.save_ra(c, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(CAR_RIDER).lw(c);
        let rider = Addr(c.g[V0][0] as u32);
        c.g[A3][0] = rider.off(RIDER_QUERY).lw(c);
        let query = c.g[A3][0] as u32;
        let dist = rider.off(RIDER_TRACK_DIST).read_fpu(c);
        c.g[V1][0] = Addr(query).off(QUERY_VTABLE).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.f[12] = rider.off(RIDER_TRACK_POS).read_fpu(c).0;
        c.f[13] = (dist + ahead).0;
        let delta = vt.off(QUERY_VT_MARKERS).lh(c) as u32;
        c.g[V0][0] = vt.off(QUERY_VT_MARKERS + 4).lw(c);
        c.g[A0][0] = sx(query.wrapping_add(delta));
        let markers = c.g[V0][0] as u32;
        call_ptr(c, markers, 0x0010_bc40)?;
        c.g[A1][0] = c.g[V0][0];
        let press = Addr(c.g[S0][0] as u32);
        press.sw(c, 0);
        c.g[A0][0] = 0;
        let mut found = false;
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A2][0] = MARKER_BOARD_PRESS;
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 3;
                let hit = Addr(c.g[V1][0] as u32);
                c.g[V0][0] = hit.lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.cycles += 5;
                    let value = hit.off(4).lw(c);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = (0 < value as i64) as u64;
                    let v = c.g[V1][0];
                    press.sw(c, v);
                    found = true;
                    break;
                }
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[V1][0] = sx(hit.0.wrapping_add(HIT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0010_bc58);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BezierPatchCache_TessUVs (0x374298)

/// Patch: tessellation level per LOD (`int n[]` at +0x19c); source patch:
/// the four corner UVs (`{u, v}` at +0x20, +0x28, +0x30, +0x38).
const PATCH_TESS_LEVELS: u32 = 0x19c;
const SRC_CORNER_UV: u32 = 0x20;
/// The frame (0x90 bytes) holds the whole loop state, as the original:
/// corners c0..c3, the per-row steps of the two edges, the current point
/// (first the raw right-edge delta), the per-column step and the raw row
/// delta.
const UV_C0: u32 = 0x00;
const UV_C1: u32 = 0x10;
const UV_C2: u32 = 0x20;
const UV_C3: u32 = 0x30;
const UV_DL: u32 = 0x40;
const UV_DR: u32 = 0x50;
const UV_P: u32 = 0x60;
const UV_D: u32 = 0x70;
const UV_ROW: u32 = 0x80;

fn frame_f(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

fn set_frame_f(c: &mut Cpu, off: u32, v: FpuF32) {
    Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

/// Copy a `{u, v}` pair through a GPR (`ldl/ldr`, `sdl/sdr`).
fn copy_uv(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.g[reg][0] = Addr(src).read_u64_unaligned(c);
    let v = c.g[reg][0];
    Addr(dst).write_u64_unaligned(c, v);
}

/// `void BezierPatchCache_TessUVs(Patch* self, Src* src, UV* out, int lod)`
/// (0x374298): bilinearly interpolate the four corner UVs over an n x n grid
/// (n = the LOD's tessellation level), row by row into `out`. A leaf; the
/// loops keep their state in the stack frame and the original's registers
/// (v1/v0 row counters, a0 n, a2 output cursor) across their back-edge
/// event checks.
pub fn bezier_patch_cache_tess_uvs(c: &mut Cpu) {
    c.cycles += 50;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x90));
    let level = (c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32);
    c.g[A0][0] = sx(level);
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    c.g[A0][0] = Addr(level).off(PATCH_TESS_LEVELS).lw(c);
    let n = c.g[A0][0] as u32;
    let src = (c.g[A1][0] as u32).wrapping_add(SRC_CORNER_UV);
    let s = sp(c);
    copy_uv(c, V0, src, s + UV_C0);
    c.g[V0][0] = sx(n.wrapping_sub(1));
    copy_uv(c, T1, src + 8, s + UV_C1);
    let n1 = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
    c.g[V1][0] = 0;
    copy_uv(c, T1, src + 0x10, s + UV_C2);
    copy_uv(c, T0, src + 0x18, s + UV_C3);
    let step = one.div(n1);
    // Per-row steps of the left (c0 -> c2) and right (c1 -> c3) edges.
    let c0u = frame_f(c, UV_C0);
    let c0v = frame_f(c, UV_C0 + 4);
    let dlu_raw = frame_f(c, UV_C2) - c0u;
    let dlv_raw = frame_f(c, UV_C2 + 4) - c0v;
    let c1u = frame_f(c, UV_C1);
    let c1v = frame_f(c, UV_C1 + 4);
    let dru_raw = frame_f(c, UV_C3) - c1u;
    let drv_raw = frame_f(c, UV_C3 + 4) - c1v;
    let dlu = dlu_raw * step;
    let dru = dru_raw * step;
    let dlv = dlv_raw * step;
    let drv = drv_raw * step;
    set_frame_f(c, UV_P, dru_raw);
    set_frame_f(c, UV_DL, dlu);
    set_frame_f(c, UV_P + 4, drv_raw);
    set_frame_f(c, UV_DR, dru);
    set_frame_f(c, UV_DL + 4, dlv);
    set_frame_f(c, UV_DR + 4, drv);
    c.f[0] = drv.0;
    c.f[1] = drv_raw.0;
    c.f[2] = dlu.0;
    c.f[3] = dlv.0;
    c.f[4] = dru.0;
    c.f[5] = c1u.0;
    c.f[8] = step.0;
    if (c.g[A0][0] as i64) > 0 {
        loop {
            // Row start: p = c0, d = (c1 - c0) * step.
            c.cycles += 18;
            let s = sp(c);
            copy_uv(c, A1, s + UV_C0, s + UV_P);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let step = FpuF32(c.f[8]);
            let c0u = frame_f(c, UV_C0);
            let c0v = frame_f(c, UV_C0 + 4);
            let ru = frame_f(c, UV_C1) - c0u;
            let rv = frame_f(c, UV_C1 + 4) - c0v;
            let du = ru * step;
            set_frame_f(c, UV_ROW, ru);
            let dv = rv * step;
            set_frame_f(c, UV_ROW + 4, rv);
            set_frame_f(c, UV_D, du);
            set_frame_f(c, UV_D + 4, dv);
            c.f[0] = ru.0;
            c.f[1] = rv.0;
            c.f[2] = dv.0;
            c.f[3] = du.0;
            if (c.g[A0][0] as i64) > 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[A0][0];
                loop {
                    // Emit p, then p += d.
                    c.cycles += 15;
                    let pu = frame_f(c, UV_P);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    let pv = frame_f(c, UV_P + 4);
                    let du = frame_f(c, UV_D);
                    let dv = frame_f(c, UV_D + 4);
                    let nu = pu + du;
                    let out = c.g[A2][0] as u32;
                    copy_uv(c, T2, s + UV_P, out);
                    let nv = pv + dv;
                    c.g[A2][0] = sx(out.wrapping_add(8));
                    set_frame_f(c, UV_P, nu);
                    set_frame_f(c, UV_P + 4, nv);
                    c.f[0] = du.0;
                    c.f[1] = nv.0;
                    c.f[2] = nu.0;
                    c.f[3] = dv.0;
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0037_43b0);
                    }
                }
            }
            // Next row: c0 += dl, c1 += dr.
            c.cycles += 19;
            let c0u = frame_f(c, UV_C0);
            c.g[V1][0] = c.g[V0][0];
            let c0v = frame_f(c, UV_C0 + 4);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let c1u = frame_f(c, UV_C1);
            let c1v = frame_f(c, UV_C1 + 4);
            let dlu = frame_f(c, UV_DL);
            let dlv = frame_f(c, UV_DL + 4);
            let dru = frame_f(c, UV_DR);
            let drv = frame_f(c, UV_DR + 4);
            let c0u = c0u + dlu;
            let c0v = c0v + dlv;
            let c1u = c1u + dru;
            let c1v = c1v + drv;
            set_frame_f(c, UV_C0, c0u);
            set_frame_f(c, UV_C0 + 4, c0v);
            set_frame_f(c, UV_C1, c1u);
            set_frame_f(c, UV_C1 + 4, c1v);
            c.f[0] = dlu.0;
            c.f[1] = drv.0;
            c.f[2] = c1u.0;
            c.f[3] = c1v.0;
            c.f[4] = dlv.0;
            c.f[5] = dru.0;
            c.f[6] = c0v.0;
            c.f[7] = c0u.0;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_4360);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
    ret(c);
}

// ---------------------------------------------------------------------------
// WorldTriggerManager_Update (0x2b5d78)

/// `SSXAudioSystem_GetApp` (0x28b180) and `strcmp` (0x4165a8), called
/// through the function table as the original does.
const AUDIO_GET_APP: u32 = 0x0028_b180;
const STRCMP: u32 = 0x0041_65a8;
const AUDIO_EMPTY_TRIGGER_UPDATE: Callee = Callee {
    addr: 0x0029_b960,
    f: ee::s08::f_0029b960,
};
const TRIGGER_STOP: Callee = Callee {
    addr: 0x002b_5758,
    f: ee::s08::f_002b5758,
};
const TRIGGER_UPDATE: Callee = Callee {
    addr: 0x002b_4c38,
    f: ee::s08::f_002b4c38,
};
/// Audio app: "audio paused" word.
const APP_PAUSED: u32 = 0x5fb0;
/// 40 trigger instances of 0x70 bytes: active flag (+0), "touched this
/// frame" (+0x34), sound name (+0x40).
const TRIGGER_COUNT_MINUS_1: u64 = 0x27;
const TRIGGER_SIZE: u32 = 0x70;
const TRIGGER_TOUCHED: u32 = 0x34;
const TRIGGER_NAME: u32 = 0x40;
/// `D_004A35A8`: the name of the empty trigger sound.
const EMPTY_TRIGGER_NAME: u32 = 0x004a_35a8;

/// `void WorldTriggerManager_Update(WorldTriggerManager* self)` (0x2b5d78):
/// unless the audio app is paused, for each active trigger instance: keep
/// it playing if it was touched this frame (`Update`), else (telling the
/// audio system about an "empty" trigger first) deactivate and stop it;
/// then clear the touched flag. The loop counters live in s1 (offset), s2
/// (remaining) as in the original, across the back-edge event check.
pub fn world_trigger_manager_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(c.g[S0][0] as u32);
        call_ptr(c, AUDIO_GET_APP, 0x002b_5d9c)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(APP_PAUSED).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[S4][0] = 1;
            c.g[S3][0] = 0x004a_0000;
            c.g[S1][0] = 0;
            c.g[S2][0] = TRIGGER_COUNT_MINUS_1;
            loop {
                c.cycles += 5;
                c.g[V0][0] = this.lw(c);
                let off = c.g[S1][0] as u32;
                c.g[A0][0] = sx(off.wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                if c.g[V1][0] == c.g[S4][0] {
                    c.cycles += 3;
                    let inst = c.g[A0][0] as u32;
                    c.g[V0][0] = Addr(inst).off(TRIGGER_TOUCHED).lw(c);
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        TRIGGER_UPDATE.call(c, 0x002b_5e30)?;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = sx(inst.wrapping_add(TRIGGER_NAME));
                        c.g[A1][0] = sx(EMPTY_TRIGGER_NAME);
                        call_ptr(c, STRCMP, 0x002b_5de4)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 8;
                            call_ptr(c, AUDIO_GET_APP, 0x002b_5df4)?;
                            let list = this.lw(c) as u32;
                            c.g[A0][0] = c.g[V0][0];
                            let name = list.wrapping_add(off).wrapping_add(TRIGGER_NAME);
                            c.g[A1][0] = sx(name);
                            AUDIO_EMPTY_TRIGGER_UPDATE.call(c, 0x002b_5e08)?;
                        }
                        c.cycles += 7;
                        let list = this.lw(c) as u32;
                        c.g[V0][0] = sx(off.wrapping_add(list));
                        Addr(off.wrapping_add(list)).sw(c, 0);
                        let list = this.lw(c) as u32;
                        c.g[A0][0] = sx(list.wrapping_add(off));
                        TRIGGER_STOP.call(c, 0x002b_5e20)?;
                    }
                    c.cycles += 3;
                    let list = this.lw(c) as u32;
                    c.g[V0][0] = sx(off.wrapping_add(list));
                    Addr(off.wrapping_add(list)).off(TRIGGER_TOUCHED).sw(c, 0);
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S1][0] = sx(off.wrapping_add(TRIGGER_SIZE));
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_5db8);
                }
            }
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EATimer_Rcnt1Handler (0x3e4db8)

/// EE timer 1 registers.
const T1_COUNT: u32 = 0x1000_0800;
const T1_MODE: u32 = 0x1000_0810;
const T1_COMP: u32 = 0x1000_0820;
/// T1_MODE: clear the compare interrupt flag (write 1).
const T1_MODE_EQUF: u64 = 0x400;
/// EATimer state: the 8 tick callbacks, tick counters, period (halfword)
/// and the "in callback" flag.
const EATIMER_CALLBACKS: u32 = 0x0045_0da8;
const EATIMER_TICKS: u32 = 0x0045_0dcc;
const EATIMER_TICKS2: u32 = 0x0045_0dd0;
const EATIMER_PERIOD: u32 = 0x0045_0de0;
const EATIMER_IN_CALLBACK: u32 = 0x0045_0de4;

/// `int EATimer_Rcnt1Handler(void)` (0x3e4db8): the timer 1 compare
/// interrupt. Advance the compare value by the period (relative to the
/// current count), reset the count, acknowledge the interrupt, bump both
/// tick counters, run each of the 8 registered callbacks with the
/// in-callback flag set, re-enable interrupts and return 0. The loop
/// keeps the original's registers (s1 slot pointer, s0 remaining) across
/// its back-edge event check.
pub fn ea_timer_rcnt1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x0045_0000;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = T1_COMP as u64;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A0][0] = T1_COUNT as u64;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = T1_MODE as u64;
        // Next compare = period + compare - count; restart the count.
        let period = Addr(EATIMER_PERIOD).lhu(c);
        let compare = Addr(T1_COMP).lw(c);
        c.g[T0][0] = 0x0045_0000;
        c.g[A3][0] = Addr(T1_COUNT).lw(c);
        c.g[T1][0] = 0x0045_0000;
        c.g[S2][0] = 0x0045_0000;
        let next = (period as u32)
            .wrapping_add(compare as u32)
            .wrapping_sub(c.g[A3][0] as u32);
        c.g[V0][0] = sx(next);
        Addr(T1_COMP).sw(c, sx(next));
        c.g[S1][0] = sx(EATIMER_CALLBACKS);
        Addr(T1_COUNT).sw(c, 0);
        c.g[S3][0] = 1;
        c.g[S0][0] = 7;
        // Acknowledge the interrupt.
        let mode = Addr(T1_MODE).lw(c) | T1_MODE_EQUF;
        Addr(T1_MODE).sw(c, mode);
        // Tick counters.
        c.g[V0][0] = sx((Addr(EATIMER_TICKS).lw(c) as u32).wrapping_add(1));
        let v = c.g[V0][0];
        Addr(EATIMER_TICKS).sw(c, v);
        c.g[V1][0] = sx((Addr(EATIMER_TICKS2).lw(c) as u32).wrapping_add(1));
        let v = c.g[V1][0];
        Addr(EATIMER_TICKS2).sw(c, v);
        loop {
            c.cycles += 3;
            let slot = Addr(c.g[S1][0] as u32);
            c.g[V0][0] = slot.lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let callback = c.g[V0][0] as u32;
                let flag = c.g[S3][0];
                Addr(EATIMER_IN_CALLBACK).sw(c, flag);
                call_ptr(c, callback, 0x003e_4e5c)?;
                Addr(EATIMER_IN_CALLBACK).sw(c, 0);
            }
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(slot.0.wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_4e48);
            }
        }
        c.cycles += 10;
        c.ei();
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path_GetClosestPoint (0x26a428)

/// Path: segment count (+0x8), start point (+0xc..0x14), segments (+0x18:
/// 0x10 bytes each, unit direction xyz and length).
const PATH_SEG_COUNT: u32 = 0x8;
const PATH_START: u32 = 0xc;
const PATH_SEGMENTS: u32 = 0x18;
const SEG_SIZE: u32 = 0x10;
/// `.lit4` at `gp-0x4a58`: the initial best distance (a large value).
const LIT_PATH_FAR: u32 = 0x4a58;
/// Frame slots (0x60 bytes): the current segment start point, the segment
/// direction, the point-to-start delta, the candidate point and VU0
/// spill slots.
const CP_START: u32 = 0x00;
const CP_DIR: u32 = 0x10;
const CP_DELTA: u32 = 0x20;
const CP_POINT: u32 = 0x30;
const CP_STEP: u32 = 0x40;
const CP_SPILL: u32 = 0x50;

/// `vf5 = a - b` (lane-wise, `vsub`), spilled through `spill` and v0 to `dst`.
fn vu_sub_spill(c: &mut Cpu, a: u32, b: u32, spill: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spill);
    c.lq(V0, spill);
    c.sq(V0, dst);
}

/// `vf5 = a + b`, spilled through `spill` and v0 to `dst`.
fn vu_add_spill(c: &mut Cpu, a: u32, b: u32, spill: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spill);
    c.lq(V0, spill);
    c.sq(V0, dst);
}

/// `vf5 = dir * s` with `s` moved in through GPR `gpr` (`mfc1`, `qmtc2` to
/// vf3), spilled to the spill slot and loaded into v0.
fn vu_scale_dir(c: &mut Cpu, gpr: usize, s: FpuF32) {
    let base = sp(c);
    c.g[gpr][0] = sx(s.0);
    c.lqc2(4, base + CP_DIR);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, base + CP_SPILL);
}

/// The 4-lane dot product `a . b` on VU0 (vf4.x, through v0), returned as
/// the float moved to f1.
fn vu_dot4_f1(c: &mut Cpu, a: u32, b: u32) -> FpuF32 {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    FpuF32(c.g[V0][0] as u32)
}

/// The squared length of the frame vector at `off` in the xy plane.
fn xy_len2(c: &mut Cpu, off: u32) -> (FpuF32, FpuF32) {
    let x = frame_f(c, off);
    let y = frame_f(c, off + 4);
    (x * x, y * y)
}

/// `float Path_GetClosestPoint(Path* path, Vec4* p, Vec4* closest,
/// float* dist, int flat)` (0x26a428): walk the path's segments; project
/// `p` onto each (in the xy plane when `flat`, else with the 4-lane VU0
/// dot product), clamp to the segment, and keep the nearest point in
/// `*closest`. Returns the distance along the path of that point in f0
/// and stores the distance to it in `*dist`. The segment loop keeps its
/// state in the frame and the original's registers (t1 index, f5 length
/// so far, f6 best length, f7/f9 best distances, f8 zero) across its
/// back-edge event check.
pub fn path_get_closest_point(c: &mut Cpu) {
    c.cycles += 17;
    let path = Addr(c.g[A0][0] as u32);
    let start_x = path.off(PATH_START).read_fpu(c);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    let start_y = path.off(PATH_START + 4).read_fpu(c);
    c.g[T1][0] = 0;
    let start_z = path.off(PATH_START + 8).read_fpu(c);
    c.g[AT][0] = 0x3f80_0000;
    c.g[V0][0] = path.off(PATH_SEG_COUNT).lw(c);
    let base = sp(c);
    set_frame_f(c, CP_START, start_x);
    set_frame_f(c, CP_START + 4, start_y);
    set_frame_f(c, CP_START + 8, start_z);
    let far = gp_lit(c, LIT_PATH_FAR);
    set_frame_f(c, CP_START + 0xc, FpuF32::ONE);
    c.f[0] = start_x.0;
    c.f[1] = start_y.0;
    c.f[2] = start_z.0;
    c.f[3] = FpuF32::ONE.0;
    c.f[5] = 0;
    c.f[6] = 0;
    c.f[7] = 0;
    c.f[9] = far.0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[8] = c.f[6];
        c.g[V1][0] = path.off(PATH_SEGMENTS).lw(c);
        loop {
            // The segment: direction into the frame, delta = p - start.
            c.cycles += 17;
            let flat = c.g[T0][0] != 0;
            let p = c.g[A1][0] as u32;
            let seg = Addr(((c.g[T1][0] as u32) << 4).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx(seg.0);
            let dx = seg.read_fpu(c);
            set_frame_f(c, CP_DIR, dx);
            let dy = seg.off(4).read_fpu(c);
            set_frame_f(c, CP_DIR + 4, dy);
            let dz = seg.off(8).read_fpu(c);
            Addr(base + CP_DIR + 0xc).sw(c, 0);
            set_frame_f(c, CP_DIR + 8, dz);
            let len = seg.off(0xc).read_fpu(c);
            c.f[0] = dz.0;
            c.f[1] = dy.0;
            c.f[4] = len.0;
            vu_sub_spill(c, p, base + CP_START, base + CP_POINT, base + CP_DELTA);
            // Projection of the delta on the direction.
            let along = if flat {
                c.cycles += 8;
                let ex = frame_f(c, CP_DELTA);
                let gx = frame_f(c, CP_DIR);
                let ey = frame_f(c, CP_DELTA + 4);
                let gy = frame_f(c, CP_DIR + 4);
                let a = ex * gx;
                let b = ey * gy;
                c.f[0] = b.0;
                c.f[2] = gy.0;
                c.f[3] = gx.0;
                a + b
            } else {
                c.cycles += 9;
                vu_dot4_f1(c, base + CP_DELTA, base + CP_DIR)
            };
            c.f[1] = along.0;
            // Clamp to [0, len].
            c.cycles += 4;
            let zero = FpuF32(c.f[8]);
            let ge = zero.le(along);
            abi::set_fcc(c, ge);
            let t = if ge {
                c.cycles += 1;
                along.min(len)
            } else {
                zero
            };
            c.f[2] = t.0;
            // Candidate point = start + dir * t; delta = p - candidate.
            c.cycles += 21;
            vu_scale_dir(c, V1, t);
            c.lq(V0, base + CP_SPILL);
            c.sq(V0, base + CP_STEP);
            vu_add_spill(c, base + CP_START, base + CP_STEP, base + CP_SPILL, base + CP_POINT);
            vu_sub_spill(c, p, base + CP_POINT, base + CP_SPILL, base + CP_DELTA);
            c.sq(V0, base + CP_STEP);
            let dist2 = if flat {
                c.cycles += 6;
                let (x2, y2) = xy_len2(c, CP_DELTA);
                c.f[0] = y2.0;
                x2 + y2
            } else {
                c.cycles += 9;
                vu_dot4_f1(c, base + CP_DELTA, base + CP_DELTA)
            };
            c.f[1] = dist2.0;
            // Nearer than the best so far?
            c.cycles += 3;
            let best = FpuF32(c.f[9]);
            let nearer = dist2.lt(best);
            abi::set_fcc(c, nearer);
            if nearer {
                c.cycles += 2;
                c.f[9] = dist2.0;
                if flat {
                    c.cycles += 2;
                    c.f[7] = dist2.0;
                } else {
                    c.cycles += 5;
                    let (x2, y2) = xy_len2(c, CP_DELTA);
                    c.f[1] = x2.0;
                    c.f[0] = y2.0;
                    c.f[7] = (x2 + y2).0;
                }
                c.cycles += 3;
                c.lq(V0, base + CP_POINT);
                c.f[6] = (FpuF32(c.f[5]) + t).0;
                let out = c.g[A2][0] as u32;
                c.sq(V0, out);
            }
            // Next segment start = start + dir * len.
            c.cycles += 17;
            vu_scale_dir(c, V0, len);
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.lq(V0, base + CP_SPILL);
            c.f[5] = (FpuF32(c.f[5]) + len).0;
            c.sq(V0, base + CP_STEP);
            c.lqc2(3, base + CP_START);
            c.lqc2(4, base + CP_STEP);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, base + CP_START);
            c.g[V0][0] = path.off(PATH_SEG_COUNT).lw(c);
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = path.off(PATH_SEGMENTS).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0026_a478);
            }
        }
    }
    c.cycles += 5;
    let dist = FpuF32(c.f[7]).sqrt();
    c.f[1] = dist.0;
    c.f[0] = c.f[6];
    Addr(c.g[A3][0] as u32).write_fpu(c, dist);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, put_words, Rng, DATA, GAME_GP};

    /// The race-slot table and the per-slot rider records
    /// `BE_GetRiderCharacter` reads.
    const PLAYER_SLOTS: u32 = 0x0053_05b0;
    const RIDER_RECORDS: u32 = 0x0053_5b20;

    /// BE_GetRiderCharDBWeight: a player's slot, its record's character
    /// byte, and the weight word of that character's DB record.
    #[test]
    fn be_get_rider_char_db_weight_matches() {
        check(
            ee::s01::f_0014ef30__generated,
            be_get_rider_char_db_weight,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14_ef30, 0);
                c.pc = 0x0014_ef30;
                let player = seed % 4;
                let slot = r.next_u32() % 8;
                let ch = r.next_u32() % 16;
                c.g[A1][0] = player as u64;
                c.bus.write32(PLAYER_SLOTS + 4 * player, slot);
                c.bus.write8(RIDER_RECORDS + slot * 0x1c + 0x11, ch as u8);
                c.bus
                    .write32(CHAR_DB + ch * CHAR_DB_RECORD + CHAR_WEIGHT, r.next_u32());
            },
        );
    }

    /// The `.lit4` constants of Player_ApplyRumble.
    fn rumble_lits(c: &mut Cpu) {
        let f = |x: f32| x.to_bits();
        put_words(
            c,
            GAME_GP - LIT_HEAVY_DECAY,
            &[
                f(0.913_333_36),
                f(18.518_518),
                f(0.991_666_7),
                f(972.222_2),
                f(0.01),
                f(5000.0),
                f(0.006_25),
            ],
        );
    }

    /// Player_ApplyRumble: off (levels cleared, the real
    /// `PadDevice_StopActuators` over a pad whose count/motor methods are
    /// fakes returning a count of 1), and on with the pad status 2 (both
    /// motors driven) or something else (decay only). Heavy levels on both
    /// sides of the floor.
    #[test]
    fn player_apply_rumble_matches() {
        covered(
            ee::s00::f_00125b18__generated,
            player_apply_rumble,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_5b18, 3);
                c.pc = 0x0012_5b18;
                rumble_lits(c);
                let state = 0x00a0_0000;
                let heavy = match seed % 4 {
                    0 => r.edge_f32(),
                    1 => r.f32(10.0),
                    _ => r.f32(3000.0),
                };
                c.bus.write32(state + RUMBLE_HEAVY, heavy);
                c.bus.write32(state + RUMBLE_LIGHT, r.f32(200.0));
                let pad = ptr(&mut r);
                let vt = vtable(c, &mut r, &[VT_STATUS, VT_SET_MOTOR]);
                c.bus.write32(pad, vt);
                c.g[A0][0] = state as u64;
                c.g[A1][0] = sx(pad);
                let on = seed % 5 != 0;
                if !on {
                    c.g[A2][0] = 0;
                    for n in 0..4 {
                        c.bus.write32(LOG_RESULTS + 4 * n, 1);
                    }
                } else {
                    c.g[A2][0] = if seed % 7 == 0 { 1 << 40 } else { 1 };
                    let status = if seed % 3 == 0 { 3 } else { 2 };
                    c.bus.write32(LOG_RESULTS, status);
                }
            },
        );
    }

    /// DynamicEmitter_Render: inactive, no particles, a frame without a
    /// texture (negative id), and the full push/draw/pop with a fake draw
    /// method; the stack top is word-aligned but not always 8-aligned (the
    /// `ldl`/`ldr` copies).
    #[test]
    fn dynamic_emitter_render_matches() {
        covered(
            ee::s12::f_00371380__generated,
            dynamic_emitter_render,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_1380, 1);
                c.pc = 0x0037_1380;
                let em = DATA;
                let ctx = DATA + 0x4000;
                let top = DATA + 0x8000 + 4 * (seed % 4);
                fill(c, &mut r, em, 0x80);
                fill(c, &mut r, top, 0x10);
                c.bus.write32(GAME_GP + PT_CTX, ctx);
                c.bus.write32(em + EM_ACTIVE, if seed % 9 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(em + EM_PARTICLES, if seed % 11 == 0 { 0 } else { 1 });
                let base = r.next_u32() % 8;
                c.bus.write32(em + EM_TEX_BASE, base);
                let fr = (r.next_u32() % 8) as f32 + 0.25 * (r.next_u32() % 4) as f32;
                c.bus.write32(em + EM_FRAME, fr.to_bits());
                let kind = r.next_u32() % 8;
                c.bus.write32(em + EM_KIND, kind);
                c.bus.write32(KIND_BLEND + 4 * kind, r.next_u32());
                for i in 0..16 {
                    let id = if seed % 7 == 0 { 0x8000_0000 | r.next_u32() } else { r.next_u32() & 0x7fff_ffff };
                    c.bus.write32(ctx + PT_TEX_IDS + 4 * i, id);
                }
                c.bus.write32(ctx + PT_RS_TOP, top);
                let vt = vtable(c, &mut r, &[PT_VT_DRAW]);
                c.bus.write32(ctx + PT_VTABLE, vt);
                c.g[A0][0] = em as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// cBoardTrailFX_RenderBatch: no trails (n <= 0), trails too short,
    /// long trails clipped to 0x32 points, more than 16 strips, a fake draw
    /// method.
    #[test]
    fn cboard_trail_fx_render_batch_matches() {
        covered(
            ee::s09::f_002ea538__generated,
            cboard_trail_fx_render_batch,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_a538, 1);
                c.pc = 0x002e_a538;
                let ctx = DATA + 0x4000;
                let top = DATA + 0x8000 + 4 * (seed % 4);
                fill(c, &mut r, top, 0x10);
                c.bus.write32(GAME_GP - GP_RENDER_CTX, ctx);
                c.bus.write32(ctx + PT_RS_TOP, top);
                let vt = vtable(c, &mut r, &[RC_VT_DRAW_TRAILS]);
                c.bus.write32(ctx + PT_VTABLE, vt);
                let div = if seed % 13 == 0 { 0 } else { r.next_u32() % 2000 + 1 };
                c.bus.write32(GAME_GP + GP_TRAIL_TEX_DIV, div);
                let list = DATA + 0xc000;
                let n: u64 = match seed % 6 {
                    0 => 0,
                    1 => (-(r.next_u32() as i64 % 5) as u64) | 0x8000_0000_0000_0000,
                    2 => 20,
                    _ => (r.next_u32() % 8) as u64,
                };
                for i in 0..20u32 {
                    let trail = DATA + 0x10000 + i * 0x100;
                    c.bus.write32(list + 4 * i, trail);
                    c.bus.write32(trail + TRAIL_LEN, r.next_u32() % 0x40);
                    c.bus.write32(trail + TRAIL_HEAD, r.next_u32() % 0x80);
                }
                c.g[A0][0] = list as u64;
                c.g[A1][0] = n;
            },
        );
    }

    /// cCutSceneTransitionMan_Update: paused, not fading, fades in both
    /// directions that end this tick (the real ForceOff/ForceOn, which reach
    /// the fake renderer method) or interpolate the colour.
    #[test]
    fn ccut_scene_transition_man_update_matches() {
        covered(
            ee::s09::f_002ea900__generated,
            ccut_scene_transition_man_update,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_a900, 1);
                c.pc = 0x002e_a900;
                let this = DATA;
                let ctx = DATA + 0x4000;
                let clock = DATA + 0x6000;
                fill(c, &mut r, this, 0x14);
                c.bus.write32(this + CUT_PAUSED, if seed % 10 == 0 { 1 } else { 0 });
                let mode = match seed % 9 {
                    0 => r.next_u32() % 8,
                    1..=4 => 2,
                    _ => 3,
                };
                c.bus.write32(this + CUT_MODE, mode);
                c.bus.write32(this + CUT_TIME, ((r.next_u32() % 200) as f32 / 100.0).to_bits());
                c.bus
                    .write32(this + CUT_DURATION, ((r.next_u32() % 200 + 10) as f32 / 100.0).to_bits());
                for i in 0..9 {
                    c.bus.write32(this + CUT_FROM + 4 * i, r.f32(255.0));
                }
                c.bus.write32(GAME_GP + GP_CLOCK, clock);
                c.bus.write32(clock + CLOCK_RATE, [60, 50, 30, 1][(seed % 4) as usize]);
                c.bus.write32(GAME_GP + PT_CTX, ctx);
                let vt = vtable(c, &mut r, &[RC_VT_SET_FADE]);
                c.bus.write32(ctx + PT_VTABLE, vt);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// ComputerActorRider_FindBoardPressMarker: no markers (the vtable
    /// method is the real leaf at 0x1406c0 returning 0), and a fake returning
    /// 1..12 hits with or without a board-press marker (positive or not)
    /// among them.
    #[test]
    fn computer_actor_rider_find_board_press_marker_matches() {
        covered(
            ee::s00::f_0010bbf8__generated,
            computer_actor_rider_find_board_press_marker,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_bbf8, 1);
                c.pc = 0x0010_bbf8;
                let this = DATA;
                let rider = DATA + 0x1000;
                let query = DATA + 0x2000;
                let press = DATA + 0x3000;
                c.bus.write32(this + CAR_RIDER, rider);
                c.bus.write32(rider + RIDER_QUERY, query);
                c.bus.write32(rider + RIDER_TRACK_POS, r.edge_f32());
                c.bus.write32(rider + RIDER_TRACK_DIST, r.f32(5000.0));
                let vt = vtable(c, &mut r, &[QUERY_VT_MARKERS]);
                c.bus.write32(query + QUERY_VTABLE, vt);
                if seed % 8 == 0 {
                    c.bus.write32(vt + QUERY_VT_MARKERS + 4, 0x0014_06c0);
                }
                let n = r.next_u32() % 12 + 1;
                c.bus.write32(LOG_RESULTS, n);
                // The hit buffer the (fake) query leaves on the stack.
                let buf = crate::difftest::SP - 0xe0;
                for i in 0..12 {
                    let ty = if r.next_u32() % 5 == 0 { 0x13 } else { r.next_u32() % 0x20 };
                    let ty = if seed % 3 == 0 && ty == 0x13 { 0x12 } else { ty };
                    c.bus.write32(buf + HIT_SIZE * i, ty);
                    c.bus.write32(buf + HIT_SIZE * i + 4, (r.next_u32() % 4) << 30);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = press as u64;
            },
        );
    }

    /// BezierPatchCache_TessUVs: grids of 0..6 points per side (and a
    /// negative level), corner UVs and the output only word-aligned in some
    /// seeds (the `ldl`/`sdl` copies).
    #[test]
    fn bezier_patch_cache_tess_uvs_matches() {
        check(
            ee::s12::f_00374298__generated,
            bezier_patch_cache_tess_uvs,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_4298, 0);
                c.pc = 0x0037_4298;
                let patch = DATA;
                let src = DATA + 0x1000 + 4 * (seed % 2);
                let out = DATA + 0x2000 + 4 * (seed / 2 % 2);
                let lod = r.next_u32() % 4;
                let n = match seed % 9 {
                    0 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() % 7,
                };
                c.bus.write32(patch + PATCH_TESS_LEVELS + 4 * lod, n);
                for i in 0..8 {
                    c.bus.write32(src + SRC_CORNER_UV + 4 * i, r.f32(4.0));
                }
                fill(c, &mut r, out, 0x60);
                c.g[A0][0] = patch as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = lod as u64;
            },
        );
    }

    /// WorldTriggerManager_Update: audio paused, and 40 instances that are
    /// inactive, touched (the real `WorldTriggerInstance_Update`, on its
    /// short path: no crowd source, flag-8 check off) or untouched (the real
    /// `strcmp` against the empty-trigger name, the real no-op
    /// `EmptyTriggerUpdate`, the real `WorldTriggerInstance_Stop` on its
    /// short path).
    #[test]
    fn world_trigger_manager_update_matches() {
        check(
            ee::s08::f_002b5d78__generated,
            world_trigger_manager_update,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_5d78, 0);
                c.pc = 0x002b_5d78;
                let this = DATA;
                let app = DATA + 0x1000;
                let list = DATA + 0x10000;
                c.bus.write32(GAME_GP + 0x410, app);
                c.bus.write32(app + APP_PAUSED, if seed % 10 == 0 { 1 } else { 0 });
                c.bus.write32(GAME_GP + 0x21e4, DATA + 0x8000);
                c.bus.write32(this, list);
                let names: [&[u8]; 3] = [b"empty\0", b"crowd\0", b"emptx\0"];
                for (i, b) in names[0].iter().enumerate() {
                    c.bus.write8(EMPTY_TRIGGER_NAME + i as u32, *b);
                }
                for i in 0..40 {
                    let t = list + TRIGGER_SIZE * i;
                    fill(c, &mut r, t, TRIGGER_SIZE / 4);
                    c.bus.write32(t, if r.next_u32() % 3 == 0 { r.next_u32() % 3 } else { 1 });
                    c.bus.write32(t + 0xc, r.next_u32() % 2);
                    c.bus.write32(t + 0x30, -9999i32 as u32);
                    c.bus.write32(t + 0x38, 0);
                    c.bus.write32(t + TRIGGER_TOUCHED, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() | 1 });
                    let name = names[(r.next_u32() % 3) as usize];
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(t + TRIGGER_NAME + k as u32, *b);
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// EATimer_Rcnt1Handler: the timer 1 registers of the test machine, a
    /// random period and tick counters, and 8 callback slots that are empty
    /// or fakes.
    #[test]
    fn ea_timer_rcnt1_handler_matches() {
        covered(
            ee::s14::f_003e4db8__generated,
            ea_timer_rcnt1_handler,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_4db8, 8);
                c.pc = 0x003e_4db8;
                c.bus.write16(EATIMER_PERIOD, r.next_u32() as u16);
                c.bus.write32(EATIMER_TICKS, r.next_u32());
                c.bus.write32(EATIMER_TICKS2, r.next_u32());
                c.bus.write32(EATIMER_IN_CALLBACK, r.next_u32());
                c.bus.write32(T1_COMP, r.next_u32() & 0xffff);
                c.bus.write32(T1_COUNT, r.next_u32() & 0xffff);
                for i in 0..8 {
                    let cb = if r.next_u32() % 3 == 0 { 0 } else { FAKE + 0x10 * i };
                    c.bus.write32(EATIMER_CALLBACKS + 4 * i, cb);
                }
            },
        );
    }

    /// Path_GetClosestPoint: paths of 0..5 segments (and a negative
    /// count), the point anywhere around them, flat (xy) and full (VU0)
    /// distances, so the projection clamps at both ends and the best point
    /// changes or not.
    #[test]
    fn path_get_closest_point_matches() {
        check(
            ee::s07::f_0026a428__generated,
            path_get_closest_point,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26_a428, 0);
                c.pc = 0x0026_a428;
                let path = DATA;
                let segs = DATA + 0x1000;
                let p = DATA + 0x2000;
                let out = DATA + 0x3000;
                let dist = DATA + 0x3100;
                let n = match seed % 11 {
                    0 => 0,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() % 5 + 1,
                };
                c.bus.write32(path + PATH_SEG_COUNT, n);
                for i in 0..3 {
                    c.bus.write32(path + PATH_START + 4 * i, r.f32(50.0));
                }
                c.bus.write32(path + PATH_SEGMENTS, segs);
                for s in 0..5 {
                    let a = (r.next_u32() % 628) as f32 / 100.0;
                    let z = (r.next_u32() % 100) as f32 / 400.0;
                    let seg = segs + SEG_SIZE * s;
                    c.bus.write32(seg, a.cos().to_bits());
                    c.bus.write32(seg + 4, a.sin().to_bits());
                    c.bus.write32(seg + 8, z.to_bits());
                    c.bus.write32(seg + 0xc, ((r.next_u32() % 4000) as f32 / 100.0).to_bits());
                }
                for i in 0..4 {
                    c.bus.write32(p + 4 * i, r.f32(80.0));
                }
                fill(c, &mut r, out, 4);
                let far = if seed % 5 == 0 { 100.0f32 } else { 1.0e30 };
                c.bus.write32(GAME_GP - LIT_PATH_FAR, far.to_bits());
                c.g[A0][0] = path as u64;
                c.g[A1][0] = p as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = dist as u64;
                c.g[T0][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 | 1 };
            },
        );
    }
}
