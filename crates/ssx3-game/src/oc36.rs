//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for world and render helpers (Bezier patches, streaming
//! views, HUD panel, lights, emitters, spatial hash), each with a
//! differential test against the generated body.
//!
//! - **Lights**: `WorldLightMan_LightDistance` (distance/attenuation of a
//!   point light at a position), `Irradiance_AddDynamicLight` (spot/point
//!   light falloff into the irradiance accumulator).
//! - **Emitters**: `DynamicEmitter_Spawn` / `_SpawnAt` (one particle into
//!   the emitter's ring buffers, colour clamped to bytes).
//! - **Paths**: `Path_GetClosestPointFrom` (walk the path segments from a
//!   cursor, keep the closest point).
//! - **Spatial hash**: `SpatialKey_ClassifyAABB`, `SpatialHash_Update`.
//! - **Rendering**: `GameModule_DrawBezierPatch` (LOD from corner distances),
//!   `PS2GraphicsMan_v58` (bilinear texel fetch of a texture), the world
//!   streamer's view-node walk, `HUD_DrawPositionPanel` and
//!   `WorldPainter_UpdateView`.
//!
//! The replacements keep their state in the guest registers the originals
//! use where callers, callees or event handlers can see them, follow the
//! call protocol of [`crate::call`] and the event rules of
//! [`crate::events`]; VU0 code is issued as the original's macro ops.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_6168, world_light_man_light_distance, "WorldLightMan_LightDistance"),
    (0x0037_17c0, dynamic_emitter_spawn_at, "DynamicEmitter_SpawnAt"),
    (0x0037_10d0, dynamic_emitter_spawn, "DynamicEmitter_Spawn"),
    (0x0032_8360, spatial_key_classify_aabb, "SpatialKey_ClassifyAABB"),
    (0x0026_a638, path_get_closest_point_from, "Path_GetClosestPointFrom"),
    (0x0022_c410, game_module_draw_bezier_patch, "GameModule_DrawBezierPatch"),
    (0x0038_89f0, ps2_graphics_man_v58, "PS2GraphicsMan_v58"),
    (0x003a_a028, world_cache_streamer_update_view_node, "cWorldCache_StreamerUpdateViewNode"),
    (0x0021_ed48, hud_draw_position_panel, "HUD_DrawPositionPanel"),
    (0x0032_91e0, spatial_hash_update, "SpatialHash_Update"),
    (0x0038_a6a8, irradiance_add_dynamic_light, "Irradiance_AddDynamicLight"),
    (0x002e_d490, world_painter_update_view, "WorldPainter_UpdateView"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const DYNAMIC_EMITTER_SPAWN: Callee = Callee { addr: 0x0037_10d0, f: ee::s12::DynamicEmitter_Spawn };
    /// `rand()`-style float source the emitter calls (cross-shard, through
    /// the function table).
    pub const RANDOM: u32 = 0x0031_77f0;
    pub const POINT_BOX_DISTANCE: Callee = Callee { addr: 0x003a_9e50, f: ee::s13::cWorldCache_PointBoxDistance };
    pub const IS_HULL_RESIDENT: Callee = Callee { addr: 0x003a_8618, f: ee::s13::cWorldCache_IsHullResident };
    pub const REQUEST_HULL: Callee = Callee { addr: 0x003a_8448, f: ee::s13::cWorldCache_RequestHull };
    pub const STREAMER_UPDATE_VIEW_NODE: Callee =
        Callee { addr: 0x003a_a028, f: ee::s13::cWorldCache_StreamerUpdateViewNode };
    pub const HUD_DRAW_PANEL_SEGMENT: Callee = Callee { addr: 0x0021_ea00, f: ee::s05::HUD_DrawPanelSegment };
    /// HUD helpers (cross-shard, through the function table).
    pub const HUD_GET_ELEM_COLOR: u32 = 0x001e_91a8;
    pub const HUD_GET_ELEM_POS: u32 = 0x001e_91f8;
    pub const HUD_GET_ELEM_SIZE: u32 = 0x001e_9220;
    pub const HUD_ALIGN_ELEM_POS: u32 = 0x001f_10f8;
    pub const HUD_DRAW_SPRITE: u32 = 0x001f_1190;
    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;
    pub const SPATIAL_KEY_FROM_AABB: Callee = Callee { addr: 0x0032_8f28, f: ee::s10::SpatialKey_FromAABB };
    pub const SPATIAL_HASH_REMOVE: Callee = Callee { addr: 0x0032_84b8, f: ee::s10::SpatialHash_Remove };
    pub const SPATIAL_HASH_INSERT: Callee = Callee { addr: 0x0032_8c20, f: ee::s10::SpatialHash_Insert };
    pub const SPATIAL_HASH_MOVE: Callee = Callee { addr: 0x0032_8808, f: ee::s10::SpatialHash_Move };
    pub const DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::__builtin_delete };
    pub const SPOT_LIGHT_VECTOR: Callee = Callee { addr: 0x0038_a530, f: ee::s12::Irradiance_SpotLightVector };
    pub const POINT_LIGHT_VECTOR: Callee = Callee { addr: 0x0038_a618, f: ee::s12::Irradiance_PointLightVector };
    pub const ADD_DIRECTIONAL: Callee = Callee { addr: 0x0038_9520, f: ee::s12::Irradiance_AddDirectional };
    pub const SAMPLE_SURFACE_COLOURS: Callee =
        Callee { addr: 0x002e_db20, f: ee::s09::WorldPainter_SampleSurfaceColours };
    pub const LERP_VIEW_COLOURS: Callee = Callee { addr: 0x002e_d338, f: ee::s09::WorldPainter_LerpViewColours };
    pub const BLEND_VIEW_COLOURS: Callee = Callee { addr: 0x002e_d1d0, f: ee::s09::WorldPainter_BlendViewColours };
    pub const GET_IRRADIANCE0: Callee = Callee { addr: 0x002e_e318, f: ee::s09::WorldPainter_GetIrradiance0 };
    pub const GET_IRRADIANCE1: Callee = Callee { addr: 0x002e_e340, f: ee::s09::WorldPainter_GetIrradiance1 };
    pub const GET_IRRADIANCE2: Callee = Callee { addr: 0x002e_e368, f: ee::s09::WorldPainter_GetIrradiance2 };
    pub const COMPUTE_COLOUR_FACTOR: Callee =
        Callee { addr: 0x002e_df00, f: ee::s09::WorldPainter_ComputeColourFactor };
    pub const GET_LIGHTING28: Callee = Callee { addr: 0x002e_eff0, f: ee::s09::WorldPainter_GetLighting28 };
    /// Cross-shard (function table).
    pub const SECTION_GET_EVENT_WORLD_SECTION: u32 = 0x0022_e098;
    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
    pub const IRRADIANCE_SCALE: u32 = 0x0038_97a0;
    pub const IRRADIANCE_SCALE_IN_PLACE: u32 = 0x0038_9810;
    pub const IRRADIANCE_ADD: u32 = 0x0038_9730;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// FPU register `f`.
#[inline(always)]
fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// Set FPU register `f`.
#[inline(always)]
fn setf(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}

/// `lui at, hi(bits); mtc1 at, f`: a float constant through `at`.
#[inline(always)]
fn fconst(c: &mut Cpu, f: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = bits as u64;
    c.f[f] = bits;
    FpuF32(bits)
}

/// `c.lt.s a, b` leaving the condition bit.
#[inline(always)]
fn lt(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.lt(b);
    abi::set_fcc(c, r);
    r
}

