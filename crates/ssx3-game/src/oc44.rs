//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for model/effect renderers and animation dispatch, each with a
//! differential test against the generated body.
//!
//! - **Rider model renderers** (`RiderModel_Render`, `RiderModel_RenderShadow`,
//!   `Rider_RenderModel`): concatenate the per-bone rotation/translation
//!   matrices on VU0 into the global bone matrix array, then either walk the
//!   parts of the model (`unk_018 != 0`: per-part LOD reference, per-part
//!   matrix and draw) or draw every part with the single LOD matrix; the
//!   rider wrapper picks the LOD, sets the rider render state and draws the
//!   model.
//! - **Effect renderers** (`cTransitionMan_Render`, `cPlayerPulseFX_Update`,
//!   `mEmitter_Render`, `Emitter_Render`, `cFlagManager_GroupRender`): fade
//!   screen quad, pulse colour interpolation, emitter billboard and flag
//!   group draws. They push a copy of the current render state on the
//!   context's stack, rewrite the flag words and call the context's virtual
//!   methods.
//! - **`ParticleModifier_Update`** and **`AnimController_DispatchEndEvents`**:
//!   per-frame modifier tick (life timer, done flag) and the end-of-animation
//!   event dispatch (collect the sequences whose node has the end flag, then
//!   call the handler for each).
//!
//! Registers, call protocol (`crate::call`), VU0 macro operations and cycle
//! charges follow the generated bodies exactly; the differential tests are
//! the proof.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_0640, rider_model_render, "RiderModel_Render"),
    (0x0031_0948, rider_model_render_shadow, "RiderModel_RenderShadow"),
    (0x0012_2448, rider_render_model, "Rider_RenderModel"),
    (0x002e_47e8, transition_man_render, "cTransitionMan_Render"),
    (0x002e_39d8, player_pulsefx_update, "cPlayerPulseFX_Update"),
    (0x0037_1688, m_emitter_render, "mEmitter_Render"),
    (0x0037_08c0, emitter_render, "Emitter_Render"),
    (0x0034_5b40, particle_modifier_update, "ParticleModifier_Update"),
    (0x0034_b9b0, flag_manager_group_render, "cFlagManager_GroupRender"),
    (0x0031_2490, anim_controller_dispatch_end_events, "AnimController_DispatchEndEvents"),
];

// ---------------------------------------------------------------------------
// Callees reached through the generated function table, and fixed addresses.

#[allow(dead_code)]
mod callee {
    use super::{ee, Callee};

