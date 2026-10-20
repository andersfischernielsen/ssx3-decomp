//! Phase 2 wave 3 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for DMA waits, VU0 uploads, view-colour lerps, the pool
//! allocator's free path, rider boost/milestone bookkeeping, 4x4 adjugates,
//! grid-mesh vertex packing and the occluder list, each with a differential
//! test against the generated body.
//!
//! The replacements keep their state in the guest registers the originals
//! use where callers, callees or event handlers can see them, follow the call
//! protocol of [`crate::call`] and issue VU0 macro operations exactly as the
//! generated code does.

use crate::abi::{self, r32};
use crate::call::{call_ptr, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_f768, dma_wait_from_scratchpad, "Dma_WaitFromScratchpad"),
    (0x0022_9f88, vu0_upload_occluder, "Vu0_UploadOccluder"),
    (0x002e_d338, world_painter_lerp_view_colours, "WorldPainter_LerpViewColours"),
    (0x0031_9f68, mem_pool_free, "MEM_PoolFree"),
    (0x0011_9210, rider_metrix_check_score_milestone, "RiderMetrix_CheckScoreMilestone"),
    (0x0010_e098, rider_add_boost, "Rider_AddBoost"),
    (0x0031_b310, mat4_adjoint, "Mat4_Adjoint"),
    (0x0038_52e8, c_ps2_grid_mesh_v03, "cPS2GridMesh_v03"),
    (0x0024_25c0, occlusion_man_build_visible, "OcclusionMan_BuildVisible"),
    (0x0013_9a20, rider_air_state_update, "RiderAirState_Update"),
    (0x0013_a7b0, rider_air_state_cast_board_ray, "RiderAirState_CastBoardRay"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    /// `MEM_PoolNNFree(base, block)`: push the block onto the size class's
    /// free list.
    pub const POOL8_FREE: Callee = Callee { addr: 0x0032_0488, f: ee::s10::MEM_Pool8Free };
    pub const POOL16_FREE: Callee = Callee { addr: 0x0032_0498, f: ee::s10::MEM_Pool16Free };
    pub const POOL32_FREE: Callee = Callee { addr: 0x0032_04a8, f: ee::s10::MEM_Pool32Free };
    pub const POOL64_FREE: Callee = Callee { addr: 0x0032_04b8, f: ee::s10::MEM_Pool64Free };
    pub const POOL128_FREE: Callee = Callee { addr: 0x0032_04d0, f: ee::s10::MEM_Pool128Free };
    pub const POOL256_FREE: Callee = Callee { addr: 0x0032_04e8, f: ee::s10::MEM_Pool256Free };
    pub const POOL512_FREE: Callee = Callee { addr: 0x0032_0500, f: ee::s10::MEM_Pool512Free };
    pub const REMOVE_HUD_MESSAGE: Callee =
        Callee { addr: 0x0011_79e0, f: ee::s00::RiderMetrix_RemoveHUDMessage };
    pub const SET_TIMED_EVENT: Callee = Callee { addr: 0x0010_e028, f: ee::s00::Rider_SetTimedEvent };
    pub const PACK_POSITION: Callee = Callee { addr: 0x0038_5a38, f: ee::s12::cPS2GridMesh_PackPosition };
    pub const BUILD_OCCLUDER_PLANES: Callee =
        Callee { addr: 0x0024_29b0, f: ee::s06::OcclusionMan_BuildOccluderPlanes };
    pub const MATH_DET3X3: Callee = Callee { addr: 0x0031_b098, f: ee::s10::Math_Det3x3 };
    /// Cross-shard calls the generator emitted through the function table.
    pub const SHOW_HUD_NUMBER: u32 = 0x0011_7b88;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_CHAR_ATTRIB_FACTOR: u32 = 0x0014_9690;
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_RESET_BOOST_PITCH: u32 = 0x0029_ab08;
    /// `qsort`
    pub const QSORT: u32 = 0x0041_8ef8;
    pub const VU0_UPLOAD_OCCLUDER: u32 = 0x0022_9f88;
    pub const AIR_APPLY_WIND: u32 = 0x0012_5970;
    pub const AIR_PREDICTOR_UPDATE: u32 = 0x0011_3648;
    pub const RIDER_GET_SUBSTATE: u32 = 0x0011_fee8;
    pub const RIDER_UPDATE_ORIENTATION: u32 = 0x0012_1aa0;
    pub const RIDER_ORIENTATION_IMPLICIT: u32 = 0x0011_e098;
    pub const COLL_SEGMENT_CTOR: u32 = 0x0032_e100;
    pub const OVERLAP_LIST_FIND_CLOSEST: u32 = 0x0033_42d0;
}

// ---------------------------------------------------------------------------
// Small helpers.

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

/// FPU register `f`.
#[inline(always)]
fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// `c.le.s a, b` leaving the condition bit set.
#[inline(always)]
fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitFromScratchpad(void)` (0x38f768): spin until the DMA
/// controller's "from scratchpad" channel (D_STAT bit 8) is idle.
pub fn dma_wait_from_scratchpad(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] = c.g[V0][0] | 0xd000;
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = c.g[V1][0] & 0x100;
    let idle = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !idle {
        c.cycles += 2;
        c.g[V1][0] = c.g[V1][0] | 0xd000;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = c.g[V0][0] & 0x100;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_f788);
            }
        }
    }
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// VU0.

/// `void Vu0_UploadOccluder(int index, const Vec4* v, const Mat4* m)`
/// (0x229f88): copy the vertex and the four matrix rows into VU0 data memory
/// at quadword `index * 5 + 0x50` (the occluder plane slot of a view).
pub fn vu0_upload_occluder(c: &mut Cpu) {
    c.cycles += 16;
    c.g[V0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 0)));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
    c.ctc2(2, 1);
    let v = abi::arg(c, 1);
    let m = abi::arg(c, 2);
    c.lqc2(1, v);
    c.lqc2(2, m);
    c.lqc2(3, p(m, 0x10));
    c.lqc2(4, p(m, 0x20));
    c.lqc2(5, p(m, 0x30));
    c.vu0().sqi(0b1111, 1, 1);
    c.vu0().sqi(0b1111, 2, 1);
    c.vu0().sqi(0b1111, 3, 1);
    c.vu0().sqi(0b1111, 4, 1);
    c.vu0().sqi(0b1111, 5, 1);
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// World painter view colours.

/// `void WorldPainter_LerpViewColours(WorldPainter* self, const Vec4* a,
/// const Vec4* b, float t)` (0x2ed338): `self->colour0/1 = old * (1 - t) +
/// new * t` (two 16-byte colour quads at self+0x28 and self+0x38).
pub fn world_painter_lerp_view_colours(c: &mut Cpu) {
    c.cycles += 86;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    let this = abi::arg(c, 0);
    let a = abi::arg(c, 1);
    let b = abi::arg(c, 2);
    // s = 1 - t, kept in f1.
    c.lwc1(5, p(this, 0x28));
    c.sub_s(1, 1, 12);
    c.lwc1(4, p(this, 0x2c));
    c.lwc1(3, p(this, 0x30));
    c.lwc1(2, p(this, 0x34));
    c.lwc1(8, p(a, 0x0c));
    c.mul_s(5, 5, 1);
    c.lwc1(0, a);
    c.mul_s(4, 4, 1);
    c.lwc1(7, p(a, 0x04));
    c.mul_s(3, 3, 1);
    c.lwc1(6, p(a, 0x08));
    c.mul_s(2, 2, 1);
    c.mul_s(8, 8, 12);
    c.swc1(5, p(sp(c), 0x10));
    c.mul_s(0, 0, 12);
    c.swc1(4, p(sp(c), 0x14));
    c.mul_s(7, 7, 12);
    c.swc1(3, p(sp(c), 0x18));
    c.mul_s(6, 6, 12);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 0);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(4, 4, 7);
    c.swc1(7, p(sp(c), 0x24));
    c.add_s(3, 3, 6);
    c.swc1(6, p(sp(c), 0x28));
    c.add_s(2, 2, 8);
    c.swc1(5, sp(c));
    c.swc1(4, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x2f));
    c.sdr(2, p(this, 0x28));
    c.sdl(3, p(this, 0x37));
    c.sdr(3, p(this, 0x30));
    // Second colour quad.
    c.lwc1(4, p(this, 0x38));
    c.lwc1(5, p(this, 0x3c));
    c.lwc1(3, p(this, 0x40));
    c.mul_s(4, 4, 1);
    c.lwc1(2, p(this, 0x44));
    c.mul_s(5, 5, 1);
    c.lwc1(8, p(b, 0x0c));
    c.mul_s(3, 3, 1);
    c.lwc1(0, b);
    c.mul_s(2, 2, 1);
    c.lwc1(7, p(b, 0x04));
    c.mul_s(8, 8, 12);
    c.lwc1(6, p(b, 0x08));
    c.mul_s(0, 0, 12);
    c.mul_s(7, 7, 12);
    c.swc1(4, p(sp(c), 0x10));
    c.mul_s(6, 6, 12);
    c.swc1(5, p(sp(c), 0x14));
    c.swc1(3, p(sp(c), 0x18));
    c.add_s(4, 4, 0);
    c.swc1(2, p(sp(c), 0x1c));
    c.add_s(5, 5, 7);
    c.add_s(3, 3, 6);
    c.swc1(0, p(sp(c), 0x20));
    c.add_s(2, 2, 8);
    c.swc1(4, sp(c));
    c.swc1(5, p(sp(c), 0x04));
    c.swc1(3, p(sp(c), 0x08));
    c.swc1(2, p(sp(c), 0x0c));
    c.swc1(7, p(sp(c), 0x24));
    c.swc1(6, p(sp(c), 0x28));
    c.swc1(8, p(sp(c), 0x2c));
    c.ldl(2, p(sp(c), 0x07));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0x0f));
    c.ldr(3, p(sp(c), 0x08));
    c.sdl(2, p(this, 0x3f));
    c.sdr(2, p(this, 0x38));
    c.sdl(3, p(this, 0x47));
    c.sdr(3, p(this, 0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Pool allocator.

/// `gp`-relative offsets of the pool free-list heads, smallest first.
mod pool {
    pub const P8: i32 = 0x2a70;
    pub const P16: i32 = 0x2a60;
    pub const P32: i32 = 0x2a6c;
    pub const P64: i32 = 0x2a7c;
    pub const P128: i32 = 0x2a84;
    pub const P256: i32 = 0x2a78;
    pub const P512: i32 = 0x2a68;
}

/// `void MEM_PoolFree(void* block)` (0x319f68): find the pool whose base lies
/// below `block` within the size class's address range and push the block
/// onto that pool's free list. Blocks outside every range are ignored.
pub fn mem_pool_free(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = c.lw(gp(c, pool::P8));
    c.g[A1][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
    c.g[V0][0] = (c.g[V0][0] < 0x800) as u64;
    let outside = c.g[V0][0] == 0;
    c.sd(sp(c), c.g[RA][0]);
    if !outside {
        c.cycles += 4;
        c.g[A0][0] = c.g[V1][0];
        if callee::POOL8_FREE.call(c, 0x0031_9f8c).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    // 16-byte class.
    c.cycles += 5;
    c.g[A0][0] = c.lw(gp(c, pool::P16));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V0][0] < 0x2000) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL16_FREE.call(c, 0x0031_9fb0).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P32));
    // 32-byte class.
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V0][0] < 0x6000) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL32_FREE.call(c, 0x0031_9fd0).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P64));
    // 64-byte class: `block - base <= 0x31fff`.
    c.cycles += 7;
    c.g[V0][0] = 0x3_0000;
    c.g[V0][0] = c.g[V0][0] | 0x1fff;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL64_FREE.call(c, 0x0031_9ffc).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P128));
    // 128-byte class: `block - base <= 0x257ff`.
    c.cycles += 7;
    c.g[V0][0] = 0x2_0000;
    c.g[V0][0] = c.g[V0][0] | 0x57ff;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL128_FREE.call(c, 0x0031_a028).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P256));
    // 256-byte class: `block - base <= 0x1ffff`.
    c.cycles += 7;
    c.g[V1][0] = 0x1_0000;
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        if callee::POOL256_FREE.call(c, 0x0031_a054).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
        return mem_pool_free_done(c);
    }
    c.g[A0][0] = c.lw(gp(c, pool::P512));
    // 512-byte class (and the fall-through when the block is below every
    // pool base, where the unsigned difference reads as huge).
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    let outside = c.g[V0][0] == 0;
    c.g[RA][0] = c.ld(sp(c));
    if !outside {
        c.cycles += 3;
        if callee::POOL512_FREE.call(c, 0x0031_a078).is_err() {
            return;
        }
        c.g[RA][0] = c.ld(sp(c));
    }
    mem_pool_free_done(c);
}