/// `c.le.s a, b`.
#[inline(always)]
fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b`.
#[inline(always)]
fn eq(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    abi::set_fcc(c, r);
    r
}

/// `x + y` of two sign-extended words (`addu`).
#[inline(always)]
fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `sra x, n`.
#[inline(always)]
fn sra32(x: u64, n: u32) -> u64 {
    sx(((x as u32) as i32 >> n) as u32)
}

/// `srl x, n`.
#[inline(always)]
fn srl32(x: u64, n: u32) -> u64 {
    sx((x as u32) >> n)
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vadd vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vsub vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf4, v; qmtc2 rt, vf3; vmulx vf5, vf4, vf3x; sqc2 vf5, out`:
/// a vector scaled by the float bits the original left in GPR `rt`.
#[inline(always)]
fn vscale(c: &mut Cpu, v: u32, rt: usize, out: u32) {
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// The inlined 4-component dot product: `lqc2 vf3, a; lqc2 vf5, b;
/// vaddw.x vf6, vf0, vf0w (1); vmul vf4, vf3, vf5; vaddy.x/vmaddz.x/
/// vmaddw.x` (x + y + z + w into vf4.x); `qmfc2 rt, vf4`. Returns the dot
/// (x lane) bits as left in `rt`.
#[inline(always)]
fn vdot4(c: &mut Cpu, a: u32, b: u32, rt: usize) -> FpuF32 {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(rt, 4);
    FpuF32(c.g[rt][0] as u32)
}

/// The common epilogue tail: `addiu sp, sp, size; jr ra` with `ra` as it is.
#[inline(always)]
fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Lights.

/// Point light fields read by `WorldLightMan_LightDistance`.
mod light {
    pub const RADIUS: u32 = 0x1c;
    pub const DIR: u32 = 0x2c;
    pub const POS: u32 = 0x38;
}

/// `bool WorldLightMan_LightDistance(this, const Vec3* pos, const Light*
/// light, float* dist, float* inv_dist /*t0*/, float* facing /*t1*/)`
/// (0x2f6168): false when `pos` is outside the light's radius. Otherwise
/// stores the distance (1 at the light's centre) and its inverse, and, when
/// `facing` is given, `-dot(light.dir, light.pos - pos) / dist`.
pub fn world_light_man_light_distance(c: &mut Cpu) {
    c.cycles += 20;
    let pos = abi::arg(c, 1);
    let lt_at = abi::arg(c, 2);
    let lx = ldf(c, 2, p(lt_at, light::POS));
    let px = ldf(c, 1, pos);
    let ly = ldf(c, 3, p(lt_at, light::POS + 4));
    let py = ldf(c, 0, p(pos, 4));
    let dx = lx - px;
    setf(c, 7, dx);
    let lz = ldf(c, 2, p(lt_at, light::POS + 8));
    let dy = ly - py;
    setf(c, 6, dy);
    let pz = ldf(c, 4, p(pos, 8));
    let dx2 = dx * dx;
    let radius = ldf(c, 0, p(lt_at, light::RADIUS));
    let dz = lz - pz;
    setf(c, 5, dz);
    let dy2 = dy * dy;
    setf(c, 3, dy2);
    let r2 = radius * radius;
    setf(c, 0, r2);
    let dz2 = dz * dz;
    setf(c, 2, dz2);
    let d2 = dx2 + dy2 + dz2;
    setf(c, 1, d2);
    let outside = lt(c, r2, d2);
    c.g[V0][0] = 0;
    if outside {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    setf(c, 0, FpuF32::ZERO);
    let (dist, inv) = if eq(c, d2, FpuF32::ZERO) {
        c.cycles += 3;
        let one = fconst(c, 4, 0x3f80_0000);
        setf(c, 3, one);
        (one, one)
    } else {
        c.cycles += 6;
        let one = fconst(c, 0, 0x3f80_0000);
        let dist = d2.sqrt();
        setf(c, 3, dist);
        let inv = one.div(dist);
        setf(c, 4, inv);
        (dist, inv)
    };
    c.cycles += 2;
    if c.g[T1][0] != 0 {
        c.cycles += 12;
        let ux = ldf(c, 0, p(lt_at, light::DIR));
        let uy = ldf(c, 2, p(lt_at, light::DIR + 4));
        let fx = dx * ux;
        let uz = ldf(c, 1, p(lt_at, light::DIR + 8));
        let fy = dy * uy;
        setf(c, 2, fy);
        let fz = dz * uz;
        setf(c, 1, fz);
        let facing = -(fx + fy + fz) * inv;
        setf(c, 0, facing);
        let out = abi::arg(c, 5);
        c.swc1(0, out);
    }
    c.swc1(3, abi::arg(c, 3));
    let _ = dist;
    c.cycles += 3;
    c.g[V0][0] = 1;
    c.swc1(4, abi::arg(c, 4));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Dynamic emitters.

/// `DynamicEmitter` fields.
mod emitter {
    /// Particle lifetime step and accumulated time (`+0x10 += +0x14 * dt`,
    /// wrapped to 0 at `+0xc`).
    pub const PERIOD: u32 = 0x0c;
    pub const TIME: u32 = 0x10;
    pub const RATE: u32 = 0x14;
    pub const ENABLED: u32 = 0x174;
    pub const CAPACITY: u32 = 0x178;
    /// Ring cursor (counts down, wraps to `CAPACITY - 1`).
    pub const CURSOR: u32 = 0x17c;
    pub const POSITIONS: u32 = 0x1a0;
    pub const VELOCITIES: u32 = 0x1a4;
    pub const DEFAULT_POS: u32 = 0x1b0;
    pub const DEFAULT_VEL: u32 = 0x1c0;
    pub const DEFAULT_COLOUR: u32 = 0x1d0;
    pub const LIVE: u32 = 0x1e0;
    pub const COLOURS: u32 = 0x204;
}

/// `v0 = cursor * 4 + colours`: the colour slot of the current particle
/// (v0/v1 as the original leaves them).
#[inline(always)]
fn colour_slot(c: &mut Cpu, e: u32) -> u32 {
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::COLOURS));
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    r32(c, V0)
}

/// One colour channel to a byte: `f3 = 0 <= v*255 ? min(v*255, 255) : 0`
/// (f0 = v*255, f1 = 255, f2 = 0 as the original leaves them).
#[inline(always)]
fn clamp_channel(c: &mut Cpu, v: FpuF32) {
    let scaled = v * fr(c, 1);
    setf(c, 0, scaled);
    let zero = fr(c, 2);
    let pos = le(c, zero, scaled);
    setf(c, 3, zero);
    if pos {
        c.cycles += 1;
        let m = scaled.min(fr(c, 1));
        setf(c, 3, m);
    }
}

/// `void DynamicEmitter_SpawnAt(Emitter* e, const Vec4* pos, const Vec4*
/// vel, const Vec4* colour /*a3*/, int flag /*t0*/, float dt /*f12*/)`
/// (0x3717c0): store the colour (or the emitter's default) as RGBA bytes
/// in the current particle's slot, then spawn it (`DynamicEmitter_Spawn`).
pub fn dynamic_emitter_spawn_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = c.g[A0][0];
        let e = r32(c, T1);
        frame.save_ra(c, 0);
        if c.g[A3][0] != 0 {
            c.cycles += 3;
            copy_q(c, V0, r32(c, A3), p(e, emitter::DEFAULT_COLOUR));
        } else {
            c.cycles += 1;
            c.g[A3][0] = sx(p(e, emitter::DEFAULT_COLOUR));
        }
        let colour = r32(c, A3);
        c.cycles += 9;
        let alpha = ldf(c, 0, p(colour, 0xc));
        fconst(c, 1, 0x437f_0000);
        setf(c, 2, FpuF32::ZERO);
        clamp_channel(c, alpha);
        // (byte offset in the slot, component of the colour to clamp next)
        for (byte, next) in [(3u32, 0u32), (0, 4), (1, 8)] {
            c.cycles += 13;
            c.cvt_w_s(0, 3);
            c.mfc1(A0, 0);
            let slot = colour_slot(c, e);
            let v = c.g[A0][0];
            c.sb(p(slot, byte), v);
            let comp = ldf(c, 0, p(colour, next));
            clamp_channel(c, comp);
        }
        c.cycles += 13;
        c.cvt_w_s(0, 3);
        c.mfc1(A3, 0);
        c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
        c.g[V1][0] = c.lw(p(e, emitter::COLOURS));
        c.g[A0][0] = c.g[T1][0];
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
        let slot = r32(c, V0);
        let v = c.g[A3][0];
        c.sb(p(slot, 2), v);
        c.g[A3][0] = c.g[T0][0];
        callee::DYNAMIC_EMITTER_SPAWN.call(c, 0x0037_18d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicEmitter_Spawn(Emitter* e, const Vec4* pos, const Vec4* vel,
/// int jitter /*a3*/, float dt /*f12*/)` (0x3710d0): if enabled, write the
/// particle at the ring cursor (position, velocity with w 0, or a random
/// [1, 2) when `jitter`), keep the live count, move the cursor back and
/// advance the emitter's clock.
pub fn dynamic_emitter_spawn(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        let e = r32(c, S0);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(p(e, emitter::ENABLED));
        c.mov_s(20, 12);
        if c.g[V0][0] != 0 {
            spawn_particle(c, e)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`dynamic_emitter_spawn`] for an enabled emitter.
fn spawn_particle(c: &mut Cpu, e: u32) -> Flow {
    // Was the slot's previous particle alive (velocity w > 0)?
    c.cycles += 10;
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    setf(c, 1, FpuF32::ZERO);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let age = ldf(c, 0, at(c, V0, 0xc));
    let alive = lt(c, FpuF32::ZERO, age);
    c.g[A0][0] = 1;
    if !alive {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    // Live count: a reused live slot without jitter -1, a dead slot with
    // jitter +1.
    c.cycles += 2;
    let mut store = false;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        if c.g[A3][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(e, emitter::LIVE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            store = true;
        }
    } else {
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(p(e, emitter::LIVE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            store = true;
        }
    }
    if store {
        c.cycles += 1;
        let v = c.g[V0][0];
        c.sw(p(e, emitter::LIVE), v);
    }
    // Default position / velocity, or copy the given ones as the defaults.
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        c.g[A1][0] = sx(p(e, emitter::DEFAULT_POS));
    } else {
        c.cycles += 2;
        copy_q(c, V0, r32(c, A1), p(e, emitter::DEFAULT_POS));
    }
    c.cycles += 2;
    if c.g[A2][0] == 0 {
        c.g[A2][0] = sx(p(e, emitter::DEFAULT_VEL));
    } else {
        c.cycles += 2;
        copy_q(c, V0, r32(c, A2), p(e, emitter::DEFAULT_VEL));
    }
    // positions[cursor] = pos, w = 0; velocities[cursor] = vel.
    c.cycles += 22;
    c.g[V1][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = c.lw(p(e, emitter::POSITIONS));
    c.lq(A0, r32(c, A1));
    c.g[V1][0] = shl32(c.g[V1][0], 4);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(A0, r32(c, V1));
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::POSITIONS));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.sw(at(c, V0, 0xc), 0);
    c.g[V1][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V1][0] = shl32(c.g[V1][0], 4);
    c.lq(A0, r32(c, A2));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(A0, r32(c, V1));
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V1][0] = c.lw(p(e, emitter::VELOCITIES));
    c.g[V0][0] = shl32(c.g[V0][0], 4);
    let jitter = c.g[A3][0] != 0;
    c.g[S1][0] = add32(c.g[V0][0], c.g[V1][0]);
    let vel = r32(c, S1);
    if jitter {
        // w = a float in [1, 2) from the random mantissa, minus 1, plus 1
        // (as the original computes it).
        c.cycles += 14;
        call_ptr(c, callee::RANDOM, 0x0037_11d0)?;
        c.g[V1][0] = 0x007f_ffff;
        c.g[A1][0] = 0x3f80_0000;
        let one = fconst(c, 1, 0x3f80_0000);
        c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A1][0];
        let r = FpuF32(r32(c, V0));
        setf(c, 2, r);
        let age = r - one + one;
        setf(c, 0, age);
        c.swc1(0, p(vel, 0xc));
    } else {
        c.cycles += 1;
        c.sw(p(vel, 0xc), 0);
    }
    // cursor-- (wrap to capacity - 1).
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(e, emitter::CURSOR));
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    let v = c.g[V0][0];
    c.sw(p(e, emitter::CURSOR), v);
    if (v as i64) < 0 {
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(e, emitter::CAPACITY));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let v = c.g[V0][0];
        c.sw(p(e, emitter::CURSOR), v);
    }
    // time += rate * dt, back to 0 once (int)time reaches the period.
    c.cycles += 10;
    let rate = ldf(c, 0, p(e, emitter::RATE));
    let time = ldf(c, 1, p(e, emitter::TIME));
    let step = rate * fr(c, 20);
    setf(c, 0, step);
    c.g[V1][0] = c.lw(p(e, emitter::PERIOD));
    let time = time + step;
    setf(c, 1, time);
    c.cvt_w_s(0, 1);
    c.mfc1(V0, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.swc1(1, p(e, emitter::TIME));
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.sw(p(e, emitter::TIME), 0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Spatial keys.

/// `int SpatialKey_ClassifyAABB(const SpatialKey* key, const AABB* box)`
/// (0x328360): the key's cell is `[(i - m) * 2^level, (i + 1 + m) *
/// 2^level]` per axis (margin `m` at gp-0x2dfc). 1 when the box (min +0,
/// max +0x10) lies outside the cell, else 2 or 0 from the corner tests
/// below (2: the box reaches past the cell).
pub fn spatial_key_classify_aabb(c: &mut Cpu) {
    let key = abi::arg(c, 0);
    let aabb = abi::arg(c, 1);
    'outside: {
        c.cycles += 14;
        c.g[A2][0] = c.lw(p(key, 4));
        c.g[V0][0] = c.lw(key);
        let margin = ldf(c, 6, gp(c, -0x2dfc));
        c.g[V0][0] = shl32(add32(c.g[V0][0], 0x7f), 23);
        let max_x = ldf(c, 10, p(aabb, 0x10));
        let size = FpuF32(r32(c, V0));
        setf(c, 3, size);
        // low corner x
        let lo_x = (FpuF32::from_i32(r32(c, A2) as i32) - margin) * size;
        setf(c, 0, lo_x);
        if lt(c, max_x, lo_x) {
            break 'outside;
        }
        c.cycles += 9;
        c.g[V1][0] = c.lw(p(key, 8));
        let max_y = ldf(c, 9, p(aabb, 0x14));
        let lo_y = (FpuF32::from_i32(r32(c, V1) as i32) - margin) * size;
        setf(c, 0, lo_y);
        if lt(c, max_y, lo_y) {
            break 'outside;
        }
        c.cycles += 10;
        c.g[A0][0] = c.lw(p(key, 0xc));
        let max_z = ldf(c, 8, p(aabb, 0x18));
        let lo_z = (FpuF32::from_i32(r32(c, A0) as i32) - margin) * size;
        setf(c, 0, lo_z);
        let out = lt(c, max_z, lo_z);
        c.g[V0][0] = add32(c.g[A2][0], 1);
        if out {
            break 'outside;
        }
        c.cycles += 9;
        let min_x = ldf(c, 7, aabb);
        let hi_x0 = FpuF32::from_i32(r32(c, V0) as i32) + margin;
        setf(c, 0, hi_x0);
        let hi_x = hi_x0 * size;
        setf(c, 4, hi_x);
        let out = lt(c, hi_x, min_x);
        c.g[V0][0] = add32(c.g[V1][0], 1);
        if out {
            break 'outside;
        }
        c.cycles += 9;
        let min_y = ldf(c, 5, p(aabb, 4));
        let hi_y0 = FpuF32::from_i32(r32(c, V0) as i32) + margin;
        setf(c, 0, hi_y0);
        let hi_y = hi_y0 * size;
        setf(c, 2, hi_y);
        let out = lt(c, hi_y, min_y);
        c.g[V0][0] = add32(c.g[A0][0], 1);
        if out {
            break 'outside;
        }
        c.cycles += 8;
        let min_z = ldf(c, 1, p(aabb, 8));
        let hi_z = (FpuF32::from_i32(r32(c, V0) as i32) + margin) * size;
        setf(c, 0, hi_z);
        if lt(c, hi_z, min_z) {
            break 'outside;
        }
        // Overlapping on every axis: 2 if the box reaches past the cell's
        // high corner. (The last two tests compare the box minimum against
        // the high corner too, as the original does.)
        c.cycles += 3;
        let mut partial = lt(c, hi_x, max_x);
        if !partial {
            c.cycles += 3;
            partial = lt(c, hi_y, max_y);
        }
        if !partial {
            c.cycles += 3;
            partial = lt(c, hi_z, max_z);
        }
        if !partial {
            c.cycles += 3;
            partial = lt(c, min_x, hi_x);
        }
        if !partial {
            c.cycles += 4;
            let below = lt(c, min_y, hi_y);
            c.g[V0][0] = 2;
            // (The original re-tests the same condition bit here.)
            if !below {
                c.cycles += 2;
                if !c.fcc() {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Paths.

/// Path fields: segment count, first point, segments (16 bytes each:
/// direction xyz, length w).
mod path {
    pub const COUNT: u32 = 0x08;
    pub const START: u32 = 0x0c;
    pub const SEGMENTS: u32 = 0x18;
}

/// Path cursor (t0, in/out): point, distance along the path, segment (-1:
/// not on the path yet).
mod cursor {
    pub const DIST: u32 = 0x10;
    pub const SEGMENT: u32 = 0x14;
}

/// Head of the segment loop (event checks).
const PATH_LOOP: u32 = 0x0026_a6c8;

/// `float Path_GetClosestPointFrom(const Path* path, const Vec4* point,
/// Vec4* closest, float* dist, PathCursor* cursor /*t0*/, int flat /*t1*/)`
/// (0x26a638): walk the segments from the cursor (or from the start) and
/// keep the point closest to `point` (in XY when `flat`), updating the
/// cursor at each improvement; stop once the closest point is no longer
/// at the segment end (a cursor that was on the path) or past the search
/// window. Returns the distance along the path (f0) and stores the
/// distance to the point (`dist`, in XY for 3D searches).
///
/// State lives in the original's registers: t2 segment, f5 distance along
/// the path, f6 window end, f8 best squared distance (output), f9 best
/// squared distance (search), f10 result.
pub fn path_get_closest_point_from(c: &mut Cpu) {
    c.cycles += 8;
    let cur = abi::arg(c, 4);
    c.g[V1][0] = c.lw(p(cur, cursor::SEGMENT));
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    setf(c, 10, FpuF32::ZERO);
    c.g[V0][0] = !c.g[V1][0];
    ldf(c, 9, gp(c, -0x4a54));
    c.g[T3][0] = srl32(c.g[V0][0], 31);
    setf(c, 8, FpuF32::ZERO);
    let sp = sp(c);
    let path_at = abi::arg(c, 0);
    if c.g[T3][0] == 0 {
        // Not on the path yet: start at its first point, no window.
        c.cycles += 14;
        ldf(c, 2, p(path_at, path::START));
        c.g[T2][0] = 0;
        ldf(c, 1, p(path_at, path::START + 4));
        setf(c, 5, FpuF32::ZERO);
        ldf(c, 0, p(path_at, path::START + 8));
        c.mov_s(6, 9);
        fconst(c, 3, 0x3f80_0000);
        c.swc1(2, p(sp, 0x10));
        c.swc1(1, p(sp, 0x14));
        c.swc1(0, p(sp, 0x18));
        c.swc1(3, p(sp, 0x1c));
        copy_q(c, V0, p(sp, 0x10), sp);
    } else {
        // From the cursor; the window ends a fixed distance further on.
        c.cycles += 7;
        let dist = ldf(c, 5, p(cur, cursor::DIST));
        c.g[T2][0] = c.g[V1][0];
        c.lq(V0, cur);
        let window = ldf(c, 0, gp(c, -0x4a50));
        c.sq(V0, sp);
        setf(c, 6, dist + window);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(path_at, path::COUNT));
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        setf(c, 7, FpuF32::ZERO);
        c.g[V1][0] = c.lw(p(path_at, path::SEGMENTS));
        while path_segment(c, sp) {
            if event_due(c) {
                deliver_event_here(c, PATH_LOOP);
            }
        }
    }
    c.cycles += 5;
    let best = fr(c, 8).sqrt();
    setf(c, 1, best);
    c.mov_s(0, 10);
    c.swc1(1, abi::arg(c, 3));
    leave(c, 0x60);
}

/// One segment of [`path_get_closest_point_from`]: segment `t2` of the
/// array in v1, starting at the point at `sp+0`. True when the walk goes
/// on (t2 advanced, v1 reloaded).
fn path_segment(c: &mut Cpu, sp: u32) -> bool {
    let path_at = abi::arg(c, 0);
    let point = abi::arg(c, 1);
    let flat = c.g[T1][0] != 0;
    // dir = segment.xyz (w = 0), rel = point - start, t = dot(rel, dir).
    c.cycles += 17;
    c.g[V0][0] = shl32(c.g[T2][0], 4);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let seg = r32(c, V0);
    ldf(c, 0, seg);
    c.swc1(0, p(sp, 0x10));
    ldf(c, 1, p(seg, 4));
    c.swc1(1, p(sp, 0x14));
    ldf(c, 0, p(seg, 8));
    c.sw(p(sp, 0x1c), 0);
    c.swc1(0, p(sp, 0x18));
    let len = ldf(c, 4, p(seg, 0xc));
    vsub(c, point, sp, p(sp, 0x30));
    copy_q(c, V0, p(sp, 0x30), p(sp, 0x20));
    let along = if !flat {
        c.cycles += 9;
        vdot4(c, p(sp, 0x20), p(sp, 0x10), V0);
        c.mtc1(V0, 2);
        fr(c, 2)
    } else {
        c.cycles += 8;
        let dy = ldf(c, 2, p(sp, 0x14));
        let rx = ldf(c, 1, p(sp, 0x20));
        let dx = ldf(c, 3, p(sp, 0x10));
        let ry = ldf(c, 0, p(sp, 0x24));
        let px = rx * dx;
        setf(c, 1, px);
        let py = ry * dy;
        setf(c, 0, py);
        let v = px + py;
        setf(c, 2, v);
        v
    };
    // t = clamp(along, 0, len)
    c.cycles += 4;
    let zero = fr(c, 7);
    let pos = le(c, zero, along);
    setf(c, 3, zero);
    if pos {
        c.cycles += 1;
        setf(c, 3, along.min(len));
    }
    let t = fr(c, 3);
    // candidate = start + dir * t (sp+0x30); rel = point - candidate.
    c.cycles += 21;
    c.mfc1(V1, 3);
    vscale(c, p(sp, 0x10), V1, p(sp, 0x50));
    copy_q(c, V0, p(sp, 0x50), p(sp, 0x40));
    vadd(c, sp, p(sp, 0x40), p(sp, 0x50));
    copy_q(c, V0, p(sp, 0x50), p(sp, 0x30));
    vsub(c, point, p(sp, 0x30), p(sp, 0x50));
    c.lq(V0, p(sp, 0x50));
    c.sq(V0, p(sp, 0x20));
    c.sq(V0, p(sp, 0x40));
    let d2 = if !flat {
        c.cycles += 9;
        vdot4(c, p(sp, 0x20), p(sp, 0x20), V0);
        c.mtc1(V0, 1);
        fr(c, 1)
    } else {
        c.cycles += 6;
        let rx = ldf(c, 1, p(sp, 0x20));
        let ry = ldf(c, 0, p(sp, 0x24));
        let ry2 = ry * ry;
        setf(c, 0, ry2);
        let v = rx * rx + ry2;
        setf(c, 1, v);
        v
    };
    c.cycles += 3;
    if lt(c, d2, fr(c, 9)) {
        // A closer point: remember it and move the cursor here.
        c.cycles += 2;
        setf(c, 9, d2);
        if !flat {
            c.cycles += 5;
            let rx = ldf(c, 1, p(sp, 0x20));
            let ry = ldf(c, 0, p(sp, 0x24));
            let rx2 = rx * rx;
            setf(c, 1, rx2);
            let ry2 = ry * ry;
            setf(c, 0, ry2);
            setf(c, 8, rx2 + ry2);
        } else {
            c.cycles += 2;
            setf(c, 8, d2);
        }
        c.cycles += 8;
        let out = abi::arg(c, 2);
        let cur = abi::arg(c, 4);
        c.lq(V1, p(sp, 0x30));
        let along_path = fr(c, 5);
        setf(c, 10, along_path + t);
        c.lq(V0, sp);
        c.sq(V1, out);
        c.sq(V0, cur);
        c.swc1(5, p(cur, cursor::DIST));
        let seg_index = c.g[T2][0];
        c.sw(p(cur, cursor::SEGMENT), seg_index);
        if c.g[T3][0] != 0 {
            c.cycles += 3;
            if eq(c, t, along) {
                return false;
            }
        }
    }
    // Past the window: stop.
    c.cycles += 3;
    if lt(c, fr(c, 6), fr(c, 5)) {
        return false;
    }
    // Next segment: start += dir * len, distance += len.
    c.cycles += 17;
    c.mfc1(V0, 4);
    vscale(c, p(sp, 0x10), V0, p(sp, 0x50));
    c.g[T2][0] = add32(c.g[T2][0], 1);
    c.lq(V0, p(sp, 0x50));
    let along_path = fr(c, 5) + len;
    setf(c, 5, along_path);
    c.sq(V0, p(sp, 0x40));
    vadd(c, sp, p(sp, 0x40), sp);
    c.g[V0][0] = c.lw(p(path_at, path::COUNT));
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.lw(p(path_at, path::SEGMENTS));
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// Bezier patches.

/// `GameModule_DrawBezierPatch(Patch* patch, View* view, void* arg /*a2*/)`
/// (0x22c410): tessellation level per patch edge from the squared
/// distances between the patch centre (+0x10) and the four corner points
/// held in the view object (+0x170..0x19c): `k * (d_a + d_b) + bias` for
/// the edges (0,1), (2,3), (0,2), (1,3), clamped to [min, max] (patch
/// +0x20..0x2c) and mapped through a byte table (gp-0x6e0); then the
/// renderer's virtual 0xa8 draws the patch with the four levels and two
/// halfwords of the view (+0x1a0/+0x1a2).
pub fn game_module_draw_bezier_patch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        let frame = Frame::open(c, 0x50);
        c.g[T4][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0x48);
        c.g[T5][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        let patch = r32(c, T5);
        let view = r32(c, T4);
        let lod_min = ldf(c, 21, p(patch, 0x28));
        // The patch centre (+0x10) against the view's four points.
        let cx = ldf(c, 8, p(patch, 0x10));
        let cy = ldf(c, 2, p(patch, 0x14));
        setf(c, 23, lod_min);
        let ax = ldf(c, 0, p(view, 0x170));
        let ay = ldf(c, 1, p(view, 0x174));
        let dx = cx - ax;
        let cz = ldf(c, 4, p(patch, 0x18));
        let dy = cy - ay;
        let az = ldf(c, 0, p(view, 0x178));
        let bx = ldf(c, 1, p(view, 0x17c));
        let by = ldf(c, 3, p(view, 0x180));
        let dz = cz - az;
        let d0x = dx * dx;
        let bz = ldf(c, 0, p(view, 0x184));
        let d0y = dy * dy;
        let ex = ldf(c, 9, p(view, 0x188));
        let dx = cx - bx;
        let ey = ldf(c, 10, p(view, 0x18c));
        let dy = cy - by;
        let ez = ldf(c, 11, p(view, 0x190));
        let d0z = dz * dz;
        let gx = ldf(c, 14, p(view, 0x194));
        let dz = cz - bz;
        let gy = ldf(c, 12, p(view, 0x198));
        let d1y = dy * dy;
        let gz = ldf(c, 15, p(view, 0x19c));
        let d1x = dx * dx;
        let k = ldf(c, 6, p(patch, 0x20));
        let dx = cx - ex;
        let bias = ldf(c, 7, p(patch, 0x24));
        let dy = cy - ey;
        let lod_max = ldf(c, 22, p(patch, 0x2c));
        let d1z = dz * dz;
        setf(c, 13, d1z);
        let dz = cz - ez;
        let d2x = dx * dx;
        let d2y = dy * dy;
        setf(c, 9, d2y);
        let dx = cx - gx;
        setf(c, 19, dx);
        let dy = cy - gy;
        setf(c, 18, dy);
        let d1 = d1x + d1y;
        let d2z = dz * dz;
        setf(c, 8, d2z);
        let d0 = d0x + d0y;
        setf(c, 16, d0y);
        setf(c, 17, d0z);
        let d3y = dy * dy;
        let dz = cz - gz;
        setf(c, 20, dz);
        let d3x = dx * dx;
        let d0 = d0 + d0z;
        let d1 = d1 + d1z;
        let d2 = d2x + d2y;
        let d3 = d3x + d3y;
        let d3z = dz * dz;
        setf(c, 4, d3z);
        let s01 = d0 + d1;
        let d2 = d2 + d2z;
        let d3 = d3 + d3z;
        let s01 = k * s01;
        let s02 = d0 + d2;
        setf(c, 5, s02);
        let s13 = d1 + d3;
        let s23 = d2 + d3;
        let e01 = bias + s01;
        let s13 = k * s13;
        setf(c, 3, s13);
        let s23 = k * s23;
        let s02k = k * s02;
        setf(c, 6, s02k);
        let in01 = le(c, lod_min, e01);
        let e13 = bias + s13;
        let e23 = bias + s23;
        let e02 = bias + s02k;
        setf(c, 10, e01);
        setf(c, 1, e13);
        setf(c, 0, e23);
        setf(c, 7, e02);
        setf(c, 2, d3);
        // Clamp each edge level to [min, max].
        if in01 {
            c.cycles += 1;
            setf(c, 23, e01.min(lod_max));
        }
        c.cycles += 4;
        let in23 = le(c, lod_min, e23);
        c.mov_s(10, 23);
        setf(c, 6, lod_min);
        if in23 {
            c.cycles += 1;
            setf(c, 6, e23.min(lod_max));
        }
        c.cycles += 4;
        let in02 = le(c, lod_min, e02);
        c.mov_s(0, 6);
        setf(c, 6, lod_min);
        if in02 {
            c.cycles += 1;
            setf(c, 6, e02.min(lod_max));
        }
        c.cycles += 4;
        let in13 = le(c, lod_min, e13);
        c.mov_s(7, 6);
        setf(c, 2, lod_min);
        if in13 {
            c.cycles += 1;
            setf(c, 2, e13.min(lod_max));
        }
        // Levels through the byte table; call the renderer.
        c.cycles += 41;
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        c.g[V1][0] = sx(gp(c, -0x6e0));
        c.cvt_w_s(1, 10);
        c.mfc1(A0, 1);
        c.g[T3][0] = c.lh(p(view, 0x1a0));
        c.cvt_w_s(0, 7);
        c.mfc1(A2, 0);
        c.g[T2][0] = c.lh(p(view, 0x1a2));
        c.cvt_w_s(0, 2);
        c.mfc1(V0, 0);
        let table = c.g[V1][0];
        c.g[A1][0] = add32(c.g[A1][0], table);
        c.g[A0][0] = add32(c.g[A0][0], table);
        let sp = sp(c);
        let v = c.g[T3][0];
        c.sw(p(sp, 0x10), v);
        c.g[A3][0] = c.lbu(r32(c, A0));
        c.g[A2][0] = add32(c.g[A2][0], table);
        c.g[V0][0] = add32(c.g[V0][0], table);
        c.g[A0][0] = c.lbu(r32(c, A1));
        c.g[V1][0] = c.lbu(r32(c, V0));
        c.g[A1][0] = c.g[T4][0];
        c.g[T1][0] = c.lbu(r32(c, A2));
        let v = c.g[A3][0];
        c.sb(sp, v);
        c.g[A2][0] = c.g[SP][0];
        let v = c.g[A0][0];
        c.sb(p(sp, 1), v);
        c.g[A3][0] = sx(p(sp, 0x10));
        let v = c.g[V1][0];
        c.sb(p(sp, 3), v);
        let v = c.g[T2][0];
        c.sw(p(sp, 0x14), v);
        let v = c.g[T1][0];
        c.sb(p(sp, 2), v);
        c.g[T1][0] = c.lw(patch);
        c.g[V0][0] = c.lw(at(c, T1, 4));
        let vt = r32(c, V0);
        c.g[A0][0] = c.lh(p(vt, 0xa8));
        c.g[V1][0] = c.lw(p(vt, 0xac));
        c.g[A0][0] = add32(c.g[T1][0], c.g[A0][0]);
        let target = r32(c, V1);
        call_ptr(c, target, 0x0022_c604)?;
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Texture sampling.

/// Texture record fields (`PS2GraphicsMan` texture table, `+0x18f4`).
mod texture {
    /// Pixel storage mode: 0 32-bit, 0xa 8-bit indexed, 0xc 4-bit indexed.
    pub const PSM: u32 = 0x0c;
    pub const PIXELS: u32 = 0x1c;
    pub const CLUT: u32 = 0x20;
    /// The GS TEX0 doubleword (TW at bit 26, TH at bit 30).
    pub const TEX0: u32 = 0x38;
}

/// Head of the texel loop (event checks).
const V58_TEXELS: u32 = 0x0038_8cd8;
/// Head of the (empty) counting loop before it.
const V58_SPIN: u32 = 0x0038_8cb0;

/// `sext(x)`: `dsll32; dsra32` (sign-extend the low word).
#[inline(always)]
fn sext(x: u64) -> u64 {
    sx(x as u32)
}

/// `1 << ((ld tex0 >> shift) & 0xf)` as the original computes it, leaving
/// v0 = the exponent and `reg` = the size. `tex0` is the doubleword
/// (a3 or a fresh `ld`).
#[inline(always)]
fn tex_size(c: &mut Cpu, tex0: u64, shift: u32, one: usize, reg: usize) -> u64 {
    c.g[V0][0] = sext((tex0 >> shift) & 0xf);
    let n = c.g[V0][0] as u32 & 31;
    c.g[reg][0] = sx((c.g[one][0] as u32) << n);
    c.g[reg][0]
}

/// `if (v < 0) v = 0; else if (size < v) v = size;` on GPR `reg` (`size`
/// in v1), with the original's cycles.
#[inline(always)]
fn clamp_index(c: &mut Cpu, reg: usize) {
    if (c.g[reg][0] as i64) < 0 {
        c.cycles += 1;
        c.g[reg][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[reg][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[reg][0] = c.g[V1][0];
        }
    }
}

/// `Colour* PS2GraphicsMan_v58(Colour* out, PS2GraphicsMan* gm, int tex,
/// float u /*f12*/, float v /*f13*/)` (0x3889f0): bilinear sample of
/// texture `tex` at (u, v) (wrapped to [0, 1)), as four floats 0..255
/// scaled by a constant (gp-0x2730); white when the texture slot is empty.
/// Returns `out`.
pub fn ps2_graphics_man_v58(c: &mut Cpu) {
    c.cycles += 8;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xe0));
    c.g[A2][0] = shl32(c.g[A2][0], 2);
    let sp = sp(c);
    c.sq(S0, p(sp, 0xd0));
    c.g[V0][0] = c.lw(at(c, A1, 0x18f4));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    c.g[T6][0] = c.lw(at(c, V0, 8));
    c.g[S0][0] = c.g[A0][0];
    let out = r32(c, S0);
    if c.g[T6][0] == 0 {
        c.cycles += 7;
        fconst(c, 0, 0x3f80_0000);
        c.sw(p(out, 8), 0);
        c.swc1(0, p(out, 0xc));
        c.swc1(0, out);
        c.swc1(0, p(out, 4));
    } else {
        v58_sample(c, sp, out);
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[S0][0];
    c.lq(S0, p(sp, 0xd0));
    leave(c, 0xe0);
}

/// The texel coordinates: `f5`/`f12` = u*w -/+ 0.5 and `f13`/`f1` = v*h
/// -/+ 0.5, each clamped to [0, size]; their integer parts (clamped again)
/// at sp+0, 4, 8, 0xc.
fn v58_coords(c: &mut Cpu, sp: u32) {
    let tex = r32(c, T6);
    c.cycles += 40;
    c.g[V1][0] = c.ld(p(tex, texture::TEX0));
    c.cvt_w_s(0, 12);
    c.mfc1(A1, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(A2, 0);
    c.g[A0][0] = 1;
    c.g[V0][0] = c.g[V1][0] >> 26;
    c.g[A3][0] = c.g[V1][0];
    c.mtc1(A1, 0);
    c.cvt_s_w(0, 0);
    c.g[V0][0] = sext(c.g[V0][0] & 0xf);
    c.mtc1(A2, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = sx(1u32 << (c.g[V0][0] as u32 & 31));
    c.g[V1][0] = sext((c.g[V1][0] >> 30) & 0xf);
    // fractional parts
    let fu = fr(c, 12) - fr(c, 0);
    setf(c, 12, fu);
    c.mtc1(V0, 2);
    c.cvt_s_w(2, 2);
    let fv = fr(c, 13) - fr(c, 1);
    setf(c, 13, fv);
    c.g[A0][0] = sx(1u32 << (c.g[V1][0] as u32 & 31));
    let half = fconst(c, 0, 0x3f00_0000);
    c.mtc1(A0, 1);
    c.cvt_s_w(1, 1);
    let width = fr(c, 2);
    let su = fu * width;
    c.g[T8][0] = c.lw(p(tex, texture::PIXELS));
    setf(c, 3, FpuF32::ZERO);
    let sv = fv * fr(c, 1);
    let u0 = su - half;
    setf(c, 5, u0);
    setf(c, 12, su + half);
    setf(c, 1, sv + half);
    let neg = lt(c, u0, FpuF32::ZERO);
    setf(c, 13, sv - half);
    if neg {
        c.cycles += 2;
        setf(c, 5, FpuF32::ZERO);
    } else {
        c.cycles += 4;
        if lt(c, width, u0) {
            setf(c, 5, width);
        }
    }
    // u1 in [0, w]
    c.cycles += 5;
    setf(c, 0, FpuF32::ZERO);
    if lt(c, fr(c, 12), FpuF32::ZERO) {
        setf(c, 12, FpuF32::ZERO);
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 26, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        if lt(c, fr(c, 0), fr(c, 12)) {
            c.mov_s(12, 0);
        }
        c.cycles += 1;
        setf(c, 0, FpuF32::ZERO);
    }
    // v0 in [0, h]
    c.cycles += 4;
    if lt(c, fr(c, 13), fr(c, 0)) {
        c.mov_s(13, 0);
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 30, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        if lt(c, fr(c, 0), fr(c, 13)) {
            c.mov_s(13, 0);
        }
    }
    // v1 in [0, h]
    c.cycles += 5;
    setf(c, 0, FpuF32::ZERO);
    if lt(c, fr(c, 1), FpuF32::ZERO) {
        c.mov_s(1, 0);
        c.cycles += 1;
        c.g[V0][0] = c.g[A3][0] >> 26;
    } else {
        c.cycles += 12;
        let tex0 = c.g[A3][0];
        c.g[V1][0] = 1;
        tex_size(c, tex0, 30, V1, V1);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        let over = lt(c, fr(c, 0), fr(c, 1));
        c.g[V0][0] = c.g[A3][0] >> 26;
        if over {
            c.cycles += 1;
            c.mov_s(1, 0);
            c.cycles += 1;
            c.g[V0][0] = c.g[A3][0] >> 26;
        }
    }
    // Integer coordinates, clamped to the size: x0, x1, y0, y1.
    c.cycles += 8;
    c.cvt_w_s(0, 5);
    c.mfc1(A0, 0);
    c.g[V0][0] = sext(c.g[V0][0] & 0xf);
    c.g[V1][0] = sx(1u32 << (c.g[V0][0] as u32 & 31));
    clamp_index(c, A0);
    for (store_off, f, reg, shift) in [(0u32, 12usize, A1, 26u32), (4, 13, A0, 30), (8, 1, A0, 30)] {
        c.cycles += 11;
        let prev = if store_off == 4 { A1 } else { A0 };
        let v = c.g[prev][0];
        c.sw(p(sp, store_off), v);
        if store_off == 8 {
            c.g[V1][0] = 1;
            c.cvt_w_s(0, f);
            c.mfc1(reg, 0);
        } else {
            c.cvt_w_s(0, f);
            c.mfc1(reg, 0);
            c.g[V1][0] = 1;
        }
        let tex0 = c.ld(p(r32(c, T6), texture::TEX0));
        c.g[V0][0] = tex0;
        tex_size(c, tex0, shift, V1, V1);
        clamp_index(c, reg);
    }
}

/// The body of [`ps2_graphics_man_v58`] for a present texture.
fn v58_sample(c: &mut Cpu, sp: u32, out: u32) {
    v58_coords(c, sp);
    c.cycles += 5;
    c.g[A2][0] = 0x7fff_ffff;
    let v = c.g[A0][0];
    c.sw(p(sp, 0xc), v);
    c.g[V0][0] = 3;
    c.g[V1][0] = u64::MAX;
    // A counting loop with an empty body (four passes).
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[T7][0] = 0;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, V58_SPIN);
        }
    }
    // The four texels (x0,y0), (x0,y1), (x1,y0), (x1,y1) as floats at
    // sp+0x10.. (t5).
    c.cycles += 3;
    c.g[T9][0] = 1;
    c.g[T5][0] = sx(p(sp, 0x10));
    loop {
        v58_texel(c, sp);
        c.cycles += 21;
        let colour = c.g[A2][0];
        c.g[A0][0] = sra32(colour, 8);
        c.g[V0][0] = srl32(colour, 24);
        c.g[V1][0] &= 0xff;
        c.g[A0][0] &= 0xff;
        c.g[A1][0] = colour & 0xff;
        c.mtc1(V0, 3);
        c.cvt_s_w(3, 3);
        c.g[T7][0] = add32(c.g[T7][0], 1);
        c.mtc1(V1, 2);
        c.cvt_s_w(2, 2);
        c.g[V0][0] = ((c.g[T7][0] as i64) < 4) as u64;
        c.mtc1(A0, 0);
        c.cvt_s_w(0, 0);
        c.mtc1(A1, 1);
        c.cvt_s_w(1, 1);
        let dst = r32(c, T5);
        c.swc1(3, p(dst, 0xc));
        c.swc1(2, dst);
        c.swc1(0, p(dst, 4));
        c.swc1(1, p(dst, 8));
        c.g[T5][0] = sx(p(dst, 0x10));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, V58_TEXELS);
        }
    }
    v58_blend(c, sp, out);
}

/// Fetch texel `t7` (x from sp+0/4, y from sp+8/0xc) into a2 (0xAARRGGBB
/// style word) and v1 (`a2 >> 16`). Unknown formats leave a2 as it was.
fn v58_texel(c: &mut Cpu, sp: u32) {
    let tex = r32(c, T6);
    c.cycles += 11;
    let i = c.g[T7][0];
    c.g[V0][0] = i & 1;
    c.g[V1][0] = sra32(i, 1);
    c.g[V0][0] = add32(c.g[V0][0], 2);
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[A0][0] = add32(sp as u64, c.g[V1][0]);
    c.g[A1][0] = add32(sp as u64, c.g[V0][0]);
    c.g[T3][0] = c.lw(p(tex, texture::PSM));
    c.g[T4][0] = c.lw(r32(c, A0));
    c.g[T2][0] = c.lw(r32(c, A1));
    let psm = c.g[T3][0];
    if psm == 0 {
        // 32-bit: pixels[y * w + x].
        c.cycles += 11;
        let tex0 = c.ld(p(tex, texture::TEX0));
        tex_size(c, tex0, 26, T9, V0);
        c.mult(0, V1, T2, V0);
        c.g[V0][0] = add32(c.g[V1][0], c.g[T4][0]);
        c.g[V0][0] = shl32(c.g[V0][0], 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[T8][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        if psm == 0xa {
            v58_texel_psmt8(c, tex);
        } else {
            c.g[V0][0] = 0xc;
            c.cycles += 2;
            let t4 = psm != c.g[V0][0];
            c.g[V1][0] = sra32(c.g[A2][0], 16);
            if t4 {
                return;
            }
            v58_texel_psmt4(c, tex);
        }
    }
    c.cycles += 2;
    c.g[A2][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sra32(c.g[A2][0], 16);
}

/// 8-bit indexed texel: the swizzled byte address, then the CLUT entry
/// address in v0.
fn v58_texel_psmt8(c: &mut Cpu, tex: u32) {
    c.cycles += 52;
    c.g[V0][0] = c.ld(p(tex, texture::TEX0));
    let y = c.g[T2][0];
    c.g[A0][0] = y & 0x3c;
    c.g[A1][0] = y & 1;
    c.g[V1][0] = sra32(y, 6);
    c.g[V0][0] >>= 26;
    c.g[A0][0] = sra32(c.g[A0][0], 1);
    c.g[V0][0] &= 0xf;
    c.g[A0][0] = add32(c.g[A0][0], c.g[A1][0]);
    c.g[A3][0] = y & 2;
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[T0][0] = y & 4;
    c.g[V0][0] = sext(c.g[V0][0]);
    c.g[T2][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.g[V0][0] = sx((c.g[T9][0] as u32) << (c.g[V0][0] as u32 & 31));
    c.mult(0, V0, T2, V0);
    c.g[A0][0] = shl32(c.g[A3][0], 1);
    let x = c.g[T4][0];
    c.g[V1][0] = x & 7;
    c.g[A2][0] = x & 0x70;
    c.g[V1][0] ^= c.g[A0][0];
    c.g[A2][0] = sra32(c.g[A2][0], 1);
    c.g[V1][0] ^= c.g[T0][0];
    c.g[A1][0] = sra32(x, 7);
    c.g[A2][0] = add32(c.g[A2][0], c.g[V1][0]);
    c.g[A0][0] = srl32(c.g[V0][0], 31);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = x & 8;
    c.g[A1][0] = shl32(c.g[A1][0], 6);
    c.g[V0][0] = sra32(c.g[V0][0], 1);
    c.g[T4][0] = add32(c.g[A1][0], c.g[A2][0]);
    c.g[V1][0] = shl32(c.g[V1][0], 1);
    c.g[V0][0] = add32(c.g[V0][0], c.g[T4][0]);
    c.g[A3][0] = shl32(c.g[A3][0], 2);
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A3][0]);
    c.g[V1][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.g[A2][0] = c.lw(p(tex, texture::CLUT));
    c.g[V0][0] = sra32(c.g[V1][0], 3);
    c.g[V0][0] = add32(c.g[T8][0], c.g[V0][0]);
    // The CLUT index with bits 3 and 4 swapped (CSM1 layout).
    c.g[A1][0] = c.lbu(r32(c, V0));
    let idx = c.g[A1][0];
    c.g[A0][0] = idx & 8;
    c.g[V0][0] = idx & 7;
    c.g[V1][0] = idx & 0x10;
    c.g[A0][0] = shl32(c.g[A0][0], 1);
    c.g[V0][0] |= c.g[A0][0];
    c.g[V1][0] = srl32(c.g[V1][0], 1);
    c.g[V0][0] |= c.g[V1][0];
    c.g[A1][0] = idx & 0xe0;
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
}

/// 4-bit indexed texel: the swizzled nibble, then the CLUT entry address
/// in v0.
fn v58_texel_psmt4(c: &mut Cpu, tex: u32) {
    c.cycles += 42;
    let x = c.g[T4][0];
    let y = c.g[T2][0];
    c.g[A0][0] = x & 0x60;
    c.g[V0][0] = y & 0xc;
    c.g[V1][0] = c.ld(p(tex, texture::TEX0));
    c.g[V0][0] = sra32(c.g[V0][0], 1);
    c.g[A0][0] = sra32(c.g[A0][0], 2);
    c.g[A1][0] = y & 1;
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    c.g[V1][0] >>= 26;
    c.g[V0][0] = sra32(y, 7);
    c.g[V1][0] &= 0xf;
    c.g[A0][0] = add32(c.g[A0][0], c.g[A1][0]);
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[A3][0] = y & 0x70;
    c.g[T0][0] = y & 2;
    c.g[T1][0] = y & 4;
    c.g[V1][0] = sext(c.g[V1][0]);
    c.g[T2][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = sx((c.g[T9][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.mult(0, V1, T2, V1);
    c.g[T0][0] = shl32(c.g[T0][0], 1);
    c.g[V0][0] = x & 7;
    c.g[A1][0] = sra32(x, 7);
    c.g[V0][0] ^= c.g[T0][0];
    c.g[A3][0] = sra32(c.g[A3][0], 1);
    c.g[V0][0] ^= c.g[T1][0];
    c.g[A2][0] = x & 0x18;
    c.g[A0][0] = srl32(c.g[V1][0], 31);
    c.g[A3][0] = add32(c.g[A3][0], c.g[V0][0]);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.g[A1][0] = shl32(c.g[A1][0], 6);
    c.g[T4][0] = add32(c.g[A1][0], c.g[A3][0]);
    c.g[V1][0] = sra32(c.g[V1][0], 1);
    c.g[V1][0] = add32(c.g[V1][0], c.g[T4][0]);
    c.g[A2][0] = add32(c.g[A2][0], c.g[T0][0]);
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[A2][0]);
    c.g[V0][0] = sra32(c.g[V1][0], 3);
    c.g[V0][0] = add32(c.g[T8][0], c.g[V0][0]);
    c.g[V1][0] &= 4;
    let high = c.g[V1][0] != 0;
    c.g[V0][0] = c.lbu(r32(c, V0));
    if high {
        c.cycles += 2;
        c.g[V0][0] = srl32(c.g[V0][0], 4);
    } else {
        c.cycles += 1;
        c.g[V0][0] &= 0xf;
    }
    c.cycles += 3;
    c.g[V1][0] = c.lw(p(tex, texture::CLUT));
    c.g[V0][0] = shl32(c.g[V0][0], 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
}

/// Bilinear blend of the four texels (VU0, through the stack) and the
/// scaled result into `out` (unaligned doubleword copies).
fn v58_blend(c: &mut Cpu, sp: u32, out: u32) {
    c.cycles += 86;
    c.cvt_w_s(0, 5);
    c.mfc1(V1, 0);
    let one = fconst(c, 3, 0x3f80_0000);
    c.cvt_w_s(0, 13);
    c.mfc1(V0, 0);
    let scale = ldf(c, 4, gp(c, -0x2730));
    c.mtc1(V1, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    let fx = fr(c, 5) - fr(c, 2);
    setf(c, 2, fx);
    let fy = fr(c, 13) - fr(c, 1);
    setf(c, 1, fy);
    let gx = one - fx;
    setf(c, 0, gx);
    let gy = one - fy;
    setf(c, 3, gy);
    setf(c, 5, fx * fy);
    setf(c, 1, gx * fy);
    setf(c, 0, gx * gy);
    // w00 * t00
    c.mfc1(V0, 0);
    vscale(c, p(sp, 0x10), V0, p(sp, 0x90));
    setf(c, 2, fx * gy);
    copy_q(c, V0, p(sp, 0x90), p(sp, 0x80));
    // + w01 * t01
    c.mfc1(V1, 1);
    vscale(c, p(sp, 0x20), V1, p(sp, 0xa0));
    copy_q(c, V0, p(sp, 0xa0), p(sp, 0x90));
    vadd(c, p(sp, 0x80), p(sp, 0x90), p(sp, 0xa0));
    copy_q(c, V0, p(sp, 0xa0), p(sp, 0x70));
    // + w10 * t10
    c.mfc1(V1, 2);
    vscale(c, p(sp, 0x30), V1, p(sp, 0xb0));
    copy_q(c, V0, p(sp, 0xb0), p(sp, 0xa0));
    vadd(c, p(sp, 0x70), p(sp, 0xa0), p(sp, 0xb0));
    copy_q(c, V0, p(sp, 0xb0), p(sp, 0x60));
    // + w11 * t11
    c.mfc1(V1, 5);
    vscale(c, p(sp, 0x40), V1, p(sp, 0xc0));
    copy_q(c, V0, p(sp, 0xc0), p(sp, 0xb0));
    vadd(c, p(sp, 0x60), p(sp, 0xb0), p(sp, 0xc0));
    c.lq(V0, p(sp, 0xc0));
    c.sq(V0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x50));
    // Scaled, with the lanes reversed (w, z, y, x) into sp+0x50.
    let w = ldf(c, 1, p(sp, 0x1c));
    let z = ldf(c, 2, p(sp, 0x18));
    let y = ldf(c, 3, p(sp, 0x14));
    setf(c, 1, w * scale);
    let x = ldf(c, 0, p(sp, 0x10));
    setf(c, 2, z * scale);
    setf(c, 3, y * scale);
    setf(c, 0, x * scale);
    c.swc1(1, p(sp, 0x50));
    c.swc1(2, p(sp, 0x54));
    c.swc1(3, p(sp, 0x58));
    c.swc1(0, p(sp, 0x5c));
    c.ldl(V0, p(sp, 0x57));
    c.ldr(V0, p(sp, 0x50));
    c.ldl(V1, p(sp, 0x5f));
    c.ldr(V1, p(sp, 0x58));
    c.sdl(V0, p(out, 7));
    c.sdr(V0, out);
    c.sdl(V1, p(out, 0xf));
    c.sdr(V1, p(out, 8));
}

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
mod view {
    pub const STRIDE: u32 = 0x50;
    pub const EYE: u32 = 0x250;
    /// View box (min, max).
    pub const BOX: u32 = 0x260;
    /// Child-plane points for the two children.
    pub const LEFT_POINT: u32 = 0x260;
    pub const RIGHT_POINT: u32 = 0x270;
    pub const HULL_RANGE: u32 = 0x280;
    pub const HULL_DIST: u32 = 0x284;
    pub const NEAREST: u32 = 0x288;
}

/// Streamer tree node fields.
mod node {
    pub const BOX_MIN: u32 = 0x00;
    pub const BOX_MAX: u32 = 0x10;
    pub const LEFT_PLANE: u32 = 0x20;
    pub const RIGHT_PLANE: u32 = 0x30;
    pub const LEFT: u32 = 0x50;
    pub const RIGHT: u32 = 0x54;
    pub const HULL: u32 = 0x58;
}

/// `v1 = view * 0x50` (`mult`, LO/HI as the original), `v0 = wc + v1`:
/// the view record (v0 = 0x50 on entry).
#[inline(always)]
fn view_record(c: &mut Cpu) -> u32 {
    c.mult(0, V1, S3, V0);
    c.g[V0][0] = add32(c.g[V1][0], c.g[S2][0]);
    r32(c, V0)
}

/// `void cWorldCache_StreamerUpdateViewNode(cWorldCache* wc, Node* node,
/// int view, int store_dist /*a3*/)` (0x3aa028): if the node's box
/// overlaps the view's box, request its hull with a priority from the
/// eye's distance to the node box (inside: a fixed priority; hull not
/// resident yet: the distance is not kept), keep the nearest distance per
/// view, and recurse into the children whose split plane faces the view.
pub fn world_cache_streamer_update_view_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = view::STRIDE as u64;
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.mult(0, V1, S3, V0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = add32(c.g[V1][0], c.g[S2][0]);
        frame.save_fpr(c, 23, 0x78);
        c.g[V0][0] = add32(c.g[V0][0], view::BOX as u64);
        frame.save_fpr(c, 22, 0x70);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let vbox = r32(c, V0);
        let nd = r32(c, S1);
        // Box overlap: view.min <= node.max and node.min <= view.max per
        // axis (x, then y, then z).
        c.g[V1][0] = 0;
        let mut overlap = true;
        for (i, (f1_at, f0_at, rev)) in [
            (p(nd, node::BOX_MAX), vbox, false),
            (p(vbox, 0x10), p(nd, node::BOX_MIN), false),
            (p(vbox, 4), p(nd, node::BOX_MAX + 4), true),
            (p(vbox, 0x14), p(nd, node::BOX_MIN + 4), false),
            (p(vbox, 8), p(nd, node::BOX_MAX + 8), true),
            (p(vbox, 0x18), p(nd, node::BOX_MIN + 8), false),
        ]
        .into_iter()
        .enumerate()
        {
            if i > 0 {
                c.cycles += if i == 5 { 6 } else { 5 };
            }
            let f1 = ldf(c, 1, f1_at);
            let f0 = ldf(c, 0, f0_at);
            let holds = if rev { le(c, f1, f0) } else { le(c, f0, f1) };
            if !holds {
                overlap = false;
                break;
            }
        }
        if overlap {
            c.g[V1][0] = 1;
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x50);
        if c.g[V1][0] != 0 && streamer_visit(c, frame)? {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x78);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// An overlapping node: hull request, then the children. `Ok(false)` when
/// the original leaves early (eye beyond the view's hull range: neither
/// request nor children; s0 already restored).
fn streamer_visit(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    let nd = r32(c, S1);
    let wc = r32(c, S2);
    c.cycles += 3;
    c.g[V0][0] = c.lw(p(nd, node::HULL));
    let has_hull = c.g[V0][0] != 0;
    c.g[S0][0] = view::STRIDE as u64;
    if has_hull {
        // dist = PointBoxDistance(wc, eye, box)
        c.cycles += 14;
        c.g[A0][0] = c.g[S2][0];
        c.mult(0, S0, S3, S0);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = sx(p(nd, node::BOX_MAX));
        setf(c, 22, FpuF32::ZERO);
        c.g[A1][0] = add32(c.g[S0][0], c.g[S2][0]);
        c.g[A1][0] = add32(c.g[A1][0], view::EYE as u64);
        callee::POINT_BOX_DISTANCE.call(c, 0x003a_a128)?;
        c.mov_s(20, 0);
        let dist = fr(c, 20);
        c.g[V0][0] = add32(c.g[S2][0], c.g[S0][0]);
        let rec = r32(c, V0);
        let inside = lt(c, dist, fr(c, 22));
        ldf(c, 21, p(rec, view::HULL_DIST));
        if inside {
            c.cycles += 7;
            hull_id(c, nd);
            ldf(c, 23, gp(c, -0x23c0));
            callee::IS_HULL_RESIDENT.call(c, 0x003a_a154)?;
            if c.g[V0][0] == 0 {
                c.mov_s(21, 22);
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 5;
            let range = ldf(c, 0, p(rec, view::HULL_RANGE));
            let beyond = lt(c, range, dist);
            frame.restore(c, S0, 0x50);
            if beyond {
                return Ok(false);
            }
            // priority = (1 - dist / range) * scale
            c.cycles += 8;
            let ratio = dist.div(range);
            setf(c, 0, ratio);
            let one = fconst(c, 1, 0x3f80_0000);
            let scale = ldf(c, 2, gp(c, -0x23bc));
            let nearer = lt(c, dist, fr(c, 21));
            let inv = one - ratio;
            setf(c, 1, inv);
            setf(c, 23, inv * scale);
            if nearer {
                c.cycles += 6;
                hull_id(c, nd);
                callee::IS_HULL_RESIDENT.call(c, 0x003a_a1a8)?;
                if c.g[V0][0] == 0 {
                    c.mov_s(21, 20);
                }
            }
        }
        // Store the distance (if asked) and keep the nearest.
        c.cycles += 2;
        let store = c.g[S4][0] != 0;
        c.g[V0][0] = view::STRIDE as u64;
        if store {
            c.cycles += 3;
            let rec = view_record(c);
            c.swc1(21, p(rec, view::HULL_DIST));
        }
        c.cycles += 8;
        c.g[V0][0] = view::STRIDE as u64;
        let rec = view_record(c);
        let nearest = ldf(c, 0, p(rec, view::NEAREST));
        if lt(c, fr(c, 21), nearest) {
            c.swc1(21, p(rec, view::NEAREST));
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw(p(nd, node::HULL));
        c.mov_s(12, 23);
        c.g[A0][0] = c.lw(wc);
        c.g[A1][0] = c.lhu(at(c, V0, 2));
        callee::REQUEST_HULL.call(c, 0x003a_a1f8)?;
    }
    // Children whose plane (dotted with the view point) is negative.
    for (child, plane, point, ret) in [
        (node::LEFT, node::LEFT_PLANE, view::LEFT_POINT, 0x003a_a250u32),
        (node::RIGHT, node::RIGHT_PLANE, view::RIGHT_POINT, 0x003a_a2a8),
    ] {
        c.cycles += 3;
        c.g[A1][0] = c.lw(p(nd, child));
        let present = c.g[A1][0] != 0;
        c.g[V0][0] = view::STRIDE as u64;
        if !present {
            continue;
        }
        c.cycles += 16;
        setf(c, 1, FpuF32::ZERO);
        let rec = view_record(c);
        let side = vdot4(c, p(nd, plane), p(rec, point), V1);
        c.mtc1(V1, 0);
        let behind = lt(c, side, FpuF32::ZERO);
        c.g[A0][0] = c.g[S2][0];
        if behind {
            c.cycles += 3;
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S4][0];
            callee::STREAMER_UPDATE_VIEW_NODE.call(c, ret)?;
        }
    }
    Ok(true)
}

/// `v0 = node->hull; a0 = wc->cache; a1 = hull->id (lhu +2)`: the
/// arguments of the hull calls.
#[inline(always)]
fn hull_id(c: &mut Cpu, nd: u32) {
    c.g[V0][0] = c.lw(p(nd, node::HULL));
    c.g[A0][0] = c.lw(r32(c, S2));
    c.g[A1][0] = c.lhu(at(c, V0, 2));
}

// ---------------------------------------------------------------------------
// HUD.

/// `ldl/ldr reg, a` (an unaligned doubleword load).
#[inline(always)]
fn ld_d(c: &mut Cpu, reg: usize, a: u32) {
    c.ldl(reg, p(a, 7));
    c.ldr(reg, a);
}

/// `sdl/sdr reg, a` (an unaligned doubleword store).
#[inline(always)]
fn sd_d(c: &mut Cpu, reg: usize, a: u32) {
    c.sdl(reg, p(a, 7));
    c.sdr(reg, a);
}

/// HUD fields used by the position panel.
mod hud {
    /// The text object (`+0x30` size, `+0x38` scaled size, `+0x40` colour).
    pub const TEXT: u32 = 0x42c;
    pub const PANEL_FLAGS: u32 = 0x474;
    pub const PANEL_X: u32 = 0x480;
    pub const PANEL_TOP: u32 = 0x484;
    pub const PANEL_BOTTOM: u32 = 0x488;
    pub const PANEL_DEPTH: u32 = 0x48c;
    pub const SPRITE: u32 = 0x494;
}

/// `void HUD_DrawPositionPanel(Hud* hud, Elem* elems, Rider* rider, int
/// ordinal /*a3*/, int place /*t0*/, Vec2* scale /*t1*/, Vec2* scale2
/// /*t2*/, int mode /*t3*/, int row /*stack*/)` (0x21ed48): the race
/// position: the place number (text, from the place-name table or the
/// ordinal digits), in mode 2 its sprite with a gauge panel (one or three
/// `HUD_DrawPanelSegment`s by the gauge width).
pub fn hud_draw_position_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x190);
        let sp = sp(c);
        fconst(c, 0, 0x3f80_0000);
        frame.save(c, S0, 0x160);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S7, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x150);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S3, 0x130);
        c.g[V1][0] = 0x004a_0000;
        frame.save(c, FP, 0xe0);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S2, 0x140);
        c.g[FP][0] = c.g[A3][0];
        frame.save(c, S4, 0x120);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 22, 0x180);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        c.swc1(0, p(sp, 4));
        c.swc1(0, sp);
        let v = c.g[A2][0];
        c.sw(p(sp, 0xc0), v);
        // The ordinal digit table (0x24 bytes at 0x46ed40) to sp+0x10.
        c.g[T4][0] = 0x0046_ed40;
        let table = 0x0046_ed40u32;
        ld_d(c, A0, table);
        ld_d(c, A1, p(table, 8));
        ld_d(c, A2, p(table, 0x10));
        sd_d(c, A0, p(sp, 0x10));
        sd_d(c, A1, p(sp, 0x18));
        sd_d(c, A2, p(sp, 0x20));
        ld_d(c, A0, p(table, 0x18));
        c.g[A1][0] = c.lw(p(table, 0x20));
        sd_d(c, A0, p(sp, 0x28));
        let v = c.g[A1][0];
        c.sw(p(sp, 0x30), v);
        let v = c.g[T2][0];
        c.sw(p(sp, 0xc4), v);
        let v = c.g[T3][0];
        c.sw(p(sp, 0xc8), v);
        // A two-character string at sp+0x40 (from 0x4a22e8).
        c.g[A3][0] = 0x004a_22e8;
        c.g[V0][0] = c.lb(0x004a_22e8);
        c.g[A1][0] = c.lb(0x004a_22e9);
        let v = c.g[V0][0];
        c.sb(p(sp, 0x40), v);
        let v = c.g[A1][0];
        c.sb(p(sp, 0x41), v);
        let ordinal = c.g[FP][0] != 0;
        c.g[A0][0] = sx(p(sp, 0x10));
        if ordinal {
            c.cycles += 3;
            c.g[V0][0] = shl32(c.g[S3][0], 2);
            c.g[V0][0] = add32(c.g[A0][0], c.g[V0][0]);
            c.g[S5][0] = c.lb(r32(c, V0));
        } else {
            c.cycles += 5;
            c.g[V0][0] = shl32(c.g[S3][0], 1);
            c.g[V0][0] = add32(c.g[V0][0], 0x1e);
            c.g[V0][0] = shl32(c.g[V0][0], 24);
            c.g[S5][0] = sra32(c.g[V0][0], 24);
        }
        // Colour (1, k, 1, 1) at sp+0x80; mode 2 takes the element's.
        c.cycles += 12;
        fconst(c, 20, 0x3f80_0000);
        c.g[T0][0] = sx(p(sp, 0x80));
        ldf(c, 0, gp(c, -0x5358));
        c.g[T4][0] = 2;
        c.g[T2][0] = c.lw(p(sp, 0xc8));
        let v = c.g[T0][0];
        c.sw(p(sp, 0xcc), v);
        c.swc1(20, p(sp, 0x84));
        c.swc1(0, p(sp, 0x80));
        c.swc1(20, p(sp, 0x88));
        let mode2 = c.g[T2][0] == c.g[T4][0];
        c.swc1(20, p(sp, 0x8c));
        if mode2 {
            c.cycles += 5;
            c.g[A0][0] = 0x24;
            c.g[A1][0] = c.g[T0][0];
            c.mult(0, V0, S5, A0);
            c.g[A0][0] = add32(c.g[V0][0], c.g[S7][0]);
            call_ptr(c, callee::HUD_GET_ELEM_COLOR, 0x0021_ee84)?;
        }
        hud_draw_place(c, sp)?;
        // (false: the gauge code went straight to the epilogue, s0 restored)
        let tail = c.g[A0][0] == 0 || hud_draw_gauge(c, sp, frame)?;
        if tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x160);
        }
        c.cycles += 14;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

/// The place text: element s5 (0x24 bytes in s7) gives position and size,
/// aligned (`HUD_AlignElemPos`), scaled, drawn with `Font_DrawText`.
/// Leaves a0 = mode (sp+0xc8) and v1 = 0x24 as the original.
fn hud_draw_place(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 84;
    c.g[V1][0] = 0x24;
    c.g[V0][0] = c.lw(gp(c, -0xf28));
    c.mult(0, V1, S5, V1);
    c.g[S4][0] = sx(p(sp, 0x50));
    c.g[V0][0] = add32(c.g[V0][0], c.g[S3][0]);
    c.g[A1][0] = c.g[S4][0];
    c.g[S2][0] = add32(c.g[V1][0], c.g[S7][0]);
    c.g[V1][0] = c.lbu(r32(c, V0));
    c.g[A0][0] = c.g[S2][0];
    let v = c.g[V1][0];
    c.sb(p(sp, 0x40), v);
    call_ptr(c, callee::HUD_GET_ELEM_POS, 0x0021_eeb0)?;
    c.g[S3][0] = sx(p(sp, 0x60));
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    call_ptr(c, callee::HUD_GET_ELEM_SIZE, 0x0021_eec4)?;
    c.g[S6][0] = sx(p(sp, 0x70));
    let elem = r32(c, S2);
    c.g[T2][0] = c.lb(p(elem, 0x21));
    c.g[T1][0] = c.lb(p(elem, 0x20));
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[T0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[T3][0] = c.g[FP][0];
    call_ptr(c, callee::HUD_ALIGN_ELEM_POS, 0x0021_eeec)?;
    // scale *= k * size
    let scale_at = r32(c, S1);
    let sx_ = ldf(c, 1, scale_at);
    c.g[A3][0] = 0xffff_ffff_ffff_fc1f;
    let sy = ldf(c, 0, p(scale_at, 4));
    c.g[A1][0] = sx(p(sp, 0x40));
    let k = ldf(c, 3, gp(c, -0x5354));
    let h = ldf(c, 2, p(sp, 0x64));
    let w = ldf(c, 4, p(sp, 0x60));
    let ny = sy * k * h;
    setf(c, 0, ny);
    let nx = sx_ * k * w;
    setf(c, 1, nx);
    c.swc1(0, p(scale_at, 4));
    c.swc1(1, scale_at);
    // Draw row: bits 5..9 of the draw context flags = elem row + row.
    c.g[V1][0] = c.lw(gp(c, -0x854));
    c.g[V0][0] = c.lb(p(elem, 0x22));
    c.g[A0][0] = c.lw(at(c, V1, 0xe84));
    c.g[A2][0] = c.lw(p(sp, 0x190));
    c.g[V1][0] = c.lw(at(c, A0, 8));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[V1][0] &= c.g[A3][0];
    c.g[V0][0] &= 0x3e0;
    c.g[V1][0] |= c.g[V0][0];
    let v = c.g[V1][0];
    c.sw(at(c, A0, 8), v);
    // Text colour from sp+0x80.
    c.g[V0][0] = c.lw(p(r32(c, S0), hud::TEXT));
    let text = r32(c, V0);
    ld_d(c, T0, p(sp, 0x80));
    ld_d(c, T2, p(sp, 0x88));
    sd_d(c, T0, p(text, 0x40));
    sd_d(c, T2, p(text, 0x48));
    scale_text(c, scale_at, V1);
    c.g[A0][0] = c.lw(p(r32(c, S0), hud::TEXT));
    ldf(c, 12, p(sp, 0x70));
    ldf(c, 13, p(sp, 0x74));
    call_ptr(c, callee::FONT_DRAW_TEXT, 0x0021_efa4)?;
    // Back to unit text scale (sp+0).
    scale_text(c, sp, V0);
    c.g[A0][0] = c.lw(p(sp, 0xc8));
    c.g[V1][0] = 0x24;
    Ok(())
}

/// `t = hud->text (reg); t->scaled = t->size * scale` (x, y).
#[inline(always)]
fn scale_text(c: &mut Cpu, scale_at: u32, reg: usize) {
    c.g[reg][0] = c.lw(p(r32(c, S0), hud::TEXT));
    let text = r32(c, reg);
    let s0 = ldf(c, 1, scale_at);
    let w = ldf(c, 0, p(text, 0x30));
    let h = ldf(c, 2, p(text, 0x34));
    let sw = w * s0;
    setf(c, 0, sw);
    c.swc1(0, p(text, 0x38));
    let s1 = ldf(c, 1, p(scale_at, 4));
    let sh = h * s1;
    setf(c, 2, sh);
    c.swc1(2, p(text, 0x3c));
}

/// Modes other than 0: the place sprite (element s5 + 1), then in mode 2
/// the gauge panel when the rider's gauge (+0x64) is positive. `Ok(false)`
/// where the original goes straight to the epilogue with s0 restored.
fn hud_draw_gauge(c: &mut Cpu, sp: u32, frame: Frame) -> Result<bool, crate::call::Unwound> {
    c.cycles += 63;
    c.g[A1][0] = c.g[S4][0];
    c.lo[0] = c.g[V1][0];
    c.madd(0, V0, S5, V1);
    c.g[S1][0] = add32(c.g[S7][0], c.g[V0][0]);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, callee::HUD_GET_ELEM_POS, 0x0021_efec)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    call_ptr(c, callee::HUD_GET_ELEM_SIZE, 0x0021_effc)?;
    c.g[A0][0] = c.lw(p(sp, 0xc4));
    c.g[T0][0] = c.g[SP][0];
    let w = ldf(c, 1, p(sp, 0x60));
    c.g[A2][0] = c.g[S4][0];
    let scale2 = r32(c, A0);
    let ky = ldf(c, 3, p(scale2, 4));
    c.g[A3][0] = c.g[S3][0];
    let kx = ldf(c, 2, scale2);
    c.g[T3][0] = c.g[FP][0];
    let h = ldf(c, 0, p(sp, 0x64));
    c.g[A0][0] = c.g[S0][0];
    setf(c, 1, w * kx);
    c.g[A1][0] = c.g[S6][0];
    setf(c, 0, h * ky);
    c.swc1(1, p(sp, 0x60));
    c.swc1(0, p(sp, 0x64));
    let place_elem = r32(c, S2);
    c.g[T2][0] = c.lb(p(place_elem, 0x21));
    c.g[T1][0] = c.lb(p(place_elem, 0x20));
    call_ptr(c, callee::HUD_ALIGN_ELEM_POS, 0x0021_f044)?;
    c.g[A1][0] = c.lw(p(sp, 0xcc));
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, callee::HUD_GET_ELEM_COLOR, 0x0021_f050)?;
    c.g[V0][0] = c.lw(p(r32(c, S0), hud::TEXT));
    let text = r32(c, V0);
    c.g[A2][0] = c.g[S6][0];
    c.g[T1][0] = c.lw(p(sp, 0xcc));
    c.g[A3][0] = c.g[S3][0];
    ld_d(c, A1, p(sp, 0x80));
    ld_d(c, T0, p(sp, 0x88));
    sd_d(c, A1, p(text, 0x40));
    sd_d(c, T0, p(text, 0x48));
    c.g[A0][0] = c.g[S0][0];
    c.g[T2][0] = 0xffff_ffff_ffff_fc1f;
    c.g[T0][0] = c.g[SP][0];
    c.g[V1][0] = c.lw(gp(c, -0x854));
    c.g[V0][0] = c.lb(at(c, S1, 0x22));
    c.g[A1][0] = c.lw(at(c, V1, 0xe84));
    c.g[V1][0] = c.lw(p(sp, 0x190));
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.g[V1][0] = c.lw(at(c, A1, 8));
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[V0][0] &= 0x3e0;
    c.g[V1][0] &= c.g[T2][0];
    c.g[V1][0] |= c.g[V0][0];
    let v = c.g[V1][0];
    c.sw(at(c, A1, 8), v);
    c.g[A1][0] = c.lw(p(r32(c, S0), hud::SPRITE));
    call_ptr(c, callee::HUD_DRAW_SPRITE, 0x0021_f0c0)?;
    c.g[T4][0] = c.lw(p(sp, 0xc8));
    c.g[V0][0] = 2;
    if c.g[T4][0] != c.g[V0][0] {
        frame.restore(c, S0, 0x160);
        return Ok(false);
    }
    c.cycles += 7;
    c.g[V1][0] = c.lw(p(sp, 0xc0));
    setf(c, 0, FpuF32::ZERO);
    let gauge = ldf(c, 1, at(c, V1, 0x64));
    let positive = le(c, FpuF32::ZERO, gauge);
    c.g[A1][0] = c.lw(p(sp, 0x190));
    if !positive {
        return Ok(true);
    }
    hud_draw_gauge_panel(c, sp, frame)
}

/// The gauge panel: row - 1, the rider's gauge rectangle (+0x54) widened
/// by the gauge (doubled past 1), then one `HUD_DrawPanelSegment` when it
/// is at most 72 wide, else three (36-wide ends and the middle).
fn hud_draw_gauge_panel(c: &mut Cpu, sp: u32, frame: Frame) -> Result<bool, crate::call::Unwound> {
    c.cycles += 36;
    c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = c.lw(gp(c, -0x854));
    c.g[A0][0] = sx(r32(c, A1).wrapping_sub(1));
    c.g[V0][0] = c.lb(at(c, S1, 0x22));
    c.g[A1][0] = c.lw(at(c, V1, 0xe84));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    let base = ldf(c, 4, gp(c, -0x5350));
    c.g[V1][0] = c.lw(at(c, A1, 8));
    c.g[V0][0] = shl32(c.g[V0][0], 5);
    c.g[V0][0] &= 0x3e0;
    let k = ldf(c, 3, gp(c, -0x534c));
    c.g[V1][0] &= c.g[A2][0];
    c.g[V1][0] |= c.g[V0][0];
    let v = c.g[V1][0];
    c.sw(at(c, A1, 8), v);
    c.g[A3][0] = c.lw(p(sp, 0xc0));
    let rider = r32(c, A3);
    ld_d(c, T0, p(rider, 0x54));
    ld_d(c, T2, p(rider, 0x5c));
    sd_d(c, T0, p(sp, 0x90));
    sd_d(c, T2, p(sp, 0x98));
    let gauge = ldf(c, 1, p(rider, 0x64));
    ld_d(c, V1, p(sp, 0x70));
    sd_d(c, V1, p(sp, 0xa0));
    ld_d(c, A3, p(sp, 0x60));
    sd_d(c, A3, p(sp, 0xb0));
    let over = lt(c, fr(c, 20), gauge);
    let rect_w = ldf(c, 2, p(sp, 0x90));
    let t = if over {
        c.cycles += 5;
        let two = fconst(c, 0, 0x4000_0000);
        let d = two - gauge;
        setf(c, 0, d);
        d * k
    } else {
        c.cycles += 1;
        gauge * k
    };
    setf(c, 0, t);
    // Rectangle at sp+0x90 (w), position sp+0xa0, size sp+0xb0.
    c.cycles += 33;
    let t = t + base;
    setf(c, 0, t);
    let w = rect_w * t;
    setf(c, 5, w);
    let sy = ldf(c, 4, p(sp, 0xb4));
    let m = ldf(c, 0, gp(c, -0x5348));
    let sxz = ldf(c, 3, p(sp, 0xb0));
    let gy = sy * m;
    setf(c, 7, gy);
    let half = fconst(c, 8, 0x3f00_0000);
    let gx = sxz * m;
    setf(c, 0, gx);
    c.swc1(5, p(sp, 0x90));
    let px = ldf(c, 2, p(sp, 0xa0));
    let hy = gy * half;
    setf(c, 6, hy);
    let py = ldf(c, 1, p(sp, 0xa4));
    let hx = gx * half;
    setf(c, 5, hx);
    c.g[V1][0] = c.lw(at(c, S0, hud::PANEL_FLAGS));
    setf(c, 3, sxz + gx);
    c.g[A0][0] = c.lw(gp(c, -0x854));
    setf(c, 1, py - hy);
    let end_w = fconst(c, 22, 0x4290_0000);
    setf(c, 2, px - hx);
    setf(c, 4, sy + gy);
    c.swc1(3, p(sp, 0xb0));
    c.swc1(1, p(sp, 0xa4));
    c.swc1(2, p(sp, 0xa0));
    c.swc1(4, p(sp, 0xb4));
    c.g[V0][0] = c.lw(at(c, A0, 0xe84));
    let v = c.g[V1][0];
    c.sh(at(c, V0, 0x10), v);
    let width = ldf(c, 12, p(sp, 0xb0));
    let wide = lt(c, end_w, width);
    c.g[S1][0] = sx(p(sp, 0xa0));
    let hud_at = r32(c, S0);
    if !wide {
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        ldf(c, 13, p(sp, 0xb4));
        ldf(c, 14, p(hud_at, hud::PANEL_TOP));
        c.g[A2][0] = c.g[S1][0];
        ldf(c, 15, p(hud_at, hud::PANEL_X));
        c.g[A1][0] = sx(p(sp, 0x90));
        ldf(c, 16, p(hud_at, hud::PANEL_BOTTOM));
        ldf(c, 17, p(hud_at, hud::PANEL_DEPTH));
        callee::HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_f2f8)?;
        return Ok(true);
    }
    // Three segments: left end, right end, middle.
    c.cycles += 47;
    let top = ldf(c, 20, p(hud_at, hud::PANEL_TOP));
    c.g[A1][0] = sx(p(sp, 0x90));
    let bottom = ldf(c, 0, p(hud_at, hud::PANEL_BOTTOM));
    c.g[A0][0] = c.g[S0][0];
    c.mov_s(14, 20);
    let end = fconst(c, 21, 0x4210_0000);
    let mid = (top + bottom) * half;
    ldf(c, 13, p(sp, 0xb4));
    ldf(c, 15, p(hud_at, hud::PANEL_X));
    setf(c, 12, end);
    ldf(c, 17, p(hud_at, hud::PANEL_DEPTH));
    c.g[A2][0] = c.g[S1][0];
    setf(c, 20, mid);
    setf(c, 16, mid);
    callee::HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_f258)?;
    let x = ldf(c, 0, p(sp, 0xa0));
    c.g[A0][0] = c.g[S0][0];
    let width = ldf(c, 12, p(sp, 0xb0));
    c.g[A1][0] = sx(p(sp, 0x90));
    let x = x + fr(c, 21);
    setf(c, 0, x);
    ldf(c, 15, p(hud_at, hud::PANEL_X));
    setf(c, 12, width - fr(c, 22));
    ldf(c, 17, p(hud_at, hud::PANEL_DEPTH));
    ldf(c, 13, p(sp, 0xb4));
    c.g[A2][0] = c.g[S1][0];
    c.swc1(0, p(sp, 0xa0));
    c.mov_s(14, 20);
    c.mov_s(16, 20);
    callee::HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_f290)?;
    let width = ldf(c, 1, p(sp, 0xb0));
    c.g[A0][0] = c.g[S0][0];
    let x = ldf(c, 0, p(sp, 0xa0));
    c.g[A2][0] = c.g[S1][0];
    let rest = width - fr(c, 22);
    setf(c, 1, rest);
    ldf(c, 13, p(sp, 0xb4));
    c.mov_s(12, 21);
    c.mov_s(14, 20);
    c.g[A1][0] = sx(p(sp, 0x90));
    setf(c, 0, x + rest);
    c.swc1(0, p(sp, 0xa0));
    let hud_at = r32(c, A0);
    ldf(c, 15, p(hud_at, hud::PANEL_X));
    ldf(c, 16, p(hud_at, hud::PANEL_BOTTOM));
    ldf(c, 17, p(hud_at, hud::PANEL_DEPTH));
    callee::HUD_DRAW_PANEL_SEGMENT.call(c, 0x0021_f2cc)?;
    frame.restore(c, S0, 0x160);
    Ok(false)
}

// ---------------------------------------------------------------------------
// Spatial hash.

const SH_ENTRY: u32 = 0x0032_91e0;
const SH_END: u32 = 0x0032_958c;
/// Resume entries (event checks in the bucket-emptiness scans): "not
/// empty" and the scan of the last three words, after a cell change and
/// after a re-insert in the same root.
const SH_MOVED_NONEMPTY: u32 = 0x0032_9358;
const SH_MOVED_TAIL: u32 = 0x0032_9368;
const SH_SAME_NONEMPTY: u32 = 0x0032_94f0;
const SH_SAME_TAIL: u32 = 0x0032_9500;

/// Where a bucket-emptiness scan starts (or resumes).
#[derive(Clone, Copy, PartialEq)]
enum Probe {
    /// The eight unrolled words (right after `SpatialHash_Remove`).
    Words,
    /// A non-zero word was found.
    NonEmpty,
    /// The loop over the last three words (a0 counter, v1 pointer).
    Tail,
}

/// `void SpatialHash_Update(SpatialHash* h, void* obj, void* user,
/// const AABB* box /*a3*/, const AABB* old_box /*t0*/)` (0x3291e0): the
/// keys of the old (sp+0) and new (sp+0x10) boxes; nothing when equal.
/// When the root octant (key coordinate signs) changes: remove under the
/// old root, free its bucket array if now empty, insert under the new
/// one. Otherwise, if the root's cell still holds the new key (same level
/// and coordinates, or a coarser level whose shifted coordinates match):
/// `SpatialHash_Move`; else remove, free if empty, insert.
/// Dispatcher-shaped in the original: re-entered at the scan loop heads
/// after an unwind.
pub fn spatial_hash_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xb0 };
        match c.pc {
            SH_ENTRY => spatial_hash_update_entry(c, frame),
            SH_MOVED_NONEMPTY => sh_moved_root(c, frame, Probe::NonEmpty),
            SH_MOVED_TAIL => sh_moved_root(c, frame, Probe::Tail),
            SH_SAME_NONEMPTY => sh_reinsert(c, frame, Probe::NonEmpty),
            SH_SAME_TAIL => sh_reinsert(c, frame, Probe::Tail),
            pc => {
                c.jump_out(pc, SH_ENTRY, SH_END);
                Ok(())
            }
        }
    })
}