    /// Same-shard direct calls (`jal`).
    pub const ANIM_SEQ_LIST_GET_AT: Callee =
        Callee { addr: 0x0031_4760, f: ee::s10::AnimSeqList_GetAt };
    pub const RIDER_COMPUTE_LOD: Callee =
        Callee { addr: 0x0012_1f30, f: ee::s00::Rider_ComputeLOD };
    pub const TRANSITION_EFFECT_UPDATE_SCREEN_RECT: Callee =
        Callee { addr: 0x002e_b8f0, f: ee::s09::TransitionEffect_UpdateScreenRect };

    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const RIDER_MODEL_RENDER: u32 = 0x0031_0640;
    pub const FLAGS64_TEST: u32 = 0x0014_46a0;
    pub const MATH_COS: u32 = 0x0031_c040;
    pub const EMITTER_UPDATE: u32 = 0x0037_0788;
    pub const WORLD_PAINTER_QUERY_RESET_ALL: u32 = 0x002c_03e8;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
mod ctx {
    pub const VTABLE: u32 = 0x10d8;
    pub const TOP: u32 = 0xe84;
    pub const TEX_IDS: u32 = 0xf50;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
mod globals {
    pub const AM_CTX: i32 = -0x854;
    pub const WORLD: i32 = -0x848;
    /// `D_0044B420`: billboard blend mode by emitter kind.
    pub const EMITTER_BLEND: u32 = 0x0044_b420;
    /// `D_004FF1A0`: the identity matrix flag groups start from.
    pub const IDENTITY_MAT: u32 = 0x004f_f1a0;
    /// `D_00501420`: the base flag render state.
    pub const FLAG_STATE: u32 = 0x0050_1420;
    /// The two global bone-matrix arrays the renderers build.
    pub const BONE_MATRICES: u32 = 0x004f_c420;
    pub const BONE_MATRICES_SHADOW: u32 = 0x004f_d420;
    /// `D_004A4244` (flag manager) and the flag/model table behind it.
    pub const FLAG_MANAGER: i32 = 0x1154;
    pub const FLAG_TABLE: i32 = 0x16c8;
}

/// `Rider` fields the renderer wrapper reads.
#[allow(dead_code)]
mod rider {
    pub const MODEL: u32 = 0x780;
    pub const ANIM: u32 = 0x784;
    pub const LOD: u32 = 0x898;
    pub const ON_SCREEN: u32 = 0xb18;
    pub const HIDDEN: u32 = 0xb1c;
    pub const HANDLER: u32 = 0x77c;
    pub const SUB_STATE: u32 = 0x2f4;
    pub const MATRIX: u32 = 0x7c0;
    pub const POSITION: u32 = 0x110;
}

/// `RiderModel` (an `sAnimModel`): the two bone tables, the parts array and
/// the per-part LOD info.
#[allow(dead_code)]
mod model {
    pub const NUM_PARTS: u32 = 0x8;
    pub const PARTS: u32 = 0xc;
    pub const NUM_BONES: u32 = 0x10;
    pub const HAS_PARTS: u32 = 0x18;
    pub const BONE_TABLE_A: u32 = 0x34;
    pub const BONE_TABLE_A_SHADOW: u32 = 0x30;
    pub const BONE_TABLE_B: u32 = 0x38;
    pub const LOD_INFO: u32 = 0x40;
    pub const SCALE_Y: u32 = 0x144;
    /// `sAnimModelPart` stride; `lods[lod]` is `{used, refs}` at `+0x18+8*lod`,
    /// `refIndex` at `+0x50`, and each `sAnimModelLodRef` is 16 bytes.
    pub const PART_STRIDE: u32 = 0x58;
    pub const PART_LODS: u32 = 0x18;
    pub const PART_REF_INDEX: u32 = 0x50;
}

// ---------------------------------------------------------------------------
// Register / memory helpers.

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

/// `lwc1 fN, a`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    c.f[n] = v.0;
    v
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `swc1 fN, a`.
#[inline(always)]
fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// Set the FPU condition bit; returns the condition.
#[inline(always)]
fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `lw`.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `lh`.
#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}

/// `lh vt+off` (the vtable slot's `this` adjustment) and `lw vt+off+4`
/// (its entry point).
#[inline(always)]
fn vslot(c: &mut Cpu, vt: u32, off: u32) -> (u64, u64) {
    (lh(c, p(vt, off)), lw(c, p(vt, off + 4)))
}

/// The 20-byte render-state copy the compiler emits as two unaligned 8-byte
/// loads/stores plus a word, leaving the loaded values in the given GPRs.
#[inline(always)]
fn copy_render_state(c: &mut Cpu, r0: usize, r1: usize, word: usize, src: u32, dst: u32) {
    c.ldl(r0, p(src, 7));
    c.ldr(r0, src);
    c.ldl(r1, p(src, 0xf));
    c.ldr(r1, p(src, 8));
    c.g[word][0] = lw(c, p(src, 0x10));
    c.sdl(r0, p(dst, 0x1b));
    c.sdr(r0, p(dst, 0x14));
    c.sdl(r1, p(dst, 0x23));
    c.sdr(r1, p(dst, 0x1c));
    let w = c.g[word][0];
    c.sw(p(dst, 0x24), w);
}

/// `out = lhs * rhs` for two 4x4 matrices through VU0 (vf4..7 = lhs,
/// vf8..11 = rhs, result rows in vf12..15), as the model renderers issue it.
fn concat_matrix(c: &mut Cpu, lhs: u32, rhs: u32, out: u32) {
    c.lqc2(4, lhs);
    c.lqc2(5, p(lhs, 0x10));
    c.lqc2(6, p(lhs, 0x20));
    c.lqc2(7, p(lhs, 0x30));
    c.lqc2(8, rhs);
    c.lqc2(9, p(rhs, 0x10));
    c.lqc2(10, p(rhs, 0x20));
    c.lqc2(11, p(rhs, 0x30));
    let vu = c.vu0();
    vu.mula(0b1111, 4, Src::Bc(8, X));
    vu.madda(0b1111, 5, Src::Bc(8, Y));
    vu.madda(0b1111, 6, Src::Bc(8, Z));
    vu.madd(0b1111, 12, 7, Src::Bc(8, W));
    vu.mula(0b1111, 4, Src::Bc(9, X));
    vu.madda(0b1111, 5, Src::Bc(9, Y));
    vu.madda(0b1111, 6, Src::Bc(9, Z));
    vu.madd(0b1111, 13, 7, Src::Bc(9, W));
    vu.mula(0b1111, 4, Src::Bc(10, X));
    vu.madda(0b1111, 5, Src::Bc(10, Y));
    vu.madda(0b1111, 6, Src::Bc(10, Z));
    vu.madd(0b1111, 14, 7, Src::Bc(10, W));
    vu.mula(0b1111, 4, Src::Bc(11, X));
    vu.madda(0b1111, 5, Src::Bc(11, Y));
    vu.madda(0b1111, 6, Src::Bc(11, Z));
    vu.madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, out);
    c.sqc2(13, p(out, 0x10));
    c.sqc2(14, p(out, 0x20));
    c.sqc2(15, p(out, 0x30));
}

// ---------------------------------------------------------------------------
// ParticleModifier_Update.

/// `void ParticleModifier_Update(ParticleModifier* m)` (0x345b40): while the
/// modifier is not done (`+0x1e4`), tick its emitter (`Emitter_Update(m+0x50,
/// dt)`); a negative life takes the single-tick path, otherwise the life is
/// re-checked and the modifier is marked done once it has run out. `f20` is
/// zeroed (the compare constant) and restored.
pub fn particle_modifier_update(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let m = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x20);
    guest(c, |c| {
        let done = lw(c, p(m, 0x1e4));
        c.g[V0][0] = done;
        if done == 0 {
            c.cycles += 6;
            let life = ldf(c, 0, p(m, 0x50));
            c.mtc1(0, 20); // f20 = 0.0
            let negative = fcc(c, life.lt(FpuF32::ZERO));
            c.g[A0][0] = sx(p(m, 0x50));
            if !negative {
                // life >= 0: tick, then mark done when it reached the end.
                c.cycles += 8;
                ldf(c, 12, gp(c, -0x2b50));
                call_ptr(c, callee::EMITTER_UPDATE, 0x0034_5b98)?;
                let life = ldf(c, 0, p(m, 0x50));
                let spent = fcc(c, life.le(FpuF32::ZERO));
                c.g[V0][0] = sx(1);
                if spent {
                    c.cycles += 1;
                    Addr(p(m, 0x1e4)).sw(c, 1);
                }
                c.cycles += 1;
            } else {
                // life < 0: one tick only.
                c.cycles += 5;
                ldf(c, 12, gp(c, -0x2b54));
                call_ptr(c, callee::EMITTER_UPDATE, 0x0034_5b84)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.restore_ra(c, 0);
    frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// mEmitter_Render (0x371688).

/// `void func_00371688(Emitter* e, int blend)` (0x371688): if the emitter
/// has a texture (`+0x174 != 0`, layer texture `+0x1e0 != 0`, and a valid
/// `texIds[base + trunc(frame)]`), push a render state: blend mode
/// `D_0044B420[kind]<<2` in `flagsA`, `blend<<5` in `flagsB`, that texture,
/// the emitter's own render state copied on top, and draw the emitter through
/// slot 84.
pub fn m_emitter_render(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    let e = r32(c, A0);
    c.g[T0][0] = c.g[A0][0];
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[A3][0] = c.g[A1][0];
    guest(c, |c| {
        'body: {
            let visible = lw(c, p(e, 0x174));
            c.g[V0][0] = visible;
            if visible == 0 {
                break 'body;
            }
            c.cycles += 3;
            let layered = lw(c, p(e, 0x1e0));
            c.g[V0][0] = layered;
            let context = lw(c, gp(c, 0x2a90)) as u32;
            c.g[S0][0] = sx(context);
            if layered == 0 {
                break 'body;
            }
            c.cycles += 10;
            let _u = ldf(c, 0, p(e, 0x10));
            c.g[V0][0] = lw(c, p(e, 0x4));
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1); // v1 = trunc(frame)
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(context.wrapping_add(r32(c, V0)));
            let tex = lw(c, p(r32(c, V0), ctx::TEX_IDS));
            c.g[T1][0] = tex;
            let t = (tex as i64) < 0;
            c.g[V0][0] = sx(0x45_0000);
            if t {
                break 'body;
            }
            c.cycles += 53;
            let kind = lw(c, p(e, 0x8));
            c.g[V1][0] = kind;
            c.g[V0][0] = sx(0x45_0000u32.wrapping_sub(0x4be0)); // &D_0044B420
            let flags_a = lw(c, p(e, 0x1e8));
            c.g[A1][0] = flags_a;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            let flags_b = lw(c, p(e, 0x1ec));
            c.g[A2][0] = flags_b;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = sx(!0x7cu32);
            let layer = lw(c, p(r32(c, V1), 0));
            c.g[V0][0] = layer;
            c.g[A3][0] = sx(r32(c, A3) << 5);
            c.g[V1][0] = sx(!0x3e0u32);
            c.g[A1][0] = flags_a & c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A2][0] = flags_b & c.g[V1][0];
            c.g[A3][0] = c.g[A3][0] & 0x3e0;
            c.g[V0][0] = c.g[V0][0] & 0x7c;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A3][0];
            let a1 = c.g[A1][0];
            Addr(p(e, 0x1e8)).sw(c, a1);
            let a2 = c.g[A2][0];
            Addr(p(e, 0x1ec)).sw(c, a2);
            c.g[A1][0] = c.g[T0][0];
            let tex = c.g[T1][0];
            c.sh(p(e, 0x1f4), tex);
            // ctx->top[1] = ctx->top[0]; ctx->top++.
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V0][0] = sx(top);
            copy_render_state(c, V1, A0, A2, top, top);
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(top);
            c.g[V0][0] = sx(p(top, 0x14));
            let new_top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, new_top);
            // *ctx->top = e->rs.
            copy_render_state(c, V0, A0, A2, p(e, 0x1e4), top);
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[V0][0] = sx(vt);
            let (delta, f) = vslot(c, vt, 0x2a0);
            c.g[A0][0] = delta;
            c.g[V1][0] = f;
            let target = f as u32;
            c.g[A0][0] = sx(context.wrapping_add(delta as u32));
            call_ptr(c, target, 0x0037_17a0)?;
            let top = lw(c, p(context, ctx::TOP));
            c.g[V0][0] = top;
            c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
            let top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, top);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
    frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_Render (0x3708c0).

/// `void Emitter_Render(Emitter* e)` (0x3708c0): cull the emitter's billboard
/// (`e+0x160`/`e+0x170`) with the camera pair from slot 43; when visible, push
/// a render state with the emitter's flags, texture
/// `texIds[base + trunc(e+0x184)]`, draw `e+0x10` through slot 82 and pop the
/// state.
pub fn emitter_render(c: &mut Cpu) {
    c.cycles += 29;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S4, 0x60);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S5, 0x50);
    frame.save(c, S1, 0x90);
    let e = r32(c, A0);
    c.g[S5][0] = c.g[A0][0];
    frame.save_ra(c, 0x40);
    let context = lw(c, gp(c, 0x2a90)) as u32;
    c.g[S4][0] = sx(context);
    c.g[A0][0] = sx(frame.at(c, 0));
    c.g[S3][0] = sx(p(e, 0x160));
    frame.save(c, S0, 0xa0);
    c.g[S2][0] = sx(p(e, 0x170));
    guest(c, |c| {
        'body: {
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[S0][0] = sx(vt);
            let (d93, f93) = vslot(c, vt, 0x2e8);
            c.g[S1][0] = d93;
            c.g[A1][0] = lh(c, p(vt, 0x158));
            let f43 = lw(c, p(vt, 0x15c));
            c.g[V0][0] = f43;
            c.g[S1][0] = sx(context.wrapping_add(d93 as u32));
            let f43 = f43 as u32;
            c.g[A1][0] = sx(context.wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, f43, 0x0037_0910)?;
            c.g[V0][0] = f93;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            let f93 = f93 as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, f93, 0x0037_0928)?;
            c.g[V1][0] = sx(1);
            let t = c.g[V0][0] == c.g[V1][0];
            frame.restore(c, S0, 0xa0);
            if t {
                break 'body;
            }
            c.cycles += 3;
            let visible = lw(c, p(e, 0xc));
            c.g[V0][0] = visible;
            let t = visible == 0;
            c.g[V0][0] = sx(0x45_0000);
            if t {
                break 'body;
            }
            c.cycles += 82;
            let kind = lw(c, p(e, 0x8));
            c.g[V1][0] = kind;
            c.g[V0][0] = sx(0x45_0000u32.wrapping_sub(0x4be0));
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(top);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = sx(!0x3e0u32);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
            let layer = lw(c, p(r32(c, V1), 0));
            c.g[T1][0] = layer;
            copy_render_state(c, 2, 3, A3, top, top);
            c.g[T5][0] = 0x40_0000;
            c.g[A3][0] = 0xffff_ffff_fe7f_ffff;
            c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
            let top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(top);
            c.g[T4][0] = sx(context);
            c.g[V0][0] = sx(p(top, 0x14));
            c.g[T3][0] = 0xffff_ffff_fff0_0fff;
            c.g[T0][0] = 0x1_4000;
            c.g[T1][0] = sx(r32(c, T1) << 2);
            c.g[V0][0] = lw(c, p(top, 0x1c));
            c.g[T6][0] = 0xffff_ffff_ffff_ff83;
            c.g[T1][0] = c.g[T1][0] & 0x7c;
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.g[A1][0] = sx(p(e, 0x10));
            let v0 = c.g[V0][0];
            Addr(p(top, 0x1c)).sw(c, v0);
            // top++.
            let new_top = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(new_top);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            let pushed = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, pushed);
            // flagsA: (flagsA & ~0x400000) | 0x400000.
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
            // texture id and draw.
            let _u = ldf(c, 0, p(e, 0x184));
            c.g[V0][0] = lw(c, p(e, 0x4));
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[T4][0] = sx(r32(c, T4).wrapping_add(r32(c, V0)));
            let tex = lw(c, p(r32(c, T4), ctx::TEX_IDS));
            c.g[V1][0] = tex;
            c.sh(p(a, 0x10), tex);
            let vt = lw(c, p(context, ctx::VTABLE)) as u32;
            c.g[V0][0] = sx(vt);
            c.g[A0][0] = lh(c, p(vt, 0x290));
            let f = lw(c, p(vt, 0x294));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[A0][0] = sx(context.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, f, 0x0037_0a78)?;
            let top = lw(c, p(context, ctx::TOP));
            c.g[V0][0] = top;
            c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
            let top = c.g[V0][0];
            Addr(p(context, ctx::TOP)).sw(c, top);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPlayerPulseFX_Update (0x2e39d8).

/// `void cPlayerPulseFX_Update(cPlayerPulseFX* p)` (0x2e39d8): if the player's
/// FX object is active, advance the pulse phase by
/// `rate * (1/60)` (`rate` = `D_004A55AC`), wrap it into `[0, 2π)` with the
/// FPU truncation the original uses, store it, and blend the four output
/// values between `a` (`+0x8`) and `b` (`+0x18`) with
/// `s = 0.5 + 0.5*cos(phase)`.
pub fn player_pulsefx_update(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let pfx = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x20);
    guest(c, |c| {
        'body: {
        let rider = lw(c, pfx) as u32;
        c.g[A0][0] = sx(rider);
        let vt = lw(c, p(rider, 0x6c0)) as u32;
        c.g[V1][0] = sx(vt);
        c.g[A0][0] = sx(p(rider, 0x6c0));
        c.g[A1][0] = lh(c, p(vt, 0x40));
        let f = lw(c, p(vt, 0x44));
        c.g[V0][0] = f;
        let f = f as u32;
        c.g[A0][0] = sx(p(rider, 0x6c0).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, f, 0x002e_3a08)?;
        let t = c.g[V0][0] == 0;
        // Read before the branch: the original loads `rate` first.
        ldf(c, 1, gp(c, 0x24bc));
        if t {
            break 'body;
        }
        c.cycles += 14;
        let rate = ldf(c, 2, gp(c, -0x3a30));
        let phase = ldf(c, 3, p(pfx, 4));
        let half_rate = setf(c, 1, rate * fr(c, 1));
        let w = ldf(c, 0, gp(c, -0x3a2c));
        li_f(c, 20, 0x3f00_0000); // 0.5
        let p_ = setf(c, 3, phase + half_rate);
        let prod = setf(c, 0, p_ * w);
        let t_ = setf(c, 0, prod + fr(c, 20));
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        let floor = fcc(c, t_.lt(fr(c, 1)));
        if floor {
            c.cycles += 3;
            let one = li_f(c, 0, 0x3f80_0000);
            setf(c, 1, fr(c, 1) - one);
        }
        c.cycles += 35;
        let two_pi = ldf(c, 0, gp(c, -0x3a28));
        let prod = setf(c, 0, fr(c, 1) * two_pi);
        let q = setf(c, 0, fr(c, 3) - prod);
        setf(c, 12, q);
        stf(c, 0, p(pfx, 4));
        call_ptr(c, callee::MATH_COS, 0x002e_3a6c)?;
        let half = setf(c, 0, fr(c, 0) * fr(c, 20));
        let one = li_f(c, 1, 0x3f80_0000);
        let a0 = ldf(c, 5, p(pfx, 0x8));
        let b0 = ldf(c, 7, p(pfx, 0x18));
        let s = setf(c, 0, half + fr(c, 20));
        let a1 = ldf(c, 4, p(pfx, 0xc));
        let b1 = ldf(c, 6, p(pfx, 0x1c));
        let a3 = ldf(c, 3, p(pfx, 0x14));
        let u = setf(c, 1, one - s);
        let b3 = ldf(c, 2, p(pfx, 0x24));
        let a2 = ldf(c, 9, p(pfx, 0x10));
        let sa0 = setf(c, 5, s * a0);
        let b2 = ldf(c, 8, p(pfx, 0x20));
        let sa1 = setf(c, 4, s * a1);
        let sa3 = setf(c, 3, s * a3);
        let ub3 = setf(c, 2, u * b3);
        let ub0 = setf(c, 7, u * b0);
        let ub1 = setf(c, 6, u * b1);
        let sa2 = setf(c, 0, s * a2);
        let ub2 = setf(c, 1, u * b2);
        let out0 = setf(c, 5, sa0 + ub0);
        let out1 = setf(c, 4, sa1 + ub1);
        let out2 = setf(c, 0, sa2 + ub2);
        let out3 = setf(c, 3, sa3 + ub3);
        stf(c, 5, p(pfx, 0x28));
        stf(c, 4, p(pfx, 0x2c));
        stf(c, 0, p(pfx, 0x30));
        stf(c, 3, p(pfx, 0x34));
        let _ = (out0, out1, out3);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.restore_ra(c, 0);
    frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimController_DispatchEndEvents (0x312490).

/// `void func_00312490(AnimController* a)` (0x312490): scan the six animation
/// sequences (`a+0x50`, 8-byte list heads) and every node of each, collect the
/// nodes whose `+0xb0` flags have bit `0x3f` (`Flags64_Test(node+0xb0, 0x3f)`),
/// then call the handler vtable slot 3 (`a+0x58`) for each collected
/// `(sequence, node)` pair in order.
pub fn anim_controller_dispatch_end_events(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x170);
    frame.save(c, S0, 0x160);
    frame.save(c, S2, 0x140);
    c.g[S0][0] = 0;
    frame.save(c, S3, 0x130);
    let anim = r32(c, A0);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0x150);
    c.g[S3][0] = 0;
    frame.save(c, S4, 0x120);
    frame.save(c, S5, 0x110);
    frame.save_ra(c, 0x100);
    c.g[A0][0] = sx(r32(c, S0) << 3);
    guest(c, |c| {
        // Outer loop: i = 0..5.
        'seq_loop: loop {
            'no_nodes: {
                c.cycles += 9;
                let lists = lw(c, p(anim, 0x50));
                c.g[V0][0] = lists;
                c.g[S4][0] = c.g[A0][0];
                c.g[A1][0] = 0;
                c.g[A0][0] = sx((lists as u32).wrapping_add(r32(c, A0)));
                let index = c.g[S0][0];
                c.g[RA][0] = sx(0x0031_24d8);
                c.g[S5][0] = sx((index as u32).wrapping_add(1));
                callee::ANIM_SEQ_LIST_GET_AT.call(c, 0x0031_24d8)?;
                c.g[S0][0] = c.g[V0][0];
                let empty = c.g[S0][0] == 0;
                c.g[V0][0] = sx(r32(c, S3) << 3);
                if empty {
                    break 'no_nodes;
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, SP)));
                let mut node = r32(c, S0);
                c.g[A0][0] = sx(p(node, 0xb0));
                // Inner loop: walk this sequence's node chain.
                'node_loop: loop {
                    'step: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0031_24f8);
                        c.g[A1][0] = 0x3f;
                        call_ptr(c, callee::FLAGS64_TEST, 0x0031_24f8)?;
                        if c.g[V0][0] == 0 {
                            node = lw(c, p(node, 0xc8)) as u32;
                            c.g[S0][0] = sx(node);
                            break 'step;
                        }
                        c.cycles += 7;
                        let lists = lw(c, p(anim, 0x50));
                        c.g[V0][0] = lists;
                        let n = r32(c, S3).wrapping_add(1);
                        c.g[S3][0] = sx(n);
                        Addr(r32(c, S1)).sw(c, sx(node) as u64);
                        let seq = (lists as u32).wrapping_add(r32(c, S4));
                        c.g[V0][0] = sx(seq);
                        Addr(p(r32(c, S1), 4)).sw(c, sx(seq) as u64);
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(8));
                        node = lw(c, p(node, 0xc8)) as u32;
                        c.g[S0][0] = sx(node);
                        break 'step;
                    }
                    c.cycles += 2;
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = sx(p(r32(c, S0), 0xb0));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_24f0);
                        }
                        continue 'node_loop;
                    }
                    break 'no_nodes;
                }
            }
            c.cycles += 4;
            c.g[S0][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, S0) << 3);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_24c0);
                }
                continue 'seq_loop;
            }
            break;
        }
        // Dispatch.
        'dispatch: {
            c.cycles += 2;
            let none = (c.g[S3][0] as i64) <= 0;
            c.g[S1][0] = c.g[S3][0];
            if none {
                break 'dispatch;
            }
            c.cycles += 3;
            c.g[S0][0] = c.g[SP][0];
            let vt = lw(c, p(anim, 0x58));
            c.g[V0][0] = vt;
            loop {
                c.cycles += 10;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A1][0] = lw(c, p(r32(c, S0), 4));
                c.g[A0][0] = lh(c, p(vt as u32, 0x18));
                c.g[A2][0] = lw(c, p(r32(c, S0), 0));
                let f = lw(c, p(vt as u32, 0x1c));
                c.g[V1][0] = f;
                c.g[A0][0] = sx(anim.wrapping_add(c.g[A0][0] as u32));
                let target = f as u32;
                c.g[RA][0] = sx(0x0031_2568);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(8));
                call_ptr(c, target, 0x0031_2568)?;
                if c.g[S1][0] == 0 {
                    break 'dispatch;
                }
                let vt = lw(c, p(anim, 0x58));
                c.g[V0][0] = vt;
                if event_due(c) {
                    deliver_event_here(c, 0x0031_2548);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S0, 0x160);
        frame.restore_ra(c, 0x100);
    frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupRender (0x34b9b0).