/// The shared `addiu sp, sp, 0x10; jr ra` of [`mem_pool_free`].
#[inline(always)]
fn mem_pool_free_done(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Rider metric milestones.

/// Rider fields the milestone check touches.
mod rider {
    /// Current boost amount (0..1), scaled for the HUD.
    pub const BOOST: u32 = 0x2f8;
    /// Boost HUD timer and its countdown.
    pub const BOOST_TIMER: u32 = 0x2f0;
    pub const BOOST_COUNT: u32 = 0x2f4;
    /// Boost state: 3 means "no boost HUD".
    pub const BOOST_STATE: u32 = 0x304;
    pub const BOOST_MASK: u32 = 0xb28;
    pub const BOOST_FLAG: u32 = 0xb2c;
    pub const CHAR_ID: u32 = 0x86c;
    pub const CHAR_COMP: u32 = 0xb34;
    /// Metric multiplier used for milestone scores.
    pub const METRIC_SCALE: u32 = 0x1c4;
}

/// `int RiderMetrix_CheckScoreMilestone(Rider* self, int* idx,
/// const GradeStep* grades, int type)` (0x119210): when the step at `*idx`
/// has been reached, advance the index, drop the five HUD messages of the
/// previous tier and show the new score; returns the awarded value or 0.
pub fn rider_metrix_check_score_milestone(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x50);
    let idx = abi::arg(c, 1);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    c.mtc1(0, 0);
    c.g[V1][0] = c.lw(idx);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(abi::arg(c, 2)));
    c.lwc1(20, r32(c, V0));
    c.c_lt_s(20, 0);
    let below_zero = c.fcc();
    c.g[S2][0] = c.g[A3][0];
    if !below_zero {
        c.cycles += 4;
        c.c_le_s(20, 12);
        if c.fcc() {
            // The step's threshold has been reached: award the milestone.
            c.lwc1(1, p(r32(c, S1), rider::METRIC_SCALE));
            c.cycles += 33;
            c.g[A0][0] = c.g[S1][0];
            c.lwc1(0, p(r32(c, V0), 0x04));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mul_s(0, 0, 1);
            c.sw(idx, c.g[V0][0]);
            c.add_s(0, 0, 2);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            // Drop the five HUD messages of the old tier.
            c.g[A1][0] = 0x1c;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_929c).is_err() {
                return;
            }
            c.g[A1][0] = 0x1d;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92a8).is_err() {
                return;
            }
            c.g[A1][0] = 0x1e;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92b4).is_err() {
                return;
            }
            c.g[A1][0] = 0x1f;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92c0).is_err() {
                return;
            }
            c.g[A1][0] = 0x20;
            c.g[A0][0] = c.g[S1][0];
            if callee::REMOVE_HUD_MESSAGE.call(c, 0x0011_92cc).is_err() {
                return;
            }
            c.cvt_w_s(0, 20);
            c.mfc1(7, 0);
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            if call_ptr(c, callee::SHOW_HUD_NUMBER, 0x0011_92ec).is_err() {
                return;
            }
            c.g[V0][0] = c.g[S0][0];
            return rider_metrix_leave(c, &frame);
        }
    }
    // Threshold negative, or not reached yet.
    c.cycles += 2;
    c.g[V0][0] = 0;
    rider_metrix_leave(c, &frame);
}

