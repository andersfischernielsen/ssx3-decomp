//! `src/render/graphicsman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beintplayer::lh as lh_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::ui::uiengine::oc391_at;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::ldf_2 as ldf;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw;
pub(crate) use crate::game::visualfx::boardwakefx::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0036_9a78,
        ps2_graphics_man_match_blended_matrix,
        "PS2GraphicsMan_MatchBlendedMatrix",
    ),
    (
        0x0036_9c28,
        ps2_graphics_man_wait_for_render_idle,
        "PS2GraphicsMan_WaitForRenderIdle",
    ),
    (
        0x0036_9ff8,
        graphics_man_snapshot_texture_table,
        "GraphicsMan_SnapshotTextureTable",
    ),
    (
        0x0036_a020,
        graphics_man_free_all_textures,
        "GraphicsMan_FreeAllTextures",
    ),
    (0x0036_a1c0, deferred_delete_push, "DeferredDelete_Push"),
    (0x0036_a2c0, ps2_graphics_man_v94, "PS2GraphicsMan_v94"),
    (
        0x0036_aa60,
        ps2_graphics_man_emit_depth_fog_clut_upload,
        "PS2GraphicsMan_EmitDepthFogClutUpload",
    ),
    (
        0x0036_aba0,
        ps2_graphics_man_reset_depth_fog_params,
        "PS2GraphicsMan_ResetDepthFogParams",
    ),
    (
        0x0036_ce00,
        emitter_set_desc_curve_a,
        "Emitter_SetDescCurveA",
    ),
    (
        0x0036_ce28,
        emitter_set_desc_curve_b,
        "Emitter_SetDescCurveB",
    ),
    (
        0x0036_cef8,
        emitter_set_desc_path_coeffs,
        "Emitter_SetDescPathCoeffs",
    ),
    (
        0x0036_d008,
        emitter_reset_desc_accel,
        "Emitter_ResetDescAccel",
    ),
    (
        0x0036_d1f0,
        emitter_set_desc_path_coeffs_scaled,
        "Emitter_SetDescPathCoeffsScaled",
    ),
    (0x0036_d3d8, emitter_set_desc_time, "Emitter_SetDescTime"),
    (
        0x0036_d3e8,
        emitter_advance_desc_time,
        "Emitter_AdvanceDescTime",
    ),
    (
        0x0036_d500,
        emitter_compute_desc_bounds,
        "Emitter_ComputeDescBounds",
    ),
    (0x0037_0758, emitter_dtor, "Emitter_dtor"),
    (0x0037_0788, emitter_update, "Emitter_Update"),
    (0x0037_0888, emitter_render_at, "Emitter_RenderAt"),
    (0x0037_08c0, emitter_render, "Emitter_Render"),
    (0x0037_0af8, emitter_load, "Emitter_Load"),
    (0x0037_0c08, dynamic_emitter_dtor, "DynamicEmitter_dtor"),
    (
        0x0036_d428,
        emitter_update_descriptor,
        "Emitter_UpdateDescriptor",
    ),
    (
        0x0036_a1e8,
        deferred_delete_advance_frame,
        "DeferredDelete_AdvanceFrame",
    ),
    (
        0x0036_c740,
        ps2_graphics_man_reset_bloom_params,
        "PS2GraphicsMan_ResetBloomParams",
    ),
    (
        0x0037_0058,
        emitter_build_descriptor,
        "Emitter_BuildDescriptor",
    ),
    (0x0037_0aa8, emitter_save, "Emitter_Save"),
    (
        0x0036_ccb8,
        emitter_randomize_desc_seeds,
        "Emitter_RandomizeDescSeeds",
    ),
    (
        0x0036_cbf8,
        emitter_init_descriptor,
        "Emitter_InitDescriptor",
    ),
    (
        0x0036_d318,
        emitter_set_desc_velocity_curve,
        "Emitter_SetDescVelocityCurve",
    ),
    (0x0037_05e0, emitter_setup, "Emitter_Setup"),
    (0x0036_a3b0, morton_spread8, "Morton_Spread8"),
    (0x0037_0018, emitter_ctor, "Emitter_ctor"),
    (0x0037_0b60, dynamic_emitter_ctor, "DynamicEmitter_ctor"),
    (
        0x0036_a190,
        graphics_man_destroy_grid_mesh,
        "GraphicsMan_DestroyGridMesh",
    ),
    (
        0x0036_98e0,
        ps2_graphics_man_alloc_blended_matrix_history,
        "PS2GraphicsMan_AllocBlendedMatrixHistory",
    ),
    (
        0x0036_a290,
        deferred_delete_flush_all,
        "DeferredDelete_FlushAll",
    ),
    (
        0x0036_9cf8,
        graphics_man_draw_indexed_triangles,
        "GraphicsMan_DrawIndexedTriangles",
    ),
    (
        0x0036_9ff0,
        graphics_man_empty_hook,
        "GraphicsMan_EmptyHook",
    ),
    (
        0x0036_a428,
        ps2_graphics_man_build_depth_fog_clut,
        "PS2GraphicsMan_BuildDepthFogClut",
    ),
    (
        0x0036_ae20,
        ps2_graphics_man_emit_depth_fog_z_copy,
        "PS2GraphicsMan_EmitDepthFogZCopy",
    ),
    (
        0x0036_b158,
        ps2_graphics_man_emit_depth_fog_blend,
        "PS2GraphicsMan_EmitDepthFogBlend",
    ),
    (
        0x0036_b5e8,
        ps2_graphics_man_emit_depth_fog_flat_fill,
        "PS2GraphicsMan_EmitDepthFogFlatFill",
    ),
    (
        0x0036_b9d8,
        ps2_graphics_man_emit_blur_downsample,
        "PS2GraphicsMan_EmitBlurDownsample",
    ),
    (
        0x0036_c188,
        ps2_graphics_man_emit_blended_fill,
        "PS2GraphicsMan_EmitBlendedFill",
    ),
    (
        0x0036_c398,
        ps2_graphics_man_emit_bloom_composite,
        "PS2GraphicsMan_EmitBloomComposite",
    ),
    (
        0x0036_ce80,
        emitter_set_desc_linear_path,
        "Emitter_SetDescLinearPath",
    ),
    (
        0x0036_cfa8,
        emitter_set_desc_accel_scaled,
        "Emitter_SetDescAccelScaled",
    ),
];

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TEX_IDS;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {
    #[cfg(test)]
    pub(crate) use crate::game::visualfx::boardwakefx::globals::EMITTER_BLEND;
}

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {

    pub(crate) use crate::game::ai::rider::callee::EMITTER_ADVANCE_DESC_TIME;

    pub(crate) use crate::game::ai::rider::callee::EMITTER_UPDATE_DESCRIPTOR;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;

    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;

    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER as EMITTER_RENDER_2;

    pub(crate) use crate::game::render::particle::callee::DEFERRED_DELETE_ADVANCE_FRAME;
}

// ---------------------------------------------------------------------------
// Emitters.

/// Emitter (descriptor) fields used by `Emitter_Update`.
pub(crate) mod emitter {

    /// Set once the emitter has been (re)started / finished this frame.
    pub const STARTED: u32 = 0xc;

    /// The remaining life is at +0; then the progress accumulator and its
    /// limit, and the rate `dt` is scaled by.
    pub const PROGRESS: u32 = 0x184;

    pub const RATE: u32 = 0x188;

    pub const LIMIT: u32 = 0x180;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
}

/// `void DeferredDelete_Push(void *obj)` (0x36a1c0): append `obj` to the
/// deferred-delete ring at 0x53ab40 indexed by the count at `gp + 0x2a98`,
/// linking it to the previous head.
pub fn deferred_delete_push(c: &mut Cpu) {
    c.cycles += 9;
    const RING: u32 = 0x54_0000 - 0x54c0;
    let count = Addr((c.g[GP][0] as u32).wrapping_add(0x2a98)).lw(c);
    c.g[A1][0] = count;
    c.g[V0][0] = 0x54_0000;
    c.g[V0][0] = sx(0x54_0000u32.wrapping_sub(0x54c0));
    c.g[A1][0] = sx((count as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let slot = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(slot).lw(c);
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    Addr(slot).sw(c, c.g[A0][0]);
    debug_assert_eq!(slot, RING.wrapping_add((count as u32).wrapping_shl(2)));
    abi::ret(c);
}

/// `void PS2GraphicsMan_ResetDepthFogParams(PS2GraphicsMan *man, uint index)`
/// (0x36aba0): fill the 0x20-byte depth-fog record for `index` from the
/// `gp + 0x123c` defaults, copying the 0x10-byte table entry at 0x547230 in
/// after the first four words.
pub fn ps2_graphics_man_reset_depth_fog_params(c: &mut Cpu) {
    c.cycles += 23;
    let gp = c.g[GP][0] as u32;
    let f1 = Addr(gp.wrapping_add(0x123c)).read_fpu(c);
    c.f[1] = f1.0;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 5);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c64));
    c.g[V1][0] = 0x50_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, f1);
    let f0 = Addr(gp.wrapping_add(0x1240)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0x4).write_fpu(c, f0);
    c.g[V0][0] = Addr(gp.wrapping_add(0x1244)).lw(c);
    Addr(dst).off(0x8).sw(c, c.g[V0][0]);
    let f0 = Addr(gp.wrapping_add(0x1248)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0xc).write_fpu(c, f0);
    c.g[A3][0] = sx(0x50_0000u32.wrapping_add(0x4720));
    let src = c.g[A3][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(5, src.wrapping_add(0xf));
    c.ldr(5, src.wrapping_add(8));
    c.sdl(2, dst.wrapping_add(0x17));
    c.sdr(2, dst.wrapping_add(0x10));
    c.sdl(5, dst.wrapping_add(0x1f));
    c.sdr(5, dst.wrapping_add(0x18));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Emitters

/// `void Emitter_SetDescTime(Emitter *e, float time)` (0x36d3d8): set the
/// current description time to `time * rate` (rate at +0x4c).
pub fn emitter_set_desc_time(c: &mut Cpu) {
    c.cycles += 4;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[0] = rate.0;
    let result = abi::farg(c, 0) * rate;
    c.f[12] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}

/// `void Emitter_AdvanceDescTime(Emitter *e, float dt)` (0x36d3e8): advance
/// the description time at +0x8 by `dt * rate` (rate at +0x4c).
pub fn emitter_advance_desc_time(c: &mut Cpu) {
    c.cycles += 6;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[1] = rate.0;
    let current = emitter.off(0x8).read_fpu(c);
    c.f[0] = current.0;
    let delta = abi::farg(c, 0) * rate;
    c.f[12] = delta.0;
    let result = current + delta;
    c.f[0] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}

/// `lh vt+off` (the vtable slot's `this` adjustment) and `lw vt+off+4`
/// (its entry point).
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u32, off: u32) -> (u64, u64) {
    (lh(c, p(vt, off)), lw(c, p(vt, off + 4)))
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

/// The frame clock object (`D_004A5B64`, `gp+0x2a74`); `+0x10` is the frame
/// rate as an integer.
pub(crate) const CLOCK: i32 = 0x2a74;

/// `void Emitter_Update(Emitter* e, float dt /*f12*/)` (0x370788): run the
/// emitter's countdown; when it has run out (a negative remaining time) the
/// descriptor is restarted, otherwise its clock is advanced and its remaining
/// life reduced, and the progress accumulator is advanced by `rate * dt`,
/// wrapping to 0 once it reaches the limit. Marks the emitter started.
///
/// State lives in the original's registers: f21 the constant 0, f22 `dt`.
pub fn emitter_update(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 22, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x20);
        c.mtc1(0, 21);
        c.lwc1(0, r32(c, S0));
        c.c_eq_s(0, 21);
        let dead = c.fcc();
        c.mov_s(22, 12);
        if !dead {
            c.cycles += 4;
            c.c_lt_s(0, 21);
            let expired = c.fcc();
            c.g[V0][0] = c.lw(gp(c, CLOCK));
            if expired {
                // Restart the descriptor: one frame's worth of time.
                c.cycles += 9;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 0, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_UPDATE_DESCRIPTOR.call(c, 0x0037_07e8)?;
                c.lwc1(0, at(c, S0, emitter::RATE));
            } else {
                // Advance its clock, then its remaining life.
                c.cycles += 17;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 20);
                c.lwc1(12, at(c, V0, 0x10));
                c.cvt_s_w(12, 12);
                c.div_s(12, 20, 12);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::EMITTER_ADVANCE_DESC_TIME.call(c, 0x0037_080c)?;
                c.g[V0][0] = c.lw(gp(c, CLOCK));
                c.lwc1(0, r32(c, S0));
                c.lwc1(1, at(c, V0, 0x10));
                c.cvt_s_w(1, 1);
                c.div_s(20, 20, 1);
                c.sub_s(0, 0, 20);
                c.c_lt_s(0, 21);
                let negative = c.fcc();
                c.swc1(0, r32(c, S0));
                if negative {
                    c.cycles += 1;
                    c.swc1(21, r32(c, S0));
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, emitter::RATE));
            }
            // progress += rate * dt, back to 0 once it reaches the limit.
            c.cycles += 9;
            c.lwc1(1, at(c, S0, emitter::PROGRESS));
            c.mul_s(0, 0, 22);
            c.g[V1][0] = c.lw(at(c, S0, emitter::LIMIT));
            c.add_s(1, 1, 0);
            c.cvt_w_s(0, 1);
            c.mfc1(2, 0);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let running = c.g[V0][0] != 0;
            c.swc1(1, at(c, S0, emitter::PROGRESS));
            if !running {
                c.cycles += 1;
                c.sw(at(c, S0, emitter::PROGRESS), 0);
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw(at(c, S0, emitter::STARTED), c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `D_004FF120`: the constant vector (0, 0, 0, 1) an emitter's acceleration is
/// reset to.
pub(crate) const ZERO_ACCEL: u32 = 0x004f_f120;

// ---------------------------------------------------------------------------
// Render, UI and world accessors.

/// `void Emitter_ResetDescAccel(Emitter* e)` (0x36d008): copies the constant
/// (0, 0, 0, 1) vector `D_004FF120` into the descriptor's acceleration at
/// +0xe0.
pub fn emitter_reset_desc_accel(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x50_0000;
    c.g[V0][0] = sx(ZERO_ACCEL);
    let src = c.g[V0][0] as u32;
    c.lq(3, src);
    let selfp = abi::arg(c, 0);
    c.sq(3, selfp.wrapping_add(0xe0));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Emitter_SetDescCurveB.

/// `void Emitter_SetDescCurveB(Emitter* e, float a, float b)` (0x36ce28):
/// scale the descriptor curve by the emitter's rate (`+0x4c`) and store the
/// two ramps: `+0x14 = (b - a) * s`, `+0x1c = a * s - (b - a) * s`.
pub fn emitter_set_desc_curve_b(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    // f13 = b, f12 = a (the EABI float argument order).
    c.sub_s(13, 13, 12);
    c.lwc1(0, this.wrapping_add(0x4c));
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.sub_s(12, 12, 13);
    c.swc1(13, this.wrapping_add(0x14));
    c.swc1(12, this.wrapping_add(0x1c));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Emitter curve.

/// `void Emitter_SetDescCurveA(EmitterDesc *d, float a, float b, float c)`
/// (0x36ce00): store the third value, the `b - a` span, and `a - 1.5 * c` into
/// the curve descriptor at `+0x10`, `+0x20` and `+0x18`.
pub fn emitter_set_desc_curve_a(c: &mut Cpu) {
    c.cycles += 9;
    let base = c.g[A0][0] as u32;
    let a = FpuF32(c.f[12]);
    let b = FpuF32(c.f[13]);
    let c3 = FpuF32(c.f[14]);
    c.g[AT][0] = 0x3fc0_0000; // 1.5
    c.mtc1(AT, 0);
    c.f[13] = (b - a).0; // span into f13
    Addr(base.wrapping_add(0x10)).write_fpu(c, c3);
    c.f[0] = (c3 * FpuF32(c.f[0])).0; // 1.5 * c
    Addr(base.wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[13]));
    c.f[12] = (a - FpuF32(c.f[0])).0; // a - 1.5 * c
    Addr(base.wrapping_add(0x18)).write_fpu(c, FpuF32(c.f[12]));
    abi::ret(c);
}

/// `void GraphicsMan_SnapshotTextureTable(GraphicsMan* self)` (0x369ff8):
/// call slot 0xf2 of the current texture table (`+0x10d8`).
pub fn graphics_man_snapshot_texture_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at_2(table, 0x3c8)).lh(c);
        c.g[A1][0] = Addr(at_2(table, 0x3cc)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a014)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GraphicsMan_FreeAllTextures(GraphicsMan* self)` (0x36a020): call
/// slot 0xf1 of the texture table.
pub fn graphics_man_free_all_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(abi::arg(c, 0), 0x10d8)).lw(c);
        let table = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at_2(table, 0x3c0)).lh(c);
        c.g[A1][0] = Addr(at_2(table, 0x3c4)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0036_a03c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `Emitter` destroying destructor (0x370758): the particle emitter's base
/// vtable lives at `self + 0x18c`.
pub fn emitter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x0037_077c, 0x0049_3160, 0x18c)
    })
}

// ---------------------------------------------------------------------------
// Graphics.

/// `void Emitter_RenderAt(Emitter* e, sMPBox* box, sMPVec4* center)`
/// (0x370888): copy the box and centre into the descriptor's quad slots and
/// render.
pub fn emitter_render_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let self_ = abi::arg(c, 0);
        let bbox = abi::arg(c, 1);
        let center = abi::arg(c, 2);
        c.lq(3, bbox);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(self_);
        c.sq(3, self_.wrapping_add(0x160));
        c.lq(2, bbox.wrapping_add(0x10));
        c.sq(2, self_.wrapping_add(0x170));
        c.lq(3, center);
        c.sq(3, self_.wrapping_add(0xc0));
        callee::EMITTER_RENDER_2.call(c, 0x0037_08b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicEmitter_dtor(DynamicEmitter* e, int flags)` (0x370c08): set
/// the vtable, free the emitter's data and `delete this` for bit 0.
pub fn dynamic_emitter_dtor(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0049_3118);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p_2(c, S1, 0x1f8), c.g[V0][0]);
    c.g[A1][0] = 1;
    if callee::DYNAMIC_EMITTER_FREE_DATA
        .call(c, 0x0037_0c34)
        .is_err()
    {
        return;
    }
    c.g[S0][0] &= 1;
    let keep = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x20);
    if keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, table::DELETE, 0x0037_0c48).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `Emitter_Load(Emitter* self, void* stream)` (0x370af8): read the 12-byte