/// `void func_0034B9B0(sFlagSetEntry* e)` (0x34b9b0): push a flag render
/// state (`D_00501420`) on the render context's stack, set its blend words
/// (`0xff000`/`0x14`, or `0x2c` when `e+3` is set), then for every model of
/// the group (`e+0x5c` count, `e+0x94` array) that is loaded
/// (`IsLoaded`: `+0x7d == 0xff`, or the two table lookups) cull the model's
/// box between `m+0x60` and `m+0x6c`; culled models are skipped, the others are
/// drawn either through the handler (`e+0x60`, slot 7) with the identity
/// matrix at `e+0x88`, or through the context's slot 96.
pub fn flag_manager_group_render(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x110);
    let manager = lw(c, gp(c, globals::FLAG_MANAGER));
    c.g[V0][0] = manager;
    frame.save(c, S4, 0xb0);
    frame.save(c, S0, 0xf0);
    let group = r32(c, A0);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S1, 0xe0);
    frame.save(c, S2, 0xd0);
    frame.save(c, S3, 0xc0);
    frame.save(c, S5, 0xa0);
    frame.save(c, S6, 0x90);
    frame.save(c, S7, 0x80);
    frame.save(c, FP, 0x70);
    frame.save_ra(c, 0x60);
    let t = manager == 0;
    frame.save_fpr(c, 20, 0x100);
    if t {
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        return;
    }
    guest(c, |c| {
        c.cycles += 30;
        let context = lw(c, gp(c, 0x2a90)) as u32;
        c.g[S3][0] = sx(context);
        c.g[A0][0] = sx(0x50_0000);
        let top = lw(c, p(context, ctx::TOP)) as u32;
        c.g[V0][0] = sx(top);
        copy_render_state(c, V1, A1, A2, top, top);
        let top = lw(c, p(context, ctx::TOP)) as u32;
        c.g[V1][0] = sx(top);
        c.g[V0][0] = sx(p(top, 0x14));
        let pushed = c.g[V0][0];
        Addr(p(context, ctx::TOP)).sw(c, pushed);
        c.g[A3][0] = globals::FLAG_STATE as u64;
        copy_render_state(c, V0, A1, A2, globals::FLAG_STATE, top);
        let flag = Addr(p(group, 3)).lb(c);
        c.g[V0][0] = flag;
        let plain = flag == 0;
        c.g[A3][0] = sx(lw(c, gp(c, 0x2a90)) as u32);
        if plain {
            c.cycles += 6;
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A0][0] = sx(a);
            c.g[V1][0] = sx(!0x7cu32);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
        } else {
            c.cycles += 16;
            c.g[V1][0] = 0xffff_ffff_fff0_0fff;
            c.g[A0][0] = sx(0xf_0000);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[A1][0] = sx(a);
            c.g[A0][0] = c.g[A0][0] | 0xf000;
            c.g[A2][0] = sx(!0x7cu32);
            c.g[V0][0] = lw(c, p(a, 0));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
            let v = c.g[V0][0];
            Addr(p(a, 0)).sw(c, v);
            let a = lw(c, p(context, ctx::TOP)) as u32;
            c.g[V1][0] = sx(a);
            c.g[V0][0] = lw(c, p(a, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] | 0x2c;
            let v = c.g[V0][0];
            Addr(p(a, 4)).sw(c, v);
        }
        // Flag group models.
        'models: {
            c.cycles += 3;
            let count = lw(c, p(group, 0x5c));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S5][0] = 0;
            if none {
                break 'models;
            }
            c.cycles += 8;
            c.g[V0][0] = sx(0x50_0000);
            c.g[AT][0] = sx(0x3f80_0000);
            c.mtc1(1, 20); // f20 = 1.0
            c.g[FP][0] = sx(0x50_0000u32.wrapping_sub(0xe60));
            c.g[S7][0] = sx(frame.at(c, 0x10));
            c.g[S6][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] = sx(r32(c, S5) << 2);
            let mut m = 0u32;
            loop {
                'loaded: {
                    'probe: {
                        c.cycles += 7;
                        c.g[V1][0] = lw(c, gp(c, globals::FLAG_TABLE));
                        c.g[V0][0] = sx(group.wrapping_add(r32(c, V0)));
                        c.g[A0][0] = 0xff;
                        m = lw(c, p(r32(c, V0), 0x94)) as u32;
                        c.g[S2][0] = sx(m);
                        let b = Addr(p(m, 0x7d)).lbu(c);
                        c.g[V0][0] = b;
                        let t = b != c.g[A0][0];
                        c.g[A1][0] = lw(c, p(r32(c, V1), 0));
                        if t {
                            break 'probe;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        break 'loaded;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, V0) << 3);
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x24));
                    c.g[V1][0] = c.g[V1][0] ^ 0x6;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0;
                        break 'loaded;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lh(c, p(m, 0x7e));
                    c.g[A0][0] = 0x18;
                    c.mult(0, V1, V0, A0);
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x3f0));
                    c.g[V1][0] = c.g[V1][0] ^ 0x3;
                    c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                    break 'loaded;
                }
                // Draw (or skip) this model.
                'draw: {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = lw(c, p(group, 0x5c));
                        break 'draw;
                    }
                    'culled: {
                        c.cycles += 32;
                        c.g[S1][0] = lw(c, p(context, ctx::VTABLE));
                        c.g[A0][0] = c.g[S6][0];
                        ldf(c, 0, p(m, 0x60));
                        c.g[S0][0] = lh(c, p(r32(c, S1), 0x2e8));
                        stf(c, 0, frame.at(c, 0));
                        c.g[S0][0] = sx(context.wrapping_add(r32(c, S0)));
                        ldf(c, 0, p(m, 0x64));
                        stf(c, 0, frame.at(c, 4));
                        ldf(c, 1, p(m, 0x68));
                        stf(c, 20, frame.at(c, 0xc));
                        stf(c, 1, frame.at(c, 8));
                        ldf(c, 0, p(m, 0x6c));
                        stf(c, 0, frame.at(c, 0x10));
                        ldf(c, 1, p(m, 0x70));
                        stf(c, 1, frame.at(c, 0x14));
                        ldf(c, 0, p(m, 0x74));
                        stf(c, 20, frame.at(c, 0x1c));
                        stf(c, 0, frame.at(c, 0x18));
                        // vt[43]: the camera matrix pair (hidden struct at sp+0x20).
                        let vt = lw(c, p(context, ctx::VTABLE)) as u32;
                        c.g[V0][0] = sx(vt);
                        c.g[A1][0] = lh(c, p(vt, 0x158));
                        let f43 = lw(c, p(vt, 0x15c));
                        c.g[V1][0] = f43;
                        let f43 = f43 as u32;
                        c.g[A1][0] = sx(context.wrapping_add(c.g[A1][0] as u32));
                        c.g[RA][0] = sx(0x0034_bba4);
                        call_ptr(c, f43, 0x0034_bba4)?;
                        // vt[93]: cull.
                        c.g[V0][0] = lw(c, p(r32(c, S1), 0x2ec));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S7][0];
                        let f93 = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0034_bbbc);
                        c.g[A3][0] = c.g[S6][0];
                        call_ptr(c, f93, 0x0034_bbbc)?;
                        c.g[V1][0] = sx(1);
                        let skip = c.g[V0][0] == c.g[V1][0];
                        c.g[A1][0] = sx(p(m, 0x10));
                        if skip {
                            break 'culled;
                        }
                        // Visible: draw with the identity matrix and the
                        // group's offset (`e+0x88`, `e+0x8c`).
                        'matrix: {
                            c.cycles += 14;
                            c.lqc2(1, r32(c, FP));
                            c.lqc2(2, p(r32(c, FP), 0x10));
                            c.lqc2(3, p(r32(c, FP), 0x20));
                            c.lqc2(4, p(r32(c, FP), 0x30));
                            c.sqc2(1, frame.at(c, 0));
                            c.sqc2(2, frame.at(c, 0x10));
                            c.sqc2(3, frame.at(c, 0x20));
                            c.sqc2(4, frame.at(c, 0x30));
                            ldf(c, 0, p(group, 0x88));
                            ldf(c, 1, p(group, 0x8c));
                            c.g[T0][0] = lw(c, p(group, 0x60));
                            stf(c, 0, frame.at(c, 0x30));
                            let no_handler = c.g[T0][0] == 0;
                            stf(c, 1, frame.at(c, 0x34));
                            if no_handler {
                                break 'matrix;
                            }
                            c.cycles += 9;
                            let handler = r32(c, T0);
                            c.g[V0][0] = lw(c, p(handler, 4));
                            c.g[A3][0] = c.g[SP][0];
                            c.g[A2][0] = lw(c, p(m, 0x94));
                            c.g[A0][0] = lh(c, p(r32(c, V0), 0x38));
                            let f = lw(c, p(r32(c, V0), 0x3c));
                            c.g[V1][0] = f;
                            let f = f as u32;
                            c.g[RA][0] = sx(0x0034_bc1c);
                            c.g[A0][0] = sx(handler.wrapping_add(r32(c, A0)));
                            call_ptr(c, f, 0x0034_bc1c)?;
                            c.g[V0][0] = lw(c, p(group, 0x5c));
                            break 'draw;
                        }
                        // No handler: draw through the context.
                        c.cycles += 9;
                        c.g[V1][0] = lw(c, p(context, ctx::VTABLE));
                        c.g[A1][0] = sx(m);
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        c.g[A0][0] = lh(c, p(r32(c, V1), 0x300));
                        c.g[T0][0] = 0;
                        let f = lw(c, p(r32(c, V1), 0x304));
                        c.g[V0][0] = f;
                        let f = f as u32;
                        c.g[RA][0] = sx(0x0034_bc48);
                        c.g[A0][0] = sx(context.wrapping_add(r32(c, A0)));
                        call_ptr(c, f, 0x0034_bc48)?;
                        break 'culled;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, p(group, 0x5c));
                    break 'draw;
                }
                c.cycles += 4;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, S5) << 2);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_bae8);
                    }
                    continue;
                }
                break 'models;
            }
        }
        c.cycles += 3;
        let top = lw(c, p(context, ctx::TOP));
        c.g[V0][0] = top;
        c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
        let top = c.g[V0][0];
        Addr(p(context, ctx::TOP)).sw(c, top);
        c.cycles += 13;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_RenderModel (0x122448).

