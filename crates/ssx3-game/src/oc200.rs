//! Staging wave 200: the functions read while building sun shadows.
//!
//! - the Nash world pass: `Nash_GatherVisible` (clear the visible lists,
//!   load the view matrices, cull the eight octant roots, collect and
//!   classify), `Nash_CollectPatches`, `Vu0_UploadMatrixPair`;
//! - the Bézier patch cache tessellators `BezierPatchCache_TessPositions`
//!   and `BezierPatchCache_TessPositionsAlt` (positions, then normals);
//! - the sun: `LensFX_UpdateSunPosition`, the `WorldPainter_GetSun*`
//!   virtual getters and the `tWPIGD_Sun` field getters they reach.
//!
//! Each replacement reproduces the generated body's registers, cycles,
//! memory and call order; VU0 macro code issues the original's
//! `lqc2`/`sqc2`/`vu0()` operations, FPU code the original's FPU
//! operations.

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_add8, nash_gather_visible, "Nash_GatherVisible"),
    (0x0022_a698, nash_collect_patches, "Nash_CollectPatches"),
    (0x0022_9f30, vu0_upload_matrix_pair, "Vu0_UploadMatrixPair"),
    (
        0x0037_4180,
        bezier_patch_cache_tess_positions,
        "BezierPatchCache_TessPositions",
    ),
    (
        0x0037_4518,
        bezier_patch_cache_tess_positions_alt,
        "BezierPatchCache_TessPositionsAlt",
    ),
    (
        0x002f_4db8,
        lens_fx_update_sun_position,
        "LensFX_UpdateSunPosition",
    ),
    (
        0x002e_e070,
        world_painter_get_sun_elevation,
        "WorldPainter_GetSunElevation",
    ),
    (
        0x002e_e0b8,
        world_painter_get_sun_azimuth,
        "WorldPainter_GetSunAzimuth",
    ),
    (
        0x002e_e100,
        world_painter_get_sun_red,
        "WorldPainter_GetSunRed",
    ),
    (
        0x002e_e148,
        world_painter_get_sun_green,
        "WorldPainter_GetSunGreen",
    ),
    (
        0x002e_e190,
        world_painter_get_sun_blue,
        "WorldPainter_GetSunBlue",
    ),
    (
        0x002e_e1d8,
        world_painter_get_sun_intensity,
        "WorldPainter_GetSunIntensity",
    ),
    (
        0x002e_e2b0,
        world_painter_get_sun_size,
        "WorldPainter_GetSunSize",
    ),
    (
        0x002c_1560,
        twpigd_sun_get_elevation,
        "tWPIGD_Sun_GetElevation",
    ),
    (0x002c_1568, twpigd_sun_get_azimuth, "tWPIGD_Sun_GetAzimuth"),
];

// ---------------------------------------------------------------------------
// Helpers.

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// gp offset (negative) of the graphics object (`D_004A289C`): the camera
/// interface's vtable at +0x10d8, the world view matrix at +0x5800.
const GRAPHICS_GP: u32 = 0x854;

mod callee {
    use super::{ee, Callee};

    pub const NASH_CULL_NODE: Callee = Callee {
        addr: 0x0022_a830,
        f: ee::s05::Nash_CullNode,
    };
    pub const VU0_UPLOAD_MATRIX_PAIR: Callee = Callee {
        addr: 0x0022_9f30,
        f: ee::s05::Vu0_UploadMatrixPair,
    };
    pub const NASH_EMPTY_HOOK: Callee = Callee {
        addr: 0x0022_9f80,
        f: ee::s05::Nash_EmptyHook,
    };
    pub const NASH_COLLECT_PATCHES: Callee = Callee {
        addr: 0x0022_a698,
        f: ee::s05::Nash_CollectPatches,
    };
    pub const NASH_COLLECT_INSTANCES: Callee = Callee {
        addr: 0x0022_a5a0,
        f: ee::s05::Nash_CollectInstances,
    };
    pub const NASH_COLLECT_EFFECTS: Callee = Callee {
        addr: 0x0022_a770,
        f: ee::s05::Nash_CollectEffects,
    };
    pub const NASH_CLASSIFY_PATCHES: Callee = Callee {
        addr: 0x0022_a408,
        f: ee::s05::Nash_ClassifyPatches,
    };
    pub const NASH_CLASSIFY_INSTANCES: Callee = Callee {
        addr: 0x0022_a368,
        f: ee::s05::Nash_ClassifyInstances,
    };
    pub const NASH_CLASSIFY_EFFECTS: Callee = Callee {
        addr: 0x0022_a4a8,
        f: ee::s05::Nash_ClassifyEffects,
    };