/// record (`f0`, `f18`, `c`) through the stream vtable and compute the
/// descriptor bounds.
pub fn emitter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        let stream = c.g[A1][0];
        c.g[A3][0] = stream;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A2][0] = 0xc;
        c.g[V1][0] = lw_2(c, stream as u32);
        c.g[A0][0] = lh_2(c, p(c.g[V1][0] as u32, 0x10));
        c.g[V0][0] = lw_2(c, p(c.g[V1][0] as u32, 0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0037_0b28)?;
        c.lwc1(1, sp(c));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x10));
        c.lwc1(0, p(sp(c), 4));
        c.g[A1][0] = sx((this as u32).wrapping_add(0x160));
        c.swc1(1, this as u32);
        c.swc1(0, p(this as u32, 0x18));
        call_ptr(c, 0x0036_d500, 0x0037_0b44)?;
        c.g[V0][0] = lw_2(c, p(sp(c), 8));
        frame.restore_ra(c, 0x10);
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void Emitter_ComputeDescBounds(const Desc* d, Bounds* out)` (0x36d500):
/// derive the emitter descriptor's world-space bounding box (min at `out+0`,
/// max at `out+0x10`) and its extent from the descriptor's vectors at
/// `+0x60`..`+0xef`, folding in the reference scale/offset vectors at
/// `gp-0x2814`.. and the existing `out` values.
pub fn emitter_compute_desc_bounds(c: &mut Cpu) {
    'b_36d52c: {
        c.cycles += 10;
        c.lwc1(0, at(c, A0, 0x14));
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xb0));
        c.lwc1(1, at(c, A0, 0x1c));
        c.add_s(2, 0, 0);
        c.add_s(0, 1, 0);
        c.add_s(1, 1, 2);
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.mov_s(3, 1);
        if t {
            break 'b_36d52c;
        } else {
            c.cycles += 1;
            c.mov_s(3, 0);
            break 'b_36d52c;
        }
    }
    'b_36d5d0: {
        c.cycles += 41;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, c.g[SP][0] as u32);
        c.sq(2, at(c, A1, 0x10));
        c.sq(2, c.g[A1][0] as u32);
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36d5d0;
        } else {
            break 'b_36d5d0;
        }
    }
    'b_36d5e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d5e8;
        } else {
            break 'b_36d5e8;
        }
    }
    'b_36d600: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d600;
        } else {
            break 'b_36d600;
        }
    }
    'b_36d618: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d618;
        } else {
            break 'b_36d618;
        }
    }
    'b_36d630: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d630;
        } else {
            break 'b_36d630;
        }
    }
    'b_36d648: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d648;
        } else {
            break 'b_36d648;
        }
    }
    'b_36d6ac: {
        c.cycles += 25;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x20));
        c.lq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36d6ac;
        } else {
            break 'b_36d6ac;
        }
    }
    'b_36d6c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d6c4;
        } else {
            break 'b_36d6c4;
        }
    }
    'b_36d6dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d6dc;
        } else {
            break 'b_36d6dc;
        }
    }
    'b_36d6f4: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d6f4;
        } else {
            break 'b_36d6f4;
        }
    }
    'b_36d70c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d70c;
        } else {
            break 'b_36d70c;
        }
    }
    'b_36d724: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d724;
        } else {
            break 'b_36d724;
        }
    }
    'b_36d7a4: {
        c.cycles += 32;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36d7a4;
        } else {
            break 'b_36d7a4;
        }
    }
    'b_36d7bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d7bc;
        } else {
            break 'b_36d7bc;
        }
    }
    'b_36d7d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d7d4;
        } else {
            break 'b_36d7d4;
        }
    }
    'b_36d7ec: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d7ec;
        } else {
            break 'b_36d7ec;
        }
    }
    'b_36d804: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d804;
        } else {
            break 'b_36d804;
        }
    }
    'b_36d81c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d81c;
        } else {
            break 'b_36d81c;
        }
    }
    'b_36d8bc: {
        c.cycles += 40;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, A0, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, c.g[A0][0] as u32);
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36d8bc;
        } else {
            break 'b_36d8bc;
        }
    }
    'b_36d8d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d8d4;
        } else {
            break 'b_36d8d4;
        }
    }
    'b_36d8ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36d8ec;
        } else {
            break 'b_36d8ec;
        }
    }
    'b_36d904: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36d904;
        } else {
            break 'b_36d904;
        }
    }
    'b_36d91c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36d91c;
        } else {
            break 'b_36d91c;
        }
    }
    'b_36d934: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36d934;
        } else {
            break 'b_36d934;
        }
    }
    'b_36d9d4: {
        c.cycles += 40;
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, A0, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x30));
        c.lq(2, at(c, SP, 0x30));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.lwc1(0, c.g[A0][0] as u32);
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36d9d4;
        } else {
            break 'b_36d9d4;
        }
    }
    'b_36d9ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36d9ec;
        } else {
            break 'b_36d9ec;
        }
    }
    'b_36da04: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36da04;
        } else {
            break 'b_36da04;
        }
    }
    'b_36da1c: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36da1c;
        } else {
            break 'b_36da1c;
        }
    }
    'b_36da34: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36da34;
        } else {
            break 'b_36da34;
        }
    }
    'b_36da4c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36da4c;
        } else {
            break 'b_36da4c;
        }
    }
    'b_36db08: {
        c.cycles += 47;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xc0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.lqc2(3, at(c, A0, 0xb0));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x40));
        c.lq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x20));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xd0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, SP, 0x20));
        c.lqc2(4, at(c, SP, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, c.g[A0][0] as u32);
        c.cvt_s_w(0, 0);
        c.sq(2, at(c, SP, 0x10));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, A0, 0xe0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.lq(2, at(c, SP, 0x60));
        c.sq(2, c.g[SP][0] as u32);
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, c.g[A1][0] as u32);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36db08;
        } else {
            break 'b_36db08;
        }
    }
    'b_36db20: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36db20;
        } else {
            break 'b_36db20;
        }
    }
    'b_36db38: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36db38;
        } else {
            break 'b_36db38;
        }
    }
    'b_36db50: {
        c.cycles += 6;
        c.lwc1(1, c.g[SP][0] as u32);
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36db50;
        } else {
            break 'b_36db50;
        }
    }
    'b_36db68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x4));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36db68;
        } else {
            break 'b_36db68;
        }
    }
    'b_36db80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x8));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36db80;
        } else {
            break 'b_36db80;
        }
    }
    'b_36dc5c: {
        c.cycles += 55;
        c.lq(3, at(c, A1, 0x10));
        c.lq(2, c.g[A1][0] as u32);
        c.sq(3, at(c, SP, 0x10));
        c.sq(2, c.g[SP][0] as u32);
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.sq(2, at(c, SP, 0x30));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dc5c;
        } else {
            break 'b_36dc5c;
        }
    }
    'b_36dc74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dc74;
        } else {
            break 'b_36dc74;
        }
    }
    'b_36dc8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dc8c;
        } else {
            break 'b_36dc8c;
        }
    }
    'b_36dca4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dca4;
        } else {
            break 'b_36dca4;
        }
    }
    'b_36dcbc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dcbc;
        } else {
            break 'b_36dcbc;
        }
    }
    'b_36dcd4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dcd4;
        } else {
            break 'b_36dcd4;
        }
    }
    'b_36dd50: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36dd50;
        } else {
            break 'b_36dd50;
        }
    }
    'b_36dd68: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36dd68;
        } else {
            break 'b_36dd68;
        }
    }
    'b_36dd80: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36dd80;
        } else {
            break 'b_36dd80;
        }
    }
    'b_36dd98: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36dd98;
        } else {
            break 'b_36dd98;
        }
    }
    'b_36ddb0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36ddb0;
        } else {
            break 'b_36ddb0;
        }
    }
    'b_36ddc8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36ddc8;
        } else {
            break 'b_36ddc8;
        }
    }
    'b_36de44: {
        c.cycles += 31;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36de44;
        } else {
            break 'b_36de44;
        }
    }
    'b_36de5c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36de5c;
        } else {
            break 'b_36de5c;
        }
    }
    'b_36de74: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36de74;
        } else {
            break 'b_36de74;
        }
    }
    'b_36de8c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36de8c;
        } else {
            break 'b_36de8c;
        }
    }
    'b_36dea4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dea4;
        } else {
            break 'b_36dea4;
        }
    }
    'b_36debc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36debc;
        } else {
            break 'b_36debc;
        }
    }
    'b_36df54: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, A0, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36df54;
        } else {
            break 'b_36df54;
        }
    }
    'b_36df6c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36df6c;
        } else {
            break 'b_36df6c;
        }
    }
    'b_36df84: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36df84;
        } else {
            break 'b_36df84;
        }
    }
    'b_36df9c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36df9c;
        } else {
            break 'b_36df9c;
        }
    }
    'b_36dfb4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36dfb4;
        } else {
            break 'b_36dfb4;
        }
    }
    'b_36dfcc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36dfcc;
        } else {
            break 'b_36dfcc;
        }
    }
    'b_36e064: {
        c.cycles += 38;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, A0, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e064;
        } else {
            break 'b_36e064;
        }
    }
    'b_36e07c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e07c;
        } else {
            break 'b_36e07c;
        }
    }
    'b_36e094: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e094;
        } else {
            break 'b_36e094;
        }
    }
    'b_36e0ac: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e0ac;
        } else {
            break 'b_36e0ac;
        }
    }
    'b_36e0c4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e0c4;
        } else {
            break 'b_36e0c4;
        }
    }
    'b_36e0dc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e0dc;
        } else {
            break 'b_36e0dc;
        }
    }
    'b_36e174: {
        c.cycles += 38;
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, A0, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x70));
        c.lq(2, at(c, SP, 0x70));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.lwc1(0, at(c, SP, 0x20));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e174;
        } else {
            break 'b_36e174;
        }
    }
    'b_36e18c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e18c;
        } else {
            break 'b_36e18c;
        }
    }
    'b_36e1a4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e1a4;
        } else {
            break 'b_36e1a4;
        }
    }
    'b_36e1bc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e1bc;
        } else {
            break 'b_36e1bc;
        }
    }
    'b_36e1d4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e1d4;
        } else {
            break 'b_36e1d4;
        }
    }
    'b_36e1ec: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e1ec;
        } else {
            break 'b_36e1ec;
        }
    }
    'b_36e2a0: {
        c.cycles += 45;
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x70));
        c.lqc2(3, at(c, A0, 0x70));
        c.lqc2(4, at(c, SP, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x80));
        c.lq(2, at(c, SP, 0x80));
        c.sq(2, at(c, SP, 0x60));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x60));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 2);
        c.lqc2(4, at(c, A0, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x50));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x40));
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x20));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x20));
            break 'b_36e2a0;
        } else {
            break 'b_36e2a0;
        }
    }
    'b_36e2b8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x24));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x24));
            break 'b_36e2b8;
        } else {
            break 'b_36e2b8;
        }
    }
    'b_36e2d0: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x28));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x28));
            break 'b_36e2d0;
        } else {
            break 'b_36e2d0;
        }
    }
    'b_36e2e8: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x40));
        c.lwc1(0, at(c, SP, 0x30));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x30));
            break 'b_36e2e8;
        } else {
            break 'b_36e2e8;
        }
    }
    'b_36e300: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x44));
        c.lwc1(0, at(c, SP, 0x34));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x34));
            break 'b_36e300;
        } else {
            break 'b_36e300;
        }
    }
    'b_36e318: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x48));
        c.lwc1(0, at(c, SP, 0x38));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, SP, 0x38));
            break 'b_36e318;
        } else {
            break 'b_36e318;
        }
    }
    'b_36e3e4: {
        c.cycles += 51;
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x50));
        c.lwc1(1, gp(c, -0x2868));
        c.lq(2, at(c, SP, 0x50));
        c.lwc1(0, gp(c, -0x2864));
        c.min_s(1, 3, 1);
        c.sq(2, at(c, SP, 0x40));
        c.lqc2(3, at(c, A0, 0x60));
        c.lqc2(4, at(c, SP, 0x20));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x60));
        c.mul_s(0, 1, 0);
        c.lq(2, at(c, SP, 0x60));
        c.lwc1(2, gp(c, -0x2860));
        c.sq(2, at(c, SP, 0x50));
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.add_s(0, 0, 2);
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.mul_s(1, 1, 0);
        c.lqc2(3, c.g[SP][0] as u32);
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, c.g[A1][0] as u32);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, c.g[A1][0] as u32);
            break 'b_36e3e4;
        } else {
            break 'b_36e3e4;
        }
    }
    'b_36e3fc: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e3fc;
        } else {
            break 'b_36e3fc;
        }
    }
    'b_36e414: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e414;
        } else {
            break 'b_36e414;
        }
    }
    'b_36e42c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e42c;
        } else {
            break 'b_36e42c;
        }
    }
    'b_36e444: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e444;
        } else {
            break 'b_36e444;
        }
    }
    'b_36e45c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e45c;
        } else {
            break 'b_36e45c;
        }
    }
    'b_36e4dc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, c.g[SP][0] as u32);
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, c.g[A1][0] as u32);
            break 'b_36e4dc;
        } else {
            break 'b_36e4dc;
        }
    }
    'b_36e4f4: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e4f4;
        } else {
            break 'b_36e4f4;
        }
    }
    'b_36e50c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e50c;
        } else {
            break 'b_36e50c;
        }
    }
    'b_36e524: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e524;
        } else {
            break 'b_36e524;
        }
    }
    'b_36e53c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e53c;
        } else {
            break 'b_36e53c;
        }
    }
    'b_36e554: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e554;
        } else {
            break 'b_36e554;
        }
    }
    'b_36e5d4: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x40));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(2, at(c, SP, 0x60));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, c.g[A1][0] as u32);
            break 'b_36e5d4;
        } else {
            break 'b_36e5d4;
        }
    }
    'b_36e5ec: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x4));
            break 'b_36e5ec;
        } else {
            break 'b_36e5ec;
        }
    }
    'b_36e604: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x8));
            break 'b_36e604;
        } else {
            break 'b_36e604;
        }
    }
    'b_36e61c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x10));
            break 'b_36e61c;
        } else {
            break 'b_36e61c;
        }
    }
    'b_36e634: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x14));
            break 'b_36e634;
        } else {
            break 'b_36e634;
        }
    }
    'b_36e64c: {
        c.cycles += 6;
        c.lwc1(2, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 2);
        if c.fcc() {
            c.swc1(2, at(c, A1, 0x18));
            break 'b_36e64c;
        } else {
            break 'b_36e64c;
        }
    }
    'b_36e6cc: {
        c.cycles += 32;
        c.mfc1(3, 3);
        c.lqc2(4, at(c, A0, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x80));
        c.lqc2(3, at(c, SP, 0x10));
        c.lqc2(4, at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0x90));
        c.lq(2, at(c, SP, 0x90));
        c.sq(2, at(c, SP, 0x70));
        c.mfc1(3, 1);
        c.lqc2(4, at(c, SP, 0x50));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.sq(2, at(c, SP, 0x90));
        c.lqc2(3, at(c, SP, 0x70));
        c.lqc2(4, at(c, SP, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, SP, 0xa0));
        c.lq(2, at(c, SP, 0xa0));
        c.lwc1(0, c.g[A1][0] as u32);
        c.sq(2, at(c, SP, 0x60));
        c.lwc1(1, at(c, SP, 0x60));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, c.g[A1][0] as u32);
            break 'b_36e6cc;
        } else {
            break 'b_36e6cc;
        }
    }
    'b_36e6e4: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x4));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x4));
            break 'b_36e6e4;
        } else {
            break 'b_36e6e4;
        }
    }
    'b_36e6fc: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x8));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x8));
            break 'b_36e6fc;
        } else {
            break 'b_36e6fc;
        }
    }
    'b_36e714: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x60));
        c.lwc1(0, at(c, A1, 0x10));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x10));
            break 'b_36e714;
        } else {
            break 'b_36e714;
        }
    }
    'b_36e72c: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x64));
        c.lwc1(0, at(c, A1, 0x14));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x14));
            break 'b_36e72c;
        } else {
            break 'b_36e72c;
        }
    }
    'b_36e744: {
        c.cycles += 6;
        c.lwc1(1, at(c, SP, 0x68));
        c.lwc1(0, at(c, A1, 0x18));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, at(c, A1, 0x18));
            break 'b_36e744;
        } else {
            break 'b_36e744;
        }
    }
    'b_36ebd4: {
        c.cycles += 38;
        c.lwc1(3, at(c, A0, 0x60));
        c.lwc1(1, at(c, SP, 0x20));
        c.lwc1(4, at(c, A0, 0x64));
        c.sub_s(1, 1, 3);
        c.lwc1(5, at(c, A0, 0x68));
        c.lwc1(2, at(c, SP, 0x34));
        c.lwc1(0, at(c, SP, 0x38));
        c.div_s(1, 3, 1);
        c.lwc1(7, at(c, SP, 0x30));
        c.lwc1(9, at(c, SP, 0x24));
        c.sub_s(2, 2, 4);
        c.lwc1(8, at(c, SP, 0x28));
        c.sub_s(0, 0, 5);
        c.sub_s(7, 7, 3);
        c.lwc1(10, gp(c, -0x285c));
        c.sub_s(9, 9, 4);
        c.lwc1(6, gp(c, -0x2858));
        c.sub_s(8, 8, 5);
        c.div_s(2, 4, 2);
        c.div_s(0, 5, 0);
        c.div_s(3, 3, 7);
        c.div_s(4, 4, 9);
        c.div_s(5, 5, 8);
        c.sub_s(1, 1, 10);
        c.mtc1(0, 7);
        c.sub_s(2, 2, 10);
        c.mul_s(8, 1, 6);
        c.sub_s(3, 3, 10);
        c.sub_s(4, 4, 10);
        c.sub_s(5, 5, 10);
        c.sub_s(0, 0, 10);
        c.c_lt_s(7, 8);
        c.mul_s(3, 3, 6);
        c.mul_s(9, 0, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(7, 2, 6);
        let t = !c.fcc();
        c.mul_s(5, 5, 6);
        if t {
            break 'b_36ebd4;
        } else {
            'b_36e874: {
                c.cycles += 38;
                c.lwc1(0, gp(c, -0x2854));
                c.lwc1(1, gp(c, -0x2850));
                c.min_s(8, 8, 0);
                c.mfc1(2, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 8, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 10);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 8, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36e874;
                } else {
                    break 'b_36e874;
                }
            }
            'b_36e88c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e88c;
                } else {
                    break 'b_36e88c;
                }
            }
            'b_36e8a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e8a4;
                } else {
                    break 'b_36e8a4;
                }
            }
            'b_36e8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e8bc;
                } else {
                    break 'b_36e8bc;
                }
            }
            'b_36e8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e8d4;
                } else {
                    break 'b_36e8d4;
                }
            }
            'b_36e8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e8ec;
                } else {
                    break 'b_36e8ec;
                }
            }
            'b_36e96c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36e96c;
                } else {
                    break 'b_36e96c;
                }
            }
            'b_36e984: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36e984;
                } else {
                    break 'b_36e984;
                }
            }
            'b_36e99c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36e99c;
                } else {
                    break 'b_36e99c;
                }
            }
            'b_36e9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36e9b4;
                } else {
                    break 'b_36e9b4;
                }
            }
            'b_36e9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36e9cc;
                } else {
                    break 'b_36e9cc;
                }
            }
            'b_36e9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36e9e4;
                } else {
                    break 'b_36e9e4;
                }
            }
            'b_36ea64: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ea64;
                } else {
                    break 'b_36ea64;
                }
            }
            'b_36ea7c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ea7c;
                } else {
                    break 'b_36ea7c;
                }
            }
            'b_36ea94: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ea94;
                } else {
                    break 'b_36ea94;
                }
            }
            'b_36eaac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eaac;
                } else {
                    break 'b_36eaac;
                }
            }
            'b_36eac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eac4;
                } else {
                    break 'b_36eac4;
                }
            }
            'b_36eadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eadc;
                } else {
                    break 'b_36eadc;
                }
            }
            'b_36eb5c: {
                c.cycles += 32;
                c.mfc1(3, 8);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36eb5c;
                } else {
                    break 'b_36eb5c;
                }
            }
            'b_36eb74: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36eb74;
                } else {
                    break 'b_36eb74;
                }
            }
            'b_36eb8c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eb8c;
                } else {
                    break 'b_36eb8c;
                }
            }
            'b_36eba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eba4;
                } else {
                    break 'b_36eba4;
                }
            }
            'b_36ebbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ebbc;
                } else {
                    break 'b_36ebbc;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36ebd4;
            } else {
                break 'b_36ebd4;
            }
        }
    }
    'b_36efe4: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 3);
        let t = !c.fcc();

        if t {
            break 'b_36efe4;
        } else {
            'b_36ec80: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x284c));
                c.lwc1(1, gp(c, -0x2848));
                c.min_s(3, 3, 0);
                c.lwc1(2, gp(c, -0x2844));
                c.mfc1(2, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 3, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 3, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ec80;
                } else {
                    break 'b_36ec80;
                }
            }
            'b_36ec98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ec98;
                } else {
                    break 'b_36ec98;
                }
            }
            'b_36ecb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ecb0;
                } else {
                    break 'b_36ecb0;
                }
            }
            'b_36ecc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ecc8;
                } else {
                    break 'b_36ecc8;
                }
            }
            'b_36ece0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ece0;
                } else {
                    break 'b_36ece0;
                }
            }
            'b_36ecf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ecf8;
                } else {
                    break 'b_36ecf8;
                }
            }
            'b_36ed78: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ed78;
                } else {
                    break 'b_36ed78;
                }
            }
            'b_36ed90: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ed90;
                } else {
                    break 'b_36ed90;
                }
            }
            'b_36eda8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eda8;
                } else {
                    break 'b_36eda8;
                }
            }
            'b_36edc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36edc0;
                } else {
                    break 'b_36edc0;
                }
            }
            'b_36edd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36edd8;
                } else {
                    break 'b_36edd8;
                }
            }
            'b_36edf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36edf0;
                } else {
                    break 'b_36edf0;
                }
            }
            'b_36ee70: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ee70;
                } else {
                    break 'b_36ee70;
                }
            }
            'b_36ee88: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ee88;
                } else {
                    break 'b_36ee88;
                }
            }
            'b_36eea0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36eea0;
                } else {
                    break 'b_36eea0;
                }
            }
            'b_36eeb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36eeb8;
                } else {
                    break 'b_36eeb8;
                }
            }
            'b_36eed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36eed0;
                } else {
                    break 'b_36eed0;
                }
            }
            'b_36eee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36eee8;
                } else {
                    break 'b_36eee8;
                }
            }
            'b_36ef68: {
                c.cycles += 32;
                c.mfc1(3, 3);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ef68;
                } else {
                    break 'b_36ef68;
                }
            }
            'b_36ef80: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ef80;
                } else {
                    break 'b_36ef80;
                }
            }
            'b_36ef98: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ef98;
                } else {
                    break 'b_36ef98;
                }
            }
            'b_36efb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36efb0;
                } else {
                    break 'b_36efb0;
                }
            }
            'b_36efc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36efc8;
                } else {
                    break 'b_36efc8;
                }
            }
            'b_36efe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36efe0;
                } else {
                    break 'b_36efe0;
                }
            }
            c.cycles += 1;
            c.mtc1(0, 0);
            break 'b_36efe4;
        }
    }
    'b_36f3ec: {
        c.cycles += 3;
        c.c_lt_s(0, 4);
        let t = !c.fcc();

        if t {
            break 'b_36f3ec;
        } else {
            'b_36f08c: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2840));
                c.lwc1(1, gp(c, -0x283c));
                c.min_s(4, 4, 0);
                c.lwc1(2, gp(c, -0x2838));
                c.mfc1(2, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 4, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 4, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f08c;
                } else {
                    break 'b_36f08c;
                }
            }
            'b_36f0a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f0a4;
                } else {
                    break 'b_36f0a4;
                }
            }
            'b_36f0bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f0bc;
                } else {
                    break 'b_36f0bc;
                }
            }
            'b_36f0d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f0d4;
                } else {
                    break 'b_36f0d4;
                }
            }
            'b_36f0ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f0ec;
                } else {
                    break 'b_36f0ec;
                }
            }
            'b_36f104: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f104;
                } else {
                    break 'b_36f104;
                }
            }
            'b_36f184: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f184;
                } else {
                    break 'b_36f184;
                }
            }
            'b_36f19c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f19c;
                } else {
                    break 'b_36f19c;
                }
            }
            'b_36f1b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f1b4;
                } else {
                    break 'b_36f1b4;
                }
            }
            'b_36f1cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f1cc;
                } else {
                    break 'b_36f1cc;
                }
            }
            'b_36f1e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f1e4;
                } else {
                    break 'b_36f1e4;
                }
            }
            'b_36f1fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f1fc;
                } else {
                    break 'b_36f1fc;
                }
            }
            'b_36f27c: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f27c;
                } else {
                    break 'b_36f27c;
                }
            }
            'b_36f294: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f294;
                } else {
                    break 'b_36f294;
                }
            }
            'b_36f2ac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f2ac;
                } else {
                    break 'b_36f2ac;
                }
            }
            'b_36f2c4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f2c4;
                } else {
                    break 'b_36f2c4;
                }
            }
            'b_36f2dc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f2dc;
                } else {
                    break 'b_36f2dc;
                }
            }
            'b_36f2f4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f2f4;
                } else {
                    break 'b_36f2f4;
                }
            }
            'b_36f374: {
                c.cycles += 32;
                c.mfc1(3, 4);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f374;
                } else {
                    break 'b_36f374;
                }
            }
            'b_36f38c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f38c;
                } else {
                    break 'b_36f38c;
                }
            }
            'b_36f3a4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f3a4;
                } else {
                    break 'b_36f3a4;
                }
            }
            'b_36f3bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f3bc;
                } else {
                    break 'b_36f3bc;
                }
            }
            'b_36f3d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f3d4;
                } else {
                    break 'b_36f3d4;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36f3ec;
            } else {
                break 'b_36f3ec;
            }
        }
    }
    'b_36f7fc: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 7);
        let t = !c.fcc();

        if t {
            break 'b_36f7fc;
        } else {
            c.cycles += 17;
            c.lwc1(0, gp(c, -0x2834));
            c.lwc1(1, gp(c, -0x2830));
            c.min_s(7, 7, 0);
            c.lwc1(2, gp(c, -0x282c));
            c.mfc1(2, 7);
            c.lqc2(4, at(c, A0, 0x60));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(c, SP, 0x90));
            c.mul_s(1, 7, 1);
            c.lq(2, at(c, SP, 0x90));
            c.sq(2, at(c, SP, 0x80));
            c.add_s(1, 1, 2);
            c.lqc2(3, c.g[SP][0] as u32);
            c.lqc2(4, at(c, SP, 0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(c, SP, 0x90));
            break 'b_36f7fc;
        }
    }
    'b_36fc04: {
        c.cycles += 3;
        c.c_lt_s(0, 5);
        let t = !c.fcc();

        if t {
            break 'b_36fc04;
        } else {
            'b_36f8a4: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x2828));
                c.lwc1(1, gp(c, -0x2824));
                c.min_s(5, 5, 0);
                c.lwc1(2, gp(c, -0x2820));
                c.mfc1(2, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 5, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 5, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f8a4;
                } else {
                    break 'b_36f8a4;
                }
            }
            'b_36f8bc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f8bc;
                } else {
                    break 'b_36f8bc;
                }
            }
            'b_36f8d4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f8d4;
                } else {
                    break 'b_36f8d4;
                }
            }
            'b_36f8ec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f8ec;
                } else {
                    break 'b_36f8ec;
                }
            }
            'b_36f904: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f904;
                } else {
                    break 'b_36f904;
                }
            }
            'b_36f91c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36f91c;
                } else {
                    break 'b_36f91c;
                }
            }
            'b_36f99c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36f99c;
                } else {
                    break 'b_36f99c;
                }
            }
            'b_36f9b4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36f9b4;
                } else {
                    break 'b_36f9b4;
                }
            }
            'b_36f9cc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36f9cc;
                } else {
                    break 'b_36f9cc;
                }
            }
            'b_36f9e4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36f9e4;
                } else {
                    break 'b_36f9e4;
                }
            }
            'b_36f9fc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36f9fc;
                } else {
                    break 'b_36f9fc;
                }
            }
            'b_36fa14: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fa14;
                } else {
                    break 'b_36fa14;
                }
            }
            'b_36fa94: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36fa94;
                } else {
                    break 'b_36fa94;
                }
            }
            'b_36faac: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36faac;
                } else {
                    break 'b_36faac;
                }
            }
            'b_36fac4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fac4;
                } else {
                    break 'b_36fac4;
                }
            }
            'b_36fadc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fadc;
                } else {
                    break 'b_36fadc;
                }
            }
            'b_36faf4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36faf4;
                } else {
                    break 'b_36faf4;
                }
            }
            'b_36fb0c: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fb0c;
                } else {
                    break 'b_36fb0c;
                }
            }
            'b_36fb8c: {
                c.cycles += 32;
                c.mfc1(3, 5);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36fb8c;
                } else {
                    break 'b_36fb8c;
                }
            }
            'b_36fba4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fba4;
                } else {
                    break 'b_36fba4;
                }
            }
            'b_36fbbc: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fbbc;
                } else {
                    break 'b_36fbbc;
                }
            }
            'b_36fbd4: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fbd4;
                } else {
                    break 'b_36fbd4;
                }
            }
            'b_36fbec: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fbec;
                } else {
                    break 'b_36fbec;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_36fc04;
            } else {
                break 'b_36fc04;
            }
        }
    }
    'b_370010: {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, 9);
        let t = !c.fcc();

        if t {
            break 'b_370010;
        } else {
            'b_36fcb0: {
                c.cycles += 39;
                c.lwc1(0, gp(c, -0x281c));
                c.lwc1(1, gp(c, -0x2818));
                c.min_s(9, 9, 0);
                c.lwc1(2, gp(c, -0x2814));
                c.mfc1(2, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.mul_s(1, 9, 1);
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.add_s(1, 1, 2);
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.mul_s(2, 9, 1);
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36fcb0;
                } else {
                    break 'b_36fcb0;
                }
            }
            'b_36fcc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fcc8;
                } else {
                    break 'b_36fcc8;
                }
            }
            'b_36fce0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fce0;
                } else {
                    break 'b_36fce0;
                }
            }
            'b_36fcf8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fcf8;
                } else {
                    break 'b_36fcf8;
                }
            }
            'b_36fd10: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fd10;
                } else {
                    break 'b_36fd10;
                }
            }
            'b_36fd28: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fd28;
                } else {
                    break 'b_36fd28;
                }
            }
            'b_36fda8: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36fda8;
                } else {
                    break 'b_36fda8;
                }
            }
            'b_36fdc0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36fdc0;
                } else {
                    break 'b_36fdc0;
                }
            }
            'b_36fdd8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fdd8;
                } else {
                    break 'b_36fdd8;
                }
            }
            'b_36fdf0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fdf0;
                } else {
                    break 'b_36fdf0;
                }
            }
            'b_36fe08: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fe08;
                } else {
                    break 'b_36fe08;
                }
            }
            'b_36fe20: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36fe20;
                } else {
                    break 'b_36fe20;
                }
            }
            'b_36fea0: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36fea0;
                } else {
                    break 'b_36fea0;
                }
            }
            'b_36feb8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36feb8;
                } else {
                    break 'b_36feb8;
                }
            }
            'b_36fed0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36fed0;
                } else {
                    break 'b_36fed0;
                }
            }
            'b_36fee8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36fee8;
                } else {
                    break 'b_36fee8;
                }
            }
            'b_36ff00: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36ff00;
                } else {
                    break 'b_36ff00;
                }
            }
            'b_36ff18: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x18));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x18));
                    break 'b_36ff18;
                } else {
                    break 'b_36ff18;
                }
            }
            'b_36ff98: {
                c.cycles += 32;
                c.mfc1(3, 9);
                c.lqc2(4, at(c, A0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x80));
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x90));
                c.lq(2, at(c, SP, 0x90));
                c.sq(2, at(c, SP, 0x70));
                c.mfc1(3, 2);
                c.lqc2(4, at(c, SP, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.sq(2, at(c, SP, 0x90));
                c.lqc2(3, at(c, SP, 0x70));
                c.lqc2(4, at(c, SP, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0xa0));
                c.lq(2, at(c, SP, 0xa0));
                c.lwc1(0, c.g[A1][0] as u32);
                c.sq(2, at(c, SP, 0x60));
                c.lwc1(1, at(c, SP, 0x60));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, c.g[A1][0] as u32);
                    break 'b_36ff98;
                } else {
                    break 'b_36ff98;
                }
            }
            'b_36ffb0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x4));
                    break 'b_36ffb0;
                } else {
                    break 'b_36ffb0;
                }
            }
            'b_36ffc8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x68));
                c.lwc1(0, at(c, A1, 0x8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x8));
                    break 'b_36ffc8;
                } else {
                    break 'b_36ffc8;
                }
            }
            'b_36ffe0: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x60));
                c.lwc1(0, at(c, A1, 0x10));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x10));
                    break 'b_36ffe0;
                } else {
                    break 'b_36ffe0;
                }
            }
            'b_36fff8: {
                c.cycles += 6;
                c.lwc1(1, at(c, SP, 0x64));
                c.lwc1(0, at(c, A1, 0x14));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, at(c, A1, 0x14));
                    break 'b_36fff8;
                } else {
                    break 'b_36fff8;
                }
            }
            c.cycles += 6;
            c.lwc1(1, at(c, SP, 0x68));
            c.lwc1(0, at(c, A1, 0x18));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, at(c, A1, 0x18));
                break 'b_370010;
            } else {
                break 'b_370010;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `int PS2GraphicsMan_WaitForRenderIdle(PS2GraphicsMan* self)` (0x369c28):
/// wait until the renderer reports idle, servicing timers and yielding the
/// EE thread between polls. Dispatcher-shaped: an event unwind leaves `pc`
/// at_3 the poll loop head and the trampoline re-enters there.
pub fn ps2_graphics_man_wait_for_render_idle(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        if pc == 0x0036_9c28 {
            c.cycles += 5;
            open(c, 0x20);
            let sp0 = spv(c);
            c.sq(16, at_3(sp0, 0x10));
            c.sd(sp0, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            pc = 0x0036_9c88;
        }
        loop {
            if pc == 0x0036_9c88 {
                c.cycles += 7;
                let this = c.g[S0][0];
                c.g[V1][0] = lw_at(c, this, 0x10d8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x90);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x94);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(this as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c9c)?;
                let idle = c.g[V0][0] == 0;
                let sp0 = spv(c);
                c.g[RA][0] = c.ld(sp0);
                if idle {
                    if event_due(c) && deliver_event(c, 0x0036_9c40) {
                        return Err(crate::call::Unwound);
                    }
                    pc = 0x0036_9c40;
                } else {
                    c.cycles += 3;
                    c.lq(16, at_3(sp0, 0x10));
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(at_3(sp0, 0x20));
                    c.pc = ra;
                    return Ok(());
                }
            } else if pc == 0x0036_9c40 {
                c.cycles += 18;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x58);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x5c);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c5c)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_TIMER_SERVICE, 0x0036_9c64)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_THREAD_YIELD, 0x0036_9c6c)?;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x60);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x64);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at_3(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c88)?;
                pc = 0x0036_9c88;
            } else if pc == 0x0036_9c3c {
                c.cycles += 1;
                pc = 0x0036_9c40;
            } else {
                c.jump_out(pc, 0x0036_9c28, 0x0036_9cb0);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// `Emitter_SetDescPathCoeffs` (0x36cef8): build the emitter's three descriptor
// vectors from the (b, c) pair — `a - 3*(b + c)`, `2*b`, `2*c` — with the
// original's COP2 macro operations.
pub fn emitter_set_desc_path_coeffs(c: &mut Cpu) {
    c.cycles += 43;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 1);
    c.lqc2(3, c.g[A2][0] as u32);
    c.lqc2(4, c.g[A3][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[AT][0] = 0x40000000;
    c.mtc1(1, 0);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mfc1(3, 1);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xb0));
    c.sq(2, c.g[SP][0] as u32);
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A2][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xc0));
    c.sq(2, c.g[SP][0] as u32);
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A3][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[A0][0] as u32).wrapping_add(0xd0));
    c.sq(2, c.g[SP][0] as u32);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Plant sway matrices.

/// `void PS2GraphicsMan_v94(PS2GraphicsMan* self, float* mtx)` (0x36a2c0):
/// multiply the object's four sway matrices by the caller's matrix and write
/// them back (VU0, in place).
pub fn ps2_graphics_man_v94(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[V0][0] = Addr(r(c, A0)).off(0xa4).lw(c);
        let none: bool = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if none {
            c.cycles += 4;
            frame.restore(c, 16, 0xa0);
            frame.restore_ra(c, 0x90);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 49;
        c.lq(2, p(r(c, S0), 0x30));
        c.g[A0][0] = sx(p(r(c, SP), 0x40));
        c.sq(2, p(r(c, SP), 0x40));
        callee::VFX_PLANTS_GET_SWAY_MATRIX.call(c, 0x0036_a2e8)?;

        c.g[V1][0] = sx(p(r(c, SP), 0x50));
        c.lqc2(4, r(c, S0));
        c.lqc2(5, p(r(c, S0), 0x10));
        c.lqc2(6, p(r(c, S0), 0x20));
        c.lqc2(7, p(r(c, S0), 0x30));
        c.lqc2(8, r(c, V0));
        c.lqc2(9, p(r(c, V0), 0x10));
        c.lqc2(10, p(r(c, V0), 0x20));
        c.lqc2(11, p(r(c, V0), 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r(c, V1));
        c.sqc2(13, p(r(c, V1), 0x10));
        c.sqc2(14, p(r(c, V1), 0x20));
        c.sqc2(15, p(r(c, V1), 0x30));
        c.lqc2(1, r(c, V1));
        c.lqc2(2, p(r(c, V1), 0x10));
        c.lqc2(3, p(r(c, V1), 0x20));
        c.lqc2(4, p(r(c, V1), 0x30));
        c.sqc2(1, r(c, SP));
        c.sqc2(2, p(r(c, SP), 0x10));
        c.sqc2(3, p(r(c, SP), 0x20));
        c.sqc2(4, p(r(c, SP), 0x30));
        c.lqc2(1, r(c, SP));
        c.lqc2(2, p(r(c, SP), 0x10));
        c.lqc2(3, p(r(c, SP), 0x20));
        c.lqc2(4, p(r(c, SP), 0x30));
        c.sqc2(1, r(c, S0));
        c.sqc2(2, p(r(c, S0), 0x10));
        c.sqc2(3, p(r(c, S0), 0x20));
        c.sqc2(4, p(r(c, S0), 0x30));

        c.cycles += 4;
        frame.restore(c, 16, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory accessors: the original's `lw rt, off(base)` and friends.

/// Register-relative guest memory, for the accessors below: the base is read
/// from the register file each time, as the compiled code does.
pub(crate) trait RegMem {
    fn pa(&self, base: usize, off: i32) -> u32;
    fn ldw_2(&mut self, base: usize, off: i32) -> u64;
    fn ldh_2(&mut self, base: usize, off: i32) -> u64;
    fn ldhu_2(&mut self, base: usize, off: i32) -> u64;
    fn ldb(&mut self, base: usize, off: i32) -> u64;
    fn ldbu(&mut self, base: usize, off: i32) -> u64;
    fn ldd(&mut self, base: usize, off: i32) -> u64;
    fn stw_2(&mut self, base: usize, off: i32, v: u64);
    fn sth_2(&mut self, base: usize, off: i32, v: u64);
    fn stb(&mut self, base: usize, off: i32, v: u64);
    fn std(&mut self, base: usize, off: i32, v: u64);
}

impl RegMem for Cpu {
    #[inline(always)]
    fn pa(&self, base: usize, off: i32) -> u32 {
        r32(self, base).wrapping_add(off as u32)
    }
    #[inline(always)]
    fn ldw_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldh_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu_2(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn ldb(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lb(self)
    }
    #[inline(always)]
    fn ldbu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lbu(self)
    }
    #[inline(always)]
    fn ldd(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).ld(self)
    }
    #[inline(always)]
    fn stw_2(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sw(self, v)
    }
    #[inline(always)]
    fn sth_2(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sh(self, v)
    }
    #[inline(always)]
    fn stb(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sb(self, v)
    }
    #[inline(always)]
    fn std(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sd(self, v)
    }
}

// ---------------------------------------------------------------------------
// Functions.

/// `void Emitter_SetDescPathCoeffsScaled(sGmFrustum* self, const Vec4* a,
/// const Vec4* b, const Vec4* c, const Vec4* d)` (0x36d1f0): store each
/// direction divided by `self->scale` (`+0x4c`) at `+0x80`, `+0x90`, `+0xa0`,
/// and `a/scale - 1.5*(b+c+d)/scale` at `+0x70` (VU0 multiplies by the
/// reciprocal of the scale).
pub fn emitter_set_desc_path_coeffs_scaled(c: &mut Cpu) {
    c.cycles += 74;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A2, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 1);
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x80));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A3, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x90));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(T0, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0xa0));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A1, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x20));
    c.lq(2, c.pa(SP, 0x20));
    c.sq(2, c.pa(SP, 0x10));
    c.lqc2(3, c.pa(A0, 0x80));
    c.lqc2(4, c.pa(A0, 0x90));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x40));
    c.lqc2(3, c.pa(SP, 0x40));
    c.lqc2(4, c.pa(A0, 0xa0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x30));
    c.mfc1(3, 1);
    c.lqc2(4, c.pa(SP, 0x30));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x20));
    c.lqc2(3, c.pa(SP, 0x10));
    c.lqc2(4, c.pa(SP, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(A0, 0x70));
    c.sq(2, c.pa(SP, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = ra;
    return;
}

/// `void PS2GraphicsMan_EmitDepthFogClutUpload(void*, int x, int y, char***
/// pp)` (0x36aa60): append the 0xB0-byte depth/fog CLUT-upload packet (six
/// GIF registers writing one CLUT at (x, y)) to the stream at `*pp`, then
/// advance the cursor past it. Leaf.
pub fn ps2_graphics_man_emit_depth_fog_clut_upload(c: &mut Cpu) {
    c.cycles += 79;
    c.g[A0][0] = c.ldw_2(A3, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sq(16, c.pa(SP, 0));
    c.por(3, 0, 0);
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] | 0x6;
    c.sq(3, c.pa(A0, 0));
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = c.g[V0][0] << 33;
    c.g[A2][0] = c.g[A2][0] << 32;
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] << 32;
    c.g[T0][0] = c.g[T0][0] | 0x4;
    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
    c.g[T7][0] = 0xe;
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.g[A1][0] = c.g[A1][0] << 32;
    c.g[T1][0] = 0x50;
    c.g[T2][0] = 0x51;
    c.std(V0, 0x10, c.g[T0][0]);
    c.g[T3][0] = 0x10;
    c.g[T3][0] = c.g[T3][0] << 32;
    c.g[T3][0] = c.g[T3][0] | 0x10;
    c.g[V0][0] = 0x30000000;
    c.g[V0][0] = c.g[V0][0] | 0x40;
    c.g[T4][0] = 0x52;
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    c.g[T5][0] = 0x53;
    c.por(4, 0, 0);
    c.g[A0][0] = 0x8000000;
    c.g[A0][0] = c.g[A0][0] << 32;
    c.g[A0][0] = c.g[A0][0] | 0x40;
    c.std(V1, 0x18, c.g[T7][0]);
    c.por(8, 0, 0);
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] | 0x2;
    c.g[T6][0] = 0x10000000;
    c.g[T6][0] = c.g[T6][0] << 32;
    c.g[T6][0] = c.g[T6][0] | 0x1;
    c.g[S0][0] = 0x3f;
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x20, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x28, c.g[T1][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x30, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x38, c.g[T2][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x40, c.g[T3][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x48, c.g[T4][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x50, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x58, c.g[T5][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.sq(4, c.pa(V0, 0x60));
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x70, c.g[A1][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x78, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.sq(8, c.pa(V1, 0x80));
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0x90, c.g[T6][0]);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0x98, c.g[T7][0]);
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.std(V0, 0xa0, 0u64);
    c.g[V1][0] = c.ldw_2(A3, 0);
    c.std(V1, 0xa8, c.g[S0][0]);
    c.lq(16, c.pa(SP, 0));
    c.g[V0][0] = c.ldw_2(A3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
    c.stw_2(A3, 0, c.g[V0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}

/// `int PS2GraphicsMan_MatchBlendedMatrix(Ent* a, Ent* b, int tol0, int tol1,
/// int tol2)` (0x369a78): do two render entries share a blended matrix?
/// Same type and key id, then type-specific equality shortcuts, then either
/// the same key ids with coordinates within the tolerances, or a swapped
/// (x/z, y) match.
pub fn ps2_graphics_man_match_blended_matrix(c: &mut Cpu) {
    'b_369c1c: {
        c.cycles += 4;
        c.g[T3][0] = c.ldw_2(A0, 0);
        c.g[V0][0] = c.ldw_2(A1, 0);
        let t = c.g[T3][0] != c.g[V0][0];
        c.g[V0][0] = 0u64;
        if t {
            break 'b_369c1c;
        } else {
            'b_369c18: {
                c.cycles += 7;
                c.g[V0][0] = c.ldw_2(A0, 0x4);
                c.g[V1][0] = c.ldw_2(A1, 0x4);
                c.g[T1][0] = c.g[V0][0];
                c.g[A0][0] = c.ldhu_2(V0, 0x2);
                c.g[V0][0] = c.ldhu_2(V1, 0x2);
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = c.g[V1][0];
                if t {
                    break 'b_369c18;
                } else {
                    'b_369ba0: {
                        'b_369b20: {
                            'b_369ae4: {
                                'b_369b1c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[T3][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[T3][0] as i64) < 3i64) as u64;
                                    if t {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.ldh_2(T1, 0);
                                        c.g[V0][0] = c.ldh_2(T2, 0);
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                        if t {
                                            break 'b_369b20;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldhu_2(T2, 0x6);
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V1][0] = c.ldhu_2(T1, 0xa);
                                                break 'b_369ba0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] = c.ldh_2(T1, 0x4);
                                                let t = true;
                                                c.g[V0][0] = c.ldh_2(T2, 0x4);
                                                if t {
                                                    break 'b_369ae4;
                                                } else {
                                                    break 'b_369b1c;
                                                }
                                            }
                                        }
                                    } else {
                                        'b_369adc: {
                                            'b_369ac8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_369ac8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[T3][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_369adc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_369c1c;
                                                        } else {
                                                            break 'b_369ac8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x3;
                                            let t = c.g[T3][0] == c.g[V0][0];
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_369b1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_369c1c;
                                                } else {
                                                    break 'b_369adc;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = c.ldh_2(T1, 0);
                                        c.g[V0][0] = c.ldh_2(T2, 0);
                                        break 'b_369ae4;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                break 'b_369b20;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V1][0] = c.ldhu_2(T1, 0x6);
                                break 'b_369b20;
                            } else {
                                c.cycles += 2;
                                let ra = c.g[RA][0] as u32;
                                c.g[V0][0] = 0x1;
                                c.pc = ra;
                                return;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.ldhu_2(T2, 0x6);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V1][0] = c.ldhu_2(T1, 0xa);
                        if t {
                            break 'b_369ba0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.ldhu_2(T2, 0xa);
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.ldhu_2(T2, 0x6);
                                break 'b_369ba0;
                            } else {
                                'b_369b4c: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh_2(T1, 0);
                                    c.g[V1][0] = c.ldh_2(T2, 0);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                                        break 'b_369b4c;
                                    } else {
                                        break 'b_369b4c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369b6c: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.ldh_2(T1, 0x4);
                                        c.g[V1][0] = c.ldh_2(T2, 0x4);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                        if (c.g[V0][0] as i64) < 0 {
                                            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                                            break 'b_369b6c;
                                        } else {
                                            break 'b_369b6c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        'b_369b8c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ldh_2(T1, 0x8);
                                            c.g[V1][0] = c.ldh_2(T2, 0x8);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[V1][0] as u32));
                                            if (c.g[V0][0] as i64) < 0 {
                                                c.g[V0][0] =
                                                    sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                                                break 'b_369b8c;
                                            } else {
                                                break 'b_369b8c;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_369c1c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.g[V0][0] = 0x1;
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_369c1c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldhu_2(T1, 0x6);
                        c.g[V0][0] = c.ldhu_2(T2, 0xa);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_369c1c;
                        } else {
                            'b_369bcc: {
                                c.cycles += 5;
                                c.g[V0][0] = c.ldh_2(T1, 0);
                                c.g[V1][0] = c.ldh_2(T2, 0);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                if (c.g[V0][0] as i64) < 0 {
                                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                                    break 'b_369bcc;
                                } else {
                                    break 'b_369bcc;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_369c1c;
                            } else {
                                'b_369bec: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh_2(T1, 0x8);
                                    c.g[V1][0] = c.ldh_2(T2, 0x4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                                        break 'b_369bec;
                                    } else {
                                        break 'b_369bec;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369c0c: {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.ldh_2(T1, 0x4);
                                        c.g[A0][0] = c.ldh_2(T2, 0x8);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                                        if (c.g[V1][0] as i64) < 0 {
                                            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                                            break 'b_369c0c;
                                        } else {
                                            break 'b_369c0c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        break 'b_369c18;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_369c1c;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// Emitter_UpdateDescriptor (0x36d428, graphicsman.cpp)

pub(crate) const EMITTER_ADVANCE_DESC_TIME: Callee = Callee {
    addr: 0x0036_d3e8,
    f: ee::s12::Emitter_AdvanceDescTime,
};

pub(crate) const VU0_RAND_NEXT: Callee = Callee {
    addr: 0x0036_d400,
    f: ee::s12::Vu0_RandNext,
};

/// Emitter descriptor fields: the emission count (int), the time
/// accumulator, the emission interval, the per-emission random seeds, the
/// emission position and its per-emission step.
pub(crate) const ED_COUNT: u32 = 0x00;

pub(crate) const ED_TIME: u32 = 0x08;

pub(crate) const ED_INTERVAL: u32 = 0x0c;

pub(crate) const ED_POS: u32 = 0xb0;

pub(crate) const ED_POS_STEP: u32 = 0xe0;

/// The random seeds re-rolled per emission, with the return address of
/// each `Vu0_RandNext` call.
pub(crate) const ED_RANDOMS: [(u32, u32); 9] = [
    (0x38, 0x0036_d488),
    (0x3c, 0x0036_d490),
    (0x40, 0x0036_d498),
    (0x44, 0x0036_d4a0),
    (0x48, 0x0036_d4a8),
    (0x50, 0x0036_d4b0),
    (0x54, 0x0036_d4b8),
    (0x58, 0x0036_d4c0),
    (0x5c, 0x0036_d4c8),
];

/// `void Emitter_UpdateDescriptor(EmitterDesc* d, float dt)` (0x36d428):
/// advances the descriptor's clock (`Emitter_AdvanceDescTime`), then while
/// `count * interval < time` steps it back by one interval, moves the
/// emission position by its step (VU0) and re-rolls the nine random seeds
/// (`Vu0_RandNext`).
pub fn emitter_update_descriptor(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        EMITTER_ADVANCE_DESC_TIME.call(c, 0x0036_d43c)?;
        let d = r32(c, S0);
        let due = |c: &mut Cpu, f: usize| {
            c.lwc1(f, d.wrapping_add(ED_COUNT));
            c.cvt_s_w(f, f);
            c.lwc1(3, d.wrapping_add(ED_INTERVAL));
            c.lwc1(2, d.wrapping_add(ED_TIME));
            c.mul_s(f, f, 3);
            c.c_lt_s(f, 2);
            c.fcc()
        };
        if due(c, 0) {
            c.cycles += 2;
            c.sub_s(0, 2, 3); // time - interval
            loop {
                c.cycles += 33;
                c.g[A0][0] = sx(d.wrapping_add(ED_RANDOMS[0].0));
                c.swc1(0, d.wrapping_add(ED_TIME));
                c.lqc2(3, d.wrapping_add(ED_POS));
                c.lqc2(4, d.wrapping_add(ED_POS_STEP));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, d.wrapping_add(ED_POS));
                for (off, ret_addr) in ED_RANDOMS {
                    c.g[A0][0] = sx(d.wrapping_add(off));
                    VU0_RAND_NEXT.call(c, ret_addr)?;
                }
                if !due(c, 1) {
                    c.cycles += 1;
                    break;
                }
                c.sub_s(0, 2, 3);
                if event_due(c) {
                    deliver_event_here(c, 0x0036_d468);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DeferredDelete_AdvanceFrame (0x36a1e8, graphicsman.cpp)

/// gp-relative index of the current deferred-delete bucket (0..3).
pub(crate) const DD_INDEX: u32 = 0x2a98;

/// The four buckets of objects whose deletion is deferred (0x53ab40).
pub(crate) const DD_BUCKETS_HI: u32 = 0x0054_0000;

pub(crate) const DD_BUCKETS_LO: u32 = 0x54c0;

/// `void DeferredDelete_AdvanceFrame()` (0x36a1e8): advances the bucket
/// index (`(index + 1) % 4`), deletes every object queued in that bucket
/// four frames ago (virtual destructor with flags 3, through the list's
/// `next` links) and empties the bucket.
pub fn deferred_delete_advance_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let index = Addr(r32(c, GP).wrapping_add(DD_INDEX));
        c.g[V0][0] = index.lw(c);
        c.g[V1][0] = u64::MAX;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = DD_BUCKETS_HI as u64;
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
        frame.save(c, S1, 0x10);
        // (index + 1) % 4, C remainder
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(DD_BUCKETS_LO));
        c.g[V1][0] = sx(r32(c, A0) << 2);
        let new_index = c.g[A0][0];
        index.sw(c, new_index);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[V0][0] = index.lw(c);
        if !empty {
            loop {
                c.cycles += 2;
                let node = r32(c, A2);
                c.g[S0][0] = Addr(node).lw(c); // next
                if c.g[A2][0] != 0 {
                    // node->~Object() (deleting, flags 3)
                    c.cycles += 6;
                    c.g[V0][0] = Addr(node.wrapping_add(4)).lw(c);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                    let dtor = r32(c, V1);
                    c.g[A0][0] = sx(node.wrapping_add(r32(c, A0)));
                    call_ptr(c, dtor, 0x0036_a260)?;
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = index.lw(c);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_a240);
                }
            }
        }
        c.cycles += 9;
        c.g[V1][0] = sx(r32(c, S1).wrapping_sub(DD_BUCKETS_LO));
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_ResetBloomParams (0x36c740)

/// PS2GraphicsMan: the bloom parameter sets (0x1c bytes each, 7 floats).
pub(crate) const BLOOM_PARAMS: u32 = 0x6cd4;

pub(crate) const BLOOM_PARAMS_SIZE: u64 = 0x1c;

/// The defaults (`D_004A43D4..E4`, `D_004A43F0/F4`) as `gp` offsets, in the
/// order of the 7 fields.
pub(crate) const BLOOM_DEFAULTS_GP: [u32; 7] =
    [0x12e4, 0x12e8, 0x12ec, 0x12f0, 0x12f4, 0x1300, 0x1304];

/// `void PS2GraphicsMan_ResetBloomParams(PS2GraphicsMan* self, int idx)`
/// (0x36c740): copy the default bloom parameters into set `idx`. Leaves
/// LO/HI = `idx * 0x1c`, a1 = the set's offset in the object, a0 = the
/// set, f0/f1 the last two defaults copied.
pub fn ps2_graphics_man_reset_bloom_params(c: &mut Cpu) {
    c.cycles += 19;
    c.g[V0][0] = BLOOM_PARAMS_SIZE;
    c.mult(0, A1, A1, V0); // a1 = idx * 0x1c
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(BLOOM_PARAMS));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    let set = r32(c, A0);
    let gp = r32(c, GP);
    for (i, off) in BLOOM_DEFAULTS_GP.into_iter().enumerate() {
        // Alternates f1, f0 as the original does.
        let f = if i % 2 == 0 { 1 } else { 0 };
        c.lwc1(f, gp.wrapping_add(off));
        c.swc1(f, set.wrapping_add(4 * i as u32));
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// Emitter_Save (0x370aa8)

/// Emitter description fields saved: rate (+0, float), lifetime (+0x18,
/// float) and the particle count (+0xc, word).
pub(crate) const EMS_RATE: u32 = 0x0;

pub(crate) const EMS_LIFE: u32 = 0x18;

pub(crate) const EMS_COUNT: u32 = 0xc;

/// cStream vtable slot "write(buf, n)" (this-adjust halfword +8, method +0xc).
pub(crate) const EMS_STREAM_WRITE: u32 = 0x8;

/// `void Emitter_Save(EmitterDesc* desc, cStream* s)` (0x370aa8): pack the
/// description's rate, lifetime and count into a 12-byte record on the
/// stack and write it to the stream.
pub fn emitter_save(c: &mut Cpu) {
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let rec = frame.at(c, 0);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0xc;
        let desc = r32(c, A0);
        c.g[V1][0] = Addr(desc.wrapping_add(EMS_COUNT)).lw(c);
        c.lwc1(1, desc.wrapping_add(EMS_RATE));
        c.lwc1(0, desc.wrapping_add(EMS_LIFE));
        c.swc1(1, rec);
        c.swc1(0, rec + 4);
        let count = c.g[V1][0];
        Addr(rec + 8).sw(c, count);
        // s->write(&rec, 12)
        let stream = r32(c, A3);
        let vtable = Addr(stream).lw(c);
        c.g[V0][0] = vtable;
        let adjust = Addr((vtable as u32).wrapping_add(EMS_STREAM_WRITE)).lh(c);
        let method = Addr((vtable as u32).wrapping_add(EMS_STREAM_WRITE + 4)).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx(stream.wrapping_add(adjust as u32));
        call_ptr(c, method as u32, 0x0037_0ae8)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_BuildDescriptor.

/// `base + off` for the register `base` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn emitter_bd_at(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// Load the three floats at `params + src` into f0, f1, f2 and store them to
/// `sp + dst` (x, y, z; the w word is left alone).
pub(crate) fn emitter_bd_stage_xyz(c: &mut Cpu, src: u32, dst: u32) {
    c.lwc1(0, emitter_bd_at(c, S0, src));
    c.lwc1(1, emitter_bd_at(c, S0, src + 4));
    c.lwc1(2, emitter_bd_at(c, S0, src + 8));
    c.swc1(0, emitter_bd_at(c, SP, dst));
    c.swc1(1, emitter_bd_at(c, SP, dst + 4));
    c.swc1(2, emitter_bd_at(c, SP, dst + 8));
}

/// `vf12 = M * v` for the 4x4 matrix at `s2` and the vector at `sp + vec`
/// (VU0 mula/madda/madda/madd), stored to `sp + out`.
pub(crate) fn emitter_bd_apply_matrix(c: &mut Cpu, vec: u32, out: u32) {
    c.lqc2(8, emitter_bd_at(c, SP, vec));
    c.lqc2(4, emitter_bd_at(c, S2, 0));
    c.lqc2(5, emitter_bd_at(c, S2, 0x10));
    c.lqc2(6, emitter_bd_at(c, S2, 0x20));
    c.lqc2(7, emitter_bd_at(c, S2, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, emitter_bd_at(c, SP, out));
}

/// The emitter lifetime in the parameter block.
pub(crate) const EMITTER_PARAM_LIFETIME: u32 = 0x0c;

/// A position-like vector of the parameter block (`params + src`, w = 0)
/// transformed by the matrix and divided by the emitter lifetime
/// (`params + 0xc`, VU0 `div`/`mul Q`), stored to `self + dst` (and to the
/// stack scratch at `sp`). `clear_w` is the compiler's redundant re-clear
/// of the staged w word.
pub(crate) fn emitter_bd_xform_div(c: &mut Cpu, src: u32, dst: u32, clear_w: bool) {
    emitter_bd_stage_xyz(c, src, 0x20);
    emitter_bd_apply_matrix(c, 0x20, 0x30);
    c.lq(2, emitter_bd_at(c, SP, 0x30));
    c.lwc1(0, emitter_bd_at(c, S0, EMITTER_PARAM_LIFETIME));
    c.sq(2, emitter_bd_at(c, SP, 0x10));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, emitter_bd_at(c, SP, 0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, emitter_bd_at(c, SP, 0x30));
    c.lq(2, emitter_bd_at(c, SP, 0x30));
    if clear_w {
        c.sw(emitter_bd_at(c, SP, 0x2c), 0);
    }
    c.sq(2, emitter_bd_at(c, S1, dst));
    c.sq(2, emitter_bd_at(c, SP, 0));
}

/// VU0 `vf5 = vf3 op vf4` through the stack scratch: `sqc2 vf5, tmp;
/// lq v0, tmp; sq v0, dst` (the compiler's copy of a returned vector).
pub(crate) fn emitter_bd_vec_copy(c: &mut Cpu, tmp: u32, dst: u32) {
    c.sqc2(5, emitter_bd_at(c, SP, tmp));
    c.lq(2, emitter_bd_at(c, SP, tmp));
    c.sq(2, emitter_bd_at(c, SP, dst));
}

/// `vf5 = vf4 * (bits of FPU reg fpr).x` with the scalar moved through the
/// GPR `gpr` (`mfc1`, `qmtc2` into vf3), the vector loaded from `sp + vec`.
pub(crate) fn emitter_bd_scale(c: &mut Cpu, gpr: usize, fpr: usize, vec: u32) {
    c.mfc1(gpr, fpr);
    c.lqc2(4, emitter_bd_at(c, SP, vec));
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// `void Emitter_BuildDescriptor(Emitter* self, Matrix* m, EmitterParams* p,
/// Box* bounds, float time)` (0x370058): fill the emitter's particle
/// descriptor from its parameter block. Sets the two spawn curves
/// (`Emitter_SetDescCurveA/B`), the start time (`time`, or random seeds when
/// `time < 0`), resets the acceleration, transforms the spawn position and
/// two spread axes by `m` (`+0xb0..+0xd0`, the position re-centred by
/// `1.5 * (axis1 + axis2)`), the four velocity vectors divided by the
/// lifetime (`+0x70..+0xa0`, the first re-centred by `1.5 * (v1+v2+v3)`),
/// the four colours scaled by 128 (`Emitter_SetDescVelocityCurve`), the
/// time (`+0x14 + 0.5 * +0x1c` when `p+8 < 0`, else 0) and the bounds.
pub fn emitter_build_descriptor(c: &mut Cpu) {
    guest(c, |c| {
        const PARAM_TIME_SIGN: u32 = 0x08;
        const PARAM_CURVE_X: u32 = 0x10;
        const PARAM_CURVE_START: u32 = 0x14;
        const PARAM_CURVE_Z: u32 = 0x18;
        const PARAM_CURVE_SPAN: u32 = 0x1c;
        const PARAM_CURVE_Y: u32 = 0xcc;
        const SELF_START_TIME: u32 = 0x38;

        c.cycles += 30;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save_fpr(c, 20, 0xe0);
        frame.save_ra(c, 0x90);
        c.g[S0][0] = c.g[A2][0]; // params
        c.g[S1][0] = c.g[A0][0]; // self
        c.g[S2][0] = c.g[A1][0]; // matrix
        c.g[S3][0] = c.g[A3][0]; // bounds
        c.mov_s(20, 12); // time

        // Curve A (x, y, z of the parameter curve).
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_X));
        c.lwc1(14, emitter_bd_at(c, S0, PARAM_CURVE_Z));
        c.lwc1(13, emitter_bd_at(c, S0, PARAM_CURVE_Y));
        Callee {
            addr: 0x0036_ce00,
            f: ee::s12::f_0036ce00,
        }
        .call(c, 0x0037_0098)?;

        // Curve B: start -/+ half the span.
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
        c.lwc1(1, emitter_bd_at(c, S0, PARAM_CURVE_START));
        c.mul_s(12, 12, 0);
        c.add_s(13, 1, 12);
        c.sub_s(12, 1, 12);
        Callee {
            addr: 0x0036_ce28,
            f: ee::s12::f_0036ce28,
        }
        .call(c, 0x0037_00bc)?;

        // Start time, or random seeds for a negative time.
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        if !c.fcc() {
            c.swc1(20, emitter_bd_at(c, S1, SELF_START_TIME));
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0036_ccb8,
                f: ee::s12::f_0036ccb8,
            }
            .call(c, 0x0037_00d8)?;
        }

        c.cycles += 305;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 20); // f20 = 0.5 from here on
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0036_d008,
            f: ee::s12::f_0036d008,
        }
        .call(c, 0x0037_00e8)?;

        // Spawn position (w = 1) through the matrix -> self+0xb0.
        c.lwc1(2, emitter_bd_at(c, S0, 0x24));
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(3, emitter_bd_at(c, S0, 0x28));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(1, emitter_bd_at(c, S0, 0x2c));
        c.g[A2][0] = sx(emitter_bd_at(c, SP, 0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[A3][0] = sx(emitter_bd_at(c, SP, 0x40));
        c.swc1(3, emitter_bd_at(c, SP, 0x14));
        c.g[T0][0] = sx(emitter_bd_at(c, SP, 0x60));
        c.swc1(2, emitter_bd_at(c, SP, 0x10));
        c.swc1(1, emitter_bd_at(c, SP, 0x18));
        c.swc1(0, emitter_bd_at(c, SP, 0x1c));
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 3); // f3 = 1.5
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sw(emitter_bd_at(c, SP, 0x1c), 0);
        c.sq(2, emitter_bd_at(c, S1, 0xb0));
        c.sq(2, emitter_bd_at(c, SP, 0));
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 4); // f4 = 128.0

        // The two spread axes (w = 0) -> self+0xc0, self+0xd0.
        emitter_bd_stage_xyz(c, 0x30, 0x10);
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sw(emitter_bd_at(c, SP, 0x1c), 0);
        c.sq(2, emitter_bd_at(c, S1, 0xc0));
        c.sq(2, emitter_bd_at(c, SP, 0));
        emitter_bd_stage_xyz(c, 0x3c, 0x10);
        emitter_bd_apply_matrix(c, 0x10, 0x20);
        c.lq(2, emitter_bd_at(c, SP, 0x20));
        c.sq(2, emitter_bd_at(c, S1, 0xd0));
        c.sq(2, emitter_bd_at(c, SP, 0));

        // position -= 1.5 * (axis1 + axis2).
        c.lqc2(3, emitter_bd_at(c, S1, 0xc0));
        c.lqc2(4, emitter_bd_at(c, S1, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x20, 0x10);
        emitter_bd_scale(c, 3, 3, 0x10);
        emitter_bd_vec_copy(c, 0x20, 0);
        c.lqc2(3, emitter_bd_at(c, S1, 0xb0));
        c.lqc2(4, emitter_bd_at(c, SP, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, emitter_bd_at(c, S1, 0xb0));

        // The four velocity vectors / lifetime -> self+0x70..+0xa0.
        c.sw(emitter_bd_at(c, SP, 0x2c), 0);
        emitter_bd_xform_div(c, 0x48, 0x70, true);
        emitter_bd_xform_div(c, 0x54, 0x80, true);
        emitter_bd_xform_div(c, 0x60, 0x90, true);
        emitter_bd_xform_div(c, 0x6c, 0xa0, false);

        // v1 -= 1.5 * (v2 + v3 + v4).
        c.lqc2(3, emitter_bd_at(c, S1, 0x80));
        c.lqc2(4, emitter_bd_at(c, S1, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x30, 0x20);
        c.lqc2(3, emitter_bd_at(c, SP, 0x20));
        c.lqc2(4, emitter_bd_at(c, S1, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        emitter_bd_vec_copy(c, 0x30, 0x10);
        emitter_bd_scale(c, 3, 3, 0x10);
        emitter_bd_vec_copy(c, 0x30, 0);
        c.lqc2(3, emitter_bd_at(c, S1, 0x70));
        c.lqc2(4, emitter_bd_at(c, SP, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, emitter_bd_at(c, S1, 0x70));

        // The four colours (w first in the block) * 128 -> sp, sp+0x20,
        // sp+0x40, sp+0x60 (the velocity-curve arguments).
        c.lwc1(0, emitter_bd_at(c, S0, 0x88));
        c.lwc1(1, emitter_bd_at(c, S0, 0x8c));
        c.lwc1(2, emitter_bd_at(c, S0, 0x90));
        c.lwc1(3, emitter_bd_at(c, S0, 0x84));
        c.swc1(0, emitter_bd_at(c, SP, 0x10));
        c.swc1(1, emitter_bd_at(c, SP, 0x14));
        c.swc1(2, emitter_bd_at(c, SP, 0x18));
        c.swc1(3, emitter_bd_at(c, SP, 0x1c));
        emitter_bd_scale(c, 3, 4, 0x10);
        emitter_bd_vec_copy(c, 0x20, 0);

        c.lwc1(0, emitter_bd_at(c, S0, 0x98));
        c.lwc1(1, emitter_bd_at(c, S0, 0x9c));
        c.lwc1(2, emitter_bd_at(c, S0, 0xa0));
        c.lwc1(3, emitter_bd_at(c, S0, 0x94));
        c.swc1(0, emitter_bd_at(c, SP, 0x30));
        c.swc1(1, emitter_bd_at(c, SP, 0x34));
        c.swc1(2, emitter_bd_at(c, SP, 0x38));
        c.swc1(3, emitter_bd_at(c, SP, 0x3c));
        emitter_bd_scale(c, 3, 4, 0x30);
        c.sqc2(5, emitter_bd_at(c, SP, 0x40));

        c.lwc1(0, emitter_bd_at(c, S0, 0xa8));
        c.lwc1(1, emitter_bd_at(c, S0, 0xac));
        c.lwc1(2, emitter_bd_at(c, S0, 0xb0));
        c.lwc1(3, emitter_bd_at(c, S0, 0xa4));
        c.lq(2, emitter_bd_at(c, SP, 0x40));
        c.swc1(0, emitter_bd_at(c, SP, 0x50));
        c.sq(2, emitter_bd_at(c, SP, 0x20));
        c.swc1(1, emitter_bd_at(c, SP, 0x54));
        c.swc1(2, emitter_bd_at(c, SP, 0x58));
        c.swc1(3, emitter_bd_at(c, SP, 0x5c));
        emitter_bd_scale(c, 3, 4, 0x50);
        c.sqc2(5, emitter_bd_at(c, SP, 0x60));
        c.lq(2, emitter_bd_at(c, SP, 0x60));

        c.lwc1(0, emitter_bd_at(c, S0, 0xb8));
        c.lwc1(2, emitter_bd_at(c, S0, 0xbc));
        c.sq(2, emitter_bd_at(c, SP, 0x40));
        c.swc1(0, emitter_bd_at(c, SP, 0x70));
        c.lwc1(0, emitter_bd_at(c, S0, 0xb4));
        c.lwc1(1, emitter_bd_at(c, S0, 0xc0));
        c.swc1(2, emitter_bd_at(c, SP, 0x74));
        c.swc1(1, emitter_bd_at(c, SP, 0x78));
        c.swc1(0, emitter_bd_at(c, SP, 0x7c));
        emitter_bd_scale(c, 2, 4, 0x70);
        c.sqc2(5, emitter_bd_at(c, SP, 0x80));

        // Velocity curve over start + 0.5 * span.
        c.lwc1(12, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
        c.lwc1(0, emitter_bd_at(c, S0, PARAM_CURVE_START));
        c.mul_s(12, 12, 20);
        c.lq(2, emitter_bd_at(c, SP, 0x80));
        c.sq(2, emitter_bd_at(c, SP, 0x60));
        c.add_s(12, 0, 12);
        Callee {
            addr: 0x0036_d318,
            f: ee::s12::f_0036d318,
        }
        .call(c, 0x0037_0588)?;

        // Time: start + 0.5 * span when p+8 < 0, else 0.
        c.mtc1(0, 12);
        c.lwc1(0, emitter_bd_at(c, S0, PARAM_TIME_SIGN));
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.cycles += 4;
            c.lwc1(0, emitter_bd_at(c, S0, PARAM_CURVE_SPAN));
            c.lwc1(1, emitter_bd_at(c, S0, PARAM_CURVE_START));
            c.mul_s(0, 0, 20);
            c.add_s(12, 1, 0);
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0036_d3d8,
            f: ee::s12::f_0036d3d8,
        }
        .call(c, 0x0037_05b4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        Callee {
            addr: 0x0036_d500,
            f: ee::s12::f_0036d500,
        }
        .call(c, 0x0037_05c0)?;

        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_RandomizeDescSeeds (0x36ccb8)

/// `Rand_NextMain` (cross-shard: through the table).
pub(crate) const EMITTER_RAND_NEXT_MAIN: u32 = 0x0031_77f0;

/// The emitter descriptor's nine random seeds (floats in [1, 2)).
pub(crate) const EMITTER_SEED_FIELDS: [u32; 9] =
    [0x38, 0x3c, 0x40, 0x44, 0x48, 0x50, 0x54, 0x58, 0x5c];

/// Return addresses of the nine `Rand_NextMain` calls.
pub(crate) const EMITTER_SEED_RETS: [u32; 9] = [
    0x0036_cce4,
    0x0036_cd08,
    0x0036_cd24,
    0x0036_cd40,
    0x0036_cd5c,
    0x0036_cd78,
    0x0036_cd94,
    0x0036_cdb0,
    0x0036_cdcc,
];

/// `void Emitter_RandomizeDescSeeds(EmitterDesc* self)` (0x36ccb8): fill the
/// descriptor's nine seed floats (+0x38..+0x48, +0x50..+0x5c) with random
/// values in [1, 2): a random mantissa under exponent 0 (`(r & 0x7fffff) |
/// 1.0f`), then `(x - 1) + 1` as the original computes it.
pub fn emitter_randomize_desc_seeds(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 82;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = 0x7f_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] |= 0xffff; // mantissa mask
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = 0x3f80_0000; // 1.0f
        call_ptr(c, EMITTER_RAND_NEXT_MAIN, EMITTER_SEED_RETS[0])?;
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        for (i, &field) in EMITTER_SEED_FIELDS.iter().enumerate() {
            let last = i == EMITTER_SEED_FIELDS.len() - 1;
            c.g[V0][0] &= c.g[S0][0];
            if last {
                frame.restore_ra(c, 0);
            }
            c.g[V0][0] |= c.g[S1][0];
            if last {
                frame.restore(c, S0, 0x30);
            }
            c.f[1] = r32(c, V0);
            if last {
                frame.restore(c, S1, 0x20);
            }
            let one = FpuF32(c.f[20]);
            c.f[0] = (FpuF32(c.f[1]) - one).0;
            c.f[0] = (FpuF32(c.f[0]) + one).0;
            if last {
                frame.restore_fpr(c, 20, 0x40);
                c.swc1(0, r32(c, S2).wrapping_add(field));
                frame.restore(c, S2, 0x10);
                frame.close(c);
            } else {
                c.swc1(0, r32(c, S2).wrapping_add(field));
                call_ptr(c, EMITTER_RAND_NEXT_MAIN, EMITTER_SEED_RETS[i + 1])?;
            }
        }
        Ok(())
    })
}

pub(crate) mod emitter_setup_callee {
    use super::*;

    pub const INIT_DESCRIPTOR: Callee = Callee {
        addr: 0x0036_cbf8,
        f: ee::s12::Emitter_InitDescriptor,
    };

    pub const BUILD_DESCRIPTOR: Callee = Callee {
        addr: 0x0037_0058,
        f: ee::s12::Emitter_BuildDescriptor,
    };
}

/// `void Emitter::Setup(Matrix* m, EmitterDef* def)` (0x3705e0): copy the
/// definition's GS state (`+0xd0`, `+0xd4`) and texture ids (`+0xc4`,
/// `+0xc8`); the start time `self+0` is `def->start` (`+8`) when negative,
/// else `start + fade_in + fade / 2` (`+0x14`, `+0x1c`); then initialise
/// the particle descriptor at `self+0x10` (grid `+0`/`+4`, span = the
/// non-negative start or `fade_in + fade / 2`, scale `+0x20`, life `+0xc`,
/// velocity `+0x78..`) and build it for time -1 with bounds at `self+0x160`.
pub fn emitter_setup(c: &mut Cpu) {
    use emitter_setup_callee as f;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x60);
        c.mtc1(0, 1);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0]; // def
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        let me = r32(c, S2);
        let def = r32(c, S1);
        c.g[V0][0] = Addr(def + 0xd0).lw(c);
        Addr(me + 0x180).sw(c, c.g[V0][0]);
        c.lwc1(0, def + 0xd4);
        c.swc1(0, me + 0x188);
        c.g[V0][0] = Addr(def + 0xc4).lw(c);
        Addr(me + 0x4).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(def + 0xc8).lw(c);
        Addr(me + 0x8).sw(c, c.g[V1][0]);
        c.lwc1(3, def + 0x8);
        c.c_lt_s(3, 1); // start < 0
        c.g[S3][0] = c.g[A1][0]; // m
        if c.fcc() {
            c.cycles += 2;
            c.mov_s(0, 3);
        } else {
            c.cycles += 7;
            c.lwc1(0, def + 0x14);
            c.lwc1(1, def + 0x1c);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 2); // 0.5
            c.add_s(0, 3, 0);
            c.mul_s(1, 1, 2);
            c.add_s(0, 0, 1);
        }

        // The descriptor's span.
        c.cycles += 7;
        c.swc1(0, me);
        c.mtc1(0, 0);
        c.lwc1(12, def + 0x8);
        c.c_le_s(0, 12); // 0 <= start
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, def + 0x1c);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1); // 0.5
            c.lwc1(2, def + 0x14);
            c.mul_s(0, 0, 1);
            c.add_s(12, 2, 0);
        }
        c.lwc1(2, def + 0x78);

        c.cycles += 28;
        c.g[S0][0] = sx(me.wrapping_add(0x10)); // the descriptor
        c.lwc1(1, def + 0x7c);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, def + 0x80);
        c.g[A3][0] = c.g[SP][0]; // &velocity
        c.g[A1][0] = Addr(def).lw(c);
        c.g[A2][0] = Addr(def + 0x4).lw(c);
        c.lwc1(13, def + 0x20);
        c.lwc1(14, def + 0xc);
        let v = sp(c);
        c.swc1(2, v);
        c.swc1(1, v + 0x4);
        c.swc1(0, v + 0x8);
        Addr(v + 0xc).sw(c, 0);
        f::INIT_DESCRIPTOR.call(c, 0x0037_06cc)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12); // -1.0
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = sx(r32(c, S2).wrapping_add(0x160)); // bounds
        f::BUILD_DESCRIPTOR.call(c, 0x0037_06e8)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_SetDescVelocityCurve (0x36d318)

/// Emitter desc -> lifetime (float).
pub(crate) const EMITTER_LIFETIME: u32 = 0x4c;

/// Emitter desc -> curve base `a - 1.5 * (c + d)` (vec4).
pub(crate) const EMITTER_CURVE_BASE: u32 = 0xf0;

/// Emitter desc -> curve control points `c` and `d` (vec4 each).
pub(crate) const EMITTER_CURVE_C: u32 = 0x100;

pub(crate) const EMITTER_CURVE_D: u32 = 0x110;

/// Emitter desc -> linear velocity rate `(b - a) / (t * lifetime)` (vec4).
pub(crate) const EMITTER_CURVE_RATE: u32 = 0x120;

/// `void Emitter_SetDescVelocityCurve(desc, const Vec4* a, const Vec4* b,
/// const Vec4* c (a3), const Vec4* d (t0), float t)` (0x36d318): sets the
/// velocity curve of a particle emitter description: base
/// `a - 1.5 * (c + d)`, rate `(b - a) / (t * lifetime)`, and the control
/// points `c`, `d`. VU0 macro ops through 16-byte stack temporaries, as the
/// original (frame 0x40); one 47-cycle block.
pub fn emitter_set_desc_velocity_curve(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, X};
    c.cycles += 47;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let tmp = sp(c);
    let desc = c.g[A0][0] as u32;
    let a = c.g[A1][0] as u32;
    let b = c.g[A2][0] as u32;
    let cp = c.g[A3][0] as u32;
    let dp = c.g[T0][0] as u32;
    c.g[AT][0] = 0x3fc0_0000;
    c.f[0] = 0x3fc0_0000; // 1.5
                          // sum = c + d
    c.lqc2(3, cp);
    c.lqc2(4, dp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, tmp.wrapping_add(0x20));
    // scaled = sum * 1.5
    c.mfc1(V1, 0);
    c.lqc2(4, tmp.wrapping_add(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, tmp.wrapping_add(0x10));
    // base = a - scaled
    c.lqc2(3, a);
    c.lqc2(4, tmp.wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, desc.wrapping_add(EMITTER_CURVE_BASE));
    c.sq(V0, tmp);
    // delta = b - a; rate = delta / (t * lifetime)
    c.lqc2(3, b);
    c.lqc2(4, a);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x20));
    c.f[0] = Addr(desc.wrapping_add(EMITTER_LIFETIME)).read_u32(c);
    c.lq(V0, tmp.wrapping_add(0x20));
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    c.sq(V0, tmp.wrapping_add(0x10));
    c.mfc1(V0, 12);
    c.qmtc2(V0, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, tmp.wrapping_add(0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, tmp.wrapping_add(0x20));
    c.lq(V1, tmp.wrapping_add(0x20));
    c.sq(V1, desc.wrapping_add(EMITTER_CURVE_RATE));
    c.sq(V1, tmp);
    // The control points.
    c.lq(V0, cp);
    c.sq(V0, desc.wrapping_add(EMITTER_CURVE_C));
    c.lq(V1, dp);
    c.sq(V1, desc.wrapping_add(EMITTER_CURVE_D));
    c.g[SP][0] = sx(tmp.wrapping_add(0x40));
    ret(c);
}

// ---------------------------------------------------------------------------
// Emitter_InitDescriptor (0x36cbf8)

/// Particle-system descriptor fields.
pub(crate) const EMD_WIDTH: u32 = 0x0;

pub(crate) const EMD_HEIGHT: u32 = 0x4;

pub(crate) const EMD_U_STEP: u32 = 0xc;

pub(crate) const EMD_INV_HEIGHT: u32 = 0x30;

pub(crate) const EMD_V_SPAN: u32 = 0x34;

pub(crate) const EMD_SCALE: u32 = 0x4c;

pub(crate) const EMD_VELOCITY: u32 = 0x60;

pub(crate) const EMD_GS_STATE: u32 = 0x130;

pub(crate) const EMD_UV_SCALE: u32 = 0x140;

/// The default GS state quadword copied into every descriptor.
pub(crate) const EMD_DEFAULT_GS_STATE: u32 = 0x004f_f120;

/// `vf4 = *from / scale` through VU0 (`vdiv Q` of f14, `vmul.xyzw vf4,
/// vf4, Q`), stored to `sp+0x20`.
pub(crate) fn emd_divide_by_scale(c: &mut Cpu, from: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.mfc1(V1, 14);
    c.qmtc2(V1, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, from);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, sp(c).wrapping_add(0x20));
}

/// `void Emitter_InitDescriptor(Desc* d, int width, int height, float x,
/// float y, float scale, const Vec4* velocity)` (0x36cbf8): sets up a
/// particle system's descriptor: its grid size, the per-column u step
/// (`x * scale / width`), the v span (`y * scale`), `1 / height`, the scale,
/// the velocity divided twice by the scale, the default GS state and a
/// (1, 1, 0, 0) uv scale.
pub fn emitter_init_descriptor(c: &mut Cpu) {
    use crate::abi::{farg, set_fpr};
    use crate::ps2f::FpuF32;
    c.cycles += 48;
    let desc = Addr(r32(c, A0));
    let width = FpuF32::from_i32(r32(c, A1) as i32);
    set_fpr(c, 0, width);
    let height = reg(c, A2);
    desc.off(EMD_HEIGHT).sw(c, height);
    let (x, y, scale) = (farg(c, 0), farg(c, 1), farg(c, 2));
    let frame = Frame::open(c, 0x30);
    let width_bits = reg(c, A1);
    desc.off(EMD_WIDTH).sw(c, width_bits);
    let v_span = y * scale;
    set_fpr(c, 13, v_span);
    set_reg(c, AT, 0x3f80_0000);
    set_fpr(c, 1, FpuF32::ONE);
    set_reg(c, A1, EMD_DEFAULT_GS_STATE as u64);
    let u_step = (x * scale).div(width);
    set_fpr(c, 12, u_step);
    desc.off(EMD_U_STEP).write_fpu(c, u_step);

    // velocity / scale / scale (VU0), through the stack.
    let velocity = r32(c, A3);
    emd_divide_by_scale(c, velocity);
    let tmp = sp(c);
    c.lq(V0, tmp.wrapping_add(0x20));
    c.sq(V0, tmp.wrapping_add(0x10));
    emd_divide_by_scale(c, tmp.wrapping_add(0x10));

    let height = FpuF32::from_i32(desc.off(EMD_HEIGHT).lw(c) as i32);
    c.lq(V0, tmp.wrapping_add(0x20));
    desc.off(EMD_V_SPAN).write_fpu(c, v_span);
    c.sq(V0, tmp);
    let inv_height = FpuF32::ONE.div(height);
    set_fpr(c, 0, inv_height);
    c.sq(V0, desc.off(EMD_VELOCITY).0);

    // uv scale (1, 1, 0, 0) and the default GS state.
    Addr(tmp).off(4).write_fpu(c, FpuF32::ONE);
    desc.off(EMD_SCALE).write_fpu(c, scale);
    Addr(tmp).write_fpu(c, FpuF32::ONE);
    Addr(tmp).off(8).sw(c, 0);
    Addr(tmp).off(0xc).sw(c, 0);
    desc.off(EMD_INV_HEIGHT).write_fpu(c, inv_height);
    c.lq(V0, tmp);
    c.lq(V1, EMD_DEFAULT_GS_STATE);
    c.sq(V0, desc.off(EMD_UV_SCALE).0);
    c.sq(V1, desc.off(EMD_GS_STATE).0);
    frame.close(c);
}

/// `uint Morton_Spread8(uint x)` (0x36a3b0): spread the low 8 bits of `x` into
/// every other bit.
pub fn morton_spread8(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = c.g[A0][0] & 0xff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0xf0f;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x3333;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x5555;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// DynamicEmitter_ctor (0x370b60)
// ---------------------------------------------------------------------------

/// The dynamic-emitter view's vtable.
pub(crate) const DYNAMIC_EMITTER_CTOR_VTABLE: u32 = 0x49_3118;

/// The 0x10-byte block `memset` clears.
pub(crate) const DYNAMIC_EMITTER_CTOR_BLOCK: u32 = 0x1e4;

/// The two 16-bit id slots (both marked free).
pub(crate) const DYNAMIC_EMITTER_CTOR_IDS: u32 = 0x1f4;

/// `sGmView* DynamicEmitter_ctor(sGmView* self)` (0x370b60): clear the view
/// header, install the vtable, construct the emitter base at `+0x20`, set the
/// default state words, mark both id slots free (`-1`) and zero the 0x10-byte
/// block.
pub fn dynamic_emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[V0][0] = sx(DYNAMIC_EMITTER_CTOR_VTABLE);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x0c), 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f8), c.g[V0][0]);
        call_ptr(c, 0x002f_7a68, 0x0037_0b90)?;
        c.g[V1][0] = u64::MAX; // -1: both id slots are free
        c.g[V0][0] = 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x178), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(DYNAMIC_EMITTER_CTOR_BLOCK));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x174), c.g[V0][0]);
        c.g[A2][0] = u64::MAX;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x170), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), 0);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(DYNAMIC_EMITTER_CTOR_IDS));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1e0), 0);
        loop {
            c.cycles += 7;
            c.sh(c.g[V1][0] as u32, c.g[A2][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[A1][0] < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0037_0bc8);
                }
                continue;
            }
            c.cycles += 8;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x10;
            // `memset(self->block, 0, 0x10)`
            call_ptr(c, 0x0041_6210, 0x0037_0bf0)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore_ra(c, 0);
            frame.restore(c, 16, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// Emitter_ctor (0x370018)
// ---------------------------------------------------------------------------

/// The emitter's vtable.
pub(crate) const EMITTER_CTOR_VTABLE: u32 = 0x49_3160;

/// `Emitter* Emitter_ctor(Emitter* self)` (0x370018): install the vtable,
/// construct the emitter base at `+0x10` and clear the two state words.
pub fn emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(EMITTER_CTOR_VTABLE);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18c), c.g[V0][0]);
        call_ptr(c, 0x002f_7a68, 0x0037_003c)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x0c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x184), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------- GraphicsMan_DestroyGridMesh