/// `void Rider_RenderModel(Rider* r)` (0x122448): if the rider is on screen,
/// compute its LOD, reset the render state's blend words to `0x14`, draw the
/// rider model (`RiderModel_Render(model, lod, a, b)`) between two
/// identity-matrix pushes, then (re)compute the hidden flag `r+0xb1c` from the
/// world state, the on-screen flag and the LOD.
pub fn rider_render_model(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    let rider = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    guest(c, |c| {
        let on_screen = lw(c, p(rider, rider::ON_SCREEN));
        c.g[V0][0] = on_screen;
        let world = lw(c, gp(c, globals::WORLD));
        c.g[V0][0] = world;
        if on_screen != 0 {
            // b = q+0xa0 != 0 or sub-state >= 11; a = q+0xa4 != 0 or
            // sub-state >= 11 (q is only valid when the handler is set).
            let handler = lw(c, p(rider, rider::HANDLER));
            c.g[V0][0] = handler;
            let mut q = (handler as u32).wrapping_add(0xd30);
            c.g[V1][0] = sx(q);
            if handler == 0 {
                q = 0;
                c.g[V1][0] = 0;
            }
            let first = lw(c, p(q, 0xa0));
            c.g[A0][0] = first;
            c.cycles += 6;
            c.g[S2][0] = 0;
            let mut v0 = first;
            if first != 0 {
                c.cycles += 2;
                c.g[S2][0] = 1;
                v0 = lw(c, p(q, 0xa4));
                c.g[V0][0] = v0;
            } else {
                c.cycles += 4;
                let sub = lw(c, p(rider, rider::SUB_STATE));
                c.g[V0][0] = sub;
                c.g[V0][0] = ((sub as i64) < 11) as u64;
                if c.g[V0][0] != 0 {
                    v0 = lw(c, p(q, 0xa4));
                    c.g[V0][0] = v0;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                    v0 = lw(c, p(q, 0xa4));
                    c.g[V0][0] = v0;
                }
            }
            c.cycles += 2;
            let nonzero = v0 != 0;
            c.g[S1][0] = 0;
            if nonzero {
                c.cycles += 1;
                c.g[S1][0] = 1;
            } else {
                c.cycles += 4;
                let sub = lw(c, p(rider, rider::SUB_STATE));
                c.g[V0][0] = sub;
                c.g[V0][0] = ((sub as i64) < 11) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[S1][0] = 1;
                }
            }
            c.cycles += 40;
            c.mtc1(0, 20); // f20 = 0.0
            c.g[RA][0] = sx(0x0012_24c8);
            c.g[A0][0] = c.g[S0][0];
            callee::RIDER_COMPUTE_LOD.call(c, 0x0012_24c8)?;
            // rider->vt[69](rider->matrix, 1)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A3][0] = g;
            c.g[A2][0] = sx(1);
            c.g[A1][0] = sx(p(rider, rider::MATRIX));
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V0][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0x228));
            let f = lw(c, p(vt as u32, 0x22c));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_24e8);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_24e8)?;
            // rs[1] = (rs[1] & ~0x7c) | 0x14; vt[27](0.0, &rider->pos)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A2][0] = g;
            c.g[V1][0] = sx(!0x7cu32);
            c.mov_s(12, 20);
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            let rs = lw(c, p(g as u32, ctx::TOP));
            c.g[A0][0] = rs;
            c.g[V0][0] = lw(c, p(rs as u32, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            let v = c.g[V0][0];
            Addr(p(rs as u32, 4)).sw(c, v);
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V1][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0xd8));
            let f = lw(c, p(vt as u32, 0xdc));
            c.g[V0][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_2520);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_2520)?;
            // RiderModel_Render(model, lod, a, b)
            c.g[A0][0] = lw(c, p(rider, rider::MODEL));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, p(rider, rider::LOD));
            c.g[RA][0] = sx(0x0012_2534);
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, callee::RIDER_MODEL_RENDER, 0x0012_2534)?;
            // vt[27](0.0, &D_004FF120)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A2][0] = g;
            c.g[A1][0] = sx(0x50_0000);
            c.mov_s(12, 20);
            c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V0][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0xd8));
            let f = lw(c, p(vt as u32, 0xdc));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_2558);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_2558)?;
            let world = lw(c, gp(c, globals::WORLD));
            c.g[V0][0] = world;
        }
        // Hidden flag (`r+0xb1c`): recomputed when the world has no active
        // rider or the flag is already set.
        let world = r32(c, V0);
        let wm = lw(c, p(world, 0x84));
        c.g[V1][0] = wm;
        let race = lw(c, p(wm as u32, 0x84));
        c.g[A0][0] = race;
        let rider_id = lw(c, p(race as u32, 0x14));
        c.g[V0][0] = rider_id;
        c.cycles += 5;
        let mut v0 = rider_id;
        let mut early = false;
        if rider_id == 0 {
            v0 = lw(c, p(rider, rider::ON_SCREEN));
            c.g[V0][0] = v0;
        } else {
            c.cycles += 3;
            let hidden = lw(c, p(rider, rider::HIDDEN));
            c.g[V0][0] = hidden;
            if hidden == 0 {
                early = true;
            } else {
                c.cycles += 1;
                v0 = lw(c, p(rider, rider::ON_SCREEN));
                c.g[V0][0] = v0;
            }
        }
        if !early {
            c.cycles += 3;
            c.g[A0][0] = sx(p(rider, rider::HIDDEN));
            let off_screen = v0 == 0;
            c.g[A1][0] = 0;
            if off_screen {
                c.cycles += 2;
                c.g[A1][0] = 1;
                let v = c.g[A1][0];
                Addr(p(rider, rider::HIDDEN)).sw(c, v);
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(rider, rider::LOD));
                c.g[V0][0] = sx(2);
                if c.g[V1][0] != c.g[V0][0] {
                    let v = c.g[A1][0];
                    Addr(p(rider, rider::HIDDEN)).sw(c, v);
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    let v = c.g[A1][0];
                    Addr(p(rider, rider::HIDDEN)).sw(c, v);
                }
            }
            c.cycles += 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cTransitionMan_Render (0x2e47e8).