/// The epilogue of [`rider_metrix_check_score_milestone`].
#[inline(always)]
fn rider_metrix_leave(c: &mut Cpu, frame: &Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Boost.

/// `void Rider_AddBoost(Rider* self, float amt, uint mask)` (0x10e098): add to
/// the boost meter (clamped to 0..1), tick the boost HUD timer and, when the
/// meter fills, start the 20-second boost countdown.
pub fn rider_add_boost(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x40);
    c.g[V0][0] = 3;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x30);
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    frame.save_fpr(c, 21, 0x38);
    let this = r32(c, S0);
    c.g[V1][0] = c.lw(p(this, rider::BOOST_STATE));
    let no_hud = c.g[V1][0] == c.g[V0][0];
    c.mov_s(20, 12);
    if !no_hud {
        c.cycles += 5;
        c.mtc1(0, 21);
        c.c_eq_s(20, 21);
        if c.fcc() {
            return rider_add_boost_leave_early(c, &frame);
        }
        c.cycles += 3;
        c.c_lt_s(21, 20);
        if c.fcc() {
            // Adding boost: report the character stat and require the mask.
            c.cycles += 13;
            if call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0010_e0f0).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 3;
            if call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0010_e0fc).is_err() {
                return;
            }
            c.g[A1][0] = c.lw(p(this, rider::CHAR_ID));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw(p(this, rider::CHAR_COMP));
            if call_ptr(c, callee::BE_CHAR_ATTRIB_FACTOR, 0x0010_e10c).is_err() {
                return;
            }
            c.g[V0][0] = c.lw(p(this, rider::BOOST_MASK));
            c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                return rider_add_boost_leave_early(c, &frame);
            }
        }
    } else {
        // `state == 3` returns without touching the meter (`break 'b_10e208`).
        return rider_add_boost_leave_late(c, &frame);
    }
    // Common tail of the meter update.
    c.cycles += 15;
    if call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x0010_e124).is_err() {
        return;
    }
    c.lwc1(12, p(this, rider::BOOST));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x0010_e138);
    c.mov_s(13, 20);
    if call_ptr(c, callee::AUDIO_RESET_BOOST_PITCH, 0x0010_e138).is_err() {
        return;
    }
    c.lwc1(0, p(this, rider::BOOST));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.add_s(0, 0, 20);
    c.c_le_s(21, 0);
    let below = !c.fcc();
    c.mov_s(2, 21);
    if !below {
        c.cycles += 1;
        c.min_s(2, 0, 1);
    }
    // Clamped meter value in f2.
    'meter: {
        c.cycles += 4;
        c.c_eq_s(2, 1);
        let full = !c.fcc();
        c.swc1(2, p(this, rider::BOOST));
        if full {
            break 'meter;
        }
        c.cycles += 3;
        c.c_lt_s(21, 20);
        if !c.fcc() {
            break 'meter;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(this, rider::BOOST_FLAG));
        if c.g[V0][0] == 0 {
            break 'meter;
        }
        // Start the timed boost event once, then run the countdown.
        'timer: {
            c.cycles += 5;
            c.lwc1(0, p(this, rider::BOOST_TIMER));
            c.c_eq_s(0, 21);
            let running = !c.fcc();
            c.mov_s(12, 21);
            if running {
                break 'timer;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 5;
            if callee::SET_TIMED_EVENT.call(c, 0x0010_e1a4).is_err() {
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
        if (c.g[V0][0] as i64) > 0 {
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(p(this, rider::BOOST_COUNT), c.g[V0][0]);
            c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        if c.g[V0][0] == 0 {
            return rider_add_boost_leave_early(c, &frame);
        }
        c.cycles += 5;
        c.lwc1(0, p(this, rider::BOOST_TIMER));
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 1);
        c.max_s(0, 0, 1);
        c.cycles += 1;
        c.swc1(0, p(this, rider::BOOST_TIMER));
        return rider_add_boost_leave_late(c, &frame);
    }
    // Not full: the meter drains to zero while boost is being removed.
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(20, 0);
    if !c.fcc() {
        return rider_add_boost_leave_early(c, &frame);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(this, rider::BOOST_COUNT));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.swc1(0, p(this, rider::BOOST_TIMER));
        return rider_add_boost_leave_late(c, &frame);
    }
    rider_add_boost_leave_early(c, &frame);
}

/// `lq s0; jr ra` epilogue of [`rider_add_boost`] (the path that restores s0
/// itself).
#[inline(always)]
fn rider_add_boost_leave_early(c: &mut Cpu, frame: &Frame) {
    let a = frame.at(c, 0x20);
    c.lq(16, a);
    rider_add_boost_finish(c, frame);
}

/// The `break 'b_10e208` epilogue of [`rider_add_boost`]: one extra cycle
/// charged for the restore of s0.
#[inline(always)]
fn rider_add_boost_leave_late(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    let a = frame.at(c, 0x20);
    c.lq(16, a);
    rider_add_boost_finish(c, frame);
}

#[inline(always)]
fn rider_add_boost_finish(c: &mut Cpu, frame: &Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 21, 0x38);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// 4x4 adjugate.

/// `void Mat4_Adjoint(Mat4* m)` (0x31b310): replace `m` with the transpose of
/// its cofactor matrix (the adjugate), built from sixteen 3x3 determinants.
pub fn mat4_adjoint(c: &mut Cpu) {
    c.cycles += 237;
    let frame = Frame::open(c, 0xa0);
    let m = abi::arg(c, 0);
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 31, 0x98);
    frame.save_fpr(c, 30, 0x90);
    frame.save_fpr(c, 29, 0x88);
    frame.save_fpr(c, 28, 0x80);
    frame.save_fpr(c, 27, 0x78);
    frame.save_fpr(c, 26, 0x70);
    frame.save_fpr(c, 25, 0x68);
    frame.save_fpr(c, 24, 0x60);
    frame.save_fpr(c, 23, 0x58);
    frame.save_fpr(c, 22, 0x50);
    frame.save_fpr(c, 21, 0x48);
    frame.save_fpr(c, 20, 0x40);
    // Load the cofactor operands into the argument registers f12..f19 and
    // keep spares on the frame.
    c.lwc1(0, p(m, 0x14));
    c.swc1(0, p(sp(c), 0x10));
    c.mov_s(12, 0);
    c.lwc1(1, p(m, 0x18));
    c.swc1(1, p(sp(c), 0x18));
    c.mov_s(15, 1);
    c.lwc1(0, p(m, 0x24));
    c.lwc1(30, p(m, 0x1c));
    c.swc1(0, p(sp(c), 0x14));
    c.mov_s(13, 0);
    c.mov_s(18, 30);
    c.lwc1(1, p(m, 0x28));
    c.swc1(1, p(sp(c), 0x1c));
    c.mov_s(16, 1);
    c.lwc1(20, p(m, 0x3c));
    c.lwc1(25, p(m, 0x2c));
    c.lwc1(23, p(m, 0x34));
    c.lwc1(22, p(m, 0x38));
    c.mov_s(19, 25);
    c.lwc1(28, m);
    c.mov_s(14, 23);
    c.swc1(20, sp(c));
    c.mov_s(17, 22);
    c.lwc1(29, p(m, 0x04));
    c.lwc1(31, p(m, 0x08));
    c.lwc1(24, p(m, 0x0c));
    c.lwc1(26, p(m, 0x10));
    c.lwc1(27, p(m, 0x20));
    c.lwc1(21, p(m, 0x30));
    if callee::MATH_DET3X3.call(c, 0x0031_b3c8).is_err() {
        return;
    }
    c.swc1(0, m);
    c.mov_s(12, 26);
    c.swc1(20, sp(c));
    c.mov_s(13, 27);
    c.lwc1(15, p(sp(c), 0x18));
    c.mov_s(14, 21);
    c.lwc1(16, p(sp(c), 0x1c));
    c.mov_s(17, 22);
    c.mov_s(18, 30);
    c.mov_s(19, 25);
    if callee::MATH_DET3X3.call(c, 0x0031_b3f4).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.lwc1(15, p(sp(c), 0x10));
    c.mov_s(12, 26);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.swc1(0, p(m, 0x10));
    c.mov_s(17, 23);
    c.mov_s(18, 30);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x14));
    if callee::MATH_DET3X3.call(c, 0x0031_b424).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x20));
    c.mov_s(12, 26);
    c.swc1(22, sp(c));
    c.mov_s(13, 27);
    c.lwc1(15, p(sp(c), 0x10));
    c.mov_s(14, 21);
    c.lwc1(16, p(sp(c), 0x14));
    c.mov_s(17, 23);
    c.lwc1(18, p(sp(c), 0x18));
    c.lwc1(19, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b450).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.lwc1(13, p(sp(c), 0x14));
    c.mov_s(12, 29);
    c.mov_s(14, 23);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x30));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b480).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x04));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    c.lwc1(16, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b4b0).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x14));
    c.mov_s(12, 28);
    c.swc1(20, sp(c));
    c.mov_s(13, 27);
    c.lwc1(16, p(sp(c), 0x14));
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.mov_s(17, 23);
    c.mov_s(18, 24);
    c.mov_s(19, 25);
    if callee::MATH_DET3X3.call(c, 0x0031_b4dc).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(22, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 27);
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.swc1(0, p(m, 0x24));
    c.mov_s(17, 23);
    c.mov_s(18, 31);
    c.lwc1(16, p(sp(c), 0x14));
    c.lwc1(19, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b50c).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x34));
    c.mov_s(12, 29);
    c.swc1(20, sp(c));
    c.mov_s(14, 23);
    c.lwc1(13, p(sp(c), 0x10));
    c.mov_s(15, 31);
    c.lwc1(16, p(sp(c), 0x18));
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    if callee::MATH_DET3X3.call(c, 0x0031_b538).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x08));
    c.mov_s(12, 28);
    c.swc1(20, sp(c));
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x18));
    c.mov_s(14, 21);
    c.mov_s(15, 31);
    c.mov_s(17, 22);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    if callee::MATH_DET3X3.call(c, 0x0031_b564).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(20, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.mov_s(14, 21);
    c.mov_s(15, 29);
    c.swc1(0, p(m, 0x18));
    c.mov_s(17, 23);
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x10));
    if callee::MATH_DET3X3.call(c, 0x0031_b594).is_err() {
        return;
    }
    c.swc1(22, sp(c));
    c.mov_s(14, 21);
    c.swc1(0, p(m, 0x28));
    c.mov_s(17, 23);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x10));
    c.mov_s(15, 29);
    c.lwc1(19, p(sp(c), 0x18));
    c.mov_s(18, 31);
    if callee::MATH_DET3X3.call(c, 0x0031_b5c0).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(25, sp(c));
    c.lwc1(13, p(sp(c), 0x10));
    c.mov_s(12, 29);
    c.lwc1(14, p(sp(c), 0x14));
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x38));
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x18));
    c.lwc1(17, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b5f0).is_err() {
        return;
    }
    c.neg_s(0, 0);
    c.swc1(25, sp(c));
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.mov_s(14, 27);
    c.mov_s(15, 31);
    c.swc1(0, p(m, 0x0c));
    c.mov_s(18, 24);
    c.mov_s(19, 30);
    c.lwc1(16, p(sp(c), 0x18));
    c.lwc1(17, p(sp(c), 0x1c));
    if callee::MATH_DET3X3.call(c, 0x0031_b620).is_err() {
        return;
    }
    c.swc1(25, sp(c));
    c.mov_s(18, 24);
    c.swc1(0, p(m, 0x1c));
    c.mov_s(19, 30);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.lwc1(16, p(sp(c), 0x10));
    c.mov_s(14, 27);
    c.lwc1(17, p(sp(c), 0x14));
    c.mov_s(15, 29);
    if callee::MATH_DET3X3.call(c, 0x0031_b64c).is_err() {
        return;
    }
    c.lwc1(1, p(sp(c), 0x1c));
    c.neg_s(0, 0);
    c.mov_s(12, 28);
    c.mov_s(13, 26);
    c.swc1(1, sp(c));
    c.mov_s(14, 27);
    c.swc1(0, p(m, 0x2c));
    c.mov_s(15, 29);
    c.mov_s(18, 31);
    c.lwc1(16, p(sp(c), 0x10));
    c.lwc1(17, p(sp(c), 0x14));
    c.lwc1(19, p(sp(c), 0x18));
    if callee::MATH_DET3X3.call(c, 0x0031_b680).is_err() {
        return;
    }
    c.swc1(0, p(m, 0x3c));
    frame.restore(c, S0, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 31, 0x98);
    frame.restore_fpr(c, 30, 0x90);
    frame.restore_fpr(c, 29, 0x88);
    frame.restore_fpr(c, 28, 0x80);
    frame.restore_fpr(c, 27, 0x78);
    frame.restore_fpr(c, 26, 0x70);
    frame.restore_fpr(c, 25, 0x68);
    frame.restore_fpr(c, 24, 0x60);
    frame.restore_fpr(c, 23, 0x58);
    frame.restore_fpr(c, 22, 0x50);
    frame.restore_fpr(c, 21, 0x48);
    frame.restore_fpr(c, 20, 0x40);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Grid mesh.