fn spatial_hash_update_entry(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    Frame::open(c, 0xb0);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S0, 0xa0);
    c.g[S5][0] = c.g[A1][0];
    frame.save(c, S6, 0x40);
    c.g[S0][0] = c.g[A3][0];
    frame.save(c, S7, 0x30);
    c.g[A1][0] = c.g[T0][0];
    c.g[A0][0] = c.g[SP][0];
    frame.save(c, S1, 0x90);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save_ra(c, 0x20);
    c.g[S6][0] = c.g[A2][0];
    // old key at sp+0, new key at sp+0x10.
    callee::SPATIAL_KEY_FROM_AABB.call(c, 0x0032_9224)?;
    let sp = sp(c);
    c.g[A0][0] = sx(p(sp, 0x10));
    c.g[A1][0] = c.g[S0][0];
    c.g[S7][0] = c.g[A0][0];
    callee::SPATIAL_KEY_FROM_AABB.call(c, 0x0032_9234)?;
    // Same key?
    c.g[V1][0] = c.lw(p(sp, 0x10));
    c.g[V0][0] = c.lw(sp);
    let differ = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = 0;
    if !differ {
        c.cycles += 4;
        c.g[V1][0] = c.lw(p(sp, 0x14));
        c.g[V0][0] = c.lw(p(sp, 4));
        let differ = c.g[V1][0] != c.g[V0][0];
        c.g[V1][0] = c.lw(p(sp, 0x18));
        if !differ {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(sp, 8));
            let differ = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw(p(sp, 0x1c));
            if !differ {
                c.cycles += 3;
                c.g[V1][0] = c.lw(p(sp, 0xc));
                c.g[V0][0] ^= c.g[V1][0];
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
    }
    c.cycles += 2;
    frame.restore(c, S0, 0xa0);
    if c.g[A0][0] != 0 {
        sh_exit(c, frame);
        return Ok(());
    }
    // Root octants (coordinate signs): s2/s1/s3 new key, v0/a1/a2 old.
    c.cycles += 13;
    c.g[T0][0] = c.lw(p(sp, 0x14));
    c.g[T1][0] = c.lw(p(sp, 0x18));
    c.g[T2][0] = c.lw(p(sp, 0x1c));
    c.g[S2][0] = srl32(c.g[T0][0], 31);
    c.g[V0][0] = c.lw(p(sp, 4));
    c.g[S1][0] = srl32(c.g[T1][0], 31);
    c.g[A0][0] = c.lw(p(sp, 8));
    c.g[S3][0] = srl32(c.g[T2][0], 31);
    c.g[V1][0] = c.lw(p(sp, 0xc));
    c.g[V0][0] = srl32(c.g[V0][0], 31);
    c.g[A1][0] = srl32(c.g[A0][0], 31);
    c.g[A2][0] = srl32(c.g[V1][0], 31);
    let moved = if c.g[S2][0] != c.g[V0][0] {
        c.cycles += 1;
        c.g[V1][0] = 0x28;
        true
    } else {
        c.cycles += 2;
        let differ = c.g[S1][0] != c.g[A1][0];
        c.g[V1][0] = 0x28;
        if differ {
            true
        } else {
            c.cycles += 2;
            if c.g[S3][0] == c.g[A2][0] {
                c.g[V0][0] = 0x50;
                false
            } else {
                c.cycles += 1;
                c.g[V1][0] = 0x28;
                true
            }
        }
    };
    if moved {
        sh_moved_root(c, frame, Probe::Words)
    } else {
        sh_same_root(c, frame)
    }
}