/// `void cTransitionMan_Render(cTransitionMan* t)` (0x2e47e8): only for the
/// transition of the active rider. Update the screen rectangle, retire a
/// pending fade reference, compute the fade amount (fade-in, hold, fade-out
/// between the two fade objects), clamp it to `[0, 1]`, reset the world
/// painter when it is nearly opaque, push a screen fade render state, set its
/// blend/texture words, call the fade object (`vt[2]`) and pop the state.
pub fn transition_man_render(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    let trans = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 22, 0x50);
    frame.save_fpr(c, 21, 0x48);
    frame.save_fpr(c, 20, 0x40);
    guest(c, |c| {
        'body: {
        let world = lw(c, gp(c, globals::WORLD));
        c.g[A1][0] = world;
        let wm = lw(c, p(world as u32, 0x84));
        c.g[V0][0] = wm;
        let rider = lw(c, p(trans, 0x74));
        c.g[A2][0] = rider;
        let race = lw(c, p(wm as u32, 0x84));
        c.g[V1][0] = race;
        let active = lw(c, p(race as u32, 0x14));
        c.g[A0][0] = active;
        let t = c.g[A2][0] != c.g[A0][0];
        frame.restore(c, S0, 0x30);
        if t {
            break 'body;
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x002e_4830);
        c.pc = 0x002e_b8f0;
        callee::TRANSITION_EFFECT_UPDATE_SCREEN_RECT.call(c, 0x002e_4830)?;
        let on = lw(c, p(trans, 0x44));
        c.g[V0][0] = on;
        let t = on == 0;
        let context = lw(c, gp(c, 0x2a90));
        c.g[S0][0] = context;
        if !t {
            // Pending fade reference.
            c.cycles += 3;
            let pending = lw(c, p(trans, 0x64));
            c.g[V0][0] = pending;
            let t = pending == 0;
            ldf(c, 20, p(trans, 0x48)); // f20 = time
            if t {
                // straight to the fade computation
            } else {
                c.cycles += 7;
                let fade_in = ldf(c, 0, p(trans, 0x54));
                let hold = ldf(c, 1, p(trans, 0x58));
                let sum = setf(c, 0, fade_in + hold);
                let reached = fcc(c, sum.le(fr(c, 20)));
                c.g[A1][0] = sx(p(trans, 0x68));
                if reached {
                    c.cycles += 6;
                    let vt = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[V0][0] = vt;
                    c.g[A0][0] = lh(c, p(vt as u32, 0x1e8));
                    let f = lw(c, p(vt as u32, 0x1ec));
                    c.g[V1][0] = f;
                    let f = f as u32;
                    c.g[RA][0] = sx(0x002e_4878);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x002e_4878)?;
                    Addr(p(trans, 0x64)).sw(c, 0);
                }
            }
            // Fade amount and target object.
            c.cycles += 3;
            let mut o = 0u32;
            let mut amount;
            let o1 = lw(c, p(trans, 0x4c));
            c.g[V0][0] = o1;
            if o1 == 0 {
                c.cycles += 9;
                let hold = ldf(c, 0, p(trans, 0x58));
                let fade_out = ldf(c, 2, p(trans, 0x5c));
                let diff = setf(c, 0, fr(c, 20) - hold);
                let one = li_f(c, 1, 0x3f80_0000);
                o = lw(c, p(trans, 0x50)) as u32;
                c.g[S2][0] = sx(o);
                let q = setf(c, 0, diff.div(fade_out));
                amount = setf(c, 0, one - q);
            } else {
                // f1 = fadeIn
                c.cycles += 3;
                let fade_in = ldf(c, 1, p(trans, 0x54));
                let in_ = fcc(c, fr(c, 20).lt(fade_in));
                if in_ {
                    c.cycles += 3;
                    let q = setf(c, 0, fr(c, 20).div(fr(c, 1)));
                    o = o1 as u32;
                    c.g[S2][0] = sx(o);
                    amount = q;
                } else {
                    c.cycles += 3;
                    let o2 = lw(c, p(trans, 0x50));
                    c.g[A0][0] = o2;
                    if o2 != 0 {
                        let hold = ldf(c, 0, p(trans, 0x58));
                        c.cycles += 5;
                        let sum = setf(c, 0, fr(c, 1) + hold);
                        let holding = fcc(c, fr(c, 20).lt(sum));
                        c.g[S2][0] = o1;
                        if holding {
                            c.cycles += 4;
                            amount = li_f(c, 0, 0x3f80_0000);
                            o = o1 as u32;
                            c.g[S2][0] = sx(o);
                        } else {
                            c.cycles += 7;
                            let diff = setf(c, 0, fr(c, 20) - sum);
                            let fade_out = ldf(c, 1, p(trans, 0x5c));
                            let one = li_f(c, 2, 0x3f80_0000);
                            c.g[S2][0] = o2;
                            o = o2 as u32;
                            let q = setf(c, 0, diff.div(fade_out));
                            amount = setf(c, 0, one - q);
                        }
                    } else {
                        c.cycles += 3;
                        let q = setf(c, 0, fr(c, 20).div(fr(c, 1)));
                        o = o1 as u32;
                        c.g[S2][0] = sx(o);
                        amount = q;
                    }
                }
            }
            // Clamp to [0, 1] into f22.
            c.cycles += 7;
            c.mtc1(0, 20); // f20 = 0.0
            c.g[AT][0] = sx(0x3f80_0000);
            c.mtc1(1, 21); // f21 = 1.0
            let nonneg = fcc(c, fr(c, 20).le(amount));
            c.mov_s(22, 20);
            if nonneg {
                c.cycles += 1;
                c.min_s(22, 0, 21);
            }
            // Nearly opaque: reset the world painter.
            c.cycles += 5;
            let threshold = ldf(c, 0, gp(c, -0x3a1c));
            let opaque = fcc(c, threshold.le(fr(c, 22)));
            if opaque {
                c.cycles += 3;
                call_ptr(c, callee::WORLD_PAINTER_QUERY_RESET_ALL, 0x002e_494c)?;
            }
            let top = lw(c, p(context as u32, ctx::TOP));
            c.g[V0][0] = top;
            // Push a copy of the render state and run the screen setup.
            'push: {
                c.cycles += 105;
                copy_render_state(c, V1, A0, A1, top as u32, top as u32);
                let nt = lw(c, p(context as u32, ctx::TOP));
                c.g[V1][0] = nt;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[V1][0] = sx((nt as u32).wrapping_add(0x14));
                Addr(p(context as u32, ctx::TOP)).sw(c, c.g[V1][0]);
                let vt = r32(c, V0);
                c.g[A0][0] = lh(c, p(vt, 0xa8));
                let f = lw(c, p(vt, 0xac));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4998);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4998)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xf8));
                let f = lw(c, p(r32(c, V0), 0xfc));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49ac);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_49ac)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0x128));
                let f = lw(c, p(r32(c, V0), 0x12c));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49c0);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_49c0)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.mov_s(12, 20);
                c.g[AT][0] = sx(0x4420_0000);
                c.mtc1(1, 14); // f14 = 640.0
                c.g[A1][0] = 0;
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xd0));
                c.g[A2][0] = 0;
                let f = lw(c, p(r32(c, V0), 0xd4));
                c.g[V1][0] = f;
                c.mov_s(17, 21);
                c.g[AT][0] = sx(0x43f0_0000);
                c.mtc1(1, 15); // f15 = 480.0
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                c.mov_s(13, 12);
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49fc);
                c.mov_s(16, 12);
                call_ptr(c, f, 0x002e_49fc)?;
                // Blend words of the pushed state.
                let vt = lw(c, p(context as u32, ctx::VTABLE));
                let top = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A3][0] = sx(top);
                c.g[A0][0] = 0xffff_ffff_ffff_fc1f;
                c.g[V1][0] = 0xffff_ffff_ffbf_ffff;
                c.g[T3][0] = 0x40_0000;
                c.g[V0][0] = lw(c, p(top, 8));
                c.g[A1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T4][0] = 0x100_0000;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.g[A2][0] = 0xffff_ffff_ffcf_ffff;
                Addr(p(top, 8)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0xffff_ffff_fff0_0fff;
                c.g[T0][0] = 0x1_4000;
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[T1][0] = sx(a);
                c.g[T5][0] = 0xffff_ffff_ffff_ff83;
                c.g[T6][0] = 0xffff_ffff_ffff_fffc;
                c.g[T2][0] = 0xffff_ffff_e000_03ff;
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[V1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[V1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 0));
                c.g[V0][0] = c.g[V0][0] | 0xc;
                Addr(p(a, 0)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                Addr(p(a, 8)).sw(c, c.g[V0][0]);
                let _ = vt;
                // Texture id.
                let use_tex = lw(c, p(trans, 0x60));
                c.g[V1][0] = use_tex;
                if use_tex == 0 {
                    let top = lw(c, p(context as u32, ctx::TOP));
                    c.g[V1][0] = top;
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    let top = c.g[V1][0];
                    c.sh(p(top as u32, 0x10), c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    let tex = lw(c, p(trans, 0x6c));
                    c.g[V1][0] = tex;
                    let top = lw(c, p(context as u32, ctx::TOP));
                    c.g[V0][0] = top;
                    c.sh(p(top as u32, 0x10), c.g[V1][0]);
                }
            }
            // Fade object: vt[2](o, amount) unless a pending tex fade owns it.
            'fade: {
                c.cycles += 3;
                let use_tex = lw(c, p(trans, 0x60));
                c.g[V0][0] = use_tex;
                let (vt, is_ctx) = if use_tex == 0 {
                    let vt = lw(c, o) as u32;
                    c.g[V0][0] = sx(vt);
                    (vt, false)
                } else {
                    c.cycles += 3;
                    let pending = lw(c, p(trans, 0x64));
                    c.g[V0][0] = pending;
                    if pending != 0 {
                        let vt = lw(c, p(context as u32, ctx::VTABLE)) as u32;
                        c.g[V0][0] = sx(vt);
                        (vt, true)
                    } else {
                        c.cycles += 1;
                        let vt = lw(c, o) as u32;
                        c.g[V0][0] = sx(vt);
                        (vt, false)
                    }
                };
                if !is_ctx {
                    // vt[2] of the fade object, amount in f12.
                    c.cycles += 6;
                    c.mov_s(12, 22);
                    c.g[A0][0] = lh(c, p(vt, 0x10));
                    let f = lw(c, p(vt, 0x14));
                    c.g[V1][0] = f;
                    let f = f as u32;
                    c.g[RA][0] = sx(0x002e_4b3c);
                    c.g[A0][0] = sx(o.wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x002e_4b3c)?;
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                }
                // vt[32](); vt[22](); top--.
                c.cycles += 12;
                c.g[A0][0] = lh(c, p(r32(c, V0), 0x100));
                let f = lw(c, p(r32(c, V0), 0x104));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4b50);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4b50)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xb0));
                let f = lw(c, p(r32(c, V0), 0xb4));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4b64);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4b64)?;
                let top = lw(c, p(context as u32, ctx::TOP));
                c.g[V0][0] = top;
                c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
                let top = c.g[V0][0];
                Addr(p(context as u32, ctx::TOP)).sw(c, top);
            }
            let _ = amount;
        }
        c.cycles += 1;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderModel_Render / RiderModel_RenderShadow.