/// `cPS2GridMesh` fields used when packing vertices.
mod grid_mesh {
    /// Number of quads across a row.
    pub const WIDTH: u32 = 0x190;
    /// Scale applied to every position.
    pub const SCALE: u32 = 0x198;
    /// Which row table (`+0x1b4` / `+0x1b8`) is current.
    pub const TAB_INDEX: u32 = 0x1a0;
    /// Set to 1 once the mesh has been packed.
    pub const PACKED: u32 = 0x1a4;
    /// Row tables (2 x pointers) and the row count.
    pub const ROW_TABS: u32 = 0x1b4;
    pub const ROWS: u32 = 0x1bc;
}

/// `void cPS2GridMesh_v03(cPS2GridMesh* self, const Vec3* verts)`
/// (0x3852e8): pack every row of the mesh's current vertex grid into its
/// output stream (two vertices per quad), then mark the mesh packed.
pub fn c_ps2_grid_mesh_v03(c: &mut Cpu) {
    c.cycles += 18;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S2, 0x80);
    frame.save(c, S7, 0x30);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0xa0);
    c.g[S7][0] = c.g[A1][0];
    frame.save(c, S1, 0x90);
    c.g[A1][0] = 0;
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    let this = r32(c, S2);
    c.g[V1][0] = c.lw(p(this, grid_mesh::TAB_INDEX));
    c.g[V0][0] = c.lw(p(this, grid_mesh::ROWS));
    let no_rows = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = (c.g[V1][0] < 1) as u64;
    if !no_rows {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(this.wrapping_add(grid_mesh::ROW_TABS));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[FP][0] = 0xc;
        c.sw(sp(c), c.g[V0][0]);
        c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
        loop {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[S6][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = c.lw(sp(c));
            c.mult(0, 5, 5, 2);
            c.g[S5][0] = 0;
            c.g[V1][0] = c.lw(r32(c, A2));
            c.g[S3][0] = 0;
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let empty_row = c.g[V0][0] == 0;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            if !empty_row {
                c.cycles += 4;
                c.mult(0, 2, 7, 30);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                c.mult(0, 2, 5, 30);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                loop {
                    c.cycles += 17;
                    c.g[A1][0] = c.lw(p(r32(c, S4), 0x04));
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    if callee::PACK_POSITION.call(c, 0x0038_5394).is_err() {
                        return;
                    }
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.g[A1][0] = c.lw(p(r32(c, S4), 0x04));
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S2][0];
                    if callee::PACK_POSITION.call(c, 0x0038_53ac).is_err() {
                        return;
                    }
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(2));
                    c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x0c));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x0c));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_5380);
                    }
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(p(this, grid_mesh::ROWS));
            c.g[A1][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lhu(p(this, grid_mesh::WIDTH));
            if event_due(c) {
                deliver_event_here(c, 0x0038_5348);
            }
        }
    }
    c.cycles += 14;
    c.g[V0][0] = 1;
    frame.restore(c, S0, 0xa0);
    c.sw(p(this, grid_mesh::PACKED), c.g[V0][0]);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Occlusion manager.

/// `OcclusionMan` fields.
mod occl {
    /// Non-zero when the manager collects views at all.
    pub const ENABLED: u32 = 0x04;
    /// The visible list: `{ const View* view; float dist; }` entries.
    pub const ENTRIES: u32 = 0x08;
    /// Number of entries collected this frame.
    pub const COUNT: u32 = 0x108;
    /// Number of views that get occluder planes (at most two).
    pub const N_VISIBLE: u32 = 0x10c;
    /// Bit `i` set once view `i` was uploaded.
    pub const MASK: u32 = 0x110;
}

/// `View` fields used here: the position (w = 1) and the outward normal.
mod view {
    pub const POS: u32 = 0x00;
    pub const NORMAL: u32 = 0x50;
    pub const MIN: u32 = 0xa0;
    pub const MAX: u32 = 0xac;
    /// Next view in the manager's list.
    pub const NEXT: u32 = 0xbc;
    /// The four occluder planes the upload writes.
    pub const PLANES: u32 = 0x60;
}

/// Camera singleton (`gp - 0x854`) and its vtable.
const CAMERA: i32 = -0x854;
const CAMERA_VT: u32 = 0x10d8;