/// The root octant changed: remove under the old key's root, drop its
/// bucket array when empty, insert under the new key's root.
fn sh_moved_root(c: &mut Cpu, frame: Frame, probe: Probe) -> Flow {
    if probe == Probe::Words {
        c.cycles += 18;
        c.g[A0][0] = 0x50;
        c.mult(0, V1, A1, V1);
        c.g[A3][0] = c.g[SP][0];
        c.mult(0, A1, V0, A0);
        c.g[V0][0] = 0x14;
        c.mult(0, V0, A2, V0);
        c.g[A2][0] = c.g[S6][0];
        c.g[A0][0] = add32(c.g[A1][0], c.g[S4][0]);
        c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[A1][0] = c.g[S5][0];
        c.g[S0][0] = add32(c.g[V0][0], c.g[V1][0]);
        c.g[A0][0] = c.lw(at(c, S0, 0x10));
        c.g[T0][0] = c.g[S0][0];
        callee::SPATIAL_HASH_REMOVE.call(c, 0x0032_92f4)?;
    }
    if !bucket_empty(c, probe, SH_MOVED_NONEMPTY, SH_MOVED_TAIL) {
        return Ok(());
    }
    c.cycles += 2;
    let empty = c.g[V0][0] != 0;
    c.g[V0][0] = 0x50;
    if empty {
        c.cycles += 4;
        c.g[A0][0] = c.lw(at(c, S0, 0x10));
        callee::DELETE.call(c, 0x0032_9394)?;
        c.sw(at(c, S0, 0x10), 0);
        c.g[V0][0] = 0x50;
    }
    c.cycles += 14;
    c.g[V1][0] = 0x28;
    c.mult(0, A0, S2, V0);
    c.mult(0, V1, S1, V1);
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S6][0];
    c.g[A3][0] = c.g[S7][0];
    c.g[V0][0] = add32(c.g[A0][0], c.g[S4][0]);
    c.g[A0][0] = 0x14;
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    c.mult(0, A0, S3, A0);
    c.g[A0][0] = add32(c.g[V0][0], c.g[A0][0]);
    callee::SPATIAL_HASH_INSERT.call(c, 0x0032_93cc)?;
    frame.restore(c, S0, 0xa0);
    sh_exit(c, frame);
    Ok(())
}