/// One bone of the concatenation loop: `out = table_a[i] * table_b[i]`,
/// routed through `scratch` and then the stack exactly as the original does
/// (`sqc2` to `scratch`, copy to `stack`, copy to `out`).
fn bone_matrix(c: &mut Cpu, a: u32, b: u32, scratch: u32, stack: u32, out: u32) {
    concat_matrix(c, a, b, scratch);
    c.lqc2(1, scratch);
    c.lqc2(2, p(scratch, 0x10));
    c.lqc2(3, p(scratch, 0x20));
    c.lqc2(4, p(scratch, 0x30));
    c.sqc2(1, stack);
    c.sqc2(2, p(stack, 0x10));
    c.sqc2(3, p(stack, 0x20));
    c.sqc2(4, p(stack, 0x30));
    c.lqc2(1, stack);
    c.lqc2(2, p(stack, 0x10));
    c.lqc2(3, p(stack, 0x20));
    c.lqc2(4, p(stack, 0x30));
    c.sqc2(1, out);
    c.sqc2(2, p(out, 0x10));
    c.sqc2(3, p(out, 0x20));
    c.sqc2(4, p(out, 0x30));
}

/// `void RiderModel_Render(RiderModel* m, int lod, int x, int y)` (0x310640):
/// concatenate the per-bone rotation (`+0x34`) and translation (`+0x38`)
/// matrices into `D_004FC420`, then either draw every part with its own LOD
/// reference (`+0x18 != 0`: `refs[refIndex]`, blend x or y by part id,
/// `vt[111]` for the matrices) or draw the parts with the single LOD matrix
/// (`vt[111]` once, then `vt[99]` per part).
pub fn rider_model_render(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x120);
    let once = lw(c, gp(c, 0x2858));
    c.g[V0][0] = once;
    frame.save(c, S1, 0x100);
    frame.save(c, S2, 0xf0);
    let model = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S6, 0xb0);
    frame.save(c, S7, 0xa0);
    let y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let x = r32(c, A2);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S3, 0xe0);
    frame.save(c, S4, 0xd0);
    frame.save(c, S5, 0xc0);
    frame.save(c, FP, 0x90);
    frame.save_ra(c, 0x80);
    let t = once != 0;
    let context = lw(c, gp(c, 0x2a90));
    c.g[S2][0] = context;
    if !t {
        c.cycles += 5;
        c.g[V0][0] = sx(0x3f);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx(1);
        Addr(gp(c, 0x2858)).sw(c, 1);
    }
    guest(c, |c| {
        'body: {
        // Bone matrices.
        'bones: {
            c.cycles += 3;
            let bones = lw(c, p(model, model::NUM_BONES));
            c.g[V0][0] = bones;
            let none = (bones as i64) <= 0;
            c.g[A2][0] = 0;
            if none {
                break 'bones;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(0x50_0000);
            c.g[A3][0] = sx(frame.at(c, 0x40));
            c.g[T0][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
            c.g[T1][0] = c.g[SP][0];
            loop {
                c.cycles += 54;
                c.g[V1][0] = lw(c, p(model, model::BONE_TABLE_A));
                c.g[V0][0] = sx(r32(c, A2) << 6);
                c.g[A0][0] = lw(c, p(model, model::BONE_TABLE_B));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                bone_matrix(
                    c,
                    r32(c, V1),
                    r32(c, V0),
                    r32(c, A3),
                    r32(c, T1),
                    r32(c, T0),
                );
                c.g[V0][0] = lw(c, p(model, model::NUM_BONES));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x40));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_06b8);
                    }
                    continue;
                }
                break;
            }
        }
        // Parts.
        'parts: {
            c.cycles += 3;
            let has_parts = lw(c, p(model, model::HAS_PARTS));
            c.g[V0][0] = has_parts;
            let plain = has_parts == 0;
            c.g[V0][0] = sx(r32(c, A1) << 3);
            if plain {
                break 'parts;
            }
            c.cycles += 3;
            let count = lw(c, p(model, model::NUM_PARTS));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S4][0] = 0;
            if none {
                c.cycles += 1;
                break 'body;
            }
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, A1) << 3);
            c.g[FP][0] = sx(0x50_0000);
            c.g[S3][0] = 0;
            loop {
                'part: {
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                    let unused = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    if unused {
                        break 'part;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, p(r32(c, A0), model::PART_REF_INDEX));
                    c.g[A0][0] = lw(c, p(r32(c, V0), 0x1c));
                    c.g[V1][0] = sx(r32(c, V1) << 4);
                    let r = r32(c, A0).wrapping_add(r32(c, V1));
                    c.g[S0][0] = sx(r);
                    c.g[V0][0] = lw(c, p(r, 0));
                    let no_ref = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
                    if no_ref {
                        break 'part;
                    }
                    c.cycles += 24;
                    // vt[111](ctx, mats, 0, ref->unk_0C, ref->unk_08, 0)
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[A3][0] = lw(c, p(r, 0xc));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, p(r32(c, V0), 0x378));
                    c.g[T1][0] = 0;
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x37c));
                    c.g[T0][0] = lw(c, p(r, 8));
                    let f = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0031_0810);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x0031_0810)?;
                    // vt[99](ctx, ref->part, ref->data, id == 2 ? x : y, 0, scale.y)
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A3][0] = c.g[S7][0];
                    c.g[A1][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[T0][0] = 0;
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[A2][0] = lw(c, p(r, 4));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0));
                    c.g[A0][0] = lh(c, p(r32(c, A1), 0x318));
                    c.g[V0][0] = lw(c, p(r32(c, A1), 0x31c));
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    c.g[A1][0] = lw(c, p(r, 0));
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    ldf(c, 12, p(model, model::SCALE_Y));
                    let f = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0031_084c);
                    if c.g[V1][0] != 0 {
                        c.g[A3][0] = c.g[S6][0];
                    }
                    call_ptr(c, f, 0x0031_084c)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_07b8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'body;
            }
        }
        // No per-part references: one matrix set for the whole LOD.
        c.cycles += 17;
        c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
        c.g[A2][0] = sx(model.wrapping_add(r32(c, V0)));
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.g[A2][0];
        c.g[A0][0] = lh(c, p(r32(c, V1), 0x378));
        c.g[A3][0] = lw(c, p(r32(c, A2), 0x44));
        c.g[A1][0] = sx(0x50_0000);
        c.g[T0][0] = lw(c, p(r32(c, V0), 0x40));
        c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
        c.g[T2][0] = lw(c, p(r32(c, V1), 0x37c));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x3be0));
        c.g[A2][0] = 0;
        let f = c.g[T2][0] as u32;
        c.g[RA][0] = sx(0x0031_08a0);
        c.g[T1][0] = 0;
        call_ptr(c, f, 0x0031_08a0)?;
        let count = lw(c, p(model, model::NUM_PARTS));
        c.g[V0][0] = count;
        let none = (count as i64) <= 0;
        c.g[S0][0] = 0;
        if none {
            c.cycles += 1;
            break 'body;
        }
        c.cycles += 1;
        c.g[S3][0] = 0;
        loop {
            'part: {
                c.cycles += 6;
                c.g[V0][0] = lw(c, p(model, model::PARTS));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                let unused = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0];
                if unused {
                    break 'part;
                }
                c.cycles += 4;
                c.g[A2][0] = lw(c, p(r32(c, V0), 0x1c));
                c.g[A1][0] = lw(c, p(r32(c, A2), 0));
                let no_ref = c.g[A1][0] == 0;
                c.g[T0][0] = 0;
                if no_ref {
                    break 'part;
                }
                c.cycles += 11;
                c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A3][0] = c.g[S7][0];
                c.g[V0][0] = lw(c, p(r32(c, A0), 0));
                c.g[A0][0] = lh(c, p(r32(c, V1), 0x318));
                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                c.g[T1][0] = lw(c, p(r32(c, V1), 0x31c));
                c.g[A2][0] = lw(c, p(r32(c, A2), 4));
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                ldf(c, 12, p(model, model::SCALE_Y));
                let f = c.g[T1][0] as u32;
                c.g[RA][0] = sx(0x0031_0904);
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S6][0];
                }
                call_ptr(c, f, 0x0031_0904)?;
            }
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_08b0);
                }
                continue;
            }
            break;
        }
        c.cycles += 1;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderModel_RenderShadow(RiderModel* m, int lod, Vec4* a, Vec4* b)`
/// (0x310948): as [`rider_model_render`] but with the shadow bone rotation
/// table (`+0x30`), the shadow matrix array (`D_004FD420`) and the shadow
/// draw entry (slot 100, `id - 1 < 2` selects x else y).
pub fn rider_model_render_shadow(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0x120);
    let once = lw(c, gp(c, 0x285c));
    c.g[V0][0] = once;
    frame.save(c, S1, 0x100);
    frame.save(c, S2, 0xf0);
    let model = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S6, 0xb0);
    frame.save(c, S7, 0xa0);
    let y = r32(c, A3);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0x110);
    let x = r32(c, A2);
    c.g[S7][0] = c.g[A2][0];
    frame.save(c, S3, 0xe0);
    frame.save(c, S4, 0xd0);
    frame.save(c, S5, 0xc0);
    frame.save(c, FP, 0x90);
    frame.save_ra(c, 0x80);
    let t = once != 0;
    let context = lw(c, gp(c, 0x2a90));
    c.g[S2][0] = context;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = sx(0x3f);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx(1);
        Addr(gp(c, 0x285c)).sw(c, 1);
    }
    guest(c, |c| {
        'body: {
        'bones: {
            c.cycles += 4;
            let bones = lw(c, p(model, model::NUM_BONES));
            c.g[V0][0] = bones;
            let none = (bones as i64) <= 0;
            c.g[A2][0] = 0;
            if none {
                break 'bones;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(0x50_0000);
            c.g[A3][0] = sx(frame.at(c, 0x40));
            c.g[T0][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
            c.g[T1][0] = c.g[SP][0];
            loop {
                c.cycles += 54;
                c.g[V1][0] = lw(c, p(model, model::BONE_TABLE_A_SHADOW));
                c.g[V0][0] = sx(r32(c, A2) << 6);
                c.g[A0][0] = lw(c, p(model, model::BONE_TABLE_B));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                bone_matrix(
                    c,
                    r32(c, V1),
                    r32(c, V0),
                    r32(c, A3),
                    r32(c, T1),
                    r32(c, T0),
                );
                c.g[V0][0] = lw(c, p(model, model::NUM_BONES));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x40));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_09b8);
                    }
                    continue;
                }
                break;
            }
        }
        'parts: {
            c.cycles += 3;
            let has_parts = lw(c, p(model, model::HAS_PARTS));
            c.g[V0][0] = has_parts;
            let plain = has_parts == 0;
            c.g[V0][0] = sx(r32(c, A1) << 3);
            if plain {
                break 'parts;
            }
            c.cycles += 3;
            let count = lw(c, p(model, model::NUM_PARTS));
            c.g[V0][0] = count;
            let none = (count as i64) <= 0;
            c.g[S4][0] = 0;
            if none {
                c.cycles += 1;
                break 'body;
            }
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, A1) << 3);
            c.g[FP][0] = sx(0x50_0000);
            c.g[S3][0] = 0;
            loop {
                'part: {
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                    let unused = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S5)));
                    if unused {
                        break 'part;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, p(r32(c, A0), model::PART_REF_INDEX));
                    c.g[A0][0] = lw(c, p(r32(c, V0), 0x1c));
                    c.g[V1][0] = sx(r32(c, V1) << 4);
                    let r = r32(c, A0).wrapping_add(r32(c, V1));
                    c.g[S0][0] = sx(r);
                    c.g[V0][0] = lw(c, p(r, 0));
                    let no_ref = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
                    if no_ref {
                        break 'part;
                    }
                    c.cycles += 23;
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[A3][0] = lw(c, p(r, 0xc));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, p(r32(c, V0), 0x378));
                    c.g[T1][0] = 0;
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0x37c));
                    c.g[T0][0] = lw(c, p(r, 8));
                    let f = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0031_0b10);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x0031_0b10)?;
                    // vt[100](ctx, ref->part, ref->data, m)
                    c.g[V0][0] = lw(c, p(model, model::PARTS));
                    c.g[A3][0] = c.g[S7][0];
                    c.g[A1][0] = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    c.g[A2][0] = lw(c, p(r, 4));
                    c.g[V1][0] = lw(c, p(r32(c, V0), 0));
                    c.g[A0][0] = lh(c, p(r32(c, A1), 0x320));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = lw(c, p(r32(c, A1), 0x324));
                    c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                    c.g[A1][0] = lw(c, p(r, 0));
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    let f = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0031_0b48);
                    if c.g[V1][0] == 0 {
                        c.g[A3][0] = c.g[S6][0];
                    }
                    call_ptr(c, f, 0x0031_0b48)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0ab8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'body;
            }
        }
        // No per-part references: one matrix set for the whole LOD.
        c.cycles += 17;
        c.g[V1][0] = lw(c, p(context as u32, ctx::VTABLE));
        c.g[A2][0] = sx(model.wrapping_add(r32(c, V0)));
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.g[A2][0];
        c.g[A0][0] = lh(c, p(r32(c, V1), 0x378));
        c.g[A3][0] = lw(c, p(r32(c, A2), 0x44));
        c.g[A1][0] = sx(0x50_0000);
        c.g[T0][0] = lw(c, p(r32(c, V0), 0x40));
        c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
        c.g[T2][0] = lw(c, p(r32(c, V1), 0x37c));
        c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0x2be0));
        c.g[A2][0] = 0;
        let f = c.g[T2][0] as u32;
        c.g[RA][0] = sx(0x0031_0b9c);
        c.g[T1][0] = 0;
        call_ptr(c, f, 0x0031_0b9c)?;
        let count = lw(c, p(model, model::NUM_PARTS));
        c.g[V0][0] = count;
        let none = (count as i64) <= 0;
        c.g[S0][0] = 0;
        if none {
            c.cycles += 1;
            break 'body;
        }
        c.cycles += 2;
        c.g[S3][0] = 0;
        loop {
            let mut no_ref_tail = false;
            'part: {
                c.cycles += 6;
                c.g[V0][0] = lw(c, p(model, model::PARTS));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[V0][0] = lw(c, p(r32(c, V1), model::PART_LODS));
                let unused = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0];
                if unused {
                    break 'part;
                }
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(r32(c, V0), 0x1c));
                c.g[A1][0] = lw(c, p(r32(c, V1), 0));
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
                    no_ref_tail = true;
                    break 'part;
                }
                c.cycles += 11;
                c.g[A3][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[V0][0] = lw(c, p(r32(c, A0), 0));
                c.g[A2][0] = lw(c, p(r32(c, V1), 4));
                c.g[A0][0] = lh(c, p(r32(c, A3), 0x320));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[V1][0] = lw(c, p(r32(c, A3), 0x324));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                c.g[A3][0] = c.g[S7][0];
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                let f = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0031_0c04);
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[S6][0];
                }
                call_ptr(c, f, 0x0031_0c04)?;
            }
            if !no_ref_tail {
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(model, model::NUM_PARTS));
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x58));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_0bb0);
                }
                continue;
            }
            break;
        }
        c.cycles += 1;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, put_vec, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 160;

    /// Fixed fixture areas, above the range `fakes::ptr` hands out (which is
    /// `DATA..DATA+0x100000`) and clear of the fake call log at 0x900000.
    const CTX: u32 = DATA + 0x20_0000;
    const TOP: u32 = DATA + 0x22_0000;
    const TEX: u32 = DATA + 0x23_0000;
    const WORLD: u32 = DATA + 0x24_0000;
    const AM_CTX: u32 = DATA + 0x25_0000;
    const AM_TOP: u32 = DATA + 0x26_0000;
    const BASE: u32 = DATA + 0x30_0000;
    const MODEL: u32 = DATA + 0x40_0000;
    const EXTRA: u32 = DATA + 0x50_0000;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }

    /// A render context object (vtable, render-state stack and a few random
    /// state words).
    fn ctx_at(c: &mut Cpu, r: &mut Rng, obj: u32, top: u32, slots: &[u32]) -> u32 {
        let vt = vtable(c, r, slots);
        c.bus.write32(obj + ctx::VTABLE, vt);
        c.bus.write32(obj + ctx::TOP, top);
        for i in 0..8 {
            c.bus.write32(top + 4 * i, r.next_u32());
        }
        vt
    }

    /// `gp+0x2a90` (the game's render context) with a texture table.
    fn put_ctx(c: &mut Cpu, r: &mut Rng, slots: &[u32]) -> u32 {
        c.bus.write32(GAME_GP + 0x2a90, CTX);
        c.bus.write32(CTX + ctx::TEX_IDS, TEX);
        ctx_at(c, r, CTX, TOP, slots)
    }

    /// The world chain `gp-0x848 -> +0x84 -> +0x84 -> +0x14`.
    fn put_world(c: &mut Cpu, r: &mut Rng, rider: u32) -> u32 {
        let (wm, race) = (WORLD, WORLD + 0x1000);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, wm + 0x100);
        c.bus.write32(wm + 0x100 + 0x84, race);
        c.bus.write32(race + 0x14, rider);
        let _ = r;
        rider
    }

    /// A `RiderModel` for the two model renderers: bone tables, optional parts
    /// with their LOD references, and the LOD info words.
    fn put_render_model(c: &mut Cpu, r: &mut Rng, base: u32, seed: u32, shadow: bool) {
        let bones = if seed % 3 == 0 { 0 } else { 2 };
        let parts = if seed % 4 == 0 { 0 } else { 2 };
        c.bus.write32(base + model::NUM_BONES, bones);
        c.bus.write32(base + model::NUM_PARTS, parts);
        c.bus.write32(base + model::HAS_PARTS, (seed % 5 != 0) as u32);
        c.bus.write32(base + model::PARTS, base + 0x8000);
        c.bus.write32(base + model::SCALE_Y, uf(r, 0.5, 2.0));
        for lod in 0..2u32 {
            c.bus.write32(base + model::LOD_INFO + 8 * lod, r.next_u32());
            c.bus.write32(base + model::LOD_INFO + 8 * lod + 4, r.next_u32());
        }
        let a = base + 0x1000;
        let b = base + 0x2000;
        c.bus.write32(
            base + if shadow { model::BONE_TABLE_A_SHADOW } else { model::BONE_TABLE_A },
            a,
        );
        c.bus.write32(base + model::BONE_TABLE_B, b);
        for i in 0..2u32 {
            for k in 0..16 {
                c.bus.write32(a + 0x40 * i + 4 * k, r.next_u32());
                c.bus.write32(b + 0x40 * i + 4 * k, r.next_u32());
            }
        }
        let lod = seed % 2;
        for i in 0..parts {
            let part = base + 0x8000 + model::PART_STRIDE * i;
            let used = ((seed + i) % 3 != 0) as u32;
            let refs = base + 0x9000 + 0x100 * i;
            c.bus.write32(part + model::PART_LODS + 8 * lod, used);
            c.bus.write32(part + model::PART_LODS + 8 * lod + 4, refs);
            c.bus.write32(part + model::PART_REF_INDEX, (seed + i) % 2);
            c.bus.write32(part, (seed + i) % 4); // part id
            for k in 0..2u32 {
                let rec = refs + 16 * k;
                c.bus.write32(rec, if (seed + k) % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(rec + 4, r.next_u32());
                c.bus.write32(rec + 8, r.next_u32());
                c.bus.write32(rec + 0xc, r.next_u32());
            }
        }
    }

    #[test]
    fn particle_modifier_update_matches() {
        check(ee::s11::f_00345b40__generated, particle_modifier_update, SEEDS, &|c, seed| {
            machine(c, seed, 0x5b40, 0);
            let m = BASE;
            c.bus.write32(m + 0x1e4, (seed % 5 == 0) as u32);
            c.bus.write32(
                m + 0x50,
                match seed % 4 {
                    0 => f(-1.0),
                    1 => 0,
                    2 => f(0.5),
                    _ => f(-0.25),
                },
            );
            // The emitter descriptor: zero burst budget, so Emitter_Update
            // returns without its generation loop.
            for i in 0..8 {
                c.bus.write32(m + 0x60 + 4 * i, 0);
            }
            for off in [0x180u32, 0x184, 0x188] {
                c.bus.write32(m + off, 0);
            }
            c.bus.write32(GAME_GP + 0x2a74, BASE + 0x1000);
            c.bus.write32(BASE + 0x1010, 60); // particles per second
            c.g[A0][0] = sx(m);
        });
    }

    #[test]
    fn m_emitter_render_matches() {
        check(ee::s12::f_00371688__generated, m_emitter_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1688, 1);
            put_ctx(c, &mut r, &[0x2a0]);
            let e = BASE;
            c.bus.write32(e + 0x174, (seed % 5 != 0) as u32);
            c.bus.write32(e + 0x1e0, (seed % 3 != 0) as u32);
            c.bus.write32(e + 4, 0); // base
            c.bus.write32(e + 8, r.next_u32() % 8); // kind
            c.bus.write32(e + 0x10, if seed % 4 == 0 { f(1.5) } else { 0 });
            for i in 0..5 {
                c.bus.write32(globals::EMITTER_BLEND + 4 * i, r.next_u32() % 32);
            }
            for i in 0..3u32 {
                let tex = if seed % 7 == 3 && i == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 0x1000
                };
                c.bus.write32(TEX + 4 * i, tex);
            }
            c.bus.write32(e + 0x1e8, r.next_u32());
            c.bus.write32(e + 0x1ec, r.next_u32());
            for i in 0..5 {
                c.bus.write32(e + 0x1e4 + 4 * i, r.next_u32());
            }
            c.bus.write32(e + 0x1f4, r.next_u32());
            c.g[A0][0] = sx(e);
            c.g[A1][0] = sx(seed % 4);
        });
    }

    #[test]
    fn emitter_render_matches() {
        check(ee::s12::f_003708c0__generated, emitter_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x08c0, 3);
            put_ctx(c, &mut r, &[0x158, 0x2e8, 0x290]);
            let e = BASE;
            c.bus.write32(e + 0xc, (seed % 6 != 0) as u32);
            c.bus.write32(e + 4, 0);
            c.bus.write32(e + 8, r.next_u32() % 8);
            c.bus.write32(e + 0x184, if seed % 5 == 0 { f(1.5) } else { 0 });
            for i in 0..3u32 {
                c.bus.write32(TEX + 4 * i, r.next_u32() % 0x1000);
            }
            for off in [0x1e8u32, 0x1ec, 0x1f4] {
                c.bus.write32(e + off, r.next_u32());
            }
            for i in 0..5 {
                c.bus.write32(e + 0x1e4 + 4 * i, r.next_u32());
            }
            for i in 0..5 {
                c.bus.write32(globals::EMITTER_BLEND + 4 * i, r.next_u32() % 32);
            }
            // The cull call is the second fake; forcing 1 means "culled".
            for k in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(e);
        });
    }

    #[test]
    fn player_pulsefx_update_matches() {
        check(ee::s09::f_002e39d8__generated, player_pulsefx_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39d8, 0);
            let pfx = BASE;
            let rider = BASE + 0x1_0000;
            let vt = BASE + 0x2_0000;
            c.bus.write32(pfx, rider);
            c.bus.write32(rider + 0x6c0, vt);
            if seed % 4 == 0 {
                // A real function that returns 0 (its argument word is zero).
                c.bus.write16(vt + 0x40, 0x100);
                c.bus.write32(vt + 0x44, 0x002c_c098);
                c.bus.write32(rider + 0x7c0, 0);
            } else {
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, FAKE);
            }
            c.bus.write32(pfx + 4, uf(&mut r, -10.0, 10.0));
            for i in 0..8 {
                c.bus.write32(pfx + 8 + 4 * i, uf(&mut r, -2.0, 2.0));
            }
            // The pulse rate and the angle constants.
            c.bus.write32(GAME_GP + 0x24bc, uf(&mut r, 0.0, 20.0));
            c.bus.write32(GAME_GP - 0x3a30, f(1.0 / 60.0));
            c.bus.write32(GAME_GP - 0x3a2c, f(0.159_154_94));
            c.bus.write32(GAME_GP - 0x3a28, f(6.283_185_5));
            c.g[A0][0] = sx(pfx);
        });
    }

    #[test]
    fn anim_controller_dispatch_end_events_matches() {
        check(ee::s10::f_00312490__generated, anim_controller_dispatch_end_events, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2490, 1);
            let anim = BASE;
            let lists = BASE + 0x1_0000;
            let nodes = BASE + 0x2_0000;
            c.bus.write32(anim + 0x50, lists);
            let vt = vtable(c, &mut r, &[0x18]);
            c.bus.write32(anim + 0x58, vt);
            for i in 0..6u32 {
                let len = (seed / (i + 1)) % 3;
                let mut head = 0u32;
                for k in 0..len {
                    let n = nodes + 0x100 * (i * 3 + k);
                    c.bus.write32(n + 0xc8, head);
                    c.bus.write32(n + 0xb0, r.next_u32());
                    c.bus.write32(
                        n + 0xb4,
                        if (seed + k + i) % 3 == 0 { 0x8000_0000 } else { 0 },
                    );
                    head = n;
                }
                c.bus.write32(lists + 8 * i, r.next_u32());
                c.bus.write32(lists + 8 * i + 4, head);
            }
            c.g[A0][0] = sx(anim);
        });
    }

    #[test]
    fn flag_manager_group_render_matches() {
        check(ee::s11::f_0034b9b0__generated, flag_manager_group_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xb9b0, 5);
            put_ctx(c, &mut r, &[0x158, 0x2e8, 0x300]);
            c.bus.write32(GAME_GP.wrapping_add(globals::FLAG_MANAGER as u32), (seed % 8 != 0) as u32);
            for i in 0..5 {
                c.bus.write32(globals::FLAG_STATE + 4 * i, r.next_u32());
            }
            for i in 0..4u32 {
                let mut m = [0u32; 4];
                m[i as usize] = f(1.0);
                put_vec(c, globals::IDENTITY_MAT + 16 * i, m);
            }
            // The model-loaded table: `base + idx*8 + 0x24` and
            // `base + field*0x18 + 0x3f0`.
            let table = BASE + 0x3_0000;
            let mbase = BASE + 0x4_0000;
            c.bus.write32(GAME_GP.wrapping_add(globals::FLAG_TABLE as u32), table);
            c.bus.write32(table, mbase);
            for idx in 0..4u32 {
                c.bus.write32(mbase + 8 * idx + 0x24, if idx == 1 { 0 } else { 6 });
            }
            for idx in 0..4u32 {
                c.bus.write32(mbase + 0x3f0 + 0x18 * idx, if idx < 2 { 3 } else { 0 });
            }
            let group = BASE;
            let count = seed % 3;
            c.bus.write32(group + 0x5c, count);
            c.bus.write8(group + 3, (seed % 4 == 1) as u8);
            c.bus.write32(group + 0x88, uf(&mut r, -10.0, 10.0));
            c.bus.write32(group + 0x8c, uf(&mut r, -10.0, 10.0));
            if seed % 5 == 0 {
                c.bus.write32(group + 0x60, 0);
            } else {
                let h = BASE + 0x5_0000;
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(h + 4, vt);
                c.bus.write32(group + 0x60, h);
            }
            for i in 0..count {
                let m = BASE + 0x1_0000 + 0x100 * i;
                c.bus.write32(group + 0x94 + 4 * i, m);
                let mode = (seed + i) % 4;
                let loaded_byte = match mode {
                    0 => 0xff,
                    1 => 1,
                    _ => 2,
                };
                c.bus.write8(m + 0x7d, loaded_byte);
                c.bus.write16(m + 0x7e, if mode == 3 { 2 } else { 1 });
                for k in 0..6 {
                    c.bus.write32(m + 0x60 + 4 * k, uf(&mut r, -5.0, 5.0));
                }
                c.bus.write32(m + 0x94, r.next_u32());
            }
            for k in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(group);
        });
    }

    #[test]
    fn transition_man_render_matches() {
        check(ee::s09::f_002e47e8__generated, transition_man_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x47e8, 7);
            put_ctx(c, &mut r, &[0x1e8, 0xa8, 0xf8, 0x128, 0xd0, 0x100, 0xb0, 0x150]);
            let trans = BASE;
            let rider = 7;
            let mismatch = seed % 8 == 0;
            put_world(c, &mut r, if mismatch { rider + 1 } else { rider });
            c.bus.write32(trans + 0x74, rider);
            c.bus.write32(trans + 0x44, (seed % 8 != 1) as u32);
            // Fade objects with a `vt[2]` entry.
            let o1 = BASE + 0x1_0000;
            let o2 = BASE + 0x2_0000;
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(o1, vt);
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(o2, vt);
            // time / fadeIn / hold / fadeOut chosen to take every branch.
            let (time, fade_in, hold, fade_out, use_o1, use_o2) = match seed % 5 {
                0 => (0.5, 1.0, 2.0, 1.0, false, true),  // o1 == 0
                1 => (0.5, 1.0, 2.0, 1.0, true, true),   // t < fadeIn
                2 => (1.5, 1.0, 2.0, 1.0, true, false),  // o2 == 0
                3 => (1.5, 1.0, 2.0, 1.0, true, true),   // hold
                _ => (10.0, 1.0, 2.0, 1.0, true, true),  // fade out (clamped)
            };
            c.bus.write32(trans + 0x48, f(time));
            c.bus.write32(trans + 0x4c, if use_o1 { o1 } else { 0 });
            c.bus.write32(trans + 0x50, if use_o2 { o2 } else { 0 });
            c.bus.write32(trans + 0x54, f(fade_in));
            c.bus.write32(trans + 0x58, f(hold));
            c.bus.write32(trans + 0x5c, f(fade_out));
            c.bus.write32(trans + 0x64, (seed % 3 == 0) as u32);
            c.bus.write32(trans + 0x68, EXTRA);
            c.bus.write32(trans + 0x60, (seed % 2) as u32);
            c.bus.write32(trans + 0x6c, r.next_u32() % 0x1000);
            for off in [0x70u32, 0x78, 0x7c, 0x80] {
                c.bus.write32(trans + off, r.next_u32());
            }
            // The near-opaque threshold (0 or 2.0).
            c.bus.write32(
                GAME_GP - 0x3a1c,
                if seed % 4 == 0 { 0 } else { f(2.0) },
            );
            c.g[A0][0] = sx(trans);
        });
    }

    #[test]
    fn rider_render_model_matches() {
        check(ee::s00::f_00122448__generated, rider_render_model, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2448, 8);
            // The AM render context (`gp-0x854`) and its vtable.
            c.bus.write32(GAME_GP - 0x854, AM_CTX);
            c.bus.write32(AM_CTX + ctx::TEX_IDS, TEX);
            ctx_at(c, &mut r, AM_CTX, AM_TOP, &[0x228, 0xd8, 0x118, 0xe8]);
            // The model renderer's own context (`gp+0x2a90`).
            put_ctx(c, &mut r, &[0x378, 0x318]);
            let rider = BASE;
            let on_screen = (seed % 5 != 0) as u32;
            let hidden = (seed % 3 == 0) as u32;
            c.bus.write32(rider + rider::ON_SCREEN, on_screen);
            c.bus.write32(rider + rider::HIDDEN, hidden);
            c.bus.write32(rider + rider::HANDLER, if seed % 4 == 0 { 0 } else { rider + 0x3000 });
            let q = rider + 0x3000 + 0xd30;
            c.bus.write32(q + 0xa0, (seed % 3 == 1) as u32);
            c.bus.write32(q + 0xa4, (seed % 6 == 1) as u32);
            c.bus.write32(rider + rider::SUB_STATE, (seed % 14) as u32);
            c.bus.write32(rider + rider::LOD, seed % 3);
            c.bus.write32(rider + 0x74, 7);
            c.bus.write32(rider + rider::MATRIX, rider + 0x4000);
            for i in 0..4 {
                c.bus.write32(rider + rider::POSITION + 4 * i, uf(&mut r, -100.0, 100.0));
            }
            // Rider vtable for Rider_ComputeLOD (`r+0x6c0`, slot 5).
            let vt = vtable(c, &mut r, &[0x28]);
            c.bus.write32(rider + 0x6c0, vt);
            // A cheap model: no bones, no parts (the real RiderModel_Render
            // then only does its LOD-matrix call).
            let model = MODEL;
            c.bus.write32(rider + rider::MODEL, model);
            c.bus.write32(model + model::NUM_BONES, 0);
            c.bus.write32(model + model::NUM_PARTS, 0);
            c.bus.write32(model + model::HAS_PARTS, 0);
            c.bus.write32(model + model::LOD_INFO, r.next_u32());
            c.bus.write32(model + model::SCALE_Y, f(1.0));
            // The active rider handle: equal on most seeds.
            put_world(c, &mut r, if seed % 6 == 0 { 8 } else { 7 });
            c.g[A0][0] = sx(rider);
        });
    }

    #[test]
    fn rider_model_render_matches() {
        check(ee::s10::f_00310640__generated, rider_model_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0640, 3);
            put_ctx(c, &mut r, &[0x378, 0x318]);
            c.bus.write32(GAME_GP + 0x2858, (seed % 6 == 0) as u32);
            let m = MODEL;
            put_render_model(c, &mut r, m, seed, false);
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(seed % 2);
            c.g[A2][0] = sx(1 + seed % 3);
            c.g[A3][0] = sx(4 + seed % 5);
        });
    }

    #[test]
    fn rider_model_render_shadow_matches() {
        check(ee::s10::f_00310948__generated, rider_model_render_shadow, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0948, 3);
            put_ctx(c, &mut r, &[0x378, 0x320]);
            c.bus.write32(GAME_GP + 0x285c, (seed % 6 == 0) as u32);
            let m = MODEL;
            put_render_model(c, &mut r, m, seed, true);
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(seed % 2);
            c.g[A2][0] = sx(1 + seed % 3);
            c.g[A3][0] = sx(4 + seed % 5);
        });
    }
}