    pub const WORLD_PAINTER_GET_SUN_TEXTURE: Callee = Callee {
        addr: 0x002e_e220,
        f: ee::s09::WorldPainter_GetSunTexture,
    };
    pub const WORLD_PAINTER_GET_SUN_AZIMUTH: Callee = Callee {
        addr: 0x002e_e0b8,
        f: ee::s09::WorldPainter_GetSunAzimuth,
    };
    pub const WORLD_PAINTER_GET_SUN_ELEVATION: Callee = Callee {
        addr: 0x002e_e070,
        f: ee::s09::WorldPainter_GetSunElevation,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
mod table_callee {
    /// `OcclusionMan_BuildVisible(man, arg)`.
    pub const OCCLUSION_MAN_BUILD_VISIBLE: u32 = 0x0024_25c0;
    /// `Math_SinCos(angle f12, float* sin, float* cos)`.
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
}

// ---------------------------------------------------------------------------
// The Nash world pass.

/// Nash world (`this`) fields written by `Nash_GatherVisible`.
mod nash {
    /// The occlusion manager (`OcclusionMan*`), +0x110 its occluder mask.
    pub const OCCLUSION: u32 = 0x00;
    /// Argument passed on to `OcclusionMan_BuildVisible` / the empty hook.
    pub const HOOK_ARG: u32 = 0x04;
    /// The camera view matrix (4 quads).
    pub const VIEW_MATRIX: u32 = 0x10;
    /// The world view matrix (4 quads, copied from graphics +0x5800).
    pub const WORLD_MATRIX: u32 = 0x50;
    /// Section tables (`*tables` = section/logical-section state arrays).
    pub const SECTION_TABLES: u32 = 0x90;
    /// Fully visible nodes: count, then (node, clipped) pairs.
    pub const VISIBLE_COUNT: u32 = 0x94;
    pub const VISIBLE: u32 = 0x98;
    /// Partially visible nodes: count, then (node, clipped) pairs.
    pub const PARTIAL_COUNT: u32 = 0x1098;
    pub const PARTIAL: u32 = 0x109c;
    /// The visible patch list (count, then patch pointers).
    pub const PATCH_COUNT: u32 = 0x50a8;
    pub const PATCHES: u32 = 0x50ac;
    /// Every list count the pass resets, in the original store order.
    pub const CLEARED: [u32; 13] = [
        0x94, 0x1098, 0x209c, 0x40a0, 0x50a8, 0x70ac, 0x78b4, 0x7abc, 0x7bc0, 0x7fc4, 0x50a4,
        0x78b0, 0x7ab8,
    ];
}

/// Copy a 4x4 matrix through VU0 registers vf1..vf4 (`lqc2` x4, `sqc2` x4).
fn copy_matrix_vu0(c: &mut Cpu, src: u32, dst: u32) {
    for (k, vf) in (1..=4).enumerate() {
        c.lqc2(vf, src.wrapping_add(0x10 * k as u32));
    }
    for (k, vf) in (1..=4).enumerate() {
        c.sqc2(vf, dst.wrapping_add(0x10 * k as u32));
    }
}

/// `Nash* Nash_GatherVisible(Nash* this, NashRoot roots[8], OcclusionMan*
/// occ, void* hook_arg, SectionTables* tables)` (0x22add8): the world
/// pass. Stores its arguments, clears every visible list, copies the camera
/// view matrix (graphics camera interface slot 43, returned on the stack)
/// to +0x10 and the graphics world matrix (+0x5800) to +0x50, builds the
/// occluder set, uploads both matrices to VU0 data memory, then culls the
/// eight octant roots (`Nash_CullNode(ctx, node, cell)`; roots are 0x14
/// apart: cell at +0, node at +0x10) with the context `{this, occ,
/// occ->mask (+0x110), inside = 0}` on the stack, and collects / classifies
/// the patches, instances and effects of the visible (+0x98) and partial
/// (+0x109c) node lists. Returns `this`.
pub fn nash_gather_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 67;
        let frame = Frame::open(c, 0x90);
        c.g[T1][0] = c.lw(r32(c, GP).wrapping_sub(GRAPHICS_GP));
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A0][0]; // this
        frame.save(c, S3, 0x50);
        c.g[A0][0] = c.g[SP][0]; // the returned matrix at sp+0
        frame.save(c, S0, 0x80);
        c.g[S3][0] = c.g[A1][0]; // roots
        frame.save_ra(c, 0x40);
        c.g[S1][0] = sx(at(c, S2, nash::VIEW_MATRIX));
        let this = r32(c, S2);
        c.sw(this.wrapping_add(nash::OCCLUSION), c.g[A2][0]);
        c.sw(this.wrapping_add(nash::HOOK_ARG), c.g[A3][0]);
        c.sw(this.wrapping_add(nash::SECTION_TABLES), c.g[T0][0]);
        for off in nash::CLEARED {
            c.sw(this.wrapping_add(off), 0);
        }
        // The camera view matrix: graphics camera interface slot 43.
        let vt = c.lw(at(c, T1, 0x10d8));
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lh((vt as u32).wrapping_add(0x158));
        c.g[V1][0] = c.lw((vt as u32).wrapping_add(0x15c));
        let method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x0022_ae58)?;
        copy_matrix_vu0(c, sp(c), r32(c, S1));
        // The world matrix.
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(GRAPHICS_GP));
        c.g[S0][0] = sx(at(c, S2, nash::WORLD_MATRIX));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x5800));
        copy_matrix_vu0(c, r32(c, V0), r32(c, S0));
        c.g[A0][0] = c.lw(at(c, S2, nash::OCCLUSION));
        c.g[A1][0] = c.lw(at(c, S2, nash::HOOK_ARG));
        call_ptr(c, table_callee::OCCLUSION_MAN_BUILD_VISIBLE, 0x0022_aeb0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::VU0_UPLOAD_MATRIX_PAIR.call(c, 0x0022_aebc)?;
        c.g[A0][0] = c.lw(at(c, S2, nash::HOOK_ARG));
        callee::NASH_EMPTY_HOOK.call(c, 0x0022_aec4)?;
        // The cull context at sp+0: {this, occ, occ->mask, inside = 0}.
        let occ = c.lw(at(c, S2, nash::OCCLUSION));
        c.g[V0][0] = occ;
        c.g[A1][0] = c.lw(at(c, S3, 0x10)); // root 0's node
        c.g[V1][0] = c.lw((occ as u32).wrapping_add(0x110));
        let ctx = sp(c);
        c.sw(ctx.wrapping_add(4), c.g[V0][0]);
        c.sw(ctx.wrapping_add(8), c.g[V1][0]);
        c.sw(ctx, c.g[S2][0]);
        c.sw(ctx.wrapping_add(0xc), 0);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S3][0];
            callee::NASH_CULL_NODE.call(c, 0x0022_aef0)?;
        }
        for k in 1..8u32 {
            let root = 0x14 * k;
            c.cycles += 3;
            c.g[A1][0] = c.lw(at(c, S3, root + 0x10));
            c.g[A2][0] = sx(at(c, S3, root));
            if c.g[A1][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[SP][0];
                callee::NASH_CULL_NODE.call(c, 0x0022_aef0 + root)?;
            }
        }
        // Collect from the visible list, classify the partial list.
        c.cycles += 34;
        c.g[S0][0] = sx(at(c, S2, nash::VISIBLE));
        for (f, ret_addr) in [
            (callee::NASH_COLLECT_PATCHES, 0x0022_af90),
            (callee::NASH_COLLECT_INSTANCES, 0x0022_afa4),
            (callee::NASH_COLLECT_EFFECTS, 0x0022_afb4),
        ] {
            if ret_addr == 0x0022_afa4 {
                c.g[S1][0] = sx(at(c, S2, nash::PARTIAL));
            }
            c.g[A2][0] = c.lw(at(c, S2, nash::VISIBLE_COUNT));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            f.call(c, ret_addr)?;
        }
        for (f, ret_addr) in [
            (callee::NASH_CLASSIFY_PATCHES, 0x0022_afc4),
            (callee::NASH_CLASSIFY_INSTANCES, 0x0022_afd4),
            (callee::NASH_CLASSIFY_EFFECTS, 0x0022_afe4),
        ] {
            c.g[A2][0] = c.lw(at(c, S2, nash::PARTIAL_COUNT));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            f.call(c, ret_addr)?;
        }
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void Nash_CollectPatches(Nash* this, NodePair* nodes, int count)`
/// (0x22a698): append the Bézier patches of the visible nodes (each node's
/// patch list at +0x24, linked through +0) to the patch list at
/// `this + 0x50ac` (count +0x50a8), drawn by
/// `GameModule_RenderBezierPatches`. A patch must belong to an active
/// section: its logical section (`i16` at +0x156) is non-negative, the
/// section (`u8` at +0x155) has state 6 (+0x24 of its 8-byte entry) and the
/// logical section state 3 (+0x3f0 of its 0x18-byte entry; tables from
/// `*(this + 0x90)`). Patches of clipped nodes (pair word +4 non-zero) are
/// culled by the VU0 sphere test microprogram at 0xef0 on the sphere at
/// +0x140 (`vi1 == 2` = outside).
pub fn nash_collect_patches(c: &mut Cpu) {
    const NODE_HEAD: u32 = 0x0022_a6a0;
    const PATCH_HEAD: u32 = 0x0022_a6c8;
    c.cycles += 2;
    if (c.g[A2][0] as i64) > 0 {
        loop {
            c.cycles += 5;
            let node = c.lw(r32(c, A1));
            c.g[V0][0] = node;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[T0][0] = c.lw((node as u32).wrapping_add(0x24)); // patch list
            c.g[T4][0] = sx(at(c, A1, 8));
            if c.g[T0][0] != 0 {
                c.cycles += 5;
                c.g[T6][0] = 0x18; // logical section entry size
                c.g[T5][0] = 2; // "outside" from the sphere test
                c.g[T3][0] = sx(at(c, A0, nash::PATCHES));
                c.g[V0][0] = c.lw(at(c, A0, nash::SECTION_TABLES));
                loop {
                    collect_patch(c);
                    c.cycles += 2;
                    if c.g[T0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw(at(c, A0, nash::SECTION_TABLES));
                    if event_due(c) {
                        deliver_event_here(c, PATCH_HEAD);
                    }
                }
            }
            c.cycles += 2;
            let more = (c.g[A2][0] as i64) > 0;
            c.g[A1][0] = c.g[T4][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, NODE_HEAD);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// One patch of `Nash_CollectPatches` (patch in t0, the section tables
/// holder in v0): the section filter, the cull test and the append; leaves
/// t0 at the next patch.
fn collect_patch(c: &mut Cpu) {
    c.cycles += 5;
    c.g[T1][0] = c.lh(at(c, T0, 0x156)); // logical section
    c.g[A3][0] = c.lw(r32(c, V0)); // section tables
    c.g[T2][0] = c.lw(at(c, A1, 4)); // the node's "clipped" flag
    c.g[V0][0] = 0;
    if (c.g[T1][0] as i64) >= 0 {
        c.cycles += 7;
        let section = c.lbu(at(c, T0, 0x155));
        c.g[V0][0] = sx((section as u32) << 3);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, V0, 0x24)) ^ 6;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            c.mult(0, V0, T1, T6); // logical * 0x18
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V0][0] = c.lw(at(c, V1, 0x3f0)) ^ 3;
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(at(c, T0, 0x140)); // bounding sphere
        let culled = c.g[T2][0] != 0 && {
            c.cycles += 7;
            c.lqc2(22, r32(c, V0));
            c.ctc2(T2, 14);
            c.vcallms(0xef0);
            c.cfc2(V1, 1);
            c.g[V1][0] == c.g[T5][0]
        };
        if !culled {
            c.cycles += 7;
            let n = c.lw(at(c, A0, nash::PATCH_COUNT));
            c.g[V1][0] = n;
            c.g[V0][0] = sx((n as u32) << 2);
            c.g[V0][0] = sx(r32(c, T3).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx((n as u32).wrapping_add(1));
            c.sw(r32(c, V0), c.g[T0][0]);
            c.sw(at(c, A0, nash::PATCH_COUNT), c.g[V1][0]);
        }
    }
    c.g[T0][0] = c.lw(r32(c, T0));
}

/// `void Vu0_UploadMatrixPair(const Mat4* a, const Mat4* b)` (0x229f30):
/// store both 4x4 matrices to VU0 data memory from qword 0x40 (`vi1 =
/// 0x40`, eight `vsqi`), for the culling microprograms.
pub fn vu0_upload_matrix_pair(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x40;
    c.ctc2(V0, 1);
    for arg in [A0, A1] {
        let m = r32(c, arg);
        for (k, vf) in (1..=4).enumerate() {
            c.lqc2(vf, m.wrapping_add(0x10 * k as u32));
        }
        for vf in 1..=4u8 {
            c.vu0().sqi(0b1111, vf, 1);
        }
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// Bézier patch cache tessellation.

/// Load the patch's 4x4 grid of control points (16 quads from `p`) into
/// vf1..vf16.
fn load_control_points(c: &mut Cpu, p: u32) {
    for k in 0..16u32 {
        c.lqc2(1 + k as usize, p.wrapping_add(0x10 * k));
    }
}

/// The curve of one parameter row: weights from the basis entry at `w`
/// (vf17) applied to each column of control points, giving the curve's
/// four control points in vf18..vf21.
fn basis_row(c: &mut Cpu, w: u32) {
    c.lqc2(17, w);
    for (k, out) in (18u8..=21).enumerate() {
        let col = 1 + 4 * k as u8;
        c.vu0().mula(0b1111, col, Src::Bc(17, X));
        c.vu0().madda(0b1111, col + 1, Src::Bc(17, Y));
        c.vu0().madda(0b1111, col + 2, Src::Bc(17, Z));
        c.vu0().madd(0b1111, out, col + 3, Src::Bc(17, W));
    }
}

/// A point of the row curve (vf18..vf21): weights from the basis entry at
/// `w` (vf22); the point in vf23.
fn curve_point(c: &mut Cpu, w: u32) {
    c.lqc2(22, w);
    c.vu0().mula(0b1111, 18, Src::Bc(22, X));
    c.vu0().madda(0b1111, 19, Src::Bc(22, Y));
    c.vu0().madda(0b1111, 20, Src::Bc(22, Z));
    c.vu0().madd(0b1111, 23, 21, Src::Bc(22, W));
}

/// The end of a row (both tessellators): `v1 = v0 (row + 1)`, `v0 = row
/// < n`; true when another row follows, `v0 = row << 4` then.
fn next_row(c: &mut Cpu, n_reg: usize) -> bool {
    c.cycles += 4;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[n_reg][0] as i64)) as u64;
    let more = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, V1) << 4);
    more
}