/// Same root octant: does the root's cell (level, x, y, z) still hold the
/// new key? Then move, else remove and re-insert.
fn sh_same_root(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = sp(c);
    c.cycles += 13;
    c.g[A0][0] = 0x28;
    c.mult(0, V1, S2, V0);
    c.mult(0, A0, S1, A0);
    c.g[A1][0] = c.lw(p(sp, 0x10));
    c.g[V0][0] = add32(c.g[V1][0], c.g[S4][0]);
    c.g[V1][0] = 0x14;
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    c.mult(0, V1, S3, V1);
    c.g[S0][0] = add32(c.g[V1][0], c.g[A0][0]);
    let cell = r32(c, S0);
    c.g[V0][0] = c.lw(cell);
    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A1)));
    let coarser_or_same = (c.g[A0][0] as i64) >= 0;
    c.g[A3][0] = c.g[SP][0];
    if !coarser_or_same {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            // A coarser cell: compare the shifted coordinates.
            c.g[V1][0] = c.lw(p(cell, 4));
            c.cycles += 3;
            let shift = r32(c, A0) & 31;
            c.g[V0][0] = sra32(c.g[T0][0], shift);
            let differ = c.g[V0][0] != c.g[V1][0];
            c.g[A1][0] = 0;
            if !differ {
                c.cycles += 4;
                c.g[V1][0] = c.lw(p(cell, 8));
                c.g[V0][0] = sra32(c.g[T1][0], shift);
                let differ = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[A1][0];
                if !differ {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(p(cell, 0xc));
                    c.g[V0][0] = sra32(c.g[T2][0], shift);
                    c.g[V0][0] ^= c.g[V1][0];
                    c.g[A1][0] = (c.g[V0][0] < 1) as u64;
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A1][0];
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
            }
        } else {
            // The same level: compare the coordinates.
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(cell, 4));
            let differ = c.g[T0][0] != c.g[V0][0];
            c.g[V1][0] = 0;
            if differ {
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(cell, 8));
                if c.g[T1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.g[V1][0];
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(p(cell, 0xc));
                    c.g[V0][0] ^= c.g[T2][0];
                    c.g[V1][0] = (c.g[V0][0] < 1) as u64;
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                }
            }
        }
    }
    c.cycles += 2;
    let contained = c.g[V0][0] != 0;
    c.g[A0][0] = c.lw(p(cell, 0x10));
    if contained {
        c.cycles += 6;
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S6][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T0][0] = sx(p(sp, 0x10));
        callee::SPATIAL_HASH_MOVE.call(c, 0x0032_9560)?;
        frame.restore(c, S0, 0xa0);
        sh_exit(c, frame);
        return Ok(());
    }
    sh_reinsert(c, frame, Probe::Words)
}

/// Same root, key no longer held: remove, drop the bucket array when
/// empty, insert again.
fn sh_reinsert(c: &mut Cpu, frame: Frame, probe: Probe) -> Flow {
    if probe == Probe::Words {
        c.cycles += 8;
        c.g[A1][0] = c.g[S5][0];
        c.g[A2][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S0][0];
        callee::SPATIAL_HASH_REMOVE.call(c, 0x0032_948c)?;
    }
    if !bucket_empty(c, probe, SH_SAME_NONEMPTY, SH_SAME_TAIL) {
        return Ok(());
    }
    c.cycles += 2;
    let empty = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if empty {
        c.cycles += 4;
        c.g[A0][0] = c.lw(at(c, S0, 0x10));
        callee::DELETE.call(c, 0x0032_952c)?;
        c.sw(at(c, S0, 0x10), 0);
        c.g[A0][0] = c.g[S0][0];
    }
    c.cycles += 6;
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S6][0];
    c.g[A3][0] = sx(p(sp(c), 0x10));
    callee::SPATIAL_HASH_INSERT.call(c, 0x0032_9544)?;
    frame.restore(c, S0, 0xa0);
    sh_exit(c, frame);
    Ok(())
}