pub(crate) const GRAPHICS_MAN_DEFERRED_DELETE_PUSH: Callee = Callee {
    addr: 0x0036_a1c0,
    f: ee::s12::DeferredDelete_Push,
};

/// `void GraphicsMan_DestroyGridMesh(GraphicsMan* gm, GridMesh* mesh)`
/// (0x0036a190): hand the mesh to the deferred-delete queue.
pub fn graphics_man_destroy_grid_mesh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        GRAPHICS_MAN_DEFERRED_DELETE_PUSH.call(c, 0x0036_a1a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_AllocBlendedMatrixHistory (0x3698e0)

/// `PS2GraphicsMan+0xf48`: the blended-matrix history (0x200 entries of
/// 0xc bytes: `{int count; Weight* weights; int}`, then their weights);
/// `+0xf4c`: the number of entries in use.
pub(crate) const PS2GM_BLEND_HISTORY: u32 = 0xf48;

pub(crate) const PS2GM_BLEND_COUNT: u32 = 0xf4c;

/// `PS2GraphicsMan_MatchBlendedMatrix` (0x369a78).
pub(crate) const PS2GM_MATCH_BLENDED_MATRIX: Callee = Callee {
    addr: 0x0036_9a78,
    f: ee::s12::PS2GraphicsMan_MatchBlendedMatrix,
};

/// `void PS2GraphicsMan_AllocBlendedMatrixHistory(PS2GraphicsMan* self,
/// BlendedMatrix* m)` (0x3698e0): allocate the history on first use (0x3000
/// bytes, each entry pointing at its 3 weight words), then look for an entry
/// that matches `m` within the tolerances (10, 20, 20): on a match copy its
/// bone ids into `m`'s weights; else append `m` (count and weights) as a new
/// entry.
pub fn ps2_graphics_man_alloc_blended_matrix_history(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        c.g[S3][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 10;
            c.g[A1][0] = 0x0049_2638; // the allocation tag
            c.g[A0][0] = 0x3000;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0036_9920)?; // operator_new_named
            Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x1800));
            c.g[A0][0] = 0x1ff;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                Addr(oc391_at(c, V0, 4)).sw(c, c.g[V1][0]);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc));
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9930);
                }
            }
        }
        // Look for a matching entry.
        c.cycles += 4;
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[S2][0] = 0;
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        'append: {
            if empty {
                break 'append;
            }
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0x14;
                c.g[T0][0] = 0x14;
                PS2GM_MATCH_BLENDED_MATRIX.call(c, 0x0036_9978)?;
                if c.g[V0][0] != 0 {
                    // Found: copy the entry's bone ids into m.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if none {
                        c.cycles += 1;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(oc391_at(c, S0, 4)).lw(c);
                        loop {
                            c.cycles += 11;
                            c.g[V1][0] = sx(r32(c, A2) << 2);
                            c.g[A1][0] = Addr(oc391_at(c, S3, 4)).lw(c);
                            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            c.g[A0][0] = Addr(r32(c, V0)).lhu(c);
                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                            Addr(r32(c, V1)).sh(c, c.g[A0][0]);
                            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V0][0] = Addr(oc391_at(c, S0, 4)).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9990);
                            }
                        }
                        c.cycles += 2;
                    }
                    return ps2gm_blend_history_exit(c, frame);
                }
                c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break 'append;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9960);
                }
            }
        }
        // Append m as entry nBlend.
        c.cycles += 10;
        c.g[V1][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[V0][0] = 0xc;
        c.g[A1][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        c.mult(0, V1, V1, V0);
        c.g[A0][0] = Addr(r32(c, S3)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        Addr(r32(c, V1)).sw(c, c.g[A0][0]);
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[T0][0] = 0xc;
            c.g[A0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
            loop {
                c.cycles += 17;
                c.g[A1][0] = sx(r32(c, A3) << 2);
                c.g[A2][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.mult(0, A0, A0, T0);
                c.g[V1][0] = Addr(oc391_at(c, S3, 4)).lw(c);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
                c.g[V0][0] = Addr(oc391_at(c, A0, 4)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                let src = r32(c, V1);
                let dst = r32(c, A1);
                c.lwl(2, src.wrapping_add(3));
                c.lwr(2, src);
                c.swl(2, dst.wrapping_add(3));
                c.swr(2, dst);
                c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9a08);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).sw(c, c.g[V0][0]);
        c.cycles += 1;
        ps2gm_blend_history_exit(c, frame)
    })
}

/// The epilogue of [`ps2_graphics_man_alloc_blended_matrix_history`].
pub(crate) fn ps2gm_blend_history_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore(c, S0, 0x40);
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void DeferredDelete_FlushAll()` (0x36a290): advances the deferred-delete
/// buckets four times, deleting everything queued.
pub fn deferred_delete_flush_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x3;
        loop {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0036_a2a8);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            callee::DEFERRED_DELETE_ADVANCE_FRAME.call(c, 0x0036_a2a8)?;
            let done = (c.g[S0][0] as i64) < 0;
            frame.restore_ra(c, 0);
            if done {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_a2a0);
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod oc1058_callee {
    use super::ee;
    use super::Callee;

    pub const SPR_STREAM_BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ee::s12::SprStream_BeginBlock,
    };

    pub const SPR_STREAM_END_BLOCK: Callee = Callee {
        addr: 0x0038_f598,
        f: ee::s12::SprStream_EndBlock,
    };
}

pub(crate) mod oc1059_blur {
    use crate::call::Callee;

    /// `SprStream_BeginBlock(stream, handle, -1, 0)`.
    pub const BEGIN_BLOCK: Callee = Callee {
        addr: 0x0038_f460,
        f: ssx3_ee::s12::SprStream_BeginBlock,
    };

    /// `SprStream_EndBlockAligned(stream, end, 4)`.
    pub const END_BLOCK_ALIGNED: Callee = Callee {
        addr: 0x0038_f668,
        f: ssx3_ee::s12::SprStream_EndBlockAligned,
    };
}

/// The low 32 bits of general register `r`.
#[inline(always)]
pub(crate) fn fog_w(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The address `off(sp)`.
#[inline(always)]
pub(crate) fn fog_sp(c: &Cpu, off: u32) -> u32 {
    (c.g[SP][0] as u32).wrapping_add(off)
}

/// Where the code continues after a fog mode's loop (the generated body's
/// join blocks `0x36a9d8`, `0x36a9dc` and `0x36a9e0`).
#[derive(Clone, Copy)]
pub(crate) enum FogTail {
    /// `cycles += 1; a2 = s2`, then `Count`.
    Clamp,
    /// `cycles += 1; v0 = a2 < 256`, then the opaque tail.
    Count,
    /// `v0 = a2 < 256` is already computed: straight to the opaque tail.
    Opaque,
}

/// Epilogue of the CLUT builder: restore s1..s6, `ra`, `f20`, pop the frame.
pub(crate) fn fog_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

/// Fog mode 0: linear ramp, `alpha = clamp(((max(i, n1) - n1) << 7) / cnt, 0,
/// 0x80)` for entries `i < n0`, stored at the swizzled index in `t0[...]`
/// (bits 3 and 4 of `i` swapped) as `colour | alpha << 24`.
pub(crate) fn fog_linear_loop(c: &mut Cpu) -> FogTail {
    c.cycles += 2;
    let empty = (c.g[S2][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if empty {
        return FogTail::Clamp;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffffffffffffffe7;
    c.g[T1][0] = 0u64;
    c.g[A3][0] = 0x80;
    loop {
        c.cycles += 15;
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[V1][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A0][0] = c.g[A2][0] & 0x8;
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_sub(fog_w(c, S1)));
        c.g[A0][0] = sx(fog_w(c, A0) << 1);
        c.g[V1][0] = sx(fog_w(c, V1) << 7);
        c.g[A1][0] = c.g[A2][0] & c.g[T2][0];
        c.div(0, V1, S3);
        c.g[V0][0] = c.g[A2][0] & 0x10;
        c.g[A1][0] |= c.g[A0][0];
        c.g[V0][0] = sx((fog_w(c, V0) as i32 >> 1) as u32);
        c.g[A1][0] |= c.g[V0][0];
        c.cycles += 4;
        c.g[V1][0] = c.lo[0];
        c.g[A0][0] = c.g[V1][0];
        let negative = (c.g[A0][0] as i64) < 0;
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if negative {
            c.cycles += 1;
            c.g[V1][0] = 0u64;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[A3][0];
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A0][0];
            }
        }
        c.cycles += 8;
        c.g[A0][0] = sx(fog_w(c, A1) << 2);
        c.g[V1][0] = sx(fog_w(c, V1) << 24);
        c.g[A0][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, T0)));
        c.g[V1][0] = c.g[S5][0] | c.g[V1][0];
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let more = c.g[V0][0] != 0u64;
        c.sw(fog_w(c, A0), c.g[V1][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0036_a738);
            }
            continue;
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[S2][0];
        return FogTail::Count;
    }
}

/// Fog modes 1 (exponential, `alpha = exp(-d)`) and 2 (exponential squared,
/// `exp(-d * d)`), `d = density * scale * (n0 - max(i, n1) + 1) /
/// cnt`: the loop evaluates `exp` with a polynomial whose coefficients sit
/// at `gp - 0x289c` (mode 1) or `gp - 0x2884` (mode 2), clamps to `0..1` and
/// scales by 128.
pub(crate) fn fog_exp_loop(c: &mut Cpu, squared: bool) -> FogTail {
    c.cycles += 2;
    let empty = (c.g[S2][0] as i64) <= 0;
    c.g[A1][0] = 0u64;
    if empty {
        return FogTail::Clamp;
    }
    c.cycles += 15;
    let base = fog_w(c, GP).wrapping_sub(if squared { 0x2884 } else { 0x289c });
    c.lwc1(13, base);
    c.g[A2][0] = 0xffffffffffffffe7;
    c.lwc1(12, base.wrapping_add(0x4));
    c.lwc1(11, base.wrapping_add(0x8));
    c.lwc1(10, base.wrapping_add(0xc));
    c.lwc1(9, base.wrapping_add(0x10));
    c.lwc1(8, base.wrapping_add(0x14));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 7);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 4);
    c.mtc1(0, 5);
    c.g[AT][0] = 0x43000000;
    c.mtc1(1, 6);
    let head = if squared { 0x0036_a910 } else { 0x0036_a800 };
    loop {
        c.cycles += if squared { 38 } else { 37 };
        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = c.g[A1][0];
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.g[S1][0];
        }
        c.lwc1(2, fog_w(c, S4).wrapping_add(0xc));
        c.g[V0][0] = sx(fog_w(c, S2).wrapping_sub(fog_w(c, V0)));
        c.g[V1][0] = c.g[A1][0] & 0x8;
        c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(0x1));
        c.mul_s(2, 2, 13);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(fog_w(c, V1) << 1);
        c.g[A0][0] = c.g[A1][0] & c.g[A2][0];
        c.g[V0][0] = c.g[A1][0] & 0x10;
        c.g[A0][0] |= c.g[V1][0];
        c.g[V0][0] = sx((fog_w(c, V0) as i32 >> 1) as u32);
        c.mul_s(2, 2, 0);
        c.g[A0][0] |= c.g[V0][0];
        c.mul_s(2, 2, 20);
        if squared {
            c.mul_s(2, 2, 2);
        }
        c.neg_s(2, 2);
        c.mul_s(0, 2, 12);
        c.add_s(0, 0, 11);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 10);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 9);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 8);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 7);
        c.mul_s(0, 0, 2);
        c.add_s(1, 0, 4);
        c.mul_s(0, 1, 2);
        c.add_s(0, 0, 4);
        c.c_le_s(5, 0);
        let negative = !c.fcc();
        c.mov_s(3, 5);
        if !negative {
            c.cycles += 1;
            c.min_s(3, 0, 4);
        }
        c.cycles += 11;
        c.mul_s(0, 3, 6);
        c.g[V1][0] = sx(fog_w(c, A0) << 2);
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_add(fog_w(c, T0)));
        c.g[A1][0] = sx(fog_w(c, A1).wrapping_add(0x1));
        c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(fog_w(c, V0) << 24);
        c.g[V0][0] = c.g[S5][0] | c.g[V0][0];
        let more = c.g[A0][0] != 0u64;
        c.sw(fog_w(c, V1), c.g[V0][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        if squared {
            return FogTail::Clamp;
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[S2][0];
        return FogTail::Count;
    }
}

/// `void PS2GraphicsMan_BuildDepthFogClut(PS2GraphicsMan *man, void *mat,
/// int idx)` (0x36a428): build the 256-entry depth fog CLUT of fog `idx`.
///
/// Transforms the fog's near and far depth (`man+idx*0x20+0x6c64`: nearZ,
/// farZ, mode, density, colour) through the view matrix
/// `man+idx*0x40+0x5930`, divides by w, and turns them into CLUT indexes
/// `n0` (near, 0..255) and `n1` (far). When the cache at `gp+0x10b8..`
/// already holds the same indexes, colour, mode and density, it returns. Else
/// it stores the new key and fills the 0x400-byte block that
/// `SprStream_BeginBlock` hands out: entries below `n0` get an alpha by the
/// fog mode (0 linear, 1 exponential, 2 exponential squared), the rest are
/// opaque (0x80), entries swizzled with `i & ~0x18 | (i & 8) << 1 | (i & 0x10)
/// >> 1`; `SprStream_EndBlock` then starts the upload.
pub fn ps2_graphics_man_build_depth_fog_clut(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        c.g[A3][0] = c.g[A2][0];
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S4, 0x70);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);

        // s4 = &man->fog[idx]; v1 = a2 = &man->mats[idx].
        c.g[V0][0] = sx(fog_w(c, A3) << 5);
        c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(0x6c64));
        c.g[S4][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, V0)));
        c.g[A2][0] = sx(fog_w(c, A3) << 6);
        c.g[V1][0] = sx(fog_w(c, A2).wrapping_add(fog_w(c, A0)));
        c.g[V1][0] = sx(fog_w(c, V1).wrapping_add(0x5930));
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(fog_w(c, A0)));
        c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x5930));
        c.g[T3][0] = c.g[A1][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 2);
        c.g[A0][0] = 0xff;

        // a = mats[idx] * (0, 0, nearZ, 1).
        c.lwc1(0, fog_w(c, S4));
        c.swc1(2, fog_sp(c, 0x1c));
        c.swc1(0, fog_sp(c, 0x18));
        c.sw(fog_sp(c, 0x10), 0u64);
        c.sw(fog_sp(c, 0x14), 0u64);
        c.lqc2(8, fog_sp(c, 0x10));
        c.lqc2(4, fog_w(c, V1));
        c.lqc2(5, fog_w(c, V1).wrapping_add(0x10));
        c.lqc2(6, fog_w(c, V1).wrapping_add(0x20));
        c.lqc2(7, fog_w(c, V1).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, fog_sp(c, 0x20));
        // b = mats[idx] * (0, 0, farZ, 1).
        c.lwc1(0, fog_w(c, S4).wrapping_add(0x4));
        c.lq(2, fog_sp(c, 0x20));
        c.swc1(0, fog_sp(c, 0x28));
        c.sq(2, fog_sp(c, 0));
        c.swc1(2, fog_sp(c, 0x2c));
        c.sw(fog_sp(c, 0x20), 0u64);
        c.sw(fog_sp(c, 0x24), 0u64);
        c.lqc2(8, fog_sp(c, 0x20));
        c.lqc2(4, fog_w(c, A2));
        c.lqc2(5, fog_w(c, A2).wrapping_add(0x10));
        c.lqc2(6, fog_w(c, A2).wrapping_add(0x20));
        c.lqc2(7, fog_w(c, A2).wrapping_add(0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, fog_sp(c, 0x30));
        // a *= 1 / a.w ; b *= 1 / b.w.
        c.lq(2, fog_sp(c, 0x30));
        c.lwc1(0, fog_sp(c, 0xc));
        c.sq(2, fog_sp(c, 0x10));
        c.div_s(0, 2, 0);
        c.mfc1(2, 0);
        c.lqc2(4, fog_sp(c, 0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, fog_sp(c, 0));
        c.lwc1(1, fog_sp(c, 0x1c));
        c.div_s(2, 2, 1);
        c.mfc1(2, 2);
        c.lqc2(4, fog_sp(c, 0x10));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, fog_sp(c, 0x10));

        // n0 = clamp((int)a.z >> 8, 0, 255) in s2.
        c.lwc1(0, fog_sp(c, 0x8));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(((fog_w(c, V0)) as i32 >> 8) as u32);
        let below = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = c.g[V0][0];
        if below {
            c.cycles += 1;
            c.g[S2][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = c.g[A0][0];
            }
        }
        // n1 = clamp((int)b.z >> 8, 0, 255) in s1.
        c.cycles += 6;
        c.lwc1(0, fog_sp(c, 0x18));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(((fog_w(c, V0)) as i32 >> 8) as u32);
        let below = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0xff;
        if below {
            c.cycles += 1;
            c.g[S1][0] = 0;
        } else {
            c.cycles += 4;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = c.g[V1][0];
            }
        }

        // cnt = max(n0 - n1 + 1, 1) in s3; colour key in s5; f20 = 1 / cnt.
        c.cycles += 41;
        c.g[AT][0] = 0x437f0000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx(fog_w(c, S2).wrapping_sub(fog_w(c, S1)));
        c.lwc1(1, fog_w(c, S4).wrapping_add(0x18));
        c.g[S3][0] = sx(fog_w(c, V0).wrapping_add(0x1));
        c.lwc1(2, fog_w(c, S4).wrapping_add(0x14));
        c.g[V0][0] = (0i64 < (c.g[S3][0] as i64)) as u64;
        c.lwc1(3, fog_w(c, S4).wrapping_add(0x1c));
        c.mul_s(1, 1, 0);
        c.mul_s(2, 2, 0);
        c.g[A0][0] = 0x1;
        c.mul_s(3, 3, 0);
        if c.g[V0][0] == 0 {
            c.g[S3][0] = c.g[A0][0];
        }
        c.cvt_w_s(0, 1);
        c.mfc1(3, 0);
        c.g[A0][0] = sx(fog_w(c, A3) << 2);
        c.cvt_w_s(0, 2);
        c.mfc1(5, 0);
        c.g[T0][0] = c.g[A0][0];
        c.cvt_w_s(0, 3);
        c.mfc1(2, 0);
        c.g[A3][0] = sx(fog_w(c, GP).wrapping_add(0x2948));
        c.g[A2][0] = sx(fog_w(c, GP).wrapping_add(0x10b8));
        c.g[V1][0] = sx(fog_w(c, V1) << 8);
        c.mtc1(19, 1);
        c.cvt_s_w(1, 1);
        c.g[A1][0] |= c.g[V1][0];
        c.g[A0][0] = sx(fog_w(c, A0).wrapping_add(fog_w(c, A2)));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx(fog_w(c, V0) << 16);
        c.g[T2][0] = c.g[A2][0];
        c.g[S5][0] = c.g[A1][0] | c.g[V0][0];
        c.g[V1][0] = c.lw(fog_w(c, A0));
        c.div_s(20, 0, 1);
        c.g[A1][0] = sx(fog_w(c, GP).wrapping_add(0x2938));
        c.g[A2][0] = sx(fog_w(c, GP).wrapping_add(0x2940));
        c.g[T1][0] = sx(fog_w(c, GP).wrapping_add(0x2950));
        c.g[T4][0] = sx(fog_w(c, GP).wrapping_add(0x2958));
        let cold = c.g[V1][0] == 0u64;
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A1)));

        // Cache check: valid, n0, n1, colour, mode, density all equal -> done.
        // `refill` is false on a hit; `slot_cycle` charges the extra cycle of
        // the paths that run through the join block 0x36a698.
        let mut hit = false;
        let mut slot_cycle = false;
        'cache: {
            if cold {
                slot_cycle = true;
                break 'cache;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S2][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A2)));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S1][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A3)));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[S5][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T1)));
            c.g[A0][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
            c.g[V1][0] = c.lw(fog_w(c, V0));
            let miss = c.g[A0][0] != c.g[V1][0];
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
            if miss {
                break 'cache;
            }
            c.cycles += 7;
            c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T4)));
            c.lwc1(1, fog_w(c, S4).wrapping_add(0xc));
            c.lwc1(0, fog_w(c, V0));
            c.c_eq_s(1, 0);
            let same = c.fcc();
            frame.restore(c, S0, 0xb0);
            if same {
                hit = true;
            } else {
                slot_cycle = true;
            }
        }
        if hit {
            fog_epilogue(c, frame);
            return Ok(());
        }
        if slot_cycle {
            c.cycles += 1;
            c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T2)));
        }

        // Miss: store the key, take the block.
        c.cycles += 23;
        c.g[S0][0] = 0x1;
        c.sw(fog_w(c, V1), c.g[S0][0]);
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A1)));
        c.sw(fog_w(c, V0), c.g[S2][0]);
        c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A2)));
        c.sw(fog_w(c, V1), c.g[S1][0]);
        c.g[V0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, A3)));
        c.sw(fog_w(c, V0), c.g[S5][0]);
        c.g[V1][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T1)));
        c.g[S6][0] = c.lw(fog_w(c, GP).wrapping_add(0x1384));
        c.g[T0][0] = sx(fog_w(c, T0).wrapping_add(fog_w(c, T4)));
        c.g[V0][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
        c.g[A1][0] = c.g[T3][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A2][0] = 0x400;
        c.sw(fog_w(c, V1), c.g[V0][0]);
        c.g[A3][0] = 0u64;
        c.lwc1(0, fog_w(c, S4).wrapping_add(0xc));
        c.swc1(0, fog_w(c, T0));
        oc1058_callee::SPR_STREAM_BEGIN_BLOCK.call(c, 0x0036_a6ec)?;
        c.g[V1][0] = c.lw(fog_w(c, S4).wrapping_add(0x8));
        let mode_exp = c.g[V1][0] == c.g[S0][0];
        c.g[T0][0] = c.g[V0][0];

        // Mode dispatch: 1 exponential, 2 exponential squared, 0 linear;
        // anything else writes only the opaque tail.
        let join = if mode_exp {
            fog_exp_loop(c, false)
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let ge2 = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x2;
            if !ge2 {
                c.cycles += 2;
                let linear = c.g[V1][0] == 0u64;
                c.g[A2][0] = c.g[S2][0];
                if linear {
                    fog_linear_loop(c)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                    FogTail::Opaque
                }
            } else {
                c.cycles += 2;
                let squared = c.g[V1][0] == c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                if squared {
                    fog_exp_loop(c, true)
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                    FogTail::Opaque
                }
            }
        };
        if matches!(join, FogTail::Clamp) {
            c.cycles += 1;
            c.g[A2][0] = c.g[S2][0];
        }
        if !matches!(join, FogTail::Opaque) {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
        }

        // Opaque tail: entries n0..255 get alpha 0x80.
        c.cycles += 2;
        let none = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0xffffffff80000000;
        if !none {
            c.cycles += 2;
            c.g[A3][0] = 0xffffffffffffffe7;
            c.g[A1][0] = c.g[S5][0] | c.g[V0][0];
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[A2][0] & 0x8;
                c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                c.g[V1][0] = c.g[A2][0] & 0x10;
                c.g[A0][0] = sx(fog_w(c, A0) << 1);
                c.g[V1][0] = sx((fog_w(c, V1) as i32 >> 1) as u32);
                c.g[V0][0] |= c.g[A0][0];
                c.g[V0][0] |= c.g[V1][0];
                c.g[A2][0] = sx(fog_w(c, A2).wrapping_add(0x1));
                c.g[V0][0] = sx(fog_w(c, V0) << 2);
                c.g[V1][0] = ((c.g[A2][0] as i64) < 256i64) as u64;
                c.g[V0][0] = sx(fog_w(c, V0).wrapping_add(fog_w(c, T0)));
                let more = c.g[V1][0] != 0u64;
                c.sw(fog_w(c, V0), c.g[A1][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_a9f0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x400;
        c.g[A2][0] = 0x4;
        oc1058_callee::SPR_STREAM_END_BLOCK.call(c, 0x0036_aa34)?;
        frame.restore(c, S0, 0xb0);
        fog_epilogue(c, frame);
        Ok(())
    })
}

/// One column-strip pass of the blur: how the stack constants are offset
/// and which registers the generated code uses for the strip count, the
/// running texture offset and the three packed coordinate words.
pub(crate) struct BlurPass {
    /// Strips left to emit (`T1`, or `T2` in the last pass).
    cnt: usize,
    /// Running `i * du << 9` accumulator (`T2`, or `T1` in the last pass).
    acc: usize,
    /// Register holding `height << 4 << 16`.
    hh: usize,
    /// Register holding the packed u and v offsets of the strip's first and
    /// second vertex (`uv0`, `uv1`).
    uv0: usize,
    uv1: usize,
    /// The strip set-up adds (`true`) or subtracts the blur offset.
    setup_add: bool,
    /// The per-strip loop adds or subtracts the blur offset.
    body_add: bool,
    /// Cycles of the set-up block.
    setup_cycles: u64,
    /// Loop head, for the event check at the back edge.
    head: u32,
}

/// Emit the strips of one blur pass into the stream block at `T9` (the
/// generated block after "any strips?"): per strip two vertices (`uv`
/// packed words, texture coordinates `x0` and `x1 | height << 20`) as
/// four 16-byte GIF primitives. Leaves every register as the generated
/// loop does (T9 past the packets, `T0`/`A3` advanced).
pub(crate) fn oc1059_blur_strips(c: &mut Cpu, p: &BlurPass) {
    let sp = |c: &Cpu, off: u32| (c.g[SP][0] as u32).wrapping_add(off);
    let off_arg = c.lw(sp(c, 0x100)); // blur offset
    c.cycles += p.setup_cycles;
    let hh_arg = c.lw(sp(c, 0xf8));
    c.g[T5][0] = 0;
    c.g[T4][0] = 3;
    let height16 = sx((hh_arg as u32) << 4);
    let a0 = c.lw(sp(c, 0x10));
    let a2 = c.lw(sp(c, 0x14));
    let step = |a: u64, o: u64, add: bool| {
        if add {
            sx((a as u32).wrapping_add(o as u32))
        } else {
            sx((a as u32).wrapping_sub(o as u32))
        }
    };
    let u_setup = step(a0, off_arg, p.setup_add);
    let v_setup = step(a2, off_arg, p.setup_add);
    c.g[p.hh][0] = height16 << 16;
    c.g[p.uv0][0] = u_setup << 16;
    let du = c.lw(sp(c, 0xc));
    c.g[p.cnt][0] = c.lw(sp(c, 0x18));
    c.g[p.uv1][0] = v_setup << 16;
    c.g[T3][0] = 5;
    c.g[p.acc][0] = 0;
    c.g[T6][0] = sx((du as u32) << 9);
    // The loop overwrites V0, V1, A0, A1 and A2 on its first pass.
    c.g[V0][0] = du;
    c.g[V1][0] = v_setup;
    c.g[A0][0] = a0;
    c.g[A1][0] = off_arg;
    c.g[A2][0] = a2;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(sp(c, 0xc));
        c.cycles += 2;
        c.cycles += 31;
        c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x200));
        c.mult(0, 3, 8, 2);
        let t9 = c.g[T9][0] as u32;
        c.sd(t9.wrapping_add(0x10), c.g[A3][0]);
        c.div(0, p.acc, 30);
        c.g[V0][0] = c.g[T0][0] | c.g[p.hh][0];
        c.sd(t9.wrapping_add(0x8), c.g[T4][0]);
        c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[FP][0] as i64)) as u64;
        c.sd(t9.wrapping_add(0x18), c.g[T3][0]);
        c.g[T0][0] = c.g[A2][0];
        c.sd(t9.wrapping_add(0x28), c.g[T4][0]);
        if c.g[A1][0] == 0 {
            c.g[T0][0] = c.g[FP][0];
        }
        c.sd(t9.wrapping_add(0x38), c.g[T3][0]);
        c.g[p.acc][0] = sx((c.g[p.acc][0] as u32).wrapping_add(c.g[T6][0] as u32));
        c.sd(t9.wrapping_add(0x30), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x200));
        c.g[V0][0] = c.lw(sp(c, 0x8));
        c.g[p.cnt][0] = sx((c.g[p.cnt][0] as u32).wrapping_sub(1));
        c.g[A1][0] = c.lw(sp(c, 0x100));
        c.g[A0][0] = c.lo[0];
        c.div(0, 3, 30);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = step(c.g[A0][0], c.g[A1][0], p.body_add);
        c.g[A0][0] |= c.g[p.uv0][0];
        c.sd(t9, c.g[A0][0]);
        c.g[V1][0] = c.lo[0];
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = step(c.g[V1][0], c.g[A1][0], p.body_add);
        c.g[V1][0] |= c.g[p.uv1][0];
        c.sd(t9.wrapping_add(0x20), c.g[V1][0]);
        let more = c.g[p.cnt][0] != 0;
        c.g[T9][0] = sx(t9.wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, p.head);
        }
    }
}