/// `void BezierPatchCache_TessPositions(BezierPatchCache* cache,
/// BezierPatch* patch, Vec3* out, int lod)` (0x374180): evaluate the bicubic
/// patch (control points at patch+0x40) on the LOD's `n x n` parameter grid
/// (basis table `cache->basis[lod]` at +0x190, `n = cache->count[lod]` at
/// +0x19c, one quad of weights per parameter) and write the positions as
/// packed vec3s, row by row. The quad goes through a 16-byte stack slot
/// (`ldl/ldr` + `lw`, then `sdl/sdr` + `sw`).
pub fn bezier_patch_cache_tess_positions(c: &mut Cpu) {
    const ROW_HEAD: u32 = 0x0037_41e8;
    const POINT_HEAD: u32 = 0x0037_4240;
    c.cycles += 25;
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x40));
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(at(c, V0, 0x190)); // basis
    c.g[A3][0] = c.lw(at(c, A0, 0x19c)); // n
    load_control_points(c, r32(c, A1));
    c.g[V1][0] = 0;
    if (c.g[A3][0] as i64) > 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[A3][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[T0][0];
                c.g[V1][0] = c.g[A3][0];
                loop {
                    c.cycles += 16;
                    curve_point(c, r32(c, A0));
                    let tmp = sp(c);
                    c.sqc2(23, tmp);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.ldl(A1, tmp.wrapping_add(7));
                    c.ldr(A1, tmp);
                    c.g[T1][0] = c.lw(tmp.wrapping_add(8));
                    let out = r32(c, A2);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.sw(out.wrapping_add(8), c.g[T1][0]);
                    let more = c.g[V1][0] != 0;
                    c.g[A2][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, POINT_HEAD);
                    }
                }
            }
            if !next_row(c, A3) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ROW_HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    ret(c);
}