/// Is the bucket array of cell s0 (`+0x10`, eleven words) all zero?
/// Leaves v0 = 1 if so, else 0 (and v1/a0 as the scan leaves them). False
/// when an event at a loop head asked the thread to unwind (pc there).
fn bucket_empty(c: &mut Cpu, mut probe: Probe, nonempty_head: u32, tail_head: u32) -> bool {
    loop {
        match probe {
            Probe::Words => {
                c.g[V1][0] = c.lw(at(c, S0, 0x10));
                let words = r32(c, V1);
                for k in 0..7u32 {
                    if k > 0 {
                        c.cycles += 3;
                    }
                    c.g[V0][0] = c.lw(p(words, 4 * k));
                    let set = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if set {
                        return true;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(words, 0x1c));
                let clear = c.g[V0][0] == 0;
                c.g[V1][0] = sx(p(words, 0x20));
                if !clear {
                    probe = Probe::NonEmpty;
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = 0;
                probe = Probe::Tail;
            }
            Probe::NonEmpty => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                return true;
            }
            Probe::Tail => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                let set = c.g[V0][0] != 0;
                c.g[A0][0] = add32(c.g[A0][0], 1);
                if set {
                    if back_edge_unwinds(c, nonempty_head) {
                        return false;
                    }
                    probe = Probe::NonEmpty;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = add32(c.g[V1][0], 4);
                if more {
                    if back_edge_unwinds(c, tail_head) {
                        return false;
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                return true;
            }
        }
    }
}

/// The epilogue: s1..s7 and ra back, `sp += 0xb0` (s0 restored before).
fn sh_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Irradiance.

const IR_ENTRY: u32 = 0x0038_a6a8;
const IR_END: u32 = 0x0038_abf8;
/// Heads of the two integer-power loops (resume entries).
const IR_POW_OUTER: u32 = 0x0038_a900;
const IR_POW_INNER: u32 = 0x0038_aa20;
/// `switch (exponent)` jump tables (.rodata) of the two falloff branches.
const IR_TABLE_OUTER: u32 = 0x0049_2e30;
const IR_TABLE_INNER: u32 = 0x0049_2e50;

/// Dynamic light fields.
mod dyn_light {
    pub const KIND: u32 = 0x10;
    pub const INTENSITY: u32 = 0x14;
    pub const RANGE: u32 = 0x1c;
    pub const COLOUR: u32 = 0x20;
    /// Cone/radius falloff exponent kind (`lb`): 1 linear, 2 square, 3 cube.
    pub const SHAPE: u32 = 0x64;
    /// Distance falloff exponent (`lb`).
    pub const EXPONENT: u32 = 0x65;
    pub const INNER: u32 = 0x5c;
    pub const OUTER: u32 = 0x60;
}

/// `void Irradiance_AddDynamicLight(Irradiance* irr, const Vec3* pos,
/// const DynLight* light)` (0x38a6a8): spot (kind 1) and point (kind 2)
/// lights as a directional contribution (`Irradiance_AddDirectional`)
/// scaled by the angular and distance falloffs (integer powers through a
/// `switch` on the exponent, or a square-and-multiply loop), clamped to
/// [0, 5]. Dispatcher-shaped in the original: re-entered at the power
/// loops' heads after an unwind.
pub fn irradiance_add_dynamic_light(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        match c.pc {
            IR_ENTRY => ir_entry(c, frame),
            IR_POW_OUTER => {
                if !pow_loop(c, 1, IR_POW_OUTER) {
                    return Ok(());
                }
                ir_outer_done(c, frame)
            }
            IR_POW_INNER => {
                if !pow_loop(c, 3, IR_POW_INNER) {
                    return Ok(());
                }
                ir_inner_done(c, frame)
            }
            pc => {
                c.jump_out(pc, IR_ENTRY, IR_END);
                Ok(())
            }
        }
    })
}

fn ir_entry(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 10;
    Frame::open(c, 0x60);
    c.g[V0][0] = 1;
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = c.g[A2][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = c.g[A0][0];
    c.g[V1][0] = c.lw(at(c, S0, dyn_light::KIND));
    let spot = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = c.g[A1][0];
    if spot {
        return ir_spot(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x50);
        ir_exit(c, frame);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 2;
    let point = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = c.g[SP][0];
    if point {
        return ir_point(c, frame);
    }
    c.cycles += 2;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
    Ok(())
}

/// The epilogue (s0 restored before).
fn ir_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// No contribution: `lq s0` and out.
fn ir_abort(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
}

/// The cone/range term into f2 (1 when the range is below a threshold or
/// the angle term within it, else `1 - (angle - threshold) * k`), then
/// f3 = the second vector term * 100, f1 = 1, v1 = the shape byte; true
/// when f3 < 1 (c.lt.s).
fn ir_cone(c: &mut Cpu, k_range: i32, k_angle: i32, k_slope: i32) -> bool {
    let light = r32(c, S0);
    let sp = sp(c);
    c.cycles += 6;
    let range = ldf(c, 1, p(light, dyn_light::RANGE));
    let min_range = ldf(c, 0, gp(c, k_range));
    let short = lt(c, range, min_range);
    let angle = ldf(c, 2, p(sp, 0x20));
    let mut excess = None;
    if !short {
        c.cycles += 5;
        let start = ldf(c, 0, gp(c, k_angle));
        if !le(c, angle, start) {
            let e = angle - start;
            setf(c, 0, e);
            excess = Some(e);
        }
    }
    match excess {
        None => {
            c.cycles += 4;
            fconst(c, 2, 0x3f80_0000);
        }
        Some(e) => {
            c.cycles += 6;
            let slope = ldf(c, 1, gp(c, k_slope));
            let one = fconst(c, 2, 0x3f80_0000);
            let d = e * slope;
            setf(c, 0, d);
            setf(c, 2, one - d);
        }
    }
    let t = ldf(c, 3, p(sp, 0x24));
    c.cycles += 9;
    let hundred = fconst(c, 0, 0x42c8_0000);
    let one = fconst(c, 1, 0x3f80_0000);
    let t = t * hundred;
    setf(c, 3, t);
    let small = lt(c, t, one);
    c.g[V1][0] = c.lb(p(light, dyn_light::SHAPE));
    small
}

/// `v1 = exponent byte; v0 = v1 < 5 (unsigned); v0 = 0x490000`: the
/// switch head. Some(target) for a table case.
fn ir_switch(c: &mut Cpu, table: u32) -> Option<u32> {
    c.cycles += 4;
    c.g[V1][0] = c.lb(at(c, S0, dyn_light::EXPONENT));
    c.g[V0][0] = (c.g[V1][0] < 5) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0049_0000;
    if !in_table {
        return None;
    }
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = sx(table);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = c.lw(r32(c, V1));
    Some(r32(c, A0))
}

/// f2 = colour.z (+0x28), a2 = sp+0x10.
#[inline(always)]
fn ir_colour_z(c: &mut Cpu) {
    ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
    c.g[A2][0] = sx(p(sp(c), 0x10));
}

/// f6, f4 = colour.x, colour.y.
#[inline(always)]
fn ir_colour_xy(c: &mut Cpu) {
    ldf(c, 6, at(c, S0, dyn_light::COLOUR));
    ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
}

/// Square-and-multiply `f[acc] *= f0^v1` from a loop head (v1 > 0). False
/// when an event at the head asked to unwind.
fn pow_loop(c: &mut Cpu, acc: usize, head: u32) -> bool {
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 1;
        if c.g[V0][0] != 0 {
            setf(c, acc, fr(c, acc) * fr(c, 0));
        }
        c.cycles += 3;
        c.g[V1][0] = sra32(c.g[V1][0], 1);
        let more = c.g[V1][0] != 0;
        setf(c, 0, fr(c, 0) * fr(c, 0));
        if !more {
            return true;
        }
        if back_edge_unwinds(c, head) {
            return false;
        }
    }
}

/// Spot light: direction/attenuation vector, the cone term, the shape
/// power, then the distance falloff on one side of the inner radius.
fn ir_spot(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = sp(c);
    c.cycles += 8;
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sx(p(sp, 0x20));
    c.g[T0][0] = sx(p(sp, 0x24));
    c.g[T1][0] = sx(p(sp, 0x28));
    callee::SPOT_LIGHT_VECTOR.call(c, 0x0038_a708)?;
    let lit = c.g[V0][0] != 0;
    let dist = ldf(c, 1, p(sp, 0x28));
    if !lit {
        ir_abort(c, frame);
        return Ok(());
    }
    c.cycles += 6;
    let light = r32(c, S0);
    let outer = ldf(c, 0, p(light, dyn_light::OUTER));
    setf(c, 7, dist);
    let inside = le(c, outer, dist);
    setf(c, 8, outer);
    if !inside {
        ir_abort(c, frame);
        return Ok(());
    }
    // Shape power of the cone term into f1 (1 by default).
    let small = ir_cone(c, -0x26e4, -0x26e0, -0x26dc);
    let mut loaded = false;
    if small {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let square = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        if square {
            c.cycles += 2;
            setf(c, 1, fr(c, 3) * fr(c, 3));
        } else {
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if high {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if c.g[V1][0] == c.g[V0][0] {
                    let sq = fr(c, 3) * fr(c, 3);
                    setf(c, 0, sq);
                    c.cycles += 2;
                    setf(c, 1, sq * fr(c, 3));
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    fconst(c, 1, 0x3f80_0000);
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    c.mov_s(1, 3);
                } else {
                    c.cycles += 2;
                    ldf(c, 0, p(light, dyn_light::INTENSITY));
                    loaded = true;
                }
            }
        }
    }
    if !loaded {
        c.cycles += 1;
        ldf(c, 0, p(light, dyn_light::INTENSITY));
    }
    // f5 = shape * intensity * cone; which side of the inner radius?
    c.cycles += 6;
    let inner = ldf(c, 3, p(light, dyn_light::INNER));
    let scaled = fr(c, 1) * fr(c, 0);
    setf(c, 0, scaled);
    let beyond = le(c, inner, fr(c, 7));
    setf(c, 1, inner);
    setf(c, 5, scaled * fr(c, 2));
    if beyond {
        ir_spot_outer(c, frame)
    } else {
        ir_spot_inner(c, frame)
    }
}

/// Beyond the inner radius: f1 = dist^exponent, f5 *= f1.
fn ir_spot_outer(c: &mut Cpu, frame: Frame) -> Flow {
    match ir_switch(c, IR_TABLE_OUTER) {
        Some(0x0038_a834) => {
            c.cycles += 5;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            fconst(c, 1, 0x3f80_0000);
            c.g[A2][0] = sx(p(sp(c), 0x10));
            c.cycles += 3;
            ir_colour_xy(c);
        }
        Some(t @ (0x0038_a848 | 0x0038_a850)) => {
            if t == 0x0038_a848 {
                c.cycles += 2;
                c.mov_s(1, 7);
            } else {
                c.cycles += 1;
                setf(c, 1, fr(c, 7) * fr(c, 7));
            }
            c.cycles += 5;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
        }
        Some(t @ (0x0038_a868 | 0x0038_a884)) => {
            c.cycles += if t == 0x0038_a868 { 7 } else { 8 };
            let d = fr(c, 7);
            let mut v = d * d;
            setf(c, 0, v);
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if t == 0x0038_a884 {
                v = v * d;
                setf(c, 0, v);
            }
            setf(c, 1, v * d);
        }
        Some(t) => {
            c.jump_out(t, IR_ENTRY, IR_END);
            return Ok(());
        }
        None => {
            // General exponent: 0 when dist == 0, else dist^n (1/dist for
            // negative n) by square-and-multiply.
            c.cycles += 5;
            setf(c, 1, FpuF32::ZERO);
            let zero = eq(c, fr(c, 7), FpuF32::ZERO);
            c.mov_s(0, 7);
            if zero {
                c.cycles += 2;
                ir_colour_z(c);
                c.cycles += 3;
                ir_colour_xy(c);
            } else {
                c.cycles += 2;
                let negative = (c.g[V1][0] as i64) < 0;
                ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
                if negative {
                    c.cycles += 4;
                    let one = fconst(c, 0, 0x3f80_0000);
                    c.g[V1][0] = sx(0u32.wrapping_sub(r32(c, V1)));
                    setf(c, 0, one.div(fr(c, 7)));
                }
                c.cycles += 6;
                c.g[A2][0] = sx(p(sp(c), 0x10));
                fconst(c, 1, 0x3f80_0000);
                ldf(c, 6, at(c, S0, dyn_light::COLOUR));
                let done = c.g[V1][0] == 0;
                ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
                if !done {
                    c.cycles += 1;
                    if !pow_loop(c, 1, IR_POW_OUTER) {
                        return Ok(());
                    }
                }
            }
        }
    }
    ir_outer_done(c, frame)
}

/// `f5 *= f1`, then the clamp and the contribution.
fn ir_outer_done(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    setf(c, 5, fr(c, 5) * fr(c, 1));
    ir_contribute(c, frame)
}

/// Within the inner radius: f3 = inner^exponent, then f5 *= f3 * (dist -
/// outer) / (inner - outer).
fn ir_spot_inner(c: &mut Cpu, frame: Frame) -> Flow {
    match ir_switch(c, IR_TABLE_INNER) {
        Some(t @ (0x0038_a948 | 0x0038_a95c)) => {
            if t == 0x0038_a948 {
                c.cycles += 5;
                ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
                fconst(c, 3, 0x3f80_0000);
                c.g[A2][0] = sx(p(sp(c), 0x10));
            } else {
                c.cycles += 2;
                ir_colour_z(c);
            }
            c.cycles += 3;
            ir_colour_xy(c);
        }
        Some(t @ (0x0038_a970 | 0x0038_a988 | 0x0038_a9a4)) => {
            let r = fr(c, 3);
            c.cycles += match t {
                0x0038_a970 => 6,
                0x0038_a988 => 7,
                _ => 8,
            };
            if t == 0x0038_a970 {
                setf(c, 3, r * r);
            } else {
                setf(c, 0, r * r);
            }
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            c.g[A2][0] = sx(p(sp(c), 0x10));
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if t == 0x0038_a9a4 {
                setf(c, 0, fr(c, 0) * r);
            }
            if t != 0x0038_a970 {
                setf(c, 3, fr(c, 0) * r);
            }
        }
        Some(t) => {
            c.jump_out(t, IR_ENTRY, IR_END);
            return Ok(());
        }
        None => {
            c.cycles += 5;
            setf(c, 2, FpuF32::ZERO);
            let zero = eq(c, fr(c, 1), FpuF32::ZERO);
            c.mov_s(0, 1);
            if zero {
                c.cycles += 6;
                c.mov_s(0, 2);
                ldf(c, 6, at(c, S0, dyn_light::COLOUR));
                ir_colour_z(c);
                ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
                c.cycles += 1;
                c.mov_s(3, 0);
                return ir_falloff(c, frame);
            }
            c.cycles += 2;
            let negative = (c.g[V1][0] as i64) < 0;
            ldf(c, 2, at(c, S0, dyn_light::COLOUR + 8));
            if negative {
                c.cycles += 4;
                let one = fconst(c, 0, 0x3f80_0000);
                c.g[V1][0] = sx(0u32.wrapping_sub(r32(c, V1)));
                setf(c, 0, one.div(fr(c, 1)));
            }
            c.cycles += 6;
            c.g[A2][0] = sx(p(sp(c), 0x10));
            fconst(c, 3, 0x3f80_0000);
            ldf(c, 6, at(c, S0, dyn_light::COLOUR));
            let done = c.g[V1][0] == 0;
            ldf(c, 4, at(c, S0, dyn_light::COLOUR + 4));
            if !done && !pow_loop(c, 3, IR_POW_INNER) {
                return Ok(());
            }
            return ir_inner_done(c, frame);
        }
    }
    ir_falloff(c, frame)
}

/// After the inner power loop: `f0 = f3; f3 = f0`, then the falloff.
fn ir_inner_done(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.mov_s(0, 3);
    c.cycles += 1;
    c.mov_s(3, 0);
    ir_falloff(c, frame)
}

/// `f5 *= f3 * (dist - outer) / (inner - outer)`.
fn ir_falloff(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    let span = fr(c, 1) - fr(c, 8);
    setf(c, 1, span);
    let d = fr(c, 7) - fr(c, 8);
    setf(c, 0, d);
    let f5 = fr(c, 5) * fr(c, 3);
    let ratio = d.div(span);
    setf(c, 0, ratio);
    setf(c, 5, f5 * ratio);
    ir_contribute(c, frame)
}

/// Clamp f5 to [0, 5] and add the colour scaled by it as a directional
/// light (direction at sp+0).
fn ir_contribute(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    setf(c, 0, FpuF32::ZERO);
    let five = fconst(c, 1, 0x40a0_0000);
    let k = fr(c, 5);
    if le(c, FpuF32::ZERO, k) {
        setf(c, 0, k.min(five));
    }
    c.cycles += 12;
    let k = fr(c, 0);
    setf(c, 5, k);
    c.g[A0][0] = c.g[S1][0];
    let sp = sp(c);
    setf(c, 2, fr(c, 2) * k);
    c.g[A1][0] = c.g[SP][0];
    setf(c, 1, fr(c, 6) * k);
    setf(c, 0, fr(c, 4) * k);
    c.swc1(2, p(sp, 0x18));
    c.swc1(1, p(sp, 0x10));
    c.swc1(0, p(sp, 0x14));
    callee::ADD_DIRECTIONAL.call(c, 0x0038_aa98)?;
    frame.restore(c, S0, 0x50);
    ir_exit(c, frame);
    Ok(())
}

/// Point light: vector, the cone/range term, the shape power, clamp to
/// [0, 5], contribution.
fn ir_point(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = sp(c);
    c.cycles += 6;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = sx(p(sp, 0x20));
    c.g[T0][0] = sx(p(sp, 0x24));
    callee::POINT_LIGHT_VECTOR.call(c, 0x0038_aab0)?;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x50);
        ir_exit(c, frame);
        return Ok(());
    }
    let light = r32(c, S0);
    let small = ir_cone(c, -0x26d8, -0x26d4, -0x26d0);
    // Shape power into f3 (1 by default); f0 = intensity.
    let mut unit = !small;
    let mut loaded = false;
    if small {
        c.cycles += 3;
        c.g[V0][0] = 2;
        let square = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
        if square {
            c.cycles += 2;
            setf(c, 3, fr(c, 3) * fr(c, 3));
        } else {
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if high {
                c.cycles += 3;
                c.g[V0][0] = 3;
                if c.g[V1][0] == c.g[V0][0] {
                    let sq = fr(c, 3) * fr(c, 3);
                    setf(c, 0, sq);
                    c.cycles += 2;
                    setf(c, 3, sq * fr(c, 3));
                } else {
                    c.cycles += 2;
                    unit = true;
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    ldf(c, 0, p(light, dyn_light::INTENSITY));
                    loaded = true;
                } else {
                    c.cycles += 2;
                    unit = true;
                }
            }
        }
    }
    if unit {
        c.cycles += 2;
        fconst(c, 3, 0x3f80_0000);
    }
    if !loaded {
        c.cycles += 1;
        ldf(c, 0, p(light, dyn_light::INTENSITY));
    }
    c.cycles += 9;
    setf(c, 1, FpuF32::ZERO);
    let scaled = fr(c, 3) * fr(c, 0);
    setf(c, 0, scaled);
    let five = fconst(c, 3, 0x40a0_0000);
    let k = scaled * fr(c, 2);
    setf(c, 5, k);
    let positive = le(c, FpuF32::ZERO, k);
    setf(c, 4, FpuF32::ZERO);
    if positive {
        c.cycles += 1;
        setf(c, 4, k.min(five));
    }
    c.cycles += 14;
    let k = fr(c, 4);
    let cz = ldf(c, 2, p(light, dyn_light::COLOUR + 8));
    setf(c, 5, k);
    let cx = ldf(c, 1, p(light, dyn_light::COLOUR));
    c.g[A0][0] = c.g[S1][0];
    let cy = ldf(c, 0, p(light, dyn_light::COLOUR + 4));
    setf(c, 2, cz * k);
    setf(c, 1, cx * k);
    c.g[A1][0] = c.g[SP][0];
    setf(c, 0, cy * k);
    c.g[A2][0] = sx(p(sp, 0x10));
    c.swc1(2, p(sp, 0x18));
    c.swc1(1, p(sp, 0x10));
    c.swc1(0, p(sp, 0x14));
    callee::ADD_DIRECTIONAL.call(c, 0x0038_abe4)?;
    ir_abort(c, frame);
    Ok(())
}

// ---------------------------------------------------------------------------
// World painter views.

/// The painter views (0xf0 bytes each).
const WP_VIEWS: u32 = 0x004f_a370;
const WP_VIEW_SIZE: u32 = 0xf0;
/// Default colour pair copied to sp+0/sp+0x10 for the rider's surface.
const WP_DEFAULT_COLOURS: u32 = 0x004f_ab20;
const WP_BLEND_COLOURS: u32 = 0x004f_ab40;
/// Loop heads (event checks).
const WP_NOTIFY_LOOP: u32 = 0x002e_d4e8;
const WP_COPY2_LOOP: u32 = 0x002e_d9ac;
const WP_COPY0_LOOP: u32 = 0x002e_d9ec;
const WP_COPY1_LOOP: u32 = 0x002e_da18;
const WP_COPY_BACK_LOOP: u32 = 0x002e_dad0;

/// `void WorldPainter_UpdateView(int view, float x /*f12*/, float y, float
/// z)` (0x2ed490): tell the view's nine painters about the position
/// (virtual 0x10); for the five player views also: the event world
/// section (view 0), the view's brightness, the rider's surface colours
/// (sampled, or lerped/blended by the rider's state) and the irradiance
/// mix (a cross-fade from the previous one, or the lighting-weighted blend
/// of two irradiance sets) into the view record.
pub fn world_painter_update_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x330);
        c.g[V0][0] = WP_VIEW_SIZE as u64;
        frame.save(c, S3, 0x2d0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x2b0);
        c.mult(0, V0, S3, V0);
        c.g[S5][0] = 0x0050_0000;
        frame.save(c, S0, 0x300);
        c.g[V1][0] = WP_VIEWS as u64;
        frame.save(c, S1, 0x2f0);
        frame.save_fpr(c, 22, 0x320);
        c.g[S1][0] = 8;
        frame.save_fpr(c, 21, 0x318);
        c.mov_s(22, 12);
        frame.save_fpr(c, 20, 0x310);
        c.mov_s(21, 13);
        frame.save(c, S2, 0x2e0);
        c.mov_s(20, 14);
        frame.save(c, S4, 0x2c0);
        c.g[S0][0] = add32(c.g[V0][0], c.g[V1][0]);
        frame.save_ra(c, 0x2a0);
        c.g[A1][0] = c.lw(r32(c, S0));
        // The nine painters: virtual 0x10 (position).
        loop {
            c.cycles += 12;
            c.mov_s(12, 22);
            c.mov_s(13, 21);
            c.mov_s(14, 20);
            c.g[V1][0] = c.lw(at(c, A1, 4));
            c.g[S0][0] = add32(c.g[S0][0], 4);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let vt = r32(c, V1);
            c.g[A0][0] = c.lh(p(vt, 0x10));
            c.g[V0][0] = c.lw(p(vt, 0x14));
            c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
            let target = r32(c, V0);
            call_ptr(c, target, 0x002e_d510)?;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, WP_NOTIFY_LOOP);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[S3][0] < 6) as u64;
        if c.g[V0][0] != 0 {
            wp_player_view(c, frame)?;
        }
        frame.restore(c, S0, 0x300);
        c.cycles += 11;
        frame.restore(c, S1, 0x2f0);
        frame.restore(c, S2, 0x2e0);
        frame.restore(c, S3, 0x2d0);
        frame.restore(c, S4, 0x2c0);
        frame.restore(c, S5, 0x2b0);
        frame.restore_ra(c, 0x2a0);
        frame.restore_fpr(c, 22, 0x320);
        frame.restore_fpr(c, 21, 0x318);
        frame.restore_fpr(c, 20, 0x310);
        frame.close(c);
        Ok(())
    })
}

/// Views 0..5. Returns with s0 about to be restored by the caller.
fn wp_player_view(c: &mut Cpu, frame: Frame) -> Flow {
    let _ = frame;
    // View 0 with event sections on: follow the rider's event section
    // (layer at gp+0x770/0xa7c) when it is a type-6 section.
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp(c, 0xa6c));
    let events_on = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gp(c, -0x848));
    let mut skip_v1 = false;
    if events_on {
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
        if other_view {
            skip_v1 = true;
        } else {
            c.cycles += 16;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x78));
            c.g[A1][0] = c.lw(at(c, A0, 0x1bc));
            call_ptr(c, callee::SECTION_GET_EVENT_WORLD_SECTION, 0x002e_d54c)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[A1][0] = shl32(c.g[A2][0], 3);
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x10));
            c.g[V0][0] = c.lw(r32(c, A0));
            c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
            c.g[V1][0] = c.lw(at(c, V0, 0x24));
            c.g[V1][0] ^= 6;
            let not_event = c.g[V1][0] != 0;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            if !not_event {
                c.cycles += 3;
                c.g[V0][0] = c.lw(gp(c, 0xa7c));
                let same = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = c.lw(gp(c, -0x848));
                if !same {
                    c.cycles += 3;
                    let v = c.g[A2][0];
                    c.sw(gp(c, 0x770), v);
                    c.sw(gp(c, 0xa7c), v);
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                }
            }
        }
    }
    if !skip_v1 {
        c.cycles += 1;
        c.g[V1][0] = WP_VIEW_SIZE as u64;
    }
    // View brightness (+0x24) and the rider's ground tile (s2).
    c.cycles += 14;
    c.mult(0, V1, S3, V1);
    c.g[A2][0] = shl32(c.g[S3][0], 2);
    c.g[A0][0] = c.lw(at(c, V0, 0x84));
    c.g[A1][0] = WP_VIEWS as u64;
    let k = ldf(c, 1, gp(c, -0x3974));
    c.g[V0][0] = c.lw(at(c, A0, 0xc));
    c.g[V1][0] = add32(c.g[V1][0], c.g[A1][0]);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    let level = ldf(c, 0, at(c, V1, 0x24));
    c.g[S1][0] = c.lw(at(c, V0, 0x28));
    c.g[A0][0] = add32(c.g[V1][0], 0x24);
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x3fc));
    let boosted = c.g[V0][0] != 0;
    let scaled = level * k;
    setf(c, 1, scaled);
    if boosted {
        c.cycles += 3;
        let add = ldf(c, 0, gp(c, -0x3970));
        setf(c, 0, scaled + add);
    } else {
        c.cycles += 1;
        setf(c, 0, scaled);
    }
    c.cycles += 5;
    c.swc1(0, r32(c, A0));
    c.g[A2][0] = c.lw(p(rider, 0x430));
    c.g[V0][0] = !c.g[A2][0];
    let none = c.g[V0][0] == 0;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    if none {
        c.cycles += 1;
        c.g[S2][0] = 0;
    } else {
        // tile = world->layers[id & 0xff]->tiles[id >> 8] (>> 8, * 4).
        c.cycles += 8;
        c.g[V0][0] = shl32(c.g[A2][0] & 0xff, 2);
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = c.lw(at(c, V1, 8));
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V0][0] = c.lw(r32(c, V0));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(at(c, V0, 0xc));
            c.cycles += 7;
            c.g[V1][0] = shl32(srl32(c.g[A2][0], 8), 2);
            c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = srl32(c.g[A0][0], 8);
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = shl32(c.g[V0][0], 2);
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        let other_view = c.g[S3][0] != 0;
        c.g[S2][0] = c.g[A0][0];
        if !other_view {
            c.cycles += 4;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            let v = c.g[V0][0];
            c.sw(gp(c, 0x770), v);
            c.sw(gp(c, 0xa7c), v);
        }
    }
    wp_surface_colours(c)?;
    wp_irradiance(c)
}