/// `void PS2GraphicsMan::EmitBlurDownsample(...)` (0x36b9d8): writes the
/// GIF packets of a blur-by-downsample into the scratchpad stream
/// (`gp+0x1384`, via `SprStream_BeginBlock` / `EndBlockAligned`). A header
/// block sets up the GS (frame, z-buffer, scissor, texture, clamp, the grey
/// vertex colour `intensity / passes`), then up to four passes
/// (`passes` at `sp+0x108`, a blur offset at `sp+0x100`) each draw the
/// source rectangle as 32-pixel-wide sprite strips, shifted by
/// (-off,-off), (+off,+off), (+off,-off) and (-off,+off) texels, which
/// the GS blends together.
pub fn ps2_graphics_man_emit_blur_downsample(c: &mut Cpu) {
    use oc1059_blur::{BEGIN_BLOCK, END_BLOCK_ALIGNED};
    guest(c, |c| {
        c.cycles += 198;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S2][0] = c.g[T0][0];
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S4][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[T4][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = sx((c.g[S4][0] as u32) << 4);
        c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[T1][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 4);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        c.g[S5][0] = 0xffffffffffffffff;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S7][0] = ((c.g[S5][0] as i64) < (c.g[S1][0] as i64)) as u64;
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        if c.g[S7][0] != 0 {
            c.g[A1][0] = c.g[S1][0];
        }
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 5) as u32);
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 4);
        c.g[S6][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5a80));
        c.g[S3][0] = c.g[A2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[V1][0] = sx((c.g[T4][0] as u32) << 4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        c.g[A1][0] = c.lw(c.g[A0][0] as u32);
        c.g[FP][0] = sx((c.g[S1][0] as u32) << 4);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.mov_s(20, 12);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T3][0]);
        c.g[A3][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A2][0] = 0xffffffffffffffff;
        c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x0036bab0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        BEGIN_BLOCK.call(c, 0x0036bab0)?;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x108));
        c.cvt_s_w(0, 0);
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[S5][0] = ((c.g[S5][0] as i64) < (c.g[S3][0] as i64)) as u64;
        c.g[T2][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3f));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3f));
        if c.g[S5][0] != 0 {
            c.g[A0][0] = c.g[S3][0];
        }
        c.div_s(20, 20, 0);
        c.mult(0, 10, 6, 10);
        if c.g[S7][0] != 0 {
            c.g[A1][0] = c.g[S1][0];
        }
        c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 6) as u32);
        c.g[V1][0] = 0u64;
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
        c.g[T1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[A3][0] = 0x1;
        c.g[T9][0] = c.g[V0][0];
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[T3][0] = c.g[A3][0];
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        if c.g[T1][0] != 0 {
            c.g[T3][0] = c.g[A0][0];
        }
        if c.g[V1][0] != 0 {
            c.g[A3][0] = c.g[A1][0];
        }
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(0xe));
        c.g[V1][0] = 0x10000000;
        c.g[A0][0] = 0x1000000;
        c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.cvt_w_s(0, 20);
        c.mfc1(8, 0);
        c.g[S6][0] = c.g[S6][0] | c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[S0][0] = c.g[S0][0] << 14;
        c.g[A2][0] = 0xa;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[S0][0] | c.g[A2][0];
        c.sd(c.g[T9][0] as u32, c.g[A1][0]);
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = c.g[A3][0] << 16;
        c.g[T3][0] = c.g[T3][0] << 14;
        c.g[A3][0] = c.g[A0][0] | c.g[A3][0];
        c.g[T3][0] = c.g[V0][0] | c.g[T3][0];
        c.g[V1][0] = 0xffffffffff000000;
        c.g[V1][0] = c.g[V1][0] << 16;
        c.g[V1][0] = c.g[V1][0] | 0x100;
        c.g[V1][0] = c.g[V1][0] << 16;
        c.g[A1][0] = 0xd502;
        c.g[A1][0] = c.g[A1][0] << 19;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
        c.g[T3][0] = c.g[T3][0] | c.g[A1][0];
        c.g[S4][0] = c.g[S4][0] << 4;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xd));
        c.g[V0][0] = 0x8000;
        c.g[V0][0] = c.g[V0][0] << 45;
        c.g[A0][0] = 0x8000;
        c.g[A0][0] = c.g[A0][0] << 17;
        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
        c.g[T2][0] = c.g[T2][0] | c.g[V0][0];
        c.g[S4][0] = c.g[S4][0] | c.g[S0][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[S2][0] = c.g[S2][0] << 24;
        c.g[T1][0] = c.g[T0][0] << 8;
        c.g[S6][0] = c.g[S6][0] | c.g[A0][0];
        c.g[V1][0] = 0xe;
        c.g[A1][0] = 0x4e;
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[S4][0] = c.g[S4][0] | c.g[S2][0];
        c.sd(c.g[T9][0] as u32, c.g[T2][0]);
        c.g[A2][0] = c.g[A2][0] << 48;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x28), c.g[A1][0]);
        c.g[T4][0] = c.g[T4][0] << 34;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x20), c.g[S6][0]);
        c.g[T1][0] = c.g[T0][0] | c.g[T1][0];
        c.g[S1][0] = c.g[S1][0] << 16;
        c.g[A0][0] = 0x4c;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.g[T0][0] = c.g[T0][0] << 16;
        c.g[S1][0] = c.g[S1][0] | c.g[A2][0];
        c.g[S4][0] = c.g[S4][0] | c.g[T4][0];
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = c.g[T1][0] | c.g[T0][0];
        c.g[T4][0] = 0x47;
        c.g[T5][0] = 0x18;
        c.g[T6][0] = 0x80;
        c.g[T6][0] = c.g[T6][0] << 32;
        c.g[T6][0] = c.g[T6][0] | 0x68;
        c.g[T7][0] = 0x42;
        c.g[T8][0] = 0x6;
        c.g[A2][0] = 0x30000;
        c.g[V0][0] = 0x3f800000;
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[V0][0] = c.g[V0][0] | 0x8000;
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = 0x8;
        c.g[S0][0] = 0x3f;
        c.g[S2][0] = 0x40;
        c.g[S3][0] = 0x60;
        c.g[S5][0] = 0x14;
        c.g[T1][0] = c.g[T1][0] | c.g[V0][0];
        c.sd((c.g[T9][0] as u32).wrapping_add(0x30), c.g[A2][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x38), c.g[T4][0]);
        c.g[V0][0] = 0x116;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x48), c.g[T5][0]);
        c.g[V1][0] = 0x1;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x50), c.g[T6][0]);
        c.g[A2][0] = 0x4;
        c.sd((c.g[T9][0] as u32).wrapping_add(0x58), c.g[T7][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x90), c.g[T3][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x98), c.g[T8][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x68), c.g[S0][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x70), c.g[S1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x78), c.g[S2][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x80), c.g[S3][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x88), c.g[S5][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0xa0), c.g[S4][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x40), 0u64);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x60), 0u64);
        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0xb0));
        c.sd(c.g[T9][0] as u32, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x20));
        c.sd((c.g[T9][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.g[RA][0] = sx(0x0036bcc0);
        c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
        END_BLOCK_ALIGNED.call(c, 0x0036bcc0)?;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = c.g[V0][0];
        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
        c.g[RA][0] = sx(0x0036bcdc);
        c.g[A2][0] = 0xffffffffffffffff;
        BEGIN_BLOCK.call(c, 0x0036bcdc)?;
        c.g[T9][0] = c.g[V0][0];
        c.g[T0][0] = 0x200;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0u64;
        if !t {
            oc1059_blur_strips(
                c,
                &BlurPass {
                    cnt: T1,
                    acc: T2,
                    hh: S0,
                    uv0: S1,
                    uv1: T7,
                    setup_add: false,
                    body_add: false,
                    setup_cycles: 18,
                    head: 0x0036_bd38,
                },
            );
        }
        'epilogue: {
            // End pass 1, then pass 2 if `passes >= 2`.
            c.cycles += 13;
            c.g[V0][0] = 0x156;
            c.g[A1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
            c.sd(c.g[T9][0] as u32, c.g[V0][0]);
            c.g[A2][0] = 0x4;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[RA][0] = sx(0x0036bde4);
            c.sd((c.g[T9][0] as u32).wrapping_add(0x8), 0u64);
            END_BLOCK_ALIGNED.call(c, 0x0036bde4)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sw(c.g[V1][0] as u32, c.g[A1][0]);
            if t {
                break 'epilogue;
            }
            // Pass 2: (+off, +off).
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x0036be0c);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036be0c)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[T9][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T0][0] = 0x200;
            c.g[S2][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
            let t = (c.g[A1][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T1,
                        acc: T2,
                        hh: S0,
                        uv0: S1,
                        uv1: T7,
                        setup_add: true,
                        body_add: true,
                        setup_cycles: 18,
                        head: 0x0036_be70,
                    },
                );
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036bf10);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036bf10)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S2][0] != 0u64;
            c.sw(c.g[A2][0] as u32, c.g[V0][0]);
            if t {
                break 'epilogue;
            }
            // Pass 3: (+off, -off).
            c.cycles += 12;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0036bf30);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036bf30)?;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
            c.g[T9][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T0][0] = 0x200;
            c.g[S2][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[A3][0] = 0u64;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T1,
                        acc: T2,
                        hh: S0,
                        uv0: S1,
                        uv1: T7,
                        setup_add: false,
                        body_add: true,
                        setup_cycles: 17,
                        head: 0x0036_bf90,
                    },
                );
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036c030);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036c030)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S2][0] != 0u64;
            c.sw(c.g[A2][0] as u32, c.g[V0][0]);
            if t {
                break 'epilogue;
            }
            // Pass 4: (-off, +off).
            c.cycles += 10;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0036c050);
            c.g[A2][0] = 0xffffffffffffffff;
            BEGIN_BLOCK.call(c, 0x0036c050)?;
            c.g[T9][0] = c.g[V0][0];
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0x200;
            if !t {
                oc1059_blur_strips(
                    c,
                    &BlurPass {
                        cnt: T2,
                        acc: T1,
                        hh: T7,
                        uv0: S1,
                        uv1: S0,
                        setup_add: true,
                        body_add: false,
                        setup_cycles: 17,
                        head: 0x0036_c0a8,
                    },
                );
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
            c.g[A2][0] = 0x4;
            c.g[RA][0] = sx(0x0036c148);
            c.g[A1][0] = c.g[T9][0];
            END_BLOCK_ALIGNED.call(c, 0x0036c148)?;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            c.sw(c.g[A2][0] as u32, c.g[V0][0]);
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// GraphicsMan_DrawIndexedTriangles (0x369cf8)

/// `void GraphicsMan_DrawIndexedTriangles(GraphicsMan *this, int numTris,
/// .., const u8 *indices (a3), const Vec2 *uv (t1), const Color *colours
/// (t2), const Vec3 *positions (t0), const Vec3 *offset (t3))` (0x369cf8):
/// draw `numTris` indexed triangles, each as a 3-vertex strip through the
/// manager's `RenderTriStrip` (vtable +0x238/+0x23c, `(3, verts, 0)`). Each
/// corner is the indexed position plus `*offset` (w = 1), the colour halved
/// and truncated to integers, and the texture coordinate (u, v, 1) from `uv`,
/// or (0, 0, 1) when `uv` is null. The 3 corners (0x30 bytes each) are built
/// on the stack.
pub fn graphics_man_draw_indexed_triangles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
        c.g[V0][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[V1][0] = u64::MAX;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S0][0] = c.g[A3][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[S1][0] = c.g[T3][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S2][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S4][0] = c.g[T1][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S5][0] = c.g[T2][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S6][0] = c.g[T0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S7][0] = c.g[A1][0];
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        'epilogue: {
            'short: {
                if c.g[S4][0] != 0 {
                    // Textured: (u, v) from the UV array.
                    c.cycles += 2;
                    let no_tris = (c.g[S7][0] as i64) <= 0;
                    c.g[V1][0] = 0;
                    if no_tris {
                        break 'short;
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 20);
                    c.mov_s(21, 0);
                    c.g[FP][0] = 0xc;
                    c.lwc1(9, c.g[S1][0] as u32);
                    loop {
                        c.cycles += 6;
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.lwc1(8, (c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = 0u64;
                        c.lwc1(7, (c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                        c.g[A2][0] = c.g[SP][0];
                        loop {
                            c.cycles += 47;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = ((c.g[A3][0] as i64) < 3i64) as u64;
                            c.mult(0, 2, 4, 30);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            c.lwc1(0, c.g[V1][0] as u32);
                            c.add_s(2, 2, 7);
                            c.add_s(1, 1, 8);
                            c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.add_s(0, 0, 9);
                            c.lwc1(5, c.g[V0][0] as u32);
                            c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x98));
                            c.mul_s(6, 6, 20);
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                            c.mul_s(5, 5, 20);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mul_s(3, 3, 20);
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x9c));
                            c.mul_s(4, 4, 20);
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lwc1(0, c.g[A0][0] as u32);
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.swc1(0, c.g[A2][0] as u32);
                            c.cvt_w_s(1, 5);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x1c));
                            c.sq(2, (c.g[A2][0] as u32).wrapping_add(0x20));
                            c.cvt_w_s(1, 3);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
                            c.cvt_w_s(1, 4);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
                            c.cvt_w_s(0, 6);
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            let t = c.g[A1][0] != 0u64;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0036_9da0);
                                }
                                continue;
                            }
                            break;
                        }
                        draw_tri(c, 0x0036_9e80)?;
                        c.g[V1][0] = c.g[S3][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.lwc1(9, c.g[S1][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9d88);
                            }
                            continue;
                        }
                        c.cycles += 2;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        break 'epilogue;
                    }
                }
                // Untextured: (0, 0, 1).
                c.cycles += 2;
                let no_tris = (c.g[S7][0] as i64) <= 0;
                c.g[V1][0] = 0;
                if no_tris {
                    break 'short;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 20);
                c.mov_s(21, 0);
                c.g[S4][0] = 0xc;
                c.lwc1(9, c.g[S1][0] as u32);
                loop {
                    c.cycles += 6;
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.lwc1(8, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0u64;
                    c.lwc1(7, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                    c.g[A1][0] = c.g[SP][0];
                    loop {
                        c.cycles += 44;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                        c.mult(0, 7, 3, 20);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S6][0] as u32));
                        c.lwc1(5, c.g[V1][0] as u32);
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.mul_s(6, 6, 20);
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                        c.mul_s(5, 5, 20);
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.add_s(2, 2, 7);
                        c.add_s(1, 1, 8);
                        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.add_s(0, 0, 9);
                        c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x98));
                        c.mul_s(3, 3, 20);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                        c.mul_s(4, 4, 20);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x9c));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb8));
                        c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x20));
                        c.cvt_w_s(0, 5);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x1c));
                        c.swc1(21, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.cvt_w_s(0, 3);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sw(c.g[A1][0] as u32, 0u64);
                        c.cvt_w_s(0, 4);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), 0u64);
                        c.cvt_w_s(0, 6);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
                        let t = c.g[A0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9ed0);
                            }
                            continue;
                        }
                        break;
                    }
                    draw_tri(c, 0x0036_9fa4)?;
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.lwc1(9, c.g[S1][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0036_9eb8);
                        }
                        continue;
                    }
                    break 'short;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        }
        c.cycles += 13;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
        c.pc = t;
        Ok(())
    })
}