/// `void BezierPatchCache_TessPositionsAlt(BezierPatchCache* cache,
/// BezierPatch* patch, Vec3* out, int lod)` (0x374518): two passes over the
/// LOD's `n x n` grid (`n` at +0x19c, value basis at +0x190, derivative
/// basis at +0x310, per LOD). Pass 1 writes one tangent (derivative-basis
/// rows, value-basis points) to `out`; pass 2 evaluates the other tangent
/// (value-basis rows, derivative-basis points) and replaces each entry
/// with the normalised cross product of the two (left unnormalised when
/// its length is 0): the patch normals.
pub fn bezier_patch_cache_tess_positions_alt(c: &mut Cpu) {
    const DU_ROW_HEAD: u32 = 0x0037_4590;
    const DU_POINT_HEAD: u32 = 0x0037_45e8;
    const DV_ROW_HEAD: u32 = 0x0037_4648;
    const DV_POINT_HEAD: u32 = 0x0037_46b0;
    c.cycles += 28;
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x40));
    c.g[V0][0] = c.g[V1][0];
    c.g[T2][0] = c.lw(at(c, V1, 0x310)); // derivative basis
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = c.lw(at(c, V0, 0x19c)); // n
    c.g[T1][0] = c.lw(at(c, A0, 0x190)); // value basis
    c.g[A3][0] = c.g[A2][0];
    load_control_points(c, r32(c, A1));
    c.g[V1][0] = 0;
    // Pass 1: d/du into out.
    if (c.g[T0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[T0][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[T1][0];
                c.g[V1][0] = c.g[T0][0];
                loop {
                    c.cycles += 16;
                    curve_point(c, r32(c, A0));
                    let tmp = sp(c);
                    c.sqc2(23, tmp);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.ldl(A1, tmp.wrapping_add(7));
                    c.ldr(A1, tmp);
                    c.g[T3][0] = c.lw(tmp.wrapping_add(8));
                    let out = r32(c, A3);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.sw(out.wrapping_add(8), c.g[T3][0]);
                    let more = c.g[V1][0] != 0;
                    c.g[A3][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, DU_POINT_HEAD);
                    }
                }
            }
            if !next_row(c, T0) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DU_ROW_HEAD);
            }
        }
    }
    // Pass 2: d/dv, crossed with d/du, normalised.
    c.cycles += 3;
    c.g[A3][0] = c.g[A2][0];
    c.g[V1][0] = 0;
    if (c.g[T0][0] as i64) > 0 {
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, V1) << 4);
        loop {
            c.cycles += 20;
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            basis_row(c, r32(c, V0));
            let empty = (c.g[T0][0] as i64) <= 0;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            if !empty {
                c.cycles += 6;
                c.mtc1(0, 9); // f9 = 0
                c.g[A0][0] = c.g[T2][0];
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 8); // f8 = 1
                c.g[V1][0] = c.g[T0][0];
                loop {
                    c.cycles += 34;
                    curve_point(c, r32(c, A0));
                    normal_at(c);
                    c.cycles += 10;
                    let tmp = sp(c);
                    c.ldl(A1, tmp.wrapping_add(0x17));
                    c.ldr(A1, tmp.wrapping_add(0x10));
                    c.g[A2][0] = c.lw(tmp.wrapping_add(0x18));
                    let out = r32(c, A3);
                    c.sdl(A1, out.wrapping_add(7));
                    c.sdr(A1, out);
                    c.sw(out.wrapping_add(8), c.g[A2][0]);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    let more = c.g[V1][0] != 0;
                    c.g[A3][0] = sx(out.wrapping_add(0xc));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, DV_POINT_HEAD);
                    }
                }
            }
            if !next_row(c, T0) {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, DV_ROW_HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    ret(c);
}

/// Pass 2 of `TessPositionsAlt` at one grid entry: `a = out[i]` (d/du),
/// `b = vf23` (d/dv, stored at sp+0); `n = a × b` at sp+0x20, then
/// `n / |n|` (or `n` when `|n| == 0`) at sp+0x10..0x18 (f9 = 0, f8 = 1).
fn normal_at(c: &mut Cpu) {
    let tmp = sp(c);
    let out = r32(c, A3);
    c.sqc2(23, tmp);
    c.lwc1(1, out.wrapping_add(8)); // a.z
    c.lwc1(6, out); // a.x
    c.lwc1(4, tmp.wrapping_add(4)); // b.y
    c.lwc1(0, tmp.wrapping_add(8)); // b.z
    c.lwc1(3, out.wrapping_add(4)); // a.y
    c.mul_s(7, 4, 1);
    c.lwc1(2, tmp); // b.x
    c.mul_s(5, 0, 6);
    c.mul_s(0, 0, 3);
    c.mul_s(1, 2, 1);
    c.mul_s(2, 2, 3);
    c.sub_s(7, 7, 0); // f7 = b.y a.z - b.z a.y
    c.sub_s(5, 5, 1); // f5 = b.z a.x - b.x a.z
    c.mul_s(4, 4, 6);
    c.mul_s(0, 7, 7);
    c.swc1(7, tmp.wrapping_add(0x20));
    c.mul_s(3, 5, 5);
    c.swc1(5, tmp.wrapping_add(0x24));
    c.sub_s(2, 2, 4); // f2 = b.x a.y - b.y a.x
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, tmp.wrapping_add(0x28));
    c.add_s(0, 0, 1);
    c.sqrt_s(0, 0);
    c.c_eq_s(0, 9);
    if c.fcc() {
        c.swc1(7, tmp.wrapping_add(0x10));
        c.cycles += 2;
        c.swc1(5, tmp.wrapping_add(0x14));
        c.swc1(2, tmp.wrapping_add(0x18));
    } else {
        c.cycles += 8;
        c.div_s(0, 8, 0); // 1 / |n|
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, tmp.wrapping_add(0x18));
        c.swc1(2, tmp.wrapping_add(0x10));
        c.swc1(0, tmp.wrapping_add(0x14));
    }
}

// ---------------------------------------------------------------------------
// The sun.

/// `LensFX` fields written by `LensFX_UpdateSunPosition`.
mod lens {
    /// Screen rectangle of the sun sprite: x, y, w, h (floats).
    pub const RECT: u32 = 0x00;
    /// The clamped copy of the rectangle drawn (x, y, w, h).
    pub const CLAMPED: u32 = 0x10;
    /// Sprite depth / size copied from +0x1f8.
    pub const DEPTH: u32 = 0x24;
    /// Visible flags (two halfwords).
    pub const VISIBLE_A: u32 = 0x2c;
    pub const VISIBLE_B: u32 = 0x2e;
    /// The projected sun position (quad).
    pub const SCREEN: u32 = 0x1f0;
    /// Occlusion state cleared when the sun is off screen.
    pub const OCCLUSION: u32 = 0x204;
}

/// gp offset of the graphics object used for projection (`+0x10d8` its
/// camera interface vtable).
const LENS_GRAPHICS_GP: u32 = 0x2a90;
/// `.lit4` entries (gp-relative, negative): the painter-angle scale
/// (degrees to radians) and the default azimuth / elevation (radians).
const LIT_ANGLE_SCALE: u32 = 0x38c0;
const LIT_DEFAULT_AZIMUTH: u32 = 0x38bc;
const LIT_DEFAULT_ELEVATION: u32 = 0x38b8;