/// The rider's surface colours into the view (`gp+0xa68`: blend pending).
/// Leaves v0 = 0xf0.
fn wp_surface_colours(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[S4][0] = add32(c.g[S1][0], 0xaac);
    call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d664)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    let state1 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0x0050_0000;
    let sp = sp(c);
    let pending;
    if state1 {
        wp_lerp_colours(c, sp)?;
        c.g[V0][0] = 1;
        pending = Some(());
    } else {
        // Tile surface type 6 with a type-3 entry: blend toward fixed
        // colours; rider state 4: pending; else sample and blend.
        c.cycles += 2;
        let has_tile = c.g[S2][0] != 0;
        c.g[V0][0] = c.lw(gp(c, 0x16c8));
        let mut special = false;
        if has_tile {
            c.cycles += 4;
            let tile = r32(c, S2);
            c.g[A1][0] = c.lh(p(tile, 0x156));
            c.g[A0][0] = c.lw(r32(c, V0));
            let neg = (c.g[A1][0] as i64) < 0;
            c.g[V0][0] = 0;
            if !neg {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(p(tile, 0x155));
                c.g[V0][0] = shl32(c.g[V0][0], 3);
                c.g[V0][0] = add32(c.g[A0][0], c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, V0, 0x24));
                c.g[V1][0] ^= 6;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x18;
                    c.mult(0, V1, A1, V0);
                    c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
                    c.g[V1][0] = c.lw(at(c, V0, 0x3f0));
                    c.g[V1][0] ^= 3;
                    c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                }
            }
            c.cycles += 2;
            special = c.g[V0][0] != 0;
        }
        if !special {
            c.cycles += 19;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            let one = fconst(c, 0, 0x3f80_0000);
            c.mult(0, A0, S3, A0);
            c.g[V0][0] = 1;
            c.g[V1][0] = WP_VIEWS as u64;
            c.g[A2][0] = 0x0050_0000;
            let v = c.g[V0][0];
            c.sw(gp(c, 0xa68), v);
            c.g[A2][0] = WP_BLEND_COLOURS as u64;
            c.swc1(0, p(sp, 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
            c.sw(sp, 0);
            c.swc1(0, p(sp, 4));
            c.g[A3][0] = 0;
            c.swc1(0, p(sp, 8));
            let _ = one;
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d8c8)?;
            c.g[V0][0] = WP_VIEW_SIZE as u64;
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::RIDER_GET_MAIN_STATE, 0x002e_d8d8)?;
        c.g[V1][0] = 4;
        let state4 = c.g[V0][0] == c.g[V1][0];
        c.g[S0][0] = sx(p(sp, 0x10));
        if state4 {
            c.cycles += 1;
            c.g[V0][0] = 1;
            pending = Some(());
        } else {
            c.cycles += 15;
            let pos = r32(c, S4);
            ldf(c, 13, p(pos, 4));
            ldf(c, 12, pos);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d908)?;
            c.g[A0][0] = WP_VIEW_SIZE as u64;
            c.g[V0][0] = WP_VIEWS as u64;
            c.mult(0, A0, S3, A0);
            c.g[A3][0] = c.lw(gp(c, 0xa68));
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
            callee::BLEND_VIEW_COLOURS.call(c, 0x002e_d928)?;
            c.sw(gp(c, 0xa68), 0);
            pending = None;
        }
    }
    if pending.is_some() {
        c.cycles += 2;
        let v = c.g[V0][0];
        c.sw(gp(c, 0xa68), v);
    }
    c.cycles += 1;
    c.g[V0][0] = WP_VIEW_SIZE as u64;
    Ok(())
}

/// Rider state 1: the default colour pair, then (on the rider's board
/// surface kinds 1 and 3) a lerp factor from the board's travel and the
/// sampled surface; `WorldPainter_LerpViewColours`.
fn wp_lerp_colours(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 24;
    c.g[V1][0] = 0x0050_0000;
    c.g[T0][0] = WP_DEFAULT_COLOURS as u64;
    ld_d(c, A1, WP_DEFAULT_COLOURS);
    ld_d(c, A2, p(WP_DEFAULT_COLOURS, 8));
    sd_d(c, A1, sp);
    sd_d(c, A2, p(sp, 8));
    c.g[T0][0] = WP_BLEND_COLOURS as u64;
    ld_d(c, V0, WP_BLEND_COLOURS);
    ld_d(c, A2, p(WP_BLEND_COLOURS, 8));
    sd_d(c, V0, p(sp, 0x10));
    sd_d(c, A2, p(sp, 0x18));
    ldf(c, 1, gp(c, -0x396c));
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    c.g[V1][0] = c.lw(at(c, V0, 0xac));
    let kind1 = c.g[V1][0] == c.g[A0][0];
    c.g[A1][0] = 0;
    let lerp = if kind1 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        c.g[V1][0] == c.g[V0][0]
    };
    if lerp {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    let on = c.g[A1][0] != 0;
    c.g[A2][0] = sx(p(sp, 0x10));
    if on {
        wp_lerp_factor(c, sp)?;
    }
    c.cycles += 9;
    c.g[A0][0] = WP_VIEW_SIZE as u64;
    c.g[V0][0] = WP_VIEWS as u64;
    c.mult(0, A0, S3, A0);
    c.mov_s(12, 1);
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
    callee::LERP_VIEW_COLOURS.call(c, 0x002e_d828)
}

/// The lerp factor (f1) from the board record (rider +0x788): travel
/// `f21 = +0x98 - +0xa0`, a soft limit `f20 = clamp(+0x98 / 2, 2,
/// +0x98)`; below it a ratio, beyond it the sampled surface colours.
fn wp_lerp_factor(c: &mut Cpu, sp: u32) -> Flow {
    c.cycles += 9;
    let rider = r32(c, S1);
    c.g[V0][0] = c.lw(p(rider, 0x788));
    let board = r32(c, V0);
    c.g[A0][0] = c.lw(p(board, 0x30));
    c.g[T0][0] = c.g[V0][0];
    let span = ldf(c, 2, p(board, 0x98));
    let pos = ldf(c, 0, p(board, 0xa0));
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = !c.g[A0][0];
    let no_surface = c.g[V1][0] == 0;
    let travel = span - pos;
    setf(c, 21, travel);
    if no_surface {
        return Ok(());
    }
    c.cycles += 11;
    let half = fconst(c, 1, 0x3f00_0000);
    let two = fconst(c, 0, 0x4000_0000);
    let limit = (span * half).max(two).min(span);
    setf(c, 20, limit);
    let beyond = lt(c, limit, travel);
    setf(c, 2, span - limit);
    if beyond {
        c.cycles += 9;
        let rest = fr(c, 2);
        let d = travel - limit;
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3968));
        setf(c, 1, FpuF32::ZERO);
        let r = (rest - d).div(rest);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        setf(c, 12, FpuF32::ZERO);
        if pos {
            c.cycles += 1;
            setf(c, 12, r.min(cap));
        }
        c.cycles += 2;
        c.mov_s(1, 12);
        return Ok(());
    }
    // The tile under the surface id (a3).
    c.cycles += 9;
    c.g[A1][0] = c.lw(gp(c, 0x16c8));
    c.g[V0][0] = shl32(c.g[A0][0] & 0xff, 2);
    c.g[V1][0] = c.lw(r32(c, A1));
    c.g[A0][0] = c.lw(at(c, V1, 8));
    c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = c.lw(r32(c, V0));
    let none = c.g[V1][0] == 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 8;
        c.g[A0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = shl32(srl32(c.g[A3][0], 8), 2);
        c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[V0][0] = srl32(c.g[V1][0], 8);
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = shl32(c.g[V0][0], 2);
        }
    }
    c.cycles += 11;
    c.g[S0][0] = sx(p(sp, 0x10));
    let board = r32(c, T0);
    ldf(c, 13, p(board, 0x38));
    ldf(c, 12, p(board, 0x34));
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = c.g[SP][0];
    callee::SAMPLE_SURFACE_COLOURS.call(c, 0x002e_d7cc)?;
    setf(c, 2, FpuF32::ZERO);
    let limit = fr(c, 20);
    let positive = lt(c, FpuF32::ZERO, limit);
    c.g[A2][0] = c.g[S0][0];
    if positive {
        c.cycles += 7;
        let d = limit - fr(c, 21);
        setf(c, 0, d);
        let cap = ldf(c, 3, gp(c, -0x3964));
        let r = d.div(limit);
        setf(c, 0, r);
        let pos = le(c, FpuF32::ZERO, r);
        c.mov_s(1, 2);
        if pos {
            c.cycles += 2;
            setf(c, 1, r.min(cap));
        }
        return Ok(());
    }
    c.cycles += 2;
    fconst(c, 1, 0x3f80_0000);
    Ok(())
}