/// DrawIndexedTriangles: `this->RenderTriStrip(3, verts, 0)` through the
/// manager's vtable (`s2` is `this`, the vertices are at `sp`).
pub(crate) fn draw_tri(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 13;
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
    c.g[S0][0] = c.g[T0][0];
    c.g[A1][0] = 0x3;
    c.g[A2][0] = c.g[SP][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_EmitDepthFogZCopy (0x36ae20), PS2GraphicsMan_EmitDepthFogBlend (0x36b158)

/// `void PS2GraphicsMan_EmitDepthFogZCopy(PS2GraphicsMan *this, int idx,
/// u32 texBase, u32 frameBase, int n, u64 **pp)` (0x36ae20; the generated
/// body does not use `idx`): append a GIF packet at `**pp` that copies the
/// Z buffer page into the frame buffer as a row of sprites (the depth-fog
/// pre-pass): a DMA tag, the register setup (FRAME at `frameBase`, ZBUF at
/// the current Z page, XYOFFSET centred in the 4096x4096 window, SCISSOR,
/// TEX0 at `texBase`, PRIM sprite), then one pair of sprites per two 8-pixel
/// columns of the framebuffer, `n` rows high. `*pp` advances past the packet.
pub fn ps2_graphics_man_emit_depth_fog_z_copy(c: &mut Cpu) {
    guest(c, |c| {
        'packet: {
            // Prologue; fb offset X = (0x1000 - width) / 2 << 4 (at sp+8).
            c.cycles += 141;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = 0x1000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A3][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
            c.sw(c.g[SP][0] as u32, c.g[T0][0]);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a30));
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let t = c.g[V0][0] as u32;
            c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a34));
            call_ptr(c, t, 0x0036ae80)?;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036aea4)?;
            // The packet header: DMA tag, then GIF tag (A+D) with 2*cols+11 qwords.
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S2][0];
            }
            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a80));
            c.g[FP][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
            c.g[S7][0] = c.lw(c.g[A1][0] as u32);
            c.g[V1][0] = sx((c.g[FP][0] as u32) << 1);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xb));
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[A0][0] = 0xe;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.sd(c.g[S7][0] as u32, c.g[V1][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32) >> 1);
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 4);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[S3][0]);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036af0c)?;
            // FRAME_1 / ZBUF_1 / XYOFFSET / SCISSOR / TEX0 and the sprite PRIM, as 64-bit stores.
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[V1][0] = 0x1000000;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[V1][0] = 0x3fff;
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V1][0] = c.g[V1][0] | 0x200;
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.g[S6][0] | c.g[V0][0];
            c.g[S0][0] = c.g[S0][0] << 32;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = c.g[S0][0] >> 32;
            c.g[A0][0] = 0x8000;
            c.g[A0][0] = c.g[A0][0] << 17;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.g[S3][0] << 32;
            c.g[A1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
            c.g[S2][0] = c.g[S2][0] << 16;
            c.g[S1][0] = c.g[S1][0] << 48;
            c.g[A2][0] = 0x30000;
            c.g[T1][0] = 0x40;
            c.g[S2][0] = c.g[S2][0] | c.g[S1][0];
            c.g[V1][0] = 0x4c;
            c.g[A0][0] = 0x4e;
            c.g[A3][0] = 0x47;
            c.g[T0][0] = 0x18;
            c.g[T2][0] = 0x3f;
            c.g[T3][0] = 0x8000;
            c.g[T3][0] = c.g[T3][0] << 24;
            c.g[T4][0] = 0x3b;
            c.g[T5][0] = 0x14;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x40), c.g[A1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x28), c.g[A0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x48), c.g[T0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x50), c.g[S2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x68), c.g[T2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x70), c.g[T3][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x78), c.g[T4][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x88), c.g[T5][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036aff0)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 5);
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V1][0] = 0xeab2;
            c.g[V1][0] = c.g[V1][0] << 20;
            c.g[V0][0] = c.g[V0][0] >> 18;
            c.g[A1][0] = 0x6;
            c.g[V0][0] = c.g[S5][0] | c.g[V0][0];
            c.g[A2][0] = 0x116;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A0][0] = 0x3f800000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[V1][0] = 0x1;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x90), c.g[V0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x98), c.g[A1][0]);
            c.g[T1][0] = 0u64;
            c.sd((c.g[S7][0] as u32).wrapping_add(0xa0), c.g[A2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xb0), c.g[A0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xb8), c.g[V1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xa8), 0u64);
            let t = (c.g[FP][0] as i64) <= 0;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0xc0));
            if t {
                break 'packet;
            }
            c.cycles += 17;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[T7][0] = c.g[S3][0] << 16;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x80000;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[T5][0] = 0x3;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[T4][0] = 0x5;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T0][0] = c.g[V0][0] << 16;
            c.g[T6][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[T3][0] = 0x88;
            c.g[T2][0] = 0x8;
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x100));
            // Sprite pairs, two columns per iteration.
            loop {
                c.cycles += 20;
                c.g[V0][0] = c.g[T2][0] | c.g[S0][0];
                c.g[V1][0] = c.g[A3][0] | c.g[T7][0];
                c.g[A0][0] = c.g[T3][0] | c.g[T6][0];
                c.g[A1][0] = c.g[A2][0] | c.g[T0][0];
                c.sd(c.g[S7][0] as u32, c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                c.sd((c.g[S7][0] as u32).wrapping_add(0x28), c.g[T5][0]);
                c.sd((c.g[S7][0] as u32).wrapping_add(0x38), c.g[T4][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x40));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_b098);
                    }
                    continue;
                }
                break;
            }
        }
        // Patch the DMA tag with the qword count and publish the new end.
        c.cycles += 27;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 0x10000000;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
        c.sw(c.g[A0][0] as u32, c.g[S7][0]);
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitDepthFogBlend(PS2GraphicsMan *this, int idx,
/// u64 y, u64 **pp)` (0x36b158): append the second depth-fog packet at
/// `**pp`: blend the frame against the Z-copied texture, as vertical sprite
/// strips over the region `idx` (the rectangle at `this + idx*8 + 0x59b0`:
/// x, y, w, h), `w/32` strips of 32 pixels (two sprites each), the final
/// fog-colour sprite and the closing GIF tag. `*pp` advances past it.
pub fn ps2_graphics_man_emit_depth_fog_blend(c: &mut Cpu) {
    guest(c, |c| {
        'strips: {
            // Prologue; the region rectangle and the framebuffer offsets.
            c.cycles += 166;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[T1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[T0][0] = c.g[T1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.g[T0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[A1][0] = c.g[V0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S4][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = 0x1000;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[FP][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b2));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.sw(c.g[SP][0] as u32, c.g[A3][0]);
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[S3][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x59b0));
            c.g[S0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x59b4));
            c.g[S2][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x59b6));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
            let t = c.g[V0][0] as u32;
            c.g[S2][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[S2][0] as u32));
            call_ptr(c, t, 0x0036b1d4)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            let t = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036b200)?;
            // The packet header and the register setup (TEX0 at the Z page, FRAME, SCISSOR, PRIM).
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S0][0];
            }
            c.g[S6][0] = c.lw(c.g[A1][0] as u32);
            c.g[S5][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
            c.g[A2][0] = 0x10000000;
            c.g[A1][0] = sx((c.g[S5][0] as u32) << 2);
            c.g[S3][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a84));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S2][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a80));
            c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb));
            c.sd(c.g[S6][0] as u32, c.g[A2][0]);
            c.g[A0][0] = 0x8000;
            c.g[A0][0] = c.g[A0][0] << 45;
            c.g[A2][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0xe;
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[FP][0] as u32));
            c.sd(c.g[S6][0] as u32, c.g[A1][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036b288)?;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.g[S3][0] << 32;
            c.g[V1][0] = 0x1000000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[S2][0] = c.g[S2][0] | c.g[V1][0];
            c.g[S0][0] = c.g[S0][0] << 16;
            c.g[S0][0] = c.g[A0][0] | c.g[S0][0];
            c.g[V1][0] = c.g[FP][0] << 32;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[S2][0] = c.g[S2][0] << 32;
            c.g[A1][0] = c.g[A1][0] >> 32;
            c.g[S2][0] = c.g[S2][0] >> 32;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[V1][0] = 0x8000;
            c.g[V1][0] = c.g[V1][0] << 17;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[S2][0] = c.g[S2][0] | c.g[V1][0];
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0x4e;
            c.g[A0][0] = c.g[A0][0] << 48;
            c.g[A2][0] = c.g[S1][0] << 32;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
            c.g[A2][0] = c.g[V0][0] | c.g[A2][0];
            c.g[A0][0] = 0x70000;
            c.g[V0][0] = 0x4c;
            c.g[A3][0] = 0x47;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x3f;
            c.g[T3][0] = 0x40;
            c.g[T4][0] = 0x14;
            c.g[V1][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x40), c.g[A2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[T0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x68), c.g[T2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x70), c.g[S0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x78), c.g[T3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x88), c.g[T4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x50), c.g[V1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036b374)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 5);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S3][0] = c.g[S3][0] << 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 18;
            c.g[V1][0] = 0xea9b;
            c.g[V1][0] = c.g[V1][0] << 20;
            c.g[S3][0] = c.g[S3][0] >> 32;
            c.g[S4][0] = c.g[S4][0] << 37;
            c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
            c.g[S4][0] = c.g[S4][0] | c.g[V1][0];
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 46;
            c.g[A0][0] = 0x3f800000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[S3][0] = c.g[S3][0] | c.g[S4][0];
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb0), c.g[A0][0]);
            c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
            c.g[V1][0] = 0x6;
            c.g[V0][0] = 0x156;
            c.g[A0][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x90), c.g[S3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x98), c.g[V1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa8), 0u64);
            let t = c.g[S5][0] == 0u64;
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc0));
            if t {
                break 'strips;
            }
            c.cycles += 24;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 4);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[T2][0] = c.g[S5][0];
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[T5][0] = 0x3;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S1][0] = c.g[A1][0] << 16;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[T7][0] = c.g[V0][0] << 16;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[S0][0] = c.g[A0][0] << 16;
            c.g[T6][0] = c.g[V1][0] << 16;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[T4][0] = 0xffff;
            c.g[T4][0] = c.g[T4][0] << 32;
            c.g[T3][0] = 0x5;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x208));
            // One iteration per strip.
            loop {
                c.cycles += 21;
                c.g[V1][0] = c.g[T0][0] | c.g[S0][0];
                c.g[A0][0] = c.g[T1][0] | c.g[S1][0];
                c.g[V0][0] = c.g[A3][0] | c.g[T7][0];
                c.g[A1][0] = c.g[A2][0] | c.g[T6][0];
                c.g[V1][0] = c.g[V1][0] | c.g[T4][0];
                c.g[A0][0] = c.g[A0][0] | c.g[T4][0];
                c.sd(c.g[S6][0] as u32, c.g[V0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[T5][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[T3][0]);
                let t = c.g[T2][0] != 0u64;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_b450);
                    }
                    continue;
                }
                break;
            }
        }
        // The closing sprites (fog colour over the framebuffer) and the epilogue.
        c.cycles += 81;
        c.g[V0][0] = 0x10000000;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.g[V1][0] = 0x10000000;
        c.g[V1][0] = c.g[V1][0] << 32;
        c.g[V1][0] = c.g[V1][0] | 0x8003;
        c.g[A0][0] = 0xe;
        c.sd(c.g[S6][0] as u32, c.g[V0][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[S1][0] = 0x1000;
        c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b4e8)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 16;
        c.g[V1][0] = 0x4c;
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b520)?;
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b540)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A3][0] = 0x40;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V0][0] = 0x18;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S1][0] = c.g[S1][0] << 32;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[S0][0] | c.g[S1][0];
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5900));
        c.g[V1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5904));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5902));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5906));
        c.g[V1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5900));
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5902));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 48;
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[A3][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sd((c.g[S6][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A2][0] = c.lw(c.g[SP][0] as u32);
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw(c.g[A2][0] as u32, c.g[S6][0]);
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitDepthFogFlatFill(PS2GraphicsMan* gm, int idx, uint64** pp)`
/// (0x36b5e8): emit the GS packet (a CLEAR/fill sprite strip, one 32 pixel
/// column per sprite pair) that fills saved rectangle `idx` of the graphics
/// manager with its flat fog colour (`gm+idx*0x20+0x6c64`: r, g, b floats
/// scaled by 256), then the frame/z restore packet; the rectangle size comes
/// from the manager's virtual width/height queries (vtable slots 0x50/0x58).
/// The packet is built at `*pp` and `*pp` is advanced past it.
pub fn ps2_graphics_man_emit_depth_fog_flat_fill(c: &mut Cpu) {
    guest(c, |c| {
        'b_36b85c: {
            c.cycles += 133;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 5);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[T0][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6c64));
            c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.g[S0][0] = 0x1000;
            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b6));
            c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x50));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw(c.g[SP][0] as u32, c.g[A2][0]);
            c.g[S1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x59b4));
            c.g[S2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x59b0));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b66c);
            c.g[S4][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x59b2));
            call_ptr(c, t, 0x0036b66c)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[FP][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b690);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036b690)?;
            c.g[V1][0] = 0xffffffffffffffff;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1f));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S1][0];
            }
            c.g[S2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a84));
            c.g[S7][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
            c.g[S5][0] = c.lw(c.g[A1][0] as u32);
            c.g[V1][0] = sx((c.g[S7][0] as u32) << 1);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7));
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a80));
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[A0][0] = 0xe;
            c.sd(c.g[S5][0] as u32, c.g[V1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0036b6fc);
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0036b6fc)?;
            c.g[V1][0] = 0x1000000;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[S2][0] = c.g[S2][0] << 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S1][0] = c.g[S1][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[V1][0] = 0x8000;
            c.g[V1][0] = c.g[V1][0] << 17;
            c.g[T4][0] = c.g[S0][0];
            c.g[S2][0] = c.g[S2][0] >> 32;
            c.g[S1][0] = c.g[S1][0] >> 32;
            c.g[S2][0] = c.g[S2][0] | c.g[V0][0];
            c.g[S1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[A0][0] = c.g[T4][0] << 32;
            c.g[A0][0] = c.g[FP][0] | c.g[A0][0];
            c.g[V1][0] = 0x4e;
            c.g[V0][0] = 0x4c;
            c.g[A1][0] = 0x70000;
            c.g[A2][0] = 0x47;
            c.g[A3][0] = 0x18;
            c.g[T0][0] = 0x11;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x46;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x40), c.g[A0][0]);
            c.g[T3][0] = 0x3f800000;
            c.g[T3][0] = c.g[T3][0] << 16;
            c.g[T3][0] = c.g[T3][0] | 0x8000;
            c.g[T3][0] = c.g[T3][0] << 16;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x30), c.g[A1][0]);
            c.g[V0][0] = 0x1;
            c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x20), c.g[S1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x38), c.g[A2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x48), c.g[A3][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x50), c.g[T0][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x60), c.g[T2][0]);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x68), 0u64);
            c.g[AT][0] = 0x43800000;
            c.mtc1(1, 1);
            c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x18));
            c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0x1c));
            c.mul_s(2, 2, 1);
            c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x14));
            c.mul_s(3, 3, 1);
            c.sd((c.g[S5][0] as u32).wrapping_add(0x78), c.g[V0][0]);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 2);
            c.mfc1(2, 1);
            c.cvt_w_s(1, 3);
            c.mfc1(3, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[V0][0] = c.g[V0][0] << 8;
            c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
            c.sd((c.g[S5][0] as u32).wrapping_add(0x70), c.g[V0][0]);
            let t = c.g[S7][0] == 0u64;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x80));
            if t {
                break 'b_36b85c;
            } else {
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = c.g[T4][0] << 16;
                c.g[A0][0] = c.g[S7][0];
                c.g[A2][0] = c.g[FP][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[T0][0] = 0xfffe;
                c.g[T0][0] = c.g[T0][0] << 31;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A3][0] = 0x5;
                c.g[S0][0] = c.g[V0][0] << 16;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x200));
                'l_36b828: loop {
                    c.cycles += 13;
                    c.g[V0][0] = c.g[A2][0] | c.g[T1][0];
                    c.g[V1][0] = c.g[A1][0] | c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                    c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
                    c.sd(c.g[S5][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    let t = c.g[A0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036b828);
                        }
                        continue 'l_36b828;
                    } else {
                        break 'b_36b85c;
                    }
                }
            }
        }
        c.cycles += 95;
        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
        c.g[A3][0] = 0x10000000;
        c.g[A3][0] = c.g[A3][0] | 0x4;
        c.g[T0][0] = 0x10000000;
        c.g[T0][0] = c.g[T0][0] << 32;
        c.g[T0][0] = c.g[T0][0] | 0x8003;
        c.g[A2][0] = 0xe;
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[S1][0] = 0x1000;
        c.g[A1][0] = 0x10000000;
        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sd(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
        c.sd(c.g[S5][0] as u32, c.g[A3][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x10), c.g[T0][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5a84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b8d8);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b8d8)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 16;
        c.g[V1][0] = 0x4c;
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.sd((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b910);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b910)?;
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0036b930);
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0036b930)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A3][0] = 0x40;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V0][0] = 0x18;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
        c.sd((c.g[S5][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S1][0] = c.g[S1][0] << 32;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S1][0] = c.g[S0][0] | c.g[S1][0];
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd((c.g[S5][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5900));
        c.g[V1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5904));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5902));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5906));
        c.g[V1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5900));
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x5902));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 48;
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.sd((c.g[S5][0] as u32).wrapping_add(0x48), c.g[A3][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sw(c.g[V1][0] as u32, c.g[S5][0]);
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2GraphicsMan_EmitBlendedFill(void* self, int x, int w, int h, int y,
/// int alpha, uint32* handle)` (0x36c188; argument names from the reference): build a GS packet in a scratchpad
/// stream block (`SprStream_BeginBlock` on the stream at gp+0x1384): a sprite
/// strip of `w/32` columns with the grey/alpha blend registers, then
/// `SprStream_EndBlockAligned` and store the next block handle in `*handle`.
pub fn ps2_graphics_man_emit_blended_fill(c: &mut Cpu) {
    guest(c, |c| {
        'b_36c1e8: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A2][0];
            c.g[S2][0] = 0xffffffffffffffff;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f));
            c.g[S2][0] = ((c.g[S2][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            if c.g[S2][0] != 0 {
                c.g[V0][0] = c.g[S0][0];
            }
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S6][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.div(0, 16, 22);
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = 0u64;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A1][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S5][0] = c.g[T1][0];
            if c.g[S6][0] == c.g[S3][0] {
                break 'b_36c1e8;
            } else {
                break 'b_36c1e8;
            }
        }
        'b_36c350: {
            c.cycles += 74;
            c.g[A2][0] = 0xffffffffffffffff;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T2][0]);
            c.sw(c.g[SP][0] as u32, c.g[A3][0]);
            c.g[FP][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.lw(c.g[T2][0] as u32);
            c.g[A0][0] = c.g[FP][0];
            c.g[V0][0] = c.lo[0];
            c.g[RA][0] = sx(0x0036c210);
            c.g[S7][0] = sx((c.g[V0][0] as u32) << 4);
            Callee {
                addr: 0x0038f460,
                f: ee::s12::SprStream_BeginBlock,
            }
            .call(c, 0x0036c210)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f));
            c.g[A3][0] = sx((c.g[S6][0] as u32) << 1);
            if c.g[S2][0] != 0 {
                c.g[V1][0] = c.g[S0][0];
            }
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[A0][0] = 0x10000000;
            c.g[S3][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A1][0] = 0x1;
            c.g[T3][0] = c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
            if c.g[S3][0] != 0 {
                c.g[A1][0] = c.g[V1][0];
            }
            c.sd(c.g[T3][0] as u32, c.g[A2][0]);
            c.g[V1][0] = c.g[S1][0] << 8;
            c.g[S4][0] = sx(((c.g[S4][0] as u32) as i32 >> 5) as u32);
            c.g[A1][0] = c.g[A1][0] << 16;
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 45;
            c.g[A0][0] = 0xffffffffff000000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x100;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A2][0] = 0xa1;
            c.g[V1][0] = c.g[S1][0] | c.g[V1][0];
            c.g[A1][0] = c.g[S4][0] | c.g[A1][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6));
            c.g[S5][0] = c.g[S5][0] << 32;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
            c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
            c.g[S5][0] = c.g[S5][0] | c.g[A2][0];
            c.g[S1][0] = c.g[S1][0] << 16;
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0xe;
            c.g[A2][0] = 0x4c;
            c.g[V0][0] = 0x3f800000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[V0][0] = c.g[V0][0] | 0x8000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x3f;
            c.g[V1][0] = c.g[V1][0] | c.g[S1][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = 0x146;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A2][0]);
            c.g[A0][0] = 0x1;
            c.sd(c.g[T3][0] as u32, c.g[A3][0]);
            c.g[A2][0] = 0u64;
            c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[T0][0]);
            c.g[A1][0] = c.g[S7][0];
            c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[S5][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[T1][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x48), c.g[T2][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x20), 0u64);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x40), 0u64);
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x50));
            c.sd(c.g[T3][0] as u32, c.g[V0][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
            let t = (c.g[S6][0] as i64) <= 0;
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_36c350;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A3][0] = c.g[S6][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = 0x5;
                c.g[A0][0] = c.g[V0][0] << 16;
                'l_36c328: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
                    c.sd(c.g[T3][0] as u32, c.g[A2][0]);
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A3][0] != 0u64;
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036c328);
                        }
                        continue 'l_36c328;
                    } else {
                        break 'b_36c350;
                    }
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = c.g[T3][0];
        c.g[RA][0] = sx(0x0036c360);
        c.g[A2][0] = 0x4;
        Callee {
            addr: 0x0038f668,
            f: ee::s12::SprStream_EndBlockAligned,
        }
        .call(c, 0x0036c360)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2GraphicsMan_EmitBloomComposite(PS2GraphicsMan* gm, ..., uint32* handle)`
/// (0x36c398): like `EmitBlendedFill`, but a textured strip: a scratchpad
/// stream block with the frame/z/texture registers (the frame width comes from
/// the manager's virtual width query, vtable slot 0x50) and one textured
/// sprite per 32 pixel column; ends with `SprStream_EndBlockAligned` and
/// stores the next block handle in `*handle` (stack argument at +0xe8).
pub fn ps2_graphics_man_emit_bloom_composite(c: &mut Cpu) {
    guest(c, |c| {
        'b_36c3fc: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[V0][0] = 0xffffffffffffffff;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[T3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1f));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S3][0];
            }
            c.g[FP][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
            c.g[S2][0] = c.g[A2][0];
            c.div(0, 18, 30);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S0][0] = c.g[T0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A2][0] = 0xffffffffffffffff;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[S7][0] = c.g[T1][0];
            if c.g[FP][0] == 0u64 {
                break 'b_36c3fc;
            } else {
                break 'b_36c3fc;
            }
        }
        'b_36c4e8: {
            'b_36c4d4: {
                c.cycles += 39;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                c.g[A3][0] = 0u64;
                c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a80));
                c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                c.g[A0][0] = c.g[V1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[RA][0] = sx(0x0036c434);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                Callee {
                    addr: 0x0038f460,
                    f: ee::s12::SprStream_BeginBlock,
                }
                .call(c, 0x0036c434)?;
                c.g[A0][0] = sx((c.g[FP][0] as u32) << 2);
                c.g[S6][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd));
                c.g[A1][0] = 0x10000000;
                c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = 0x8000;
                c.g[V0][0] = c.g[V0][0] << 45;
                c.sd(c.g[S6][0] as u32, c.g[V1][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0xe;
                c.sd(c.g[S6][0] as u32, c.g[A0][0]);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 5) as u32);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0036c48c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0036c48c)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_36c4d4;
                } else {
                    c.cycles += 15;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0036c4ac);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0036c4ac)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
                    c.g[V1][0] = 0xffffffffff000000;
                    c.g[V1][0] = c.g[V1][0] << 16;
                    c.g[V1][0] = c.g[V1][0] | 0x100;
                    c.g[V1][0] = c.g[V1][0] << 16;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = c.g[V0][0] >> 16;
                    c.g[V0][0] = c.g[S0][0] | c.g[V0][0];
                    let t = true;
                    c.g[S0][0] = c.g[V0][0] | c.g[V1][0];
                    if t {
                        break 'b_36c4e8;
                    } else {
                        break 'b_36c4d4;
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffff000000;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[V0][0] = c.g[V0][0] | 0x101;
            c.g[V0][0] = c.g[V0][0] << 16;
            c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
            break 'b_36c4e8;
        }
        'b_36c6f4: {
            c.cycles += 100;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[T0][0] = 0x1;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[S2][0];
            }
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd8));
            c.g[V0][0] = ((0u64 as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[S3][0] as u32));
            if c.g[V0][0] != 0 {
                c.g[T0][0] = c.g[V1][0];
            }
            c.g[T1][0] = c.g[A2][0] << 8;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[T1][0] = c.g[A2][0] | c.g[T1][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.g[A0][0] << 16;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.g[A0][0] = c.g[S7][0] | c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x1000000;
            c.g[T0][0] = c.g[T0][0] << 14;
            c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.g[S5][0] | c.g[A1][0];
            c.g[V0][0] = c.g[A2][0] << 16;
            c.g[T2][0] = c.g[T2][0] << 34;
            c.g[T1][0] = c.g[T1][0] | c.g[V0][0];
            c.g[A2][0] = c.g[A2][0] << 24;
            c.g[V0][0] = c.g[V1][0] << 32;
            c.g[T1][0] = c.g[T1][0] | c.g[A2][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A2][0] = 0xd502;
            c.g[A2][0] = c.g[A2][0] << 19;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0xa;
            c.g[T0][0] = c.g[V0][0] | c.g[T0][0];
            c.g[T2][0] = c.g[T2][0] | c.g[V1][0];
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 17;
            c.g[V1][0] = 0x29;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[T0][0] = c.g[T0][0] | c.g[A2][0];
            c.g[T3][0] = c.g[T3][0] << 32;
            c.sd(c.g[S4][0] as u32, c.g[S0][0]);
            c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[T3][0] = c.g[T3][0] | c.g[V1][0];
            c.g[V0][0] = 0x4c;
            c.g[A2][0] = 0x30000;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A1][0]);
            c.g[A3][0] = c.g[A3][0] << 48;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.g[T4][0] = 0xfe00;
            c.g[T4][0] = c.g[T4][0] << 46;
            c.g[V1][0] = 0x4e;
            c.g[T5][0] = c.g[T5][0] << 14;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | c.g[A3][0];
            c.g[T5][0] = c.g[T5][0] | c.g[T2][0];
            c.g[T1][0] = c.g[T1][0] | c.g[T4][0];
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = 0x47;
            c.g[T2][0] = 0x18;
            c.g[T4][0] = 0x42;
            c.g[T6][0] = 0x3f;
            c.g[T7][0] = 0x40;
            c.g[S0][0] = 0x60;
            c.g[S1][0] = 0x14;
            c.g[S2][0] = 0x6;
            c.g[S3][0] = 0x8;
            c.g[S4][0] = 0x156;
            c.g[S5][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.g[T9][0] = sx((c.g[S7][0] as u32) << 4);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[T2][0]);
            c.g[S7][0] = 0x8;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x50), c.g[T3][0]);
            c.g[T8][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sd((c.g[S6][0] as u32).wrapping_add(0x58), c.g[T4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x68), c.g[T6][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x70), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x78), c.g[T7][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x80), c.g[S0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x88), c.g[S1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x90), c.g[T0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x98), c.g[S2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa0), c.g[T5][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa8), c.g[S3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb0), c.g[S4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xc0), c.g[T1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xc8), c.g[S5][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x40), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb8), 0u64);
            let t = (c.g[FP][0] as i64) <= 0;
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xd0));
            if t {
                break 'b_36c6f4;
            } else {
                c.cycles += 12;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.g[FP][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = 0x3;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[T0][0] = 0x5;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
                c.g[T2][0] = c.g[V0][0] << 16;
                c.g[A3][0] = c.g[V1][0] << 16;
                c.g[T3][0] = c.g[A0][0] << 16;
                'l_36c6a8: loop {
                    c.cycles += 19;
                    c.g[A0][0] = c.g[T8][0] | c.g[A3][0];
                    c.g[V0][0] = c.g[T9][0] | c.g[T2][0];
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x200));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A1][0] = c.g[V1][0] | c.g[T3][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[T9][0] = c.g[V1][0];
                    c.sd(c.g[S6][0] as u32, c.g[S7][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                    c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[T1][0]);
                    c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                    let t = c.g[A2][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0036c6a8);
                        }
                        continue 'l_36c6a8;
                    } else {
                        break 'b_36c6f4;
                    }
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x0036c704);
        c.g[A2][0] = 0x4;
        Callee {
            addr: 0x0038f668,
            f: ee::s12::SprStream_EndBlockAligned,
        }
        .call(c, 0x0036c704)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void GraphicsMan_EmptyHook(void)` (0x369ff0): a hook that does nothing.