/// `void LensFX_UpdateSunPosition(LensFX* fx, Camera* cam, bool alt)`
/// (0x2f4db8): position the sun sprite. Painter view `alt ? 7 : 6`; a sun
/// texture of -1 hides the sun. Otherwise the sun direction comes from the
/// painter's azimuth / elevation (scaled to radians; the `.lit4` defaults
/// when both are 0) through `Math_SinCos`, is put `cam+0x58 - 500` units
/// from the camera position (camera row 3 transformed by the view rows),
/// and projected (camera interface slot 91 = off screen test; slot 92 =
/// project). On screen: both visible flags 1, the projected point at
/// +0x1f0, the 16x16 rect centred on it at +0, and its copy at +0x10 with
/// x / y clamped to `[0, viewport edge - 16]`; off screen: flags and the
/// occlusion state cleared.
pub fn lens_fx_update_sun_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x80);
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A0][0]; // fx
        frame.save(c, S3, 0x70);
        c.g[S1][0] = c.g[A1][0]; // camera
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A2][0]; // alt view
        frame.save(c, S0, 0xa0);
        c.g[A0][0] = 6;
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 22, 0xc0);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        c.g[S4][0] = c.lw(r32(c, GP).wrapping_add(LENS_GRAPHICS_GP));
        callee::WORLD_PAINTER_GET_SUN_TEXTURE.call(c, 0x002f_4df8)?;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] == c.g[V1][0] {
            // No sun.
            c.cycles += 3;
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        c.lwc1(1, at(c, S1, 0x58));
        // The painter angles.
        c.cycles += 18;
        c.g[V0][0] = 6;
        c.g[AT][0] = 0x43fa_0000; // 500.0
        c.mtc1(AT, 0);
        c.g[S0][0] = if c.g[S3][0] == 0 { c.g[V0][0] } else { 7 };
        c.lwc1(20, r32(c, GP).wrapping_sub(LIT_ANGLE_SCALE));
        c.sub_s(22, 1, 0); // distance = cam+0x58 - 500
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_AZIMUTH.call(c, 0x002f_4e34)?;
        c.mul_s(21, 0, 20); // azimuth
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_PAINTER_GET_SUN_ELEVATION.call(c, 0x002f_4e40)?;
        c.mul_s(20, 0, 20); // elevation
        c.mtc1(0, 0);
        c.c_eq_s(21, 0);
        let azimuth_set = !c.fcc();
        c.mov_s(12, 21);
        let painter_angles = azimuth_set || {
            c.cycles += 4;
            c.c_eq_s(20, 0);
            !c.fcc()
        };
        if azimuth_set {
            c.cycles += 1;
        }
        c.g[A0][0] = sx(sp(c).wrapping_add(0x40)); // &sincos(azimuth)
        if painter_angles {
            c.cycles += 7;
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e98)?;
            c.mov_s(12, 20);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4ea8)?;
        } else {
            c.cycles += 9;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_AZIMUTH));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x44));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e74)?;
            c.lwc1(12, r32(c, GP).wrapping_sub(LIT_DEFAULT_ELEVATION));
            c.g[A0][0] = sx(sp(c).wrapping_add(0x48));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x4c));
            call_ptr(c, table_callee::MATH_SIN_COS, 0x002f_4e84)?;
        }
        c.lq(V0, at(c, S1, 0x40));
        sun_world_position(c);
        // Off screen?
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh((vt as u32).wrapping_add(0x2d8));
        c.g[V0][0] = c.lw((vt as u32).wrapping_add(0x2dc));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002f_5024)?;
        let off_screen = c.g[V0][0] != 0;
        c.g[V1][0] = 1;
        if off_screen {
            c.cycles += 4;
            c.sw(at(c, S2, lens::OCCLUSION), 0);
            c.sh(at(c, S2, lens::VISIBLE_A), 0);
            c.sh(at(c, S2, lens::VISIBLE_B), 0);
            sun_epilogue(c, frame);
            return Ok(());
        }
        // Project the sun to the screen.
        c.cycles += 41;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.sh(at(c, S2, lens::VISIBLE_B), c.g[V1][0]);
        c.g[A2][0] = c.g[S0][0];
        c.sh(at(c, S2, lens::VISIBLE_A), c.g[V1][0]);
        let vt = c.lw(at(c, S4, 0x10d8));
        c.g[V0][0] = vt;
        c.g[A1][0] = c.lh((vt as u32).wrapping_add(0x2e0));
        c.g[V1][0] = c.lw((vt as u32).wrapping_add(0x2e4));
        let method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002f_5060)?;
        sun_screen_rect(c);
        sun_epilogue(c, frame);
        Ok(())
    })
}

/// The sun's world position (v0 = camera row 3 at entry): the camera
/// position `-(row 3)` through the camera rows (FPU) at sp+0, the
/// direction `(r44 r4c, r40 r4c, r48, 0)` from the two `Math_SinCos`
/// result pairs at sp+0x40..0x4c, and `sp+0x10 = position + direction *
/// distance (f22)` (VU0 macro ops). Leaves s0 = a1 = sp+0x10.
fn sun_world_position(c: &mut Cpu) {
    c.cycles += 96;
    let sp = sp(c);
    let cam = r32(c, S1);
    let at_sp = |o: u32| sp.wrapping_add(o);
    let at_cam = |o: u32| cam.wrapping_add(o);
    c.g[S0][0] = sx(at_sp(0x10));
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
    c.mtc1(AT, 0);
    c.g[A1][0] = c.g[S0][0];
    // Negate the camera translation row: (-1) * row 3.
    c.sq(V0, at_sp(0x20));
    c.mfc1(V1, 0);
    c.lqc2(4, at_sp(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x30));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 14); // f14 = 1.0
    c.lq(V0, at_sp(0x30));
    // Transform it by the camera rotation rows (FPU).
    c.lwc1(6, at_cam(0x10));
    c.sq(V0, sp);
    c.lwc1(12, at_cam(0x14));
    c.lwc1(3, at_cam(0x20));
    c.lwc1(9, at_cam(0x24));
    c.lwc1(2, at_cam(0x30));
    c.lwc1(1, at_cam(0x34));
    c.lwc1(0, sp);
    c.lwc1(5, at_sp(0x4));
    c.lwc1(7, at_cam(0x40));
    c.mul_s(6, 0, 6);
    c.lwc1(8, at_cam(0x44));
    c.mul_s(12, 5, 12);
    c.mul_s(9, 5, 9);
    c.lwc1(10, at_cam(0x18));
    c.mul_s(1, 5, 1);
    c.lwc1(11, at_cam(0x28));
    c.mul_s(3, 0, 3);
    c.lwc1(4, at_sp(0x8));
    c.mul_s(2, 0, 2);
    c.lwc1(13, at_cam(0x38));
    c.mul_s(5, 5, 8);
    c.mul_s(0, 0, 7);
    c.lwc1(8, at_cam(0x48));
    c.add_s(3, 3, 9);
    c.lwc1(7, at_cam(0x2c));
    c.add_s(2, 2, 1);
    c.lwc1(9, at_cam(0x1c));
    c.add_s(0, 0, 5);
    c.lwc1(1, at_sp(0xc));
    c.mul_s(8, 4, 8);
    c.lwc1(5, at_cam(0x4c));
    c.mul_s(10, 4, 10);
    c.mul_s(11, 4, 11);
    c.add_s(6, 6, 12);
    c.mul_s(4, 4, 13);
    c.lwc1(12, at_cam(0x3c));
    c.mul_s(5, 1, 5);
    c.sq(V0, at_sp(0x10));
    c.mul_s(9, 1, 9);
    c.mul_s(7, 1, 7);
    c.mul_s(1, 1, 12);
    c.add_s(6, 6, 10);
    c.add_s(3, 3, 11);
    c.add_s(2, 2, 4);
    c.add_s(0, 0, 8);
    c.add_s(3, 3, 7);
    c.add_s(2, 2, 1);
    c.add_s(0, 0, 5);
    c.add_s(6, 6, 9);
    c.swc1(3, at_sp(0x14));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x1c));
    c.swc1(6, at_sp(0x10));
    c.lq(V0, at_sp(0x10));
    c.sq(V0, sp); // the camera position
                  // The direction from the Math_SinCos results.
    c.lwc1(3, at_sp(0x4c));
    c.lwc1(0, at_sp(0x44));
    c.lwc1(1, at_sp(0x40));
    c.mul_s(0, 0, 3);
    c.lwc1(2, at_sp(0x48));
    c.mul_s(1, 1, 3);
    c.swc1(14, at_sp(0xc));
    c.swc1(2, at_sp(0x18));
    c.swc1(0, at_sp(0x10));
    c.swc1(1, at_sp(0x14));
    c.sw(at_sp(0x1c), 0);
    // position = camera + direction * distance (VU0).
    c.mfc1(V0, 22);
    c.lqc2(4, at_sp(0x10));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at_sp(0x10));
    c.lqc2(3, at_sp(0x10));
    c.lqc2(4, sp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at_sp(0x10));
}