/// Copy `n` 0x20-byte blocks with two `lq`/`sq` pairs through `r0`/`r1`
/// (`src`/`dst` registers advanced, loop until `src == end_reg`), event
/// checks at `head`.
fn wp_copy_blocks(c: &mut Cpu, src: usize, dst: usize, end_reg: usize, r0: usize, r1: usize, head: u32) {
    loop {
        c.cycles += 7;
        let s = r32(c, src);
        let d = r32(c, dst);
        c.lq(r0, s);
        c.lq(r1, p(s, 0x10));
        c.sq(r0, d);
        c.sq(r1, p(d, 0x10));
        c.g[src][0] = sx(p(s, 0x20));
        let more = c.g[src][0] != c.g[end_reg][0];
        c.g[dst][0] = sx(p(r32(c, dst), 0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The view's irradiance (+0x50, 0xa0 bytes): a cross-fade (fade >
/// threshold) or the lighting-weighted mix of sets 0 and 1.
fn wp_irradiance(c: &mut Cpu) -> Flow {
    let sp = sp(c);
    c.cycles += 9;
    c.g[V1][0] = WP_VIEWS as u64;
    c.mult(0, A2, S3, V0);
    let threshold = ldf(c, 1, gp(c, -0x3960));
    c.g[V0][0] = add32(c.g[A2][0], c.g[V1][0]);
    let fade = ldf(c, 0, at(c, V0, 0x24));
    let fading = lt(c, threshold, fade);
    c.g[A0][0] = 1;
    if !fading {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    c.cycles += 2;
    let fading = c.g[A0][0] != 0;
    c.g[S0][0] = add32(c.g[V1][0], 0x50);
    if fading {
        c.cycles += 19;
        c.g[S2][0] = sx(p(sp, 0xc0));
        c.g[S0][0] = add32(c.g[A2][0], c.g[S0][0]);
        c.g[S1][0] = sx(p(sp, 0x20));
        c.g[A0][0] = c.g[S3][0];
        callee::GET_IRRADIANCE2.call(c, 0x002e_d974)?;
        ldf(c, 12, gp(c, 0xa80));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_SCALE, 0x002e_d984)?;
        let t = ldf(c, 0, gp(c, 0xa80));
        c.g[A0][0] = c.g[S0][0];
        let one = fconst(c, 12, 0x3f80_0000);
        setf(c, 12, one - t);
        call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_d99c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_d9ac)?;
        wp_copy_blocks(c, S1, S0, S2, A0, A1, WP_COPY2_LOOP);
        c.cycles += 2;
        return Ok(());
    }
    // Sets 0 and 1 to sp+0 and sp+0xa0.
    c.cycles += 7;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE0.call(c, 0x002e_d9d8)?;
    c.g[S4][0] = sx(p(sp, 0xa0));
    c.g[S5][0] = sx(p(sp, 0x160));
    c.g[S2][0] = sx(p(sp, 0x200));
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    c.g[V1][0] = c.g[SP][0];
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY0_LOOP);
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_IRRADIANCE1.call(c, 0x002e_da10)?;
    c.g[V1][0] = c.g[S4][0];
    c.g[A0][0] = add32(c.g[V0][0], 0xa0);
    wp_copy_blocks(c, V0, V1, A0, T0, A1, WP_COPY1_LOOP);
    // w = factor * lighting; mix = set0 * w + set1 * (1 - w); then the
    // cross-fade with the view's previous irradiance (+0x68).
    c.cycles += 39;
    c.g[S1][0] = WP_VIEW_SIZE as u64;
    c.g[S0][0] = 0x0050_0000;
    c.mult(0, S1, S3, S1);
    c.g[S0][0] = sx(0x004f_a3a8);
    let one = fconst(c, 21, 0x3f80_0000);
    c.g[A0][0] = add32(c.g[S1][0], c.g[S0][0]);
    callee::COMPUTE_COLOUR_FACTOR.call(c, 0x002e_da54)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S3][0];
    callee::GET_LIGHTING28.call(c, 0x002e_da60)?;
    let w = fr(c, 20) * fr(c, 0);
    setf(c, 20, w);
    c.g[S0][0] = add32(c.g[S0][0], 0x18);
    c.g[A0][0] = c.g[SP][0];
    c.g[S1][0] = add32(c.g[S1][0], c.g[S0][0]);
    setf(c, 12, w);
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da78)?;
    setf(c, 12, fr(c, 21) - fr(c, 20));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_da88)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_da98)?;
    ldf(c, 12, gp(c, 0xa80));
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_daa4)?;
    let t = ldf(c, 12, gp(c, 0xa80));
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    setf(c, 12, fr(c, 21) - t);
    let _ = one;
    call_ptr(c, callee::IRRADIANCE_SCALE_IN_PLACE, 0x002e_dab8)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, callee::IRRADIANCE_ADD, 0x002e_dac8)?;
    c.g[V0][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S5][0];
    wp_copy_blocks(c, A0, S1, V0, T0, V1, WP_COPY_BACK_LOOP);
    c.cycles += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP, SP as TEST_SP};

    const SEEDS: u32 = 250;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    /// Scrambled registers and the game's gp.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    fn put_vec3(c: &mut Cpu, r: &mut Rng, at: u32, lo: f32, hi: f32) {
        for k in 0..3 {
            c.bus.write32(at + 4 * k, f(uf(r, lo, hi)));
        }
    }

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    #[test]
    fn world_light_man_light_distance_matches() {
        check(ee::s09::f_002f6168__generated, world_light_man_light_distance, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2f61);
            let (pos, light, out) = (DATA, DATA + 0x100, DATA + 0x200);
            let spread = [0.0f32, 40.0, 3.0, 8.0][(seed % 4) as usize];
            for k in 0..3 {
                let l = uf(&mut r, -50.0, 50.0);
                c.bus.write32(light + 0x38 + 4 * k, f(l));
                c.bus.write32(pos + 4 * k, f(l + uf(&mut r, -spread, spread)));
            }
            c.bus.write32(light + 0x1c, f(uf(&mut r, 2.0, 20.0)));
            put_vec3(c, &mut r, light + 0x2c, -1.0, 1.0);
            c.g[A1][0] = sx(pos);
            c.g[A2][0] = sx(light);
            c.g[A3][0] = sx(out);
            c.g[T0][0] = sx(out + 4);
            c.g[T1][0] = if seed % 3 == 0 { 0 } else { sx(out + 8) };
        });
    }

    /// An emitter at `e` with four-slot rings.
    fn put_emitter(c: &mut Cpu, r: &mut Rng, e: u32, seed: u32) {
        c.bus.write32(e + 0x174, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x1000);
        c.bus.write32(e + 0x1a4, DATA + 0x2000);
        c.bus.write32(e + 0x204, DATA + 0x3000);
        c.bus.write32(e + 0xc, 1 + r.next_u32() % 4);
        c.bus.write32(e + 0x10, f(uf(r, 0.0, 4.0)));
        c.bus.write32(e + 0x14, f(uf(r, 0.0, 2.0)));
        c.bus.write32(e + 0x1e0, r.next_u32() % 8);
        fill(c, r, e + 0x1b0, 8);
        for k in 0..4 {
            c.bus.write32(e + 0x1d0 + 4 * k, f(uf(r, -0.5, 1.5)));
        }
        for i in 0..4 {
            fill(c, r, DATA + 0x2000 + 0x10 * i, 3);
            c.bus.write32(DATA + 0x2000 + 0x10 * i + 0xc, f(uf(r, -1.0, 1.0)));
        }
    }

    #[test]
    fn dynamic_emitter_spawn_matches() {
        check(ee::s12::f_003710d0__generated, dynamic_emitter_spawn, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3710);
            let e = DATA;
            put_emitter(c, &mut r, e, seed);
            fill(c, &mut r, DATA + 0x400, 8);
            c.g[A0][0] = sx(e);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
            c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x410) };
            c.g[A3][0] = (seed / 2 % 2) as u64;
            c.f[12] = f(uf(&mut r, 0.0, 3.0));
        });
    }

    #[test]
    fn dynamic_emitter_spawn_at_matches() {
        check(ee::s12::f_003717c0__generated, dynamic_emitter_spawn_at, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3717);
            let e = DATA;
            put_emitter(c, &mut r, e, seed);
            fill(c, &mut r, DATA + 0x400, 8);
            for k in 0..4 {
                c.bus.write32(DATA + 0x420 + 4 * k, f(uf(&mut r, -0.5, 1.5)));
            }
            c.g[A0][0] = sx(e);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x400) };
            c.g[A2][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x410) };
            c.g[A3][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x420) };
            c.g[T0][0] = (seed / 2 % 2) as u64;
            c.f[12] = f(uf(&mut r, 0.0, 3.0));
        });
    }

    #[test]
    fn spatial_key_classify_aabb_matches() {
        check(ee::s10::f_00328360__generated, spatial_key_classify_aabb, 300, &|c, seed| {
            let mut r = base(c, seed, 0x3283);
            let (key, aabb) = (DATA, DATA + 0x40);
            let level = 11 + r.next_u32() % 3;
            let size = (1u32 << level) as f32;
            // Exact corners (no margin) for the degenerate-box seeds.
            let touching = seed % 10 == 9;
            let m = if touching { 0.0 } else { uf(&mut r, 0.0, 0.5) };
            c.bus.write32(gp_at(-0x2dfc), f(m));
            c.bus.write32(key, level);
            for k in 0..3u32 {
                let i = (r.next_u32() % 9) as i32 - 4;
                c.bus.write32(key + 4 + 4 * k, i as u32);
                let lo = (i as f32 - m) * size;
                let hi = (i as f32 + 1.0 + m) * size;
                let w = hi - lo;
                let case = if touching {
                    if k < 2 { 5 } else { 0 }
                } else if r.next_u32() % 3 != 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                let (mn, mx) = match case {
                    0 => {
                        let a = lo + uf(&mut r, 0.05, 0.45) * w;
                        (a, a + uf(&mut r, 0.0, 0.4) * w)
                    }
                    1 => (lo - 0.2 * w, lo + 0.2 * w),
                    2 => (hi - 0.2 * w, hi + 0.2 * w),
                    3 => (lo - 0.5 * w, lo - 0.1 * w),
                    4 => (hi + 0.1 * w, hi + 0.5 * w),
                    _ => (hi, hi),
                };
                c.bus.write32(aabb + 4 * k, f(mn));
                c.bus.write32(aabb + 0x10 + 4 * k, f(mx));
            }
            c.g[A0][0] = sx(key);
            c.g[A1][0] = sx(aabb);
        });
    }

    #[test]
    fn path_get_closest_point_from_matches() {
        check(ee::s07::f_0026a638__generated, path_get_closest_point_from, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x26a6);
            let (pth, point, out, dist, cur, segs) =
                (DATA, DATA + 0x80, DATA + 0x90, DATA + 0xa0, DATA + 0xc0, DATA + 0x100);
            let n = 1 + r.next_u32() % 5;
            c.bus.write32(pth + 8, n);
            put_vec3(c, &mut r, pth + 0xc, -10.0, 10.0);
            c.bus.write32(pth + 0x18, segs);
            for i in 0..n {
                let s = segs + 0x10 * i;
                put_vec3(c, &mut r, s, -1.0, 1.0);
                c.bus.write32(s + 0xc, f(uf(&mut r, 0.5, 10.0)));
            }
            put_vec3(c, &mut r, point, -20.0, 20.0);
            c.bus.write32(point + 0xc, f(uf(&mut r, -1.0, 1.0)));
            if seed % 3 == 0 {
                c.bus.write32(cur + 0x14, u32::MAX);
            } else {
                put_vec3(c, &mut r, cur, -10.0, 10.0);
                c.bus.write32(cur + 0xc, f(1.0));
                c.bus.write32(cur + 0x10, f(uf(&mut r, 0.0, 20.0)));
                c.bus.write32(cur + 0x14, r.next_u32() % n);
            }
            c.bus.write32(gp_at(-0x4a54), f(1.0e30));
            c.bus.write32(gp_at(-0x4a50), f(uf(&mut r, 0.0, 15.0)));
            c.g[A0][0] = sx(pth);
            c.g[A1][0] = sx(point);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = sx(dist);
            c.g[T0][0] = sx(cur);
            c.g[T1][0] = (seed / 3 % 2) as u64;
        });
    }

    #[test]
    fn game_module_draw_bezier_patch_matches() {
        check(ee::s05::f_0022c410__generated, game_module_draw_bezier_patch, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x22c4, 1);
            let (patch, view, obj, vt) = (DATA, DATA + 0x200, DATA + 0x400, DATA + 0x500);
            put_vec3(c, &mut r, patch + 0x10, -100.0, 100.0);
            for k in 0..12 {
                c.bus.write32(view + 0x170 + 4 * k, f(uf(&mut r, -100.0, 100.0)));
            }
            c.bus.write32(view + 0x1a0, r.next_u32());
            c.bus.write32(patch + 0x20, f(uf(&mut r, 0.0, 0.0005)));
            c.bus.write32(patch + 0x24, f(uf(&mut r, -2.0, 8.0)));
            let lo = uf(&mut r, 0.0, 2.0);
            c.bus.write32(patch + 0x28, f(lo));
            c.bus.write32(patch + 0x2c, f(lo + uf(&mut r, 0.0, 6.0)));
            fill(c, &mut r, gp_at(-0x6e0), 4);
            c.bus.write32(patch, obj);
            c.bus.write32(obj + 4, vt);
            c.bus.write16(vt + 0xa8, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0xac, FAKE);
            c.g[A0][0] = sx(patch);
            c.g[A1][0] = sx(view);
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ps2_graphics_man_v58_matches() {
        check(ee::s12::f_003889f0__generated, ps2_graphics_man_v58, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3889);
            let (out, gm, table, tex, clut, pixels) =
                (DATA, DATA + 0x100, DATA + 0x2000, DATA + 0x2100, DATA + 0x2400, DATA + 0x4000);
            c.bus.write32(gm + 0x18f4, table);
            let idx = r.next_u32() % 4;
            c.bus.write32(table + 8 + 4 * idx, if seed % 9 == 0 { 0 } else { tex });
            let psm = [0u32, 0xa, 0xc, 0x13][(seed % 4) as usize];
            c.bus.write32(tex + 0xc, psm);
            c.bus.write32(tex + 0x1c, pixels);
            c.bus.write32(tex + 0x20, clut);
            let (tw, th) = (1 + r.next_u32() % 6, 1 + r.next_u32() % 6);
            let tex0 = (r.next_u32() as u64 & 0x03ff_ffff)
                | (tw as u64) << 26
                | (th as u64) << 30
                | (r.next_u32() as u64) << 34;
            c.bus.write64(tex + 0x38, tex0);
            fill(c, &mut r, pixels, 0x1000);
            fill(c, &mut r, clut, 0x100);
            c.bus.write32(gp_at(-0x2730), f(uf(&mut r, 0.0, 2.0)));
            c.f[12] = if seed % 11 == 0 { f(0.0) } else { f(uf(&mut r, -0.3, 1.3)) };
            c.f[13] = if seed % 13 == 0 { f(1.0) } else { f(uf(&mut r, -0.3, 1.3)) };
            c.g[A0][0] = sx(out);
            c.g[A1][0] = sx(gm);
            c.g[A2][0] = idx as u64;
        });
    }

    #[test]
    fn world_cache_streamer_update_view_node_matches() {
        check(ee::s13::f_003aa028__generated, world_cache_streamer_update_view_node, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3aa0);
            let (wc, cache, hulls) = (DATA, DATA + 0x8000, DATA + 0x3000);
            c.bus.write32(wc, cache);
            c.bus.write32(gp_at(0x2a74), DATA + 0x9000);
            c.bus.write32(gp_at(-0x23c0), f(uf(&mut r, 0.0, 4.0)));
            c.bus.write32(gp_at(-0x23bc), f(uf(&mut r, 0.0, 4.0)));
            for id in 0..8 {
                c.bus.write32(cache + 0x3f0 + 0x18 * id, r.next_u32() % 5);
            }
            let view = r.next_u32() % 2;
            let rec = wc + 0x250 + 0x50 * view;
            put_vec3(c, &mut r, rec, -10.0, 10.0);
            c.bus.write32(rec + 0xc, f(1.0));
            for k in 0..3 {
                let lo = uf(&mut r, -12.0, 4.0);
                c.bus.write32(rec + 0x10 + 4 * k, f(lo));
                c.bus.write32(rec + 0x20 + 4 * k, f(lo + uf(&mut r, 0.0, 16.0)));
            }
            c.bus.write32(rec + 0x1c, f(uf(&mut r, -1.0, 1.0)));
            c.bus.write32(rec + 0x2c, f(uf(&mut r, -1.0, 1.0)));
            c.bus.write32(rec + 0x30, f(uf(&mut r, 1.0, 30.0)));
            c.bus.write32(rec + 0x34, f(uf(&mut r, 0.0, 30.0)));
            c.bus.write32(rec + 0x38, f(uf(&mut r, 0.0, 30.0)));
            // A root with up to two leaf children.
            for i in 0..3u32 {
                let nd = DATA + 0x1000 + 0x100 * i;
                for k in 0..3 {
                    let lo = uf(&mut r, -12.0, 8.0);
                    c.bus.write32(nd + 4 * k, f(lo));
                    c.bus.write32(nd + 0x10 + 4 * k, f(lo + uf(&mut r, 0.0, 10.0)));
                }
                for k in 0..8 {
                    c.bus.write32(nd + 0x20 + 4 * k, f(uf(&mut r, -1.0, 1.0)));
                }
                let hull = hulls + 0x10 * i;
                c.bus.write16(hull + 2, (r.next_u32() % 8) as u16);
                c.bus.write32(nd + 0x58, if r.next_u32() % 4 == 0 { 0 } else { hull });
                let (left, right) = if i == 0 { (DATA + 0x1100, DATA + 0x1200) } else { (0, 0) };
                c.bus.write32(nd + 0x50, if seed % 3 == 0 { 0 } else { left });
                c.bus.write32(nd + 0x54, if seed % 5 == 0 { 0 } else { right });
            }
            c.g[A0][0] = sx(wc);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = view as u64;
            c.g[A3][0] = (seed / 2 % 2) as u64;
        });
    }

    #[test]
    fn hud_draw_position_panel_matches() {
        check(ee::s05::f_0021ed48__generated, hud_draw_position_panel, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x21ed, 6);
            let (hud, elems, rider, text, sprite) = (DATA, DATA + 0x1000, DATA + 0x1c00, DATA + 0x800, DATA + 0x900);
            c.bus.write32(hud + 0x42c, text);
            c.bus.write32(hud + 0x494, sprite);
            c.bus.write32(hud + 0x474, r.next_u32());
            for k in 0..4 {
                c.bus.write32(hud + 0x480 + 4 * k, f(uf(&mut r, 0.0, 50.0)));
            }
            c.bus.write32(text + 0x30, f(uf(&mut r, 1.0, 20.0)));
            c.bus.write32(text + 0x34, f(uf(&mut r, 1.0, 20.0)));
            // 48 HUD elements: position, size, colour, alignment, row.
            for i in 0..48u32 {
                let e = elems + 0x24 * i;
                c.bus.write16(e, (r.next_u32() % 640) as u16);
                c.bus.write16(e + 2, (r.next_u32() % 448) as u16);
                c.bus.write16(e + 4, (r.next_u32() % 64) as u16);
                c.bus.write16(e + 6, (r.next_u32() % 64) as u16);
                for k in 0..4 {
                    c.bus.write32(e + 0x10 + 4 * k, f(uf(&mut r, 0.0, 1.0)));
                }
                c.bus.write8(e + 0x20, (r.next_u32() % 3) as u8);
                c.bus.write8(e + 0x21, (r.next_u32() % 3) as u8);
                c.bus.write8(e + 0x22, (r.next_u32() % 8) as u8);
            }
            // The ordinal digit table and the two-character string.
            let digits: Vec<u32> = (0..9).map(|_| r.next_u32() % 40).collect();
            put_words(c, 0x0046_ed40, &digits);
            c.bus.write8(0x004a_22e8, b'0');
            c.bus.write8(0x004a_22e9, 0);
            c.bus.write32(gp_at(-0xf28), DATA + 0x1800);
            fill(c, &mut r, DATA + 0x1800, 4);
            c.bus.write32(gp_at(-0x854), DATA + 0x1900);
            c.bus.write32(DATA + 0x1900 + 0xe84, DATA + 0x1a00);
            c.bus.write32(DATA + 0x1a00 + 8, r.next_u32());
            for off in [0x5358, 0x5354, 0x5350, 0x534c, 0x5348] {
                c.bus.write32(gp_at(-off), f(uf(&mut r, 0.1, 2.0)));
            }
            put_vec3(c, &mut r, DATA + 0x1b00, 0.5, 2.0);
            put_vec3(c, &mut r, DATA + 0x1b10, 0.5, 2.0);
            put_vec3(c, &mut r, rider + 0x54, 0.0, 200.0);
            c.bus.write32(rider + 0x60, f(uf(&mut r, 0.0, 200.0)));
            c.bus.write32(rider + 0x64, f(uf(&mut r, -0.5, 1.8)));
            c.bus.write32(TEST_SP, r.next_u32() % 8);
            c.g[A0][0] = sx(hud);
            c.g[A1][0] = sx(elems);
            c.g[A2][0] = sx(rider);
            c.g[A3][0] = (seed % 2) as u64;
            c.g[T0][0] = (r.next_u32() % 8) as u64;
            c.g[T1][0] = sx(DATA + 0x1b00);
            c.g[T2][0] = sx(DATA + 0x1b10);
            c.g[T3][0] = (seed / 2 % 3) as u64;
        });
    }

    /// The spatial hash test world: level-11 keys (tiny scale), roots at
    /// level 11 holding the coordinates the case needs.
    #[test]
    fn spatial_hash_update_matches() {
        check(ee::s10::f_003291e0__generated, spatial_hash_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3291, 0);
            let (hash, obj_node, old_box, new_box) = (DATA, DATA + 0x3000, DATA + 0x100, DATA + 0x140);
            c.bus.write32(gp_at(-0x2df8), f(1.0e-6));
            c.bus.write32(gp_at(-0x2df4), f(0.0));
            let case = seed % 4;
            let cell = |r: &mut Rng| -> i32 { (r.next_u32() % 6) as i32 - 3 };
            let old: [i32; 3] = [cell(&mut r), cell(&mut r), cell(&mut r)];
            let mut new = old;
            match case {
                0 => {}
                1 => new[0] = if old[0] < 0 { 1 } else { -2 },
                2 => new[1] = if old[1] == -1 { -2 } else if old[1] == 0 { 1 } else { old[1] + old[1].signum() },
                _ => new[2] = if old[2] == -1 { -2 } else if old[2] == 0 { 1 } else { old[2] + old[2].signum() },
            }
            let put_box = |c: &mut Cpu, r: &mut Rng, at: u32, k: &[i32; 3]| {
                for a in 0..3 {
                    let centre = (k[a] as f32 + 0.5) * 2048.0 + uf(r, -500.0, 500.0);
                    let half = uf(r, 0.0, 50.0);
                    c.bus.write32(at + 4 * a as u32, f(centre - half));
                    c.bus.write32(at + 0x10 + 4 * a as u32, f(centre + half));
                }
            };
            put_box(c, &mut r, old_box, &old);
            if case == 0 {
                for w in 0..8 {
                    let v = c.bus.read32(old_box + 4 * w);
                    c.bus.write32(new_box + 4 * w, v);
                }
            } else {
                put_box(c, &mut r, new_box, &new);
            }
            // Roots: level 11 at the coordinates of the key that lands
            // there (case 3: the old key's, so the new one is not held).
            let sign = |k: &[i32; 3]| -> u32 {
                0x50 * (k[0] < 0) as u32 + 0x28 * (k[1] < 0) as u32 + 0x14 * (k[2] < 0) as u32
            };
            for (i, k) in [old, new].iter().enumerate() {
                let root = hash + sign(k);
                let held = if case == 3 { &old } else { k };
                c.bus.write32(root, 11);
                for a in 0..3 {
                    c.bus.write32(root + 4 + 4 * a as u32, held[a] as u32);
                }
                // Bucket arrays: empty (a pool-range address, freed) on
                // some moved-root seeds; else a live word among the first
                // eight, or (odd seeds) only the object's list head, which
                // stays set because the object has a successor.
                let empty = case == 1 && i == 0 && seed % 8 == 1;
                let arr = if empty { 0x200 } else { DATA + 0x1000 + 0x100 * i as u32 };
                c.bus.write32(root + 0x10, arr);
                if !empty && seed % 2 == 0 {
                    let w = r.next_u32() % 8;
                    c.bus.write32(arr + 4 * w, 1 + r.next_u32() % 100);
                }
            }
            let empty = case == 1 && seed % 8 == 1;
            c.bus.write32(obj_node, if empty || seed % 2 == 0 { 0 } else { DATA + 0x3100 });
            c.bus.write32(obj_node + 4, 0);
            c.g[A0][0] = sx(hash);
            // (List heads 0..2 lie inside the scanned words.)
            c.g[A1][0] = (r.next_u32() % 3) as u64;
            c.g[A2][0] = sx(obj_node);
            c.g[A3][0] = sx(new_box);
            c.g[T0][0] = sx(old_box);
        });
    }

    #[test]
    fn irradiance_add_dynamic_light_matches() {
        check(ee::s12::f_0038a6a8__generated, irradiance_add_dynamic_light, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x38a6, 0);
            put_words(c, IR_TABLE_OUTER, &[0x0038_a834, 0x0038_a848, 0x0038_a850, 0x0038_a868, 0x0038_a884]);
            put_words(c, IR_TABLE_INNER, &[0x0038_a948, 0x0038_a95c, 0x0038_a970, 0x0038_a988, 0x0038_a9a4]);
            let (irr, pos, light) = (DATA, DATA + 0x400, DATA + 0x500);
            let kind = [1u32, 2, 0, 3, 1, 2][(seed % 6) as usize];
            c.bus.write32(light + 0x10, kind);
            put_vec3(c, &mut r, pos, -20.0, 20.0);
            put_vec3(c, &mut r, light, -20.0, 20.0);
            c.bus.write32(light + 0xc, f(1.0));
            fill(c, &mut r, irr, 0x40);
            c.bus.write32(light + 0x14, f(uf(&mut r, 0.0, 3.0)));
            c.bus.write32(light + 0x18, f(uf(&mut r, -1.0, 1.0)));
            c.bus.write32(light + 0x1c, f(uf(&mut r, 0.0, 2.0)));
            put_vec3(c, &mut r, light + 0x20, 0.0, 1.0);
            put_vec3(c, &mut r, light + 0x2c, -1.0, 1.0);
            fill(c, &mut r, light + 0x38, 8);
            for k in 0..8 {
                c.bus.write32(light + 0x40 + 4 * k, f(uf(&mut r, -30.0, 30.0)));
            }
            let outer = uf(&mut r, 0.0, 10.0);
            c.bus.write32(light + 0x60, f(outer));
            c.bus.write32(light + 0x5c, f(outer + uf(&mut r, 0.0, 30.0)));
            c.bus.write8(light + 0x64, (r.next_u32() % 5) as u8);
            let exponent = match seed % 5 {
                0 => (r.next_u32() % 5) as u8,
                1 => (r.next_u32() % 4) as u8,
                2 => 5 + (r.next_u32() % 6) as u8,
                3 => (0u8).wrapping_sub(1 + (r.next_u32() % 4) as u8),
                _ => (r.next_u32() % 3) as u8,
            };
            c.bus.write8(light + 0x65, exponent);
            for off in [0x26e4, 0x26e0, 0x26dc, 0x26d8, 0x26d4, 0x26d0] {
                c.bus.write32(gp_at(-off), f(uf(&mut r, 0.0, 1.0)));
            }
            c.g[A0][0] = sx(irr);
            c.g[A1][0] = sx(pos);
            c.g[A2][0] = sx(light);
        });
    }

    #[test]
    fn world_painter_update_view_matches() {
        check(ee::s09::f_002ed490__generated, world_painter_update_view, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2ed4, 12);
            let view = seed % 7;
            // The nine painters of every view: virtual 0x10 is a fake.
            let vt = DATA + 0x500;
            c.bus.write16(vt + 0x10, 0);
            c.bus.write32(vt + 0x14, FAKE);
            for v in 0..6u32 {
                let rec = WP_VIEWS + WP_VIEW_SIZE * v;
                for i in 0..9u32 {
                    let painter = DATA + 0x600 + 0x10 * i;
                    c.bus.write32(painter + 4, vt);
                    c.bus.write32(rec + 4 * i, painter);
                }
                c.bus.write32(rec + 0x24, f(uf(&mut r, 0.0, 1.0)));
                fill(c, &mut r, rec + 0x50, 0x28);
            }
            c.f[12] = f(uf(&mut r, -100.0, 100.0));
            c.f[13] = f(uf(&mut r, -100.0, 100.0));
            c.f[14] = f(uf(&mut r, -100.0, 100.0));
            // World, sections, riders.
            let (world, x, y, z, sections, riders, rider, board) = (
                DATA + 0x1_0000,
                DATA + 0x1_0100,
                DATA + 0x1_0200,
                DATA + 0x1_0300,
                DATA + 0x1_0400,
                DATA + 0x1_0500,
                DATA + 0x1_1000,
                DATA + 0x1_2000,
            );
            c.bus.write32(gp_at(0xa6c), seed / 7 % 2);
            c.bus.write32(gp_at(0xa74), 0);
            c.bus.write32(gp_at(0xa7c), r.next_u32() % 3);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, x);
            c.bus.write32(x + 0x78, y);
            c.bus.write32(y + 0x1bc, r.next_u32() % 4);
            c.bus.write32(x + 0x10, z);
            c.bus.write32(z, sections);
            for i in 0..16 {
                c.bus.write32(sections + 8 * i + 0x24, if r.next_u32() % 2 == 0 { 6 } else { 2 });
            }
            c.bus.write32(x + 0xc, riders);
            for v in 0..6u32 {
                c.bus.write32(riders + 4 * v + 0x28, rider);
            }
            for off in [-0x3974, -0x3970, -0x396c, -0x3968, -0x3964, -0x3960] {
                c.bus.write32(gp_at(off), f(uf(&mut r, 0.0, 1.0)));
            }
            c.bus.write32(gp_at(0xa80), f(uf(&mut r, 0.0, 1.0)));
            c.bus.write32(rider + 0x3fc, r.next_u32() % 2);
            let surface = if r.next_u32() % 4 == 0 { u32::MAX } else { (r.next_u32() % 2) << 8 | r.next_u32() % 2 };
            c.bus.write32(rider + 0x430, surface);
            c.bus.write32(rider + 0x788, board);
            c.bus.write32(rider + 0x77c, DATA + 0x1_1800);
            c.bus.write32(DATA + 0x1_1800 + 0xde0, [1u32, 4, 2][(seed / 2 % 3) as usize]);
            put_vec3(c, &mut r, rider + 0xaac, -50.0, 50.0);
            c.bus.write32(board + 0xac, [1u32, 3, 2][(seed / 3 % 3) as usize]);
            c.bus.write32(board + 0x30, if seed % 5 == 0 { u32::MAX } else { surface & 0x1ff });
            let span = uf(&mut r, 0.0, 12.0);
            c.bus.write32(board + 0x98, f(span));
            c.bus.write32(board + 0xa0, f(uf(&mut r, -2.0, span)));
            c.bus.write32(board + 0x34, f(uf(&mut r, -50.0, 50.0)));
            c.bus.write32(board + 0x38, f(uf(&mut r, -50.0, 50.0)));
            // Painter layers (gp+0x16c8): layer table, tiles, tile types.
            let (w2, layer_list, table, layer, tiles, tile) = (
                DATA + 0x1_3000,
                DATA + 0x1_3100,
                DATA + 0x1_3800,
                DATA + 0x1_3900,
                DATA + 0x1_3a00,
                DATA + 0x1_4000,
            );
            c.bus.write32(gp_at(0x16c8), w2);
            c.bus.write32(w2, layer_list);
            c.bus.write32(layer_list + 8, table);
            for i in 0..2 {
                c.bus.write32(table + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { layer });
                c.bus.write32(tiles + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { (tile >> 2) << 8 });
            }
            c.bus.write32(layer + 0xc, tiles);
            c.bus.write16(tile + 0x156, if r.next_u32() % 4 == 0 { 0xffff } else { (r.next_u32() % 3) as u16 });
            c.bus.write8(tile + 0x155, (r.next_u32() % 3) as u8);
            for i in 0..3 {
                c.bus.write32(layer_list + 8 * i + 0x24, if r.next_u32() % 2 == 0 { 6 } else { 1 });
                c.bus.write32(layer_list + 0x18 * i + 0x3f0, if r.next_u32() % 2 == 0 { 3 } else { 0 });
            }
            c.g[A0][0] = view as u64;
        });
    }
}