pub fn graphics_man_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void Emitter_SetDescLinearPath(Emitter* self, sGfxVec4* a, sGfxVec4* b)`
/// (0x36ce80, tParticleEmitter_setLine): stores `2a - b` at +0xB0, `b - a` at
/// +0xC0 and the constant quad at 0x4ff120 at +0xD0.
pub fn emitter_set_desc_linear_path(c: &mut Cpu) {
    c.cycles += 30;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    let a = c.g[A1][0] as u32;
    let b = c.g[A2][0] as u32;
    let t20 = frame.at(c, 0x20);
    let t10 = frame.at(c, 0x10);
    let t0 = frame.at(c, 0);
    c.g[AT][0] = 0x4000_0000; // 2.0f
    c.mtc1(1, 0);
    c.mfc1(2, 0);
    c.lqc2(4, a);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X)); // 2 * a
    c.sqc2(5, t20);
    c.g[A3][0] = 0x0050_0000;
    c.lq(2, t20);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xee0)); // 0x4ff120
    c.sq(2, t10);
    c.lqc2(3, t10);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4)); // 2a - b
    c.sqc2(5, t20);
    c.lq(2, t20);
    c.sq(2, this.wrapping_add(0xb0));
    c.sq(2, t0);
    c.lqc2(3, b);
    c.lqc2(4, a);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4)); // b - a
    c.sqc2(5, t10);
    c.lq(3, t10);
    c.sq(3, this.wrapping_add(0xc0));
    c.sq(3, t0);
    let quad_addr = c.g[A3][0] as u32;
    c.lq(2, quad_addr);
    c.sq(2, this.wrapping_add(0xd0));
    frame.close(c);
}

/// `void Emitter_SetDescAccelScaled(Emitter* self, sGfxVec4* v, float s)`
/// (0x36cfa8, tParticleEmitter_setMotion): stores `(v * self.+0xC) / (s *
/// self.+0x4C)` at +0xE0.
pub fn emitter_set_desc_accel_scaled(c: &mut Cpu) {
    c.cycles += 23;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    let v = c.g[A1][0] as u32;
    let t20 = frame.at(c, 0x20);
    let t10 = frame.at(c, 0x10);
    let t0 = frame.at(c, 0);
    c.lwc1(0, this.wrapping_add(0xc));
    c.mfc1(3, 0);
    c.lqc2(4, v);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X)); // v * self.+0xC
    c.sqc2(5, t20);
    c.lq(2, t20);
    c.lwc1(0, this.wrapping_add(0x4c));
    c.sq(2, t10);
    c.mul_s(12, 12, 0); // s * self.+0x4C
    c.mfc1(3, 12);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, t10);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, t20);
    c.lq(2, t20);
    c.sq(2, this.wrapping_add(0xe0));
    c.sq(2, t0);
    frame.close(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::compare;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;
    use crate::difftest::put_vec;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base;

    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::fe_render_fixture;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_2;

    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;

    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::f;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE;

    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;

    use ssx3_ee as ee;

    use ssx3_ee::s12;

    use ssx3_ee_rt::cpu::sx;

    #[test]
    fn deferred_delete_push_matches() {
        check(
            ssx3_ee::s12::f_0036a1c0,
            deferred_delete_push,
            200,
            &|c, seed| {
                let mut r = Rng(0x36a1_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                let count = r.next_u32() % 8;
                c.bus.write32(GAME_GP + 0x2a98, count);
                c.bus.write32(0x53ab40 + count * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_reset_depth_fog_params_matches() {
        check(
            ssx3_ee::s12::f_0036aba0,
            ps2_graphics_man_reset_depth_fog_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x36ab_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(GAME_GP + 0x123c, r.edge_f32());
                c.bus.write32(GAME_GP + 0x1240, r.edge_f32());
                c.bus.write32(GAME_GP + 0x1244, r.next_u32());
                c.bus.write32(GAME_GP + 0x1248, r.edge_f32());
            },
        );
    }

    #[test]
    fn emitter_set_desc_time_matches() {
        check(
            ssx3_ee::s12::f_0036d3d8,
            emitter_set_desc_time,
            200,
            &|c, seed| {
                let mut r = Rng(0x36d3_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(DATA + 0x4c, r.edge_f32());
            },
        );
    }

    #[test]
    fn emitter_advance_desc_time_matches() {
        check(
            ssx3_ee::s12::f_0036d3e8,
            emitter_advance_desc_time,
            200,
            &|c, seed| {
                let mut r = Rng(0x36d3_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(DATA + 0x4c, r.edge_f32());
                c.bus.write32(DATA + 0x8, r.edge_f32());
            },
        );
    }

    #[test]
    fn emitter_render_matches() {
        check(
            ee::s12::f_003708c0__generated,
            emitter_render,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x08c0, 3);
                put_ctx(c, &mut r, &[0x158, 0x2e8, 0x290]);
                let e = BASE;
                c.bus.write32(e + 0xc, (seed % 6 != 0) as u32);
                c.bus.write32(e + 4, 0);
                c.bus.write32(e + 8, r.next_u32() % 8);
                c.bus
                    .write32(e + 0x184, if seed % 5 == 0 { f(1.5) } else { 0 });
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
                    c.bus
                        .write32(globals::EMITTER_BLEND + 4 * i, r.next_u32() % 32);
                }
                // The cull call is the second fake; forcing 1 means "culled".
                for k in 0..8u32 {
                    c.bus
                        .write32(LOG_RESULTS + 4 * k, if seed % 2 == 0 { 0 } else { 1 });
                }
                c.g[A0][0] = sx(e);
            },
        );
    }

    #[test]
    fn emitter_update_matches() {
        check(
            ssx3_ee::s12::f_00370788__generated,
            emitter_update,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 79, 0);
                let e = DATA;
                c.g[A0][0] = sx(e);
                c.f[12] = if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                // Remaining life: 0 (idle), negative (restart) or positive.
                let remain = match seed % 3 {
                    0 => 0,
                    1 => 0xbf80_0000 | (r.next_u32() & 0x003f_ffff),
                    _ => r.f32(120.0),
                };
                c.bus.write32(e, remain);
                // The descriptor at e+0x10: all zero, so both callees take their
                // short paths (progress 0, limit 0, rate 0).
                for off in [
                    0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
                    0x40, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c,
                ] {
                    c.bus.write32(e + off, 0);
                }
                // Progress, its limit and the rate `dt` scales.
                let limit = (r.next_u32() % 6) as i32;
                c.bus.write32(e + 0x180, limit as u32);
                c.bus
                    .write32(e + 0x184, if seed % 5 == 0 { 0 } else { r.edge_f32() });
                c.bus
                    .write32(e + 0x188, if seed % 4 == 0 { 0 } else { r.edge_f32() });
                c.bus.write32(e + 0xc, r.next_u32());
                // The frame clock object.
                c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
                c.bus.write32(
                    DATA + 0x310,
                    if seed % 7 == 0 {
                        1
                    } else {
                        30 + r.next_u32() % 60
                    },
                );
            },
        );
    }

    #[test]
    fn emitter_reset_desc_accel_matches() {
        check(
            ee::s12::f_0036d008__generated,
            emitter_reset_desc_accel,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0036_d008, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                put_vec(c, 0x004f_f120, [0, 0, 0, 0x3f80_0000]);
                c.bus.write32(DATA + 0xe0, r.next_u32());
            },
        );
    }

    #[test]
    fn emitter_set_desc_curve_b_matches() {
        crate::difftest::check(
            ee::s12::f_0036ce28__generated,
            emitter_set_desc_curve_b,
            200,
            &|c, seed| {
                let mut r = Rng(0x36ce_2801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.bus.write32(DATA + 0x4c, r.edge_f32());
            },
        );
    }

    // -------------------------------------------------------- emitter / UI text

    #[test]
    fn emitter_set_desc_curve_a_matches() {
        check(
            ee::s12::f_0036ce00__generated,
            emitter_set_desc_curve_a,
            200,
            &|c, seed| {
                let mut r = Rng(0x36ce_0000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[A0][0] = SCRATCH as u64;
                for f in 12..15 {
                    c.f[f] = r.edge_f32();
                }
            },
        );
    }

    #[test]
    fn graphics_man_snapshot_texture_table_matches() {
        covered(
            s12::f_00369ff8__generated,
            graphics_man_snapshot_texture_table,
            120,
            &|c, seed| {
                machine(c, seed, 0x369ff8, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x40);
                slot(c, VT, 0x3c8);
                c.bus.write32(OBJ + 0x10d8, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn graphics_man_free_all_textures_matches() {
        covered(
            s12::f_0036a020__generated,
            graphics_man_free_all_textures,
            120,
            &|c, seed| {
                machine(c, seed, 0x36a020, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x40);
                slot(c, VT, 0x3c0);
                c.bus.write32(OBJ + 0x10d8, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn emitter_dtor_matches() {
        check(
            ee::s12::f_00370758__generated,
            emitter_dtor,
            SEEDS_3,
            &|c, seed| destructor_setup(c, seed, 0x3707_58),
        );
    }

    // --------------------------------------------------------- graphics

    #[test]
    fn emitter_render_at_matches() {
        check(
            ee::s12::f_00370888__generated,
            emitter_render_at,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3708_88, 0);
                let desc = DATA + 0x2000;
                c.g[A0][0] = sx(desc);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x440);
                crate::difftest::put_vec(
                    c,
                    DATA + 0x400,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                crate::difftest::put_vec(
                    c,
                    DATA + 0x410,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                crate::difftest::put_vec(
                    c,
                    DATA + 0x440,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                fill(c, &mut r, desc, 0x200);
                // Nothing pending on the descriptor: Emitter_Render returns early.
                c.bus.write32(desc + 0xc, 0);
            },
        );
    }

    #[test]
    fn dynamic_emitter_dtor_matches() {
        check(
            ssx3_ee::s12::f_00370c08__generated,
            dynamic_emitter_dtor,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x370c08, 0);
                heap_init(c);
                let this = alloc(c, 0x400);
                let a = if seed % 3 == 0 { alloc(c, 0x40) } else { 0 };
                let b = if seed % 5 == 0 { alloc(c, 0x40) } else { 0 };
                c.bus.write32(this + 0x1a0, a);
                c.bus.write32(this + 0x1a4, b);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn emitter_load_matches() {
        check(
            ssx3_ee::s12::f_00370af8__generated,
            emitter_load,
            80,
            &|c, seed| {
                let mut r = setup(c, seed, 0x370af8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                // stream->v02: vtable slot 0x10/0x14 (a fake method that just logs).
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(DATA + 0x800, vt);
            },
        );
    }

    /// A pointer to the first descriptor byte is `a0`, the box being widened
    /// is `a1`; every float the pass reads (scalars at `+0x14`/`+0x1c`, the
    /// vectors at `+0x60`..`+0xef`, the box, the `gp-0x2868` reference
    /// vectors) is random, and the seeds reach both sides of the clamps.
    #[test]
    fn emitter_compute_desc_bounds_matches() {
        check(
            ssx3_ee::s12::f_0036d500__generated,
            emitter_compute_desc_bounds,
            300,
            &|c, seed| {
                let mut r = Rng(0x0036_d500 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                for i in 0..0x40u32 {
                    let v = if i % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                    c.bus.write32(DATA + 4 * i, v);
                }
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(100.0));
                }
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_wait_for_render_idle_matches() {
        check(
            ee::s12::f_00369c28__generated,
            ps2_graphics_man_wait_for_render_idle,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x369c28, 4);
                c.pc = 0x0036_9c28;
                let o = fe_render_fixture(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Busy: the fake answers nonzero and the poll returns at_3 once.
                // Idle: a real zero returner sends the body through the flush
                // block, whose first virtual call unwinds the thread so the
                // poll loop stops after one iteration.
                if seed % 2 == 0 {
                    c.bus.write32(o + 0x94, 0x002d_1bd8);
                    c.bus.write32(GAME_GP + 0x16c8, 0);
                    c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, 0);
                } else {
                    c.bus.write32(o + 0x94, FAKE);
                    c.bus.write32(LOG_RESULTS, 0x11);
                }
                c.bus.write32(GAME_GP + 0x2a74, 0);
            },
        );
    }

    #[test]
    fn emitter_set_desc_path_coeffs_matches() {
        check(
            ee::s12::f_0036cef8__generated,
            emitter_set_desc_path_coeffs,
            100,
            &|c, seed| {
                let mut r = base(c, seed, 0x36ce);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x300);
                for k in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x200 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0x300 + 4 * k, r.next_u32());
                }
            },
        );
    }

    // --------------------------------------------------------- memory card

    #[test]
    fn ps2_graphics_man_v94_matches() {
        strict(
            ee::s12::f_0036a2c0__generated,
            ps2_graphics_man_v94,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x36a2, 0);
                let mtx = DATA + 0x800;
                let obj = DATA + 0xa00;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = mtx as u64;
                w(c, DATA + 0xa4, if seed % 4 == 0 { 0 } else { obj });
                for off in (0..0x40u32).step_by(4) {
                    w(c, mtx + off, r.edge_f32());
                    w(c, obj + off, r.edge_f32());
                }
            },
        );
    }

    /// Compare every observable: hard registers, caller-saved temporaries,
    /// LO/HI, cycles, pc and memory.
    pub(crate) fn same(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    #[test]
    fn emitter_set_desc_path_coeffs_scaled_matches() {
        same(
            ssx3_ee::s12::f_0036d1f0__generated,
            emitter_set_desc_path_coeffs_scaled,
            200,
            &|c, seed| {
                let mut r = Rng(0x36d1f0 ^ seed as u64);
                scramble(c, &mut r);
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                c.g[A3][0] = (DATA + 0x280) as u64;
                c.g[T0][0] = (DATA + 0x2c0) as u64;
                let scale = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(self_at + 0x4c, scale);
                for base in [0x200u32, 0x240, 0x280, 0x2c0, 0x80, 0x90, 0xa0] {
                    put_vec(
                        c,
                        DATA + base,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_emit_depth_fog_clut_upload_matches() {
        same(
            ssx3_ee::s12::f_0036aa60__generated,
            ps2_graphics_man_emit_depth_fog_clut_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x36aa60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (r.next_u32() % 4096) as u64;
                c.g[A2][0] = (r.next_u32() % 4096) as u64;
                c.g[A3][0] = (DATA + 0x400) as u64;
                let dst = DATA + 0x1000;
                c.bus.write32(DATA + 0x400, dst);
                for i in 0..0x30 {
                    c.bus.write8(dst + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_match_blended_matrix_matches() {
        same(
            ssx3_ee::s12::f_00369a78__generated,
            ps2_graphics_man_match_blended_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x369a78 ^ seed as u64);
                scramble(c, &mut r);
                let a = DATA + 0x100;
                let b = DATA + 0x140;
                let ka = DATA + 0x200;
                let kb = DATA + 0x240;
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                let ty = (seed % 5) as u32;
                c.bus.write32(a, ty);
                c.bus
                    .write32(b, if seed % 4 == 0 { (ty + 1) % 5 } else { ty });
                c.bus.write32(a + 4, ka);
                c.bus.write32(b + 4, kb);
                // Key: id u16 at +2, x/y/z s16 at +0/+4/+8, idA/idB u16 at +6/+0xa.
                for k in [ka, kb] {
                    c.bus.write16(k, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 2, 1 + (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 4, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 6, (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 8, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 0xa, (r.next_u32() % 3) as u16);
                }
                if seed % 2 == 0 {
                    for off in [0u32, 2, 4, 8] {
                        let v = c.bus.read16(ka + off);
                        c.bus.write16(kb + off, v);
                    }
                }
            },
        );
    }

    /// Emitter_UpdateDescriptor: no emission due, one, or up to five
    /// (count * interval against the advanced time; positive intervals).
    #[test]
    fn emitter_update_descriptor_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        crate::difftest::check(
            ee::s12::f_0036d428__generated,
            emitter_update_descriptor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36d4_28, 0);
                c.pc = 0x0036_d428;
                let f = |x: f32| x.to_bits();
                let d = DATA;
                for k in 0..0x40 {
                    c.bus.write32(d + 4 * k, r.next_u32());
                }
                let count = (r.next_u32() % 6) as i32 - 1;
                let interval = 0.25 + (r.next_u32() % 8) as f32 * 0.25;
                c.bus.write32(d + 0x0, count as u32);
                c.bus.write32(d + 0xc, f(interval));
                let behind = (seed % 7) as f32 - 1.5; // emissions due, roughly
                c.bus
                    .write32(d + 0x8, f(count as f32 * interval + behind * interval));
                c.bus
                    .write32(d + 0x4c, f(1.0 + (r.next_u32() % 4) as f32 * 0.5));
                for k in 0..8 {
                    c.bus.write32(d + 0xb0 + 4 * k, r.f32(100.0));
                    c.bus.write32(d + 0xe0 + 4 * k, r.f32(10.0));
                }
                c.g[A0][0] = sx(d);
                c.f[12] = f((r.next_u32() % 4) as f32 * 0.0625);
            },
        );
    }

    /// DeferredDelete_AdvanceFrame: bucket indices -6..5 (both remainder
    /// forms), buckets with 0-3 queued objects whose destructors are fakes.
    #[test]
    fn deferred_delete_advance_frame_matches() {
        use crate::difftest::fakes::vtable;
        const NODES: u32 = DATA + 0x20_0000;
        check(
            ee::s12::f_0036a1e8__generated,
            deferred_delete_advance_frame,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36_a1e8, 4);
                c.pc = 0x0036_a1e8;
                let gp = crate::difftest::GAME_GP;
                let index = (seed % 12) as i32 - 6;
                c.bus.write32(gp + DD_INDEX, index as u32);
                let next = (index + 1) % 4;
                let bucket = DD_BUCKETS_HI - DD_BUCKETS_LO;
                for k in 0..4 {
                    c.bus.write32(bucket + 4 * k, r.next_u32());
                }
                let n = (seed / 12) % 4;
                let mut head = 0;
                for i in (0..n).rev() {
                    let node = NODES + 0x40 * i;
                    c.bus.write32(node, head);
                    let vt = vtable(c, &mut r, &[0x8]);
                    c.bus.write32(node + 4, vt);
                    head = node;
                }
                c.bus.write32(bucket.wrapping_add((next * 4) as u32), head);
            },
        );
    }

    /// Random defaults in `.sdata`, set index 0..7 on a random object.
    #[test]
    fn ps2_graphics_man_reset_bloom_params_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, GAME_GP};
        check_lohi(
            ee::s12::f_0036c740__generated,
            ps2_graphics_man_reset_bloom_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x36_c740 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                for off in BLOOM_DEFAULTS_GP {
                    c.bus.write32(GAME_GP + off, r.edge_f32());
                }
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = (seed % 8) as u64;
                c.pc = 0x0036_c740;
            },
        );
    }

    /// Emitter_Save: random description fields; the stream's write is a
    /// fake (it logs the record pointer and size; some calls unwind).
    #[test]
    fn emitter_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const DESC: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        covered(
            ee::s12::f_00370aa8__generated,
            emitter_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_0aa8, 1);
                c.pc = 0x0037_0aa8;
                let vt = vtable(c, &mut r, &[EMS_STREAM_WRITE]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(DESC + EMS_RATE, r.edge_f32());
                c.bus.write32(DESC + EMS_LIFE, r.edge_f32());
                c.bus.write32(DESC + EMS_COUNT, r.next_u32());
                c.g[A0][0] = sx(DESC);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// Emitter_BuildDescriptor with real callees: random emitter (`a0`,
    /// rate at `+0x4c`), matrix (`a1`), parameter block (`a2`) and bounds
    /// (`a3`); the time `f12` is negative on some seeds (random seeds through
    /// `Rand_NextMain`, whose state at `0x4ff018` is random too) and `p+8`
    /// is negative on some seeds (the time branch).
    #[test]
    fn emitter_build_descriptor_matches() {
        const SELF: u32 = DATA;
        const PARAMS: u32 = DATA + 0x200;
        const MATRIX: u32 = DATA + 0x400;
        const BOUNDS: u32 = DATA + 0x500;
        const RAND_STATE: u32 = 0x0050_0000 - 0xfe8;
        check(
            ee::s12::f_00370058__generated,
            emitter_build_descriptor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0037_0058 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0037_0058;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(MATRIX);
                c.g[A2][0] = sx(PARAMS);
                c.g[A3][0] = sx(BOUNDS);
                for i in 0..0x50u32 {
                    let v = if i % 7 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                    c.bus.write32(SELF + 4 * i, v);
                }
                for i in 0..0x34u32 {
                    c.bus.write32(PARAMS + 4 * i, r.f32(50.0));
                }
                for i in 0..0x10u32 {
                    c.bus.write32(MATRIX + 4 * i, r.f32(4.0));
                }
                for i in 0..8u32 {
                    c.bus.write32(BOUNDS + 4 * i, r.f32(100.0));
                }
                for i in 0..6u32 {
                    c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
                }
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.f32(10.0));
                }
                // Lifetime / rate non-zero on most seeds.
                if seed % 5 != 0 {
                    c.bus.write32(PARAMS + 0xc, 1.5f32.to_bits());
                    c.bus.write32(SELF + 0x4c, 0.25f32.to_bits());
                }
                let time = r.f32(10.0) & 0x7fff_ffff;
                c.f[12] = if seed % 2 == 0 {
                    time | 0x8000_0000
                } else {
                    time
                };
                let sign = c.bus.read32(PARAMS + 8) & 0x7fff_ffff;
                let sign = if seed % 3 == 0 {
                    sign | 0x8000_0000
                } else {
                    sign
                };
                c.bus.write32(PARAMS + 8, sign);
            },
        );
    }

    /// Emitter_RandomizeDescSeeds: the real Rand_NextMain on a random
    /// generator state; random f20 (saved and restored).
    #[test]
    fn emitter_randomize_desc_seeds_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA};
        check(
            ee::s12::f_0036ccb8__generated,
            emitter_randomize_desc_seeds,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_ccb8 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.pc = 0x0036_ccb8;
                // Rand_NextMain's generator (0x500000 - 0xfe8).
                fill(c, &mut r, 0x004f_f018, 0x10);
                c.f[20] = r.edge_f32();
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    pub(crate) use crate::game::object::effectlink::tests::emitter_def_setup;

    /// `Emitter_Setup` with the real descriptor init/build on
    /// [`emitter_def_setup`] and a random matrix.
    #[test]
    fn emitter_setup_matches() {
        use crate::difftest::scramble;
        const EMITTER: u32 = DATA;
        const DEF: u32 = DATA + 0x400;
        const MATRIX: u32 = DATA + 0x600;
        check(
            ee::s12::f_003705e0__generated,
            emitter_setup,
            200,
            &|c, seed| {
                let mut r = Rng(0x0037_05e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0037_05e0;
                c.g[GP][0] = sx(GAME_GP);
                emitter_def_setup(c, &mut r, seed, EMITTER, DEF);
                for i in 0..0x10 {
                    c.bus.write32(MATRIX + 4 * i, r.f32(4.0));
                }
                c.g[A0][0] = sx(EMITTER);
                c.g[A1][0] = sx(MATRIX);
                c.g[A2][0] = sx(DEF);
            },
        );
    }

    /// Emitter_SetDescVelocityCurve: random vectors (sometimes edge
    /// floats), lifetimes and t, including zero (VU0 division by zero).
    #[test]
    fn emitter_set_desc_velocity_curve_matches() {
        check(
            ee::s12::f_0036d318__generated,
            emitter_set_desc_velocity_curve,
            300,
            &|c, seed| {
                let mut r = Rng(0x36d3_1800 ^ seed as u64);
                scramble(c, &mut r);
                let desc = DATA;
                let vecs = DATA + 0x1000;
                fill(c, &mut r, desc, 0x130 / 4);
                for i in 0..16 {
                    let v = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                    c.bus.write32(vecs + 4 * i, v);
                }
                c.bus.write32(
                    desc + EMITTER_LIFETIME,
                    match seed % 7 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(10.0),
                    },
                );
                c.f[12] = if seed % 11 == 0 { 0 } else { r.f32(4.0) };
                c.g[A0][0] = sx(desc);
                c.g[A1][0] = sx(vecs);
                c.g[A2][0] = sx(vecs + 0x10);
                c.g[A3][0] = sx(vecs + 0x20);
                c.g[T0][0] = sx(vecs + 0x30);
                c.pc = 0x0036_d318;
            },
        );
    }

    /// Emitter_InitDescriptor: random grid sizes (zero and negative
    /// included), edge-case floats for x, y and the scale, a random velocity
    /// and default GS state.
    #[test]
    fn emitter_init_descriptor_matches() {
        check(
            ee::s12::f_0036cbf8__generated,
            emitter_init_descriptor,
            300,
            &|c, seed| {
                let mut r = Rng(0x36_cbf8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0036_cbf8;
                let desc = DATA;
                let velocity = DATA + 0x400;
                fill(c, &mut r, desc, 0x60);
                fill(c, &mut r, EMD_DEFAULT_GS_STATE, 4);
                for i in 0..4 {
                    let v = if seed % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                    c.bus.write32(velocity + 4 * i, v);
                }
                let dim = |r: &mut Rng| match seed % 6 {
                    0 => 0,
                    1 => r.next_u32(),
                    _ => r.next_u32() % 64 + 1,
                };
                c.g[A1][0] = (c.g[A1][0] & !0xffff_ffff) | dim(&mut r) as u64;
                c.g[A2][0] = (c.g[A2][0] & !0xffff_ffff) | dim(&mut r) as u64;
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(10.0)
                };
                c.f[13] = if seed % 3 == 1 {
                    r.edge_f32()
                } else {
                    r.f32(10.0)
                };
                c.f[14] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                c.g[A0][0] = sx(desc);
                c.g[A3][0] = sx(velocity);
            },
        );
    }

    #[test]
    fn morton_spread8_matches() {
        check(ssx3_ee::s12::f_0036a3b0, morton_spread8, 200, &|c, seed| {
            let mut r = Rng(0x36a3_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
        });
    }

    /// `DynamicEmitter_ctor`: the view is constructed over fresh memory; the
    /// real emitter base constructor and `memset` run.
    #[test]
    fn dynamic_emitter_ctor_matches() {
        check(
            ee::s12::f_00370b60__generated,
            dynamic_emitter_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x370b_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x84);
            },
        );
    }

    /// `Emitter_ctor`: the emitter is constructed over fresh memory; the real
    /// base constructor runs.
    #[test]
    fn emitter_ctor_matches() {
        check(
            ee::s12::f_00370018__generated,
            emitter_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3700_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x80);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    pub(crate) use crate::game::object::movenode::tests::oc338_fake;
    pub(crate) use crate::game::object::movenode::tests::OC338_CTRL;
    pub(crate) use crate::game::object::movenode::tests::OC338_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc338_fake!(
        oc338_fake_deferred_push,
        OC338_PREV_DEFERRED_PUSH,
        0x0036_a1c0,
        ee::s12::f_0036a1c0__generated
    );

    /// `GraphicsMan_DestroyGridMesh`: the mesh goes to a logging fake of
    /// `DeferredDelete_Push`.
    #[test]
    fn graphics_man_destroy_grid_mesh_matches() {
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0036A1C0),
            oc338_fake_deferred_push,
            &OC338_PREV_DEFERRED_PUSH,
        );
        covered(
            ee::s12::f_0036a190__generated,
            graphics_man_destroy_grid_mesh,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_a190, 1);
                c.pc = 0x0036_a190;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.g[A0][0] = sx(DATA + 0x5_0000);
                c.g[A1][0] = sx(DATA + 0x5_1000 + (r.next_u32() & 0xff0));
            },
        );
    }

    /// PS2GraphicsMan_AllocBlendedMatrixHistory: a fresh history (the real
    /// allocator), or 0..4 entries of 0..3 weights of which one may equal
    /// the matrix (the real MatchBlendedMatrix): found (bone copy) or
    /// appended.
    #[test]
    fn ps2_graphics_man_alloc_blended_matrix_history_matches() {
        check(
            ee::s12::f_003698e0__generated,
            ps2_graphics_man_alloc_blended_matrix_history,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3698e0, 0);
                crate::game::wscript::wscriptman::tests::alloc_ready(c);
                c.pc = 0x0036_98e0;
                let this = DATA + 0x100;
                let m = DATA + 0x1800;
                let history = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(m);
                let fresh = seed % 6 == 0;
                c.bus
                    .write32(this + PS2GM_BLEND_HISTORY, if fresh { 0 } else { history });
                let n = (seed / 6) % 5;
                c.bus
                    .write32(this + PS2GM_BLEND_COUNT, if fresh { 0 } else { n });
                // The matrix: 0..3 weights of small values.
                let count = (seed / 2) % 4;
                c.bus.write32(m, count);
                c.bus.write32(m + 4, m + 0x40);
                for k in 0..4u32 {
                    c.bus.write32(m + 0x40 + 4 * k, r.next_u32() & 0x000f_000f);
                }
                // The entries; one of them may equal the matrix.
                let same = if seed % 3 == 0 { n } else { r.next_u32() % 4 };
                for i in 0..4u32 {
                    let e = history + 0xc * i;
                    let w = DATA + 0x3000 + 0x40 * i;
                    c.bus.write32(e + 4, w);
                    if i == same {
                        c.bus.write32(e, count);
                        for k in 0..4u32 {
                            let v = c.bus.read32(m + 0x40 + 4 * k);
                            c.bus.write32(w + 4 * k, v);
                        }
                    } else {
                        c.bus.write32(e, r.next_u32() % 4);
                        for k in 0..4u32 {
                            c.bus.write32(w + 4 * k, r.next_u32() & 0x000f_000f);
                        }
                    }
                }
                // The append target past the in-use entries.
                for i in 4..6u32 {
                    c.bus
                        .write32(history + 0xc * i + 4, DATA + 0x3000 + 0x40 * i);
                }
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    /// `DeferredDelete_FlushAll`: empty buckets, so the real
    /// `DeferredDelete_AdvanceFrame` just advances the index four times.
    #[test]
    fn deferred_delete_flush_all_matches() {
        use crate::game::render::graphicsman::{DD_BUCKETS_HI, DD_BUCKETS_LO, DD_INDEX};
        check(
            ee::s12::f_0036a290__generated,
            deferred_delete_flush_all,
            120,
            &|c, seed| {
                machine(c, seed, 0x36_a290, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + DD_INDEX, seed % 4);
                let buckets = DD_BUCKETS_HI - DD_BUCKETS_LO;
                for k in 0..4u32 {
                    c.bus.write32(buckets + 4 * k, 0);
                }
                c.pc = 0x0036_a290;
            },
        );
    }

    #[test]
    fn deferred_delete_flush_all_events() {
        use crate::game::render::graphicsman::{DD_BUCKETS_HI, DD_BUCKETS_LO, DD_INDEX};
        driven!(gen_flush, ee::s12::f_0036a290__generated);
        driven!(hand_flush, deferred_delete_flush_all);
        check_events(gen_flush, hand_flush, 60, &|c, seed| {
            machine(c, seed / 3, 0x36_a290, 0);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + DD_INDEX, seed % 4);
            let buckets = DD_BUCKETS_HI - DD_BUCKETS_LO;
            for k in 0..4u32 {
                c.bus.write32(buckets + 4 * k, 0);
            }
            c.pc = 0x0036_a290;
            schedule(
                c,
                &mut Rng(0x36_a290 ^ (seed as u64) << 8),
                seed,
                0x0036_a290,
            )
        });
    }

    use crate::difftest::check_lohi;

    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    use crate::difftest::SP as STACK_TOP;

    /// Gate word of this module's callee fakes (they only act when it holds
    /// `OC1058_MAGIC` in the machine's RAM).
    pub(crate) const OC1058_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC1058_MAGIC: u32 = 0x1058_f06c;

    /// The block `SprStream_BeginBlock` hands out in the fixture.
    pub(crate) const OC1058_BLOCK: u32 = DATA + 0x1_0000;

    pub(crate) static OC1058_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1058_PREV_END: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1058_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1058_CTRL) != OC1058_MAGIC {
            return fall_back(c, &OC1058_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        fake(c, 0x0038_f460, c.g[RA][0] as u32);
    }

    pub(crate) fn oc1058_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1058_CTRL) != OC1058_MAGIC {
            return fall_back(c, &OC1058_PREV_END, ee::s12::f_0038f598__generated);
        }
        fake(c, 0x0038_f598, c.g[RA][0] as u32);
    }

    pub(crate) fn oc1058_install() {
        hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1058_fake_begin,
            &OC1058_PREV_BEGIN,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0038F598),
            oc1058_fake_end,
            &OC1058_PREV_END,
        );
    }

    pub(crate) fn oc1058_idx(seed: u32) -> u32 {
        (seed / 3) % 4
    }

    /// The fixture with an empty cache. Seed bits: `seed % 9` the cache
    /// variant (see `oc1058_setup`), `seed / 9 % 6` the fog mode, `seed / 54 % 8`
    /// the depth regime (clamps, `cnt <= 0`, `n0 == 0`), `seed / 5 % 4` the w.
    pub(crate) fn oc1058_build(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1058_0000_dead ^ (seed as u64) << 7);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(OC1058_CTRL, OC1058_MAGIC);
        let unwind = if seed % 7 == 0 {
            0
        } else if seed % 11 == 0 {
            1
        } else {
            u32::MAX
        };
        c.bus.write32(LOG_UNWIND_AT, unwind);
        c.bus.write32(LOG_RESULTS, OC1058_BLOCK);

        let idx = oc1058_idx(seed);
        let man = DATA;
        c.g[A0][0] = man as u64;
        c.g[A1][0] = (DATA + 0x2_0000) as u64;
        c.g[A2][0] = idx as u64;

        let w0 = [1.0f32, 2.0, 0.5, 4.0][((seed / 5) % 4) as usize];
        let (n0, n1): (i32, i32) = match (seed / 54) % 8 {
            0 => (96, 16),
            1 => (-5, -9),
            2 => (300, 40),
            3 => (80, -4),
            4 => (48, 300),
            5 => (0, 0),
            6 => (
                r.next_u32() as i32 % 400 - 50,
                r.next_u32() as i32 % 400 - 50,
            ),
            _ => (10, 10),
        };
        let z = |n: i32| (((n * 256 + 100) as f32) * w0).to_bits();
        let mode = [0u32, 1, 2, 3, 0xffff_ffff, 1][((seed / 9) % 6) as usize];
        let density = [0.0f32, 0.5, 1.0, 3.5][(seed % 4) as usize];

        // The fog record at man + idx * 0x20 + 0x6c64.
        let fog = man + idx * 0x20 + 0x6c64;
        c.bus.write32(fog, z(n0));
        c.bus.write32(fog + 0x4, z(n1));
        c.bus.write32(fog + 0x8, mode);
        c.bus.write32(fog + 0xc, density.to_bits());
        for (i, off) in [0x14u32, 0x18, 0x1c].into_iter().enumerate() {
            let v = ((r.next_u32() % 256) as f32 / 255.0).min(1.0) * if i == 0 { 1.0 } else { 0.9 };
            c.bus.write32(fog + off, v.to_bits());
        }
        // The matrix at man + idx * 0x40 + 0x5930: z' = z, w' = w0.
        let mat = man + idx * 0x40 + 0x5930;
        let rows: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, w0],
        ];
        for (ri, row) in rows.iter().enumerate() {
            for (ci, v) in row.iter().enumerate() {
                c.bus
                    .write32(mat + 16 * ri as u32 + 4 * ci as u32, v.to_bits());
            }
        }
        // exp polynomial coefficients (modes 1 and 2) in the data segment.
        for i in 0..12u32 {
            let v = 0.01f32 + 0.07 * (i % 6) as f32;
            c.bus.write32(
                GAME_GP.wrapping_sub(0x289c).wrapping_add(4 * i),
                v.to_bits(),
            );
        }
        c.bus.write32(GAME_GP + 0x1384, DATA + 0x3_0000);
        // Empty cache.
        c.bus.write32(GAME_GP + 0x10b8 + 4 * idx, 0);
    }

    /// Cache variants by `seed % 9`: 0 empty; 1..=5 filled from a real run with
    /// one key mismatching (n0, n1, colour, mode, density); 6 all equal (hit);
    /// 7 and 8 valid but random.
    pub(crate) fn oc1058_setup(c: &mut Cpu, seed: u32) {
        oc1058_build(c, seed);
        let idx = oc1058_idx(seed);
        let words = [0x10b8u32, 0x2938, 0x2940, 0x2948, 0x2950, 0x2958];
        let variant = seed % 9;
        if (1..=6).contains(&variant) {
            let mut t = fresh();
            oc1058_build(&mut t, seed);
            t.bus.write32(LOG_UNWIND_AT, u32::MAX);
            run_generated(&mut t, ssx3_ee::s12::f_0036a428__generated);
            for (k, off) in words.into_iter().enumerate() {
                let a = GAME_GP + off + 4 * idx;
                let mut v = t.bus.read32(a);
                if k >= 1 && k as u32 == variant {
                    v ^= 0x100;
                }
                c.bus.write32(a, v);
            }
        } else if variant >= 7 {
            let mut r = Rng(0x7777_0000 ^ seed as u64);
            for off in words {
                c.bus.write32(GAME_GP + off + 4 * idx, r.next_u32());
            }
            c.bus.write32(GAME_GP + 0x10b8 + 4 * idx, 1);
        }
    }

    /// Every fog mode (linear, exponential, exponential squared and an
    /// unknown one), the depth clamps, `cnt <= 0`, `n0 == 0`, every cache key
    /// mismatch and the hit, and an unwind in either callee. The `v0 == 0`
    /// skip of the opaque tail needs `n0 > 255`, which the clamp rules out.
    #[test]
    fn build_depth_fog_clut_matches() {
        oc1058_install();
        check_reaching(
            ssx3_ee::s12::f_0036a428__generated,
            ps2_graphics_man_build_depth_fog_clut,
            440,
            &[0x0036_a6ec, 0x0036_aa34],
            &oc1058_setup,
        );
    }

    /// `PS2GraphicsMan_EmitBlurDownsample`: random sizes (including zero and
    /// negative widths, so no strips), `passes` 0..=5 (each exit after
    /// passes 1-4), random intensity, offsets and rectangle; a scratchpad
    /// stream (two slots whose buffers are in RAM) at gp+0x1384 with the
    /// real `SprStream_BeginBlock` / `EndBlockAligned` callees.
    #[test]
    fn ps2_graphics_man_emit_blur_downsample_matches() {
        check(
            ee::s12::f_0036b9d8__generated,
            ps2_graphics_man_emit_blur_downsample,
            250,
            &|c, seed| {
                let mut r = Rng(0x0036_b9d8 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = sx(STACK_TOP);
                let man = DATA + 0x1_0000;
                let stream = DATA + 0x7_0000;
                let handle = DATA + 0x7_8000;
                c.bus.write32(GAME_GP + 0x1384, stream);
                let slot = r.next_u32() % 2;
                c.bus.write32(stream + 4, 2);
                c.bus.write32(stream + 8, slot);
                c.bus.write32(stream + 0xc, slot);
                c.bus.write32(stream + 0x10, 0x101);
                c.bus.write32(stream + 0x14, DATA + 0x7_1000);
                c.bus.write32(stream + 0x18, DATA + 0x7_3000);
                fill(c, &mut r, DATA + 0x7_1000, 0x400);
                fill(c, &mut r, DATA + 0x7_3000, 0x400);
                fill(c, &mut r, man + 0x5a70, 0x20);
                c.bus.write32(handle, DATA + 0x7_4000);
                let width = match seed % 7 {
                    0 => 0,
                    1 => -40i32 as u32,
                    2 => 32,
                    3 => 256,
                    4 => 100,
                    _ => 1 + r.next_u32() % 256,
                };
                let passes = [1u32, 2, 3, 4, 5, 0, 4, 3][(seed / 7 % 8) as usize];
                let fb_w = [0u32, 64, 640, 100, -64i32 as u32][(seed % 5) as usize];
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(r.next_u32() & 0x3fff_ffff);
                c.g[A2][0] = sx(fb_w);
                c.g[A3][0] = r.next_u32() as u64 % 600;
                c.g[T0][0] = r.next_u32() as u64 % 400;
                c.g[T1][0] = r.next_u32() as u64 % 600;
                c.g[T2][0] = r.next_u32() as u64 % 400;
                c.g[T3][0] = sx(r.next_u32() & 0x3fff_ffff);
                c.f[12] = r.f32(255.0);
                let args = STACK_TOP;
                c.bus.write32(args, width); // width
                c.bus.write32(args + 0x8, 1 + r.next_u32() % 300); // height
                c.bus.write32(args + 0x10, r.next_u32() % 4); // blur offset
                c.bus.write32(args + 0x18, passes);
                c.bus.write32(args + 0x20, handle);
            },
        );
    }

    /// Fixture of the three graphics functions: the manager at `this`, its
    /// sub-object vtable (`this+0x10d8`: method deltas at +0x238/+0x50/+0x58
    /// = 0, entries at +0x23c/+0x54/+0x5c = fake methods).
    pub(crate) const OC1063_THIS: u32 = DATA + 0x400;

    pub(crate) const OC1063_VT: u32 = DATA + 0x600;

    pub(crate) fn oc1063_manager(c: &mut Cpu) {
        c.bus.write32(OC1063_THIS + 0x10d8, OC1063_VT);
        for off in [0x238, 0x50, 0x58] {
            c.bus.write16(OC1063_VT + off, 0);
        }
        for off in [0x23c, 0x54, 0x5c] {
            c.bus.write32(OC1063_VT + off, FAKE);
        }
    }

    /// GraphicsMan_DrawIndexedTriangles: 0-3 triangles (and a negative count)
    /// with and without UVs; `RenderTriStrip` is a fake that may unwind.
    #[test]
    fn graphics_man_draw_indexed_triangles_matches() {
        check_lohi(
            ee::s12::f_00369cf8__generated,
            graphics_man_draw_indexed_triangles,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x369cf8, 3);
                c.pc = 0x0036_9cf8;
                oc1063_manager(c);
                let (indices, uv, colours, positions, offset) = (
                    DATA + 0x800,
                    DATA + 0xa00,
                    DATA + 0xc00,
                    DATA + 0xe00,
                    DATA + 0xf80,
                );
                for i in 0..12 {
                    c.bus.write8(indices + i, (r.next_u32() % 8) as u8);
                }
                for i in 0..16 {
                    c.bus.write32(uv + 4 * i, r.edge_f32());
                }
                for i in 0..32 {
                    c.bus.write32(colours + 4 * i, r.edge_f32());
                }
                for i in 0..24 {
                    c.bus.write32(positions + 4 * i, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(offset + 4 * i, r.edge_f32());
                }
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    n => n as u64 - 1,
                };
                c.g[A3][0] = indices as u64;
                c.g[T1][0] = if seed % 2 == 0 { uv as u64 } else { 0 };
                c.g[T2][0] = colours as u64;
                c.g[T0][0] = positions as u64;
                c.g[T3][0] = offset as u64;
            },
        );
    }

    /// Force the results of the first fake calls to plausible widths.
    pub(crate) fn oc1063_widths(c: &mut Cpu, r: &mut Rng, n: u32) {
        for i in 0..n {
            c.bus
                .write32(LOG_RESULTS + 4 * i, 512 + 16 * (r.next_u32() % 32));
        }
    }

    /// PS2GraphicsMan_EmitDepthFogZCopy: frame sizes from 0 to 130 pixels
    /// (and a negative one) so the sprite loop runs 0 to 16 times; the width
    /// queries are fake method calls.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_z_copy_matches() {
        const PP: u32 = DATA + 0x500;
        const PACKET: u32 = DATA + 0x2_0000;
        covered(
            ee::s12::f_0036ae20__generated,
            ps2_graphics_man_emit_depth_fog_z_copy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36ae20, 4);
                c.pc = 0x0036_ae20;
                oc1063_manager(c);
                oc1063_widths(c, &mut r, 4);
                let width = if seed % 11 == 0 {
                    0xffff_fffd
                } else if seed % 7 == 0 {
                    0
                } else {
                    r.next_u32() % 131
                };
                c.bus.write32(OC1063_THIS + 0x5a30, width);
                c.bus.write32(OC1063_THIS + 0x5a34, 1 + r.next_u32() % 600);
                c.bus.write32(OC1063_THIS + 0x5a80, r.next_u32() & 0xffff);
                c.bus.write32(PP, PACKET + 16 * (r.next_u32() % 4));
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[T0][0] = (r.next_u32() % 64) as u64;
                c.g[T1][0] = PP as u64;
            },
        );
    }

    /// PS2GraphicsMan_EmitDepthFogBlend: region widths of 0 to 4 strips (and
    /// more), four regions to pick from; the width queries are fake calls.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_blend_matches() {
        const PP: u32 = DATA + 0x500;
        const PACKET: u32 = DATA + 0x2_0000;
        covered(
            ee::s12::f_0036b158__generated,
            ps2_graphics_man_emit_depth_fog_blend,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36b158, 7);
                c.pc = 0x0036_b158;
                oc1063_manager(c);
                oc1063_widths(c, &mut r, 8);
                for idx in 0..4u32 {
                    let rect = OC1063_THIS + 0x59b0 + 8 * idx;
                    c.bus.write16(rect, (r.next_u32() % 640) as u16);
                    c.bus.write16(rect + 2, (r.next_u32() % 480) as u16);
                    let w = match r.next_u32() % 5 {
                        0 => 0,
                        1 => 31,
                        _ => r.next_u32() % 200,
                    };
                    c.bus.write16(rect + 4, w as u16);
                    c.bus.write16(rect + 6, (r.next_u32() % 480) as u16);
                }
                for off in [0x5900, 0x5902, 0x5904, 0x5906] {
                    c.bus
                        .write16(OC1063_THIS + off, (r.next_u32() % 700) as u16);
                }
                c.bus.write32(OC1063_THIS + 0x5a80, r.next_u32() & 0xffff);
                c.bus.write32(OC1063_THIS + 0x5a84, r.next_u32() & 0xffff);
                c.bus.write32(PP, PACKET + 16 * (r.next_u32() % 4));
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = PP as u64;
            },
        );
    }

    /// The scratchpad stream (gp+0x1384): two slots, buffers in RAM.
    pub(crate) const OC1064_STREAM: u32 = DATA + 0x7_0000;

    /// Point gp+0x1384 at a two-slot stream whose blocks live in RAM (as the
    /// `DeferMan_SortBuckets` fixture does); the real stream callees run.
    pub(crate) fn oc1064_stream(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP + 0x1384, OC1064_STREAM);
        let slot = r.next_u32() % 2;
        c.bus.write32(OC1064_STREAM + 4, 2);
        c.bus.write32(OC1064_STREAM + 8, slot);
        c.bus.write32(OC1064_STREAM + 0xc, slot);
        c.bus.write32(OC1064_STREAM + 0x10, 0x101);
        c.bus.write32(OC1064_STREAM + 0x14, DATA + 0x7_1000);
        c.bus.write32(OC1064_STREAM + 0x18, DATA + 0x7_3000);
        fill(c, r, DATA + 0x7_1000, 0x400);
        fill(c, r, DATA + 0x7_3000, 0x400);
    }

    /// A rectangle width in pixels: none, less than a column, or several.
    pub(crate) fn oc1064_width(r: &mut Rng, seed: u32) -> u32 {
        match seed % 6 {
            0 => 0,
            1 => 1 + r.next_u32() % 31,
            _ => r.next_u32() % 700,
        }
    }

    /// `PS2GraphicsMan_EmitDepthFogFlatFill`: a manager with one of four saved
    /// rectangles (no columns, part of one, several), the fog colour floats,
    /// the two virtual size queries as fakes (forced results; some calls
    /// unwind) and the packet cursor `*pp`.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_flat_fill_matches() {
        const GM: u32 = DATA;
        const VT: u32 = DATA + 0x7000;
        const PP: u32 = DATA + 0x7800;
        const BUF: u32 = DATA + 0x8000;
        covered(
            ee::s12::f_0036b5e8__generated,
            ps2_graphics_man_emit_depth_fog_flat_fill,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_b5e8, 6);
                c.pc = 0x0036_b5e8;
                let idx = r.next_u32() % 4;
                c.bus.write32(GM + 0x10d8, VT);
                for slot in [0x50, 0x58] {
                    c.bus.write16(VT + slot, (r.next_u32() as u16) & 0xfc);
                }
                c.bus.write32(VT + 0x54, FAKE);
                c.bus.write32(VT + 0x5c, FAKE + 0x10);
                let rect = GM + idx * 8;
                c.bus.write16(rect + 0x59b0, r.next_u32() as u16 % 640);
                c.bus.write16(rect + 0x59b2, r.next_u32() as u16 % 640);
                let w = oc1064_width(&mut r, seed);
                c.bus.write16(rect + 0x59b4, w as u16);
                c.bus.write16(rect + 0x59b6, r.next_u32() as u16 % 480);
                c.bus.write32(GM + 0x5a80, r.next_u32());
                c.bus.write32(GM + 0x5a84, r.next_u32());
                let fog = GM + idx * 0x20 + 0x6c64;
                for off in [0x14, 0x18, 0x1c] {
                    let v = if seed % 5 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(fog + off, v);
                }
                for k in 0..6 {
                    c.bus.write32(LOG_RESULTS + 4 * k, r.next_u32() % 0x1400);
                }
                c.bus.write32(PP, BUF);
                fill(c, &mut r, BUF, 0x20);
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(PP);
            },
        );
    }

    /// `PS2GraphicsMan_EmitBlendedFill`: a stream block with random strip
    /// width (none, partial, several columns; a few negative), position,
    /// alpha and grey; the real stream callees run.
    #[test]
    fn ps2_graphics_man_emit_blended_fill_matches() {
        const HANDLE: u32 = DATA + 0x6800;
        check(
            ee::s12::f_0036c188__generated,
            ps2_graphics_man_emit_blended_fill,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_c188 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                oc1064_stream(c, &mut r);
                c.bus
                    .write32(HANDLE, DATA + 0x7_8000 + (r.next_u32() & 0xff0));
                let w = if seed % 11 == 7 {
                    (r.next_u32() % 100).wrapping_neg()
                } else {
                    oc1064_width(&mut r, seed)
                };
                c.g[A1][0] = (r.next_u32() % 640) as u64;
                c.g[A2][0] = sx(w);
                c.g[A3][0] = (r.next_u32() % 480) as u64;
                c.g[T0][0] = (r.next_u32() % 480) as u64;
                c.g[T1][0] = (r.next_u32() & 0xff) as u64;
                c.g[T2][0] = sx(HANDLE);
            },
        );
    }

    /// `PS2GraphicsMan_EmitBloomComposite`: the width query is the real leaf at
    /// 0x1406c0 (returns 0: the frame word without a width) or a fake with a
    /// forced result; arguments in registers and on the caller's stack.
    #[test]
    fn ps2_graphics_man_emit_bloom_composite_matches() {
        const GM: u32 = DATA;
        const VT: u32 = DATA + 0x6000;
        const HANDLE: u32 = DATA + 0x6800;
        const STACK: u32 = DATA + 0xa_0000;
        covered(
            ee::s12::f_0036c398__generated,
            ps2_graphics_man_emit_bloom_composite,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0036_c398, 2);
                c.pc = 0x0036_c398;
                oc1064_stream(c, &mut r);
                c.g[SP][0] = sx(STACK);
                c.bus.write32(GM + 0x10d8, VT);
                c.bus.write16(VT + 0x50, (r.next_u32() as u16) & 0xfc);
                c.bus
                    .write32(VT + 0x54, if seed % 3 == 0 { 0x0014_06c0 } else { FAKE });
                c.bus.write32(GM + 0x5a80, r.next_u32());
                for k in 0..2 {
                    c.bus
                        .write32(LOG_RESULTS + 4 * k, 0x40 | (r.next_u32() % 0x1000));
                }
                c.bus
                    .write32(HANDLE, DATA + 0x7_8000 + (r.next_u32() & 0xff0));
                for off in [0, 8, 0x10] {
                    c.bus.write32(STACK + off, r.next_u32());
                }
                c.bus.write32(STACK + 0x18, HANDLE);
                let w = if seed % 11 == 7 {
                    (r.next_u32() % 100).wrapping_neg()
                } else {
                    oc1064_width(&mut r, seed)
                };
                c.g[A0][0] = sx(GM);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (r.next_u32() % 1024) as u64;
                c.g[A3][0] = (r.next_u32() % 480) as u64;
                c.g[T0][0] = (r.next_u32() % 640) as u64;
                c.g[T1][0] = (r.next_u32() % 480) as u64;
                c.g[T2][0] = (r.next_u32() % 480) as u64;
                c.g[T3][0] = sx(w);
            },
        );
    }

    #[test]
    fn graphics_man_empty_hook_matches() {
        check(
            ee::s12::f_00369ff0__generated,
            graphics_man_empty_hook,
            64,
            &|c, seed| {
                let mut r = Rng(0x0036_9ff0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn emitter_set_desc_linear_path_matches() {
        check(
            ee::s12::f_0036ce80__generated,
            emitter_set_desc_linear_path,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_ce80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2010);
                let a = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                let b = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, DATA + 0x2000, a);
                put_vec(c, DATA + 0x2010, b);
            },
        );
    }

    #[test]
    fn emitter_set_desc_accel_scaled_matches() {
        check(
            ee::s12::f_0036cfa8__generated,
            emitter_set_desc_accel_scaled,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_cfa8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, DATA + 0x2000, v);
                c.bus.write32(DATA + 0x1000 + 0xc, r.edge_f32());
                c.bus.write32(DATA + 0x1000 + 0x4c, r.edge_f32());
                c.f[12] = r.edge_f32();
            },
        );
    }
}