/// After the projection (the screen point returned at sp+0x20): store it at
/// +0x1f0, centre a 16x16 rect on it at +0, copy the rect to +0x10 and
/// clamp its x to `[0, cam[0] + cam[2] - 16]` and its y to `[0, cam[0] +
/// cam[3] - 16]` (negative becomes 0; the viewport words at cam+0..0xc),
/// copy the depth from +0x1f8 to +0x24.
fn sun_screen_rect(c: &mut Cpu) {
    let fx = r32(c, S2);
    let cam = r32(c, S1);
    c.lq(V0, sp(c).wrapping_add(0x20));
    c.g[AT][0] = 0x4100_0000; // 8.0
    c.mtc1(AT, 2);
    c.sq(V0, fx.wrapping_add(lens::SCREEN));
    c.g[AT][0] = 0x4180_0000; // 16.0
    c.mtc1(AT, 4);
    c.lwc1(0, fx.wrapping_add(lens::SCREEN));
    c.lwc1(1, fx.wrapping_add(lens::SCREEN + 4));
    c.sub_s(0, 0, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 8));
    c.sub_s(1, 1, 2);
    c.swc1(4, fx.wrapping_add(lens::RECT + 0xc));
    c.mtc1(0, 3); // f3 = 0
    c.swc1(0, fx.wrapping_add(lens::RECT));
    c.swc1(1, fx.wrapping_add(lens::RECT + 4));
    c.mov_s(5, 3);
    // Copy the rect (unaligned doubleword pairs).
    c.ldl(V0, fx.wrapping_add(lens::RECT + 7));
    c.ldr(V0, fx.wrapping_add(lens::RECT));
    c.ldl(V1, fx.wrapping_add(lens::RECT + 0xf));
    c.ldr(V1, fx.wrapping_add(lens::RECT + 8));
    c.sdl(V0, fx.wrapping_add(lens::CLAMPED + 7));
    c.sdr(V0, fx.wrapping_add(lens::CLAMPED));
    c.sdl(V1, fx.wrapping_add(lens::CLAMPED + 0xf));
    c.sdr(V1, fx.wrapping_add(lens::CLAMPED + 8));
    // Clamp x.
    c.lwc1(0, cam);
    c.lwc1(1, cam.wrapping_add(8));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED));
    c.add_s(0, 0, 1);
    c.c_le_s(3, 2);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    // Clamp y.
    c.cycles += 9;
    c.swc1(5, fx.wrapping_add(lens::CLAMPED));
    c.lwc1(2, fx.wrapping_add(lens::CLAMPED + 4));
    c.mov_s(5, 3);
    c.lwc1(1, cam.wrapping_add(0xc));
    c.lwc1(0, cam);
    c.c_le_s(3, 2);
    c.add_s(0, 0, 1);
    let inside = c.fcc();
    c.sub_s(0, 0, 4);
    if inside {
        c.cycles += 1;
        c.min_s(5, 2, 0);
    }
    c.cycles += 3;
    c.lwc1(0, fx.wrapping_add(0x1f8));
    c.swc1(5, fx.wrapping_add(lens::CLAMPED + 4));
    c.swc1(0, fx.wrapping_add(lens::DEPTH));
}

/// `LensFX_UpdateSunPosition`'s shared epilogue.
fn sun_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore_ra(c, 0x50);
    frame.restore_fpr(c, 22, 0xc0);
    frame.restore_fpr(c, 21, 0xb8);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
}

/// The world-painter view table `D_004FA370` (built as `0x500000 -
/// 0x5c90`), stride 0xf0; the sun interface holder at +0x14.
const PAINTER_VIEWS_HI: u64 = 0x50_0000;
const PAINTER_VIEWS_LO: u32 = 0x5c90;
const PAINTER_VIEW_SUN: u32 = 0x14;