/// `int OcclusionMan_BuildVisible(OcclusionMan* self, const Box* bb)`
/// (0x2425c0): collect the views whose box overlaps `bb` and that are in
/// front of the camera, sort them by distance and build the occluder planes
/// of the nearest two.
pub fn occlusion_man_build_visible(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0xd0);
    frame.save(c, S2, 0x90);
    frame.save(c, S3, 0x80);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0xb0);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S1, 0xa0);
    frame.save_ra(c, 0x70);
    frame.save_fpr(c, 20, 0xc0);
    let this = r32(c, S2);
    let bbox = r32(c, S3);
    c.sw(p(this, occl::N_VISIBLE), 0);
    c.g[V0][0] = c.lw(p(this, occl::ENABLED));
    let disabled = c.g[V0][0] == 0;
    c.sw(p(this, occl::MASK), 0);
    if !disabled {
        c.cycles += 90;
        // The camera's matrix, written to the frame by its own method.
        c.g[A2][0] = c.lw(gp(c, CAMERA));
        c.g[A0][0] = c.g[SP][0];
        let cam = r32(c, A2);
        c.g[V0][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A1][0] = c.lh(p(r32(c, V0), 0x158));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x15c));
        let target = r32(c, V1);
        c.g[A1][0] = sx(cam.wrapping_add(r32(c, A1)));
        if call_ptr(c, target, 0x0024_2610).is_err() {
            return;
        }
        // The camera's row position, scaled by -1.
        c.g[A1][0] = c.lw(gp(c, CAMERA));
        let cam = r32(c, A1);
        c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A0][0] = c.lh(p(r32(c, V1), 0x118));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x11c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0024_2628).is_err() {
            return;
        }
        c.lq(3, p(r32(c, V0), 0x30));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.sq(3, p(sp(c), 0x50));
        c.mfc1(4, 0);
        c.lqc2(4, p(sp(c), 0x50));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p(sp(c), 0x60));
        c.lq(2, p(sp(c), 0x60));
        c.g[A1][0] = c.lw(gp(c, CAMERA));
        c.sq(2, p(sp(c), 0x40));
        let cam = r32(c, A1);
        c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A0][0] = c.lh(p(r32(c, V1), 0x118));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x11c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0024_266c).is_err() {
            return;
        }
        // Rotate the (negated) row position into world space and keep it on
        // the frame as the view position.
        c.lwc1(6, p(r32(c, V0), 0x00));
        c.lwc1(1, p(r32(c, V0), 0x04));
        c.lwc1(5, p(r32(c, V0), 0x10));
        c.lwc1(10, p(r32(c, V0), 0x14));
        c.lwc1(4, p(r32(c, V0), 0x30));
        c.lwc1(11, p(r32(c, V0), 0x34));
        c.lwc1(0, p(sp(c), 0x40));
        c.lwc1(2, p(sp(c), 0x44));
        c.lwc1(7, p(r32(c, V0), 0x20));
        c.mul_s(4, 0, 4);
        c.lwc1(15, p(r32(c, V0), 0x24));
        c.mul_s(1, 2, 1);
        c.mul_s(11, 2, 11);
        c.lwc1(13, p(r32(c, V0), 0x08));
        c.mul_s(6, 0, 6);
        c.lwc1(12, p(r32(c, V0), 0x18));
        c.mul_s(5, 0, 5);
        c.lwc1(9, p(r32(c, V0), 0x38));
        c.mul_s(10, 2, 10);
        c.lwc1(3, p(sp(c), 0x48));
        c.mul_s(0, 0, 7);
        c.lwc1(16, p(r32(c, V0), 0x28));
        c.mul_s(2, 2, 15);
        c.lwc1(14, p(r32(c, V0), 0x3c));
        c.add_s(6, 6, 1);
        c.lwc1(8, p(r32(c, V0), 0x0c));
        c.mul_s(9, 3, 9);
        c.lwc1(7, p(r32(c, V0), 0x1c));
        c.mul_s(13, 3, 13);
        c.lwc1(1, p(sp(c), 0x4c));
        c.mul_s(12, 3, 12);
        c.lwc1(15, p(r32(c, V0), 0x2c));
        c.add_s(5, 5, 10);
        c.sw(p(this, occl::COUNT), 0);
        c.add_s(0, 0, 2);
        c.g[S1][0] = c.lw(this);
        c.add_s(4, 4, 11);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 17);
        c.mul_s(3, 3, 16);
        c.mul_s(14, 1, 14);
        c.mul_s(8, 1, 8);
        c.mul_s(7, 1, 7);
        c.add_s(6, 6, 13);
        c.add_s(5, 5, 12);
        c.add_s(0, 0, 3);
        c.add_s(4, 4, 9);
        c.mul_s(1, 1, 15);
        c.add_s(6, 6, 8);
        c.add_s(5, 5, 7);
        c.add_s(0, 0, 1);
        c.add_s(4, 4, 14);
        c.swc1(6, p(sp(c), 0x50));
        c.swc1(5, p(sp(c), 0x54));
        c.swc1(0, p(sp(c), 0x58));
        c.swc1(4, p(sp(c), 0x5c));
        c.lq(2, p(sp(c), 0x50));
        c.sq(2, p(sp(c), 0x40));
        let no_views = c.g[S1][0] == 0;
        c.swc1(17, p(sp(c), 0x4c));
        if !no_views {
            c.cycles += 3;
            c.mov_s(20, 17);
            c.g[S0][0] = c.g[S1][0];
            loop {
                // The view's box against `bb`, all six planes.
                c.cycles += 19;
                c.lwc1(1, bbox);
                c.lwc1(3, p(r32(c, S0), view::MIN));
                c.g[V0][0] = 0;
                c.g[S1][0] = c.lw(p(r32(c, S1), view::NEXT));
                c.swc1(3, p(sp(c), 0x50));
                c.lwc1(4, p(r32(c, S0), view::MIN + 4));
                c.swc1(4, p(sp(c), 0x54));
                c.lwc1(5, p(r32(c, S0), view::MIN + 8));
                c.swc1(20, p(sp(c), 0x5c));
                c.swc1(5, p(sp(c), 0x58));
                c.lwc1(0, p(r32(c, S0), view::MAX));
                c.swc1(0, p(sp(c), 0x60));
                c.c_le_s(1, 0);
                c.lwc1(1, p(r32(c, S0), view::MAX + 4));
                c.swc1(1, p(sp(c), 0x64));
                c.lwc1(2, p(r32(c, S0), view::MAX + 8));
                c.swc1(20, p(sp(c), 0x6c));
                let miss = !c.fcc();
                c.swc1(2, p(sp(c), 0x68));
                'overlap: {
                    if miss {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x10));
                    let f3 = fr(c, 3);
                    if !le(c, f3, f0) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x04));
                    let f1 = fr(c, 1);
                    if !le(c, f0, f1) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x14));
                    let f4 = fr(c, 4);
                    if !le(c, f4, f0) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x08));
                    let f2 = fr(c, 2);
                    if !le(c, f0, f2) {
                        break 'overlap;
                    }
                    c.cycles += 5;
                    let f0 = ldf(c, 0, p(bbox, 0x18));
                    let f5 = fr(c, 5);
                    if le(c, f5, f0) {
                        c.g[V0][0] = 1;
                    }
                }
                // Cull test and the normal's facing test.
                'visible: {
                    c.cycles += 2;
                    let miss = c.g[V0][0] == 0;
                    c.g[T0][0] = c.lw(gp(c, CAMERA));
                    if miss {
                        break 'visible;
                    }
                    c.cycles += 11;
                    c.g[A1][0] = sx(p(sp(c), 0x50));
                    c.g[A2][0] = sx(p(sp(c), 0x60));
                    c.g[A3][0] = c.g[SP][0];
                    let cam = r32(c, T0);
                    c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
                    c.g[A0][0] = c.lh(p(r32(c, V1), 0x2e8));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x2ec));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
                    if call_ptr(c, target, 0x0024_2830).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.g[V0][0] ^ 1;
                    if c.g[V0][0] == 0 {
                        break 'visible;
                    }
                    c.cycles += 13;
                    c.mtc1(0, 0);
                    c.lqc2(3, p(sp(c), 0x40));
                    c.lqc2(5, p(r32(c, S0), view::NORMAL));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 1);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        break 'visible;
                    }
                    // Keep the view and its squared distance from the camera.
                    c.cycles += 20;
                    c.g[V1][0] = c.lw(p(this, occl::COUNT));
                    c.lqc2(3, p(sp(c), 0x40));
                    c.lqc2(4, p(r32(c, S0), view::POS));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 7, 5, Src::Vec(5));
                    c.vu0().adda(0b1000, 7, Src::Bc(7, Y));
                    c.vu0().madd(0b1000, 7, 6, Src::Bc(7, Z));
                    c.qmfc2(2, 7);
                    c.mtc1(2, 0);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V1][0] = sx(this.wrapping_add(r32(c, V1)));
                    c.sw(p(r32(c, V1), 0x08), c.g[S0][0]);
                    c.g[V0][0] = c.lw(p(this, occl::COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
                    c.swc1(0, p(r32(c, V0), 0x0c));
                    c.g[V1][0] = c.lw(p(this, occl::COUNT));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    c.sw(p(this, occl::COUNT), c.g[V1][0]);
                }
                c.cycles += 2;
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0024_2768);
                }
            }
        }
        // Sort by distance and build the planes of the nearest two.
        c.cycles += 3;
        c.g[A1][0] = c.lw(p(this, occl::COUNT));
        let empty = c.g[A1][0] == 0;
        c.g[S0][0] = sx(p(this, occl::ENTRIES));
        if !empty {
            c.cycles += 12;
            c.g[A3][0] = 0x24_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2978));
            c.g[A2][0] = 8;
            c.g[S1][0] = 0;
            if call_ptr(c, callee::QSORT, 0x0024_28ec).is_err() {
                return;
            }
            c.g[V0][0] = c.lw(p(this, occl::COUNT));
            c.g[A0][0] = 2;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            let none = (c.g[V0][0] as i64) <= 0;
            c.sw(p(this, occl::N_VISIBLE), c.g[V0][0]);
            if !none {
                c.cycles += 3;
                c.g[S3][0] = 1;
                c.g[V0][0] = c.lw(p(this, occl::MASK));
                loop {
                    c.cycles += 18;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[A1][0] = sx(p(sp(c), 0x40));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw(p(this, occl::MASK), c.g[V0][0]);
                    c.g[A2][0] = c.lw(r32(c, S0));
                    if callee::BUILD_OCCLUDER_PLANES.call(c, 0x0024_292c).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.lw(r32(c, S0));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(view::PLANES));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(view::NORMAL));
                    if call_ptr(c, callee::VU0_UPLOAD_OCCLUDER, 0x0024_2948).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.lw(p(this, occl::N_VISIBLE));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw(p(this, occl::MASK));
                    if event_due(c) {
                        deliver_event_here(c, 0x0024_2910);
                    }
                }
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, S0, 0xb0);
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore_ra(c, 0x70);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Rider air state.

/// Rider fields used by the air-state update.
mod air {
    /// The rider object behind the state.
    pub const RIDER: u32 = 0x04;
    /// The rider's animation / air-predictor state.
    pub const ANIM: u32 = 0x788;
    /// The rider's sub-state object.
    pub const SUB: u32 = 0x77c;
    /// Delta time of the frame.
    pub const DT: u32 = 0x300;
    /// Predicted position / velocity buffers handed to the predictor.
    pub const PRED_A: u32 = 0x110;
    pub const PRED_B: u32 = 0x1e0;
    /// Up vector and the copy the physics step reads.
    pub const UP: u32 = 0x180;
    pub const UP_COPY: u32 = 0x370;
    pub const GRAVITY: u32 = 0x3d0;
    /// The orientation the air state drives.
    pub const ORIENT: u32 = 0x1b0;
    /// Set while the wind force applies.
    pub const WIND: u32 = 0x874;
}

/// Air-predictor fields.
mod anim {
    /// Current state of the animation (1 and 3 are air states).
    pub const STATE: u32 = 0xac;
    /// The orientation target and its "thin" threshold.
    pub const ORIENT: u32 = 0x10;
    pub const THIN: u32 = 0x28;
    /// Index into the race manager's section records.
    pub const SECTION: u32 = 0x90;
    /// Section flags (bit 3 marks a boost pad).
    pub const FLAGS: u32 = 0x94;
    /// Time window of the air phase.
    pub const T0: u32 = 0x98;
    pub const T1: u32 = 0xa0;
    pub const ANGLE: u32 = 0x258;
}

/// World / race manager offsets used to find the rider's section record.
const WORLD: i32 = -0x848;
const SECTIONS: u32 = 0x44;

/// `void RiderAirState_Update(RiderAirState* self)` (0x139a20): the rider's
/// air-state tick: wind force, one air-prediction step, then (in the air)
/// choose the target orientation and the turn rate and hand them to
/// `Rider_UpdateOrientation`, finally syncing the implicit orientation.
pub fn rider_air_state_update(c: &mut Cpu) {
    c.cycles += 33;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    frame.save_fpr(c, 21, 0x58);
    frame.save_fpr(c, 20, 0x50);
    let this = r32(c, S0);
    // Wind force, then one step of the air prediction.
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    if call_ptr(c, callee::AIR_APPLY_WIND, 0x0013_9a44).is_err() {
        return;
    }
    c.g[A1][0] = c.lw(p(this, air::RIDER));
    c.lwc1(0, gp(c, -0x71d4));
    c.lwc1(12, p(r32(c, A1), air::DT));
    c.g[A2][0] = sx(p(r32(c, A1), air::PRED_B));
    c.g[A0][0] = c.lw(p(r32(c, A1), air::ANIM));
    c.mul_s(12, 12, 0);
    c.g[A1][0] = sx(p(r32(c, A1), air::PRED_A));
    if call_ptr(c, callee::AIR_PREDICTOR_UPDATE, 0x0013_9a64).is_err() {
        return;
    }
    // Copy the up vector, then the constant gravity step.
    c.g[A1][0] = c.lw(p(this, air::RIDER));
    c.g[A0][0] = 0x50_0000;
    c.g[S1][0] = c.g[A0][0];
    c.lq(3, p(r32(c, A1), air::UP));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
    c.g[A2][0] = 1;
    c.sq(3, sp(c));
    c.sq(3, p(r32(c, A1), air::UP_COPY));
    c.lq(2, r32(c, A0));
    c.g[V1][0] = c.lw(p(this, air::RIDER));
    c.sq(2, p(r32(c, V1), air::GRAVITY));
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    c.g[V0][0] = c.lw(p(r32(c, A0), air::ANIM));
    c.g[V1][0] = c.lw(p(r32(c, V0), anim::STATE));
    c.g[A3][0] = 0;
    let in_air = if c.g[V1][0] == c.g[A2][0] {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        c.g[V1][0] == c.g[V0][0]
    };
    if in_air {
        c.cycles += 1;
        c.g[A3][0] = 1;
    }
    c.cycles += 2;
    c.g[V1][0] = 0xb0;
    if c.g[A3][0] != 0 {
        'body: {
            // The rider's section record, and whether it is a boost section.
            c.cycles += 12;
            c.g[T1][0] = c.lw(p(this, air::RIDER));
            c.g[V0][0] = c.lw(gp(c, WORLD));
            c.g[T2][0] = 0;
            c.g[A2][0] = c.lw(p(r32(c, T1), air::ANIM));
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = c.lw(p(r32(c, V0), 0x84));
            c.g[T0][0] = c.lw(p(r32(c, A2), anim::SECTION));
            c.g[V0][0] = c.lw(p(r32(c, A0), SECTIONS));
            c.mult(0, 3, 8, 3);
            c.g[A3][0] = c.lh(p(r32(c, A2), anim::FLAGS));
            let no_flags = c.g[A3][0] == c.g[A1][0];
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            if !no_flags {
                c.cycles += 2;
                c.g[V0][0] = c.g[A3][0] & 0x8;
                c.g[T2][0] = (0 < c.g[V0][0]) as u64;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x12;
            if c.g[T0][0] != c.g[V0][0] {
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(r32(c, V1), SECTIONS));
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    if c.g[T2][0] != 0 {
                        // Turn rate: dt over the air window, or the fixed step.
                        'rate: {
                            c.cycles += 8;
                            c.lwc1(0, p(r32(c, A2), anim::T1));
                            c.lwc1(1, p(r32(c, A2), anim::T0));
                            c.lwc1(2, gp(c, -0x71d0));
                            c.sub_s(1, 1, 0);
                            c.c_le_s(2, 1);
                            let small = !c.fcc();
                            c.lwc1(12, p(r32(c, T1), air::DT));
                            if !small {
                                c.cycles += 3;
                                c.div_s(20, 12, 1);
                                c.g[A0][0] = c.lw(p(this, air::RIDER));
                                break 'rate;
                            }
                            c.cycles += 3;
                            c.lwc1(0, gp(c, -0x71cc));
                            c.mul_s(20, 12, 0);
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        // Turn amount for the frame.
                        c.cycles += 7;
                        c.lwc1(1, gp(c, -0x71c8));
                        c.lwc1(0, p(r32(c, A0), air::DT));
                        c.mul_s(21, 0, 1);
                        if call_ptr(c, callee::RIDER_GET_SUBSTATE, 0x0013_9b5c).is_err() {
                            return;
                        }
                        c.g[V1][0] = 5;
                        let not_grind = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = c.lw(p(this, air::RIDER));
                        if !not_grind {
                            // On a rail: keep the up vector unless it has
                            // tilted too far.
                            c.cycles += 9;
                            c.g[V1][0] = 1;
                            c.lwc1(1, gp(c, -0x71c4));
                            c.g[V0][0] = c.lw(p(r32(c, A0), air::SUB));
                            c.lwc1(0, p(r32(c, V0), anim::ANGLE));
                            c.abs_s(0, 0);
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V1][0] = 0;
                            }
                            c.cycles += 2;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xee0));
                            if c.g[V1][0] != 0 {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw(p(r32(c, A0), air::ANIM));
                                c.mov_s(12, 20);
                                c.mov_s(13, 21);
                                c.cycles += 4;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c50).is_err() {
                                    return;
                                }
                                break 'body;
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        // Steep or flipped: steer towards the target.
                        c.cycles += 7;
                        c.lwc1(1, gp(c, -0x71c0));
                        c.g[V1][0] = c.lw(p(r32(c, A0), air::ANIM));
                        c.lwc1(0, p(r32(c, V1), anim::THIN));
                        c.c_lt_s(0, 1);
                        let thin = c.fcc();
                        c.g[A1][0] = sx(p(r32(c, V1), 0x20));
                        if !thin {
                            c.cycles += 3;
                            let flat = c.lw(this) == 0;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xee0));
                            if flat {
                                c.cycles += 3;
                                c.mov_s(12, 20);
                                c.mov_s(13, 21);
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c64).is_err() {
                                    return;
                                }
                                break 'body;
                            }
                        }
                        // Build the up-vector target, keep its facing.
                        c.cycles += 18;
                        c.lq(3, p(r32(c, V1), anim::ORIENT));
                        c.mtc1(0, 1);
                        c.sq(3, sp(c));
                        c.lq(2, p(r32(c, A0), air::ORIENT));
                        c.sq(2, p(sp(c), 0x10));
                        c.lqc2(3, sp(c));
                        c.lqc2(5, p(sp(c), 0x10));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 1);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        } else {
                        c.cycles += 8;
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 0);
                        c.mfc1(2, 0);
                        c.lqc2(4, sp(c));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, sp(c));
                        c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        c.cycles += 4;
                        c.mov_s(12, 20);
                        c.mov_s(13, 21);
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = c.lw(p(r32(c, A0), air::ANIM));
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c50).is_err() {
                            return;
                        }
                    }
                }
            }
        }
    }
    c.cycles += 9;
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    if call_ptr(c, callee::RIDER_ORIENTATION_IMPLICIT, 0x0013_9c6c).is_err() {
        return;
    }
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 21, 0x58);
    frame.restore_fpr(c, 20, 0x50);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Board ray.