/// The `WorldPainter_GetSun*` float getters (index in a0): `obj =
/// *views[index].sun`, call its virtual slot `vt_off` (`{delta, fn}` at
/// `obj->vtable + vt_off`) with `obj + delta`, and return the float the
/// method's result points at.
fn sun_float_getter(c: &mut Cpu, vt_off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, A0, A0, V1); // a0 = index * 0xf0
        c.g[V0][0] = PAINTER_VIEWS_HI;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((PAINTER_VIEWS_HI as u32).wrapping_sub(PAINTER_VIEWS_LO));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(at(c, A0, PAINTER_VIEW_SUN));
        c.g[A2][0] = c.lw(r32(c, V1));
        c.g[A1][0] = c.lw(at(c, A2, 4));
        c.g[A0][0] = c.lh(at(c, A1, vt_off));
        c.g[V0][0] = c.lw(at(c, A1, vt_off + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetSunElevation(int view)` (0x2ee070): sun slot
/// 27 (`tWPIGD_Sun_GetElevation`).
pub fn world_painter_get_sun_elevation(c: &mut Cpu) {
    sun_float_getter(c, 0xd8, 0x002e_e0a8);
}

/// `float WorldPainter_GetSunAzimuth(int view)` (0x2ee0b8): sun slot 28
/// (`tWPIGD_Sun_GetAzimuth`).
pub fn world_painter_get_sun_azimuth(c: &mut Cpu) {
    sun_float_getter(c, 0xe0, 0x002e_e0f0);
}

/// `float WorldPainter_GetSunRed(int view)` (0x2ee100): sun slot 29.
pub fn world_painter_get_sun_red(c: &mut Cpu) {
    sun_float_getter(c, 0xe8, 0x002e_e138);
}

/// `float WorldPainter_GetSunGreen(int view)` (0x2ee148): sun slot 30.
pub fn world_painter_get_sun_green(c: &mut Cpu) {
    sun_float_getter(c, 0xf0, 0x002e_e180);
}

/// `float WorldPainter_GetSunBlue(int view)` (0x2ee190): sun slot 31.
pub fn world_painter_get_sun_blue(c: &mut Cpu) {
    sun_float_getter(c, 0xf8, 0x002e_e1c8);
}

/// `float WorldPainter_GetSunIntensity(int view)` (0x2ee1d8): sun slot 32.
pub fn world_painter_get_sun_intensity(c: &mut Cpu) {
    sun_float_getter(c, 0x100, 0x002e_e210);
}

/// `float WorldPainter_GetSunSize(int view)` (0x2ee2b0): sun slot 35.
pub fn world_painter_get_sun_size(c: &mut Cpu) {
    sun_float_getter(c, 0x118, 0x002e_e2e8);
}

/// `float* tWPIGD_Sun::GetElevation()` (0x2c1560): `&this->elevation`
/// (+0x08).
pub fn twpigd_sun_get_elevation(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x8));
    ret(c);
}

/// `float* tWPIGD_Sun::GetAzimuth()` (0x2c1568): `&this->azimuth` (+0x10).
pub fn twpigd_sun_get_azimuth(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
    ret(c);
}

#[cfg(test)]
mod tests {
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, fake_call, machine, FAKE, LOG_COUNT};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::{reg::*, sx};
    use ssx3_ee_rt::Cpu;

    /// A random float in [-range, range) as bits.
    fn f(r: &mut Rng, range: f32) -> u32 {
        r.f32(range)
    }

    // ------------------------------------------------------------ Nash pass

    /// The VU0 microprogram at 0xef0 for the tests: `vi1 = vi14` (the
    /// node's clip word), so a clip word of 2 culls.
    fn put_cull_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0xef0;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E bit
        c.bus.write32(at + 8, 0x8000_033c); // NOP
        c.bus.write32(at + 12, 0x0000_02ff); // NOP
    }

    /// `Nash_CollectPatches` input: `nodes` (node, clipped) pairs with up
    /// to `per_node` patches each, the section tables, a patch count.
    fn patches_setup(c: &mut Cpu, r: &mut Rng, nodes: u32, per_node: u32) -> u32 {
        put_cull_microprogram(c);
        let this = DATA + 0x400;
        let tables = DATA + 0x40000;
        c.bus.write32(this + 0x90, DATA + 0x100);
        c.bus.write32(DATA + 0x100, tables);
        for s in 0..16 {
            let state = if r.next_u32() % 2 == 0 {
                6
            } else {
                r.next_u32() % 8
            };
            c.bus.write32(tables + 8 * s + 0x24, state);
        }
        for l in 0..32u32 {
            let state = if r.next_u32() % 2 == 0 {
                3
            } else {
                r.next_u32() % 8
            };
            c.bus.write32(tables + 0x18 * l + 0x3f0, state);
        }
        c.bus.write32(this + 0x50a8, r.next_u32() % 10);
        let pairs = DATA + 0x200;
        let mut patch = DATA + 0x10000;
        for i in 0..nodes {
            let node = DATA + 0x8000 + 0x40 * i;
            c.bus.write32(pairs + 8 * i, node);
            c.bus.write32(
                pairs + 8 * i + 4,
                [0, 1, 2, 2, r.next_u32()][(r.next_u32() % 5) as usize],
            );
            let n = r.next_u32() % (per_node + 1);
            let mut head = 0;
            for _ in 0..n {
                fill(c, r, patch + 0x140, 4);
                c.bus.write32(patch, head);
                c.bus.write8(patch + 0x155, (r.next_u32() % 16) as u8);
                let logical = if r.next_u32() % 5 == 0 {
                    -((r.next_u32() % 8) as i32) - 1
                } else {
                    (r.next_u32() % 32) as i32
                };
                c.bus.write16(patch + 0x156, logical as u16);
                head = patch;
                patch += 0x200;
            }
            c.bus.write32(node + 0x24, head);
        }
        c.g[A0][0] = this as u64;
        c.g[A1][0] = pairs as u64;
        c.g[A2][0] = if nodes == 0 && r.next_u32() % 2 == 0 {
            sx(0xffff_fff0)
        } else {
            nodes as u64
        };
        this
    }

    #[test]
    fn nash_collect_patches_matches() {
        let gen = ssx3_ee::s05::f_0022a698__generated;
        let hand = super::nash_collect_patches;
        check_events(gen, hand, 300, &|c, seed| {
            let mut r = Rng(0x7061_7463 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let nodes = [0, 1, 3, 5][(seed / 3 % 4) as usize];
            patches_setup(c, &mut r, nodes, 5);
            schedule(c, &mut r, seed, 0x0022_a698)
        });
        // Every patch path (negative logical / section / logical filter,
        // clipped kept / culled, unclipped) for one patch.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400 {
            let mut c = crate::difftest::fresh();
            let mut r = Rng(0x7031 ^ (seed as u64) << 20);
            scramble(&mut c, &mut r);
            let this = patches_setup(&mut c, &mut r, 1, 1);
            let before = c.bus.read32(this + 0x50a8);
            crate::difftest::run_generated(&mut c, gen);
            seen.insert((c.cycles, c.bus.read32(this + 0x50a8) - before));
        }
        assert!(seen.len() >= 5, "patch paths seen: {seen:?}");
    }

    #[test]
    fn vu0_upload_matrix_pair_matches() {
        check(
            ssx3_ee::s05::f_00229f30__generated,
            super::vu0_upload_matrix_pair,
            100,
            &|c, seed| {
                let mut r = Rng(0x7570_6c64 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA, 32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
            },
        );
    }

    /// `Nash_GatherVisible` input: the world at `this`, the eight roots
    /// (each present with probability 1/2; small octrees of level 11..12
    /// with empty instance / patch / effect lists), an occlusion manager
    /// with no views (`OcclusionMan_BuildVisible` takes its short path),
    /// the graphics object with a fake camera interface slot 43 and a world
    /// matrix. `Nash_CullNode` runs on the hand-written VU0 routines.
    fn gather_setup(c: &mut Cpu, r: &mut Rng) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_vu::vu::vu0lib::run);
        let this = DATA + 0x20000;
        let roots = DATA + 0x300;
        let occ = DATA + 0x600;
        let tables = DATA + 0x700;
        let gfx = DATA + 0x30000;
        let vt = DATA + 0x38000;
        c.bus.write32(GAME_GP - 0x854, gfx);
        c.bus.write32(gfx + 0x10d8, vt);
        let delta = (r.next_u32() as i16 as i32 % 64) & !3;
        c.bus.write16(vt + 0x158, delta as u16);
        c.bus.write32(vt + 0x15c, FAKE);
        for i in 0..16 {
            c.bus.write32(gfx + 0x5800 + 4 * i, f(r, 2.0));
        }
        // Stale list counts and patch counts (the pass clears them).
        for off in [0x94u32, 0x1098, 0x209c, 0x50a8, 0x7fc4] {
            c.bus.write32(this + off, r.next_u32() % 50);
        }
        c.bus.write32(occ + 4, 0);
        c.bus.write32(occ + 0x110, r.next_u32());
        c.bus.write32(tables, DATA + 0x800);
        let mut next = DATA + 0x10000;
        fn tree(c: &mut Cpu, r: &mut Rng, next: &mut u32, depth: u32) -> u32 {
            let node = *next;
            *next += 0x40;
            for k in 0..8 {
                let child = if depth > 0 && r.next_u32() % 2 == 0 {
                    tree(c, r, next, depth - 1)
                } else {
                    0
                };
                c.bus.write32(node + 4 * k, child);
            }
            node
        }
        for k in 0..8u32 {
            let cell = roots + 0x14 * k;
            let level = 11 + r.next_u32() % 2;
            c.bus.write32(cell, level);
            for a in 1..4u32 {
                c.bus
                    .write32(cell + 4 * a, ((r.next_u32() % 9) as i32 - 4) as u32);
            }
            let node = if r.next_u32() % 2 == 0 {
                tree(c, r, &mut next, level - 11)
            } else {
                0
            };
            c.bus.write32(cell + 0x10, node);
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(roots);
        c.g[A2][0] = sx(occ);
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = sx(tables);
    }

    #[test]
    fn nash_gather_visible_matches() {
        covered(
            ssx3_ee::s05::f_0022add8__generated,
            super::nash_gather_visible,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6761_7468, 1);
                gather_setup(c, &mut r);
            },
        );
    }

    // ---------------------------------------------------- patch tessellation

    /// A patch cache with the LOD's basis tables and count, a patch with
    /// control points (all zero for some seeds: zero normals), an output
    /// grid. Returns nothing; sets a0..a3.
    fn tess_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let cache = DATA;
        let patch = DATA + 0x1000;
        let value = DATA + 0x2000;
        let deriv = DATA + 0x3000;
        let out = DATA + 0x8000;
        let lod = r.next_u32() % 3;
        for l in 0..3 {
            c.bus.write32(cache + 0x190 + 4 * l, value + 0x100 * l);
            c.bus.write32(cache + 0x310 + 4 * l, deriv + 0x100 * l);
            let n = match (seed / 3) % 6 {
                0 => 0,
                1 => 0xffff_fffe,
                k => k,
            };
            c.bus.write32(cache + 0x19c + 4 * l, n);
        }
        for i in 0..(3 * 0x40) {
            c.bus.write32(value + 4 * i, f(r, 1.0));
            c.bus.write32(deriv + 4 * i, f(r, 3.0));
        }
        let zero = seed % 5 == 0;
        for i in 0..64 {
            let v = if zero {
                0
            } else if i % 7 == 0 {
                r.edge_f32()
            } else {
                f(r, 50.0)
            };
            c.bus.write32(patch + 0x40 + 4 * i, v);
        }
        fill(c, r, out, 80);
        c.g[A0][0] = sx(cache);
        c.g[A1][0] = sx(patch);
        c.g[A2][0] = sx(out);
        c.g[A3][0] = lod as u64 | (r.next_u32() as u64) << 32;
    }

    #[test]
    fn bezier_patch_cache_tess_positions_matches() {
        check_events(
            ssx3_ee::s12::f_00374180__generated,
            super::bezier_patch_cache_tess_positions,
            240,
            &|c, seed| {
                let mut r = Rng(0x7465_7373 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                tess_setup(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0037_4180)
            },
        );
    }

    #[test]
    fn bezier_patch_cache_tess_positions_alt_matches() {
        check_events(
            ssx3_ee::s12::f_00374518__generated,
            super::bezier_patch_cache_tess_positions_alt,
            240,
            &|c, seed| {
                let mut r = Rng(0x616c_7474 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                tess_setup(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x0037_4518)
            },
        );
    }

    // ----------------------------------------------------------------- sun

    /// Bit n set: fake call n returns 0 (for the off-screen test).
    const ZERO_RESULTS: u32 = fakes::LOG + 0x34;

    fn fake_maybe_zero(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let n = c.bus.read32(LOG_COUNT).wrapping_sub(1);
        if n < 32 && c.bus.read32(ZERO_RESULTS) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// The painter view table entry `view`: its sun holder -> object ->
    /// vtable whose slots `slots` are fakes (FAKE + 0x10 i).
    fn painter_sun(c: &mut Cpu, r: &mut Rng, view: u32, holder: u32, vt: u32, slots: &[u32]) {
        let entry = 0x004f_a370u32.wrapping_add(view.wrapping_mul(0xf0));
        let obj = holder + 0x40;
        c.bus.write32(entry + 0x14, holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    fn sun_getter_setup(c: &mut Cpu, seed: u32, vt_off: u32) {
        let mut r = machine(c, seed, 0x5375_6e00 ^ vt_off as u64, 1);
        let view = seed % 8;
        c.g[A0][0] = view as u64 | (r.next_u32() as u64) << 32;
        painter_sun(c, &mut r, view, DATA + 0x200, DATA + 0x6000, &[vt_off]);
    }

    macro_rules! sun_getter_test {
        ($name:ident, $gen:path, $hand:path, $off:expr) => {
            #[test]
            fn $name() {
                covered($gen, $hand, 160, &|c, seed| sun_getter_setup(c, seed, $off));
            }
        };
    }

    sun_getter_test!(
        world_painter_get_sun_elevation_matches,
        ssx3_ee::s09::f_002ee070__generated,
        super::world_painter_get_sun_elevation,
        0xd8
    );
    sun_getter_test!(
        world_painter_get_sun_azimuth_matches,
        ssx3_ee::s09::f_002ee0b8__generated,
        super::world_painter_get_sun_azimuth,
        0xe0
    );
    sun_getter_test!(
        world_painter_get_sun_red_matches,
        ssx3_ee::s09::f_002ee100__generated,
        super::world_painter_get_sun_red,
        0xe8
    );
    sun_getter_test!(
        world_painter_get_sun_green_matches,
        ssx3_ee::s09::f_002ee148__generated,
        super::world_painter_get_sun_green,
        0xf0
    );
    sun_getter_test!(
        world_painter_get_sun_blue_matches,
        ssx3_ee::s09::f_002ee190__generated,
        super::world_painter_get_sun_blue,
        0xf8
    );
    sun_getter_test!(
        world_painter_get_sun_intensity_matches,
        ssx3_ee::s09::f_002ee1d8__generated,
        super::world_painter_get_sun_intensity,
        0x100
    );
    sun_getter_test!(
        world_painter_get_sun_size_matches,
        ssx3_ee::s09::f_002ee2b0__generated,
        super::world_painter_get_sun_size,
        0x118
    );

    #[test]
    fn twpigd_sun_get_elevation_matches() {
        check(
            ssx3_ee::s08::f_002c1560__generated,
            super::twpigd_sun_get_elevation,
            100,
            &|c, seed| {
                let mut r = Rng(0x2c15_60 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn twpigd_sun_get_azimuth_matches() {
        check(
            ssx3_ee::s08::f_002c1568__generated,
            super::twpigd_sun_get_azimuth,
            100,
            &|c, seed| {
                let mut r = Rng(0x2c15_68 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    /// `LensFX_UpdateSunPosition` input. Fake calls: 0 the sun texture
    /// (-1 for a quarter of the seeds), 1 the azimuth, 2 the elevation
    /// (each 0.0 for some seeds: the `.lit4` default angles), 3 the
    /// off-screen test (0 = on screen for half the seeds), 4 the
    /// projection. `Math_SinCos` runs for real.
    fn sun_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x4c46_5853, 5);
        c.hooks.miss = fake_maybe_zero;
        let fx = DATA + 0x1000;
        let cam = DATA + 0x2000;
        let gfx = DATA + 0x3000;
        let results = DATA + 0x100;
        // Painter views 6 and 7 share one sun object.
        for view in [6, 7] {
            painter_sun(
                c,
                &mut r,
                view,
                DATA + 0x200,
                DATA + 0x6000,
                &[0xd8, 0xe0, 0x108],
            );
        }
        // The projection interface.
        c.bus.write32(GAME_GP + 0x2a90, gfx);
        c.bus.write32(gfx + 0x10d8, DATA + 0x7000);
        for (i, off) in [0x2d8u32, 0x2e0].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 % 64) & !3;
            c.bus.write16(DATA + 0x7000 + off, delta as u16);
            c.bus
                .write32(DATA + 0x7000 + off + 4, FAKE + 0x40 + 0x10 * i as u32);
        }
        // Forced fake results: pointers to the texture / azimuth /
        // elevation words.
        let texture = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(results, texture);
        let az = if seed % 3 == 0 { 0 } else { f(&mut r, 360.0) };
        let el = if seed % 5 < 2 { 0 } else { f(&mut r, 90.0) };
        c.bus.write32(results + 4, az);
        c.bus.write32(results + 8, el);
        for (n, p) in [results, results + 4, results + 8].into_iter().enumerate() {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n as u32, p);
        }
        c.bus
            .write32(ZERO_RESULTS, if seed % 2 == 0 { 1 << 3 } else { 0 });
        // `.lit4`: degrees to radians, the default angles.
        c.bus
            .write32(GAME_GP - 0x38c0, (std::f32::consts::PI / 180.0).to_bits());
        c.bus.write32(GAME_GP - 0x38bc, f(&mut r, 3.0));
        c.bus.write32(GAME_GP - 0x38b8, f(&mut r, 1.5));
        // The camera: viewport, view rows, translation, distance.
        for i in 0..4 {
            c.bus.write32(cam + 4 * i, f(&mut r, 640.0));
        }
        for i in 4..0x16 {
            c.bus.write32(cam + 4 * i, f(&mut r, 2.0));
        }
        c.bus.write32(cam + 0x40, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x44, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x48, f(&mut r, 1000.0));
        c.bus.write32(cam + 0x58, f(&mut r, 3000.0));
        fill(c, &mut r, fx, 16);
        fill(c, &mut r, fx + 0x1f0, 8);
        c.g[A0][0] = sx(fx);
        c.g[A1][0] = sx(cam);
        c.g[A2][0] = if seed % 7 < 3 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
    }

    #[test]
    fn lens_fx_update_sun_position_matches() {
        covered(
            ssx3_ee::s09::f_002f4db8__generated,
            super::lens_fx_update_sun_position,
            300,
            &|c, seed| sun_setup(c, seed),
        );
    }
}