/// `float RiderAirState_CastBoardRay(RiderAirState* self, Hit* hit)` (0x13a7b0):
/// shoot a 200-unit segment along the board's forward direction through the
/// rider's overlap list and return the distance of the closest contact (or
/// the negative result of the query). A hit with an object dispatches to that
/// object's contact handler.
pub fn rider_air_state_cast_board_ray(c: &mut Cpu) {
    c.cycles += 63;
    let frame = Frame::open(c, 0x160);
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    frame.save(c, S0, 0x140);
    c.g[A2][0] = sx(p(sp(c), 0xe0));
    frame.save(c, S1, 0x130);
    c.g[S0][0] = sx(p(sp(c), 0x10));
    frame.save(c, S2, 0x120);
    c.g[S1][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x150);
    c.g[S2][0] = c.g[A1][0];
    frame.save_ra(c, 0x110);
    c.g[A1][0] = sx(p(sp(c), 0xc0));
    c.lwc1(12, gp(c, -0x71a4));
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] = c.lw(p(r32(c, S1), 0x04));
    c.g[A3][0] = 2;
    c.g[V1][0] = c.lw(p(r32(c, V0), 0x780));
    c.lq(8, p(r32(c, V0), 0x180));
    c.g[T1][0] = c.lw(p(r32(c, V1), 0x2c));
    c.g[V1][0] = c.lw(p(r32(c, V0), 0x8a0));
    c.sq(8, sp(c));
    c.mfc1(2, 0);
    c.lqc2(4, sp(c));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0xe0));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.lq(2, p(sp(c), 0xe0));
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sq(2, p(sp(c), 0xd0));
    c.lqc2(3, r32(c, T1));
    c.lqc2(4, p(sp(c), 0xd0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0xe0));
    c.lq(2, p(sp(c), 0xe0));
    c.sq(2, p(sp(c), 0xc0));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x100));
    c.lq(2, p(sp(c), 0x100));
    c.sq(2, p(sp(c), 0xf0));
    c.lqc2(3, r32(c, T1));
    c.lqc2(4, p(sp(c), 0xf0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x100));
    c.lq(2, p(sp(c), 0x100));
    c.sq(2, p(sp(c), 0xe0));
    if call_ptr(c, callee::COLL_SEGMENT_CTOR, 0x0013_a87c).is_err() {
        return;
    }
    c.g[V1][0] = c.lw(p(r32(c, S1), 0x04));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lw(p(r32(c, V1), 0x864));
    c.g[A0][0] = c.lw(p(r32(c, V1), 0x860));
    if call_ptr(c, callee::OVERLAP_LIST_FIND_CLOSEST, 0x0013_a894).is_err() {
        return;
    }
    c.mov_s(20, 0);
    c.mtc1(0, 0);
    c.c_le_s(0, 20);
    let behind = !c.fcc();
    c.mov_s(0, 20);
    let mut extra = behind;
    if !behind {
        c.cycles += 3;
        c.g[A0][0] = c.lw(p(r32(c, S2), 0x50));
        let no_hit = c.g[A0][0] == 0;
        c.lq(16, p(sp(c), 0x140));
        if !no_hit {
            c.cycles += 3;
            c.g[A2][0] = c.lw(p(r32(c, A0), 0x0c));
            let no_object = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S2][0];
            if !no_object {
                c.cycles += 6;
                c.g[V0][0] = c.lw(p(r32(c, A2), 0x0c));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x150));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x154));
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(r32(c, A0)));
                if call_ptr(c, target, 0x0013_a8d8).is_err() {
                    return;
                }
                c.mov_s(0, 20);
                extra = true;
            }
        }
    }
    if extra {
        c.cycles += 1;
        c.lq(16, p(sp(c), 0x140));
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x130);
    frame.restore(c, S2, 0x120);
    frame.restore_ra(c, 0x110);
    frame.restore_fpr(c, 20, 0x150);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    /// A `jr ra` stub the tests use for vtable slots whose result does not
    /// matter (`VfxVisualEffectsTest_Update`).
    const STUB: u32 = 0x0038_f7f8;

    #[test]
    fn dma_wait_from_scratchpad_matches() {
        check(ee::s12::f_0038f768__generated, dma_wait_from_scratchpad, 40, &|c, seed| {
            let mut r = Rng(0x9e37_79b9 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn vu0_upload_occluder_matches() {
        check(ee::s05::f_00229f88__generated, vu0_upload_occluder, 120, &|c, seed| {
            let mut r = Rng(0x51ed_2701 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx((seed % 4) as u32);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = sx(DATA + 0x140);
            for i in 0..4 {
                put_vec(c, DATA + 0x100 + i * 0x10, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
                put_vec(c, DATA + 0x140 + i * 0x10, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            }
        });
    }

    #[test]
    fn world_painter_lerp_view_colours_matches() {
        check(ee::s09::f_002ed338__generated, world_painter_lerp_view_colours, 120, &|c, seed| {
            let mut r = Rng(0x2f_ed33 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = sx(DATA + 0x140);
            for i in 0..8 {
                c.bus.write32(DATA + 0x28 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x100 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x140 + i * 4, r.edge_f32());
            }
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn mem_pool_free_matches() {
        const POOLS: [i32; 7] = [0x2a70, 0x2a60, 0x2a6c, 0x2a7c, 0x2a84, 0x2a78, 0x2a68];
        check(ee::s10::f_00319f68__generated, mem_pool_free, 140, &|c, seed| {
            let mut r = Rng(0x319f_68 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let block = DATA + 0x2000;
            c.g[A0][0] = sx(block);
            // Every pool base starts far above the block; the seed picks one
            // that is just below it so that size class is chosen.
            let far = DATA + 0x40_0000;
            for (k, off) in POOLS.iter().enumerate() {
                let base = if k as u32 == seed % 8 { block - 0x100 } else { far };
                c.bus.write32((GAME_GP as i32 + *off) as u32, base);
            }
            c.bus.write32(block, r.next_u32());
        });
    }

    #[test]
    fn rider_metrix_check_score_milestone_matches() {
        check(ee::s00::f_00119210__generated, rider_metrix_check_score_milestone, 120, &|c, seed| {
            let mut r = Rng(0x1192_10 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let idx = DATA + 0x100;
            let table = DATA + 0x200;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(idx);
            c.g[A2][0] = sx(table);
            c.g[A3][0] = sx(2);
            let i = if seed % 3 == 0 { 0 } else { r.next_u32() % 3 };
            c.bus.write32(idx, i);
            // Seeds cover a negative threshold, an unreached step and a hit.
            let (threshold, x) = match seed % 3 {
                0 => ((-1.0f32).to_bits(), 1.0f32.to_bits()),
                1 => (5.0f32.to_bits(), 1.0f32.to_bits()),
                _ => (0.5f32.to_bits(), 1.0f32.to_bits()),
            };
            c.bus.write32(table + i * 8, threshold);
            c.bus.write32(table + i * 8 + 4, r.f32(100.0));
            c.bus.write32(this + rider::METRIC_SCALE, r.edge_f32());
            c.f[12] = x;
        });
    }

    #[test]
    fn rider_add_boost_matches() {
        check(ee::s00::f_0010e098__generated, rider_add_boost, 240, &|c, seed| {
            let mut r = Rng(0x10e0_98 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            // BELibrary singleton: a zeroed dummy object, else it loads from
            // disc forever.
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x800);
            // cBE_getInterface indexes a table at 0x5308a8; steer the index
            // into RAM and put a dummy interface (with a do-nothing method)
            // there.
            c.bus.write32(GAME_GP - 0x1efc, 0x03ec_bfd6);
            c.bus.write32(0x0083_0000, DATA + 0x900);
            c.bus.write32(DATA + 0x900, 0);
            c.bus.write32(DATA + 0x904, DATA + 0x940);
            c.bus.write16(DATA + 0x948, 0);
            c.bus.write32(DATA + 0x94c, STUB);
            let (state, amt, boost, timer, count, mask_field, flag) = match seed % 8 {
                0 => (3, 0.5f32, 0.0f32, 0.0f32, 0u32, 0xffff_ffffu32, 1u32),
                1 => (0, 0.0, 0.0, 0.0, 0, 0xffff_ffff, 1),
                2 => (0, -0.5, 0.2, 0.0, 4, 0xffff_ffff, 1),
                3 => (0, 0.5, 0.0, 0.0, 0, 0, 1),
                4 => (0, 0.1, 0.2, 0.0, 0, 0xffff_ffff, 1),
                5 => (0, 0.9, 0.5, 0.0, 5, 0xffff_ffff, 1),
                6 => (0, 0.9, 0.5, 3.0, 10, 0xffff_ffff, 1),
                _ => (0, -0.5, 0.2, 0.0, 20, 0xffff_ffff, 1),
            };
            c.bus.write32(this + rider::BOOST_STATE, state);
            c.bus.write32(this + rider::BOOST, boost.to_bits());
            c.bus.write32(this + rider::BOOST_TIMER, timer.to_bits());
            c.bus.write32(this + rider::BOOST_COUNT, count);
            c.bus.write32(this + rider::BOOST_MASK, mask_field);
            c.bus.write32(this + rider::BOOST_FLAG, flag);
            c.bus.write32(this + rider::CHAR_ID, r.next_u32());
            c.bus.write32(this + rider::CHAR_COMP, r.next_u32());
            c.bus.write32(this + 0x874, 0);
            c.f[12] = amt.to_bits();
        });
    }

    #[test]
    fn mat4_adjoint_matches() {
        check(ee::s10::f_0031b310__generated, mat4_adjoint, 120, &|c, seed| {
            let mut r = Rng(0x31b3_10 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            for i in 0..4 {
                let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, DATA + i * 0x10, v);
            }
        });
    }

    #[test]
    fn c_ps2_grid_mesh_v03_matches() {
        check(ee::s12::f_003852e8__generated, c_ps2_grid_mesh_v03, 120, &|c, seed| {
            let mut r = Rng(0x3852_e8 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            let verts = DATA + 0x4000;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(verts);
            let w = seed % 4;
            let rows = (seed / 4) % 3;
            let tab = if seed % 2 == 0 { DATA + 0x1000 } else { DATA + 0x2000 };
            c.bus.write16(this + grid_mesh::WIDTH, w as u16);
            c.bus.write32(this + grid_mesh::SCALE, 1.0f32.to_bits());
            c.bus.write32(this + grid_mesh::TAB_INDEX, (seed % 2) as u32);
            c.bus.write32(this + grid_mesh::PACKED, 0);
            c.bus.write32(this + grid_mesh::ROWS, rows);
            c.bus.write32(this + grid_mesh::ROW_TABS, tab);
            c.bus.write32(this + grid_mesh::ROW_TABS + 4, tab);
            for i in 0..rows as u32 {
                c.bus.write32(tab + i * 16, r.next_u32());
                c.bus.write32(tab + i * 16 + 4, DATA + 0x3000 + i * 0x100);
            }
            for i in 0..16 {
                c.bus.write32(verts + i * 12, r.edge_f32());
                c.bus.write32(verts + i * 12 + 4, r.edge_f32());
                c.bus.write32(verts + i * 12 + 8, r.edge_f32());
            }
        });
    }

    #[test]
    fn occlusion_man_build_visible_matches() {
        check(ee::s06::f_002425c0__generated, occlusion_man_build_visible, 160, &|c, seed| {
            let mut r = Rng(0x2425_c0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let bbox = DATA + 0x400;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(bbox);
            c.bus.write32(this + occl::ENABLED, if seed % 7 == 0 { 0 } else { 1 });
            c.bus.write32(this + occl::COUNT, 0);
            c.bus.write32(this + occl::N_VISIBLE, 0);
            c.bus.write32(this + occl::MASK, 0);
            for i in 0..6 {
                c.bus.write32(bbox + i * 4, r.f32(100.0));
            }
            // Camera singleton: a matrix method that writes nothing (the frame
            // is already zero) and a row-position method with a zero result.
            let cam = DATA + 0x800;
            let vt = DATA + 0x1000;
            c.bus.write32(GAME_GP - 0x854, cam);
            c.bus.write32(cam + CAMERA_VT, vt);
            for off in [0x158u32, 0x118, 0x2e8] {
                c.bus.write16(vt + off, 0);
            }
            for off in [0x15c, 0x11c, 0x2ec] {
                c.bus.write32(vt + off, STUB);
            }
            // View list.
            let n = seed % 4;
            c.bus.write32(this, 0);
            let mut prev = 0u32;
            for i in 0..n {
                let v = DATA + 0x2000 + i * 0x100;
                if i == 0 {
                    c.bus.write32(this, v);
                } else {
                    c.bus.write32(prev + view::NEXT, v);
                }
                c.bus.write32(v + view::NEXT, 0);
                for k in 0..4 {
                    c.bus.write32(v + view::POS + k * 4, r.f32(50.0));
                    c.bus.write32(v + view::NORMAL + k * 4, r.f32(2.0));
                }
                for k in 0..3 {
                    c.bus.write32(v + view::MIN + k * 4, r.f32(100.0));
                    c.bus.write32(v + view::MAX + k * 4, r.f32(100.0));
                }
                prev = v;
            }
        });
    }
    #[test]
    fn rider_air_state_update_matches() {
        check(ee::s01::f_00139a20__generated, rider_air_state_update, 160, &|c, seed| {
            let mut r = Rng(0x139a_20 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let rider = DATA + 0x100;
            let anim = DATA + 0x8000;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + air::RIDER, rider);
            c.bus.write32(this, 0);
            for k in 0..4 {
                c.bus.write32(rider + air::UP + k * 4, r.f32(1.0));
                c.bus.write32(rider + air::ORIENT + k * 4, r.f32(1.0));
            }
            c.bus.write32(rider + air::DT, 0.016f32.to_bits());
            c.bus.write32(rider + air::ANIM, anim);
            c.bus.write32(rider + air::SUB, DATA + 0x500);
            c.bus.write32(rider + air::WIND, 0);
            let angle = if seed % 4 == 2 { 0.5f32 } else { 0.001f32 };
            c.bus.write32(DATA + 0x500 + anim::ANGLE, angle.to_bits());
            // world -> +0x84 -> +0x44: the race manager's section records.
            let world = DATA + 0x600;
            let mgr = DATA + 0x620;
            c.bus.write32((GAME_GP as i32 + WORLD) as u32, world);
            c.bus.write32(world + 0x84, mgr);
            c.bus.write32(mgr + SECTIONS, DATA + 0x640);
            c.bus.write32(DATA + 0x640 + 0xb0 + SECTIONS, 0);
            // Animation / predictor state.
            let state = match seed % 4 { 0 => 2u32, 1 => 3, 2 => 1, _ => 3 };
            c.bus.write32(anim + anim::STATE, state);
            c.bus.write32(anim + anim::SECTION, 1);
            c.bus.write16(anim + anim::FLAGS, 8);
            c.bus.write32(anim + anim::T0, 2.0f32.to_bits());
            c.bus.write32(anim + anim::T1, 0.0f32.to_bits());
            c.bus.write32(anim + 0x58, 0.0f32.to_bits());
            c.bus.write32(anim + 0xa4, 0.0f32.to_bits());
            c.bus.write32(anim + 0xa8, 1.0f32.to_bits());
            let thin = if seed % 8 == 3 { 0.5f32 } else { 0.1f32 };
            c.bus.write32(anim + anim::THIN, thin.to_bits());
            for k in 0..4 {
                c.bus.write32(anim + anim::ORIENT + k * 4, r.f32(1.0));
            }
            let sub = if seed % 4 == 2 { 5u32 } else { 0 };
            c.bus.write32(DATA + 0x500 + 0xde4, sub);
            // The float constants the state reads through gp.
            c.bus.write32((GAME_GP as i32 - 0x71d4) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71d0) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71cc) as u32, 60.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c8) as u32, 1.396f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c4) as u32, 0.0872f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c0) as u32, 0.3f32.to_bits());
            // The predictor's internal step budget and rates.
            c.bus.write32((GAME_GP as i32 - 0x7c5c) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c74) as u32, 2.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c70) as u32, 1.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c6c) as u32, 1.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c60) as u32, 0.0f32.to_bits());
        });
    }
    #[test]
    fn rider_air_state_cast_board_ray_matches() {
        check(ee::s01::f_0013a7b0__generated, rider_air_state_cast_board_ray, 120, &|c, seed| {
            let mut r = Rng(0x13a7_b0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let this = DATA;
            let rider = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x600);
            c.bus.write32(this + 0x4, rider);
            c.bus.write32(rider + 0x780, DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x2c, DATA + 0x300);
            c.bus.write32(rider + 0x8a0, 0);
            c.bus.write32(rider + 0x860, 0);
            c.bus.write32(rider + 0x864, 0x40);
            for k in 0..4 {
                c.bus.write32(rider + 0x180 + k * 4, r.f32(1.0));
                c.bus.write32(DATA + 0x300 + k * 4, r.f32(10.0));
            }
            c.bus.write32((GAME_GP as i32 - 0x71a4) as u32, 0.575f32.to_bits());
        });
    }
}
